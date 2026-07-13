//! Append-oriented exFAT storage driver.
//!
//! The driver is a thin owner of an `exfat-slim` filesystem. Callers receive
//! opaque handles and never borrow filesystem objects directly.

mod directory;
mod driver;
mod error;
mod handles;
mod mount;
mod read;
#[cfg(feature = "stm32")]
pub mod stm32;
mod volume;
mod write;

pub use driver::{StorageDriver, BLOCK_BYTES, CACHE_BLOCKS, MAX_WRITE_HANDLES};
pub use error::{PartitionedDeviceError, StorageError, StorageResult, VolumeDetectError};
pub use exfat_slim::asynchronous::BlockDevice as StorageBlockDevice;
pub use handles::{FileHandle, ReadHandle};
pub use volume::{detect_exfat_volume, ExfatVolume, PartitionedBlockDevice};
