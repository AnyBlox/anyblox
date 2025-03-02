use pretty_assertions::assert_eq;
use rand::distributions::Distribution;
use std::arch::x86_64::*;

pub struct IntForInput {
    pub data: Vec<u8>,
    pub raw_len: usize,
}

pub fn gen_input(len: usize) -> IntForInput {
    use rand::{distributions::Uniform, rngs::StdRng, Rng, SeedableRng};
    const INTS_IN_BLOCK: usize = 256;

    let mut rng = StdRng::from_seed([42; 32]);
    let min_d = Uniform::new_inclusive(i32::MIN, i32::MAX);
    let size_d = Uniform::new_inclusive(0, 2);

    let mut input = Vec::with_capacity(len);
    let mut rem_len = len;
    let mut raw_len = 0;

    input.extend_from_slice(&(INTS_IN_BLOCK as u32).to_le_bytes());
    rem_len -= 4;

    while rem_len > 0 {
        assert!(rem_len > 8);
        let min = min_d.sample(&mut rng);
        let ub = i32::MAX as i64 - min as i64;
        let ub = u32::try_from(ub).unwrap();
        let size = size_d.sample(&mut rng);

        input.extend_from_slice(&min.to_le_bytes());
        rem_len -= 8;

        match size {
            0 => {
                input.extend_from_slice(&8_i32.to_le_bytes());
                let rem = std::cmp::min(rem_len, INTS_IN_BLOCK);
                let zero_idx = rng.gen_range(0..rem);
                let ub = std::cmp::min(ub, u8::MAX as u32) as u8;
                let elem_d = Uniform::new_inclusive(0, ub);

                for i in 0..rem {
                    if i == zero_idx {
                        input.push(0);
                    } else {
                        let elem = elem_d.sample(&mut rng);
                        input.push(elem);
                    }
                }
                rem_len -= rem;
                raw_len += rem;
            }
            1 => {
                input.extend_from_slice(&16_i32.to_le_bytes());
                let rem = std::cmp::min(rem_len / 2, INTS_IN_BLOCK);
                let zero_idx = rng.gen_range(0..rem);
                let ub = std::cmp::min(ub, u16::MAX as u32) as u16;
                let elem_d = Uniform::new_inclusive(0, ub);

                for i in 0..rem {
                    if i == zero_idx {
                        input.extend_from_slice(&[0, 0]);
                    } else {
                        let elem = elem_d.sample(&mut rng);
                        input.extend_from_slice(&elem.to_le_bytes());
                    }
                }
                rem_len -= rem * 2;
                raw_len += rem;
            }
            _ => {
                input.extend_from_slice(&32_i32.to_le_bytes());
                let rem = std::cmp::min(rem_len / 4, INTS_IN_BLOCK);
                let zero_idx = rng.gen_range(0..rem);
                let elem_d = Uniform::new_inclusive(0, ub);

                for i in 0..rem {
                    if i == zero_idx {
                        input.extend_from_slice(&[0, 0, 0, 0]);
                    } else {
                        let elem = elem_d.sample(&mut rng);
                        input.extend_from_slice(&elem.to_le_bytes());
                    }
                }
                rem_len -= rem * 4;
                raw_len += rem;
            }
        }
    }

    IntForInput { data: input, raw_len }
}

pub fn check_result(input: &[u8], output: &[i32]) {
    let ints_in_block = u32::from_le_bytes(input[..4].try_into().unwrap()) as usize;
    let mut i = 4;
    let mut ints = vec![];

    while i < input.len() {
        let min = i32::from_le_bytes(input[i..i + 4].try_into().unwrap());
        i += 4;
        let size = u32::from_le_bytes(input[i..i + 4].try_into().unwrap()) as usize;
        i += 4;
        let actual_in_block = std::cmp::min(input[i..].len() * 8 / size, ints_in_block);

        match size {
            8 => {
                for &byte in &input[i..i + actual_in_block] {
                    let int = i32::from(byte);
                    ints.push(int + min);
                }
                i += actual_in_block;
            }
            16 => {
                for bytes in input[i..i + actual_in_block * 2].chunks(2) {
                    let short = u16::from_le_bytes(bytes.try_into().unwrap());
                    ints.push(i32::from(short) + min);
                }
                i += actual_in_block * 2;
            }
            32 => {
                for bytes in input[i..i + actual_in_block * 4].chunks(4) {
                    let int = u32::from_le_bytes(bytes.try_into().unwrap());
                    ints.push(i32::try_from(int as i64 + min as i64).expect("invalid data"));
                }
                i += actual_in_block * 4;
            }
            _ => unreachable!(),
        }
    }

    assert_eq!(output, ints)
}

