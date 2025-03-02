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
    path: PathBuf
}

fn main() -> Result<()> {
    let args = Args::parse();
    let wasm = read_wasm(&args.path)?;
    let hash = blake3::hash(&wasm);
    let hex_string = hash.to_hex();

    println!("{hex_string}");

    Ok(())
}
fn read_wasm(path: &Path) -> Result<Vec<u8>> {
    let mut buf = vec![];
    let mut file = File::open(path)?;
    file.read_to_end(&mut buf)?;
    Ok(buf)
}
