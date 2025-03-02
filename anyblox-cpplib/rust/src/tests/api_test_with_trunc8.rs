use arrow::array::AsArray;

use crate as anyblox_ffi;
use std::{
    error::Error,
    io::{self, Write},
    mem,
    os::fd::{self, AsFd, AsRawFd, FromRawFd},
    path::Path,
    sync::Arc,
};
use std::{fs, io::Read, path::PathBuf};

const RES_DIR: &str = "src/tests/res";

fn load_extension(metadata_path: impl AsRef<Path>, wasm_path: impl AsRef<Path>) -> Result<MemFd, Box<dyn Error>> {
    use anyblox_format::model::AnyBloxType;
    let mut metadata_path_buf = PathBuf::from(RES_DIR);
    let mut wasm_path_buf = PathBuf::from(RES_DIR);
    metadata_path_buf.push(metadata_path);
    wasm_path_buf.push(wasm_path);

    let metadata = anyblox_format::de::deserialize_metadata_from_file(&metadata_path_buf)?;
    assert_eq!(metadata.ty(), AnyBloxType::Extension);

    let mut wasm_bytes = vec![];
    fs::File::open(&wasm_path_buf)?.read_to_end(&mut wasm_bytes)?;

    let mut memfd = MemFdBuilder::new()?;
    anyblox_format::ser::serialize_extension(&metadata, &wasm_bytes, &mut memfd)?;

    Ok(memfd.finish()?)
}

fn load_self_contained(
    metadata_path: impl AsRef<Path>,
    wasm_path: impl AsRef<Path>,
    input: &[u8],
) -> Result<MemFd, Box<dyn Error>> {
    use anyblox_format::model::AnyBloxType;
    let mut metadata_path_buf = PathBuf::from(RES_DIR);
    let mut wasm_path_buf = PathBuf::from(RES_DIR);
    metadata_path_buf.push(metadata_path);
    wasm_path_buf.push(wasm_path);

    let metadata = anyblox_format::de::deserialize_metadata_from_file(&metadata_path_buf)?;
    assert_eq!(metadata.ty(), AnyBloxType::SelfContained);

    let mut wasm_bytes = vec![];
    fs::File::open(&wasm_path_buf)?.read_to_end(&mut wasm_bytes)?;

    let mut memfd = MemFdBuilder::new()?;
    anyblox_format::ser::serialize_self_contained(&metadata, &wasm_bytes, input, &mut memfd)?;

    Ok(memfd.finish()?)
}

