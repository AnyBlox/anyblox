#![feature(core_intrinsics)]

use anyblox::{bundle::AnyBloxBundle, AnyBloxJobParametersBuilder, AnyBloxRuntime};
use anyblox_bench::Bench;
use anyhow::Result;
use clap::{Args, Parser, Subcommand, ValueEnum};
use sha2::Digest as Sha2Digest;
use sha3::Digest as Sha3Digest;
use std::intrinsics::black_box;
use std::io::Read;
use std::{
    fs,
    path::{Path, PathBuf},
    sync::{atomic::AtomicBool, Arc},
    time::Instant,
};
use streebog::Digest as StreebogDigest;
use whirlpool::Digest as WhirlpoolDigest;

#[derive(ValueEnum, Debug, Clone, Copy)]
pub enum HashAlgo {
    Blake3,
    Sha2_256,
    Sha2_512,
    Sha3_256,
    Sha3_512,
    Whirlpool,
    Streebog_256,
    Streebog_512,
}

#[derive(Debug, Parser, Clone)]
pub struct Cli {
    #[clap(short, long)]
    path: PathBuf,
    #[clap(short, long)]
    algo: HashAlgo,
}

fn main() -> Result<()> {
    let args = Cli::parse();
    let decoder = read_wasm(&args.path)?;
    measure(&decoder, args.algo)?;

    Ok(())
}

fn measure(decoder: &[u8], algo: HashAlgo) -> Result<()> {
    match algo {
        HashAlgo::Blake3 => measure_blake3(decoder),
        HashAlgo::Sha2_256 => measure_sha2_256(decoder),
        HashAlgo::Sha2_512 => measure_sha2_512(decoder),
        HashAlgo::Sha3_256 => measure_sha3_256(decoder),
        HashAlgo::Sha3_512 => measure_sha3_512(decoder),
        HashAlgo::Whirlpool => measure_whirlpool(decoder),
        HashAlgo::Streebog_256 => measure_streebog_256(decoder),
        HashAlgo::Streebog_512 => measure_streebog_512(decoder),
    }?;

    Ok(())
}

macro_rules! bench {
    ($fun:ident, $name:expr, $call:expr) => {
        fn $fun(decoder: &[u8]) -> Result<()> {
            let bench = Bench::run(
                $name.to_string(),
                || {},
                |_| {
                    let hash = $call(decoder);
                    black_box(hash);
                },
                true,
            );
            println!("{} bench: {bench}", $name);
            Ok(())
        }
    };
}

bench!(measure_blake3, "blake3", |d| blake3::hash(d));
bench!(measure_sha2_256, "sha2_256", |d| sha2::Sha256::digest(d));
bench!(measure_sha2_512, "sha2_512", |d| sha2::Sha512::digest(d));
bench!(measure_sha3_256, "sha3_256", |d| sha3::Sha3_256::digest(d));
bench!(measure_sha3_512, "sha3_512", |d| sha3::Sha3_512::digest(d));
bench!(measure_whirlpool, "whirlpool", |d| whirlpool::Whirlpool::digest(d));
bench!(measure_streebog_256, "streebog_256", |d| streebog::Streebog256::digest(
    d
));
bench!(measure_streebog_512, "streebog_512", |d| streebog::Streebog512::digest(
    d
));

fn read_wasm(path: &Path) -> Result<Vec<u8>> {
    let mut file = fs::File::open(path)?;
    let mut bytes = vec![];
    file.read_to_end(&mut bytes)?;

    Ok(bytes)
}
