use arrow::ffi::{FFI_ArrowArray, FFI_ArrowSchema};
use anyblox::{
    bundle::AnyBloxBundle, ColumnProjection, AnyBloxJob, AnyBloxJobParametersBuilder,
};
use jni::{
    errors::{Exception, ToException},
    objects::{JObject, JString},
    JNIEnv,
};
use jni::{
    objects::JClass,
    sys::{jboolean, jbyte, jchar, jdouble, jfloat, jint, jlong, jlongArray, jobject, jobjectArray, jshort, jstring},
};
use regex::Regex;
use std::{
    any::Any,
    convert,
    fmt::Write,
    panic::{catch_unwind, UnwindSafe},
    rc::Rc,
    sync::{Arc, LazyLock, Mutex},
};

static PANIC_BACKTRACE: LazyLock<Arc<Mutex<Option<String>>>> = LazyLock::new(|| Arc::new(Mutex::new(None)));

struct JniBundleContext {
    bundle: AnyBloxBundle,
    schema: Option<Rc<FFI_ArrowSchema>>,
}

struct JniJobContext {
    job: AnyBloxJob,
    batch: Option<Rc<FFI_ArrowArray>>,
    _schema: Option<Rc<FFI_ArrowSchema>>,
}

#[no_mangle]
pub extern "system" fn Java_org_anyblox_ffi_AnyBloxNative_init(_e: JNIEnv, _jclass: JClass) {
    init_errors();
}

#[no_mangle]
pub unsafe extern "system" fn Java_org_anyblox_ffi_AnyBloxNative_createRuntime(
    e: JNIEnv,
    _jclass: JClass,
    config: jlong,
) -> jlong {
    use std::sync::atomic::{AtomicBool, Ordering};
    static RUNTIME_EXISTS: AtomicBool = AtomicBool::new(false);
    try_unwrap_or_throw(&e, |mut _env| {
        RUNTIME_EXISTS
            .compare_exchange(false, true, Ordering::Acquire, Ordering::Relaxed)
            .map_err(|_| AnyBloxJniError::MultipleRuntimes)?;

        let config = config as *mut anyblox::config::Config;
        let config = Box::from_raw(config);
        let runtime = anyblox::build_engine(*config)?;
        let res = Box::new(runtime);
        Ok(Box::into_raw(res) as i64)
    })
}

#[no_mangle]
pub extern "system" fn Java_org_anyblox_ffi_AnyBloxNative_createConfigBuilder(e: JNIEnv, _jclass: JClass) -> jlong {
    try_unwrap_or_throw(&e, |mut _env| {
        let mut builder = anyblox::config::ConfigBuilder::new();
        builder.enable_opentelemetry(false);
        let res = Box::new(builder);
        Ok(Box::into_raw(res) as i64)
    })
}

#[no_mangle]
pub extern "system" fn Java_org_anyblox_ffi_AnyBloxNative_dropConfigBuilder(
    e: JNIEnv,
    _jclass: JClass,
    builder: jlong,
) {
    try_unwrap_or_throw(&e, |_| unsafe {
        let builder = builder as *mut anyblox::config::ConfigBuilder;
        drop(Box::from_raw(builder));
        Ok(())
    })
}

#[no_mangle]
pub extern "system" fn Java_org_anyblox_ffi_AnyBloxNative_configBuilderSetWasmCacheLimit(
    e: JNIEnv,
    _jclass: JClass,
    builder: jlong,
    limit: jlong,
) {
    try_unwrap_or_throw(&e, |_| unsafe {
        let limit = usize::try_from(limit)?;
        let builder = builder as *mut anyblox::config::ConfigBuilder;
        (*builder).set_wasm_cache_limit(limit);
        Ok(())
    })
}

#[no_mangle]
pub extern "system" fn Java_org_anyblox_ffi_AnyBloxNative_configBuilderSetThreadVirtualMemoryLimit(
    e: JNIEnv,
    _jclass: JClass,
    builder: jlong,
    limit: jlong,
) {
    try_unwrap_or_throw(&e, |_| unsafe {
        let limit = usize::try_from(limit)?;
        let builder = builder as *mut anyblox::config::ConfigBuilder;
        (*builder).set_thread_virtual_memory_limit(limit);
        Ok(())
    })
}

