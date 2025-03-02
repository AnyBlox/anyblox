use anyhow::{bail, Result};
use arrow::array::AsArray;
use indicatif::MultiProgress;
use parquet::arrow::{
        arrow_reader::ParquetRecordBatchReaderBuilder, ProjectionMask,
    };
use std::{
    borrow::Cow,
    env,
    fs::{self},
    io::{self, Seek, Write},
};

const TUPLE_COUNT: usize = 99_997_497;
const TUPLES_PER_PAGE: usize = 122_880;
const PARTITIONS: usize = 4;

struct TrackedWrite<W> {
    write: W,
    pos: usize,
}

impl<W> io::Write for TrackedWrite<W>
where
    W: io::Write,
{
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        let res = self.write.write(buf)?;
        self.pos += res;
        Ok(res)
    }

    fn flush(&mut self) -> io::Result<()> {
        self.write.flush()
    }
}

impl<W> io::Seek for TrackedWrite<W>
where
    W: io::Seek,
{
    fn seek(&mut self, pos: io::SeekFrom) -> io::Result<u64> {
        let res = self.write.seek(pos)?;
        self.pos = res as usize;
        Ok(res)
    }
}

impl<W> TrackedWrite<W> {
    pub fn new(write: W) -> Self {
        Self { write, pos: 0 }
    }

    pub fn pos(&self) -> usize {
        self.pos
    }
}

impl<W> TrackedWrite<W>
where
    W: io::Write,
{
    pub fn align(&mut self) -> std::io::Result<()> {
        if self.pos % 64 != 0 {
            let pad = 64 - (self.pos % 64);
            self.write_all(&[0; 64][..pad])?;
        }
        Ok(())
    }
}

fn main() -> Result<()> {
    let args = env::args().collect::<Vec<_>>();
    if args.len() < 3 || args.len() > 4 {
        bail!("usage: {} INPUT_PARQUET OUTPUT_DIR [FSST_ITERS]", args[0]);
    }
    let input_path = &args[1];
    let output_dir = &args[2];
    let fsst_iters = if args.len() == 4 {
        args[3].parse().expect("FSST_ITERS has invalid format")
    } else {
        5
    };

    let pages = TUPLE_COUNT.div_ceil(TUPLES_PER_PAGE);
    let pages_per_part = pages.div_ceil(PARTITIONS);
    let multi_bar = MultiProgress::new();

    for part in 0..PARTITIONS {
        let first_page = part * pages_per_part;
        let first_tuple = part * pages_per_part * TUPLES_PER_PAGE;
        let last_page = std::cmp::min(first_page + pages_per_part, pages);

        let file_path = format!("{output_dir}/clickbench_{part}.fsst");
        let mut writer = TrackedWrite::new(io::BufWriter::new(fs::File::create(&file_path)?));
        let mut column_starts = vec![];
        writer.write_all(&(u32::try_from(TUPLES_PER_PAGE).unwrap()).to_le_bytes())?;
        writer.write_all(&(u32::try_from(first_tuple).unwrap()).to_le_bytes())?;
        for _ in 0..6 {
            writer.write_all(&0_u32.to_le_bytes())?;
        }
        for column in 1..6 {
            writer.align()?;
            let column_start = writer.pos();
            column_starts.push(column_start);
            let mut page_starts = vec![];
            for _ in first_page..last_page {
                writer.write_all(&0_u32.to_le_bytes())?;
            }

            let progress_bar = get_progress_bar(format!("converting column {column} to fsst..."));
            multi_bar.add(progress_bar.clone());
            let mut parquet_reader = {
                let file = std::fs::File::open(input_path)?;
                let builder = ParquetRecordBatchReaderBuilder::try_new(file)?;
                let schema = builder.parquet_schema();
                let projection = ProjectionMask::leaves(schema, [column]);
                let builder = builder
                    .with_offset(first_tuple)
                    .with_batch_size(TUPLES_PER_PAGE)
                    .with_projection(projection);
                builder.build()?
            };

            for page in first_page..last_page {
                let arrow_batch = parquet_reader.next().unwrap()?;
                let len = arrow_batch.num_rows();
                if page != pages - 1 {
                    assert_eq!(len, TUPLES_PER_PAGE);
                } else {
                    assert_eq!(len, 96057);
                }
                writer.align()?;
                page_starts.push(writer.pos());

                let strings = arrow_batch
                    .column(0)
                    .as_string::<i32>()
                    .into_iter()
                    .map(|s| s.unwrap_or(""))
                    .collect::<Vec<_>>();
                compress::fsst::compress(
                    &strings,
                    &mut writer,
                    fsst_iters,
                    get_fsst_progress_bar_style(),
                    Some(&multi_bar),
                )?;

                progress_bar.inc(len as u64);
            }

            writer.flush()?;
            let column_end = writer.pos();
            writer.seek(io::SeekFrom::Start(column_start as u64))?;
            for start in page_starts {
                writer.write_all(&(u32::try_from(start).unwrap()).to_le_bytes())?;
            }
            writer.seek(io::SeekFrom::Start(column_end as u64))?;
            progress_bar.finish();
            multi_bar.remove(&progress_bar);
        }

        writer.flush()?;
        writer.seek(io::SeekFrom::Start(8))?;
        for start in column_starts {
            writer.write_all(&(u32::try_from(start).unwrap()).to_le_bytes())?;
        }
    }

    Ok(())
}

fn get_progress_bar<S: Into<Cow<'static, str>>>(msg: S) -> indicatif::ProgressBar {
    use indicatif::{ProgressBar, ProgressStyle};
    let style = ProgressStyle::with_template(
        " {spinner:.cyan} {prefix} [{elapsed_precise}] {wide_bar} {msg} {pos:>7}/{len:7} [{per_sec}]",
    )
    .unwrap();

    let progress = ProgressBar::new(TUPLE_COUNT as u64).with_style(style);
    progress.set_prefix(msg);

    progress
}

fn get_fsst_progress_bar_style() -> indicatif::ProgressStyle {
    use indicatif::ProgressStyle;
    ProgressStyle::with_template(
        " {spinner:.cyan} {prefix} [{elapsed_precise}] {wide_bar} {msg} [{decimal_bytes_per_sec}]",
    )
    .unwrap()
}
