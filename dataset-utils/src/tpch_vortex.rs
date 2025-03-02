use anyhow::{bail, Result};
use arrow::{
    array::{PrimitiveArray, RecordBatch, StringArray},
    datatypes::{DataType, Field, Float64Type, Int32Type, Schema, UInt8Type},
};
use chrono::{Datelike, NaiveDate, NaiveDateTime};
use rust_decimal::Decimal;
use std::{borrow::Cow, env, sync::Arc};
use tokio::io::{AsyncSeekExt, AsyncWrite, AsyncWriteExt};
use vortex::{
    buffer::io_buf::IoBuf,
    file::VortexFileWriter,
    io::VortexWrite,
    sampling_compressor::{SamplingCompressor, DEFAULT_COMPRESSORS},
    ArrayData,
};

#[derive(serde::Deserialize, Debug)]
struct LineItemRow {
    orderkey: i32,
    partkey: i32,
    suppkey: i32,
    linenumber: i32,
    quantity: Decimal,
    extendedprice: Decimal,
    discount: Decimal,
    tax: Decimal,
    returnflag: char,
    linestatus: char,
    shipdate: NaiveDate,
    commitdate: NaiveDate,
    receiptdate: NaiveDate,
    shipinstruct: String,
    shipmode: String,
    comment: String,
}

struct LineItemTable {
    orderkey: Vec<i32>,
    partkey: Vec<i32>,
    suppkey: Vec<i32>,
    linenumber: Vec<i32>,
    quantity: Vec<Decimal>,
    extendedprice: Vec<Decimal>,
    discount: Vec<Decimal>,
    tax: Vec<Decimal>,
    returnflag: Vec<char>,
    linestatus: Vec<char>,
    shipdate: Vec<NaiveDate>,
    commitdate: Vec<NaiveDate>,
    receiptdate: Vec<NaiveDate>,
    shipinstruct: Vec<String>,
    shipmode: Vec<String>,
    comment: Vec<String>,
}

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
        bail!("usage: {} INPUT_CSV OUTPUT_PATH", args[0]);
    }
    let input_path = &args[1];
    let output_path = &args[2];

    let mut csv = csv::ReaderBuilder::new()
        .delimiter(b'|')
        .has_headers(false)
        .from_path(input_path)?;

    let mut table = LineItemTable::with_capacity(6_000_000); // for sc = 1
    let spinner = get_spinner("reading csv...");

    for result in csv.deserialize() {
        let record: LineItemRow = result?;
        table.push(record);
        spinner.inc(1);
    }

    const TUPLES_PER_PAGE: usize = 122_880;
    let pages = table.len().div_ceil(TUPLES_PER_PAGE);
    let file = tokio::fs::File::create(output_path).await?;
    let mut write = TrackedWrite::new(file);

    let mut page_starts = vec![];
    let mut page_ends = vec![];
    write
        .write_all(&(u32::try_from(TUPLES_PER_PAGE).unwrap()).to_le_bytes())
        .await?;
    for _ in 0..pages {
        write.write_all(&0_u32.to_le_bytes()).await?;
        write.write_all(&0_u32.to_le_bytes()).await?;
    }

    let compressor = SamplingCompressor::new(DEFAULT_COMPRESSORS.into());

    for page in 0..pages {
        write.align().await?;
        VortexWrite::flush(&mut write).await?;
        page_starts.push(write.current_offset());
        let start_tuple = TUPLES_PER_PAGE * page;
        let last_tuple = std::cmp::min(start_tuple + TUPLES_PER_PAGE, table.len());
        let batch = table.slice_to_arrow(start_tuple..last_tuple)?;

        let cvtx = compressor
            .compress(&ArrayData::try_from(batch)?, None)
            .unwrap()
            .into_array();
        let writer = VortexFileWriter::new(&mut write);
        let writer = writer.write_array_columns(cvtx).await?;
        writer.finalize().await?;
        page_ends.push(write.current_offset());
    }

    let mut file = tokio::fs::OpenOptions::new().write(true).open(output_path).await?;
    file.seek(std::io::SeekFrom::Start(4)).await?;
    println!("{page_starts:?}");
    println!("{page_ends:?}");
    for (start, end) in page_starts.into_iter().zip(page_ends.into_iter()) {
        file.write_all(&(u32::try_from(start).unwrap()).to_le_bytes()).await?;
        file.write_all(&(u32::try_from(end).unwrap()).to_le_bytes()).await?;
    }

    Ok(())
}

