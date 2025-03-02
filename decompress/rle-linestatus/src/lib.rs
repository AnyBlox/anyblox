#![no_std]
extern crate alloc;

use alloc::vec::Vec;
use arrow::ArrowArray;
use decoder_lib::*;
use ffi_utils::WasmPtr;

const ZERO: u8 = b'O';
const ONE: u8 = b'F';

struct State {
    output: OutputColumn,
}

struct OutputColumn {
    buf: Vec<u8>,
    to_skip: usize,
}

struct RleDecoder<'a> {
    // The remaining number of values in RLE for this run
    rle_left: u32,
    // The remaining number of values in Bit-Packing for this run
    bit_packed_left: u32,
    // The current value for the case of RLE mode
    current_value: Option<u8>,
    // Bit reader loaded with input buffer.
    bit_reader: BitReader,
    // Output column
    output: &'a mut OutputColumn,
}

pub struct BitReader {
    /// The byte buffer to read from
    data: *const u8,
    /// The length of the byte buffer
    data_len: usize,
    /// Bytes are memcpy'd from `buffer` and values are read from this variable.
    /// This is faster than reading values byte by byte directly from `buffer`
    ///
    /// This is only populated when `self.bit_offset != 0`
    buffered_values: u32,
    ///
    /// End                                         Start
    /// |............|B|B|B|B|B|B|B|B|..............|
    ///                   ^          ^
    ///                 bit_offset   byte_offset
    ///
    /// Current byte offset in `buffer`
    byte_offset: usize,
    /// Current bit offset in `buffered_values`
    bit_offset: usize,
}

impl<'a> RleDecoder<'a> {
    pub fn new(data: *const u8, data_len: usize, output: &'a mut OutputColumn) -> Self {
        RleDecoder {
            rle_left: 0,
            bit_packed_left: 0,
            current_value: None,
            bit_reader: BitReader::new(data, data_len),
            output,
        }
    }

    pub fn read(&mut self, count: usize, tuples_per_page: usize) {
        let mut values_read = 0;
        while values_read < count {
            self.reload();
            let mut read_in_page = 0;
            while values_read < count && read_in_page < tuples_per_page {
                let remaining = core::cmp::min(tuples_per_page - read_in_page, count - values_read);
                if self.rle_left > 0 {
                    let num_values = core::cmp::min(remaining, self.rle_left as usize);
                    for _ in 0..num_values {
                        let repeated_value = self.current_value.unwrap();
                        self.output.push(repeated_value);
                    }
                    self.rle_left -= num_values as u32;
                    values_read += num_values;
                    read_in_page += num_values;
                } else if self.bit_packed_left > 0 {
                    let mut num_values = core::cmp::min(remaining, self.bit_packed_left as usize);

                    num_values = self.bit_reader.read_batch(num_values, self.output);

                    self.bit_packed_left = 0;
                    values_read += num_values;
                    read_in_page += num_values;
                } else {
                    self.reload();
                }
            }
        }
    }

    fn reload(&mut self) {
        let indicator_value = self.bit_reader.get_vlq_int();
        if indicator_value & 1 == 1 {
            self.bit_packed_left = ((indicator_value >> 1) * 8) as u32;
        } else {
            self.rle_left = (indicator_value >> 1) as u32;
            self.current_value = Some(self.bit_reader.read_byte());
            debug_assert!(self.current_value.is_some());
        }
    }
}

/// Maximum byte length for a VLQ encoded integer
/// MAX_VLQ_BYTE_LEN = 5 for i32, and MAX_VLQ_BYTE_LEN = 10 for i64
pub const MAX_VLQ_BYTE_LEN: usize = 10;

impl BitReader {
    pub fn new(data: *const u8, data_len: usize) -> Self {
        BitReader {
            data,
            data_len,
            buffered_values: 0,
            byte_offset: 0,
            bit_offset: 0,
        }
    }

    /// Gets the current byte offset
    #[inline]
    pub fn get_byte_offset(&self) -> usize {
        self.byte_offset + (self.bit_offset + 7) / 8
    }

    pub fn read_bit(&mut self) -> u8 {
        if self.bit_offset == 0 {
            self.load_buffered_values()
        }

        let bit = (self.buffered_values >> self.bit_offset) & 1;
        self.bit_offset += 1;

        if self.bit_offset == 32 {
            self.byte_offset += 4;
            self.bit_offset = 0;
        }

        bit as u8
    }

    pub fn read_byte(&mut self) -> u8 {
        self.byte_offset = self.get_byte_offset();
        self.bit_offset = 0;

        let b = unsafe { self.data.add(self.byte_offset).read() };
        self.byte_offset += 1;
        b
    }

    fn load_buffered_values(&mut self) {
        let bytes_to_read = core::cmp::min(self.data_len - self.byte_offset, 4);
        let mut buf = [0; 4];
        unsafe {
            core::ptr::copy_nonoverlapping(
                self.data.add(self.byte_offset),
                buf.as_mut_ptr(),
                bytes_to_read,
            );
        }
        self.buffered_values = u32::from_le_bytes(buf);
    }