#[test_log::test]
fn test_extension() -> Result<(), Box<dyn Error>> {
    let anyblox_file = load_extension("trunc8-extension.toml", "trunc8.wasm")?;

    // This is how the API is expected to be used from C++, but written as a test here to ease debugging.
    // We start a runtime and a single decode job for 1024 tuples.
    const SIZE: usize = 1024;
    let input = get_test_input(SIZE)?;

    // Everything with FFI is unsafe, shocker.
    unsafe {
        let builder = anyblox_ffi::config_builder_create();
        anyblox_ffi::config_builder_set_wasm_cache_limit(builder, 2 * 1024 * 1024 * 1024);
        let config = anyblox_ffi::config_builder_finish(builder);
        let runtime = anyblox_ffi::runtime_create(config);

        let bundle = anyblox_ffi::bundle_open_extension(
            anyblox_file.as_fd().as_raw_fd(),
            anyblox_file.len,
            input.as_fd().as_raw_fd(),
            input.len,
        );
        let metadata = anyblox_ffi::bundle_metadata(&*bundle);
        assert_eq!(1, anyblox_ffi::schema_fields_count(&metadata));
        let field_name = {
            let raw_slice = anyblox_ffi::schema_field_name(&metadata, 0);
            let byte_slice = std::slice::from_raw_parts(raw_slice.ptr, raw_slice.len);
            std::str::from_utf8(byte_slice).unwrap()
        };
        assert_eq!("col1", field_name);
        assert_eq!(
            crate::SchemaDataType::Int32,
            anyblox_ffi::schema_field_datatype(&metadata, 0)
        );
        assert!(!anyblox_ffi::schema_field_nullable(&metadata, 0));
        let arrow_schema = ::arrow::datatypes::Schema::new([Arc::new(::arrow::datatypes::Field::new(
            "col1",
            arrow::datatypes::DataType::UInt32,
            false,
        ))]);

        let expected = input
            .as_slice()
            .iter()
            .map(|&x| if x == 0 { None } else { Some(x as u32) })
            .collect::<Vec<_>>();
        let expected_null_count = expected.iter().filter(|x| x.is_none()).count();

        let job_init = anyblox_ffi::runtime_decode_init(runtime, bundle, true);

        let raw_batch = anyblox_ffi::job_run_and_block(runtime, job_init.job_context, 0, SIZE);
        let batch = raw_batch.into_arrow_record_batch(Arc::new(arrow_schema))?;

        assert_eq!(1, batch.num_columns());
        assert_eq!(expected.len(), batch.num_rows());
        let null_count = batch.column(0).null_count();
        let int_array = batch.column(0).as_primitive::<::arrow::datatypes::UInt32Type>();

        let output = int_array.into_iter().collect::<Vec<_>>();

        drop(batch);
        anyblox_ffi::job_drop(job_init.job_context);
        anyblox_ffi::bundle_drop(bundle);
        anyblox_ffi::runtime_drop(runtime);

        assert_eq!(output, *expected);
        assert_eq!(null_count, expected_null_count);
    }

    Ok(())
}

#[test_log::test]
fn test_self_contained() -> Result<(), Box<dyn Error>> {
    // This is how the API is expected to be used from C++, but written as a test here to ease debugging.
    // We start a runtime and a single decode job for 1024 tuples.
    const SIZE: usize = 1024;
    let input = get_test_input(SIZE)?;
    let anyblox_file = load_self_contained("trunc8-self-contained.toml", "trunc8.wasm", input.as_slice())?;

    // Everything with FFI is unsafe, shocker.
    unsafe {
        let builder = anyblox_ffi::config_builder_create();
        anyblox_ffi::config_builder_set_wasm_cache_limit(builder, 2 * 1024 * 1024 * 1024);
        let config = anyblox_ffi::config_builder_finish(builder);
        let runtime = anyblox_ffi::runtime_create(config);

        let bundle = anyblox_ffi::bundle_open_self_contained(anyblox_file.as_fd().as_raw_fd(), anyblox_file.len);
        let metadata = anyblox_ffi::bundle_metadata(&*bundle);
        assert_eq!(1, anyblox_ffi::schema_fields_count(&metadata));
        let field_name = {
            let raw_slice = anyblox_ffi::schema_field_name(&metadata, 0);
            let byte_slice = std::slice::from_raw_parts(raw_slice.ptr, raw_slice.len);
            std::str::from_utf8(byte_slice).unwrap()
        };
        assert_eq!("col1", field_name);
        assert_eq!(
            crate::SchemaDataType::Int32,
            anyblox_ffi::schema_field_datatype(&metadata, 0)
        );
        assert!(!anyblox_ffi::schema_field_nullable(&metadata, 0));
        let arrow_schema = ::arrow::datatypes::Schema::new([Arc::new(::arrow::datatypes::Field::new(
            "col1",
            arrow::datatypes::DataType::UInt32,
            false,
        ))]);

        let expected = input
            .as_slice()
            .iter()
            .map(|&x| if x == 0 { None } else { Some(x as u32) })
            .collect::<Vec<_>>();
        let expected_null_count = expected.iter().filter(|x| x.is_none()).count();

        let job_init = anyblox_ffi::runtime_decode_init(runtime, bundle, true);

        let raw_batch = anyblox_ffi::job_run_and_block(runtime, job_init.job_context, 0, SIZE);
        let batch = raw_batch.into_arrow_record_batch(Arc::new(arrow_schema))?;

        assert_eq!(1, batch.num_columns());
        assert_eq!(expected.len(), batch.num_rows());
        let null_count = batch.column(0).null_count();
        let int_array = batch.column(0).as_primitive::<::arrow::datatypes::UInt32Type>();

        let output = int_array.into_iter().collect::<Vec<_>>();

        drop(batch);
        anyblox_ffi::job_drop(job_init.job_context);
        anyblox_ffi::bundle_drop(bundle);
        anyblox_ffi::runtime_drop(runtime);

        assert_eq!(output, *expected);
        assert_eq!(null_count, expected_null_count);
    }

    Ok(())
}

