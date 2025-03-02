#![no_std]

#[link(wasm_import_module = "host")]
extern "C" {
    pub fn log(ptr: *const u8, size: u32);
}

#[macro_export]
macro_rules! log {
    ($($arg:tt)+) => {
        unsafe {
            $crate::Log.start();
            write!($crate::Log, $($arg)+).unwrap();
            $crate::Log.end();
        }
    };
}

pub struct Log;

static mut STATIC_LOG: LogBuf = LogBuf::new();

impl Log {
    pub unsafe fn start(self) {
        STATIC_LOG.clear();
    }

    pub unsafe fn end(self) {
        STATIC_LOG.flush();
    }
}

impl core::fmt::Write for Log {
    fn write_fmt(&mut self, args: core::fmt::Arguments<'_>) -> core::fmt::Result {
        unsafe {
            STATIC_LOG.write_fmt(args)?;
        }
        Ok(())
    }

    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        unsafe {
            STATIC_LOG.write_str(s)?;
        }
        Ok(())
    }
}

struct LogBuf {
    buf: [u8; 1024],
    idx: usize,
}

impl LogBuf {
    const fn new() -> Self {
        Self {
            buf: [0; 1024],
            idx: 0,
        }
    }

    fn rem(&self) -> usize {
        self.buf.len() - self.idx
    }

    fn clear(&mut self) {
        self.idx = 0;
    }

    fn flush(&mut self) {
        unsafe { log(self.buf.as_ptr(), self.idx as u32) };
    }
}

impl core::fmt::Write for LogBuf {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        let bytes = s.as_bytes();
        if bytes.len() > self.rem() {
            self.clear();
            self.write_str("ERROR: too long log message")?;
            self.flush();
        }
        self.buf[self.idx..self.idx + bytes.len()].copy_from_slice(bytes);
        self.idx += bytes.len();

        Ok(())
    }
}
