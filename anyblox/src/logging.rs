use crate::{config::Config, RuntimeError};
use std::{
    fs, io,
    path::{Path, PathBuf},
};
use tracing_subscriber::registry::LookupSpan;

struct LogFiles {
    text_log_file: PathBuf,
}

impl LogFiles {
    pub fn open(directory: &Path) -> Result<Self, RuntimeError> {
        use std::os::unix::fs::symlink;
        let new_file_stem = Self::get_file_stem();
        let mut text_log_file: PathBuf = directory.to_path_buf();
        text_log_file.push(&new_file_stem);
        text_log_file.set_extension("log");

        let mut symlink_file: PathBuf = directory.to_path_buf();
        symlink_file.push("anyblox-log-latest");
        symlink_file.set_extension("log");

        fs::create_dir_all(directory)?;
        match fs::remove_file(&symlink_file) {
            Ok(_) => (),
            Err(err) if err.kind() == io::ErrorKind::NotFound => (),
            Err(err) => return Err(err.into()),
        };
        symlink(&text_log_file, &symlink_file)?;
        Ok(Self {
            text_log_file: text_log_file.file_name().unwrap().into(),
        })
    }

    pub fn get_file_stem() -> String {
        let now = chrono::Utc::now();
        format!("anyblox-log-{}", now.format("%Y-%m-%d-%H-%M-%S-%6f"))
    }
}

pub fn default_log_directory() -> PathBuf {
    let mut home: PathBuf = std::env::var("HOME").expect("HOME env variable must be defined").into();
    home.push("./.anyblox");
    home.push("log");
    home
}

pub fn default_oltp_endpoint() -> &'static str {
    "http://localhost:4317"
}

pub fn init_tracing(config: &Config) -> Result<Tracing, RuntimeError> {
    use tracing::level_filters::LevelFilter;
    use tracing_subscriber::{
        fmt::{self, format::FmtSpan},
        prelude::*,
        EnvFilter,
    };

    let log_directory = config
        .log_directory()
        .map(|x| x.to_path_buf())
        .unwrap_or_else(default_log_directory);
    let log_files = LogFiles::open(&log_directory)?;

    let (file_log, guard) = tracing_appender::non_blocking(tracing_appender::rolling::never(
        &log_directory,
        log_files.text_log_file,
    ));

    // Set the default level to DEBUG for debug builds, INFO for release.
    let env_filter = EnvFilter::builder()
        .with_default_directive(LevelFilter::from_level(config.log_level()).into())
        .from_env_lossy();
    // By default log compact and note all spans that are created.
    let fmt_subscriber = fmt::layer()
        .compact()
        .with_span_events(
            #[cfg(debug_assertions)]
            FmtSpan::FULL,
            #[cfg(not(debug_assertions))]
            FmtSpan::NONE,
        )
        .with_thread_ids(true)
        .with_writer(file_log.clone());
    // For errors use the pretty formatter including source locations.
    let pretty_fmt_subscriber = fmt::layer()
        .pretty()
        .with_writer(file_log)
        .with_filter(LevelFilter::ERROR);

    let base_layers = tracing_subscriber::registry()
        .with(env_filter)
        .with(fmt_subscriber)
        .with(pretty_fmt_subscriber);

    #[cfg(feature = "opentelemetry")]
    {
        let result = if config.enable_opentelemetry() {
            let oltp_endpoint = config.oltp_collector_endpoint().unwrap_or(default_oltp_endpoint());
            let opentelemetry_subscriber = configure_opentelemetry(oltp_endpoint)?;
            base_layers.with(opentelemetry_subscriber).try_init()
        } else {
            base_layers.try_init()
        };

        if let Err(err) = result {
            tracing::warn!("error initializing tracing, subscriber already exists: {err}");
        }
    }
    #[cfg(not(feature = "opentelemetry"))]
    {
        let result = base_layers.try_init();
        if let Err(err) = result {
            tracing::warn!("error initializing tracing, subscriber already exists: {err}");
        }
    }

    Ok(Tracing { _guard: guard })
}

#[cfg(feature = "opentelemetry")]
fn configure_opentelemetry<S: tracing::Subscriber + for<'span> LookupSpan<'span>>(
    oltp_endpoint: &str,
) -> Result<
    tracing_opentelemetry::OpenTelemetryLayer<S, opentelemetry_sdk::trace::Tracer>,
    opentelemetry::trace::TraceError,
> {
    use opentelemetry::{trace::TracerProvider, KeyValue};
    use opentelemetry_otlp::WithExportConfig;
    use opentelemetry_sdk::{trace, Resource};
    let resource = Resource::new(vec![KeyValue::new("service.name", "anyblox")]);
    let trace_config = trace::Config::default().with_resource(resource);

    let exporter = opentelemetry_otlp::new_exporter().tonic().with_endpoint(oltp_endpoint);
    let provider = opentelemetry_otlp::new_pipeline()
        .tracing()
        .with_exporter(exporter)
        .with_trace_config(trace_config)
        .install_batch(opentelemetry_sdk::runtime::Tokio)?;

    let tracer = provider.tracer("anyblox");
    let tracing = tracing_opentelemetry::layer().with_tracer(tracer);

    Ok(tracing)
}

pub struct Tracing {
    _guard: tracing_appender::non_blocking::WorkerGuard,
}

#[cfg(feature = "opentelemetry")]
impl Drop for Tracing {
    fn drop(&mut self) {
        opentelemetry::global::shutdown_tracer_provider();
    }
}