struct MemFdBuilder {
    fd: fd::OwnedFd,
    file: fs::File,
    len: usize,
}

struct MemFd {
    fd: fd::OwnedFd,
    ptr: *mut u8,
    len: usize,
}

impl MemFdBuilder {
    pub fn new() -> Result<Self, io::Error> {
        use rustix::fs::{memfd_create, MemfdFlags};
        let fd = memfd_create("test", MemfdFlags::ALLOW_SEALING)?;
        let file = unsafe { fs::File::from_raw_fd(fd.as_raw_fd()) };

        Ok(MemFdBuilder { fd, file, len: 0 })
    }

    pub fn finish(mut self) -> Result<MemFd, io::Error> {
        self.flush()?;
        mem::forget(self.file);
        MemFd::new(self.fd, self.len)
    }
}

impl io::Write for MemFdBuilder {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        let written = self.file.write(buf)?;
        self.len += written;
        Ok(written)
    }

    fn flush(&mut self) -> io::Result<()> {
        self.file.flush()
    }
}

impl MemFd {
    pub fn create(len: usize) -> Result<Self, io::Error> {
        use rustix::fs::{ftruncate, memfd_create, MemfdFlags};

        let fd = memfd_create("test", MemfdFlags::ALLOW_SEALING)?;
        ftruncate(&fd, len as u64)?;

        Self::new(fd, len)
    }

    fn new(fd: fd::OwnedFd, len: usize) -> Result<Self, io::Error> {
        use rustix::{
            fs::{fcntl_add_seals, SealFlags},
            mm::{mmap, MapFlags, ProtFlags},
        };

        fcntl_add_seals(&fd, SealFlags::SHRINK | SealFlags::GROW)?;
        let ptr = unsafe {
            mmap(
                std::ptr::null_mut(),
                len,
                ProtFlags::READ | ProtFlags::WRITE,
                MapFlags::SHARED | MapFlags::NORESERVE,
                &fd,
                0,
            )
        }?;

        Ok(Self {
            fd,
            ptr: ptr.cast(),
            len,
        })
    }

    pub fn as_slice(&self) -> &[u8] {
        unsafe { std::slice::from_raw_parts(self.ptr, self.len) }
    }

    pub fn as_mut_slice(&mut self) -> &mut [u8] {
        unsafe { std::slice::from_raw_parts_mut(self.ptr, self.len) }
    }

    fn flush(&self) -> Result<(), io::Error> {
        use rustix::mm::{msync, MsyncFlags};
        unsafe { msync(self.ptr.cast(), self.len, MsyncFlags::SYNC | MsyncFlags::INVALIDATE)? };
        Ok(())
    }
}

impl fd::AsFd for MemFd {
    fn as_fd(&self) -> std::os::unix::prelude::BorrowedFd<'_> {
        self.fd.as_fd()
    }
}

impl Drop for MemFd {
    fn drop(&mut self) {
        use rustix::mm::munmap;
        unsafe { munmap(self.ptr.cast(), self.len).expect("MemFd munmap failed") }
    }
}

fn get_test_input(size: usize) -> Result<MemFd, io::Error> {
    let mut input = MemFd::create(size)?;
    for i in 0..size {
        // Some arbitrary polynomial mod 256.
        let b = ((i * i + 7493 * i + 16) % 256) as u8;
        input.as_mut_slice()[i] = b;
    }
    input.flush()?;

    Ok(input)
}
