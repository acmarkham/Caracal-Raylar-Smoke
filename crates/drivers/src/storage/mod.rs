//! Append-oriented exFAT storage driver.
//!
//! The driver is a thin owner of an `exfat-slim` filesystem. Callers receive
//! opaque handles and never borrow filesystem objects directly.

mod directory;
mod driver;
mod error;
mod handles;
mod identity;
mod mount;
mod read;
#[cfg(feature = "stm32")]
pub mod stm32;
mod volume;
mod write;

pub use driver::{BLOCK_BYTES, CACHE_BLOCKS, MAX_WRITE_HANDLES, StorageDriver};
pub use error::{PartitionedDeviceError, StorageError, StorageResult, VolumeDetectError};
pub use exfat_slim::asynchronous::BlockDevice as StorageBlockDevice;
pub use exfat_slim::timestamp::Timestamp as StorageTimestamp;
pub use handles::{FileHandle, ReadHandle};
pub use identity::StorageDeviceIdentity;
pub use volume::{ExfatVolume, PartitionedBlockDevice, detect_exfat_volume};
