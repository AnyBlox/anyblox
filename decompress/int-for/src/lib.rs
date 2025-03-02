#![no_std]
use decoder_lib::*;

#[no_mangle]
pub unsafe extern "C" fn entry(data_ptr: *const u8, data_len: usize) {
    let mut in_offset = 0;

    let ints_in_block = data_ptr.cast::<u32>().read_unaligned() as usize;
    in_offset += 4;
    let mut writer = io::BufEmitter::new();

    while in_offset < data_len {
        let min = data_ptr.add(in_offset).cast::<i32>().read_unaligned();
        in_offset += 4;
        let size = data_ptr.add(in_offset).cast::<u32>().read_unaligned() as usize;
        in_offset += 4;
        let rem = data_len - in_offset;

        let actual_in_block = core::cmp::min(rem * 8 / size, ints_in_block);

        match size {
            8 => decode_block_8(data_ptr, &mut in_offset, &mut writer, min, actual_in_block),
            16 => decode_block_16(data_ptr, &mut in_offset, &mut writer, min, actual_in_block),
            32 => decode_block_32(data_ptr, &mut in_offset, &mut writer, min, actual_in_block),
            _ => unreachable!(),
        }
    }
}

#[inline(always)]
unsafe fn decode_block_8(
    data_ptr: *const u8,
    in_offset: &mut usize,
    writer: &mut io::BufEmitter,
    min: i32,
    ints_in_block: usize,
) {
    for _ in 0..ints_in_block {
        let b = data_ptr.add(*in_offset).read();
        let int = b as i64 + min as i64;
        writer.write(int as i32);
        *in_offset += 1;
    }
}

#[inline(always)]
unsafe fn decode_block_16(
    data_ptr: *const u8,
    in_offset: &mut usize,
    writer: &mut io::BufEmitter,
    min: i32,
    ints_in_block: usize,
) {
    for _ in 0..ints_in_block {
        let b = data_ptr.add(*in_offset).cast::<u16>().read_unaligned();
        let int = b as i64 + min as i64;
        writer.write(int as i32);
        *in_offset += 2;
    }
}

#[inline(always)]
unsafe fn decode_block_32(
    data_ptr: *const u8,
    in_offset: &mut usize,
    writer: &mut io::BufEmitter,
    min: i32,
    ints_in_block: usize,
) {
    for _ in 0..ints_in_block {
        let b = data_ptr.add(*in_offset).cast::<u32>().read_unaligned();
        let int = b as i64 + min as i64;
        writer.write(int as i32);
        *in_offset += 4;
    }
}
