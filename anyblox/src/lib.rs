#![feature(avx512_target_feature)]
#![feature(stdarch_x86_avx512)]
#![allow(clippy::enum_variant_names)]
mod arrow;
pub mod builtins;
pub mod bundle;
mod column_projection;
pub mod commands;
pub mod config;
mod data_bufs;
pub mod fmt;
pub mod logging;
mod programs;
mod units;
pub mod version;
mod wasm;

pub use crate::arrow::AnyBloxRecordBatch;
use arrow::Utf8Validator;
use bundle::AnyBloxBundle;
use config::Config;
use data_bufs::Datasets;
use programs::Programs;
use std::{
    fmt::{Debug, Display},
    sync::{Arc, OnceLock},
    time::Instant,
};
use thiserror::Error;
use tracing::instrument;
use version::{AnyBloxVersionCompatibility, ANYBLOX_API_VERSION};
use wasm::mem::WasmMemoryManager;

pub use crate::programs::sinks;
pub use column_projection::{ColumnIndexError, ColumnProjection};
use static_assertions::assert_not_impl_any;

pub struct AnyBloxRuntime {
    _config: Config,
    engine: wasmtime::Engine,
    datasets: Datasets,
    programs: Programs,
    wasm_memory_manager: Arc<WasmMemoryManager>,
    _tracing_guard: logging::Tracing,
}

pub struct DecompressionResult {
    hash: u32,
    times_read: usize,
    bytes_read: usize,
    times_written: usize,
    bytes_written: usize,
}

pub struct TimedResult {
    elapsed: std::time::Duration,
    details: DecompressionResult,
}

impl AnyBloxRuntime {
    #[instrument(skip(self, job_parameters))]
    pub fn init_blocking_job(&self, job_parameters: AnyBloxJobParameters) -> Result<AnyBloxJob, RuntimeError> {
        let anyblox_bundle = job_parameters.bundle;
        let schema: ::arrow::datatypes::Schema = anyblox_bundle.metadata().schema().into();
        let schema = job_parameters.projection.project_schema(&schema)?;
        let utf8_validator = if job_parameters.validate_utf8 {
            let validator = Utf8Validator::for_schema(&schema);
            if !validator.is_empty() {
                Some(validator)
            } else {
                None
            }
        } else {
            None
        };
        let wasm_program = self
            .programs
            .load_wasm_blocking(anyblox_bundle.decoder(), anyblox_bundle.metadata().decoder().uri())?;
        let dataset = self.datasets.load_dataset(
            anyblox_bundle.dataset_fd(),
            anyblox_bundle.dataset_offset(),
            anyblox_bundle.dataset_len(),
        )?;

        let start = Instant::now();
        let _span = tracing::info_span!("prepare wasm work item").entered();
        let prepared_program = wasm_program.prepare(
            &self.engine,
            dataset,
            Arc::new(schema),
            self.wasm_memory_manager.clone(),
        )?;
        tracing::info!(
            "work item prepare time: {: >8.4}ms",
            start.elapsed().as_secs_f32() * 1_000.0
        );

        Ok(AnyBloxJob {
            prepared_program,
            projection: job_parameters.projection,
            utf8_validator,
        })
    }