#[no_mangle]
pub extern "system" fn Java_org_anyblox_ffi_AnyBloxNative_configBuilderSetLogLevel(
    e: JNIEnv,
    _jclass: JClass,
    builder: jlong,
    log_level: jint,
) {
    try_unwrap_or_throw(&e, |_| unsafe {
        let log_level = anyblox::config::LogLevel::try_from(log_level)?;
        let builder = builder as *mut anyblox::config::ConfigBuilder;
        (*builder).set_log_level(log_level);
        Ok(())
    })
}

#[no_mangle]
pub unsafe extern "system" fn Java_org_anyblox_ffi_AnyBloxNative_configBuilderSetLogDirectory(
    e: JNIEnv,
    _jclass: JClass,
    builder: jlong,
    path: jstring,
) {
    try_unwrap_or_throw(&e, |mut env| {
        let path: JString = JString::from_raw(path);
        let path: String = env.get_string(&path)?.into();
        let builder = builder as *mut anyblox::config::ConfigBuilder;
        (*builder).set_log_directory(&path);
        Ok(())
    })
}

#[no_mangle]
pub unsafe extern "system" fn Java_org_anyblox_ffi_AnyBloxNative_configBuilderCompileWithDebug(
    e: JNIEnv,
    _jclass: JClass,
    builder: jlong,
    value: jboolean,
) {
    try_unwrap_or_throw(&e, |_| {
        let builder = builder as *mut anyblox::config::ConfigBuilder;
        (*builder).compile_with_debug(value != 0);
        Ok(())
    })
}

#[no_mangle]
pub unsafe extern "system" fn Java_org_anyblox_ffi_AnyBloxNative_configBuilderFinish(
    e: JNIEnv,
    _jclass: JClass,
    builder: jlong,
) -> jlong {
    try_unwrap_or_throw(&e, |_| {
        let builder = builder as *mut anyblox::config::ConfigBuilder;
        let config = Box::from_raw(builder).into_config();
        let res = Box::new(config);
        Ok(Box::into_raw(res) as i64)
    })
}

#[no_mangle]
pub unsafe extern "system" fn Java_org_anyblox_ffi_AnyBloxNative_dropConfig(
    e: JNIEnv,
    _jclass: JClass,
    config: jlong,
) {
    try_unwrap_or_throw(&e, |_| unsafe {
        let config = config as *mut anyblox::config::Config;
        drop(Box::from_raw(config));
        Ok(())
    })
}

#[no_mangle]
pub unsafe extern "system" fn Java_org_anyblox_ffi_AnyBloxNative_dropRuntime(
    e: JNIEnv,
    _jclass: JClass,
    runtime: jlong,
) {
    try_unwrap_or_throw(&e, |_| unsafe {
        let runtime = runtime as *mut anyblox::AnyBloxRuntime;
        drop(Box::from_raw(runtime));
        Ok(())
    })
}

#[no_mangle]
pub unsafe extern "system" fn Java_org_anyblox_ffi_AnyBloxNative_runtimeDecodeInit(
    e: JNIEnv,
    _jclass: JClass,
    runtime: jlong,
    bundle: jlong,
    validate_utf8: jboolean,
) -> jlongArray {
    try_unwrap_or_throw(&e, |env| unsafe {
        let bundle_obj = bundle as *mut anyblox::bundle::AnyBloxBundle;
        let schema = (*bundle_obj).metadata().schema();
        let projection = ColumnProjection::all(schema.fields().len())?;
        Ok(Java_org_anyblox_ffi_AnyBloxNative_runtimeDecodeInitWithProjection(
            env,
            _jclass,
            runtime,
            bundle,
            projection.raw_mask() as i64,
            validate_utf8,
        ))
    })
}

