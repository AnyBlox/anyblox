use super::AnyBloxRecordBatch;
use crate::RuntimeError;
use arrow::datatypes::{DataType, Schema};
use std::ffi::c_void;

#[derive(Debug)]
pub struct Utf8Validator {
    utf8_cols: Vec<usize>,
    view_cols: Vec<usize>,
}

impl Utf8Validator {
    pub fn for_schema(schema: &Schema) -> Self {
        let mut utf8_cols = vec![];
        let mut view_cols = vec![];
        for (i, f) in schema.fields().iter().enumerate() {
            match f.data_type() {
                DataType::Utf8 => utf8_cols.push(i),
                DataType::Utf8View => view_cols.push(i),
                DataType::LargeUtf8 => unimplemented!(),
                _ => (),
            }
        }
        Self { utf8_cols, view_cols }
    }

    pub fn is_empty(&self) -> bool {
        self.utf8_cols.is_empty() && self.view_cols.is_empty()
    }

    pub fn validate_utf8_columns(&self, batch: &AnyBloxRecordBatch) -> Result<(), RuntimeError> {
        for &i in self.utf8_cols.iter() {
            unsafe {
                let column = batch.children.add(i).read();
                let len = (*column).row_count();
                let offsets_buf = (*column).buffers.add(1).read();
                let data_buf = (*column).buffers.add(2).read();
                if !Self::validate_utf8_column(offsets_buf, data_buf, len) {
                    return Err(RuntimeError::Utf8ValidationError(i));
                }
            }
        }
        for &i in self.view_cols.iter() {
            unsafe {
                let column = batch.children.add(i).read();
                let len = (*column).row_count();
                let views_buf = (*column).buffers.add(1).read();
                let data_bufs = (*column).buffers.add(2);
                if !Self::validate_utf8_view(views_buf, data_bufs, len) {
                    return Err(RuntimeError::Utf8ValidationError(i));
                }
            }
        }
        Ok(())
    }

    unsafe fn validate_utf8_column(offsets: *const c_void, data: *const c_void, len: usize) -> bool {
        let mut offsets = offsets.cast::<u32>().add(1);
        let data = data.cast::<u8>();
        let mut offset = 0;
        let mut idx = 0;

        while idx < len {
            let next_offset = offsets.read() as usize;
            let str_len = next_offset - offset;
            let utf8_buf = std::slice::from_raw_parts(data.add(offset), str_len);
            if !simdutf::validate_utf8(utf8_buf) {
                return false;
            }

            offset = next_offset;
            offsets = offsets.add(1);
            idx += 1;
        }

        true
    }

    unsafe fn validate_utf8_view(views: *const c_void, buffers: *mut *const c_void, len: usize) -> bool {
        let views = views.cast::<u32>();
        let buffers = buffers.cast::<*const u8>();
        let mut idx = 0;

        while idx < len {
            let view = views.add(4 * idx);
            let str_len = view.read() as usize;
            let utf8_buf = if str_len <= 12 {
                let ptr = view.add(1).cast::<u8>();
                std::slice::from_raw_parts(ptr, str_len)
            } else {
                let buf_idx = view.add(2).read() as usize;
                let offset = view.add(3).read() as usize;
                let buf = buffers.add(buf_idx).read();
                std::slice::from_raw_parts(buf.add(offset), str_len)
            };

            if !simdutf::validate_utf8(utf8_buf) {
                return false;
            }
            idx += 1;
        }

        true
    }
}
