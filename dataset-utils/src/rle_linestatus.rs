use anyhow::Result;
use clap::Parser;
use std::{
    borrow::Cow,
    fs::{self},
    io::{self, Seek, Write},
    path::{Path, PathBuf},
    sync::Arc,
    time::Duration,
};

#[derive(Parser)]
struct Args {
    #[clap(long)]
    lineitem_csv: PathBuf,
    #[clap(short, long)]
    output_directory: PathBuf,
}

fn main() -> Result<()> {
    let args = Args::parse();

    let bytes = read_bytes(&args.lineitem_csv)?;

    let output_binary_path = args.output_directory.join("tpch-linestatus-rlebitpack.bin");
    let output_csv_path = args.output_directory.join("tpch-linestatus-rlebitpack.csv");
    let output_parquet_path = args.output_directory.join("tpch-linestatus-rlebitpack.parquet");

    write_binary(&bytes, &output_binary_path)?;
    write_csv(&bytes, &output_csv_path)?;
    write_parquet(&bytes, &output_parquet_path)?;

    Ok(())
}

fn write_binary<P: AsRef<Path>>(bytes: &[u8], output_path: P) -> Result<()> {
    let mut writer = io::BufWriter::new(fs::File::create(&output_path)?);

    let mut current_row = 0;
    const ROWS_IN_PAGE: usize = 8192;

    let mut idx = 0_usize;
    let total_pages = bytes.len().div_ceil(ROWS_IN_PAGE);
    writer.write_all(&(total_pages as u32).to_le_bytes())?;
    writer.write_all(&(ROWS_IN_PAGE as u32).to_le_bytes())?;
    idx += 8;
    for _ in 0..total_pages {
        writer.write_all(&[0, 0, 0, 0])?;
        idx += 4;
    }
    let mut page_starts = Vec::with_capacity(total_pages);

    while current_row < bytes.len() {
        page_starts.push(idx);

        // Encode a single page.
        let last_in_page = current_row + ROWS_IN_PAGE;
        let mut encoder = parquet::column::RleEncoder::new(1, ROWS_IN_PAGE);

        while current_row < last_in_page && current_row < bytes.len() {
            let b = bytes[current_row];
            let b = if b == b'F' { 1 } else { 0 };
            encoder.put(b as u64);
            current_row += 1;
        }

        let page_data = encoder.consume();
        writer.write_all(&page_data)?;
        idx += page_data.len();
    }

    // Now fixup the offsets.
    let mut writer = io::BufWriter::new(fs::File::options().write(true).open(&output_path)?);
    writer.seek(io::SeekFrom::Start(8))?;
    for offset in page_starts {
        let off = offset as u32;
        writer.write_all(&off.to_le_bytes())?;
    }

    Ok(())
}

fn write_csv<P: AsRef<Path>>(bytes: &[u8], output_path: P) -> Result<()> {
    let mut csv = csv::WriterBuilder::new()
        .delimiter(b'|')
        .has_headers(false)
        .from_path(output_path)?;

    for b in bytes {
        csv.write_record([String::from(*b as char)])?;
    }

    Ok(())
}

fn write_parquet<P: AsRef<Path>>(bytes: &[u8], output_path: P) -> Result<()> {
    use arrow::{
        array::{PrimitiveBuilder, RecordBatch},
        datatypes::{DataType, Field, SchemaBuilder, UInt8Type},
    };
    use parquet::{
        arrow::ArrowWriter,
        basic::Compression,
        file::properties::WriterProperties,
    };

    let mut byte_buffer: PrimitiveBuilder<UInt8Type> = PrimitiveBuilder::new();

    byte_buffer.append_slice(bytes);

    let spinner = get_spinner("writing parquet...");
    spinner.enable_steady_tick(Duration::from_millis(250));
    let schema = {
        let mut builder = SchemaBuilder::new();
        builder.push(Field::new("col1", DataType::UInt8, false));
        builder.finish()
    };
    let byte_array = byte_buffer.finish();
    let batch = RecordBatch::try_new(Arc::new(schema), vec![Arc::new(byte_array)])?;

    let props = WriterProperties::builder().set_compression(Compression::UNCOMPRESSED);

    let out_file = fs::File::create(&output_path)?;
    let mut writer = ArrowWriter::try_new(out_file, batch.schema(), Some(props.build()))?;

    writer.write(&batch)?;
    writer.close()?;

    spinner.finish();

    Ok(())
}

fn read_bytes(path: &Path) -> Result<Vec<u8>> {
    let mut csv = csv::ReaderBuilder::new()
        .delimiter(b'|')
        .has_headers(false)
        .from_path(path)?;
    let mut bytes = vec![];
    let spinner = get_spinner("reading csv...");

    for record in csv.records() {
        let record = record?;
        let str = record.get(9).expect("no column with given index");
        assert!(str.len() == 1);
        bytes.push(str.as_bytes()[0]);

        spinner.inc(1);
    }
    spinner.finish();

    Ok(bytes)
}

fn get_spinner<S: Into<Cow<'static, str>>>(msg: S) -> indicatif::ProgressBar {
    use indicatif::{ProgressBar, ProgressStyle};
    let style = ProgressStyle::with_template(" {spinner:.cyan} {prefix} [{elapsed_precise}] {msg} [{pos}] [{per_sec}]")
        .unwrap();

    let progress = ProgressBar::new_spinner().with_style(style);
    progress.set_prefix(msg);

    progress
}
