use anyhow::{bail, Context, Result};
use clap::Parser;
use anyblox::{build_engine, config, AnyBloxRuntime, NativeJob};
use rustyline::error::ReadlineError;
use std::{
    os::fd::AsRawFd,
    path::Path,
    sync::Arc,
    time::{Duration, Instant},
};
use tokio::fs;
use tracing::{debug, info};

#[derive(Debug, Parser)]
struct Args {
    #[clap(long)]
    wasm_cache_limit: Option<usize>,
}

const DEFAULT_BATCH_SIZE: usize = 100_000;

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();
    let mut rl = rustyline::DefaultEditor::new()?;

    // Try to load history.
    let history_path = match std::env::var("HOME") {
        Ok(mut home) => {
            home.push_str("/.anyblox_history");
            Some(home)
        }
        Err(err) => {
            tracing::error!("{err}\nCould not read HOME env defined, cannot create history");
            None
        }
    };
    if let Some(path) = &history_path {
        if rl.load_history(path).is_err() {
            tracing::debug!("No history file.");
        }
    }

    let config = args.into_config();
    let mut engine = build_engine(config).context("error building wasmtime")?;
    tracing::info!("Engine initialized, listening for commands...");

    // Main loop.
    loop {
        let line = rl.readline("> ");
        match line {
            Ok(line) if line.chars().all(|x| x.is_ascii_whitespace()) => (),
            Ok(line) => {
                if let Err(err) = rl.add_history_entry(&line) {
                    tracing::error!("Error saving history: {err}");
                }
                match run_once(&mut engine, line).await {
                    Ok(false) => break,
                    Ok(true) => (),
                    Err(err) => {
                        eprintln!("{err}");
                        tracing::error!("{err}")
                    }
                }
            }
            Err(ReadlineError::Interrupted | ReadlineError::Eof) => break,
            Err(err) => {
                eprintln!("{err}");
                tracing::error!("{err}");
            }
        }
    }

    tracing::info!("Exiting...");

    // Try to save history
    if let Some(path) = &history_path {
        if let Err(err) = rl.save_history(path) {
            tracing::error!("Error saving history: {err}");
        }
    }

    Ok(())
}

