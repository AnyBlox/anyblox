//! This is a _slow_ implementation of FSST based on Boncz et al.
//! The compressed result might differ from the actual paper's algorithm,
//! because we don't use the lossy perfect hashing technique.
//!
//! Instead of speed, the algorithm errs on side of obvious correctness.
//! A side-effect of this is that we are able to achieve better compression factors,
//! as we don't trade them off for speed. This is beneficial for stress-testing decompression.
use std::{
    collections::HashMap,
    fmt::Display,
    io::{self, Write},
};

pub fn compress<S: AsRef<[u8]>, W: Write>(
    strings: &[S],
    mut output: W,
    fsst_iters: usize,
    progress_bar_style: indicatif::ProgressStyle,
    multi_progress: Option<&indicatif::MultiProgress>,
) -> Result<usize, io::Error> {
    let total_len = strings.iter().map(|x| x.as_ref().len()).sum();
    let input = Input { strings, total_len };
    let progress_bar = indicatif::ProgressBar::new(0).with_style(progress_bar_style);
    if let Some(mp) = multi_progress {
        mp.add(progress_bar.clone());
    }
    progress_bar.set_prefix("fsst compression");
    let symbol_table = SymbolTable::construct(&input, &progress_bar, fsst_iters);
    progress_bar.set_prefix("committing fsst compression");
    let result = compress_payload_with(&input, &symbol_table, &progress_bar);

    output.write_all(&symbol_table.byte_encoding())?;
    output.write_all(&result.offsets)?;
    output.write_all(&result.string_data)?;
    let written = symbol_table.byte_encoding().len() + result.offsets.len() + result.string_data.len();

    if let Some(mp) = multi_progress {
        mp.remove(&progress_bar);
        progress_bar.finish_and_clear();
    }
    Ok(written)
}

pub fn compress_blocked<W: Write>(
    data: &[u8],
    output: W,
    fsst_iters: usize,
    block_size: usize,
    progress_bar_style: indicatif::ProgressStyle,
    multi_progress: Option<&indicatif::MultiProgress>,
) -> Result<usize, io::Error> {
    let blocks = data.chunks(block_size).collect::<Vec<_>>();
    compress(&blocks, output, fsst_iters, progress_bar_style, multi_progress)
}

struct Input<'a, S> {
    strings: &'a [S],
    total_len: usize,
}

struct SymbolTable {
    symbols: [u64; 255],
    lens: [usize; 255],
    codes: HashMap<(u64, usize), u8>,
}

struct CompressionResult {
    offsets: Vec<u8>,
    string_data: Vec<u8>,
}

impl SymbolTable {
    fn empty() -> Self {
        Self {
            symbols: [0; 255],
            lens: [0; 255],
            codes: HashMap::new(),
        }
    }

    #[cfg(test)]
    fn new<S: AsRef<[u8]>, I: IntoIterator<Item = S>>(entries: I) -> Self {
        let mut table = Self::empty();
        for (i, entry) in entries.into_iter().enumerate() {
            assert!(i < 255);
            let bytes = entry.as_ref();
            let len = bytes.len();
            assert!(len <= 8);
            let mut u64_bytes = [0; 8];
            u64_bytes[..len].copy_from_slice(bytes);
            let symbol = u64::from_le_bytes(u64_bytes);

            table.symbols[i] = symbol;
            table.lens[i] = len;
            table.codes.insert((symbol, len), i as u8);
        }

        table
    }

    fn is_full(&self) -> bool {
        self.codes.len() == 255
    }

    /// The encoding has 1 + N bytes + padding + 8*N, where N is the number of symbols in the table,
    /// and padding is the amount required to have symbols aligned to the 8-byte boundary.
    /// |N: u8|len0:u8|len1:u8|...|lenN:u8|opt_padding|sym0: u64|sym1: u64|...|symN:u64|
    fn byte_encoding(&self) -> Vec<u8> {
        let tab_len = self.codes.len();
        let mut vec = Vec::with_capacity(1 + 8 * tab_len + tab_len);

        vec.push(tab_len as u8);

        for len in &self.lens[..tab_len] {
            vec.push(*len as u8);
        }
        while vec.len() % 8 != 0 {
            vec.push(0);
        }
        for symbol in &self.symbols[..tab_len] {
            vec.extend_from_slice(&symbol.to_le_bytes());
        }

        vec
    }

    fn try_insert(&mut self, symbol: u64, len: usize) -> Option<u8> {
        if self.is_full() || self.codes.contains_key(&(symbol, len)) {
            None
        } else {
            let new_code = self.codes.len();
            assert_eq!(self.symbols[new_code], 0);
            self.symbols[new_code] = symbol;
            self.lens[new_code] = len;
            self.codes.insert((symbol, len), new_code as u8);
            Some(new_code as u8)
        }
    }

