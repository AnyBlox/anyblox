use std::{ptr, sync::Mutex};

use crate::ColumnProjection;
mod rle;
mod rle_linestatus;
mod rle_simd_avx2;
mod rle_simd_avx512;
mod rle_simd_avx512_stateless;
mod rle_simd_sse2;
mod taxpayer_fsst;
mod taxpayer_libfsst;
mod tpch_vortex;
//mod trunc8;

pub struct NativeProgram {
    inner: Mutex<Box<dyn NativeImpl>>,
}

unsafe impl Send for NativeProgram{}

#[derive(Debug)]
pub struct NativeBatch {
    pub length: u32,
    pub null_count: u32,
    pub offset: u32,
    pub n_buffers: u32,
    pub n_children: u32,
    // void** buffers
    pub buffers: *mut *const u8,
    // ArrowArray** children
    pub children: *mut *mut NativeBatch,
    // ArrowArray* dictionary
    pub dictionary: *mut NativeBatch,
}

impl NativeBatch {
    pub const fn empty() -> Self {
        Self {
            length: 0,
            null_count: 0,
            offset: 0,
            n_buffers: 0,
            n_children: 0,
            buffers: ptr::null_mut(),
            children: ptr::null_mut(),
            dictionary: ptr::null_mut(),
        }
    }

    pub fn row_count(&self) -> u32 {
        self.length
    }

    pub fn null_count(&self) -> u32 {
        self.null_count
    }

    pub fn offset(&self) -> u32 {
        self.offset
    }

    pub fn buffer_count(&self) -> u32 {
        self.n_buffers
    }

    pub fn children_count(&self) -> u32 {
        self.n_children
    }

    pub unsafe fn buffers(&self) -> impl Iterator<Item = *const u8> + '_ {
        struct BufIter<'a> {
            bufs: &'a [*const u8],
            idx: usize,
        }
        impl Iterator for BufIter<'_> {
            type Item = *const u8;

            fn next(&mut self) -> Option<Self::Item> {
                if self.idx == self.bufs.len() {
                    None
                } else {
                    let buf = self.bufs[self.idx];
                    self.idx += 1;
                    Some(buf)
                }
            }
        }

        if self.n_buffers == 0 {
            // This is required because the actual ptr can be whatever garbage, and Rust requires even empty slices to have proper pointers.
            BufIter { bufs: &[], idx: 0 }
        } else {
            BufIter {
                bufs: core::slice::from_raw_parts(self.buffers, self.n_buffers as usize),
                idx: 0,
            }
        }
    }

    pub unsafe fn children(&self) -> impl Iterator<Item = *mut Self> + '_ {
        struct ChildIter<'a> {
            children: &'a [*mut NativeBatch],
            idx: usize,
        }
        impl Iterator for ChildIter<'_> {
            type Item = *mut NativeBatch;

            fn next(&mut self) -> Option<Self::Item> {
                if self.idx == self.children.len() {
                    None
                } else {
                    let child = self.children[self.idx];
                    self.idx += 1;
                    Some(child)
                }
            }
        }

        if self.n_children == 0 {
            // This is required because the actual ptr can be whatever garbage, and Rust requires even empty slices to have proper pointers.
            ChildIter { children: &[], idx: 0 }
        } else {
            ChildIter {
                children: core::slice::from_raw_parts(self.children.cast(), self.n_children as usize),
                idx: 0,
            }
        }
    }

    pub unsafe fn dictionary(&self) -> *mut Self {
        if self.dictionary.is_null() {
            core::ptr::null_mut()
        } else {
            self.dictionary
        }
    }
}

trait NativeImpl {
    fn anyblox_decode(
        &mut self,
        data: &[u8],
        start_tuple: usize,
        tuple_count: usize,
        projection: ColumnProjection,
    ) -> &NativeBatch;
}

impl NativeProgram {
    pub fn run<'p>(
        &'p mut self,
        data: &[u8],
        start_tuple: usize,
        tuple_count: usize,
        projection: ColumnProjection,
    ) -> Result<&'p NativeBatch, arrow::error::ArrowError> {
        let batch = self
            .inner
            .get_mut()
            .unwrap()
            .anyblox_decode(data, start_tuple, tuple_count, projection);
        Ok(batch)
    }

    pub fn rle() -> Self {
        Self {
            inner: Mutex::new(Box::new(rle::NativeRleImpl::default())),
        }
    }

    pub fn rle_linestatus() -> Self {
        Self {
            inner: Mutex::new(Box::new(rle_linestatus::NativeRleLinestatusImpl::default())),
        }
    }

    pub fn rle_simd() -> Self {
        Self {
            inner: Mutex::new(Box::new(rle_simd_avx512::NativeRleSimdAvx512Impl::default())),
        }
    }

    pub fn rle_simd_avx2() -> Self {
        Self {
            inner: Mutex::new(Box::new(rle_simd_avx2::NativeRleSimdAvx2Impl::default())),
        }
    }

    pub fn rle_simd_sse2() -> Self {
        Self {
            inner: Mutex::new(Box::new(rle_simd_sse2::NativeRleSimdSse2Impl::default())),
        }
    }

    pub fn rle_simd_stateless() -> Self {
        Self {
            inner: Mutex::new(Box::new(
                rle_simd_avx512_stateless::NativeRleSimdAvx512StatelessImpl::default(),
            )),
        }
    }

    /*pub fn trunc8() -> Self {
        Self {
            inner: Mutex::new(Box::new(trunc8::NativeTrunc8Impl::default())),
        }
    }*/

    pub fn taxpayer_fsst() -> Self {
        Self {
            inner: Mutex::new(Box::new(taxpayer_fsst::NativeTaxpayerFsstImpl::default())),
        }
    }

    pub fn taxpayer_libfsst() -> Self {
        Self {
            inner: Mutex::new(Box::new(taxpayer_libfsst::NativeTaxpayerLibfsstImpl::default())),
        }
    }

    pub fn tpch_vortex() -> Self {
        Self {
            inner: Mutex::new(Box::new(tpch_vortex::NativeTpchVortexImpl::default())),
        }
    }
}
