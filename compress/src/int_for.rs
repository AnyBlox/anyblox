use std::{
    arch::x86_64::*,
    fmt::Display,
    io::{self, Write},
};
use thiserror::Error;

const INTS_IN_REG: usize = BYTES_IN_REG / 4;
const LONGS_IN_REG: usize = BYTES_IN_REG / 8;
const BYTES_IN_REG: usize = 64;
const BLOCK_MAP_GRANULARITY: usize = 64 * 1024;

pub struct CompressionStats {
    input_bytes: u64,
    output_bytes: u64,
    blocks: u64,
}

impl CompressionStats {
    fn compression_factor(&self) -> f32 {
        self.input_bytes as f32 / self.output_bytes as f32
    }

    fn block_overhead(&self) -> f32 {
        8.0 * self.blocks as f32 / self.output_bytes as f32
    }
}

impl Display for CompressionStats {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            "Compression {:.2} ({}B to {}B)",
            self.compression_factor(),
            self.input_bytes,
            self.output_bytes
        )?;
        writeln!(
            f,
            "Blocks: {:.2}; total overhead: {}",
            self.blocks,
            self.block_overhead()
        )
    }
}

/// The compressed format is: first the block size, describing the number of *tuples* in each block (4 bytes).
/// Then the blocks follow, and at the end the block map.
///
/// Each block starts with the minimum value (frame of reference, 4 bytes) and
/// the size (8/16/32/64, 4 bytes). Then the FoR-shifted and truncated values follow.
/// Say the block size is B. Then each block takes 8 + B, 8 + 2B, 8 + 4B, or 8 + 8B bytes.
/// This value is always divisible by 2 as long as the block size is divisible by 2.
///
/// To aid in finding the start place of decompression, we insert a block map at the end.
/// The block map contains the starting offset of a block that contains the K2^16-th tuple
/// for each applicable K. The final value is the length of this map.
pub fn compress_32<W: Write>(
    ints: &[i32],
    mut output: W,
    block_size: usize,
    progress_bar_style: indicatif::ProgressStyle,
) -> Result<usize, EncodingError> {
    assert!(block_size % 4 == 0);
    let bytes: &[u8] = unsafe { std::slice::from_raw_parts(ints.as_ptr().cast(), ints.len() * 4) };
    let mut block_map = Vec::with_capacity(ints.len() / BLOCK_MAP_GRANULARITY);
    let mut current_count_for_map = 0;
    if bytes.len() < BYTES_IN_REG {
        return Err(EncodingError::InputTooShort);
    }
    if bytes.len() % 4 != 0 {
        return Err(EncodingError::InvalidInputSize);
    }

    let progress: indicatif::ProgressBar =
        indicatif::ProgressBar::new(bytes.len() as u64).with_style(progress_bar_style);
    setup_progress_for_encoding(&progress, bytes.len() as u64);
    let mut blocks = 0;
    let mut written = 0;
    output.write_all(&(block_size as u32).to_le_bytes())?;
    written += 4;

    for block in bytes.chunks(block_size * 4) {
        let mut start = 0;
        let mut min = unsafe { _mm512_set1_epi32(i32::MAX) };
        let mut max = unsafe { _mm512_set1_epi32(i32::MIN) };

        while start + BYTES_IN_REG < block.len() {
            unsafe {
                let v_reg = _mm512_loadu_si512(block.as_ptr().add(start).cast());
                min = _mm512_min_epi32(v_reg, min);
                max = _mm512_max_epi32(v_reg, max);
                start += BYTES_IN_REG;
            }
        }

        if start != block.len() {
            unsafe {
                let start = block.len() - BYTES_IN_REG;
                let v_reg = _mm512_loadu_si512(block.as_ptr().add(start).cast());
                min = _mm512_min_epi32(v_reg, min);
            }
        }

        let min = collapse_min_v_reg_32(min);
        let max = collapse_max_v_reg_32(max);
        let size = determine_size_32(min, max);
        output.write_all(&min.to_le_bytes())?;
        output.write_all(&size.as_int_32().to_le_bytes())?;
        written += 8;

        let min_v = unsafe { _mm512_set1_epi32(min) };
        let mut buf = [0; 8 * 1024];
        let mut pos_in_buf = 0;
        let mut chunks = block.chunks_exact(BYTES_IN_REG);
        for chunk in &mut chunks {
            unsafe {
                let v_reg = _mm512_loadu_si512(chunk.as_ptr().cast());
                let enc = _mm512_sub_epi32(v_reg, min_v);
                let base_addr = buf.as_mut_ptr().add(pos_in_buf);

                match size {
                    Size::Byte => {
                        let mask = 0b0001_0001_0001_0001_0001_0001_0001_0001_0001_0001_0001_0001_0001_0001_0001_0001;
                        _mm512_mask_compressstoreu_epi8(base_addr.cast(), mask, enc);
                        pos_in_buf += BYTES_IN_REG / 4;
                    }
                    Size::TwoBytes => {
                        let mask = 0b0101_0101_0101_0101_0101_0101_0101_0101;
                        _mm512_mask_compressstoreu_epi16(base_addr.cast(), mask, enc);
                        pos_in_buf += BYTES_IN_REG / 2;
                    }
                    Size::FourBytes => {
                        _mm512_storeu_si512(buf.as_mut_ptr().add(pos_in_buf).cast(), enc);
                        pos_in_buf += BYTES_IN_REG;
                    }
                    Size::EightBytes => unreachable!("32-bit integers cannot have a 64-bit delta"),
                }
            }
            if pos_in_buf == buf.len() {
                output.write_all(&buf)?;
                written += pos_in_buf;
                pos_in_buf = 0;
            }
            progress.inc(BYTES_IN_REG as u64);
        }

        output.write_all(&buf[..pos_in_buf])?;
        written += pos_in_buf;
        progress.inc(pos_in_buf as u64);

        for int in chunks.remainder().chunks(4) {
            let i32 = i32::from_le_bytes(int.try_into().expect("full 32 bit ints"));
            output.write_all(&(i32 - min).to_le_bytes())?;
            written += 4;
            progress.inc(4);
        }

        blocks += 1;
        current_count_for_map += block.len();

        if current_count_for_map % BLOCK_MAP_GRANULARITY == 0 {
            current_count_for_map = 0;
            block_map.push(written);
        }
    }

    let block_map_len = block_map.len();
    for offset in block_map {
        output.write_all(&(offset as u32).to_le_bytes())?;
        written += 4;
    }
    output.write_all(&(block_map_len as u32).to_le_bytes())?;
    written += 4;

    progress.finish();
    let stats = CompressionStats {
        input_bytes: bytes.len() as u64,
        output_bytes: written as u64,
        blocks,
    };
    println!("IntFor: {stats}");
    Ok(written)
}

