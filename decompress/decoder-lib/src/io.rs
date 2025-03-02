const BUF_SIZE: usize = 64 * 1024;
const BUF_PAD: usize = 64;

pub fn write_slice<T>(slice: &[T]) {
    let len = core::mem::size_of_val(slice);
    unsafe { crate::write(slice.as_ptr().cast(), len as u32) }
}

pub struct RawBuf {
    buf: [u8; BUF_SIZE + BUF_PAD],
    idx: usize,
}

impl RawBuf {
    pub fn new() -> Self {
        Self {
            buf: [0; BUF_SIZE + BUF_PAD],
            idx: 0,
        }
    }

    #[inline(always)]
    pub fn as_ptr(&self) -> *const u8 {
        self.buf.as_ptr()
    }

    #[inline(always)]
    pub fn as_mut_ptr(&mut self) -> *mut u8 {
        self.buf.as_mut_ptr()
    }

    #[inline(always)]
    pub fn write_unaligned<T>(&mut self, value: T, offset_by: usize) {
        assert!(core::mem::size_of::<T>() < BUF_SIZE);
        if self.rem_capacity() < core::mem::size_of::<T>() {
            self.flush();
        }
        unsafe {
            self.unsafe_write_unaligned(value, offset_by);
        }
    }

    #[inline(always)]
    pub unsafe fn unsafe_write_unaligned<T>(&mut self, value: T, offset_by: usize) {
        unsafe {
            self.buf
                .as_mut_ptr()
                .add(self.idx)
                .cast::<T>()
                .write_unaligned(value);
        }
        self.offset(offset_by);
    }

    pub fn offset(&mut self, offset: usize) {
        self.idx += offset;
    }

    #[inline(always)]
    pub fn flush(&mut self) {
        write_slice(&self.buf[..self.idx]);
        self.idx = 0;
    }

    #[inline(always)]
    pub fn idx(&self) -> usize {
        self.idx
    }

    #[inline(always)]
    pub fn rem_capacity(&self) -> usize {
        BUF_SIZE - self.idx
    }
}

impl Default for RawBuf {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for RawBuf {
    fn drop(&mut self) {
        self.flush();
    }
}

pub struct BufEmitter {
    buf: RawBuf,
}

impl BufEmitter {
    pub fn new() -> Self {
        Self { buf: RawBuf::new() }
    }

    #[inline(always)]
    pub fn write<T>(&mut self, value: T) {
        self.buf.write_unaligned(value, core::mem::size_of::<T>())
    }

    #[inline]
    pub fn write_buf(&mut self, data: &[u8]) {
        if self.rem_capacity() < data.len() {
            self.flush();
            if data.len() > BUF_SIZE {
                write_slice(data);
                return;
            }
        }

        let end = self.buf.idx + data.len();
        unsafe {
            core::ptr::copy_nonoverlapping(
                data.as_ptr(),
                self.buf.as_mut_ptr().add(self.buf.idx),
                end - self.buf.idx,
            )
        };
        self.buf.idx = end;
    }

    #[inline(always)]
    pub fn flush(&mut self) {
        self.buf.flush()
    }

    #[inline(always)]
    pub fn rem_capacity(&self) -> usize {
        self.buf.rem_capacity()
    }
}

impl Default for BufEmitter {
    fn default() -> Self {
        Self::new()
    }
}
