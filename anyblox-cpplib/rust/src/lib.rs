use std::{ffi::OsStr, os::fd};

use anyblox::{AnyBloxRecordBatch, ColumnProjection};

#[cfg(test)]
mod tests;

pub struct AnyBloxConfig {
    inner: anyblox::config::Config,
}

pub struct AnyBloxConfigBuilder {
    inner: anyblox::config::ConfigBuilder,
}

pub struct AnyBloxRuntime {
    inner: anyblox::AnyBloxRuntime,
}

pub struct AnyBloxJobContext {
    job: anyblox::AnyBloxJob,
}

pub struct AnyBloxBundle {
    inner: anyblox::bundle::AnyBloxBundle,
}

#[repr(C)]
pub enum AnyBloxFile<'a> {
    Extension(*mut anyblox_format::model::AnyBloxExtension<'a>),
    SelfContained(*mut anyblox_format::model::AnyBloxSelfContained<'a>),
}

#[repr(C)]
pub struct AnyBloxMetadata {
    schema: *const anyblox_format::model::Schema,
    decoder_metadata: *const anyblox_format::model::DecoderMetadata,
    data_metadata: *const anyblox_format::model::DataMetadata,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SchemaDataType {
    Null,
    Boolean,
    Int8,
    Int16,
    Int32,
    Int64,
    UInt8,
    UInt16,
    UInt32,
    UInt64,
    Float16,
    Float32,
    Float64,
    Date32,
    Date64,
    Binary,
    LargeBinary,
    BinaryView,
    Utf8,
    LargeUtf8,
    Utf8View,
    FixedSizeBinary(i32),
}

#[repr(C)]
pub struct MemSlice {
    ptr: *mut u8,
    len: usize,
}

#[repr(C)]
pub enum COption<T> {
    Some(T),
    None,
}

#[repr(C)]
pub enum LogLevel {
    Trace = 0,
    Debug = 1,
    Info = 2,
    Warn = 3,
    Error = 4,
}

#[repr(C)]
pub struct AnyBloxJobInit {
    job_context: *mut AnyBloxJobContext,
}

// impl AnyBloxConfigBuilder {
#[no_mangle]
pub extern "C" fn config_builder_create() -> *mut AnyBloxConfigBuilder {
    let mut inner = anyblox::config::ConfigBuilder::default();
    inner.enable_opentelemetry(false);
    Box::leak(Box::new(AnyBloxConfigBuilder { inner }))
}

#[no_mangle]
pub unsafe extern "C" fn config_builder_drop(builder: *mut AnyBloxConfigBuilder) {
    drop(Box::from_raw(builder));
}

#[no_mangle]
pub unsafe extern "C" fn config_builder_set_wasm_cache_limit(builder: *mut AnyBloxConfigBuilder, limit: usize) {
    (*builder).inner.set_wasm_cache_limit(limit);
}

#[no_mangle]
pub unsafe extern "C" fn config_builder_set_thread_virtual_memory_limit(
    builder: *mut AnyBloxConfigBuilder,
    limit: usize,
) {
    (*builder).inner.set_thread_virtual_memory_limit(limit);
}

#[no_mangle]
pub unsafe extern "C" fn config_builder_set_log_level(builder: *mut AnyBloxConfigBuilder, log_level: LogLevel) {
    (*builder).inner.set_log_level(log_level.into());
}

#[no_mangle]
pub unsafe extern "C" fn config_builder_set_log_directory(builder: *mut AnyBloxConfigBuilder, directory: MemSlice) {
    let path: &OsStr = directory.into();
    (*builder).inner.set_log_directory(path);
}

#[no_mangle]
pub unsafe extern "C" fn config_builder_compile_with_debug(builder: *mut AnyBloxConfigBuilder, value: bool) {
    (*builder).inner.compile_with_debug(value);
}

#[no_mangle]
pub unsafe extern "C" fn config_builder_set_oltp_collector_endpoint(
    builder: *mut AnyBloxConfigBuilder,
    endpoint: MemSlice,
) {
    (*builder).inner.set_oltp_collector_endpoint(endpoint.into());
}

#[no_mangle]
pub unsafe extern "C" fn config_builder_finish(builder: *mut AnyBloxConfigBuilder) -> *mut AnyBloxConfig {
    let builder = Box::from_raw(builder);
    let config = builder.inner.into_config();
    Box::leak(Box::new(AnyBloxConfig { inner: config }))
}
// }

// impl AnyBloxConfig {
#[no_mangle]
pub unsafe extern "C" fn config_drop(config: *mut AnyBloxConfig) {
    drop(Box::from_raw(config));
}
// }

// impl AnyBloxRuntime {
#[no_mangle]
pub unsafe extern "C" fn runtime_create(config: *mut AnyBloxConfig) -> *mut AnyBloxRuntime {
    // There can only be one runtime per process, or else tracing gets insane.
    use std::sync::atomic::{AtomicBool, Ordering};
    static RUNTIME_EXISTS: AtomicBool = AtomicBool::new(false);
    match RUNTIME_EXISTS.compare_exchange(false, true, Ordering::Acquire, Ordering::Relaxed) {
        Ok(_) => {
            let config = Box::from_raw(config).inner;
            match anyblox::build_engine(config) {
                Ok(runtime) => Box::leak(Box::new(AnyBloxRuntime { inner: runtime })),
                Err(err) => panic!("{err}"),
            }
        }
        Err(_) => panic!("cannot create more than one runtime per process"),
    }
}

#[no_mangle]
pub unsafe extern "C" fn runtime_decode_init(
    runtime: *mut AnyBloxRuntime,
    input: *mut AnyBloxBundle,
    validate_utf8: bool,
) -> AnyBloxJobInit {
    return match result(runtime, input, validate_utf8) {
        Ok(init) => init,
        Err(err) => panic!("{err}"),
    };

    unsafe fn result(
        runtime: *mut AnyBloxRuntime,
        input: *mut AnyBloxBundle,
        validate_utf8: bool,
    ) -> Result<AnyBloxJobInit, anyblox::RuntimeError> {
        let bundle_ref = &(*input).inner;
        let mut builder = anyblox::AnyBloxJobParametersBuilder::new();
        if !validate_utf8 {
            builder.do_not_validate_utf8();
        }
        let params = builder.finish(bundle_ref)?;

        let job = (*runtime).inner.init_blocking_job(params)?;
        let context = Box::leak(Box::new(AnyBloxJobContext { job }));

        Ok(AnyBloxJobInit { job_context: context })
    }
}

#[no_mangle]
pub unsafe extern "C" fn runtime_decode_init_with_projection(
    runtime: *mut AnyBloxRuntime,
    input: *mut AnyBloxBundle,
    projection: u64,
    validate_utf8: bool,
) -> AnyBloxJobInit {
    return match result(runtime, input, projection, validate_utf8) {
        Ok(init) => init,
        Err(err) => panic!("{err}"),
    };

    unsafe fn result(
        runtime: *mut AnyBloxRuntime,
        input: *mut AnyBloxBundle,
        projection: u64,
        validate_utf8: bool,
    ) -> Result<AnyBloxJobInit, anyblox::RuntimeError> {
        let bundle_ref = &(*input).inner;
        let projection = ColumnProjection::from_raw_mask(projection);
        let mut builder = anyblox::AnyBloxJobParametersBuilder::new();
        if !validate_utf8 {
            builder.do_not_validate_utf8();
        }
        let params = builder.with_column_projection(projection).finish(bundle_ref)?;

        let job = (*runtime).inner.init_blocking_job(params)?;
        let context = Box::leak(Box::new(AnyBloxJobContext { job }));

        Ok(AnyBloxJobInit { job_context: context })
    }
}

#[no_mangle]
pub unsafe extern "C" fn runtime_drop(runtime: *mut AnyBloxRuntime) {
    drop(Box::from_raw(runtime));
}
// }

// impl AnyBloxJobContext {
#[no_mangle]
pub unsafe extern "C" fn job_run_and_block(
    runtime: *mut AnyBloxRuntime,
    job_context: *mut AnyBloxJobContext,
    first_tuple: usize,
    tuple_count: usize,
) -> AnyBloxRecordBatch {
    return match result(runtime, job_context, first_tuple, tuple_count) {
        Ok(x) => x,
        Err(err) => panic!("{err}"),
    };

    unsafe fn result(
        runtime: *mut AnyBloxRuntime,
        job_context: *mut AnyBloxJobContext,
        first_tuple: usize,
        tuple_count: usize,
    ) -> Result<AnyBloxRecordBatch, anyblox::RuntimeError> {
        let job_ref = &mut (*job_context).job;
        let anyblox_batch = (*runtime).inner.run_blocking_job(job_ref, first_tuple, tuple_count)?;
        Ok(anyblox_batch)
    }
}

#[no_mangle]
pub unsafe extern "C" fn job_drop(context: *mut AnyBloxJobContext) {
    drop(Box::from_raw(context));
}
// }

// impl AnyBloxBundle {
#[no_mangle]
pub unsafe extern "C" fn bundle_open_extension<'a>(
    anyblox_fd: fd::RawFd,
    anyblox_len: usize,
    dataset_fd: fd::RawFd,
    dataset_len: usize,
) -> *mut AnyBloxBundle {
    return match result(anyblox_fd, anyblox_len, dataset_fd, dataset_len) {
        Ok(x) => x,
        Err(err) => panic!("{err}"),
    };

    unsafe fn result(
        anyblox_fd: fd::RawFd,
        anyblox_len: usize,
        dataset_fd: fd::RawFd,
        dataset_len: usize,
    ) -> Result<*mut AnyBloxBundle, anyblox::RuntimeError> {
        let inner = anyblox::bundle::AnyBloxBundle::new_extension(
            fd::BorrowedFd::borrow_raw(anyblox_fd),
            anyblox_len,
            fd::BorrowedFd::borrow_raw(dataset_fd),
            dataset_len,
        )?;

        let bundle = AnyBloxBundle { inner };
        Ok(Box::leak(Box::new(bundle)))
    }
}

#[no_mangle]
pub unsafe extern "C" fn bundle_open_self_contained<'a>(fd: fd::RawFd, len: usize) -> *mut AnyBloxBundle {
    return match result(fd, len) {
        Ok(x) => x,
        Err(err) => panic!("{err}"),
    };

    unsafe fn result(fd: fd::RawFd, len: usize) -> Result<*mut AnyBloxBundle, anyblox::RuntimeError> {
        let inner = anyblox::bundle::AnyBloxBundle::new_self_contained(fd::BorrowedFd::borrow_raw(fd), len)?;

        let bundle = AnyBloxBundle { inner };
        Ok(Box::leak(Box::new(bundle)))
    }
}

