pub fn gen_input(size: usize) -> Vec<u8> {
    use rand::{distributions::Uniform, rngs::StdRng, Rng, SeedableRng};
    let rng = StdRng::from_seed([42; 32]);

    rng.sample_iter(Uniform::new_inclusive(u8::MIN, u8::MAX))
        .take(size)
        .collect()
}

pub fn check_result(input: &[u8], output: &[u32]) {
    assert_eq!(4 * input.len(), output.len());

    for (i, b) in input.iter().enumerate() {
        let first = b & 0b11000000;
        let second = b & 0b00110000;
        let third = b & 0b00001100;
        let fourth = b & 0b00000011;

        assert_eq!(output[4 * i], u32::from(first >> 6));
        assert_eq!(output[4 * i + 1], u32::from(second >> 4));
        assert_eq!(output[4 * i + 2], u32::from(third >> 2));
        assert_eq!(output[4 * i + 3], u32::from(fourth));
    }
}

pub fn implementation(input: &[u8], output: &mut [u32]) {
    let input_ptr = input.as_ptr();
    let output_ptr = output.as_mut_ptr();
    for i in 0..input.len() {
        let b = unsafe { *input_ptr.add(i) };
        let first = b & 0b11000000;
        let second = b & 0b00110000;
        let third = b & 0b00001100;
        let fourth = b & 0b00000011;

        unsafe {
            *output_ptr.add(4 * i) = u32::from(first >> 6);
            *output_ptr.add(4 * i + 1) = u32::from(second >> 4);
            *output_ptr.add(4 * i + 2) = u32::from(third >> 2);
            *output_ptr.add(4 * i + 3) = u32::from(fourth);
        }
    }
}

pub fn simd_implementation(input: &[u8], output: &mut [u32]) {
    use std::arch::x86_64::*;
    const BYTE_MASK: [u8; 16] = [
        0b11000000, 0b00110000, 0b00001100, 0b00000011, 0b11000000, 0b00110000, 0b00001100, 0b00000011, 0b11000000,
        0b00110000, 0b00001100, 0b00000011, 0b11000000, 0b00110000, 0b00001100, 0b00000011,
    ];
    const SHIFT_MASK: [u32; 16] = [6, 4, 2, 0, 6, 4, 2, 0, 6, 4, 2, 0, 6, 4, 2, 0];

    unsafe { simd_impl(input, output) }

    #[target_feature(enable = "avx512f")]
    unsafe fn simd_impl(input: &[u8], output: &mut [u32]) {
        let input_ptr = input.as_ptr();
        let output_ptr = output.as_mut_ptr();
        let bitmask = _mm_loadu_si128(BYTE_MASK.as_ptr().cast());
        let shiftmask = _mm512_loadu_si512(SHIFT_MASK.as_ptr().cast());
        let mut i = 0;
        let mut buf = [0; 16];

        while i < input.len() {
            for j in 0..4 {
                let x = *input_ptr.add(i + j);
                for k in 0..4 {
                    buf[j * 4 + k] = x;
                }
            }

            let b_v = unsafe { _mm_loadu_si128(buf.as_ptr().cast()) };
            let b_v2 = unsafe { _mm_and_si128(b_v, bitmask) };
            let u32_v = unsafe { _mm512_cvtepu8_epi32(b_v2) };
            let u32_v2 = unsafe { _mm512_srlv_epi32(u32_v, shiftmask) };

            unsafe { _mm512_storeu_epi32(output_ptr.add(4 * i).cast(), u32_v2) };
            i += 4;
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::packing::check_result;

    #[test]
    fn baseline() {
        let data = [0b00011011_u8, 0b01010101, 0b11111111, 0b01100110];
        let expected = [0_u32, 1, 2, 3, 1, 1, 1, 1, 3, 3, 3, 3, 1, 2, 1, 2];

        let mut decoded = [0_u32; 4 * 4];

        super::implementation(&data, &mut decoded);

        assert_eq!(decoded, expected);
        check_result(&data, &decoded);
    }
}