#[tracing::instrument(skip(engine), level = "debug")]
async fn run_once(engine: &mut AnyBloxRuntime, line: String) -> Result<bool> {
    use anyblox::commands::Command;
    match Command::parse(line) {
        Ok(cmd) => match cmd {
            Command::Run(args) => {
                let anyblox_bundle = open_anyblox_bundle(args.anyblox_path(), args.dataset_path()).await?;
                tracing::info_span!("run wasm job").in_scope(|| {
                    let metadata = anyblox_bundle.metadata().clone();
                    let last_requested_tuple = args.tuple_start() + args.tuple_count();
                    if last_requested_tuple as u64 > metadata.data().count() {
                        bail!(
                            "requested more tuples than there are in the dataset ({})",
                            metadata.data().count()
                        );
                    }
                    let mut param_builder = anyblox::AnyBloxJobParametersBuilder::new();
                    let job_params = if args.columns().is_empty() {
                        param_builder.finish(&anyblox_bundle)?
                    } else {
                        let projection = anyblox::ColumnProjection::from_indices(args.columns().iter().copied())?;
                        param_builder
                            .with_column_projection(projection)
                            .finish(&anyblox_bundle)?
                    };

                    let init_start = Instant::now();
                    let job = engine.init_blocking_job(job_params)?;
                    let init_duration = init_start.elapsed();
                    let schema = job.schema();
                    let batch_size = args.batch_size().unwrap_or(DEFAULT_BATCH_SIZE);
                    let job_duration = if args.verify() {
                        let sink = anyblox::sinks::TableSink::<anyblox::sinks::ChecksumSink>::new(schema.clone());
                        run_with_sink(engine, job, batch_size, args.tuple_start(), args.tuple_count(), sink)?
                    } else {
                        let sink = anyblox::sinks::EmptySink::new(schema);
                        run_with_sink(engine, job, batch_size, args.tuple_start(), args.tuple_count(), sink)?
                    };

                    println!(
                        "total: {: >8.4}ms (init: {: >8.4}ms, job: {: >8.4}ms)",
                        (init_duration + job_duration).as_secs_f32() * 1_000.0,
                        init_duration.as_secs_f32() * 1_000.0,
                        job_duration.as_secs_f32() * 1_000.0
                    );

                    fn run_with_sink<S: anyblox::sinks::Sink>(
                        engine: &mut AnyBloxRuntime,
                        mut job: anyblox::AnyBloxJob,
                        batch_size: usize,
                        tuple_start: usize,
                        tuple_count: usize,
                        mut sink: S,
                    ) -> Result<Duration> {
                        let job_start = Instant::now();

                        let mut rem_tuples = tuple_count;
                        let mut tuple_start = tuple_start;
                        while rem_tuples > 0 {
                            let request = std::cmp::min(batch_size, rem_tuples);
                            let res = engine.run_blocking_job(&mut job, tuple_start, request)?;
                            debug!("batch received: {res:?}");
                            tuple_start += res.row_count();
                            rem_tuples -= res.row_count();
                            info!("rem: {rem_tuples}");
                            sink.consume_batch(res)?;
                        }
                        let job_elapsed = job_start.elapsed();
                        let check = sink.into_result();

                        println!("{check}");
                        Ok(job_elapsed)
                    }
                    Ok(())
                })?;
            }
            Command::Native(args) => {
                let anyblox_bundle = open_anyblox_bundle(args.anyblox_path(), args.dataset_path()).await?;
                tracing::info_span!("run native job").in_scope(|| {
                    let init_start = Instant::now();
                    let mmap = tracing::info_span!("setup dataset mmap").in_scope(|| {
                        let _ = tracing::info_span!("setup dataset mmap").entered();
                        unsafe {
                            memmap2::MmapOptions::new()
                                .len(anyblox_bundle.dataset_len())
                                .offset(anyblox_bundle.dataset_offset() as u64)
                                .map(anyblox_bundle.dataset_fd().as_raw_fd())
                        }
                    })?;
                    let schema: Arc<arrow::datatypes::Schema> = Arc::new(anyblox_bundle.metadata().schema().into());
                    let job = if args.columns().is_empty() {
                        engine.init_native_job(anyblox_bundle.metadata().decoder().uri(), &schema, true)?
                    } else {
                        let projection = anyblox::ColumnProjection::from_indices(args.columns().iter().copied())?;
                        engine.init_native_job_with_projection(
                            anyblox_bundle.metadata().decoder().uri(),
                            &schema,
                            projection,
                            true,
                        )?
                    };
                    let init_elapsed = init_start.elapsed();
                    let batch_size = args.batch_size().unwrap_or(DEFAULT_BATCH_SIZE);
                    let job_duration = if args.verify() {
                        let sink = anyblox::sinks::TableSink::<anyblox::sinks::ChecksumSink>::new(job.schema());
                        run_with_sink(
                            engine,
                            job,
                            &mmap,
                            batch_size,
                            args.tuple_start(),
                            args.tuple_count(),
                            sink,
                        )?
                    } else {
                        let sink = anyblox::sinks::EmptySink::new(schema.clone());
                        run_with_sink(
                            engine,
                            job,
                            &mmap,
                            batch_size,
                            args.tuple_start(),
                            args.tuple_count(),
                            sink,
                        )?
                    };

                    println!(
                        "total: {: >8.4}ms (init: {: >8.4}ms, job: {: >8.4}ms)",
                        (init_elapsed + job_duration).as_secs_f32() * 1_000.0,
                        init_elapsed.as_secs_f32() * 1_000.0,
                        job_duration.as_secs_f32() * 1_000.0
                    );

                    fn run_with_sink<S: anyblox::sinks::Sink>(
                        engine: &mut AnyBloxRuntime,
                        mut job: NativeJob,
                        data: &[u8],
                        batch_size: usize,
                        tuple_start: usize,
                        tuple_count: usize,
                        mut sink: S,
                    ) -> Result<Duration> {
                        let job_start = Instant::now();

                        let mut rem_tuples = tuple_count;
                        let mut tuple_start = tuple_start;
                        while rem_tuples > 0 {
                            let request = std::cmp::min(batch_size, rem_tuples);
                            let batch = engine.run_native_job(&mut job, data, tuple_start, request)?;

                            debug!("batch received: {batch:?}");
                            tuple_start += batch.row_count();
                            rem_tuples -= batch.row_count();
                            sink.consume_batch(batch)?;
                        }
                        let job_elapsed = job_start.elapsed();
                        let check = sink.into_result();

                        println!("{check}");
                        Ok(job_elapsed)
                    }

                    Ok::<(), anyhow::Error>(())
                })?;
            }
            Command::Quit => return Ok(false),
        },
        Err(err) if err.kind() == clap::error::ErrorKind::DisplayHelp => {
            println!("{err}");
        }
        Err(err) => {
            eprintln!("{err}");
            tracing::error!("{err}");
            bail!("invalid command")
        }
    }

    Ok(true)
}

#[tracing::instrument]
async fn open_anyblox_bundle(
    anyblox_path: &Path,
    dataset_path: Option<&Path>,
) -> Result<anyblox::bundle::AnyBloxBundle> {
    let bundle_file = fs::File::open(anyblox_path).await?;
    let bundle_len = bundle_file.metadata().await?.len() as usize;
    let dataset_file = if let Some(path) = dataset_path {
        let file = fs::File::open(path).await?;
        let len = file.metadata().await?.len();
        Some((file, len as usize))
    } else {
        None
    };
    let anyblox_bundle = if let Some((dataset_file, dataset_len)) = dataset_file {
        anyblox::bundle::AnyBloxBundle::new_extension(bundle_file, bundle_len as usize, dataset_file, dataset_len)?
    } else {
        anyblox::bundle::AnyBloxBundle::new_self_contained(bundle_file, bundle_len)?
    };
    Ok(anyblox_bundle)
}

impl Args {
    fn into_config(self) -> config::Config {
        let mut config_builder = config::ConfigBuilder::new();
        config_builder.set_log_level(config::LogLevel::Trace);

        if let Some(limit) = self.wasm_cache_limit {
            config_builder.set_wasm_cache_limit(limit);
        }

        #[cfg(feature = "opentelemetry")]
        config_builder.enable_opentelemetry(true);

        config_builder.into()
    }
}
