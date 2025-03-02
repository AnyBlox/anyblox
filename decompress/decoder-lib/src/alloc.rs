use talc::{Talc, Talck, WasmHandler};

struct NotAMutex {}

unsafe impl lock_api::RawMutex for NotAMutex {
    const INIT: Self = NotAMutex {};

    type GuardMarker = ();

    fn lock(&self) {}

    fn try_lock(&self) -> bool {
        true
    }

    unsafe fn unlock(&self) {}
}

#[global_allocator]
static ALLOCATOR: Talck<NotAMutex, WasmHandler> = Talc::new(unsafe { WasmHandler::new() }).lock();