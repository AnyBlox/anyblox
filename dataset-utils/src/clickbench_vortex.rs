use anyhow::{bail, Result};
use parquet::arrow::arrow_reader::ParquetRecordBatchReaderBuilder;
use std::{borrow::Cow, env};
use tokio::io::{AsyncSeekExt, AsyncWrite, AsyncWriteExt};
use vortex::{
    buffer::io_buf::IoBuf,
    file::VortexFileWriter,
    io::VortexWrite,
    sampling_compressor::{SamplingCompressor, DEFAULT_COMPRESSORS},
    ArrayData,
};

const TUPLE_COUNT: usize = 99_997_497;
const TUPLES_PER_PAGE: usize = 122_880;
const PARTITIONS: usize = 4;

struct TrackedWrite {
    file: TokioWrap,
    len: usize,
}

impl TrackedWrite {
    pub fn new(file: tokio::fs::File) -> Self {
        Self {
            file: TokioWrap(file),
            len: 0,
        }
    }

    pub fn current_offset(&self) -> usize {
        self.len
    }
}

impl TrackedWrite {
    pub async fn write_all(&mut self, buf: &[u8]) -> std::io::Result<()> {
        self.len += buf.len();
        self.file.0.write_all(buf).await
    }

    pub async fn align(&mut self) -> std::io::Result<()> {
        if self.len % 64 != 0 {
            let pad = 64 - (self.len % 64);
            self.file.0.write_all(&[0; 64][..pad]).await?;
            self.len += pad;
        }
        Ok(())
    }
}

impl AsyncWrite for TrackedWrite {
    fn poll_write(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
        buf: &[u8],
    ) -> std::task::Poll<std::result::Result<usize, std::io::Error>> {
        unsafe { self.map_unchecked_mut(|x| &mut x.file.0).poll_write(cx, buf) }
    }

    fn poll_flush(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<std::result::Result<(), std::io::Error>> {
        unsafe { self.map_unchecked_mut(|x| &mut x.file.0).poll_flush(cx) }
    }

    fn poll_shutdown(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<std::result::Result<(), std::io::Error>> {
        unsafe { self.map_unchecked_mut(|x| &mut x.file.0).poll_shutdown(cx) }
    }
}

impl vortex::io::VortexWrite for TrackedWrite {
    async fn write_all<B: IoBuf>(&mut self, buffer: B) -> std::io::Result<B> {
        let result = vortex::io::VortexWrite::write_all(&mut self.file, buffer).await?;
        self.len += result.as_slice().len();
        Ok(result)
    }

    fn flush(&mut self) -> impl std::future::Future<Output = std::io::Result<()>> {
        vortex::io::VortexWrite::flush(&mut self.file)
    }

    fn shutdown(&mut self) -> impl std::future::Future<Output = std::io::Result<()>> {
        vortex::io::VortexWrite::shutdown(&mut self.file)
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = env::args().collect::<Vec<_>>();
    if args.len() < 3 || args.len() > 4 {
        bail!("usage: {} INPUT_PARQUET OUTPUT_DIR", args[0]);
    }
    let input_path = &args[1];
    let output_dir = &args[2];

    let mut parquet_reader = {
        let file = std::fs::File::open(input_path)?;
        let builder = ParquetRecordBatchReaderBuilder::try_new(file)?.with_batch_size(TUPLES_PER_PAGE);
        builder.build()?
    };

    let progress_bar = get_progress_bar("converting parquet to vortex...");
    let pages = TUPLE_COUNT.div_ceil(TUPLES_PER_PAGE);
    let pages_per_part = pages.div_ceil(PARTITIONS);
    let compressor = SamplingCompressor::new(DEFAULT_COMPRESSORS.into());

    for part in 0..PARTITIONS {
        let first_page = part * pages_per_part;
        let last_page = std::cmp::min(first_page + pages_per_part, pages);

        let file_path = format!("{output_dir}/clickbench_{part}.vortex");
        let file = tokio::fs::File::create(&file_path).await?;
        let mut write = TrackedWrite::new(file);
        let mut page_starts = vec![];
        let mut page_ends = vec![];
        write
            .write_all(&(u32::try_from(TUPLES_PER_PAGE).unwrap()).to_le_bytes())
            .await?;
        for _ in first_page..last_page {
            write.write_all(&0_u32.to_le_bytes()).await?;
            write.write_all(&0_u32.to_le_bytes()).await?;
        }

        for page in first_page..last_page {
            let arrow_batch = parquet_reader.next().unwrap()?;
            let len = arrow_batch.num_rows();
            if page != pages - 1 {
                assert_eq!(len, TUPLES_PER_PAGE);
            } else {
                assert_eq!(len, 96057);
            }
            write.align().await?;
            VortexWrite::flush(&mut write).await?;
            page_starts.push(write.current_offset());

            let cvtx = compressor
                .compress(&ArrayData::try_from(arrow_batch)?, None)
                .unwrap()
                .into_array();
            let writer = VortexFileWriter::new(&mut write);
            let writer = writer.write_array_columns(cvtx).await?;
            writer.finalize().await?;
            page_ends.push(write.current_offset());

            progress_bar.inc(len as u64);
        }

        let mut file = tokio::fs::OpenOptions::new().write(true).open(file_path).await?;
        file.seek(std::io::SeekFrom::Start(4)).await?;
        println!("{page_starts:?}");
        println!("{page_ends:?}");
        for (start, end) in page_starts.into_iter().zip(page_ends.into_iter()) {
            file.write_all(&(u32::try_from(start).unwrap()).to_le_bytes()).await?;
            file.write_all(&(u32::try_from(end).unwrap()).to_le_bytes()).await?;
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

struct TokioWrap(tokio::fs::File);

impl VortexWrite for TokioWrap {
    async fn write_all<B: IoBuf>(&mut self, buffer: B) -> std::io::Result<B> {
        AsyncWriteExt::write_all(&mut self.0, buffer.as_slice()).await?;
        Ok(buffer)
    }

    async fn flush(&mut self) -> std::io::Result<()> {
        AsyncWriteExt::flush(&mut self.0).await
    }

    async fn shutdown(&mut self) -> std::io::Result<()> {
        AsyncWriteExt::shutdown(&mut self.0).await
    }
}