#[no_mangle]
pub unsafe extern "system" fn Java_org_anyblox_ffi_AnyBloxNative_runtimeDecodeInitWithProjection(
    e: JNIEnv,
    _jclass: JClass,
    runtime: jlong,
    bundle: jlong,
    projection: jlong,
    validate_utf8: jboolean,
) -> jlongArray {
    try_unwrap_or_throw(&e, |env| unsafe {
        let runtime = runtime as *mut anyblox::AnyBloxRuntime;
        let bundle = bundle as *mut anyblox::bundle::AnyBloxBundle;
        let projection = ColumnProjection::from_raw_mask(projection as u64);

        let mut builder = AnyBloxJobParametersBuilder::new();
        if validate_utf8 == 0 {
            builder.do_not_validate_utf8();
        }
        let params = builder.with_column_projection(projection).finish(&*bundle)?;
        let job = (*runtime).init_blocking_job(params)?;

        let schema = job.schema();
        let ffi_schema = Rc::new(FFI_ArrowSchema::try_from(schema.as_ref())?);
        let ctx = JniJobContext {
            job,
            batch: None,
            _schema: Some(ffi_schema.clone()),
        };

        let res = Box::new(ctx);
        let ctx_ptr = Box::into_raw(res) as i64;
        let schema_ptr = Rc::into_raw(ffi_schema) as i64;

        let long_array = env.new_long_array(2)?;
        env.set_long_array_region(&long_array, 0, &[ctx_ptr, schema_ptr])?;
        Ok(long_array.into_raw())
    })
}

#[no_mangle]
pub unsafe extern "system" fn Java_org_anyblox_ffi_AnyBloxNative_jobRunAndBlock(
    e: JNIEnv,
    _jclass: JClass,
    runtime: jlong,
    job: jlong,
    first_tuple: jlong,
    tuple_count: jlong,
) -> jlong {
    try_unwrap_or_throw(&e, |_| unsafe {
        let runtime = runtime as *mut anyblox::AnyBloxRuntime;
        let job_ctx = job as *mut JniJobContext;

        (*job_ctx).batch = None;
        let job = &mut (*job_ctx).job;

        let batch = (*runtime).run_blocking_job(job, first_tuple as usize, tuple_count as usize)?;
        let ffi_batch: FFI_ArrowArray = std::mem::transmute(batch);
        let ffi_batch = Rc::new(ffi_batch);
        (*job_ctx).batch = Some(ffi_batch.clone());

        let ptr = Rc::into_raw(ffi_batch);
        Ok(ptr as i64)
    })
}

#[no_mangle]
pub unsafe extern "system" fn Java_org_anyblox_ffi_AnyBloxNative_dropJob(e: JNIEnv, _jclass: JClass, job: jlong) {
    try_unwrap_or_throw(&e, |_| unsafe {
        let job = job as *mut JniJobContext;
        drop(Box::from_raw(job));
        Ok(())
    })
}

#[no_mangle]
pub unsafe extern "system" fn Java_org_anyblox_ffi_AnyBloxNative_bundleOpenSelfContained(
    e: JNIEnv,
    _jclass: JClass,
    fd: jint,
    len: jlong,
) -> jlong {
    try_unwrap_or_throw(&e, |_| {
        let fd = std::os::unix::prelude::BorrowedFd::borrow_raw(fd);
        let bundle = anyblox::bundle::AnyBloxBundle::new_self_contained(fd, len as usize)?;
        let bundle_ctx = JniBundleContext { bundle, schema: None };
        let res = Box::new(bundle_ctx);
        Ok(Box::into_raw(res) as i64)
    })
}

#[no_mangle]
pub unsafe extern "system" fn Java_org_anyblox_ffi_AnyBloxNative_bundleOpenExtensionAndData(
    e: JNIEnv,
    _jclass: JClass,
    anyblox_fd: jint,
    anyblox_len: jlong,
    data_fd: jint,
    data_len: jlong,
) -> jlong {
    try_unwrap_or_throw(&e, |_| {
        let anyblox_fd = std::os::unix::prelude::BorrowedFd::borrow_raw(anyblox_fd);
        let data_fd = std::os::unix::prelude::BorrowedFd::borrow_raw(data_fd);
        let bundle = anyblox::bundle::AnyBloxBundle::new_extension(
            anyblox_fd,
            anyblox_len as usize,
            data_fd,
            data_len as usize,
        )?;
        let bundle_ctx = JniBundleContext { bundle, schema: None };
        let res = Box::new(bundle_ctx);
        Ok(Box::into_raw(res) as i64)
    })
}