    fn construct<S: AsRef<[u8]>>(input: &Input<S>, progress_bar: &indicatif::ProgressBar, fsst_iters: usize) -> Self {
        let algorithm = SymbolTableConstructor::new(fsst_iters);
        algorithm.run(input, progress_bar)
    }

    fn get_code(&self, symbol: u64, len: usize) -> Option<u8> {
        self.codes.get(&(symbol, len)).copied()
    }
}

struct SymbolTableConstructor {
    st: SymbolTable,
    rem_iters: usize,
}

impl SymbolTableConstructor {
    fn new(fsst_iters: usize) -> Self {
        Self {
            st: SymbolTable::empty(),
            rem_iters: fsst_iters,
        }
    }

    fn run<S: AsRef<[u8]>>(mut self, input: &Input<S>, progress_bar: &indicatif::ProgressBar) -> SymbolTable {
        let mut best_st = SymbolTable::empty();
        let mut best_score = input.total_len;

        while self.rem_iters > 0 {
            progress_bar.set_prefix(format!("remaining iterations: {}", self.rem_iters));
            // Allocate on the heap.
            let (mut count_single, mut count_double) = unsafe {
                use std::alloc::{alloc_zeroed, Layout};
                let layout_single = Layout::new::<[usize; 512]>();
                let layout_double = Layout::new::<[[usize; 512]; 512]>();
                let raw_single = alloc_zeroed(layout_single) as *mut [usize; 512];
                let raw_double = alloc_zeroed(layout_double) as *mut [[usize; 512]; 512];
                (Box::from_raw(raw_single), Box::from_raw(raw_double))
            };
            let score = self.compress_with_current(input, &mut count_single, &mut count_double, progress_bar);
            let new_st = self.make_table(&count_single, &count_double);

            if score < best_score {
                best_score = score;
                best_st = self.st;
            }
            progress_bar.set_message(format!(
                "Effective ratio now: {:.4} ({}/{})",
                input.total_len as f32 / score as f32,
                input.total_len,
                score
            ));

            self.st = new_st;
            self.rem_iters -= 1;
        }

        progress_bar.finish();
        best_st
    }

    fn compress_with_current<S: AsRef<[u8]>>(
        &self,
        input: &Input<S>,
        count_single: &mut [usize; 512],
        count_double: &mut [[usize; 512]; 512],
        progress_bar: &indicatif::ProgressBar,
    ) -> usize {
        let mut prev = None;
        let mut encoding_len = 0;
        progress_bar.set_length(input.total_len as u64);
        progress_bar.reset();
        // This is the max compression factor, we will need at least this much.
        for str in input.strings {
            let bytes = str.as_ref();
            let mut i = 0;
            while i < bytes.len() {
                let byte = bytes[i];
                let best_code = find_best_code(bytes, i, &self.st);
                let ext_code = if best_code == 255 {
                    // Escape.
                    i += 1;
                    encoding_len += 2;
                    progress_bar.inc(1);
                    byte as u16
                } else {
                    count_single[byte as usize] += 1;
                    if let Some(prev) = prev {
                        count_double[prev as usize][byte as usize] += 1;
                    }
                    i += self.st.lens[best_code as usize];
                    encoding_len += 1;
                    progress_bar.inc(self.st.lens[best_code as usize] as u64);
                    (best_code as u16) + 256
                };

                count_single[ext_code as usize] += 1;
                if let Some(prev) = prev {
                    count_double[prev as usize][ext_code as usize] += 1;
                }

                prev = Some(ext_code);
            }
        }
        progress_bar.finish();
        encoding_len
    }

