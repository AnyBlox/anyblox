use anyhow::{bail, Result};
use arrow::array::{Array, AsArray};
use clap::Parser;
use anyblox::{bundle::AnyBloxBundle, AnyBloxJobParametersBuilder, AnyBloxRecordBatch, AnyBloxRuntime};
use indicatif::ProgressBar;
use std::{
    borrow::Cow,
    fmt::Display,
    fs::{self},
    io::Write,
    os::fd::AsRawFd,
    path::{Path, PathBuf},
    sync::Arc,
    time::{Duration, Instant},
};

#[derive(Parser)]
struct Args {
    #[clap(short, long)]
    anyblox_file: PathBuf,
    #[clap(short, long)]
    output_path: PathBuf,
    #[clap(short, long)]
    data_path: Option<PathBuf>,
    /// Run the native implementation instead of the wasm decoder.
    #[clap(long, default_value = "false")]
    native: bool,
    /// Override the default batch size.
    #[clap(short, long, default_value = "100000")]
    batch_size: usize,
    /// Format UInt8 values as characters in the output instead of as integers.
    #[clap(long, default_value = "false")]
    byte_is_char: bool,
    /// Format Int64 values as Decimal128(10, 2).
    #[clap(long, default_value = "false")]
    tpch_decimals: bool,
}

fn main() -> Result<()> {
    let args = Args::parse();
    let bundle = open_bundle(&args.anyblox_file, args.data_path.as_deref())?;
    let runtime = {
        let mut config = anyblox::config::ConfigBuilder::new();
        config.enable_opentelemetry(false);
        config.set_log_level(anyblox::config::LogLevel::Trace);
        anyblox::build_engine(config.into_config())?
    };
    if args.native {
        run_native(
            runtime,
            bundle,
            &args.output_path,
            args.batch_size,
            args.byte_is_char,
            args.tpch_decimals,
        )?;
    } else {
        run(
            runtime,
            bundle,
            &args.output_path,
            args.batch_size,
            args.byte_is_char,
            args.tpch_decimals,
        )?;
    }

    Ok(())
}

fn open_bundle(anyblox_path: &Path, data_path: Option<&Path>) -> Result<AnyBloxBundle> {
    let anyblox_file = fs::File::open(anyblox_path)?;
    let anyblox_len = anyblox_file.metadata()?.len() as usize;
    let bundle = if let Some(data) = data_path {
        let data_file = fs::File::open(data)?;
        let data_len = data_file.metadata()?.len() as usize;
        AnyBloxBundle::new_extension(anyblox_file, anyblox_len, data_file, data_len)
    } else {
        AnyBloxBundle::new_self_contained(anyblox_file, anyblox_len)
    }?;

    Ok(bundle)
}

fn run(
    runtime: AnyBloxRuntime,
    bundle: AnyBloxBundle,
    output_path: &Path,
    batch_size: usize,
    byte_is_char: bool,
    tpch_decimals: bool,
) -> Result<()> {
    let init_start = Instant::now();
    let params = {
        let mut builder = AnyBloxJobParametersBuilder::new();
        builder.finish(&bundle)?
    };
    let mut job = runtime.init_blocking_job(params)?;
    let init_duration = init_start.elapsed();
    let schema: Arc<arrow::datatypes::Schema> = Arc::new(bundle.metadata().schema().into());
    let mut run_duration = Duration::ZERO;

    let mut output_csv = csv::WriterBuilder::new()
        .has_headers(false)
        .delimiter(b'|')
        .from_path(output_path)?;

    let mut start_tuple = 0;
    let mut tuple_count = bundle.metadata().data().count() as usize;
    let progress = ProgressBar::new(tuple_count as u64).with_style(get_progress_bar_style());

    while tuple_count > 0 {
        let chunk_size = std::cmp::min(tuple_count, batch_size);

        let batch_start = Instant::now();
        let batch = runtime.run_blocking_job(&mut job, start_tuple, chunk_size)?;
        run_duration += batch_start.elapsed();
        let row_count = batch.row_count();

        write_batch(&mut output_csv, batch, schema.clone(), byte_is_char, tpch_decimals)?;
        progress.inc(row_count as u64);
        start_tuple += row_count;
        tuple_count -= row_count;
    }

    progress.finish();
    println!(
        "AnyBlox elapsed {} total ({} init, {} batches)",
        FmtDuration(init_duration + run_duration),
        FmtDuration(init_duration),
        FmtDuration(run_duration),
    );

    Ok(())
}

