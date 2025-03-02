use crate::ffi_utils::WasmPtr;

#[repr(C)]
pub struct ArrowArray {
    pub length: u32,
    pub null_count: u32,
    pub offset: u32,
    pub n_buffers: u32,
    pub n_children: u32,
    // void** buffers
    pub buffers: WasmPtr,
    // ArrowArray** children
    pub children: WasmPtr,
    // ArrowArray* dictionary
    pub dictionary: WasmPtr,
}

impl ArrowArray {
    pub const fn empty() -> Self {
        Self {
            length: 0,
            null_count: 0,
            offset: 0,
            n_buffers: 0,
            n_children: 0,
            buffers: WasmPtr::NULL,
            children: WasmPtr::NULL,
            dictionary: WasmPtr::NULL,
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

    pub unsafe fn buffers(&self, base_ptr: *const u8) -> impl Iterator<Item = *const u8> + '_ {
        struct BufIter<'a> {
            base_ptr: *const u8,
            bufs: &'a [WasmPtr],
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
                    if buf.is_null() {
                        Some(core::ptr::null())
                    } else {
                        Some(unsafe { buf.as_native(self.base_ptr) })
                    }
                }
            }
        }

        if self.n_buffers == 0 {
            // This is required because the actual ptr can be whatever garbage, and Rust requires even empty slices to have proper pointers.
            BufIter {
                base_ptr,
                bufs: &[],
                idx: 0,
            }
        } else {
            // This is required because the actual ptr can be whatever garbage, and Rust requires even empty slices to have proper pointers.
            BufIter {
                base_ptr,
                bufs: core::slice::from_raw_parts(
                    self.buffers.as_native(base_ptr).cast(),
                    self.n_buffers as usize,
                ),
                idx: 0,
            }
        }
    }

    pub unsafe fn children(&self, base_ptr: *const u8) -> impl Iterator<Item = *mut Self> + '_ {
        struct ChildIter<'a> {
            base_ptr: *const u8,
            children: &'a [WasmPtr],
            idx: usize,
        }
        impl Iterator for ChildIter<'_> {
            type Item = *mut ArrowArray;

            fn next(&mut self) -> Option<Self::Item> {
                if self.idx == self.children.len() {
                    None
                } else {
                    let child = self.children[self.idx];
                    self.idx += 1;
                    Some(unsafe { child.as_native(self.base_ptr).cast_mut().cast() })
                }
            }
        }

        if self.n_children == 0 {
            ChildIter {
                base_ptr,
                children: &[],
                idx: 0,
            }
        } else {
            ChildIter {
                base_ptr,
                children: core::slice::from_raw_parts(
                    self.children.as_native(base_ptr).cast(),
                    self.n_children as usize,
                ),
                idx: 0,
            }
        }
    }

    pub unsafe fn dictionary(&self, base_ptr: *const u8) -> *mut Self {
        if self.dictionary.is_null() {
            core::ptr::null_mut()
        } else {
            self.dictionary.as_native_mut(base_ptr.cast_mut()).cast()
        }
    }
}

#[repr(C)]
pub struct RecordBatch {
    buffers: [WasmPtr; 64],
    buffer_lens: [u32; 64],
    fld_to_data_buf: [u32; 64],
    fld_to_validity_buf: [u32; 64],
    fld_to_offsets_buf: [u32; 64],
    null_counts: [u32; 64],
    buf_count: u32,
    row_count: u32,
}

impl RecordBatch {
    pub const fn empty() -> Self {
        Self {
            buffers: [WasmPtr::NULL; 64],
            buffer_lens: [0; 64],
            fld_to_data_buf: [0; 64],
            fld_to_validity_buf: [0; 64],
            fld_to_offsets_buf: [0; 64],
            null_counts: [0; 64],
            buf_count: 0,
            row_count: 0,
        }
    }

    pub fn new<const B: usize, const F: usize>(
        buffers: [WasmPtr; B],
        buffer_lens: [u32; B],
        fld_to_data_buf: [u32; F],
        fld_to_validity_buf: [u32; F],
        fld_to_offsets_buf: [u32; F],
        null_counts: [u32; F],
        row_count: u32,
    ) -> Self {
        let mut x = Self::empty();
        x.write(
            buffers,
            buffer_lens,
            fld_to_data_buf,
            fld_to_validity_buf,
            fld_to_offsets_buf,
            null_counts,
            row_count,
        );
        x
    }

    pub fn write<const B: usize, const F: usize>(
        &mut self,
        buffers: [WasmPtr; B],
        buffer_lens: [u32; B],
        fld_to_data_buf: [u32; F],
        fld_to_validity_buf: [u32; F],
        fld_to_offsets_buf: [u32; F],
        null_counts: [u32; F],
        row_count: u32,
    ) {
        assert!(B <= 64);
        assert!(F <= 64);
        self.buffers[..B].copy_from_slice(&buffers);
        self.buffer_lens[..B].copy_from_slice(&buffer_lens);
        self.fld_to_data_buf[..F].copy_from_slice(&fld_to_data_buf);
        self.fld_to_validity_buf[..F].copy_from_slice(&fld_to_validity_buf);
        self.fld_to_offsets_buf[..F].copy_from_slice(&fld_to_offsets_buf);
        self.null_counts[..F].copy_from_slice(&null_counts);
        self.row_count = row_count;
        self.buf_count = B as u32;
    }

    pub unsafe fn buffer(&self, buf_idx: usize, base_ptr: *const u8) -> &[u8] {
        let len = self.buffer_lens[buf_idx] as usize;
        core::slice::from_raw_parts(self.buffers[buf_idx].as_native(base_ptr), len)
    }

    pub fn buffer_count(&self) -> u32 {
        self.buf_count as u32
    }

    pub fn data_buffer_of_field(&self, fld_idx: usize) -> u32 {
        self.fld_to_data_buf[fld_idx]
    }

    pub fn validity_buffer_of_field(&self, fld_idx: usize) -> u32 {
        self.fld_to_validity_buf[fld_idx]
    }

    pub fn offsets_buffer_of_field(&self, fld_idx: usize) -> u32 {
        self.fld_to_offsets_buf[fld_idx]
    }

    pub fn null_count(&self, fld_idx: usize) -> u32 {
        self.null_counts[fld_idx]
    }

    pub fn row_count(&self) -> u32 {
        self.row_count
    }
}

impl core::fmt::Debug for RecordBatch {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let buf_count = self.buffer_count() as usize;
        f.debug_struct("RecordBatch")
            .field("buffers", &&self.buffers[..buf_count])
            .field("buffer_lens", &&self.buffer_lens[..buf_count])
            .field("fld_to_data_buf", &&self.fld_to_data_buf[..buf_count])
            .field("fld_to_null_buf", &&self.fld_to_validity_buf[..buf_count])
            .field("null_counts", &&self.null_counts[..buf_count])
            .field("row_count", &self.row_count)
            .finish()
    }
}
