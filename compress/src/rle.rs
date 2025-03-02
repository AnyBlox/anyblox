use std::io::{self, Write};

pub trait RleEncodable {
    fn write_bytes<W: Write>(&self, writer: &mut W) -> Result<usize, io::Error>;
}

macro_rules! rle_for_int {
    ($int:ty) => {
        impl RleEncodable for $int {
            fn write_bytes<W: Write>(&self, writer: &mut W) -> Result<usize, io::Error> {
                let bytes = self.to_le_bytes();
                writer.write_all(&bytes)?;
                Ok(bytes.len())
            }
        }
    };
}

rle_for_int!(i8);
rle_for_int!(u8);
rle_for_int!(i16);
rle_for_int!(u16);
rle_for_int!(i32);
rle_for_int!(u32);
rle_for_int!(i64);
rle_for_int!(u64);
rle_for_int!(i128);
rle_for_int!(u128);

/// Offset-based RLE encoding for 32-bit values.
/// The runs are encoded as the *next id* of the value after the run.
/// As an example, [1, 1, 1, 2, 2, 3] is encoded as:
/// [3, 5, 6] [1, 2, 3]
pub fn compress<R: RleEncodable + Copy + Eq, W: Write>(
    input: &[R],
    mut output: W,
    progress_bar_style: indicatif::ProgressStyle,
) -> Result<usize, io::Error> {
    let progress_bar = indicatif::ProgressBar::new(2 * input.len() as u64).with_style(progress_bar_style);
    progress_bar.set_prefix("RLE encoding...");

    let mut ints = input.iter();
    let mut values = vec![];
    let mut written = 0;
    let Some(&first) = ints.next() else {
        return Ok(0);
    };
    progress_bar.inc(4);
    let mut run = IntRun(first, 1);

    for &int in ints {
        match run.try_extend(int) {
            ExtendResult::Extended(ext_run) => run = ext_run,
            ExtendResult::NewRun(new_run) => {
                values.push(*run.int());
                output.write_all(&run.offset().to_le_bytes())?;
                run = new_run;
                written += 4;
            }
        }
        progress_bar.inc(1);
    }

    values.push(*run.int());
    output.write_all(&run.offset().to_le_bytes())?;
    written += 4;
    progress_bar.inc(1);

    for value in values {
        written += value.write_bytes(&mut output)?;
        progress_bar.inc(1);
    }
    written += 4;
    progress_bar.finish();

    println!(
        "RLE: Compression {:.2} ({}B to {}B)",
        (input.len() as f32 * 4.0 / written as f32),
        input.len() * 4,
        written
    );
    Ok(written)
}

struct IntRun<T>(T, u32);

enum ExtendResult<T> {
    Extended(IntRun<T>),
    NewRun(IntRun<T>),
}

impl<T> IntRun<T> {
    fn int(&self) -> &T {
        &self.0
    }

    fn offset(&self) -> u32 {
        self.1
    }
}

impl<T: Eq + Copy> IntRun<T> {
    fn try_extend(&self, int: T) -> ExtendResult<T> {
        if int.eq(&self.0) {
            ExtendResult::Extended(IntRun(self.0, self.1 + 1))
        } else {
            ExtendResult::NewRun(IntRun(int, self.1 + 1))
        }
    }
}

#[cfg(test)]
mod tests {
    use std::error::Error;

    #[test]
    pub fn test_1() -> Result<(), Box<dyn Error>> {
        let ints = [1, 1, 1, 2, 2, 3];
        let mut result = vec![];
        let expected: [u32; 6] = [3, 5, 6, 1, 2, 3];

        super::compress(&ints, &mut result, indicatif::ProgressStyle::default_bar())?;
        let reinterpret = unsafe { std::slice::from_raw_parts(result.as_ptr().cast::<u32>(), result.len() / 4) };

        assert_eq!(reinterpret, &expected);
        Ok(())
    }
}
