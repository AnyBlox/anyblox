use crate::ColumnProjection;
use super::NativeImpl;

#[derive(Default)]
pub struct NativeTrunc8Impl {
    state: Option<State>,
}

struct State {
    buffer: NullableColumnBuffer<u32>,
    batch: super::NativeBatch,
}

impl NativeImpl for NativeTrunc8Impl {
    fn anyblox_decode(
        &mut self,
        data: &[u8],
        start_tuple: usize,
        tuple_count: usize,
        _projection: ColumnProjection,
    ) -> &super::NativeBatch {
        let state = self.state.get_or_insert_with(|| State {
            buffer: NullableColumnBuffer::new(),
            batch: super::NativeBatch::empty(),
        });

        state.buffer.clear();

        let mut offset = start_tuple;
        let tuple_count = core::cmp::min(tuple_count, state.buffer.max_values);
        let end_offset = start_tuple + tuple_count;

        while offset < end_offset {
            let b = unsafe { data.as_ptr().add(offset).read() };
            if b == 0 {
                state.buffer.add_null();
            } else {
                let i = b as u32;
                state.buffer.add_value(i);
            }
            offset += 1;
        }

        unsafe { state.buffer.write_to_batch(&mut state.batch) };

        &state.batch
    }
}

const BUF_SIZE: usize = 64 * 1024;

struct NullableColumnBuffer<T> {
    raw_buffer: [u8; BUF_SIZE],
    current_validity: usize,
    validity_byte: u8,
    validity_byte_idx: usize,
    current_values: usize,
    values_start: usize,
    max_values: usize,
    null_count: u32,
    values_count: u32,
    phantom: core::marker::PhantomData<T>,
}

impl<T> NullableColumnBuffer<T> {
    pub const fn new() -> Self {
        let t_size = size_of::<T>();
        let max_values = (8 * BUF_SIZE) / (8 * t_size + 1);
        let validity_len = (max_values + 7) / 8;

        // The buffers have to be aligned to 8-byte boundaries.
        // If the values buffer is not aligned we simply decrease the max_values by however
        // many is necessary.
        let max_aligned_values = max_values - (validity_len % 8 * 8);
        let validity_aligned_len = (max_aligned_values + 7) / 8;

        Self {
            raw_buffer: [0; 64 * 1024],
            current_validity: 0,
            current_values: validity_aligned_len,
            values_start: validity_aligned_len,
            validity_byte: 0,
            validity_byte_idx: 0,
            max_values: max_aligned_values,
            null_count: 0,
            values_count: 0,
            phantom: core::marker::PhantomData,
        }
    }

    fn add_null(&mut self) {
        self.validity_byte |= 1 << self.validity_byte_idx;
        self.validity_byte_idx += 1;
        unsafe {
            self.raw_buffer
                .as_mut_ptr()
                .add(self.current_values)
                .cast::<u32>()
                .write(0)
        };
        self.current_values += size_of::<T>();
        self.null_count += 1;
        self.values_count += 1;

        self.try_push_validity();
    }

    fn add_value(&mut self, value: T) {
        self.validity_byte_idx += 1;
        unsafe {
            self.raw_buffer
                .as_mut_ptr()
                .add(self.current_values)
                .cast::<T>()
                .write(value)
        };
        self.current_values += size_of::<T>();
        self.values_count += 1;
        self.try_push_validity();
    }

    fn try_push_validity(&mut self) {
        if self.validity_byte_idx == 8 {
            self.force_push_validity();
        }
    }

    fn force_push_validity(&mut self) {
        unsafe {
            self.raw_buffer
                .as_mut_ptr()
                .add(self.current_validity)
                .write(!self.validity_byte)
        }
        self.validity_byte = 0;
        self.validity_byte_idx = 0;
        self.current_validity += 1;
    }

    fn clear(&mut self) {
        self.current_validity = 0;
        self.current_values = self.values_start;
        self.values_count = 0;
        self.null_count = 0;
        self.validity_byte = 0;
        self.validity_byte_idx = 0;
    }

    unsafe fn write_to_batch(&mut self, batch: *mut super::NativeBatch) {
        if self.validity_byte_idx != 0 {
            self.force_push_validity();
        }
        let base_ptr = self.raw_buffer.as_ptr();
        let values_ptr = unsafe { base_ptr.add(self.values_start) };

        (*batch).write(
            [values_ptr, base_ptr],
            [self.values_count * 4, self.current_validity as u32],
            [0],
            [1],
            [0],
            [self.null_count],
            self.values_count,
        )
    }
}
