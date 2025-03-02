#![no_std]
extern crate alloc;

use arrow::ArrowArray;
use core::{
    arch::wasm32::{v128_load32_splat, v128_store},
    fmt::Write,
};
use decoder_lib::*;
use ffi_utils::WasmPtr;

struct State {
    run_idx: usize,
    next_tuple: usize,
    buffer: Int32ColumnBuffer,
}

#[no_mangle]
#[target_feature(enable = "simd128")]
pub unsafe extern "C" fn anyblox_decode(
    data: *const u32,
    data_length: usize,
    start_tuple: usize,
    tuple_count: usize,
    state: *mut u8,
    _projection_mask_1: u32,
    _projection_mask_2: u32,
) -> *const u8 {
    let data_length = data_length / size_of::<u32>();
    let run_count = data_length / 2;

    let state_check = state.read();
    if state_check == 0 {
        let state_obj = State {
            run_idx: 0,
            next_tuple: 0,
            buffer: Int32ColumnBuffer::with_capacity(tuple_count),
        };
        state.write(1);
        state
            .add(align_of::<State>())
            .cast::<State>()
            .write(state_obj);
    }
    let state: &mut State = &mut *state.add(align_of::<State>()).cast::<State>();
    state.buffer.clear();
    state.buffer.reserve(tuple_count);

    let values_ptr = data.add(run_count);

    if state.next_tuple > start_tuple {
        state.run_idx = 0;
        state.next_tuple = 0;
    }

    seek(start_tuple, data, state, run_count);

    let mut rem_tuples = tuple_count;
    while state.run_idx < data_length {
        let run_len = (data.add(state.run_idx).read() as usize) - state.next_tuple;
        let elem_ptr = values_ptr.add(state.run_idx);
        if run_len >= rem_tuples {
            state.buffer.write_times(elem_ptr, rem_tuples);
            state.next_tuple += rem_tuples;
            break;
        } else {
            state.buffer.write_times(elem_ptr, run_len);
            state.next_tuple += run_len;
            state.run_idx += 1;
            rem_tuples -= run_len;
        }
    }

    state.buffer.write_to(&raw mut BATCH);
    let ptr: *const ArrowArray = &raw const BATCH;
    ptr.cast::<u8>()
}

static mut BATCH: ArrowArray = ArrowArray::empty();

unsafe fn seek(tuple_idx: usize, data: *const u32, state: &mut State, run_count: usize) {
    log!(
        "seek {} from {} (limit {})",
        tuple_idx,
        state.run_idx,
        run_count
    );
    let mut start = state.run_idx;
    let mut end = run_count;

    while start < end {
        let mid = (start + end) / 2;
        log!("binsearch iter: [{start} .. {mid} .. {end})");
        let last_tuple_in_run = data.add(mid).read();
        if last_tuple_in_run > tuple_idx as u32 {
            end = mid;
        } else {
            start = mid + 1;
        }
    }

    log!("final result: {start}");
    state.run_idx = start;
    state.next_tuple = tuple_idx;
}

struct Int32ColumnBuffer {
    ptr: *mut u8,
    capacity: usize,
    len: usize,
}

impl Int32ColumnBuffer {
    pub fn with_capacity(count: usize) -> Self {
        let len = count * size_of::<i32>();
        let capacity = core::cmp::max(len, 64) + 16;

        let ptr = unsafe {
            let layout = core::alloc::Layout::from_size_align_unchecked(capacity, 64);
            ::alloc::alloc::alloc(layout)
        };

        Self {
            ptr,
            capacity,
            len: 0,
        }
    }

    pub fn reserve(&mut self, count: usize) {
        let new_capacity = count * size_of::<i32>() + 16;
        if new_capacity <= self.capacity {
            return;
        }

        let new_ptr = unsafe {
            let old_layout = core::alloc::Layout::from_size_align_unchecked(self.capacity, 64);
            ::alloc::alloc::realloc(self.ptr, old_layout, new_capacity)
        };

        self.ptr = new_ptr;
        self.capacity = new_capacity;
    }

    #[target_feature(enable = "simd128")]
    pub unsafe fn write_times(&mut self, val_ptr: *const u32, times: usize) {
        let elem_v = unsafe { v128_load32_splat(val_ptr) };
        let mut rem_times = times as i32;

        while rem_times > 0 {
            unsafe { v128_store(self.ptr.add(self.len * 4).cast(), elem_v) };
            self.len += 4;
            rem_times -= 4;
        }

        // Fixup any overflowing writes.
        if rem_times < 0 {
            let overflow = -rem_times;
            self.len -= overflow as usize;
        }
    }

    fn clear(&mut self) {
        self.len = 0;
    }

    unsafe fn write_to(&self, batch: *mut ArrowArray) {
        static mut BUFFER_ARRAY: [WasmPtr; 2] = [WasmPtr::NULL, WasmPtr::NULL];
        static mut INNER_BATCH: ArrowArray = ArrowArray::empty();
        static mut CHILDREN_ARRAY: [WasmPtr; 1] = [WasmPtr::NULL];
        static mut TOP_LEVEL_BUFFER_ARRAY: [WasmPtr; 1] = [WasmPtr::NULL];

        CHILDREN_ARRAY[0] = WasmPtr::from(&raw const INNER_BATCH as u32);
        BUFFER_ARRAY[1] = self.ptr.into();

        (*batch).length = self.len as u32;
        (*batch).null_count = 0;
        (*batch).n_buffers = 1;
        (*batch).n_children = 1;
        (*batch).buffers = WasmPtr::from((&raw mut TOP_LEVEL_BUFFER_ARRAY) as u32);
        (*batch).children = WasmPtr::from((&raw mut CHILDREN_ARRAY) as u32);
        (*batch).dictionary = WasmPtr::NULL;

        INNER_BATCH.length = self.len as u32;
        INNER_BATCH.null_count = 0;
        INNER_BATCH.n_buffers = 2;
        INNER_BATCH.n_children = 0;
        INNER_BATCH.buffers = WasmPtr::from((&raw mut BUFFER_ARRAY) as u32);
        INNER_BATCH.children = WasmPtr::NULL;
        INNER_BATCH.dictionary = WasmPtr::NULL;
    }
}

impl Drop for Int32ColumnBuffer {
    fn drop(&mut self) {
        unsafe {
            let layout = core::alloc::Layout::from_size_align_unchecked(self.capacity, 64);
            ::alloc::alloc::dealloc(self.ptr, layout);
        }
    }
}
