use anyhow::Result;
use clap::Parser;
use parquet::{
    basic::{LogicalType, Repetition, Type as PhysicalType},
    file::{
        properties::WriterProperties,
        reader::{FileReader, SerializedFileReader},
        writer::SerializedFileWriter,
    },
    schema::types::Type,
};
use std::{
    borrow::Cow,
    collections::HashSet,
    fs::{File},
    io::{Read},
    path::{Path, PathBuf},
    sync::Arc,
};

#[derive(Parser)]
struct Args {
    #[clap(long)]
    lineitem_csv: PathBuf,
    #[clap(long)]
    wasm_path: PathBuf,
    #[clap(short, long)]
    output_path: PathBuf,
}

fn main() -> Result<()> {
    let args = Args::parse();

    let wasm = read_wasm(&args.wasm_path)?;
    let bytes = read_bytes(&args.lineitem_csv)?;

    let file = File::create(&args.output_path)?;
    let props = WriterProperties::builder()
        .set_dictionary_enabled(true)
        .set_compression(parquet::basic::Compression::UNCOMPRESSED)
        .set_bloom_filter_enabled(false)
        .set_statistics_enabled(parquet::file::properties::EnabledStatistics::Page)
        .build();
    let col_type = Type::primitive_type_builder("linestatus", PhysicalType::INT32)
        .with_logical_type(Some(LogicalType::Integer {
            bit_width: 8,
            is_signed: false,
        }))
        .with_repetition(Repetition::REQUIRED)
        .build()?;
    let schema = Type::group_type_builder("linestatus")
        .with_fields(vec![Arc::new(col_type)])
        .build()?;
    let mut writer = SerializedFileWriter::new(file, schema.into(), props.into())?;

    let mut current_row = 0;
    const ROWS_IN_GROUP: usize = 1 << 20;
    const ROWS_IN_PAGE: usize = 20480;
    while current_row < bytes.len() {
        let mut row_group = writer.next_row_group()?;

        // Encode a row group.
        let last_in_group = current_row + ROWS_IN_GROUP;
        let mut column = row_group.next_column_anyblox::<i32>()?.expect("the one column");
        column.write_decoder(&wasm, "1.0.0".to_string())?;
        let mut bloom_filter = column.get_empty_bloom_filter()?;

        while current_row < last_in_group && current_row < bytes.len() {
            // Encode a single page.
            let last_in_page = current_row + ROWS_IN_PAGE;
            let mut min = None;
            let mut max = None;
            let mut distinct = HashSet::new();
            let mut uncompressed_len = 0;
            let mut num_values = 0;
            let mut encoder = parquet::column::RleEncoder::new(1, ROWS_IN_PAGE);

            while current_row < last_in_page && current_row < last_in_group && current_row < bytes.len() {
                let b = bytes[current_row];
                let b = if b == b'F' { 1 } else { 0 };
                distinct.insert(b);
                encoder.put(b as u64);
                if let Some(min_v) = min {
                    min = Some(std::cmp::min(min_v, b));
                } else {
                    min = Some(b);
                }
                if let Some(max_v) = max {
                    max = Some(std::cmp::max(max_v, b));
                } else {
                    max = Some(b);
                }
                uncompressed_len += 1;
                num_values += 1;
                current_row += 1;
                if let Some(x) = bloom_filter.as_mut() {
                    x.insert(&b);
                }
            }

            let page_data = encoder.consume();
            column.write_data_page(
                &page_data,
                min,
                max,
                0,
                num_values,
                Some(distinct.len() as u64),
                uncompressed_len,
            )?;
        }
        column.close(bloom_filter)?;
        let row_result = row_group.close()?;

        println!("Row group close: {row_result:?}");
    }

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

fn read_wasm(path: &Path) -> Result<Vec<u8>> {
    let mut buf = vec![];
    let mut file = File::open(path)?;
    file.read_to_end(&mut buf)?;
    Ok(buf)
}

fn get_spinner<S: Into<Cow<'static, str>>>(msg: S) -> indicatif::ProgressBar {
    use indicatif::{ProgressBar, ProgressStyle};
    let style = ProgressStyle::with_template(" {spinner:.cyan} {prefix} [{elapsed_precise}] {msg} [{pos}] [{per_sec}]")
        .unwrap();

    let progress = ProgressBar::new_spinner().with_style(style);
    progress.set_prefix(msg);

    progress
}
