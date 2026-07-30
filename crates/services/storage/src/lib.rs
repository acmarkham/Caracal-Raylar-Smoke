#![no_std]

#[cfg(test)]
extern crate std;

mod backend;
mod policy;
mod service;
mod types;

pub use backend::StorageBackend;
pub use service::{StorageService, UtcClock, DEFAULT_MAX_STREAMS};
pub use types::{StorageLayout, StorageServiceError, StreamHandle, StreamKind};
