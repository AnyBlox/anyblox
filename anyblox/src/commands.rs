use clap::{Args, Parser, Subcommand};
use std::path::{Path, PathBuf};

/// Experimental runtime for portable decompression.
#[derive(Debug, Parser)]
#[command(no_binary_name = true)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Debug, Subcommand, Clone)]
pub enum Command {
    /// Run a program on a dataset.
    Run(RunArgs),
    /// Run the native algorithm on a dataset.
    Native(NativeArgs),
    /// Exit the runtime.
    #[command(alias = "q")]
    #[command(alias = "exit")]
    Quit,
}

#[derive(Debug, Args, Clone)]
#[command(flatten_help = true)]
pub struct RunArgs {
    /// Path to the AnyBlox file.
    anyblox_path: PathBuf,
    /// At which tuple to start the decoding process.
    tuple_start: usize,
    /// How many tuples to decode.
    tuple_count: usize,
    /// Optional path to the dataset - required if the AnyBlox file is an Extension.
    dataset_path: Option<PathBuf>,
    #[clap(short, long, required = false, default_value = "true")]
    /// Whether to compute a hash of all decoded data - heavily affects performance.
    verify: bool,
    #[clap(short, long, required = false)]
    columns: Vec<usize>,
    #[clap(short, long, required = false)]
    batch_size: Option<usize>,
}

#[derive(Debug, Args, Clone)]
#[command(flatten_help = true)]
pub struct NativeArgs {
    /// Path to the AnyBlox file.
    anyblox_path: PathBuf,
    /// At which tuple to start the decoding process.
    tuple_start: usize,
    /// How many tuples to decode.
    tuple_count: usize,
    /// Optional path to the dataset - required if the AnyBlox file is an Extension.
    dataset_path: Option<PathBuf>,
    #[clap(short, long, required = false, default_value = "true")]
    /// Whether to compute a hash of all decoded data - heavily affects performance.
    verify: bool,
    #[clap(short, long, required = false)]
    columns: Vec<usize>,
    #[clap(short, long, required = false)]
    batch_size: Option<usize>,
}

#[derive(Debug, Args, Clone)]
#[command(flatten_help = true)]
pub struct BenchArgs {
    /// Path to the AnyBlox file.
    anyblox_path: PathBuf,
    /// Optional path to the dataset - required if the AnyBlox file is an Extension.
    dataset_path: Option<PathBuf>,
}

impl Command {
    pub fn parse(str: String) -> Result<Self, clap::Error> {
        let iter = str.split_ascii_whitespace();
        Cli::try_parse_from(iter).map(|x| x.command)
    }
}

impl RunArgs {
    pub fn anyblox_path(&self) -> &Path {
        &self.anyblox_path
    }

    pub fn dataset_path(&self) -> Option<&Path> {
        self.dataset_path.as_deref()
    }

    pub fn tuple_start(&self) -> usize {
        self.tuple_start
    }

    pub fn tuple_count(&self) -> usize {
        self.tuple_count
    }

    pub fn verify(&self) -> bool {
        self.verify
    }

    pub fn columns(&self) -> &[usize] {
        &self.columns
    }

    pub fn batch_size(&self) -> Option<usize> {
        self.batch_size
    }
}

impl NativeArgs {
    pub fn anyblox_path(&self) -> &Path {
        &self.anyblox_path
    }

    pub fn dataset_path(&self) -> Option<&Path> {
        self.dataset_path.as_deref()
    }

    pub fn tuple_start(&self) -> usize {
        self.tuple_start
    }

    pub fn tuple_count(&self) -> usize {
        self.tuple_count
    }

    pub fn verify(&self) -> bool {
        self.verify
    }

    pub fn columns(&self) -> &[usize] {
        &self.columns
    }

    pub fn batch_size(&self) -> Option<usize> {
        self.batch_size
    }
}

impl BenchArgs {
    pub fn anyblox_path(&self) -> &Path {
        &self.anyblox_path
    }

    pub fn dataset_path(&self) -> Option<&Path> {
        self.dataset_path.as_deref()
    }
}
