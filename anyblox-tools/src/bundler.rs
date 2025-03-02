use anyhow::{anyhow, Result};
use clap::Parser;
use anyblox_format::model::*;
use std::{
    fs::File,
    io::{BufWriter, Read},
    path::{Path, PathBuf},
};

#[derive(Parser)]
struct Args {
    #[clap(long)]
    metadata_path: PathBuf,
    #[clap(long)]
    wasm_path: PathBuf,
    #[clap(short, long)]
    output_path: PathBuf,
    #[clap(long)]
    data_path: Option<PathBuf>,
}

fn main() -> Result<()> {
    let args = Args::parse();
    let metadata = anyblox_format::de::deserialize_metadata_from_file(&args.metadata_path)?;
    let wasm = read_wasm(&args.wasm_path)?;

    match metadata.ty() {
        AnyBloxType::SelfContained => {
            if let Some(data_path) = args.data_path.as_ref() {
                write_self_contained(metadata, &wasm, data_path, &args.output_path)?;
            } else {
                return Err(anyhow!("self-contained anyblox format but no data path provided"));
            }
        }
        AnyBloxType::Extension => write_extension(metadata, &wasm, &args.output_path)?,
    }

    Ok(())
}

fn write_extension(metadata: Metadata, wasm: &[u8], output_path: &Path) -> Result<()> {
    let file = File::create(output_path)?;
    let writer = BufWriter::new(file);

    anyblox_format::ser::serialize_extension(&metadata, wasm, writer)?;
    Ok(())
}

fn write_self_contained(metadata: Metadata, wasm: &[u8], data_path: &Path, output_path: &Path) -> Result<()> {
    let file = File::create(output_path)?;
    let data = File::open(data_path)?;
    let writer = BufWriter::new(file);
    let data_mmap = unsafe { memmap2::Mmap::map(&data) }?;

    anyblox_format::ser::serialize_self_contained(&metadata, wasm, &data_mmap, writer)?;
    Ok(())
}

fn read_wasm(path: &Path) -> Result<Vec<u8>> {
    let mut buf = vec![];
    let mut file = File::open(path)?;
    file.read_to_end(&mut buf)?;
    Ok(buf)
}
