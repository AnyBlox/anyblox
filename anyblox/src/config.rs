use crate::units::*;
use std::path::{Path, PathBuf};

pub struct ConfigBuilder {
    config: Config,
}

pub struct Config {
    enable_opentelemetry: bool,
    log_level: tracing::Level,
    log_directory: Option<PathBuf>,
    thread_virtual_memory_limit: usize,
    compile_with_debug: bool,
    oltp_collector_endpoint: Option<String>,
    wasm_cache_limit: usize,
}

pub enum LogLevel {
    Trace = 0,
    Debug = 1,
    Info = 2,
    Warn = 3,
    Error = 4,
}

impl TryFrom<i32> for LogLevel {
    type Error = LogLevelConversionError;

    fn try_from(value: i32) -> Result<Self, LogLevelConversionError> {
        match value {
            0 => Ok(LogLevel::Trace),
            1 => Ok(LogLevel::Debug),
            2 => Ok(LogLevel::Info),
            3 => Ok(LogLevel::Warn),
            4 => Ok(LogLevel::Error),
            _ => Err(LogLevelConversionError { val: value }),
        }
    }
}

#[derive(thiserror::Error, Debug)]
#[error("invalid LogLevel raw value: {val}")]
pub struct LogLevelConversionError {
    val: i32,
}

impl Default for Config {
    fn default() -> Self {
        #[cfg(debug_assertions)]
        let log_level = tracing::Level::DEBUG;
        #[cfg(not(debug_assertions))]
        let log_level = tracing::Level::INFO;
        #[cfg(debug_assertions)]
        let compile_with_debug = true;
        #[cfg(not(debug_assertions))]
        let compile_with_debug = false;
        #[cfg(feature = "opentelemetry")]
        let enable_opentelemetry = true;
        #[cfg(not(feature = "opentelemetry"))]
        let enable_opentelemetry = false;
        Self {
            enable_opentelemetry,
            compile_with_debug,
            log_level,
            log_directory: None,
            thread_virtual_memory_limit: 8 * GIB,
            oltp_collector_endpoint: None,
            wasm_cache_limit: 128 * MB,
        }
    }
}

impl ConfigBuilder {
    pub fn new() -> Self {
        Self {
            config: Config::default(),
        }
    }

    pub fn enable_opentelemetry(&mut self, value: bool) -> &mut Self {
        #[cfg(not(feature = "opentelemetry"))]
        {
            if value {
                panic!("enable_opentelemetry not available since the opentelemetry feature is disabled")
            }
        }

        self.config.enable_opentelemetry = value;
        self
    }

    pub fn compile_with_debug(&mut self, value: bool) -> &mut Self {
        self.config.compile_with_debug = value;
        self
    }

    pub fn set_log_level(&mut self, level: LogLevel) -> &mut Self {
        self.config.log_level = level.into();
        self
    }

    pub fn set_log_directory<P: AsRef<Path>>(&mut self, dir: P) -> &mut Self {
        self.config.log_directory = Some(dir.as_ref().to_owned());
        self
    }

    pub fn set_thread_virtual_memory_limit(&mut self, limit: usize) -> &mut Self {
        self.config.thread_virtual_memory_limit = limit;
        self
    }

    pub fn set_oltp_collector_endpoint(&mut self, str: String) -> &mut Self {
        self.config.oltp_collector_endpoint = Some(str);
        self
    }

    pub fn set_wasm_cache_limit(&mut self, limit: usize) -> &mut Self {
        self.config.wasm_cache_limit = limit;
        self
    }

    pub fn into_config(self) -> Config {
        self.config
    }
}

impl Default for ConfigBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl From<ConfigBuilder> for Config {
    fn from(value: ConfigBuilder) -> Self {
        value.into_config()
    }
}

impl Config {
    pub fn enable_opentelemetry(&self) -> bool {
        self.enable_opentelemetry
    }

    pub fn compile_with_debug(&self) -> bool {
        self.compile_with_debug
    }

    pub fn log_level(&self) -> tracing::Level {
        self.log_level
    }

    pub fn log_directory(&self) -> Option<&Path> {
        self.log_directory.as_deref()
    }

    pub fn thread_virtual_memory_limit(&self) -> usize {
        self.thread_virtual_memory_limit
    }

    pub fn oltp_collector_endpoint(&self) -> Option<&str> {
        self.oltp_collector_endpoint.as_deref()
    }

    pub fn wasm_cache_limit(&self) -> usize {
        self.wasm_cache_limit
    }
}

impl From<LogLevel> for tracing::Level {
    fn from(value: LogLevel) -> Self {
        match value {
            LogLevel::Trace => tracing::Level::TRACE,
            LogLevel::Debug => tracing::Level::DEBUG,
            LogLevel::Info => tracing::Level::INFO,
            LogLevel::Warn => tracing::Level::WARN,
            LogLevel::Error => tracing::Level::ERROR,
        }
    }
}