#[no_mangle]
pub unsafe extern "system" fn Java_org_anyblox_ffi_AnyBloxNative_bundleDecoder(
    e: JNIEnv,
    _jclass: JClass,
    bundle: jlong,
) -> jlongArray {
    try_unwrap_or_throw(&e, |env| unsafe {
        let ctx = bundle as *mut JniBundleContext;
        let bundle = &(*ctx).bundle;
        let decoder = bundle.decoder();
        let decoder_ptr = decoder.as_ptr() as usize as i64;
        let decoder_len = decoder.len() as i64;
        let long_array = env.new_long_array(2)?;
        env.set_long_array_region(&long_array, 0, &[decoder_ptr, decoder_len])?;
        Ok(long_array.into_raw())
    })
}

#[no_mangle]
pub unsafe extern "system" fn Java_org_anyblox_ffi_AnyBloxNative_bundleMetadataData(
    e: JNIEnv,
    _jclass: JClass,
    bundle: jlong,
) -> jobjectArray {
    try_unwrap_or_throw(&e, |mut env| unsafe {
        let ctx = bundle as *mut JniBundleContext;
        let bundle = &(*ctx).bundle;
        let data = bundle.metadata().data();
        let obj_class = env.find_class("java/lang/Object")?;
        let long_class = env.find_class("java/lang/Long")?;
        let obj_array = env.new_object_array(4, obj_class, JObject::null())?;

        let name_obj = env.new_string(data.name())?;
        let count_obj = env.new_object(&long_class, "(J)V", &[(data.count() as i64).into()])?;
        let description_obj = match data.description() {
            Some(d) => env.new_string(d)?,
            None => JString::default(),
        };
        let size_in_bytes_obj = match data.size_in_bytes() {
            Some(s) => env.new_object(long_class, "(J)V", &[(s as i64).into()])?,
            None => JObject::default(),
        };

        env.set_object_array_element(&obj_array, 0, name_obj)?;
        env.set_object_array_element(&obj_array, 1, count_obj)?;
        env.set_object_array_element(&obj_array, 2, description_obj)?;
        env.set_object_array_element(&obj_array, 3, size_in_bytes_obj)?;

        Ok(obj_array.into_raw())
    })
}

#[no_mangle]
pub unsafe extern "system" fn Java_org_anyblox_ffi_AnyBloxNative_bundleMetadataDecoder(
    e: JNIEnv,
    _jclass: JClass,
    bundle: jlong,
) -> jobjectArray {
    try_unwrap_or_throw(&e, |mut env| unsafe {
        let ctx = bundle as *mut JniBundleContext;
        let bundle = &(*ctx).bundle;
        let decoder = bundle.metadata().decoder();
        let obj_class = env.find_class("java/lang/Object")?;
        let long_class = env.find_class("java/lang/Long")?;
        let obj_array = env.new_object_array(5, obj_class, JObject::null())?;

        let uri_obj = env.new_string(decoder.uri())?;
        let description_obj = match decoder.description() {
            Some(d) => env.new_string(d)?,
            None => JString::default(),
        };
        let license_obj = match decoder.license() {
            Some(l) => env.new_string(l)?,
            None => JString::default(),
        };
        let checksum_obj = match decoder.checksum_blake3() {
            Some(v) => env.new_string(v)?,
            None => JString::default(),
        };
        let min_batch_size_obj = match decoder.min_batch_size() {
            Some(s) => env.new_object(long_class, "(J)V", &[(s as i64).into()])?,
            None => JObject::default(),
        };

        env.set_object_array_element(&obj_array, 0, uri_obj)?;
        env.set_object_array_element(&obj_array, 1, description_obj)?;
        env.set_object_array_element(&obj_array, 2, license_obj)?;
        env.set_object_array_element(&obj_array, 3, checksum_obj)?;
        env.set_object_array_element(&obj_array, 4, min_batch_size_obj)?;

        Ok(obj_array.into_raw())
    })
}

#[no_mangle]
pub unsafe extern "system" fn Java_org_anyblox_ffi_AnyBloxNative_bundleMetadataSchema(
    e: JNIEnv,
    _jclass: JClass,
    bundle: jlong,
) -> jlong {
    try_unwrap_or_throw(&e, |_| unsafe {
        let ctx = bundle as *mut JniBundleContext;

        if let Some(schema) = &(*ctx).schema {
            let ptr = Rc::into_raw(schema.clone());
            return Ok(ptr as i64);
        }
        let schema: ::arrow::datatypes::Schema = (*ctx).bundle.metadata().schema().into();
        let ffi_schema = Rc::new(FFI_ArrowSchema::try_from(schema)?);

        (*ctx).schema = Some(ffi_schema.clone());
        let ptr = Rc::into_raw(ffi_schema);
        Ok(ptr as i64)
    })
}

