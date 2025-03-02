use anyhow::{bail, Result};
use std::{
    borrow::Cow,
    env, fs,
    io::{self, Write},
};

/// We read the any CSV and RLE-encode one of the columns.
/// It has to be actually integral and '|' separated.
fn main() -> Result<()> {
    let args = env::args().collect::<Vec<_>>();
    let argc = args.len();
    if argc < 5 {
        bail!("usage: {} INPUT_CSVS... OUTPUT_BINARY OUTPUT_CSV COL_IDX", args[0]);
    }

    let output_binary_path = &args[argc - 3];
    let output_csv_path = &args[argc - 2];
    let col_idx: usize = args[argc - 1].parse().expect("COL_IDX has invalid format");
    let input_paths = &args[1..argc - 3];
    let mut integers = vec![];
    let spinner = get_spinner("reading csv...");

    for entry in input_paths {
        spinner.set_message(format!("reading csv {}...", entry));
        let mut csv = csv::ReaderBuilder::new()
            .delimiter(b'|')
            .has_headers(false)
            .from_path(entry)?;

        for record in csv.records() {
            let record = record?;
            let str = record.get(col_idx).expect("no column with given index");
            let int = str.parse::<i64>().expect("cannot parse as i64");
            integers.push(int);
            spinner.inc(1);
        }
    }
    spinner.finish();

    write_csv(&integers, output_csv_path)?;

    let writer = io::BufWriter::new(fs::File::create(output_binary_path)?);
    let min = *integers.iter().min().unwrap();
    let max = *integers.iter().max().unwrap();

    if min >= (i32::MIN as i64) && max <= (i32::MAX as i64) {
        rle_as_32(integers, writer)?;
    } else {
        rle_64(&integers, writer)?;
    }

    Ok(())
}

fn write_csv(integers: &[i64], output_path: &str) -> Result<()> {
    let mut csv = csv::WriterBuilder::new()
        .delimiter(b'|')
        .has_headers(false)
        .from_path(output_path)?;

    for int in integers {
        csv.write_record([int.to_string()])?;
    }

    Ok(())
}

fn rle_as_32<W: Write>(column: Vec<i64>, output: W) -> Result<usize> {
    let column = column.into_iter().map(|x| x as i32).collect::<Vec<_>>();
    Ok(compress::rle::compress(&column, output, get_progress_bar_style())?)
}

fn rle_64<W: Write>(column: &[i64], output: W) -> Result<usize> {
    Ok(compress::rle::compress(column, output, get_progress_bar_style())?)
}

fn get_progress_bar_style() -> indicatif::ProgressStyle {
    use indicatif::ProgressStyle;
    ProgressStyle::with_template(
        " {spinner:.cyan} {prefix} [{elapsed_precise}] {wide_bar} {msg} [{decimal_bytes_per_sec}]",
    )
    .unwrap()
}

fn get_spinner<S: Into<Cow<'static, str>>>(msg: S) -> indicatif::ProgressBar {
    use indicatif::{ProgressBar, ProgressStyle};
    let style = ProgressStyle::with_template(" {spinner:.cyan} {prefix} [{elapsed_precise}] {msg} [{pos}] [{per_sec}]")
        .unwrap();

    let progress = ProgressBar::new_spinner().with_style(style);
    progress.set_prefix(msg);

    progress
}