#[no_mangle]
pub unsafe extern "C" fn bundle_decoder<'a>(bundle: &AnyBloxBundle) -> MemSlice {
    let decoder = bundle.inner.decoder();
    decoder.into()
}

#[no_mangle]
pub unsafe extern "C" fn bundle_metadata<'a>(bundle: &AnyBloxBundle) -> AnyBloxMetadata {
    let metadata = bundle.inner.metadata();
    let schema = metadata.schema();
    let decoder = metadata.decoder();
    let data = metadata.data();
    AnyBloxMetadata {
        schema: schema as _,
        decoder_metadata: decoder as _,
        data_metadata: data as _,
    }
}

#[no_mangle]
pub unsafe extern "C" fn bundle_drop(bundle: *mut AnyBloxBundle) {
    drop(Box::from_raw(bundle))
}
// }

// impl AnyBloxMetadata {
// Schema
#[no_mangle]
pub unsafe extern "C" fn schema_fields_count(metadata: &AnyBloxMetadata) -> usize {
    (*metadata.schema).fields().len()
}

#[no_mangle]
pub unsafe extern "C" fn schema_field_name(metadata: &AnyBloxMetadata, field_idx: usize) -> MemSlice {
    (*metadata.schema).fields()[field_idx].name().as_bytes().into()
}