    pub fn get_vlq_int(&mut self) -> i64 {
        let mut shift = 0;
        let mut v: i64 = 0;
        loop {
            let byte = self.read_byte();
            v |= ((byte & 0x7F) as i64) << shift;
            shift += 7;
            debug_assert!(
                shift <= MAX_VLQ_BYTE_LEN * 7,
                "Num of bytes exceed MAX_VLQ_BYTE_LEN ({MAX_VLQ_BYTE_LEN})"
            );
            if byte & 0x80 == 0 {
                return v;
            }
        }
    }

    fn read_batch(&mut self, mut values_to_read: usize, output: &mut OutputColumn) -> usize {
        let remaining_bits = (self.data_len - self.byte_offset) * 8 - self.bit_offset;
        if remaining_bits < values_to_read {
            values_to_read = remaining_bits;
        }

        let mut i = 0;
        if self.bit_offset != 0 {
            while i < values_to_read && self.bit_offset != 0 {
                let value = self.read_bit();
                output.push(value);
                i += 1;
            }
        }

        while values_to_read - i >= 8 {
            unsafe { unpack(self.data.add(self.byte_offset), output) };
            self.byte_offset += 1;
            i += 8;
        }

        while i < values_to_read {
            let value = self.read_bit();
            output.push(value);
            i += 1;
        }

        return values_to_read;

        unsafe fn unpack(input: *const u8, output: &mut OutputColumn) {
            let byte = input.read();
            for i in 0..8 {
                output.push((byte >> i) & 1);
            }
        }
    }
}

impl OutputColumn {
    pub fn new() -> Self {
        Self {
            buf: Vec::new(),
            to_skip: 0,
        }
    }

    pub fn reset(&mut self, tuple_count: usize, to_skip: usize) {
        self.buf.clear();
        self.buf.reserve(tuple_count);
        self.to_skip = to_skip;
    }

    pub fn len(&self) -> usize {
        self.buf.len()
    }

    pub fn push(&mut self, byte: u8) {
        if self.to_skip > 0 {
            self.to_skip -= 1;
        } else {
            self.buf.push(if byte == 0 { ZERO } else { ONE });
        }
    }

    pub fn buf(&self) -> &[u8] {
        &self.buf
    }
}

#[no_mangle]
pub unsafe extern "C" fn anyblox_decode(
    data: *const u8,
    data_length: usize,
    start_tuple: usize,
    tuple_count: usize,
    state: *mut u8,
    _projection_mask_1: u32,
    _projection_mask_2: u32,
) -> *const u8 {
    let state_check = state.read();
    if state_check == 0 {
        let state_obj = State {
            output: OutputColumn::new(),
        };
        state.write(1);
        state
            .add(align_of::<State>())
            .cast::<State>()
            .write(state_obj);
    }
    let state: &mut State = &mut *state.add(align_of::<State>()).cast::<State>();

    let tuples_per_page = data.cast::<u32>().add(1).read();
    let start_page = (start_tuple as u32) / tuples_per_page;
    let skip_in_first = (start_tuple as u32) % tuples_per_page;
    let start_idx = data.cast::<u32>().add(2 + start_page as usize).read() as usize;

    let idx = start_idx;
    state.output.reset(tuple_count, skip_in_first as usize);

    let mut decoder = RleDecoder::new(data.add(idx), data_length - idx, &mut state.output);

    decoder.read(
        tuple_count + skip_in_first as usize,
        tuples_per_page as usize,
    );

    write(state.output.buf(), &raw mut BATCH);
    let ptr: *const ArrowArray = &raw const BATCH;
    ptr.cast::<u8>()
}

static mut BATCH: ArrowArray = ArrowArray::empty();

unsafe fn write(buffer: &[u8], batch: *mut ArrowArray) {
    static mut BUFFER_ARRAY: [WasmPtr; 2] = [WasmPtr::NULL, WasmPtr::NULL];
    static mut INNER_BATCH: ArrowArray = ArrowArray::empty();
    static mut CHILDREN_ARRAY: [WasmPtr; 1] = [WasmPtr::NULL];
    static mut TOP_LEVEL_BUFFER_ARRAY: [WasmPtr; 1] = [WasmPtr::NULL];

    CHILDREN_ARRAY[0] = WasmPtr::from(&raw const INNER_BATCH as u32);
    BUFFER_ARRAY[1] = buffer.as_ptr().into();

    (*batch).length = buffer.len() as u32;
    (*batch).null_count = 0;
    (*batch).n_buffers = 1;
    (*batch).n_children = 1;
    (*batch).buffers = WasmPtr::from((&raw mut TOP_LEVEL_BUFFER_ARRAY) as u32);
    (*batch).children = WasmPtr::from((&raw mut CHILDREN_ARRAY) as u32);
    (*batch).dictionary = WasmPtr::NULL;

    INNER_BATCH.length = buffer.len() as u32;
    INNER_BATCH.null_count = 0;
    INNER_BATCH.n_buffers = 2;
    INNER_BATCH.n_children = 0;
    INNER_BATCH.buffers = WasmPtr::from((&raw mut BUFFER_ARRAY) as u32);
    INNER_BATCH.children = WasmPtr::NULL;
    INNER_BATCH.dictionary = WasmPtr::NULL;
}