impl LineItemTable {
    pub fn with_capacity(size: usize) -> Self {
        Self {
            orderkey: Vec::with_capacity(size),
            partkey: Vec::with_capacity(size),
            suppkey: Vec::with_capacity(size),
            linenumber: Vec::with_capacity(size),
            quantity: Vec::with_capacity(size),
            extendedprice: Vec::with_capacity(size),
            discount: Vec::with_capacity(size),
            tax: Vec::with_capacity(size),
            returnflag: Vec::with_capacity(size),
            linestatus: Vec::with_capacity(size),
            shipdate: Vec::with_capacity(size),
            commitdate: Vec::with_capacity(size),
            receiptdate: Vec::with_capacity(size),
            shipinstruct: Vec::with_capacity(size),
            shipmode: Vec::with_capacity(size),
            comment: Vec::with_capacity(size),
        }
    }

    pub fn push(&mut self, record: LineItemRow) {
        self.orderkey.push(record.orderkey);
        self.partkey.push(record.partkey);
        self.suppkey.push(record.suppkey);
        self.linenumber.push(record.linenumber);
        self.quantity.push(record.quantity);
        self.extendedprice.push(record.extendedprice);
        self.discount.push(record.discount);
        self.tax.push(record.tax);
        self.returnflag.push(record.returnflag);
        self.linestatus.push(record.linestatus);
        self.shipdate.push(record.shipdate);
        self.commitdate.push(record.commitdate);
        self.receiptdate.push(record.receiptdate);
        self.shipinstruct.push(record.shipinstruct);
        self.shipmode.push(record.shipmode);
        self.comment.push(record.comment);
    }

