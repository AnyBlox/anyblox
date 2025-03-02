use anyhow::Result;
use clap::{Args, Parser, Subcommand};
use anyblox::{bundle::AnyBloxBundle, AnyBloxJobParametersBuilder, AnyBloxRuntime};
use anyblox_bench::Bench;
use std::{
    fs,
    path::{Path, PathBuf},
    sync::{atomic::AtomicBool, Arc},
    time::Instant,
};
use std::time::Duration;

const GIB: usize = 1024 * 1024 * 1024;

#[derive(Debug, Subcommand, Clone)]
enum Command {
    /// Measure overhead of calling into wasmtime.
    CallTime(CallTimeArgs),
    /// Measure overhead of UTF-8 validation.
    Utf8Validation(Utf8ValidationArgs),
    /// Measure overhead of the dataset hook.
    HookTime(HookTimeArgs),
    /// Measure compilation time of the decoder.
    CompilationTime(CompilationArgs),
    /// Continuously run job init to gather trace for an external profiler.
    ProfileJobInit(ProfileJobInitArgs),
}

#[derive(Debug, Args, Clone)]
#[command(flatten_help = true)]
struct CallTimeArgs {
    #[clap(short = 'p', long)]
    empty_bundle_path: PathBuf,
    #[clap(short, long)]
    threads: usize,
    #[clap(short, long)]
    samples: usize,
}

#[derive(Debug, Args, Clone)]
#[command(flatten_help = true)]
struct Utf8ValidationArgs {
    #[clap(short = 'p', long)]
    bundle_path: PathBuf,
    #[clap(short, long)]
    threads: usize,
    #[clap(short, long)]
    batch_size: usize,
}

#[derive(Debug, Args, Clone)]
#[command(flatten_help = true)]
struct HookTimeArgs {
    #[clap(short = 'p', long)]
    bundle_path: PathBuf,
    #[clap(short, long)]
    threads: usize,
    #[clap(short, long)]
    samples: usize,
}

#[derive(Debug, Args, Clone)]
#[command(flatten_help = true)]
struct ProfileJobInitArgs {
    #[clap(short = 'p', long)]
    bundle_path: PathBuf,
    #[clap(short, long)]
    threads: usize,
    #[clap(short, long)]
    seconds: usize,
}

#[derive(Debug, Args, Clone)]
#[command(flatten_help = true)]
struct CompilationArgs {
    bundle_path: PathBuf,
}

#[derive(Debug, Parser, Clone)]
pub struct Cli {
    #[command(subcommand)]
    command: Command,
}

fn main() -> Result<()> {
    let args = Cli::parse();

    match args.command {
        Command::CallTime(call_time_args) => run_call_time(call_time_args)?,
        Command::Utf8Validation(utf8_validation_args) => run_utf8_validation(utf8_validation_args)?,
        Command::HookTime(hook_time_args) => run_hook_time(hook_time_args)?,
        Command::CompilationTime(compilation_args) => run_compilation_time(compilation_args)?,
        Command::ProfileJobInit(profile_job_init_args) => run_profile_job_init(profile_job_init_args)?,
    };

    Ok(())
}

fn run_call_time(args: CallTimeArgs) -> Result<()> {
    let engine = Arc::new(build_engine(16 * GIB)?);
    let bundle = open_anyblox_bundle(&args.empty_bundle_path, None)?;
    let sample_size = args.samples;

    let setup_f = |_: usize| {
        let mut job_builder = anyblox::AnyBloxJobParametersBuilder::new();
        let job_params = job_builder.finish(&bundle).expect("incorrect bundle");
        
        engine.init_blocking_job(job_params).expect("init_blocking_job")
    };
    let run_f = |mut job: anyblox::AnyBloxJob| {
        for _ in 0..sample_size {
            let res = engine.run_blocking_job(&mut job, 0, 1).expect("run_blocking_job");
            black_box(res);
        }
        job.peak_memory_usage()
    };

    let bench = Bench::par_run("wasm_ablation_calltime".to_string(), setup_f, run_f, args.threads, true);
    bench.save_to_file(format!(
        "./exp/wasm_ablation_calltime_t{}_s{}.csv",
        args.threads, args.samples
    ))?;
    println!("{}", bench.with_context(1, 1));

    Ok(())
}