pub fn compress_64<W: Write>(
    ints: &[i64],
    mut output: W,
    block_size: usize,
    progress_bar_style: indicatif::ProgressStyle,
) -> Result<usize, EncodingError> {
    assert!(block_size % 4 == 0);
    let bytes: &[u8] = unsafe { std::slice::from_raw_parts(ints.as_ptr().cast(), ints.len() * 8) };
    if bytes.len() < BYTES_IN_REG {
        return Err(EncodingError::InputTooShort);
    }
    if bytes.len() % 8 != 0 {
        return Err(EncodingError::InvalidInputSize);
    }

    let progress: indicatif::ProgressBar =
        indicatif::ProgressBar::new(bytes.len() as u64).with_style(progress_bar_style);
    setup_progress_for_encoding(&progress, bytes.len() as u64);
    let mut blocks = 0;
    let mut written = 0;
    output.write_all(&(block_size as u64).to_le_bytes())?;
    written += 8;

    for block in bytes.chunks(block_size * 8) {
        let mut start = 0;
        let mut min = unsafe { _mm512_set1_epi64(i64::MAX) };
        let mut max = unsafe { _mm512_set1_epi64(i64::MIN) };

        while start + BYTES_IN_REG < block.len() {
            unsafe {
                let v_reg = _mm512_loadu_si512(block.as_ptr().add(start).cast());
                min = _mm512_min_epi64(v_reg, min);
                max = _mm512_max_epi64(v_reg, max);
                start += BYTES_IN_REG;
            }
        }

        if start != block.len() {
            unsafe {
                let start = block.len() - BYTES_IN_REG;
                let v_reg = _mm512_loadu_si512(block.as_ptr().add(start).cast());
                min = _mm512_min_epi64(v_reg, min);
            }
        }

        let min = collapse_min_v_reg_64(min);
        let max = collapse_max_v_reg_64(max);
        let size = determine_size_64(min, max);
        output.write_all(&min.to_le_bytes())?;
        output.write_all(&size.as_int_64().to_le_bytes())?;
        written += 16;

        let min_v = unsafe { _mm512_set1_epi64(min) };
        let mut buf = [0; 8 * 1024];
        let mut pos_in_buf = 0;
        let mut chunks = block.chunks_exact(BYTES_IN_REG);
        for chunk in &mut chunks {
            unsafe {
                let v_reg = _mm512_loadu_si512(chunk.as_ptr().cast());
                let enc = _mm512_sub_epi32(v_reg, min_v);
                let base_addr = buf.as_mut_ptr().add(pos_in_buf);

                match size {
                    Size::Byte => {
                        let mask = 0b0000_0001_0000_0001_0000_0001_0000_0001_0000_0001_0000_0001_0000_0001_0000_0001;
                        _mm512_mask_compressstoreu_epi8(base_addr.cast(), mask, enc);
                        pos_in_buf += BYTES_IN_REG / 8;
                    }
                    Size::TwoBytes => {
                        let mask = 0b0001_0001_0001_0001_0001_0001_0001_0001;
                        _mm512_mask_compressstoreu_epi16(base_addr.cast(), mask, enc);
                        pos_in_buf += BYTES_IN_REG / 4;
                    }
                    Size::FourBytes => {
                        let mask = 0b0101_0101_0101_0101;
                        _mm512_mask_compressstoreu_epi32(base_addr.cast(), mask, enc);
                        pos_in_buf += BYTES_IN_REG / 2;
                    }
                    Size::EightBytes => {
                        _mm512_storeu_si512(buf.as_mut_ptr().add(pos_in_buf).cast(), enc);
                        pos_in_buf += BYTES_IN_REG;
                    }
                }
            }
            if pos_in_buf == buf.len() {
                output.write_all(&buf)?;
                written += pos_in_buf;
                pos_in_buf = 0;
            }
            progress.inc(BYTES_IN_REG as u64);
        }

        output.write_all(&buf[..pos_in_buf])?;
        written += pos_in_buf;
        progress.inc(pos_in_buf as u64);

        for int in chunks.remainder().chunks(8) {
            let u64 = i64::from_le_bytes(int.try_into().expect("full 64 bit ints"));
            output.write_all(&(u64 - min).to_le_bytes())?;
            written += 8;
            progress.inc(8);
        }

        blocks += 1;
    }

    progress.finish();
    let stats = CompressionStats {
        input_bytes: bytes.len() as u64,
        output_bytes: written as u64,
        blocks,
    };
    println!("IntFor: {stats}");
    Ok(written)
}