#[no_mangle]
pub unsafe extern "C" fn schema_field_datatype(metadata: &AnyBloxMetadata, field_idx: usize) -> SchemaDataType {
    (*metadata.schema).fields()[field_idx].ty().into()
}

#[no_mangle]
pub unsafe extern "C" fn schema_field_nullable(metadata: &AnyBloxMetadata, field_idx: usize) -> bool {
    (*metadata.schema).fields()[field_idx].nullable()
}
// end Schema
// Decoder
#[no_mangle]
pub unsafe extern "C" fn decoder_metadata_uri(metadata: &AnyBloxMetadata) -> MemSlice {
    (*metadata.decoder_metadata).uri().as_bytes().into()
}

#[no_mangle]
pub unsafe extern "C" fn decoder_metadata_description(metadata: &AnyBloxMetadata) -> COption<MemSlice> {
    (*metadata.decoder_metadata)
        .description()
        .map(|x| x.as_bytes().into())
        .into()
}

#[no_mangle]
pub unsafe extern "C" fn decoder_metadata_license(metadata: &AnyBloxMetadata) -> COption<MemSlice> {
    (*metadata.decoder_metadata)
        .license()
        .map(|x| x.as_bytes().into())
        .into()
}

#[no_mangle]
pub unsafe extern "C" fn decoder_metadata_checksum_blake3(metadata: &AnyBloxMetadata) -> COption<MemSlice> {
    (*metadata.decoder_metadata)
        .checksum_blake3()
        .map(|x| x.as_bytes().into())
        .into()
}