pub fn implementation(input: &[u8], output: &mut [i32]) {
    let input_ptr = input.as_ptr();
    let mut in_offset = 0;
    let mut output_ptr = output.as_mut_ptr();

    let ints_in_block = unsafe { input_ptr.cast::<u32>().read_unaligned() } as usize;
    in_offset += 4;

    while in_offset < input.len() {
        unsafe {
            let min = input_ptr.add(in_offset).cast::<i32>().read_unaligned();
            in_offset += 4;
            let size = input_ptr.add(in_offset).cast::<u32>().read_unaligned() as usize;
            in_offset += 4;
            let rem = input.len() - in_offset;

            let actual_in_block = std::cmp::min(rem * 8 / size, ints_in_block);

            match size {
                8 => decode_block_8(input_ptr, &mut in_offset, &mut output_ptr, min, actual_in_block),
                16 => decode_block_16(input_ptr, &mut in_offset, &mut output_ptr, min, actual_in_block),
                32 => decode_block_32(input_ptr, &mut in_offset, &mut output_ptr, min, actual_in_block),
                _ => unreachable!(),
            }
        }
    }
}

pub fn simd_implementation(input: &[u8], output: &mut [i32]) {
    unsafe { simd_impl(input, output) }

    #[target_feature(enable = "avx512f")]
    #[target_feature(enable = "avx512vbmi2")]
    unsafe fn simd_impl(input: &[u8], output: &mut [i32]) {
        let input_ptr = input.as_ptr();
        let mut in_offset = 0;
        let mut output_ptr = output.as_mut_ptr();

        let ints_in_block = unsafe { input_ptr.cast::<u32>().read_unaligned() } as usize;
        in_offset += 4;

        while in_offset < input.len() {
            unsafe {
                let min = input_ptr.add(in_offset).cast::<i32>().read_unaligned();
                in_offset += 4;
                let size = input_ptr.add(in_offset).cast::<u32>().read_unaligned() as usize;
                in_offset += 4;
                let rem = input.len() - in_offset;

                let actual_in_block = std::cmp::min(rem * 8 / size, ints_in_block);

                if actual_in_block < ints_in_block {
                    match size {
                        8 => decode_block_8(input_ptr, &mut in_offset, &mut output_ptr, min, actual_in_block),
                        16 => decode_block_16(input_ptr, &mut in_offset, &mut output_ptr, min, actual_in_block),
                        32 => decode_block_32(input_ptr, &mut in_offset, &mut output_ptr, min, actual_in_block),
                        _ => unreachable!(),
                    }
                } else {
                    match size {
                        8 => simd_decode_block_8(input_ptr, &mut in_offset, &mut output_ptr, min, actual_in_block),
                        16 => simd_decode_block_16(input_ptr, &mut in_offset, &mut output_ptr, min, actual_in_block),
                        32 => simd_decode_block_32(input_ptr, &mut in_offset, &mut output_ptr, min, actual_in_block),
                        _ => unreachable!(),
                    }
                }
            }
        }
    }
}

unsafe fn decode_block_8(
    input_ptr: *const u8,
    in_offset: &mut usize,
    output_ptr: &mut *mut i32,
    min: i32,
    ints_in_block: usize,
) {
    for _ in 0..ints_in_block {
        let b = input_ptr.add(*in_offset).read();
        let int = b as i64 + min as i64;
        output_ptr.write_unaligned(int as i32);
        *in_offset += 1;
        *output_ptr = output_ptr.add(1);
    }
}

unsafe fn decode_block_16(
    input_ptr: *const u8,
    in_offset: &mut usize,
    output_ptr: &mut *mut i32,
    min: i32,
    ints_in_block: usize,
) {
    for _ in 0..ints_in_block {
        let b = input_ptr.add(*in_offset).cast::<u16>().read_unaligned();
        let int = b as i64 + min as i64;
        output_ptr.write_unaligned(int as i32);
        *in_offset += 2;
        *output_ptr = output_ptr.add(1);
    }
}

unsafe fn decode_block_32(
    input_ptr: *const u8,
    in_offset: &mut usize,
    output_ptr: &mut *mut i32,
    min: i32,
    ints_in_block: usize,
) {
    for _ in 0..ints_in_block {
        let b = input_ptr.add(*in_offset).cast::<u32>().read_unaligned();
        let int = b as i64 + min as i64;
        output_ptr.write_unaligned(int as i32);
        *in_offset += 4;
        *output_ptr = output_ptr.add(1);
    }
}

#[target_feature(enable = "avx512f")]
#[target_feature(enable = "avx512vbmi2")]
unsafe fn simd_decode_block_8(
    input_ptr: *const u8,
    in_offset: &mut usize,
    output_ptr: &mut *mut i32,
    min: i32,
    ints_in_block: usize,
) {
    let min = _mm512_set1_epi32(min);
    let mask = 0b0001_0001_0001_0001_0001_0001_0001_0001_0001_0001_0001_0001_0001_0001_0001_0001;

    for _ in 0..(ints_in_block / 16) {
        let v_reg = _mm512_maskz_expandloadu_epi8(mask, input_ptr.add(*in_offset).cast());
        let dec = _mm512_add_epi32(v_reg, min);
        _mm512_storeu_si512(output_ptr.cast(), dec);

        *in_offset += 16;
        *output_ptr = output_ptr.add(16);
    }
}

