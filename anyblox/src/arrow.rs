mod byte_repr;
mod utf8_validation;

use crate::programs;
pub use byte_repr::ByteRepr;
use std::{
    ffi::c_void,
    fmt::Debug,
    ptr::{self},
    sync::Arc,
};
pub use utf8_validation::Utf8Validator;

#[repr(C)]
pub struct AnyBloxRecordBatch {
    length: i64,
    null_count: i64,
    offset: i64,
    n_buffers: i64,
    n_children: i64,
    buffers: *mut *const c_void,
    children: *mut *mut AnyBloxRecordBatch,
    dictionary: *mut AnyBloxRecordBatch,
    release: Option<unsafe extern "C" fn(arg1: *mut AnyBloxRecordBatch)>,
    private_data: *mut c_void,
}

struct PrivateData {
    buffers: Vec<*const c_void>,
    children: Vec<*mut AnyBloxRecordBatch>,
    dictionary: *mut AnyBloxRecordBatch,
}

impl AnyBloxRecordBatch {
    pub fn buffers(&self) -> &[*const c_void] {
        unsafe { std::slice::from_raw_parts(self.buffers, self.n_buffers as usize) }
    }

    pub fn row_count(&self) -> usize {
        self.length as usize
    }

    pub fn null_count(&self) -> usize {
        self.null_count as usize
    }

    pub unsafe fn from_native(batch: &programs::native::NativeBatch) -> Self {
        let buf_count = batch.buffer_count() as usize;

        let mut result = AnyBloxRecordBatch {
            length: i64::from(batch.row_count()),
            null_count: i64::from(batch.null_count()),
            offset: i64::from(batch.offset()),
            n_buffers: i64::from(batch.buffer_count()),
            n_children: i64::from(batch.children_count()),
            buffers: ptr::null_mut(),
            children: ptr::null_mut(),
            dictionary: ptr::null_mut(),
            release: None,
            private_data: ptr::null_mut(),
        };

        let mut buffers = Vec::with_capacity(buf_count);
        let mut children = Vec::with_capacity(buf_count);

        for buf in batch.buffers() {
            buffers.push(buf.cast::<c_void>())
        }
        result.buffers = buffers.as_mut_ptr();

        for child in batch.children() {
            let converted = Self::from_native(&*child);
            children.push(Box::into_raw(Box::new(converted)));
        }
        result.children = children.as_mut_ptr();

        let dictionary = batch.dictionary();
        if !dictionary.is_null() {
            let converted = Self::from_native(&*dictionary);
            result.dictionary = Box::into_raw(Box::new(converted));
        }

        result.private_data = Box::into_raw(Box::new(PrivateData {
            buffers,
            children,
            dictionary: result.dictionary,
        }))
        .cast();
        result.release = Some(release_batch);

        result
    }

    pub unsafe fn from_wasm(
        base_ptr: *const u8,
        batch: &decoder_lib::arrow::ArrowArray,
    ) -> Result<Self, ::arrow::error::ArrowError> {
        let buf_count = batch.buffer_count() as usize;

        let mut result = AnyBloxRecordBatch {
            length: i64::from(batch.row_count()),
            null_count: i64::from(batch.null_count()),
            offset: i64::from(batch.offset()),
            n_buffers: i64::from(batch.buffer_count()),
            n_children: i64::from(batch.children_count()),
            buffers: ptr::null_mut(),
            children: ptr::null_mut(),
            dictionary: ptr::null_mut(),
            release: None,
            private_data: ptr::null_mut(),
        };

        let mut buffers = Vec::with_capacity(buf_count);
        let mut children = Vec::with_capacity(buf_count);

        for buf in batch.buffers(base_ptr) {
            buffers.push(buf.cast::<c_void>())
        }
        result.buffers = buffers.as_mut_ptr();

        for child in batch.children(base_ptr) {
            let converted = Self::from_wasm(base_ptr, &*child)?;
            children.push(Box::into_raw(Box::new(converted)));
        }
        result.children = children.as_mut_ptr();

        let dictionary = batch.dictionary(base_ptr);
        if !dictionary.is_null() {
            let converted = Self::from_wasm(base_ptr, &*dictionary)?;
            result.dictionary = Box::into_raw(Box::new(converted));
        }

        result.private_data = Box::into_raw(Box::new(PrivateData {
            buffers,
            children,
            dictionary: result.dictionary,
        }))
        .cast();
        result.release = Some(release_batch);

        Ok(result)
    }

    pub fn into_arrow_ffi(self) -> ::arrow::ffi::FFI_ArrowArray {
        unsafe { std::mem::transmute(self) }
    }

    pub fn into_arrow_record_batch(
        self,
        schema: Arc<::arrow::datatypes::Schema>,
    ) -> Result<::arrow::record_batch::RecordBatch, ::arrow::error::ArrowError> {
        let struct_field = ::arrow::datatypes::DataType::Struct(schema.fields().clone());

        let array_data = unsafe {
            let ffi_array: ::arrow::ffi::FFI_ArrowArray = std::mem::transmute(self);
            ::arrow::ffi::from_ffi_and_data_type(ffi_array, struct_field)?
        };
        let array = ::arrow::array::StructArray::from(array_data);
        ::arrow::record_batch::RecordBatch::try_new(schema, array.columns().to_vec())
    }
}

impl Debug for AnyBloxRecordBatch {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut f = f.debug_struct("AnyBloxRecordBatch");
        f.field("length", &self.length)
            .field("null_count", &self.null_count)
            .field("offset", &self.offset)
            .field("n_buffers", &self.n_buffers)
            .field("n_children", &self.n_children);
        if self.release.is_none() {
            f.field("release", &None::<()>).finish_non_exhaustive()
        } else {
            let data = self.private_data as *mut PrivateData;

            f.field("private_data", unsafe { &*data })
                .field("release", &"...")
                .finish_non_exhaustive()
        }
    }
}

impl Debug for PrivateData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        struct List<'a>(&'a [*mut AnyBloxRecordBatch]);
        impl Debug for List<'_> {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                let mut f = f.debug_list();
                for child in self.0 {
                    f.entry(unsafe { &**child });
                }
                f.finish()
            }
        }

        let mut d = f.debug_struct("PrivateData");
        d.field("buffers", &self.buffers)
            .field("children", &List(&self.children));

        if self.dictionary.is_null() {
            d.field("dictionary", &"<null>").finish()
        } else {
            d.field("dictionary", unsafe { &*self.dictionary }).finish()
        }
    }
}

impl Drop for AnyBloxRecordBatch {
    fn drop(&mut self) {
        if let Some(release) = self.release {
            unsafe { release(self) }
        }
    }
}

unsafe extern "C" fn release_batch(batch: *mut AnyBloxRecordBatch) {
    if batch.is_null() {
        return;
    }
    let array = &mut *batch;

    let private = Box::from_raw(array.private_data as *mut PrivateData);
    for child in private.children.into_iter() {
        drop(Box::from_raw(child));
    }
    if !private.dictionary.is_null() {
        drop(Box::from_raw(private.dictionary));
    }

    array.release = None;
}