fn run_native(
    runtime: AnyBloxRuntime,
    bundle: AnyBloxBundle,
    output_path: &Path,
    batch_size: usize,
    byte_is_char: bool,
    tpch_decimals: bool,
) -> Result<()> {
    let init_start = Instant::now();
    let schema: ::arrow::datatypes::Schema = bundle.metadata().schema().into();
    let mut job = runtime.init_native_job(bundle.metadata().decoder().uri(), &schema, true)?;
    let mmap = unsafe {
        memmap2::MmapOptions::new()
            .len(bundle.dataset_len())
            .offset(bundle.dataset_offset() as u64)
            .map(bundle.dataset_fd().as_raw_fd())
    }?;
    let init_duration = init_start.elapsed();
    let schema: Arc<arrow::datatypes::Schema> = Arc::new(bundle.metadata().schema().into());
    let mut run_duration = Duration::ZERO;

    let mut output_csv = csv::WriterBuilder::new()
        .has_headers(false)
        .delimiter(b'|')
        .from_path(output_path)?;

    let mut start_tuple = 0;
    let mut tuple_count = bundle.metadata().data().count() as usize;
    let progress = ProgressBar::new(tuple_count as u64).with_style(get_progress_bar_style());

    while tuple_count > 0 {
        let chunk_size = std::cmp::min(tuple_count, batch_size);

        let batch_start = Instant::now();
        let batch = runtime.run_native_job(&mut job, &mmap, start_tuple, chunk_size)?;
        run_duration += batch_start.elapsed();
        let row_count = batch.row_count();

        write_batch(&mut output_csv, batch, schema.clone(), byte_is_char, tpch_decimals)?;
        progress.inc(row_count as u64);
        start_tuple += row_count;
        tuple_count -= row_count;
    }

    progress.finish();
    println!(
        "AnyBlox elapsed {} total ({} init, {} batches)",
        FmtDuration(init_duration + run_duration),
        FmtDuration(init_duration),
        FmtDuration(run_duration),
    );

    Ok(())
}

fn write_batch<W: Write>(
    csv: &mut csv::Writer<W>,
    batch: AnyBloxRecordBatch,
    schema: Arc<arrow::datatypes::Schema>,
    byte_is_char: bool,
    tpch_decimals: bool,
) -> Result<()> {
    let columns = BatchStringColumns::new(batch, schema, byte_is_char, tpch_decimals)?;
    for row in columns.iter() {
        let mut string_record = csv::StringRecord::new();
        for col in row {
            string_record.push_field(col.as_deref().unwrap_or("NULL"));
        }
        csv.write_byte_record(string_record.as_byte_record())?;
    }
    Ok(())
}

struct BatchStringColumns {
    batch: arrow::record_batch::RecordBatch,
    column_readers: Vec<Box<dyn ColumnReader>>,
}

struct BatchStringColumnsIterator<'a> {
    inner: &'a BatchStringColumns,
    idx: usize,
}

impl BatchStringColumns {
    fn new(
        batch: AnyBloxRecordBatch,
        schema: Arc<arrow::datatypes::Schema>,
        byte_is_char: bool,
        tpch_decimals: bool,
    ) -> Result<Self> {
        let batch = batch.into_arrow_record_batch(schema)?;
        let column_readers = batch
            .columns()
            .iter()
            .map(|a| create_column_reader(a, byte_is_char, tpch_decimals))
            .collect::<Result<Vec<_>>>()?;
        Ok(Self { batch, column_readers })
    }

    fn iter(&self) -> BatchStringColumnsIterator {
        BatchStringColumnsIterator { inner: self, idx: 0 }
    }
}

impl<'a> Iterator for BatchStringColumnsIterator<'a> {
    type Item = Vec<Option<Cow<'a, str>>>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.idx >= self.inner.batch.num_rows() {
            return None;
        }

        let mut item = Vec::with_capacity(self.inner.column_readers.len());
        for reader in &self.inner.column_readers {
            item.push(reader.get(self.idx));
        }
        self.idx += 1;
        Some(item)
    }
}

