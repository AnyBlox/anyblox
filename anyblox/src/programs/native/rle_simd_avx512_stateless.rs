use super::{NativeBatch, NativeImpl};
use crate::ColumnProjection;
use std::ptr;

#[derive(Default)]
pub struct NativeRleSimdAvx512StatelessImpl {
    state: Option<State>,
}

struct State {
    batch_capsule: BatchCapsule,
}

struct BatchCapsule {
    batch: super::NativeBatch,
    buffer_array: [*mut u8; 2],
    inner_batch: NativeBatch,
    children_array: [*mut NativeBatch; 1],
    top_level_buffer_array: [*mut NativeBatch; 1],
}

impl NativeImpl for NativeRleSimdAvx512StatelessImpl {
    fn anyblox_decode(
        &mut self,
        data: &[u8],
        start_tuple: usize,
        tuple_count: usize,
        _projection: ColumnProjection,
    ) -> &NativeBatch {
        unsafe { self.anyblox_decode_impl(data, start_tuple, tuple_count) }
    }
}

impl NativeRleSimdAvx512StatelessImpl {
    #[target_feature(enable = "avx512f")]
    unsafe fn anyblox_decode_impl(&mut self, data: &[u8], start_tuple: usize, tuple_count: usize) -> &NativeBatch {
        let data_length = data.len() / size_of::<u32>();
        let run_count = data_length / 2;
        let data = data.as_ptr().cast::<u32>();

        let state = self.state.get_or_insert_with(|| State {
            batch_capsule: BatchCapsule::new(),
        });
        let mut run_idx = 0;
        let mut next_tuple = 0;
        let mut buffer = Int32ColumnBuffer::with_capacity(tuple_count);

        unsafe {
            let values_ptr = data.add(run_count);

            if next_tuple != start_tuple {
                seek(start_tuple, data, run_count, &mut run_idx, &mut next_tuple);
            }

            let mut rem_tuples = tuple_count;
            while run_idx < data_length {
                let run_len = (data.add(run_idx).read() as usize) - next_tuple;
                let elem_ptr = values_ptr.add(run_idx);
                if run_len >= rem_tuples {
                    buffer.write_times(elem_ptr, rem_tuples);
                    break;
                } else {
                    buffer.write_times(elem_ptr, run_len);
                    next_tuple += run_len;
                    run_idx += 1;
                    rem_tuples -= run_len;
                }
            }

            buffer.write_to(&mut state.batch_capsule);
            &state.batch_capsule.batch
        }
    }
}

unsafe fn seek(tuple_idx: usize, data: *const u32, run_count: usize, run_idx: &mut usize, next_tuple: &mut usize) {
    let mut start = *run_idx;
    let mut end = run_count;

    while start < end {
        let mid = (start + end) / 2;
        let last_tuple_in_run = data.add(mid).read();
        if last_tuple_in_run > tuple_idx as u32 {
            end = mid;
        } else {
            start = mid + 1;
        }
    }

    *run_idx = start;
    *next_tuple = tuple_idx;
}

struct Int32ColumnBuffer {
    ptr: *mut u8,
    capacity: usize,
    len: usize,
}

impl Int32ColumnBuffer {
    pub fn with_capacity(count: usize) -> Self {
        let len = count * size_of::<i32>();
        let capacity = core::cmp::max(len, 64) + 64;

        let ptr = unsafe {
            let layout = core::alloc::Layout::from_size_align_unchecked(capacity, 64);
            std::alloc::alloc(layout)
        };

        Self { ptr, capacity, len: 0 }
    }

    #[target_feature(enable = "avx512f")]
    pub unsafe fn write_times(&mut self, val_ptr: *const u32, times: usize) {
        use core::arch::x86_64::{_mm512_set1_epi32, _mm512_storeu_epi32};
        let elem = val_ptr.read();
        let elem_v = unsafe { _mm512_set1_epi32(elem as i32) };
        let mut rem_times = times as i32;

        while rem_times > 0 {
            unsafe { _mm512_storeu_epi32(self.ptr.add(self.len * 4).cast(), elem_v) };
            self.len += 16;
            rem_times -= 16;
        }

        // Fixup any overflowing writes.
        if rem_times < 0 {
            let overflow = -rem_times;
            self.len -= overflow as usize;
        }
    }

    unsafe fn write_to(&self, capsule: &mut BatchCapsule) {
        capsule.batch.length = self.len as u32;
        capsule.batch.null_count = 0;
        capsule.batch.n_buffers = 1;
        capsule.batch.n_children = 1;
        capsule.batch.buffers = capsule.top_level_buffer_array.as_mut_ptr().cast();
        capsule.batch.children = capsule.children_array.as_mut_ptr().cast();
        capsule.batch.dictionary = ptr::null_mut();

        capsule.children_array[0] = &raw mut capsule.inner_batch;

        capsule.buffer_array[0] = ptr::null_mut();
        capsule.buffer_array[1] = self.ptr;

        capsule.inner_batch.length = self.len as u32;
        capsule.inner_batch.null_count = 0;
        capsule.inner_batch.n_buffers = 2;
        capsule.inner_batch.n_children = 0;
        capsule.inner_batch.buffers = capsule.buffer_array.as_mut_ptr().cast();
        capsule.inner_batch.children = ptr::null_mut();
        capsule.inner_batch.dictionary = ptr::null_mut();
    }
}

impl Drop for Int32ColumnBuffer {
    fn drop(&mut self) {
        unsafe {
            let layout = core::alloc::Layout::from_size_align_unchecked(self.capacity, 64);
            std::alloc::dealloc(self.ptr, layout);
        }
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
}
