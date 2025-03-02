use anyhow::Result;
use arrow::{
    array::PrimitiveBuilder,
    datatypes::SchemaBuilder,
};
use clap::Parser;
use parquet::{
    arrow::ArrowWriter,
    file::{
        properties::WriterProperties,
        reader::{FileReader, SerializedFileReader},
    },
};
use std::{
    borrow::Cow,
    fs::File,
    path::{Path, PathBuf},
    sync::Arc,
};

#[derive(Parser)]
struct Args {
    #[clap(long)]
    lineitem_csv: PathBuf,
    #[clap(short, long)]
    output_path: PathBuf,
}

fn main() -> Result<()> {
    let args = Args::parse();

    let bytes = read_bytes(&args.lineitem_csv)?;

    let encoded = bytes
        .into_iter()
        .map(|x| if x == b'F' { 1 } else { 0 })
        .collect::<Vec<_>>();
    let arrow_schema = {
        let mut builder = SchemaBuilder::new();
        builder.push(arrow::datatypes::Field::new(
            "linestatus",
            arrow::datatypes::DataType::UInt8,
            false,
        ));
        Arc::new(builder.finish())
    };
    let arrow_array = {
        let mut builder = PrimitiveBuilder::<arrow::datatypes::UInt8Type>::new();
        builder.append_slice(&encoded);
        builder.finish()
    };
    let batch = arrow::record_batch::RecordBatch::try_new(arrow_schema.clone(), vec![Arc::new(arrow_array)])?;

    let file = File::create(&args.output_path)?;
    let props = WriterProperties::builder()
        .set_dictionary_enabled(true)
        .set_compression(parquet::basic::Compression::UNCOMPRESSED)
        .set_bloom_filter_enabled(false)
        .set_statistics_enabled(parquet::file::properties::EnabledStatistics::Page)
        .build();
    let mut writer = ArrowWriter::try_new(file, arrow_schema, Some(props))?;

    writer.write(&batch)?;
    writer.close()?;

    // Read

    let file = File::open(&args.output_path)?;
    let reader = SerializedFileReader::new(file)?;
    let meta = reader.metadata();
    println!("Metadata:");
    println!("#row groups: {}", meta.num_row_groups());
    println!("version: {}", meta.file_metadata().version());
    println!("schema: {:?}", meta.file_metadata().schema());
    println!("#rows: {}", meta.file_metadata().num_rows());
    println!("created by: {:?}", meta.file_metadata().created_by());
    println!("kvs: {:?}", meta.file_metadata().key_value_metadata());
    println!("col orders: {:?}", meta.file_metadata().column_orders());

    let row_group = reader.get_row_group(0)?;
    println!("Row group:");
    println!("compressed size: {}", row_group.metadata().compressed_size());
    println!("file offset: {:?}", row_group.metadata().file_offset());
    println!("#column chunks: {}", row_group.metadata().num_columns());
    println!("#rows: {}", row_group.metadata().num_rows());
    println!("total byte size: {}", row_group.metadata().total_byte_size());

    let bloom = row_group.get_column_bloom_filter(0);
    println!("Bloom: {bloom:?}");

    let column = row_group.metadata().column(0);
    println!("Column metadata:");
    println!("bloom filter length: {:?}", column.bloom_filter_length());
    println!("bloom filter offset: {:?}", column.bloom_filter_offset());
    println!("byte range: {:?}", column.byte_range());
    println!("descr: {:?}", column.column_descr());
    println!("column index length: {:?}", column.column_index_length());
    println!("column index offset: {:?}", column.column_index_offset());
    println!("compression: {}", column.compression());
    println!("data page offset: {}", column.data_page_offset());
    println!("dict page offset: {:?}", column.dictionary_page_offset());
    println!("file offset: {:?}", column.file_offset());
    println!("file path: {:?}", column.file_path());
    println!("index page offset: {:?}", column.index_page_offset());
    println!("offset index length: {:?}", column.offset_index_length());
    println!("offset index offset: {:?}", column.offset_index_offset());
    println!("page encoding stats: {:?}", column.page_encoding_stats());
    println!("uncompressed size: {:?}", column.uncompressed_size());

    let mut column = row_group.get_column_page_reader(0)?;
    let mut i = 0;

    while let Some(page) = column.get_next_page()? {
        println!("Column page {i}:");
        println!("encoding: {}", page.encoding());
        println!("type: {}", page.page_type());
        println!("#values: {}", page.num_values());
        println!("stats: {:?}", page.statistics());
        println!("len: {}", page.buffer().len());
        i += 1;
    }

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
