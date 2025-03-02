#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(feature = "alloc")]
pub mod alloc;
pub mod arrow;
pub mod column_projection;
pub mod ffi_utils;
#[cfg(feature = "wasm-core")]
pub mod io;
#[cfg(feature = "log")]
pub use decoder_logger::log;
#[cfg(not(feature = "log"))]
#[macro_export]
macro_rules! log {
    ($($arg:tt)+) => {};
}

#[cfg(feature = "wasm-core")]
#[link(wasm_import_module = "host")]
extern "C" {
    pub fn write(ptr: *const u8, len: u32);
    fn panic(ptr: *const u8, len: u32);
}

#[cfg(all(feature = "panic-handler", not(feature = "std"), not(test)))]
#[panic_handler]
fn panic_handler(info: &core::panic::PanicInfo) -> ! {
    use core::fmt::Write;
    log!("oops, panicked");
    let mut buf = PanicBuf::new();
    let _ = write!(buf, "{info}");
    unsafe { panic(buf.as_ptr(), buf.len() as u32) };
    loop {}
}

#[cfg(all(feature = "panic-handler", feature = "std", not(test)))]
pub fn panic_handler(info: &std::panic::PanicHookInfo) {
    use core::fmt::Write;
    log!("oops, panicked");
    let mut buf = PanicBuf::new();
    let _ = write!(buf, "{info}");
    unsafe { panic(buf.as_ptr(), buf.len() as u32) };
    std::process::exit(1);
}

#[cfg(all(feature = "panic-handler", not(test)))]
struct PanicBuf {
    buf: [u8; 1024],
    idx: usize,
}

#[cfg(all(feature = "panic-handler", not(test)))]
impl PanicBuf {
    const fn new() -> Self {
        Self {
            buf: [0; 1024],
            idx: 0,
        }
    }

    fn as_ptr(&self) -> *const u8 {
        self.buf.as_ptr()
    }

    fn len(&self) -> usize {
        self.buf.len()
    }

    fn rem(&self) -> usize {
        self.buf.len() - self.idx
    }

    fn clear(&mut self) {
        self.idx = 0;
    }
}

#[cfg(all(feature = "panic-handler", not(test)))]
impl core::fmt::Write for PanicBuf {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        let bytes = s.as_bytes();
        if bytes.len() > self.rem() {
            self.clear();
            self.write_str("ERROR: too long log message")?;
            return Ok(());
        }
        self.buf[self.idx..self.idx + bytes.len()].copy_from_slice(bytes);
        self.idx += bytes.len();

        Ok(())
    }
}