#[no_mangle]
pub unsafe extern "system" fn Java_org_anyblox_ffi_AnyBloxNative_dropBundle(
    e: JNIEnv,
    _jclass: JClass,
    bundle: jlong,
) {
    try_unwrap_or_throw(&e, |_| unsafe {
        let bundle = bundle as *mut JniBundleContext;
        drop(Box::from_raw(bundle));
        Ok(())
    })
}

// It is currently undefined behavior to unwind from Rust code into foreign code, so we can wrap
// our JNI functions and turn these panics into a `RuntimeException`.
pub fn try_unwrap_or_throw<T, F>(env: &JNIEnv, f: F) -> T
where
    T: JNIDefault,
    F: FnOnce(JNIEnv) -> Result<T, AnyBloxJniError> + UnwindSafe,
{
    let mut env1 = unsafe { JNIEnv::from_raw(env.get_raw()).unwrap() };
    let env2 = unsafe { JNIEnv::from_raw(env.get_raw()).unwrap() };
    unwrap_or_throw_default(
        &mut env1,
        flatten(catch_unwind(curry(f, env2)).map_err(AnyBloxJniError::from)),
    )
}

// Unwrap the result returned from `panic::catch_unwind` when `Ok`, otherwise throw a
// `RuntimeException` back to the calling Java.  Since a return result is required, use `JNIDefault`
// to create a reasonable result.  This returned default value will be ignored due to the exception.
pub fn unwrap_or_throw_default<T: JNIDefault>(env: &mut JNIEnv, result: std::result::Result<T, AnyBloxJniError>) -> T {
    match result {
        Ok(value) => value,
        Err(err) => {
            let backtrace = match err {
                AnyBloxJniError::Panic { msg: _ } => PANIC_BACKTRACE.lock().unwrap().take(),
                _ => None,
            };
            throw_exception(env, &err, backtrace);
            T::default()
        }
    }
}

fn throw_exception(env: &mut JNIEnv, error: &AnyBloxJniError, backtrace: Option<String>) {
    // If there isn't already an exception?
    if env.exception_check().is_ok() {
        // ... then throw new exception
        {
            let exception = error.to_exception();
            match backtrace {
                Some(backtrace_string) => env.throw_new(
                    exception.class,
                    to_stacktrace_string(exception.msg, backtrace_string).unwrap(),
                ),
                _ => env.throw_new(exception.class, exception.msg),
            }
        }
        .expect("Thrown exception")
    }
}

fn flatten<T, E>(result: Result<Result<T, E>, E>) -> Result<T, E> {
    result.and_then(convert::identity)
}

// Implements "currying" from `FnOnce(T) -> R` to `FnOnce() -> R`, given
// an instance of T. Curring is not supported in Rust so we have to use this
// custom function to achieve something similar here.
fn curry<'a, T: 'a, F, R>(f: F, t: T) -> impl FnOnce() -> R + 'a
where
    F: FnOnce(T) -> R + 'a,
{
    || f(t)
}
/// Returns the "default value" for a type.  This is used for JNI code in order to facilitate
/// returning a value in cases where an exception is thrown.  This value will never be used, as the
/// JVM will note the pending exception.
///
/// Default values are often some kind of initial value, identity value, or anything else that
/// may make sense as a default.
///
/// NOTE: We can't just use [Default] since both the trait and the object are defined in other
/// crates.
/// See [Rust Compiler Error Index - E0117](https://doc.rust-lang.org/error-index.html#E0117)
pub trait JNIDefault {
    fn default() -> Self;
}

impl JNIDefault for jboolean {
    fn default() -> jboolean {
        0
    }
}

impl JNIDefault for jbyte {
    fn default() -> jbyte {
        0
    }
}

impl JNIDefault for jchar {
    fn default() -> jchar {
        0
    }
}

impl JNIDefault for jdouble {
    fn default() -> jdouble {
        0.0
    }
}

impl JNIDefault for jfloat {
    fn default() -> jfloat {
        0.0
    }
}

