use crate::RuntimeError;
use anyblox_format::model::{AnyBloxExtension, AnyBloxSelfContained, Metadata};
use std::{
    fmt::Debug,
    io,
    os::fd::{self, AsFd, AsRawFd},
};

pub struct MappedFd {
    fd: fd::OwnedFd,
    map: &'static [u8],
}

#[derive(Debug)]
pub struct MappableFd {
    fd: fd::OwnedFd,
    len: usize,
    offset: usize,
}

#[derive(Debug)]
pub enum AnyBloxBundle {
    Extension(MappedFd, MappedFd, AnyBloxExtension<'static>),
    SelfContained(MappedFd, MappableFd, AnyBloxSelfContained<'static>),
}

impl AnyBloxBundle {
    pub fn new_extension(
        anyblox_fd: impl fd::AsFd,
        anyblox_len: usize,
        dataset_fd: impl fd::AsFd,
        dataset_len: usize,
    ) -> Result<Self, RuntimeError> {
        let anyblox_map = MappedFd::map(anyblox_fd, anyblox_len)?;
        let dataset_map = MappedFd::map(dataset_fd, dataset_len)?;

        let file = anyblox_format::de::deserialize_bytes(anyblox_map.map)?;
        match file {
            anyblox_format::model::AnyBloxFile::Extension(ext) => Ok(Self::Extension(anyblox_map, dataset_map, ext)),
            anyblox_format::model::AnyBloxFile::SelfContained(_) => Err(RuntimeError::ExpectedExtension),
        }
    }

    pub fn new_self_contained<F: fd::AsFd>(fd: F, len: usize) -> Result<Self, RuntimeError> {
        let anyblox_map = MappedFd::map(&fd, len)?;

        let file = anyblox_format::de::deserialize_bytes(anyblox_map.map)?;
        match file {
            anyblox_format::model::AnyBloxFile::Extension(_) => Err(RuntimeError::ExpectedSelfContained),
            anyblox_format::model::AnyBloxFile::SelfContained(sc) => {
                // We need to create an fd for the dataset itself.
                // This requires duplicating the anyblox fd and seeking to the start of the dataset.
                // This relies on the internal implementation of deserialize_bytes creating the buffer from the raw pointer
                // of the slice passed to it. We can therefore ask for the distance between the start of the map
                // to the dataset's buffer's pointer and divine the seek distance from there.
                let base_ptr = anyblox_map.map.as_ptr();
                let dataset_ptr = sc.data().as_ptr();
                let distance = unsafe { dataset_ptr.offset_from(base_ptr) };
                assert!(distance >= 0);
                tracing::debug!(
                    "with anyblox at {base_ptr:?} and the dataset at {dataset_ptr:?} the distance is {distance}"
                );
                let dataset_fd = anyblox_map.fd.try_clone()?;
                let dataset = MappableFd {
                    fd: dataset_fd,
                    len: sc.data().len(),
                    offset: distance as usize,
                };
                Ok(Self::SelfContained(anyblox_map, dataset, sc))
            }
        }
    }

    pub fn anyblox_fd(&self) -> fd::BorrowedFd {
        match self {
            AnyBloxBundle::Extension(anyblox, _, _) => anyblox.fd.as_fd(),
            AnyBloxBundle::SelfContained(anyblox, _, _) => anyblox.fd.as_fd(),
        }
    }

    pub fn dataset_fd(&self) -> fd::BorrowedFd {
        match self {
            AnyBloxBundle::Extension(_, dataset, _) => dataset.fd.as_fd(),
            AnyBloxBundle::SelfContained(_, dataset, _) => dataset.fd.as_fd(),
        }
    }

    pub fn dataset_offset(&self) -> usize {
        match self {
            AnyBloxBundle::Extension(_, _, _) => 0,
            AnyBloxBundle::SelfContained(_, dataset_fd, _) => dataset_fd.offset,
        }
    }

    pub fn dataset_len(&self) -> usize {
        match self {
            AnyBloxBundle::Extension(_, dataset_map, _) => dataset_map.map.len(),
            AnyBloxBundle::SelfContained(_, dataset_fd, _) => dataset_fd.len,
        }
    }

    pub fn metadata(&self) -> &Metadata {
        match self {
            AnyBloxBundle::Extension(_, _, ext) => ext.metadata(),
            AnyBloxBundle::SelfContained(_, _, sc) => sc.metadata(),
        }
    }

    pub fn decoder(&self) -> &[u8] {
        match self {
            AnyBloxBundle::Extension(_, _, ext) => ext.decoder(),
            AnyBloxBundle::SelfContained(_, _, sc) => sc.decoder(),
        }
    }

    pub fn is_extension(&self) -> bool {
        matches!(self, Self::Extension(_, _, _))
    }

    pub fn is_self_contained(&self) -> bool {
        matches!(self, Self::SelfContained(_, _, _))
    }
}

impl MappedFd {
    fn map<F: fd::AsFd>(fd: F, len: usize) -> Result<Self, io::Error> {
        use rustix::mm::{mmap, MapFlags, ProtFlags};
        let fd = fd.as_fd().try_clone_to_owned()?;
        let ptr = unsafe {
            mmap(
                std::ptr::null_mut(),
                len,
                ProtFlags::READ,
                MapFlags::PRIVATE | MapFlags::NORESERVE,
                &fd,
                0,
            )?
        };
        let slice = unsafe { std::slice::from_raw_parts(ptr.cast::<u8>(), len) };

        tracing::debug!("anyblox file memory map for {} created at {ptr:?}", fd.as_raw_fd(),);

        Ok(Self { fd, map: slice })
    }
}

impl Drop for MappedFd {
    fn drop(&mut self) {
        use rustix::mm::munmap;
        tracing::debug!(
            "removing anyblox file memory map for {} created at {:?}",
            self.fd.as_raw_fd(),
            self.map.as_ptr()
        );
        unsafe {
            munmap(self.map.as_ptr().cast_mut().cast(), self.map.len()).expect("MappedFd munmap failed");
        }
    }
}

impl Debug for MappedFd {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MappedFd")
            .field("fd", &self.fd)
            .field("map.ptr", &self.map.as_ptr())
            .field("map.len", &self.map.len())
            .finish()
    }
}