fn run_utf8_validation(args: Utf8ValidationArgs) -> Result<()> {
    let engine = Arc::new(build_engine(16 * GIB)?);
    let bundle = open_anyblox_bundle(&args.bundle_path, None)?;
    let tuple_count = bundle.metadata().data().count() as usize;

    let setup_f = |validate_utf8: bool| {
        let engine = engine.clone();
        let bundle = &bundle;
        move |thread_id| {
            let one_thread_chunk_size = tuple_count / args.threads;
            let start_tuple = thread_id * one_thread_chunk_size;
            let tuple_count = if thread_id == args.threads - 1 {
                one_thread_chunk_size + tuple_count % args.threads
            } else {
                one_thread_chunk_size
            };
            let mut job_builder = anyblox::AnyBloxJobParametersBuilder::new();
            if !validate_utf8 {
                job_builder.do_not_validate_utf8();
            }
            let job_params = job_builder.finish(bundle).expect("incorrect bundle");
            let job = engine.init_blocking_job(job_params).expect("init_blocking_job");
            (job, start_tuple, tuple_count)
        }
    };
    let run_f = |mut job: (anyblox::AnyBloxJob, usize, usize)| {
        let mut rem_tuples = job.2;
        let mut tuple_start = job.1;
        while rem_tuples > 0 {
            let request = std::cmp::min(rem_tuples, args.batch_size);
            let res = engine
                .run_blocking_job(&mut job.0, tuple_start, request)
                .expect("run_blocking_job");
            tuple_start += res.row_count();
            rem_tuples -= res.row_count();
            black_box(res);
        }
        job.0.peak_memory_usage()
    };

    let bench_enabled = Bench::par_run(
        "wasm_ablation_utf8_enabled".to_string(),
        setup_f(true),
        run_f,
        args.threads,
        true,
    );
    let bench_disabled = Bench::par_run(
        "wasm_ablation_utf8_disabled".to_string(),
        setup_f(false),
        run_f,
        args.threads,
        true,
    );
    bench_enabled.save_to_file(format!(
        "./exp/wasm_ablation_utf8_t{}_b{}_enabled.csv",
        args.threads, args.batch_size
    ))?;
    bench_disabled.save_to_file(format!(
        "./exp/wasm_ablation_utf8_t{}_b{}_disabled.csv",
        args.threads, args.batch_size
    ))?;

    Ok(())
}

fn run_compilation_time(args: CompilationArgs) -> Result<()> {
    let engine = default_wasmtime_engine()?;
    let bundle = open_anyblox_bundle(&args.bundle_path, None)?;
    let decoder = bundle.decoder();

    let setup_f = |_| {};
    let run_f = |_| {
        let module = wasmtime::Module::new(&engine, decoder).unwrap();
        let len = module.text().len();
        black_box(module);
        len
    };

    let bench = Bench::par_run("wasm_ablation_compilation".to_string(), setup_f, run_f, 1, true);
    bench.save_to_file(format!(
        "./exp/wasm_ablation_hook_{}.csv",
        bundle.metadata().decoder().uri()
    ))?;

    Ok(())
}