    pub fn slice_to_arrow(&self, range: std::ops::Range<usize>) -> Result<RecordBatch> {
        let schema = Schema::new([
            Arc::new(Field::new("l_orderkey", DataType::Int32, false)),
            Arc::new(Field::new("l_partkey", DataType::Int32, false)),
            Arc::new(Field::new("l_suppkey", DataType::Int32, false)),
            Arc::new(Field::new("l_linenumber", DataType::Int32, false)),
            Arc::new(Field::new("l_quantity", DataType::Float64, false)),
            Arc::new(Field::new("l_extendedprice", DataType::Float64, false)),
            Arc::new(Field::new("l_discount", DataType::Float64, false)),
            Arc::new(Field::new("l_tax", DataType::Float64, false)),
            Arc::new(Field::new("l_returnflag", DataType::UInt8, false)),
            Arc::new(Field::new("l_linestatus", DataType::UInt8, false)),
            Arc::new(Field::new("l_shipdate", DataType::Int32, false)),
            Arc::new(Field::new("l_commitdate", DataType::Int32, false)),
            Arc::new(Field::new("l_receiptdate", DataType::Int32, false)),
            Arc::new(Field::new("l_shipinstruct", DataType::Utf8, false)),
            Arc::new(Field::new("l_shipmode", DataType::Utf8, false)),
            Arc::new(Field::new("l_comment", DataType::Utf8, false)),
        ]);

        let l_orderkey: PrimitiveArray<Int32Type> =
            PrimitiveArray::from_iter_values(self.orderkey[range.clone()].iter().copied());
        let l_partkey: PrimitiveArray<Int32Type> =
            PrimitiveArray::from_iter_values(self.partkey[range.clone()].iter().copied());
        let l_suppkey: PrimitiveArray<Int32Type> =
            PrimitiveArray::from_iter_values(self.suppkey[range.clone()].iter().copied());
        let l_linenumber: PrimitiveArray<Int32Type> =
            PrimitiveArray::from_iter_values(self.linenumber[range.clone()].iter().copied());
        let l_quantity: PrimitiveArray<Float64Type> =
            PrimitiveArray::from_iter_values(self.quantity[range.clone()].iter().map(|x| f64::try_from(*x).unwrap()));
        let l_extendedprice: PrimitiveArray<Float64Type> = PrimitiveArray::from_iter_values(
            self.extendedprice[range.clone()]
                .iter()
                .map(|x| f64::try_from(*x).unwrap()),
        );
        let l_discount: PrimitiveArray<Float64Type> =
            PrimitiveArray::from_iter_values(self.discount[range.clone()].iter().map(|x| f64::try_from(*x).unwrap()));
        let l_tax: PrimitiveArray<Float64Type> =
            PrimitiveArray::from_iter_values(self.tax[range.clone()].iter().map(|x| f64::try_from(*x).unwrap()));
        let l_returnflag: PrimitiveArray<UInt8Type> = PrimitiveArray::from_iter_values(
            self.returnflag[range.clone()]
                .iter()
                .copied()
                .map(|x| u8::try_from(x).unwrap()),
        );
        let l_linestatus: PrimitiveArray<UInt8Type> = PrimitiveArray::from_iter_values(
            self.linestatus[range.clone()]
                .iter()
                .copied()
                .map(|x| u8::try_from(x).unwrap()),
        );
        let l_shipdate: PrimitiveArray<Int32Type> = PrimitiveArray::from_iter_values(
            self.shipdate[range.clone()]
                .iter()
                .map(|x| x.num_days_from_ce() - NaiveDateTime::UNIX_EPOCH.num_days_from_ce()),
        );
        let l_commitdate: PrimitiveArray<Int32Type> = PrimitiveArray::from_iter_values(
            self.commitdate[range.clone()]
                .iter()
                .map(|x| x.num_days_from_ce() - NaiveDateTime::UNIX_EPOCH.num_days_from_ce()),
        );
        let l_receiptdate: PrimitiveArray<Int32Type> = PrimitiveArray::from_iter_values(
            self.receiptdate[range.clone()]
                .iter()
                .map(|x| x.num_days_from_ce() - NaiveDateTime::UNIX_EPOCH.num_days_from_ce()),
        );
        let l_shipinstruct: StringArray = StringArray::from_iter_values(self.shipinstruct[range.clone()].iter());
        let l_shipmode: StringArray = StringArray::from_iter_values(self.shipmode[range.clone()].iter());
        let l_comment: StringArray = StringArray::from_iter_values(self.comment[range.clone()].iter());

        let record_batch = RecordBatch::try_new(
            Arc::new(schema),
            vec![
                Arc::new(l_orderkey),
                Arc::new(l_partkey),
                Arc::new(l_suppkey),
                Arc::new(l_linenumber),
                Arc::new(l_quantity),
                Arc::new(l_extendedprice),
                Arc::new(l_discount),
                Arc::new(l_tax),
                Arc::new(l_returnflag),
                Arc::new(l_linestatus),
                Arc::new(l_shipdate),
                Arc::new(l_commitdate),
                Arc::new(l_receiptdate),
                Arc::new(l_shipinstruct),
                Arc::new(l_shipmode),
                Arc::new(l_comment),
            ],
        )?;

        Ok(record_batch)
    }

    pub fn len(&self) -> usize {
        self.orderkey.len()
    }
}

fn get_spinner<S: Into<Cow<'static, str>>>(msg: S) -> indicatif::ProgressBar {
    use indicatif::{ProgressBar, ProgressStyle};
    let style = ProgressStyle::with_template(" {spinner:.cyan} {prefix} [{elapsed_precise}] {msg} [{pos}] [{per_sec}]")
        .unwrap();

    let progress = ProgressBar::new_spinner().with_style(style);
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