    #[instrument(skip(self, anyblox_job))]
    pub fn run_blocking_job<'a>(
        &self,
        anyblox_job: &mut AnyBloxJob,
        start_tuple: usize,
        tuple_count: usize,
    ) -> Result<AnyBloxRecordBatch, RuntimeError> {
        let start = Instant::now();

        let _span = tracing::info_span!("run wasm work item").entered();
        let base_ptr = anyblox_job.base_ptr();
        let batch =
            anyblox_job
                .prepared_program
                .decode_batch(start_tuple, tuple_count, anyblox_job.projection.into())?;
        tracing::info!(
            "work item completed in {: >8.4}ms",
            start.elapsed().as_secs_f32() * 1_000.0
        );
        tracing::info!("row count: {}", batch.row_count());
        let batch = unsafe { AnyBloxRecordBatch::from_wasm(base_ptr, batch)? };
        tracing::debug!("batch: {batch:?}");

        if let Some(utf8_validator) = anyblox_job.utf8_validator.as_ref() {
            utf8_validator.validate_utf8_columns(&batch)?;
        }

        Ok(batch)
    }

    #[instrument(skip(self))]
    pub fn init_native_job(
        &self,
        algorithm_name: &str,
        schema: &::arrow::datatypes::Schema,
        validate_utf8: bool,
    ) -> Result<NativeJob, RuntimeError> {
        self.init_native_job_with_projection(
            algorithm_name,
            schema,
            ColumnProjection::all(schema.fields().len())?,
            validate_utf8,
        )
    }

    #[instrument(skip(self))]
    pub fn init_native_job_with_projection(
        &self,
        algorithm_name: &str,
        schema: &::arrow::datatypes::Schema,
        projection: ColumnProjection,
        validate_utf8: bool,
    ) -> Result<NativeJob, RuntimeError> {
        let program = self.programs.get_native(algorithm_name)?;
        let schema = projection.project_schema(schema)?;
        let utf8_validator = if validate_utf8 {
            let validator = Utf8Validator::for_schema(&schema);
            if !validator.is_empty() {
                Some(validator)
            } else {
                None
            }
        } else {
            None
        };
        Ok(NativeJob {
            program,
            projection,
            schema: Arc::new(schema),
            utf8_validator,
        })
    }

    #[instrument(skip(self, job, input))]
    pub fn run_native_job(
        &self,
        job: &mut NativeJob,
        input: &[u8],
        tuple_start: usize,
        tuple_count: usize,
    ) -> Result<AnyBloxRecordBatch, RuntimeError> {
        let start = Instant::now();

        let _span = tracing::info_span!("run native work item").entered();
        let record_batch = job.program.run(input, tuple_start, tuple_count, job.projection)?;
        tracing::info!(
            "work item completed in {: >8.4}ms",
            start.elapsed().as_secs_f32() * 1_000.0
        );
        let result = unsafe { AnyBloxRecordBatch::from_native(record_batch) };

        if let Some(utf8_validator) = job.utf8_validator.as_ref() {
            utf8_validator.validate_utf8_columns(&result)?;
        }

        Ok(result)
    }
}

fn default_config() -> &'static wasmtime::Config {
    use wasmtime::{Config, OptLevel};
    static ENGINE_CONFIG: OnceLock<wasmtime::Config> = OnceLock::new();

    ENGINE_CONFIG.get_or_init(|| {
        let mut config = Config::new();
        config
            .cranelift_opt_level(OptLevel::SpeedAndSize)
            .allocation_strategy(wasmtime::InstanceAllocationStrategy::OnDemand);
        //.cranelift_pcc(true)
        //.profiler(wasmtime::ProfilingStrategy::JitDump)

        config
    })
}

pub fn build_engine(config: Config) -> Result<AnyBloxRuntime, RuntimeError> {
    let logging = { logging::init_tracing(&config) }?;
    let datasets = Datasets::new(&config)?;
    let wasm_memory_manager = Arc::new(WasmMemoryManager::new(&config));
    let mut wasm_config = default_config().clone();
    wasm_config.with_host_memory(wasm_memory_manager.memory_creator().clone());
    if config.compile_with_debug() {
        wasm_config.debug_info(true);
        wasm_config.wasm_backtrace_details(wasmtime::WasmBacktraceDetails::Enable);
        wasm_config.profiler(wasmtime::ProfilingStrategy::PerfMap);
        wasm_config.coredump_on_trap(true);
    }
    let engine = wasmtime::Engine::new(&wasm_config)?;
    let programs = Programs::new(engine.clone(), &config)?;

    #[cfg(debug_assertions)]
    tracing::warn!("This is an unoptimized build.");

    Ok(AnyBloxRuntime {
        _config: config,
        engine,
        datasets,
        programs,
        wasm_memory_manager,
        _tracing_guard: logging,
    })
}

#[derive(Debug)]
pub struct AnyBloxJobParametersBuilder {
    validate_utf8: bool,
    custom_projection: Option<ColumnProjection>,
}

#[derive(Debug)]
pub struct AnyBloxJobParameters<'a> {
    bundle: &'a AnyBloxBundle,
    validate_utf8: bool,
    projection: ColumnProjection,
}

pub struct AnyBloxJob {
    prepared_program: programs::wasm::ActiveWasmProgram,
    utf8_validator: Option<arrow::Utf8Validator>,
    projection: ColumnProjection,
}

assert_not_impl_any!(AnyBloxJob: Send, Sync);

pub struct NativeJob {
    program: programs::native::NativeProgram,
    projection: ColumnProjection,
    schema: Arc<::arrow::datatypes::Schema>,
    utf8_validator: Option<arrow::Utf8Validator>,
}

impl AnyBloxJob {
    pub fn schema(&self) -> Arc<::arrow::datatypes::Schema> {
        self.prepared_program.schema()
    }