impl JNIDefault for jint {
    fn default() -> jint {
        0
    }
}

impl JNIDefault for jlong {
    fn default() -> jlong {
        0
    }
}

/// The "default value" for all returned objects, such as [jstring], [jlongArray], etc.
impl JNIDefault for jobject {
    fn default() -> jobject {
        std::ptr::null_mut()
    }
}

impl JNIDefault for jshort {
    fn default() -> jshort {
        0
    }
}

impl JNIDefault for () {
    fn default() {}
}

#[derive(thiserror::Error, Debug)]
pub enum AnyBloxJniError {
    #[error(transparent)]
    RuntimeError(#[from] anyblox::RuntimeError),
    #[error(transparent)]
    ColumnIndexError(#[from] anyblox::ColumnIndexError),
    #[error(transparent)]
    ArrowError(#[from] arrow::error::ArrowError),
    #[error(transparent)]
    JNIError {
        #[from]
        source: jni::errors::Error,
    },
    #[error(transparent)]
    ConversionError {
        #[from]
        source: std::num::TryFromIntError,
    },
    #[error(transparent)]
    LogLevelError {
        #[from]
        source: anyblox::config::LogLevelConversionError,
    },
    #[error("cannot create more than one runtime per process")]
    MultipleRuntimes,
    #[error("{msg}")]
    Panic { msg: String },
}

/// Converts the results from `panic::catch_unwind` (e.g. a panic) to a `AnyBloxJniError`
impl convert::From<Box<dyn Any + Send>> for AnyBloxJniError {
    fn from(e: Box<dyn Any + Send>) -> Self {
        AnyBloxJniError::Panic {
            msg: match e.downcast_ref::<&str>() {
                Some(s) => s.to_string(),
                None => match e.downcast_ref::<String>() {
                    Some(msg) => msg.to_string(),
                    None => "unknown panic".to_string(),
                },
            },
        }
    }
}

impl jni::errors::ToException for AnyBloxJniError {
    fn to_exception(&self) -> Exception {
        Exception {
            class: "org/anyblox/AnyBloxNativeException".to_string(),
            msg: self.to_string(),
        }
    }
}

fn to_stacktrace_string(msg: String, backtrace_string: String) -> Result<String, StacktraceError> {
    let mut res = String::new();
    write!(&mut res, "{}", msg).map_err(|error| StacktraceError::Message(error.to_string()))?;

    // Use multi-line mode and named capture groups to identify the following stacktrace fields:
    // - dc = declaredClass
    // - mn = methodName
    // - fn = fileName (optional)
    // - line = file line number (optional)
    // - col = file col number within the line (optional)
    let re = Regex::new(r"(?m)^\s*\d+: (?<dc>.*?)(?<mn>[^:]+)\n(\s*at\s+(?<fn>[^:]+):(?<line>\d+):(?<col>\d+)$)?")?;
    for c in re.captures_iter(backtrace_string.as_str()) {
        write!(
            &mut res,
            "\n        at {}{}({}:{})",
            c.name("dc")
                .ok_or_else(|| StacktraceError::RequiredField("declared class".to_string()))?
                .as_str(),
            c.name("mn")
                .ok_or_else(|| StacktraceError::RequiredField("method name".to_string()))?
                .as_str(),
            // There are internal calls within the backtrace that don't provide file information
            c.name("fn").map(|m| m.as_str()).unwrap_or("__internal__"),
            c.name("line")
                .map(|m| m.as_str().parse().expect("numeric line number"))
                .unwrap_or(0)
        )?;
    }

    Ok(res)
}

#[derive(Debug, thiserror::Error)]
enum StacktraceError {
    #[error("Unable to initialize message: {0}")]
    Message(String),
    #[error("Unable to initialize backtrace regex: {0}")]
    Regex(#[from] regex::Error),
    #[error("Required field missing: {0}")]
    RequiredField(String),
    #[error("Unable to format stacktrace element: {0}")]
    Element(#[from] std::fmt::Error),
}

fn init_errors() {
    std::panic::set_hook(Box::new(|_panic_info| {
        // Capture the backtrace for a panic
        *PANIC_BACKTRACE.lock().unwrap() = Some(std::backtrace::Backtrace::force_capture().to_string());
    }));
}