    fn make_table(&self, count_single: &[usize; 512], count_double: &[[usize; 512]; 512]) -> SymbolTable {
        let mut candidates = vec![];

        for code in 0_u16..511 {
            let len = get_len(&self.st, code);
            let symbol = get_symbol(&self.st, code);
            let gain = len * count_single[code as usize];
            if gain > 0 {
                candidates.push((gain, (symbol, len)));
            }

            // Doubles.
            for code2 in 0_u16..511 {
                let len2 = get_len(&self.st, code2);
                let symbol2 = get_symbol(&self.st, code2);
                let (symbol, len) = concat_symbols(symbol, symbol2, len, len2);
                let gain = len * count_double[code as usize][code2 as usize];
                if gain > 0 {
                    candidates.push((gain, (symbol, len)));
                }
            }
        }

        candidates.sort_by_key(|(g, _)| *g);
        let mut new_st = SymbolTable::empty();

        while !new_st.is_full() {
            let Some((_, (symbol, len))) = candidates.pop() else {
                break;
            };
            new_st.try_insert(symbol, len);
        }

        return new_st;

        fn get_len(st: &SymbolTable, code: u16) -> usize {
            if code <= 255 {
                1
            } else {
                st.lens[(code - 256) as usize]
            }
        }

        fn get_symbol(st: &SymbolTable, code: u16) -> u64 {
            if code <= 255 {
                code as u64
            } else {
                st.symbols[(code - 256) as usize]
            }
        }

        fn concat_symbols(s1: u64, s2: u64, l1: usize, l2: usize) -> (u64, usize) {
            let mut buf = [0; 8];
            let len_from_2 = std::cmp::min(l2, 8 - l1);
            let len = l1 + len_from_2;
            buf[..l1].copy_from_slice(&s1.to_le_bytes()[..l1]);
            buf[l1..len].copy_from_slice(&s2.to_le_bytes()[..len_from_2]);
            let symbol = u64::from_le_bytes(buf);
            (symbol, len)
        }
    }
}

fn compress_payload_with<S: AsRef<[u8]>>(
    input: &Input<S>,
    st: &SymbolTable,
    progress_bar: &indicatif::ProgressBar,
) -> CompressionResult {
    let mut offsets = Vec::with_capacity(4 * input.strings.len() + 4);
    // This is the max compression factor, we will need at least this much.
    let mut string_data = Vec::with_capacity(input.total_len / 8);
    progress_bar.set_length(input.total_len as u64);
    progress_bar.reset();
    offsets.extend_from_slice(&[0; 4]);
    for str in input.strings {
        let bytes = str.as_ref();
        let mut i = 0;
        while i < bytes.len() {
            let best_code = find_best_code(bytes, i, st);

            if best_code == 255 {
                // Escape.
                string_data.push(255);
                string_data.push(bytes[i]);
                i += 1;
                progress_bar.inc(1);
            } else {
                string_data.push(best_code);
                i += st.lens[best_code as usize];
                progress_bar.inc(st.lens[best_code as usize] as u64);
            }
        }
        let offset = u32::try_from(string_data.len()).expect("offsets to fit in 32 bits");
        offsets.extend_from_slice(&offset.to_le_bytes());
    }

    progress_bar.finish();
    CompressionResult { offsets, string_data }
}

fn find_best_code(input: &[u8], index: usize, st: &SymbolTable) -> u8 {
    let mut len = 1;
    let mut best_code = 255;
    while len <= 8 && index + len <= input.len() {
        let symbol_bytes = &input[index..index + len];
        let mut symbol_buf = [0; 8];
        symbol_buf[..len].copy_from_slice(symbol_bytes);
        let symbol = u64::from_le_bytes(symbol_buf);
        if let Some(code) = st.get_code(symbol, len) {
            best_code = code;
            assert_eq!(symbol, st.symbols[best_code as usize]);
            assert_eq!(len, st.lens[best_code as usize]);
        }
        len += 1;
    }
    best_code
}

impl Display for SymbolTable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "FSST Symbol Table:")?;

        for i in 0..self.codes.len() {
            write!(f, "  ")?;
            let symbol = self.symbols[i];
            let len = self.lens[i];
            for b in &symbol.to_le_bytes()[..len] {
                let c = *b as char;
                write!(f, "{c}")?;
            }
            writeln!(f, "  ->  {i}")?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::{compress_payload_with, SymbolTable};

    #[test]
    fn figure_1_corpus_compression() {
        let strings = [
            r"http://in.tum.de",
            r"http://cwi.nl",
            r"www.uni-jena.de",
            r"www.wikipedia.org",
            r"http://www.vldb.org",
        ];
        let total_len = strings.iter().map(|x| x.len()).sum();
        let input = super::Input {
            strings: &strings,
            total_len,
        };
        let symbol_table = SymbolTable::new(vec![
            "http://", "www.", "uni-jena", ".de", ".org", "a", "in.tum", "cwi.nl", "wikipedi", "vldb",
        ]);
        let expected_offsets_u32 = [3_u32, 5, 8, 12, 16];
        let expected_offsets = unsafe {
            std::slice::from_raw_parts(
                expected_offsets_u32.as_ptr().cast::<u8>(),
                expected_offsets_u32.len() * 4,
            )
        };
        let expected_string_data = [0_u8, 6, 3, 0, 7, 1, 2, 3, 1, 8, 5, 4, 0, 1, 9, 4];

        let result = compress_payload_with(&input, &symbol_table, &indicatif::ProgressBar::hidden());

        assert_eq!(result.offsets, expected_offsets);
        assert_eq!(result.string_data, expected_string_data);
    }
}
