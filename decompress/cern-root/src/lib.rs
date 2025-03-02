use anyroot::anyblox::DecoderState;
use arrow::array::{Array, ArrayRef as ArrowArrayRef};
use arrow::record_batch::RecordBatch as ArrowRecordBatch;
use decoder_lib::arrow::ArrowArray;
use decoder_lib::column_projection::ColumnProjection;
use decoder_lib::ffi_utils::WasmPtr;
use std::slice;

struct State {
    batch: ArrowArray,
    buffer_arrays: [Vec<WasmPtr>; 64],
    inner_batches: [ArrowArray; 64],
    children: [WasmPtr; 64],
    top_level_buffer_array: [WasmPtr; 1],
    current_batch: Option<ArrowRecordBatch>,
    decoder_state: DecoderState,
}

#[no_mangle]
#[target_feature(enable = "simd128")]
pub unsafe extern "C" fn anyblox_decode(
    data: *const u8,
    data_length: usize,
    start_tuple: usize,
    tuple_count: usize,
    state: *mut u8,
    projection_mask_1: u32,
    projection_mask_2: u32,
) -> *const u8 {
    std::panic::set_hook(Box::new(decoder_lib::panic_handler));
    let data_slice = unsafe { slice::from_raw_parts(data, data_length) };
    let projection = ColumnProjection::new(projection_mask_1, projection_mask_2);
    let state_check = state.read();
    if state_check == 0 {
        let state_obj = State::new(data_slice, start_tuple, tuple_count, projection);
        state.write(1);
        state.add(align_of::<State>()).cast::<State>().write(state_obj);
    }
    let state: &mut State = &mut *state.add(align_of::<State>()).cast::<State>();
    state.reset_output();

    let batch = state.decoder_state.cache.invalidate(
        data_slice,
        &state.decoder_state.file,
        start_tuple as i32,
        tuple_count as i32,
        projection.as_u64(),
    );
    state.write(batch);

    let ptr: *const ArrowArray = &raw const state.batch;
    ptr.cast::<u8>()
}

macro_rules! rep64 {
    ($e:expr) => {
        [
            $e, $e, $e, $e, $e, $e, $e, $e, $e, $e, $e, $e, $e, $e, $e, $e, $e, $e, $e, $e, $e, $e, $e, $e, $e, $e, $e,
            $e, $e, $e, $e, $e, $e, $e, $e, $e, $e, $e, $e, $e, $e, $e, $e, $e, $e, $e, $e, $e, $e, $e, $e, $e, $e, $e,
            $e, $e, $e, $e, $e, $e, $e, $e, $e, $e,
        ]
    };
}
impl State {
    fn new(data_slice: &'static [u8], start_tuple: usize, tuple_count: usize, columns: ColumnProjection) -> Self {
        let decoder_state = DecoderState::new(data_slice, start_tuple as i32, tuple_count as i32, columns.as_u64());

        Self {
            batch: ArrowArray::empty(),
            buffer_arrays: rep64!(vec![WasmPtr::NULL, WasmPtr::NULL]),
            inner_batches: rep64!(ArrowArray::empty()),
            children: [WasmPtr::NULL; 64],
            top_level_buffer_array: [WasmPtr::NULL],
            current_batch: None,
            decoder_state,
        }
    }

    fn write(&mut self, batch: ArrowRecordBatch) {
        self.batch.length = batch.num_rows() as u32;
        for col in batch.columns() {
            match col.data_type() {
                arrow::datatypes::DataType::Boolean => self.write_bool(col),
                arrow::datatypes::DataType::Int32 => self.write_primitive(col, 4),
                arrow::datatypes::DataType::UInt32 => self.write_primitive(col, 4),
                arrow::datatypes::DataType::Float64 => self.write_primitive(col, 8),
                _ => unimplemented!(),
            }
        }
        self.current_batch = Some(batch);
    }

