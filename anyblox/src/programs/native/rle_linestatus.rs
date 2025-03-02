use std::ptr;

use super::{NativeBatch, NativeImpl};
use crate::ColumnProjection;

#[derive(Default)]
pub struct NativeRleLinestatusImpl {
    state: Option<State>,
}

struct State {
    output: OutputColumn,
    batch_capsule: BatchCapsule,
}

struct BatchCapsule {
    batch: super::NativeBatch,
    buffer_array: [*mut u8; 2],
    inner_batch: NativeBatch,
    children_array: [*mut NativeBatch; 1],
    top_level_buffer_array: [*mut NativeBatch; 1],
}

impl NativeImpl for NativeRleLinestatusImpl {
    fn anyblox_decode(
        &mut self,
        data: &[u8],
        start_tuple: usize,
        tuple_count: usize,
        _projection: ColumnProjection,
    ) -> &NativeBatch {
        let state = self.state.get_or_insert_with(|| State {
            output: OutputColumn::new(),
            batch_capsule: BatchCapsule::new(),
        });

        unsafe {
            let tuples_per_page = data.as_ptr().cast::<u32>().add(1).read();
            let start_page = (start_tuple as u32) / tuples_per_page;
            let skip_in_first = (start_tuple as u32) % tuples_per_page;
            let start_idx = data.as_ptr().cast::<u32>().add(2 + start_page as usize).read() as usize;

            let idx = start_idx;
            state.output.reset(tuple_count, skip_in_first as usize);
            let mut decoder = RleDecoder::new(data.as_ptr().add(idx), data.len() - idx, &mut state.output);

            decoder.read(tuple_count + skip_in_first as usize, tuples_per_page as usize);

            state.batch_capsule.write(state.output.buf());
        }

        &state.batch_capsule.batch
    }
}

impl BatchCapsule {
    pub fn new() -> Self {
        Self {
            batch: NativeBatch::empty(),
            buffer_array: [std::ptr::null_mut(); 2],
            inner_batch: NativeBatch::empty(),
            children_array: [std::ptr::null_mut(); 1],
            top_level_buffer_array: [std::ptr::null_mut(); 1],
        }
    }

    pub unsafe fn write(&mut self, buffer: &[u8]) {
        self.batch.length = buffer.len() as u32;
        self.batch.null_count = 0;
        self.batch.n_buffers = 1;
        self.batch.n_children = 1;
        self.batch.buffers = self.top_level_buffer_array.as_mut_ptr().cast();
        self.batch.children = self.children_array.as_mut_ptr().cast();
        self.batch.dictionary = ptr::null_mut();

        self.children_array[0] = &raw mut self.inner_batch;

        self.buffer_array[0] = ptr::null_mut();
        self.buffer_array[1] = buffer.as_ptr().cast_mut();

        self.inner_batch.length = buffer.len() as u32;
        self.inner_batch.null_count = 0;
        self.inner_batch.n_buffers = 2;
        self.inner_batch.n_children = 0;
        self.inner_batch.buffers = self.buffer_array.as_mut_ptr().cast();
        self.inner_batch.children = ptr::null_mut();
        self.inner_batch.dictionary = ptr::null_mut();
    }
}

const ZERO: u8 = b'O';
const ONE: u8 = b'F';

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
    buffered_values: u64,
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

        if self.bit_offset == 64 {
            self.byte_offset += 8;
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
        let bytes_to_read = core::cmp::min(self.data_len - self.byte_offset, 8);
        let mut buf = [0; 8];
        unsafe {
            core::ptr::copy_nonoverlapping(self.data.add(self.byte_offset), buf.as_mut_ptr(), bytes_to_read);
        }
        self.buffered_values = u64::from_le_bytes(buf);
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
            buf: vec![],
            to_skip: 0,
        }
    }

    pub fn reset(&mut self, tuple_count: usize, to_skip: usize) {
        self.buf.clear();
        self.buf.reserve(tuple_count);
        self.to_skip = to_skip;
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
