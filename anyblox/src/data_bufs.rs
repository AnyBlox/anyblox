use crate::config::Config;
use rustix::fd::{AsFd, AsRawFd};
use std::os::fd::{self, RawFd};
use thiserror::Error;

pub struct Datasets {}

#[derive(Debug)]
pub struct Dataset {
    original_raw_fd: RawFd,
    fd: fd::OwnedFd,
    offset: usize,
    len: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DatasetDescriptor {
    original_raw_fd: RawFd,
    offset: usize,
    len: usize,
}

impl Dataset {
    pub fn fd(&self) -> fd::BorrowedFd {
        self.fd.as_fd()
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn offset(&self) -> usize {
        self.offset
    }

    pub fn descriptor(&self) -> DatasetDescriptor {
        DatasetDescriptor {
            original_raw_fd: self.original_raw_fd,
            offset: self.offset,
            len: self.len,
        }
    }
}

impl DatasetDescriptor {
    pub fn len(&self) -> usize {
        self.len
    }
}

impl AsFd for Dataset {
    fn as_fd(&self) -> fd::BorrowedFd<'_> {
        self.fd()
    }
}

impl AsRawFd for Dataset {
    fn as_raw_fd(&self) -> fd::RawFd {
        self.fd().as_raw_fd()
    }
}

impl Datasets {
    pub fn new(_config: &Config) -> Result<Self, Error> {
        Ok(Self {})
    }

    #[tracing::instrument(skip(self, fd), fields(fd = fd.as_fd().as_raw_fd()))]
    pub fn load_dataset<F: AsFd>(&self, fd: F, offset: usize, len: usize) -> Result<Dataset, Error> {
        tracing::debug!("creating new dataset with {:?} and {len}", fd.as_fd());
        let original_raw_fd = fd.as_fd().as_raw_fd();
        let fd = fd.as_fd().try_clone_to_owned()?;
        tracing::debug!("cloned fd: {fd:?}");

        Ok(Dataset {
            original_raw_fd,
            fd,
            offset,
            len,
        })
    }
}

#[derive(Debug, Error, Clone)]
pub enum Error {
    #[error("io error: {0}")]
    IoError(String),
    #[error(transparent)]
    SyscallError(#[from] rustix::io::Errno),
    #[error("data path does not point to an existing directory")]
    DataIsNotDirectory,
    #[error("pending task was cancelled unexpectedly")]
    TaskCancelled,
    #[error("async task panicked: {0}")]
    TaskPanicked(String),
}

impl From<std::io::Error> for Error {
    fn from(value: std::io::Error) -> Self {
        Self::IoError(value.to_string())
    }
}

impl From<tokio::task::JoinError> for Error {
    fn from(value: tokio::task::JoinError) -> Self {
        if value.is_cancelled() {
            Self::TaskCancelled
        } else {
            Self::TaskPanicked(value.to_string())
        }
    }
}
