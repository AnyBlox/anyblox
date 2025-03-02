use std::ptr;

use super::{NativeBatch, NativeImpl};
use crate::ColumnProjection;

#[derive(Default)]
pub struct NativeRleImpl {
    state: Option<State>,
}

struct State {
    run_idx: usize,
    next_tuple: usize,
    buffer: Vec<i32>,
    batch: super::NativeBatch,
}

impl NativeImpl for NativeRleImpl {
    fn anyblox_decode(
        &mut self,
        data: &[u8],
        start_tuple: usize,
        tuple_count: usize,
        _projection: ColumnProjection,
    ) -> &NativeBatch {
        let state = self.state.get_or_insert_with(|| State {
            run_idx: 0,
            next_tuple: 0,
            buffer: Vec::with_capacity(tuple_count),
            batch: super::NativeBatch::empty(),
        });

        let data_length = data.len() / size_of::<u32>();
        let run_count = data_length / 2;
        let data = data.as_ptr().cast::<u32>();

        unsafe {
            let buffer = &mut state.buffer;
            buffer.clear();
            buffer.reserve(tuple_count);

            let values_ptr = data.add(run_count);

            if state.next_tuple > start_tuple {
                state.run_idx = 0;
                state.next_tuple = 0;
            }

            if state.next_tuple != start_tuple {
                seek(start_tuple, data, state, run_count);
            }

            let mut rem_tuples = tuple_count;
            while state.run_idx < data_length {
                let run_len = (data.add(state.run_idx).read() as usize) - state.next_tuple;
                let elem = values_ptr.add(state.run_idx).read() as i32;
                if run_len >= rem_tuples {
                    for _ in 0..rem_tuples {
                        state.buffer.push(elem);
                    }
                    state.next_tuple += rem_tuples;
                    break;
                } else {
                    for _ in 0..run_len {
                        state.buffer.push(elem);
                    }
                    state.next_tuple += run_len;
                    state.run_idx += 1;
                    rem_tuples -= run_len;
                }
            }

            write(&state.buffer, &raw mut state.batch);
            &state.batch
        }
    }
}

unsafe fn seek(tuple_idx: usize, data: *const u32, state: &mut State, run_count: usize) {
    let mut start = state.run_idx;
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

    state.run_idx = start;
    state.next_tuple = tuple_idx;
}

unsafe fn write(buffer: &[i32], batch: *mut NativeBatch) {
    static mut BUFFER_ARRAY: [*mut u8; 2] = [ptr::null_mut(), ptr::null_mut()];
    static mut INNER_BATCH: NativeBatch = NativeBatch::empty();
    static mut CHILDREN_ARRAY: [*mut NativeBatch; 1] = [&raw mut INNER_BATCH];
    static mut TOP_LEVEL_BUFFER_ARRAY: [*mut NativeBatch; 1] = [ptr::null_mut()];

    BUFFER_ARRAY[1] = buffer.as_ptr().cast_mut().cast();

    (*batch).length = buffer.len() as u32;
    (*batch).null_count = 0;
    (*batch).n_buffers = 1;
    (*batch).n_children = 1;
    (*batch).buffers = (&raw mut TOP_LEVEL_BUFFER_ARRAY).cast();
    (*batch).children = (&raw mut CHILDREN_ARRAY).cast();
    (*batch).dictionary = ptr::null_mut();

    INNER_BATCH.length = buffer.len() as u32;
    INNER_BATCH.null_count = 0;
    INNER_BATCH.n_buffers = 2;
    INNER_BATCH.n_children = 0;
    INNER_BATCH.buffers = (&raw mut BUFFER_ARRAY).cast();
    INNER_BATCH.children = ptr::null_mut();
    INNER_BATCH.dictionary = ptr::null_mut();
}
