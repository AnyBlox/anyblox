const FSST_ESCAPE: u8 = 255;
const ESCAPE_PROBABILITY: f32 = 0.1;

pub struct FsstInput {
    pub table: SymbolTable,
    pub payload: Vec<u8>,
    pub raw_len: usize,
}

pub struct SymbolTable {
    pub bytes_table: [u64; 255],
    pub lens_table: [u8; 255],
}

pub struct Symbol {
    bytes: u64,
    len: u8,
}

impl Symbol {
    #[must_use]
    pub fn len(&self) -> usize {
        self.len as usize
    }

    pub fn byte_at(&self, i: usize) -> u8 {
        ((self.bytes >> (i * 8)) & 0xFF) as u8
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

impl SymbolTable {
    pub fn at(&self, idx: u8) -> Symbol {
        Symbol {
            bytes: self.bytes_table[idx as usize],
            len: self.lens_table[idx as usize],
        }
    }
}

pub fn gen_input(size: usize) -> FsstInput {
    use rand::{distributions::Uniform, rngs::StdRng, Rng, SeedableRng};
    let mut rng = StdRng::from_seed([42; 32]);
    let sym_len_d = Uniform::new_inclusive(1, 8);
    let sym_code_d = Uniform::new_inclusive(1, 254);

    // Generating a "random" string that compresses well with FSST is challenging,
    // as uniform distributions will just result in random noise and few patterns.
    // We pull a switcheroo - generate a nice FSST table, then a sequence that looks
    // like an FSST compressed string with desired properties, and construct the "original"
    // string from that.
    let mut symbol_table = SymbolTable {
        bytes_table: [0; 255],
        lens_table: [0; 255],
    };

    for i in 0..255 {
        let len = rng.sample(sym_len_d);
        symbol_table.lens_table[i] = len;
        symbol_table.bytes_table[i] = rng.gen();
        symbol_table.bytes_table[i] &= 0xFFFFFFFFFFFFFFFF << (len * 8);
    }

    let mut payload = Vec::with_capacity(size);
    let mut raw_len = 0;
    let mut i = 0;

    while i + 1 < size {
        if rng.gen::<f32>() < ESCAPE_PROBABILITY {
            payload.push(FSST_ESCAPE);
            payload.push(rng.gen());
            raw_len += 1;
            i += 2;
        } else {
            let b = rng.sample(sym_code_d);
            payload.push(b);
            raw_len += symbol_table.at(b).len as usize;
            i += 1;
        }
    }
    let b = rng.sample(sym_code_d);
    payload.push(b);
    raw_len += symbol_table.at(b).len as usize;

    FsstInput {
        table: symbol_table,
        payload,
        raw_len,
    }
}

pub fn implementation(input: &FsstInput, output: &mut [u8]) {
    let mut in_idx = 0;
    let mut out_idx = 0;
    let input_ptr = input.payload.as_ptr();
    let output_ptr = output.as_mut_ptr();

    while in_idx < input.payload.len() && out_idx + 8 <= output.len() {
        let val = unsafe { *input_ptr.add(in_idx) };

        if val == FSST_ESCAPE {
            let elem = unsafe { *input_ptr.add(in_idx + 1) };
            in_idx += 2;
            unsafe { *output_ptr.add(out_idx) = elem };
            out_idx += 1;
        } else {
            let symbol = &input.table.at(val);
            unsafe { output_ptr.add(out_idx).cast::<u64>().write_unaligned(symbol.bytes) };
            in_idx += 1;
            out_idx += symbol.len();
        }
    }

    // The rest.
    while in_idx < input.payload.len() {
        let val = unsafe { *input_ptr.add(in_idx) };

        if val == FSST_ESCAPE {
            let elem = unsafe { *input_ptr.add(in_idx + 1) };
            in_idx += 2;
            unsafe { *output_ptr.add(out_idx) = elem };
            out_idx += 1;
        } else {
            let symbol = &input.table.at(val);
            for i in 0..symbol.len() {
                unsafe { output_ptr.add(out_idx + i).write(symbol.byte_at(i)) };
            }
            in_idx += 1;
            out_idx += symbol.len();
        }
    }
}

pub fn check_result(input: &FsstInput, output: &[u8]) {
    let mut in_idx = 0;
    let mut out_idx = 0;

    while in_idx < input.payload.len() {
        let val = input.payload[in_idx];

        if val == FSST_ESCAPE {
            let elem = input.payload[in_idx + 1];
            in_idx += 2;
            assert_eq!(output[out_idx], elem, "mismatch in escape");
            out_idx += 1;
        } else {
            let symbol = &input.table.at(val);
            for i in 0..symbol.len() {
                let b = symbol.byte_at(i);
                assert_eq!(output[out_idx + i], b, "mismatch in byte {b} of symbol {val}");
            }
            in_idx += 1;
            out_idx += symbol.len();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn baseline_1() {
        let data = [0, FSST_ESCAPE, 0x88, 7, 129];
        let mut table = SymbolTable {
            bytes_table: [0; 255],
            lens_table: [0; 255],
        };
        table.bytes_table[0] = 0x000000000000A69A;
        table.lens_table[0] = 2;
        table.bytes_table[7] = 0x0000000000BB42BB;
        table.lens_table[7] = 3;
        table.bytes_table[129] = 0xDEADBEEF0BAD1DEA;
        table.lens_table[129] = 8;

        let expected = [
            0x9A, 0xA6, 0x88, 0xBB, 0x42, 0xBB, 0xEA, 0x1D, 0xAD, 0x0b, 0xEF, 0xBE, 0xAD, 0xDE,
        ];
        let fsst_input = FsstInput {
            payload: data.to_vec(),
            table,
            raw_len: expected.len(),
        };

        let mut decoded = [0_u8; 14];

        super::implementation(&fsst_input, &mut decoded);

        assert_eq!(decoded, expected);
        check_result(&fsst_input, &decoded);
    }

    #[test]
    fn baseline_2() {
        let data = [0, 0, 1, 1, 2, 2, 3, 3];
        let mut table = SymbolTable {
            bytes_table: [0; 255],
            lens_table: [0; 255],
        };
        table.bytes_table[0] = 0x00000000000000AB;
        table.lens_table[0] = 1;
        table.bytes_table[1] = 0xA0B0C0D0E0F01234;
        table.lens_table[1] = 8;
        table.bytes_table[2] = 0x00000000000000CD;
        table.lens_table[2] = 1;
        table.bytes_table[3] = 0x0102030405060708;
        table.lens_table[3] = 8;

        let expected = [
            0xAB, 0xAB, 0x34, 0x12, 0xF0, 0xE0, 0xD0, 0xC0, 0xB0, 0xA0, 0x34, 0x12, 0xF0, 0xE0, 0xD0, 0xC0, 0xB0, 0xA0,
            0xCD, 0xCD, 0x08, 0x07, 0x06, 0x05, 0x04, 0x03, 0x02, 0x01, 0x08, 0x07, 0x06, 0x05, 0x04, 0x03, 0x02, 0x01,
        ];
        let fsst_input = FsstInput {
            payload: data.to_vec(),
            table,
            raw_len: expected.len(),
        };

        let mut decoded = [0_u8; 36];

        super::implementation(&fsst_input, &mut decoded);

        assert_eq!(decoded, expected);
        check_result(&fsst_input, &decoded);
    }

    #[test]
    fn baseline_3() {
        let data = [0, 1, 2];
        let mut table = SymbolTable {
            bytes_table: [0; 255],
            lens_table: [0; 255],
        };
        table.bytes_table[0] = 0x000000000000ABCD;
        table.lens_table[0] = 2;
        table.bytes_table[1] = 0x0000000000001234;
        table.lens_table[1] = 2;
        table.bytes_table[2] = 0x0000000000005678;
        table.lens_table[2] = 2;

        let expected = [0xCD, 0xAB, 0x34, 0x12, 0x78, 0x56];
        let fsst_input = FsstInput {
            payload: data.to_vec(),
            table,
            raw_len: expected.len(),
        };

        let mut decoded = [0_u8; 6];

        super::implementation(&fsst_input, &mut decoded);

        assert_eq!(decoded, expected);
        check_result(&fsst_input, &decoded);
    }
}
