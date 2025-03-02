#![no_std]
use core::arch::wasm32::*;
use decoder_lib::*;

#[no_mangle]
#[target_feature(enable = "simd128")]
pub unsafe extern "C" fn entry(data_ptr: *const u8, data_len: usize) {
    let mut in_offset = 0;
    let mut buf = ForEmitter::new();

    let ints_in_block = unsafe { data_ptr.cast::<u32>().read_unaligned() } as usize;
    in_offset += 4;

    while in_offset < data_len {
        unsafe {
            let min = data_ptr.add(in_offset).cast::<i32>().read_unaligned();
            in_offset += 4;
            let size = data_ptr.add(in_offset).cast::<u32>().read_unaligned() as usize;
            in_offset += 4;
            let rem = data_len - in_offset;

            let actual_in_block = core::cmp::min(rem * 8 / size, ints_in_block);

            if actual_in_block < ints_in_block {
                match size {
                    8 => decode_block_8(data_ptr, &mut in_offset, &mut buf, min, actual_in_block),
                    16 => decode_block_16(data_ptr, &mut in_offset, &mut buf, min, actual_in_block),
                    32 => decode_block_32(data_ptr, &mut in_offset, &mut buf, min, actual_in_block),
                    _ => unreachable!(),
                }
            } else {
                match size {
                    8 => simd_decode_block_8(
                        data_ptr,
                        &mut in_offset,
                        &mut buf,
                        min,
                        actual_in_block,
                    ),
                    16 => simd_decode_block_16(
                        data_ptr,
                        &mut in_offset,
                        &mut buf,
                        min,
                        actual_in_block,
                    ),
                    32 => simd_decode_block_32(
                        data_ptr,
                        &mut in_offset,
                        &mut buf,
                        min,
                        actual_in_block,
                    ),
                    _ => unreachable!(),
                }
            }
        }
    }
}

#[inline(always)]
unsafe fn decode_block_8(
    data_ptr: *const u8,
    in_offset: &mut usize,
    writer: &mut ForEmitter,
    min: i32,
    ints_in_block: usize,
) {
    for _ in 0..ints_in_block {
        let b = data_ptr.add(*in_offset).read();
        let int = b as i64 + min as i64;
        writer.write_one(int as i32);
        *in_offset += 1;
    }
}

#[inline(always)]
unsafe fn decode_block_16(
    data_ptr: *const u8,
    in_offset: &mut usize,
    writer: &mut ForEmitter,
    min: i32,
    ints_in_block: usize,
) {
    for _ in 0..ints_in_block {
        let b = data_ptr.add(*in_offset).cast::<u16>().read_unaligned();
        let int = b as i64 + min as i64;
        writer.write_one(int as i32);
        *in_offset += 2;
    }
}

#[inline(always)]
unsafe fn decode_block_32(
    data_ptr: *const u8,
    in_offset: &mut usize,
    writer: &mut ForEmitter,
    min: i32,
    ints_in_block: usize,
) {
    for _ in 0..ints_in_block {
        let b = data_ptr.add(*in_offset).cast::<u32>().read_unaligned();
        let int = b as i64 + min as i64;
        writer.write_one(int as i32);
        *in_offset += 4;
    }
}

#[inline]
#[target_feature(enable = "simd128")]
unsafe fn simd_decode_block_8(
    input_ptr: *const u8,
    in_offset: &mut usize,
    output: &mut ForEmitter,
    min: i32,
    ints_in_block: usize,
) {
    let min = i32x4_splat(min);
    let zeroes = i32x4_splat(0);

    for _ in 0..(ints_in_block / 4) {
        // This is harder because wasm does not have an 8x4 -> 32x4 conversion.
        // We first load the 8 bytes as a single 32-bit integer and set them as the first element.
        // v_reg = [ b1 b2 b3 b4 | 0 0 0 0 | 0 0 0 0 | 0 0 0 0 ]
        let v_reg = v128_load32_zero(input_ptr.add(*in_offset).cast());
        // Now by shuffling we create the actual vector. The second vector does not matter.
        let v_reg =
            u8x16_shuffle::<0, 4, 5, 6, 1, 7, 8, 9, 2, 10, 11, 12, 3, 13, 14, 15>(v_reg, zeroes);
        let dec = i32x4_add(v_reg, min);
        output.write_vector(dec);

        *in_offset += 4;
    }
}

#[inline]
#[target_feature(enable = "simd128")]
unsafe fn simd_decode_block_16(
    input_ptr: *const u8,
    in_offset: &mut usize,
    output: &mut ForEmitter,
    min: i32,
    ints_in_block: usize,
) {
    let min = i32x4_splat(min);

    for _ in 0..(ints_in_block / 4) {
        let v_reg = i32x4_load_extend_u16x4(input_ptr.add(*in_offset).cast());
        let dec = i32x4_add(v_reg, min);
        output.write_vector(dec);

        *in_offset += 8;
    }
}

#[inline]
#[target_feature(enable = "simd128")]
unsafe fn simd_decode_block_32(
    input_ptr: *const u8,
    in_offset: &mut usize,
    output: &mut ForEmitter,
    min: i32,
    ints_in_block: usize,
) {
    let min = i32x4_splat(min);

    for _ in 0..(ints_in_block / 4) {
        let v_reg = v128_load(input_ptr.add(*in_offset).cast());
        let dec = i32x4_add(v_reg, min);
        output.write_vector(dec);

        *in_offset += 16;
    }
}

pub struct ForEmitter {
    buf: io::RawBuf,
}

impl ForEmitter {
    pub fn new() -> Self {
        Self {
            buf: io::RawBuf::new(),
        }
    }

    #[inline(always)]
    pub fn write_one(&mut self, val: i32) {
        self.buf.write_unaligned(val, core::mem::size_of::<i32>());
    }

    #[inline]
    #[target_feature(enable = "simd128")]
    pub unsafe fn write_vector(&mut self, vec: v128) {
        if self.buf.rem_capacity() < 16 {
            self.buf.flush();
        }
        v128_store(self.buf.as_mut_ptr().add(self.buf.idx()).cast(), vec);
        self.buf.offset(16);
    }
}