fn collapse_min_v_reg_32(min: __m512i) -> i32 {
    let mut buf = [0; INTS_IN_REG];
    unsafe { _mm512_storeu_si512(buf.as_mut_ptr().cast(), min) };
    let mut min = i32::MAX;
    for int in buf {
        min = std::cmp::min(min, int);
    }
    min
}

fn collapse_max_v_reg_32(max: __m512i) -> i32 {
    let mut buf = [0; INTS_IN_REG];
    unsafe { _mm512_storeu_si512(buf.as_mut_ptr().cast(), max) };
    let mut max = i32::MIN;
    for int in buf {
        max = std::cmp::max(max, int);
    }
    max
}

fn collapse_min_v_reg_64(min: __m512i) -> i64 {
    let mut buf = [0; LONGS_IN_REG];
    unsafe { _mm512_storeu_si512(buf.as_mut_ptr().cast(), min) };
    let mut min = i64::MAX;
    for int in buf {
        min = std::cmp::min(min, int);
    }
    min
}

fn collapse_max_v_reg_64(max: __m512i) -> i64 {
    let mut buf = [0; LONGS_IN_REG];
    unsafe { _mm512_storeu_si512(buf.as_mut_ptr().cast(), max) };
    let mut max = i64::MIN;
    for int in buf {
        max = std::cmp::max(max, int);
    }
    max
}

#[derive(Debug, Clone, Copy)]
enum Size {
    Byte,
    TwoBytes,
    FourBytes,
    EightBytes,
}

impl Size {
    fn as_int_32(&self) -> u32 {
        match self {
            Self::Byte => 8,
            Self::TwoBytes => 16,
            Self::FourBytes => 32,
            Self::EightBytes => 64,
        }
    }

    fn as_int_64(&self) -> u64 {
        match self {
            Self::Byte => 8,
            Self::TwoBytes => 16,
            Self::FourBytes => 32,
            Self::EightBytes => 64,
        }
    }
}

