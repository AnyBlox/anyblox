use anyhow::{bail, Result};
use arrow::{
    array::{PrimitiveBuilder, RecordBatch},
    datatypes::{DataType, Field, Int32Type, SchemaBuilder},
};
use clap::{Parser, ValueEnum};
use parquet::{
    arrow::ArrowWriter,
    basic::{Compression, Encoding},
    file::properties::{WriterProperties, WriterPropertiesBuilder},
};
use std::{
    fs::File,
    io::{self, BufReader, Read},
    path::PathBuf,
    sync::Arc,
    time::Duration,
};

#[derive(Parser)]
struct Args {
    input_path: PathBuf,
    output_path: PathBuf,
    #[clap(short, long)]
    compression: Option<CompressionArg>,
    #[clap(short, long)]
    encoding: Option<EncodingArg>,
}

#[derive(Clone, Copy, PartialEq, Eq, ValueEnum)]
enum CompressionArg {
    Snappy,
}

#[derive(Clone, Copy, PartialEq, Eq, ValueEnum)]
enum EncodingArg {
    Rle,
    Dictionary,
}

fn main() -> Result<()> {
    let args = Args::parse();

    match args.input_path.extension() {
        Some(x) if x == "integer" => write_int(args)?,
        Some(x) if x == "string" => write_string(args)?,
        _ => bail!("unsupported input file extension"),
    };

    Ok(())
}

fn write_int(args: Args) -> Result<()> {
    let file = File::open(&args.input_path)?;
    let len = file.metadata()?.len();
    let mut input = BufReader::new(file);
    let mut int_buffer: PrimitiveBuilder<Int32Type> = PrimitiveBuilder::new();
    let mut read_buf = [0; 4];
    let progress_bar = indicatif::ProgressBar::new(len).with_style(get_progress_bar_style());
    progress_bar.set_message("reading raw...");

    loop {
        match input.read_exact(&mut read_buf) {
            Ok(_) => {
                let int = i32::from_le_bytes(read_buf);
                int_buffer.append_value(int);
                progress_bar.inc(4);
            }
            Err(err) if err.kind() == io::ErrorKind::UnexpectedEof => break,
            Err(err) => return Err(err.into()),
        }
    }

    progress_bar.finish();
    let spinner = indicatif::ProgressBar::new_spinner().with_style(get_spinner_style());
    spinner.set_prefix("writing parquet...");
    spinner.enable_steady_tick(Duration::from_millis(250));
    let schema = {
        let mut builder = SchemaBuilder::new();
        builder.push(Field::new("col1", DataType::Int32, false));
        builder.finish()
    };
    let ints = int_buffer.finish();
    let batch = RecordBatch::try_new(Arc::new(schema), vec![Arc::new(ints)])?;

    let props = WriterProperties::builder();
    let props = set_compression(props, args.compression);
    let props = set_encoding(props, args.encoding);

    let out_file = File::create(&args.output_path)?;
    let mut writer = ArrowWriter::try_new(out_file, batch.schema(), Some(props.build()))?;

    writer.write(&batch)?;
    writer.close()?;

    spinner.finish();

    Ok(())
}

fn write_string(_args: Args) -> Result<()> {
    todo!()
}

fn set_compression(builder: WriterPropertiesBuilder, compression: Option<CompressionArg>) -> WriterPropertiesBuilder {
    match compression {
        None => builder.set_compression(Compression::UNCOMPRESSED),
        Some(CompressionArg::Snappy) => builder.set_compression(Compression::SNAPPY),
    }
}

fn set_encoding(builder: WriterPropertiesBuilder, encoding: Option<EncodingArg>) -> WriterPropertiesBuilder {
    match encoding {
        None => builder.set_dictionary_enabled(false).set_encoding(Encoding::PLAIN),
        Some(EncodingArg::Rle) => builder.set_dictionary_enabled(false).set_encoding(Encoding::RLE),
        Some(EncodingArg::Dictionary) => builder.set_dictionary_enabled(true),
    }
}

fn get_progress_bar_style() -> indicatif::ProgressStyle {
    use indicatif::ProgressStyle;
    ProgressStyle::with_template(
        " {spinner:.cyan} {prefix} [{elapsed_precise}] {wide_bar} {msg} [{decimal_bytes_per_sec}]",
    )
    .unwrap()
}

fn get_spinner_style() -> indicatif::ProgressStyle {
    use indicatif::ProgressStyle;
    ProgressStyle::with_template(" {spinner:.cyan} {prefix} [{elapsed_precise}] {msg} [{pos}] [{per_sec}]").unwrap()
}
