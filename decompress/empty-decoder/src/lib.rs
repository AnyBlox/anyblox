#![no_std]
use decoder_lib::arrow::ArrowArray;

#[no_mangle]
#[target_feature(enable = "simd128")]
pub unsafe extern "C" fn anyblox_decode(
    _data: *const u8,
    _data_length: usize,
    _start_tuple: usize,
    _tuple_count: usize,
    _state: *mut u8,
    _projection_mask_1: u32,
    _projection_mask_2: u32,
) -> *const u8 {
    let ptr: *const ArrowArray = &raw const BATCH;
    ptr.cast::<u8>()
}

static mut BATCH: ArrowArray = ArrowArray::empty();