fn determine_size_32(min: i32, max: i32) -> Size {
    let delta = max - min;
    match delta {
        0..=0xFF => Size::Byte,
        0x100..=0xFFFF => Size::TwoBytes,
        _ => Size::FourBytes,
    }
}

fn determine_size_64(min: i64, max: i64) -> Size {
    let delta = max - min;
    match delta {
        0..=0xFF => Size::Byte,
        0x100..=0xFFFF => Size::TwoBytes,
        0x10000..=0xFFFFFFFF => Size::FourBytes,
        _ => Size::EightBytes,
    }
}

fn setup_progress_for_encoding(progress: &indicatif::ProgressBar, len: u64) {
    progress.finish();
    progress.reset();
    progress.set_prefix("IntFor encoding");
    progress.disable_steady_tick();
    progress.set_length(len);
}

#[derive(Debug, Error)]
pub enum EncodingError {
    #[error(transparent)]
    IoError(#[from] io::Error),
    #[error("integer too large for RLE: {0}")]
    IntegerTooLarge(u32),
    #[error("input byte length not divisible by 4")]
    InvalidInputSize,
    #[error("input byte length has to be at least 32")]
    InputTooShort,
}

#[cfg(test)]
mod tests {
    use indicatif::ProgressStyle;

    use super::compress_32;

    #[test]
    fn encode_8bit() {
        let data = [
            13_057, 13_058, 13_059, 13_060, 13_061, 13_062, 13_063, 13_064, 13_064, 13_063, 13_062, 13_061, 13_060,
            13_059, 13_058, 13_056,
        ];
        let expected = [
            0x00, 0x01, 0x00, 0x00, 0x00, 0x33, 0x00, 0x00, 0x08, 0x00, 0x00, 0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06,
            0x07, 0x08, 0x08, 0x07, 0x06, 0x05, 0x04, 0x03, 0x02, 0x00,
        ];

        let mut result = vec![];
        compress_32(&data, &mut result, 255, ProgressStyle::default_bar()).unwrap();

        assert_eq!(expected, *result);
    }

    #[test]
    fn encode_16bit() {
        let data = [
            596, 1141, 1686, 2231, 2776, 3321, 3866, 4155, 651, 922, 1193, 1464, 1735, 2006, 2277, 307,
        ];
        let expected = [
            0x00, 0x01, 0x00, 0x00, 0x33, 0x01, 0x00, 0x00, 0x10, 0x00, 0x00, 0x00, 0x21, 0x01, 0x42, 0x03, 0x63, 0x05,
            0x84, 0x07, 0xa5, 0x09, 0xc6, 0x0b, 0xe7, 0x0d, 0x08, 0x0f, 0x58, 0x01, 0x67, 0x02, 0x76, 0x03, 0x85, 0x04,
            0x94, 0x05, 0xa3, 0x06, 0xb2, 0x07, 0x00, 0x00,
        ];

        let mut result = vec![];
        compress_32(&data, &mut result, 255, ProgressStyle::default_bar()).unwrap();

        assert_eq!(expected, *result);
    }

    #[test]
    fn encode_32bit() {
        let data = [
            100_000, 200_000, 300_000, 400_000, 500_000, 600_000, 700_000, 800_000, -100_000, -200_000, -300_000,
            -400_000, -500_000, -600_000, -700_000, -800_000,
        ];
        let expected = [
            0x00, 0x01, 0x00, 0x00, 0x00, 0xcb, 0xf3, 0xff, 0x20, 0x00, 0x00, 0x00, 0xa0, 0xbb, 0x0d, 0x00, 0x40, 0x42,
            0x0f, 0x00, 0xe0, 0xc8, 0x10, 0x00, 0x80, 0x4f, 0x12, 0x00, 0x20, 0xd6, 0x13, 0x00, 0xc0, 0x5c, 0x15, 0x00,
            0x60, 0xe3, 0x16, 0x00, 0x00, 0x6a, 0x18, 0x00, 0x60, 0xae, 0x0a, 0x00, 0xc0, 0x27, 0x09, 0x00, 0x20, 0xa1,
            0x07, 0x00, 0x80, 0x1a, 0x06, 0x00, 0xe0, 0x93, 0x04, 0x00, 0x40, 0x0d, 0x03, 0x00, 0xa0, 0x86, 0x01, 0x00,
            0x00, 0x00, 0x00, 0x00,
        ];

        let mut result = vec![];
        compress_32(&data, &mut result, 255, ProgressStyle::default_bar()).unwrap();

        assert_eq!(expected, *result);
    }
}
