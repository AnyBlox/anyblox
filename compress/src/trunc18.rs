use std::io::{self, Write};

/// Truncate to 18 bits.
pub fn compress_32<W: Write>(
    values: &[i32],
    mut output: W,
    progress_bar_style: indicatif::ProgressStyle,
) -> Result<usize, io::Error> {
    let progress_bar = indicatif::ProgressBar::new(values.len() as u64).with_style(progress_bar_style);
    progress_bar.set_prefix("Trunc18 encoding...");
    let mut written = 0;

    // We go through the values in batches of 4, giving a total of 72 encoded bits,
    // that is 9 bytes. If the last batch is uneven we fill with zeros.
    let mut chunks_of_4 = values.chunks_exact(4);
    let mut buf = [0; 9];
    let mut done = false;
    while !done {
        let (a, b, c, d) = if let Some(chunk) = chunks_of_4.next() {
            progress_bar.inc(4);
            (chunk[0], chunk[1], chunk[2], chunk[3])
        } else {
            let partial = chunks_of_4.remainder();
            done = true;
            if partial.is_empty() {
                break;
            } else if partial.len() == 1 {
                progress_bar.inc(1);
                (partial[0], 0, 0, 0)
            } else if partial.len() == 2 {
                progress_bar.inc(2);
                (partial[0], partial[1], 0, 0)
            } else {
                progress_bar.inc(3);
                (partial[0], partial[1], partial[2], 0)
            }
        };
        /*buf[0] = ((a & 0b0000_0000_0000_0011_1111_1100_0000_0000) >> 10) as u8;
        buf[1] = ((a & 0b0000_0000_0000_0000_0000_0011_1111_1100) >> 2) as u8;
        buf[2] = (a & 0b0000_0000_0000_0000_0000_0000_0000_0011) as u8
            | ((b & 0b0000_0000_0000_0011_1111_0000_0000_0000) >> 10) as u8;
        buf[3] = ((b & 0b0000_0000_0000_0000_0000_1111_1111_0000) >> 4) as u8;
        buf[4] = (b & 0b0000_0000_0000_0000_0000_0000_0000_1111) as u8
            | ((c & 0b0000_0000_0000_0011_1100_0000_0000_0000) >> 10) as u8;
        buf[5] = ((c & 0b0000_0000_0000_0000_0011_1111_1100_0000) >> 6) as u8;
        buf[6] = (c & 0b0000_0000_0000_0000_0000_0000_0011_1111) as u8
            | ((d & 0b0000_0000_0000_0011_0000_0000_0000_0000) >> 10) as u8;
        buf[7] = ((d & 0b0000_0000_0000_0000_1111_1111_0000_0000) >> 8) as u8;
        buf[8] = (d & 0b0000_0000_0000_0000_0000_0000_1111_1111) as u8;*/
        buf[0] = (a & 0b0000_0000_0000_0000_0000_0000_1111_1111) as u8;
        buf[1] = ((a & 0b0000_0000_0000_0000_1111_1111_0000_0000) >> 8) as u8;
        buf[2] = ((a & 0b0000_0000_0000_0011_0000_0000_0000_0000) >> 16) as u8
            | ((b & 0b0000_0000_0000_0000_0000_0000_0011_1111) << 2) as u8;
        buf[3] = ((b & 0b0000_0000_0000_0000_0011_1111_1100_0000) >> 6) as u8;
        buf[4] = ((b & 0b0000_0000_0000_0011_1100_0000_0000_0000) >> 14) as u8
            | ((c & 0b0000_0000_0000_0000_0000_0000_0000_1111) << 4) as u8;
        buf[5] = ((c & 0b0000_0000_0000_0000_0000_1111_1111_0000) >> 4) as u8;
        buf[6] = ((c & 0b0000_0000_0000_0011_1111_0000_0000_0000) >> 12) as u8
            | ((d & 0b0000_0000_0000_0000_0000_0000_0000_0011) << 6) as u8;
        buf[7] = ((d & 0b0000_0000_0000_0000_0000_0011_1111_1100) >> 2) as u8;
        buf[8] = ((d & 0b0000_0000_0000_0011_1111_1100_0000_0000) >> 10) as u8;

        output.write_all(&buf)?;
        written += 9;
    }

    progress_bar.finish();
    println!(
        "Trunc18: Compression {:.2} ({}B to {}B)",
        (values.len() as f32 * 4.0 / written as f32),
        values.len() * 4,
        written
    );
    Ok(written)
}

#[cfg(test)]
mod tests {
    use std::error::Error;

    #[test]
    pub fn test_1() -> Result<(), Box<dyn Error>> {
        let ints = [0xbad, 0x1dea, 0xdead, 0xbeef, 0x1234, 0x5678];
        let mut result = vec![];
        let expected = [
            173, 11, 168, 119, 208, 234, 205, 187, 47, 52, 18, 224, 89, 1, 0, 0, 0, 0,
        ];

        super::compress_32(&ints, &mut result, indicatif::ProgressStyle::default_bar())?;

        assert_eq!(result, &expected);
        Ok(())
    }
}