fn run_hook_time(args: HookTimeArgs) -> Result<()> {
    let engine = Arc::new(build_engine(8 * GIB)?);

    // Open the bundle once and init a job to cache the compiled decoder.
    let name = {
        let bundle = open_anyblox_bundle(&args.bundle_path, None).unwrap();
        let mut job_builder = anyblox::AnyBloxJobParametersBuilder::new();
        let job_params = job_builder.finish(&bundle).expect("incorrect bundle");
        let _ = engine.init_blocking_job(job_params).expect("init_blocking_job");
        bundle.metadata().decoder().uri().to_string()
    };

    // Bench fresh initialization for a thread.
    let mut cold_runs = vec![];
    for i in 0..(args.samples + 10) {
        let barrier = Arc::new(std::sync::Barrier::new(args.threads));

        let handles = (0..args.threads)
            .map(|_| {
                let engine = engine.clone();
                let args = args.clone();
                let barrier = barrier.clone();
                std::thread::spawn(move || {
                    let bundle = open_anyblox_bundle(&args.bundle_path, None).unwrap();
                    let params = { AnyBloxJobParametersBuilder::new().finish(&bundle).unwrap() };
                    barrier.wait();
                    let start = Instant::now();
                    let job = engine.init_blocking_job(params).unwrap();
                    let elapsed = start.elapsed();
                    black_box(job);
                    elapsed.as_nanos()
                })
            })
            .collect::<Vec<_>>();
        let res = handles.into_iter().map(|h| h.join().unwrap()).max().unwrap();
        if i >= 10 {
            cold_runs.push(res);
        }
    }
    use std::io::Write;
    let path: PathBuf = format!(
        "./exp/wasm_ablation_thread_init_{name}_t{}_s{}.csv",
        args.threads, args.samples
    )
    .into();
    if let Some(dir_path) = path.parent() {
        std::fs::create_dir_all(dir_path)?
    }
    let mut file = std::fs::File::create(&path)?;

    writeln!(file, "cold_ns")?;
    for c in cold_runs {
        writeln!(file, "{c}")?;
    }

    println!("Saved bench results to {}", path.display());

    // Each thread has one memory. To force a reset we alternate between two identical copies of the bundle
    // every time. We first need to copy the file temporarily to avoid sharing the same fd for both.
    let file_copy_path = PathBuf::from("/tmp/anyblox-ablation-hook-tmpfile");
    fs::copy(&args.bundle_path, &file_copy_path)?;
    struct ThreadState {
        first: AtomicBool,
        first_bundle: AnyBloxBundle,
        second_bundle: AnyBloxBundle,
    }
    let mut thread_states = vec![];
    for _ in 0..args.threads {
        thread_states.push(ThreadState {
            first: AtomicBool::new(true),
            first_bundle: open_anyblox_bundle(&args.bundle_path, None)?,
            second_bundle: open_anyblox_bundle(&file_copy_path, None)?,
        });
    }

    // Bench job init for a new bundle.
    let setup_f = |thread_id| {
        let state: &ThreadState = &thread_states[thread_id];
        if state.first.fetch_not(std::sync::atomic::Ordering::Relaxed) {
            &state.first_bundle
        } else {
            &state.second_bundle
        }
    };
    let run_f = |bundle| {
        let params = { AnyBloxJobParametersBuilder::new().finish(bundle).unwrap() };
        let job = engine.init_blocking_job(params).expect("init_blocking_job");
        black_box(job);
        0
    };
    let bench = Bench::par_run("wasm_ablation_init".to_string(), setup_f, run_f, args.threads, true);
    bench.save_to_file(format!(
        "./exp/wasm_ablation_bundle_init_{}_t{}.csv",
        name, args.threads
    ))?;
    fs::remove_file(file_copy_path)?;

    // Bench job init for an existing bundle.
    let bundle = open_anyblox_bundle(&args.bundle_path, None).unwrap();
    let setup_f = |_| {
        let params = { AnyBloxJobParametersBuilder::new().finish(&bundle).unwrap() };
        params
    };
    let run_f = |params| {
        let job = engine.init_blocking_job(params).expect("init_blocking_job");
        black_box(job);
        0
    };
    let bench = Bench::par_run("wasm_ablation_init".to_string(), setup_f, run_f, args.threads, true);
    bench.save_to_file(format!("./exp/wasm_ablation_job_init_{}_t{}.csv", name, args.threads))?;

    Ok(())
}

fn run_profile_job_init(args: ProfileJobInitArgs) -> Result<()> {
    let engine = Arc::new(build_engine(8 * GIB)?);

    let bundle = open_anyblox_bundle(&args.bundle_path, None).unwrap();
    let mut job_builder = anyblox::AnyBloxJobParametersBuilder::new();
    let job_params = job_builder.finish(&bundle).expect("incorrect bundle");
    let _ = engine.init_blocking_job(job_params).expect("init_blocking_job");

    // Bench job init for an existing bundle.
    let bundle = open_anyblox_bundle(&args.bundle_path, None).unwrap();
    let start = Instant::now();

    while start.elapsed() < Duration::from_secs(args.seconds as u64) {
        for _ in 0..1000 {
            let params = { AnyBloxJobParametersBuilder::new().finish(&bundle).unwrap() };
            let job = engine.init_blocking_job(params).expect("init_blocking_job");
            black_box(job);
        }
    }

    Ok(())
}

fn default_wasmtime_engine() -> Result<wasmtime::Engine> {
    use wasmtime::{Config, OptLevel};

    let mut config = Config::new();
    config
        .cranelift_opt_level(OptLevel::SpeedAndSize)
        .allocation_strategy(wasmtime::InstanceAllocationStrategy::OnDemand)
        .async_support(false);
    wasmtime::Engine::new(&config)
}

fn build_engine(virtual_memory_limit: usize) -> Result<AnyBloxRuntime> {
    let mut config_builder = anyblox::config::ConfigBuilder::new();
    config_builder.set_wasm_cache_limit(64 * 1024 * 1024 * 1024);
    config_builder.set_thread_virtual_memory_limit(virtual_memory_limit);
    config_builder.enable_opentelemetry(false);

    let config = config_builder.into();
    let runtime = anyblox::build_engine(config)?;
    Ok(runtime)
}

fn open_anyblox_bundle(anyblox_path: &Path, dataset_path: Option<&Path>) -> Result<AnyBloxBundle> {
    let bundle_file = fs::File::open(anyblox_path)?;
    drop(bundle_file);
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
