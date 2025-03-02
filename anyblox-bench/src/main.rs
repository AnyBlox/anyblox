use anyhow::{bail, Result};
use clap::{Parser, ValueEnum};
use anyblox::{bundle::AnyBloxBundle, sinks::Sink, ColumnProjection, AnyBloxRuntime};
use anyblox_bench::{Bench, Measurements};
use std::{
    fs,
    os::fd::AsRawFd,
    path::{Path, PathBuf},
    sync::Arc,
};

#[derive(Debug, Parser, Clone)]
pub struct Args {
    /// Path to the AnyBlox file.
    anyblox_path: PathBuf,
    /// At which tuple to start the decoding process.
    start_tuple: usize,
    /// How many tuples to decode.
    tuple_count: usize,
    /// How many tuples to request per one batch.
    batch_size: usize,
    /// Runtime type.
    runtime: Runtime,
    /// Number of threads to partition the job into.
    ///
    /// If not specified the number of (virtual) CPUs minus one is used as default.
    /// The minus one is because there is always one thread dedicated to run the benchmark and record the results.
    #[clap(short, long)]
    threads: Option<usize>,
    /// Optional path to the dataset - required if the AnyBlox file is an Extension.
    #[clap(short, long)]
    dataset_path: Option<PathBuf>,
    /// Do a much quicker benchmark with slightly less precise results.
    #[clap(short, long, required = false, default_value = "false")]
    quick: bool,
    /// Optional column projection. Can be specified multiple times to select which columns to decode.
    #[clap(short, long, required = false)]
    columns: Vec<usize>,
    /// Skip validating UTF8 columns. This is required for safety,
    /// but for benchmarks we want to sometimes measure its impact.
    #[clap(long, required = false, default_value = "false")]
    no_validate_utf8: bool,
}

struct JobConfig {
    engine: Arc<AnyBloxRuntime>,
    bundle: AnyBloxBundle,
    projection: ColumnProjection,
    start_tuple: usize,
    tuple_count: usize,
    batch_size: usize,
    threads: usize,
    precise: bool,
    validate_utf8: bool,
}

#[derive(Debug, Clone, Copy, ValueEnum)]
pub enum Runtime {
    Native,
    Wasm,
}

fn main() -> Result<()> {
    let args = Args::parse();
    let engine = Arc::new(build_engine()?);

    let parse_measurements = Measurements::measure(
        "anyblox bundle parse".to_string(),
        || {},
        |()| {
            open_anyblox_bundle(&args.anyblox_path, args.dataset_path.as_deref()).expect("open_anyblox_bundle");
        },
    );
    print!("anyblox bundle parse {parse_measurements}");

    let anyblox_bundle =
        open_anyblox_bundle(&args.anyblox_path, args.dataset_path.as_deref()).expect("open_anyblox_bundle");
    let metadata = anyblox_bundle.metadata().clone();
    let last_requested_tuple = args.start_tuple + args.tuple_count;
    if last_requested_tuple as u64 > metadata.data().count() {
        bail!(
            "requested more tuples than there are in the dataset ({})",
            metadata.data().count()
        );
    }

    let threads = args.threads.unwrap_or_else(|| num_cpus::get() - 1);
    rayon::ThreadPoolBuilder::new().num_threads(threads).build_global()?;

    let projection = if args.columns.is_empty() {
        ColumnProjection::all(anyblox_bundle.metadata().schema().fields().len())?
    } else {
        ColumnProjection::from_indices(args.columns)?
    };

    let job_config = JobConfig {
        engine,
        bundle: anyblox_bundle,
        projection,
        start_tuple: args.start_tuple,
        tuple_count: args.tuple_count,
        batch_size: args.batch_size,
        threads,
        precise: !args.quick,
        validate_utf8: !args.no_validate_utf8,
    };

    match args.runtime {
        Runtime::Native => {
            bench_native(job_config)?;
        }
        Runtime::Wasm => {
            bench_wasm(job_config)?;
        }
    }

    Ok(())
}

fn bench_native(config: JobConfig) -> Result<()> {
    let engine = config.engine;
    let bundle = config.bundle;

    let metadata = bundle.metadata().clone();
    let in_data_size = bundle.dataset_len();
    let data = unsafe {
        memmap2::MmapOptions::new()
            .len(bundle.dataset_len())
            .offset(bundle.dataset_offset() as u64)
            .map(bundle.dataset_fd().as_raw_fd())
    }?;
    let schema: Arc<arrow::datatypes::Schema> = Arc::new(metadata.schema().into());
    let setup_f = |thread_id| {
        let one_thread_chunk_size = config.tuple_count / config.threads;
        let start_tuple = thread_id * one_thread_chunk_size;
        let tuple_count = if thread_id == config.threads - 1 {
            one_thread_chunk_size + config.tuple_count % config.threads
        } else {
            one_thread_chunk_size
        };
        let job = engine
            .init_native_job_with_projection(
                metadata.decoder().uri(),
                schema.as_ref(),
                config.projection,
                config.validate_utf8,
            )
            .unwrap();

        (job, start_tuple, tuple_count)
    };
    let run_f = |mut job: (anyblox::NativeJob, usize, usize)| {
        let mut rem_tuples = job.2;
        let mut tuple_start = job.1;
        while rem_tuples > 0 {
            let request = std::cmp::min(rem_tuples, config.batch_size);
            let batch = engine
                .run_native_job(&mut job.0, &data, tuple_start, request)
                .expect("run_native");
            tuple_start += batch.row_count();
            rem_tuples -= batch.row_count();
            black_box(batch);
        }
        0
    };

    let bench = Bench::par_run(
        format!("native_{}_{}", metadata.data().name(), metadata.decoder().uri()),
        setup_f,
        run_f,
        config.threads,
        config.precise,
    );

    // Run one last time outside measurements to get the out size.
    let mut job = engine
        .init_native_job_with_projection(
            metadata.decoder().uri(),
            &schema,
            config.projection,
            config.validate_utf8,
        )
        .unwrap();
    let mut rem_tuples = config.tuple_count;
    let mut tuple_start = config.start_tuple;
    let mut sink = anyblox::sinks::TableSink::<anyblox::sinks::ChecksumSink>::new(job.schema());
    while rem_tuples > 0 {
        let request = std::cmp::min(rem_tuples, config.batch_size);
        let batch = engine.run_native_job(&mut job, &data, tuple_start, request)?;
        tuple_start += batch.row_count();
        rem_tuples -= batch.row_count();
        sink.consume_batch(batch)?;
    }

    let path = format!(
        "./exp/native_{}_{}_{}_t{}_b{}_u{}.csv",
        metadata.decoder().uri(),
        metadata.data().name(),
        metadata.ty(),
        config.threads,
        config.batch_size,
        config.validate_utf8
    );
    bench.save_to_file(path)?;

    let result = sink.into_result();
    println!("{}", bench.with_context(in_data_size, result.total_bytes()));
    println!("{result}");

    Ok(())
}