    fn write_primitive(&mut self, array: &ArrowArrayRef, value_size: usize) {
        let idx = self.batch.n_children as usize;
        let ptr = unsafe { array.to_data().buffers()[0].as_ptr().add(value_size * array.offset()) };
        match value_size {
            4 => unsafe { byte_swap_4(ptr.cast_mut(), self.batch.length as usize) },
            8 => unsafe { byte_swap_8(ptr.cast_mut(), self.batch.length as usize) },
            _ => unreachable!(),
        }

        self.buffer_arrays[idx][0] = WasmPtr::NULL;
        self.buffer_arrays[idx][1] =
            WasmPtr::from(unsafe { array.to_data().buffers()[0].as_ptr().add(value_size * array.offset()) });

        self.inner_batches[idx] = ArrowArray {
            length: self.batch.length,
            null_count: 0,
            offset: 0,
            n_buffers: 2,
            n_children: 0,
            buffers: WasmPtr::from(self.buffer_arrays[idx].as_ptr()),
            children: WasmPtr::NULL,
            dictionary: WasmPtr::NULL,
        };

        self.children[idx] = WasmPtr::from(unsafe { self.inner_batches.as_mut_ptr().add(idx) });
        self.batch.n_children += 1;
    }

    fn write_bool(&mut self, array: &ArrowArrayRef) {
        let idx = self.batch.n_children as usize;

        self.buffer_arrays[idx][0] = WasmPtr::NULL;
        self.buffer_arrays[idx][1] = WasmPtr::from(unsafe { array.to_data().buffers()[0].as_ptr() });

        self.inner_batches[idx] = ArrowArray {
            length: self.batch.length,
            null_count: 0,
            offset: array.offset() as u32,
            n_buffers: 2,
            n_children: 0,
            buffers: WasmPtr::from(self.buffer_arrays[idx].as_ptr()),
            children: WasmPtr::NULL,
            dictionary: WasmPtr::NULL,
        };

        self.children[idx] = WasmPtr::from(unsafe { self.inner_batches.as_mut_ptr().add(idx) });
        self.batch.n_children += 1;
    }

    fn reset_output(&mut self) {
        self.batch.buffers = WasmPtr::from(self.top_level_buffer_array.as_ptr());
        self.batch.children = WasmPtr::from(self.children.as_ptr());
        self.batch.dictionary = WasmPtr::NULL;
        self.batch.length = 0;
        self.batch.n_buffers = 1;
        self.batch.n_children = 0;
        self.batch.null_count = 0;
        self.batch.offset = 0;
        self.current_batch = None;
    }
}

#[target_feature(enable = "simd128")]
unsafe fn byte_swap_4(ptr: *mut u8, length: usize) {
    use core::arch::wasm32::*;
    let mut i = 0;
    while i + 16 <= 4 * length {
        let v = v128_load(ptr.add(i).cast());
        let res = i8x16_shuffle::<3, 2, 1, 0, 7, 6, 5, 4, 11, 10, 9, 8, 15, 14, 13, 12>(v, v);
        v128_store(ptr.add(i).cast(), res);
        i += 16;
    }
    while i < 4 * length {
        let x = ptr.add(i).cast::<u32>().read();
        let y = x.swap_bytes();
        ptr.add(i).cast::<u32>().write(y);
        i += 4;
    }
}

#[target_feature(enable = "simd128")]
unsafe fn byte_swap_8(ptr: *mut u8, length: usize) {
    use core::arch::wasm32::*;
    let mut i = 0;
    while i + 16 <= 8 * length {
        let v = v128_load(ptr.add(i).cast());
        let res = i8x16_shuffle::<7, 6, 5, 4, 3, 2, 1, 0, 15, 14, 13, 12, 11, 10, 9, 8>(v, v);
        v128_store(ptr.add(i).cast(), res);
        i += 16;
    }
    while i < 8 * length {
        let x = ptr.add(i).cast::<u64>().read();
        let y = x.swap_bytes();
        ptr.add(i).cast::<u64>().write(y);
        i += 8;
    }
}