fn create_column_reader(
    array: &Arc<dyn arrow::array::Array>,
    byte_is_char: bool,
    tpch_decimals: bool,
) -> Result<Box<dyn ColumnReader>> {
    use arrow::datatypes::*;
    match array.data_type() {
        DataType::Utf8 => {
            let string_array = array.as_string::<i32>().clone();
            let reader = StringArrayReader::new(string_array);
            Ok(Box::new(reader))
        }
        DataType::LargeUtf8 => {
            let string_array = array.as_string::<i64>().clone();
            let reader = StringArrayReader::new(string_array);
            Ok(Box::new(reader))
        }
        DataType::Utf8View => {
            let string_array = array.as_string_view().clone();
            let reader = StringViewArrayReader::new(string_array);
            Ok(Box::new(reader))
        }
        DataType::Int8 => {
            let primitive_array = array.as_primitive::<Int8Type>().clone();
            let reader = PrimitiveArrayReader::new(primitive_array);
            Ok(Box::new(reader))
        }
        DataType::Int16 => {
            let primitive_array = array.as_primitive::<Int16Type>().clone();
            let reader = PrimitiveArrayReader::new(primitive_array);
            Ok(Box::new(reader))
        }
        DataType::Int32 => {
            let primitive_array = array.as_primitive::<Int32Type>().clone();
            let reader = PrimitiveArrayReader::new(primitive_array);
            Ok(Box::new(reader))
        }
        DataType::Int64 if tpch_decimals => {
            let primitive_array = array.as_primitive::<Int64Type>().clone();
            let reader = FakeTpchDecimalArrayReader::new(primitive_array);
            Ok(Box::new(reader))
        }
        DataType::Int64 if !tpch_decimals => {
            let primitive_array = array.as_primitive::<Int64Type>().clone();
            let reader = PrimitiveArrayReader::new(primitive_array);
            Ok(Box::new(reader))
        }
        DataType::UInt8 if byte_is_char => {
            let primitive_array = array.as_primitive::<UInt8Type>().clone();
            let reader = CharArrayReader::new(primitive_array);
            Ok(Box::new(reader))
        }
        DataType::UInt8 if !byte_is_char => {
            let primitive_array = array.as_primitive::<UInt8Type>().clone();
            let reader = PrimitiveArrayReader::new(primitive_array);
            Ok(Box::new(reader))
        }
        DataType::UInt16 => {
            let primitive_array = array.as_primitive::<UInt16Type>().clone();
            let reader = PrimitiveArrayReader::new(primitive_array);
            Ok(Box::new(reader))
        }
        DataType::UInt32 => {
            let primitive_array = array.as_primitive::<UInt32Type>().clone();
            let reader = PrimitiveArrayReader::new(primitive_array);
            Ok(Box::new(reader))
        }
        DataType::UInt64 => {
            let primitive_array = array.as_primitive::<UInt64Type>().clone();
            let reader = PrimitiveArrayReader::new(primitive_array);
            Ok(Box::new(reader))
        }
        DataType::Date32 => {
            let primitive_array = array.as_primitive::<Date32Type>().clone();
            let reader = Date32ArrayReader::new(primitive_array);
            Ok(Box::new(reader))
        }
        _ => bail!("unsupported datatype: {}", array.data_type()),
    }
}

trait ColumnReader {
    fn get(&self, idx: usize) -> Option<Cow<str>>;
}

struct StringArrayReader<O: arrow::array::OffsetSizeTrait> {
    array: arrow::array::GenericStringArray<O>,
}

struct StringViewArrayReader {
    array: arrow::array::GenericByteViewArray<arrow::datatypes::StringViewType>,
}

struct Date32ArrayReader {
    array: arrow::array::PrimitiveArray<arrow::datatypes::Date32Type>,
}

struct PrimitiveArrayReader<N: arrow::array::ArrowPrimitiveType> {
    array: arrow::array::PrimitiveArray<N>,
}

struct CharArrayReader {
    array: arrow::array::PrimitiveArray<arrow::datatypes::UInt8Type>,
}