fn bench_wasm(config: JobConfig) -> Result<()> {
    let engine = config.engine;
    let bundle = config.bundle;

    let metadata = bundle.metadata();
    let in_data_size = bundle.dataset_len();
    let setup_f = |thread_id| {
        let one_thread_chunk_size = config.tuple_count / config.threads;
        let start_tuple = thread_id * one_thread_chunk_size;
        let tuple_count = if thread_id == config.threads - 1 {
            one_thread_chunk_size + config.tuple_count % config.threads
        } else {
            one_thread_chunk_size
        };
        let mut job_builder = anyblox::AnyBloxJobParametersBuilder::new();
        job_builder.with_column_projection(config.projection);
        if !config.validate_utf8 {
            job_builder.do_not_validate_utf8();
        }
        let job_params = job_builder.finish(&bundle).expect("incorrect bundle");
        let job = engine.init_blocking_job(job_params).expect("init_blocking_job");
        (job, start_tuple, tuple_count)
    };
    let run_f = |mut job: (anyblox::AnyBloxJob, usize, usize)| {
        let mut rem_tuples = job.2;
        let mut tuple_start = job.1;
        while rem_tuples > 0 {
            let request = std::cmp::min(rem_tuples, config.batch_size);
            let res = engine
                .run_blocking_job(&mut job.0, tuple_start, request)
                .expect("run_blocking_job");
            tuple_start += res.row_count();
            rem_tuples -= res.row_count();
            black_box(res);
        }
        job.0.peak_memory_usage()
    };

    let bench = Bench::par_run(
        format!("wasm_{}_{}", metadata.data().name(), metadata.decoder().uri()),
        setup_f,
        run_f,
        config.threads,
        config.precise,
    );

    // Run one last time outside measurements to get the out size.
    let mut job_builder = anyblox::AnyBloxJobParametersBuilder::new();
    job_builder.with_column_projection(config.projection);
    if !config.validate_utf8 {
        job_builder.do_not_validate_utf8();
    }
    let job_params = job_builder.finish(&bundle).expect("incorrect bundle");

    let mut job = engine.init_blocking_job(job_params)?;
    let mut rem_tuples = config.tuple_count;
    let mut tuple_start = config.start_tuple;
    let mut sink = anyblox::sinks::TableSink::<anyblox::sinks::ChecksumSink>::new(job.schema());
    while rem_tuples > 0 {
        let request = std::cmp::min(rem_tuples, config.batch_size);
        let batch = engine.run_blocking_job(&mut job, tuple_start, request)?;
        tuple_start += batch.row_count();
        rem_tuples -= batch.row_count();
        sink.consume_batch(batch)?;
    }

    let path = format!(
        "./exp/wasm_{}_{}_{}_t{}_b{}_u{}.csv",
        metadata.decoder().uri(),
        metadata.data().name(),
        metadata.ty(),
        config.threads,
        config.batch_size,
        config.validate_utf8
    );
    bench.save_to_file(path)?;

    let result = sink.into_result();
    println!("{}", bench.with_context(in_data_size, result.total_bytes()));
    println!("{result}");

    Ok(())
}

fn build_engine() -> Result<AnyBloxRuntime> {
    let mut config_builder = anyblox::config::ConfigBuilder::new();
    config_builder.set_wasm_cache_limit(64 * 1024 * 1024);
    config_builder.enable_opentelemetry(false);

    let config = config_builder.into();
    let runtime = anyblox::build_engine(config)?;
    Ok(runtime)
}

fn open_anyblox_bundle(anyblox_path: &Path, dataset_path: Option<&Path>) -> Result<AnyBloxBundle> {
    let bundle_file = fs::File::open(anyblox_path)?;
    let bundle_len = bundle_file.metadata()?.len() as usize;
    let dataset_file = if let Some(path) = dataset_path {
        let file = fs::File::open(path)?;
        let len = file.metadata()?.len();
        Some((file, len as usize))
    } else {
        None
    };
    let anyblox_bundle = if let Some((dataset_file, dataset_len)) = dataset_file {
        AnyBloxBundle::new_extension(bundle_file, bundle_len, dataset_file, dataset_len)?
    } else {
        AnyBloxBundle::new_self_contained(bundle_file, bundle_len)?
    };
    Ok(anyblox_bundle)
}

#[inline(never)]
fn black_box<T>(t: T) {
    drop(t)
}