    pub fn base_ptr(&mut self) -> *const u8 {
        self.prepared_program.base_ptr()
    }

    pub fn force_offset(&mut self, offset: u32) {
        self.prepared_program.set_forced_offset(offset);
    }

    pub fn peak_memory_usage(&self) -> usize {
        self.prepared_program.peak_memory_usage()
    }
}

impl NativeJob {
    pub fn schema(&self) -> Arc<::arrow::datatypes::Schema> {
        self.schema.clone()
    }
}

impl AnyBloxJobParametersBuilder {
    pub fn new() -> Self {
        Self {
            validate_utf8: true,
            custom_projection: None,
        }
    }

    pub fn with_column_projection(&mut self, projection: ColumnProjection) -> &mut Self {
        self.custom_projection = Some(projection);
        self
    }

    pub fn do_not_validate_utf8(&mut self) -> &mut Self {
        self.validate_utf8 = false;
        self
    }

    pub fn finish<'a>(&mut self, bundle: &'a AnyBloxBundle) -> Result<AnyBloxJobParameters<'a>, RuntimeError> {
        if !ANYBLOX_API_VERSION.can_run_bundle(bundle.metadata().version()) {
            return Err(RuntimeError::IncompatibleBundleVersion(
                ANYBLOX_API_VERSION,
                bundle.metadata().version(),
            ));
        }
        let schema = bundle.metadata().schema();
        let projection = match self.custom_projection {
            Some(p) => p,
            None => ColumnProjection::all(schema.fields().len())?,
        };
        Ok(AnyBloxJobParameters {
            bundle,
            projection,
            validate_utf8: self.validate_utf8,
        })
    }
}

impl Default for AnyBloxJobParametersBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Error)]
pub enum RuntimeError {
    #[error(transparent)]
    WasmTimeError(#[from] wasmtime::Error),
    #[error(transparent)]
    IoError(#[from] std::io::Error),
    #[error(transparent)]
    ThreadPoolError(#[from] rayon::ThreadPoolBuildError),
    #[error(transparent)]
    DataBufsError(#[from] data_bufs::Error),
    #[error(transparent)]
    ProgramsError(#[from] programs::Error),
    #[error(transparent)]
    FormatError(#[from] anyblox_format::error::Error),
    #[error(transparent)]
    ArrowError(#[from] ::arrow::error::ArrowError),
    #[cfg(feature = "opentelemetry")]
    #[error(transparent)]
    OpenTelemetryError(#[from] opentelemetry_otlp::ExporterBuildError),
    #[error("the current runtime API version {0} cannot run the bundle versioned at {1}")]
    IncompatibleBundleVersion(version::AnyBloxVersion, version::AnyBloxVersion),
    #[error("expected the AnyBlox file to be an Extension, but it is Self-Contained")]
    ExpectedExtension,
    #[error("expected the AnyBlox file to be Self-Contained, but it is an Extension")]
    ExpectedSelfContained,
    #[error(transparent)]
    ColumnProjectionError(#[from] column_projection::ColumnIndexError),
    #[error("invalid utf8 returned in column {0}")]
    Utf8ValidationError(usize),
}

impl From<rustix::io::Errno> for RuntimeError {
    fn from(value: rustix::io::Errno) -> Self {
        Self::IoError(value.into())
    }
}

struct BytesPerSecond(usize, f32);

impl TimedResult {
    fn in_thpt(&self) -> BytesPerSecond {
        BytesPerSecond(self.details.bytes_read, self.elapsed.as_secs_f32())
    }

    fn out_thpt(&self) -> BytesPerSecond {
        BytesPerSecond(self.details.bytes_written, self.elapsed.as_secs_f32())
    }
}

impl Display for BytesPerSecond {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let gb = self.0 as f32 / 1_000_000_000.0;
        let gbs = gb / self.1;
        write!(f, "{gbs: >8.4} GB/s")
    }
}

impl Display for TimedResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let ms = self.elapsed.as_secs_f32() * 1_000.0;
        let in_thpt = self.in_thpt();
        let out_thpt = self.out_thpt();

        write!(f, "{:0>8x}    ", self.details.hash)?;
        write!(
            f,
            "elapsed: {ms: >8.4} ms [Thpt in: {in_thpt}  out: {out_thpt} I/O: {}/{} Compression ratio: {:.2}]",
            self.details.times_read,
            self.details.times_written,
            self.details.bytes_written as f32 / self.details.bytes_read as f32
        )
    }
}