#[target_feature(enable = "avx512f")]
#[target_feature(enable = "avx512vbmi2")]
unsafe fn simd_decode_block_16(
    input_ptr: *const u8,
    in_offset: &mut usize,
    output_ptr: &mut *mut i32,
    min: i32,
    ints_in_block: usize,
) {
    let min = _mm512_set1_epi32(min);
    let mask = 0b0101_0101_0101_0101_0101_0101_0101_0101;

    for _ in 0..(ints_in_block / 16) {
        let v_reg = _mm512_maskz_expandloadu_epi16(mask, input_ptr.add(*in_offset).cast());
        let dec = _mm512_add_epi32(v_reg, min);
        _mm512_storeu_si512(output_ptr.cast(), dec);

        *in_offset += 32;
        *output_ptr = output_ptr.add(16);
    }
}

#[target_feature(enable = "avx512f")]
#[target_feature(enable = "avx512vbmi2")]
unsafe fn simd_decode_block_32(
    input_ptr: *const u8,
    in_offset: &mut usize,
    output_ptr: &mut *mut i32,
    min: i32,
    ints_in_block: usize,
) {
    let min = _mm512_set1_epi32(min);

    for _ in 0..(ints_in_block / 16) {
        let v_reg = _mm512_loadu_si512(input_ptr.add(*in_offset).cast());
        let dec = _mm512_add_epi32(v_reg, min);
        _mm512_storeu_si512(output_ptr.cast(), dec);

        *in_offset += 64;
        *output_ptr = output_ptr.add(16);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn baseline_8bit() {
        let data = [
            0x10, 0x00, 0x00, 0x00, 0x00, 0x33, 0x00, 0x00, 0x08, 0x00, 0x00, 0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06,
            0x07, 0x08, 0x08, 0x07, 0x06, 0x05, 0x04, 0x03, 0x02, 0x00,
        ];
        let expected = [
            13_057, 13_058, 13_059, 13_060, 13_061, 13_062, 13_063, 13_064, 13_064, 13_063, 13_062, 13_061, 13_060,
            13_059, 13_058, 13_056,
        ];

        let mut decoded = [0_i32; 16];

        super::implementation(&data, &mut decoded);

        assert_eq!(decoded, expected);
        check_result(&data, &decoded);
    }

    #[test]
    fn baseline_16bit() {
        let data = [
            0x00, 0x01, 0x00, 0x00, 0x33, 0x01, 0x00, 0x00, 0x10, 0x00, 0x00, 0x00, 0x21, 0x01, 0x42, 0x03, 0x63, 0x05,
            0x84, 0x07, 0xa5, 0x09, 0xc6, 0x0b, 0xe7, 0x0d, 0x08, 0x0f, 0x58, 0x01, 0x67, 0x02, 0x76, 0x03, 0x85, 0x04,
            0x94, 0x05, 0xa3, 0x06, 0xb2, 0x07, 0x00, 0x00,
        ];
        let expected = [
            596, 1141, 1686, 2231, 2776, 3321, 3866, 4155, 651, 922, 1193, 1464, 1735, 2006, 2277, 307,
        ];

        let mut decoded = [0_i32; 16];

        super::implementation(&data, &mut decoded);

        assert_eq!(decoded, expected);
        check_result(&data, &decoded);
    }

    #[test]
    fn baseline_32bit() {
        let data = [
            0x00, 0x01, 0x00, 0x00, 0x00, 0xcb, 0xf3, 0xff, 0x20, 0x00, 0x00, 0x00, 0xa0, 0xbb, 0x0d, 0x00, 0x40, 0x42,
            0x0f, 0x00, 0xe0, 0xc8, 0x10, 0x00, 0x80, 0x4f, 0x12, 0x00, 0x20, 0xd6, 0x13, 0x00, 0xc0, 0x5c, 0x15, 0x00,
            0x60, 0xe3, 0x16, 0x00, 0x00, 0x6a, 0x18, 0x00, 0x60, 0xae, 0x0a, 0x00, 0xc0, 0x27, 0x09, 0x00, 0x20, 0xa1,
            0x07, 0x00, 0x80, 0x1a, 0x06, 0x00, 0xe0, 0x93, 0x04, 0x00, 0x40, 0x0d, 0x03, 0x00, 0xa0, 0x86, 0x01, 0x00,
            0x00, 0x00, 0x00, 0x00,
        ];
        let expected = [
            100_000, 200_000, 300_000, 400_000, 500_000, 600_000, 700_000, 800_000, -100_000, -200_000, -300_000,
            -400_000, -500_000, -600_000, -700_000, -800_000,
        ];

        let mut decoded = [0_i32; 16];

        super::implementation(&data, &mut decoded);

        assert_eq!(decoded, expected);
        check_result(&data, &decoded);
    }
}
