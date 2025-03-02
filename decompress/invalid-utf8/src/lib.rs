#![no_std]
use arrow::ArrowArray;
use decoder_lib::*;
use ffi_utils::WasmPtr;

static mut BATCH: ArrowArray = ArrowArray::empty();
static DATA: [u8; 4] = [0xf0, 0x90, 0x8c, 0xbc];
static OFFSETS: [u32; 3] = [0, 2, 4];

#[no_mangle]
pub unsafe extern "C" fn anyblox_decode(
    _data: *const u32,
    _data_length: usize,
    start_tuple: usize,
    tuple_count: usize,
    _state: *mut u8,
    _projection_mask_1: u32,
    _projection_mask_2: u32,
) -> *const u8 {
    static mut BUFFER_ARRAY: [WasmPtr; 3] = [WasmPtr::NULL, WasmPtr::NULL, WasmPtr::NULL];
    static mut INNER_BATCH: ArrowArray = ArrowArray::empty();
    static mut CHILDREN_ARRAY: [WasmPtr; 1] = [WasmPtr::NULL];
    static mut TOP_LEVEL_BUFFER_ARRAY: [WasmPtr; 1] = [WasmPtr::NULL];

    let (data, offsets) = if start_tuple == 0 && tuple_count >= 2 {
        (&DATA[..], &OFFSETS[..])
    } else if start_tuple == 0 && tuple_count == 1 {
        (&DATA[..2], &OFFSETS[..2])
    } else if start_tuple == 1 && tuple_count == 1 {
        (&DATA[2..], &OFFSETS[..2])
    } else {
        (&DATA[..0], &OFFSETS[..1])
    };

    CHILDREN_ARRAY[0] = WasmPtr::from(&raw const INNER_BATCH as u32);
    BUFFER_ARRAY[0] = WasmPtr::NULL;
    BUFFER_ARRAY[1] = offsets.as_ptr().cast::<u8>().into();
    BUFFER_ARRAY[2] = data.as_ptr().into();

    BATCH.length = offsets.len() as u32 - 1;
    BATCH.null_count = 0;
    BATCH.n_buffers = 1;
    BATCH.n_children = 1;
    BATCH.buffers = WasmPtr::from((&raw mut TOP_LEVEL_BUFFER_ARRAY) as u32);
    BATCH.children = WasmPtr::from((&raw mut CHILDREN_ARRAY) as u32);
    BATCH.dictionary = WasmPtr::NULL;

    INNER_BATCH.length = offsets.len() as u32 - 1;
    INNER_BATCH.null_count = 0;
    INNER_BATCH.n_buffers = 3;
    INNER_BATCH.n_children = 0;
    INNER_BATCH.buffers = WasmPtr::from((&raw mut BUFFER_ARRAY) as u32);
    INNER_BATCH.children = WasmPtr::NULL;
    INNER_BATCH.dictionary = WasmPtr::NULL;

    let ptr: *const ArrowArray = &raw const BATCH;
    ptr.cast::<u8>()
}
