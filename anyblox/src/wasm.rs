use crate::units::*;
use thiserror::Error;
pub mod mem;

pub const PAGE_SIZE: usize = 64 * KIB;
pub const MAX_MEM_SIZE: usize = 4 * GIB;
pub const MAX_PAGE_COUNT: usize = MAX_MEM_SIZE / PAGE_SIZE;
pub const DEFAULT_INITIAL_RESERVATION_IN_PAGES: usize = 16;

#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    SyscallError(#[from] rustix::io::Errno),
}

// Nothing unsafe here, the pointers are not accessible, we're just printing the addresses for easier debugging.
unsafe impl Send for Error {}
unsafe impl Sync for Error {}