struct FakeTpchDecimalArrayReader {
    array: arrow::array::PrimitiveArray<arrow::datatypes::Int64Type>,
}

impl<O: arrow::array::OffsetSizeTrait> StringArrayReader<O> {
    pub fn new(array: arrow::array::GenericStringArray<O>) -> Self {
        Self { array }
    }
}

impl StringViewArrayReader {
    pub fn new(array: arrow::array::GenericByteViewArray<arrow::datatypes::StringViewType>) -> Self {
        Self { array }
    }
}

impl Date32ArrayReader {
    pub fn new(array: arrow::array::PrimitiveArray<arrow::datatypes::Date32Type>) -> Self {
        Self { array }
    }
}

impl<N: arrow::array::ArrowPrimitiveType> PrimitiveArrayReader<N> {
    pub fn new(array: arrow::array::PrimitiveArray<N>) -> Self {
        Self { array }
    }
}

impl CharArrayReader {
    pub fn new(array: arrow::array::PrimitiveArray<arrow::datatypes::UInt8Type>) -> Self {
        Self { array }
    }
}

impl FakeTpchDecimalArrayReader {
    pub fn new(array: arrow::array::PrimitiveArray<arrow::datatypes::Int64Type>) -> Self {
        Self { array }
    }
}

impl<O: arrow::array::OffsetSizeTrait> ColumnReader for StringArrayReader<O> {
    fn get(&self, idx: usize) -> Option<Cow<str>> {
        if self.array.is_null(idx) {
            None
        } else {
            Some(Cow::Borrowed(self.array.value(idx)))
        }
    }
}

impl ColumnReader for StringViewArrayReader {
    fn get(&self, idx: usize) -> Option<Cow<str>> {
        if self.array.is_null(idx) {
            None
        } else {
            Some(Cow::Borrowed(self.array.value(idx)))
        }
    }
}

impl ColumnReader for Date32ArrayReader {
    fn get(&self, idx: usize) -> Option<Cow<str>> {
        use chrono::{Days, NaiveDateTime};
        if self.array.is_null(idx) {
            None
        } else {
            let days = self.array.value(idx);
            let date = if days > 0 {
                NaiveDateTime::UNIX_EPOCH
                    .date()
                    .checked_add_days(Days::new(days as u64))
                    .unwrap()
            } else {
                NaiveDateTime::UNIX_EPOCH
                    .date()
                    .checked_sub_days(Days::new(-days as u64))
                    .unwrap()
            };
            Some(Cow::Owned(date.format("%Y-%m-%d").to_string()))
        }
    }
}

impl<N: arrow::array::ArrowPrimitiveType> ColumnReader for PrimitiveArrayReader<N>
where
    N::Native: ToString,
{
    fn get(&self, idx: usize) -> Option<Cow<str>> {
        if self.array.is_null(idx) {
            None
        } else {
            Some(Cow::Owned(self.array.value(idx).to_string()))
        }
    }
}

impl ColumnReader for CharArrayReader {
    fn get(&self, idx: usize) -> Option<Cow<str>> {
        if self.array.is_null(idx) {
            None
        } else {
            Some(Cow::Owned((self.array.value(idx) as char).to_string()))
        }
    }
}

impl ColumnReader for FakeTpchDecimalArrayReader {
    fn get(&self, idx: usize) -> Option<Cow<str>> {
        if self.array.is_null(idx) {
            None
        } else {
            let int = self.array.value(idx);
            let after_decimal = int.abs() % 100;
            let before_decimal = int.abs() / 100;
            let string = format!("{}{before_decimal}.{after_decimal:0>2}", if int < 0 { "-" } else { "" });
            Some(Cow::Owned(string))
        }
    }
}

fn get_progress_bar_style() -> indicatif::ProgressStyle {
    use indicatif::ProgressStyle;
    ProgressStyle::with_template(" {spinner:.cyan} {prefix} [{elapsed_precise}] {wide_bar} {msg} [{per_sec}] [{eta}]")
        .unwrap()
}

struct FmtDuration(Duration);

impl Display for FmtDuration {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{: >8.4}ms", self.0.as_secs_f32() * 1_000.0)
    }
}