#[no_mangle]
pub unsafe extern "C" fn decoder_metadata_min_batch_size(metadata: &AnyBloxMetadata) -> COption<u64> {
    (*metadata.decoder_metadata).min_batch_size().into()
}
// end Decoder
// Data
#[no_mangle]
pub unsafe extern "C" fn data_metadata_name(metadata: &AnyBloxMetadata) -> MemSlice {
    (*metadata.data_metadata).name().as_bytes().into()
}

#[no_mangle]
pub unsafe extern "C" fn data_metadata_count(metadata: &AnyBloxMetadata) -> u64 {
    (*metadata.data_metadata).count()
}

#[no_mangle]
pub unsafe extern "C" fn data_metadata_description(metadata: &AnyBloxMetadata) -> COption<MemSlice> {
    (*metadata.data_metadata)
        .description()
        .map(|x| x.as_bytes().into())
        .into()
}

#[no_mangle]
pub unsafe extern "C" fn data_metadata_size_in_bytes(metadata: &AnyBloxMetadata) -> COption<u64> {
    (*metadata.data_metadata).size_in_bytes().into()
}
// end Data
// }

impl From<&anyblox_format::model::DataType> for SchemaDataType {
    fn from(value: &anyblox_format::model::DataType) -> Self {
        use anyblox_format::model::DataType;
        match value {
            DataType::Null => SchemaDataType::Null,
            DataType::Boolean => SchemaDataType::Boolean,
            DataType::Int8 => SchemaDataType::Int8,
            DataType::Int16 => SchemaDataType::Int16,
            DataType::Int32 => SchemaDataType::Int32,
            DataType::Int64 => SchemaDataType::Int64,
            DataType::UInt8 => SchemaDataType::UInt8,
            DataType::UInt16 => SchemaDataType::UInt16,
            DataType::UInt32 => SchemaDataType::UInt32,
            DataType::UInt64 => SchemaDataType::UInt64,
            DataType::Float16 => SchemaDataType::Float16,
            DataType::Float32 => SchemaDataType::Float32,
            DataType::Float64 => SchemaDataType::Float64,
            DataType::Date32 => SchemaDataType::Date32,
            DataType::Date64 => SchemaDataType::Date64,
            DataType::Binary => SchemaDataType::Binary,
            DataType::LargeBinary => SchemaDataType::LargeBinary,
            DataType::BinaryView => SchemaDataType::BinaryView,
            DataType::Utf8 => SchemaDataType::Utf8,
            DataType::LargeUtf8 => SchemaDataType::LargeUtf8,
            DataType::Utf8View => SchemaDataType::Utf8View,
            DataType::FixedSizeBinary(s) => SchemaDataType::FixedSizeBinary(*s),
            _ => unimplemented!(),
        }
    }
}

impl<'a> From<&'a [u8]> for MemSlice {
    fn from(value: &'a [u8]) -> Self {
        Self {
            ptr: value.as_ptr().cast_mut(),
            len: value.len(),
        }
    }
}

impl From<MemSlice> for &[u8] {
    fn from(value: MemSlice) -> Self {
        unsafe { std::slice::from_raw_parts(value.ptr, value.len) }
    }
}

impl From<MemSlice> for &std::ffi::OsStr {
    fn from(value: MemSlice) -> Self {
        use std::os::unix::ffi::OsStrExt;
        OsStr::from_bytes(value.into())
    }
}

impl From<MemSlice> for String {
    fn from(value: MemSlice) -> Self {
        String::from_utf8_lossy(value.into()).into_owned()
    }
}

impl From<LogLevel> for anyblox::config::LogLevel {
    fn from(value: LogLevel) -> Self {
        match value {
            LogLevel::Trace => anyblox::config::LogLevel::Trace,
            LogLevel::Debug => anyblox::config::LogLevel::Debug,
            LogLevel::Info => anyblox::config::LogLevel::Info,
            LogLevel::Warn => anyblox::config::LogLevel::Warn,
            LogLevel::Error => anyblox::config::LogLevel::Error,
        }
    }
}

impl<T> From<Option<T>> for COption<T> {
    fn from(value: Option<T>) -> Self {
        match value {
            Some(x) => COption::Some(x),
            None => COption::None,
        }
    }
}
