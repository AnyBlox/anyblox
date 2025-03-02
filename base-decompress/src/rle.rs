const MAX_RUN_LENGTH: u32 = 32;
const SINGLETON_PROBABILITY: f32 = 0.5;

pub struct RleInput {
    pub data: Vec<u32>,
    pub raw_len: usize,
}

impl RleInput {
    pub fn data_bytes(&self) -> &[u8] {
        unsafe { std::slice::from_raw_parts(self.data.as_ptr().cast::<u8>(), self.data.len() * 4) }
    }
}

fn encode_run_len(x: u32) -> u32 {
    x | 0x80000000
}

fn decode_run_len(x: u32) -> u32 {
    x & 0x7FFFFFFF
}

fn is_encoded_run_len(x: u32) -> bool {
    x > 0x7FFFFFFF
}

pub fn gen_input(len: usize) -> RleInput {
    use rand::{distributions::Uniform, rngs::StdRng, Rng, SeedableRng};
    let mut rng = StdRng::from_seed([42; 32]);
    let elem_d = Uniform::new_inclusive(0, 0x7FFFFFFF);
    let run_d = Uniform::new_inclusive(2, MAX_RUN_LENGTH);

    let mut input = Vec::with_capacity(len);
    let mut i = 0;
    let mut raw_len = 0;

    while i + 1 < len {
        let elem = rng.sample(elem_d);
        if rng.gen::<f32>() < SINGLETON_PROBABILITY {
            input.push(elem);
            i += 1;
            raw_len += 1;
        } else {
            let run_len = rng.sample(run_d);
            input.push(encode_run_len(run_len));
            input.push(elem);
            i += 2;
            raw_len += run_len as usize;
        }
    }

    if i < len {
        // We have one element left, encode it as a singleton.
        let elem = rng.sample(elem_d);
        input.push(elem);
        raw_len += 1;
    }

    RleInput { data: input, raw_len }
}

pub fn check_result(input: &RleInput, output: &[u32]) {
    assert_eq!(input.raw_len, output.len(), "differing lengths");

    let mut in_idx = 0;
    let mut out_idx = 0;

    while in_idx < input.data.len() {
        let val = input.data[in_idx];

        if is_encoded_run_len(val) {
            let run_len = decode_run_len(val);
            let elem = input.data[in_idx + 1];
            in_idx += 2;
            for _ in 0..run_len {
                assert_eq!(output[out_idx], elem, "mismatch in run");
                out_idx += 1;
            }
        } else {
            // singleton
            assert_eq!(output[out_idx], val, "mismatch in singleton");
            out_idx += 1;
            in_idx += 1;
        }
    }

    assert_eq!(out_idx, output.len());
}

pub fn implementation(input: &RleInput, output: &mut [u32]) {
    let mut in_idx = 0;
    let mut out_idx = 0;
    let input_ptr = input.data.as_ptr();
    let output_ptr = output.as_mut_ptr();

    while in_idx < input.data.len() {
        let val = unsafe { *input_ptr.add(in_idx) };

        if is_encoded_run_len(val) {
            let run_len = decode_run_len(val);
            let elem = unsafe { *input_ptr.add(in_idx + 1) };
            in_idx += 2;
            for _ in 0..run_len {
                unsafe { *output_ptr.add(out_idx) = elem };
                out_idx += 1;
            }
        } else {
            // singleton
            unsafe { *output_ptr.add(out_idx) = val };
            in_idx += 1;
            out_idx += 1;
        }
    }
}

pub fn simd_implementation(input: &RleInput, output: &mut [u32]) {
    use core::arch::x86_64::*;
    unsafe { simd_impl(input, output) }

    #[target_feature(enable = "avx512f")]
    unsafe fn simd_impl(input: &RleInput, output: &mut [u32]) {
        let mut in_idx = 0;
        let mut out_idx = 0;
        let input_ptr = input.data.as_ptr();
        let output_ptr = output.as_mut_ptr();

        while in_idx < input.data.len() {
            let val = unsafe { *input_ptr.add(in_idx) };

            if is_encoded_run_len(val) {
                let run_len = decode_run_len(val);
                let elem = unsafe { *input_ptr.add(in_idx + 1) };
                let elem_v = unsafe { _mm512_set1_epi32(elem as i32) };
                in_idx += 2;
                let times = (run_len + 15) / 16;
                for i in 0..times {
                    let i = i as usize;
                    unsafe { _mm512_storeu_epi32(output_ptr.add(out_idx + i * 16).cast(), elem_v) };
                }
                out_idx += run_len as usize;
            } else {
                // singleton
                unsafe { *output_ptr.add(out_idx) = val };
                in_idx += 1;
                out_idx += 1;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn baseline_1() {
        let data = [encode_run_len(3), 42, encode_run_len(2), 43, 44];
        let expected = [42, 42, 42, 43, 43, 44];
        let rle = RleInput {
            data: data.to_vec(),
            raw_len: 6,
        };

        let mut decoded = [0_u32; 6];

        super::implementation(&rle, &mut decoded);

        assert_eq!(decoded, expected);
        check_result(&rle, &decoded);
    }

    #[test]
    fn baseline_2() {
        let data = [42, 43, 44];
        let expected = [42, 43, 44];
        let rle = RleInput {
            data: data.to_vec(),
            raw_len: 3,
        };

        let mut decoded = [0_u32; 3];

        super::implementation(&rle, &mut decoded);

        assert_eq!(decoded, expected);
        check_result(&rle, &decoded);
    }

    #[test]
    fn baseline_3() {
        let data = [3, encode_run_len(3), 2, 3, 2, encode_run_len(2), 3];
        let expected = [3, 2, 2, 2, 3, 2, 3, 3];
        let rle = RleInput {
            data: data.to_vec(),
            raw_len: 8,
        };

        let mut decoded = [0_u32; 8];

        super::implementation(&rle, &mut decoded);

        assert_eq!(decoded, expected);
        check_result(&rle, &decoded);
    }
}
