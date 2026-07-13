use exfat_slim::asynchronous::error::ExFatError;

pub type StorageResult<T, E> = Result<T, StorageError<E>>;

#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VolumeDetectError<E> {
    Io(E),
    NoFilesystem,
    CardTooLarge,
    UnsupportedSectorSize,
    InvalidVolume,
}

#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PartitionedDeviceError<E> {
    Inner(E),
    OutOfRange,
}

#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Debug)]
pub enum StorageError<E> {
    Filesystem(ExFatError<E>),
    CardRemoved,
    CardNotPresent,
    NoFilesystem,
    DirectoryNotFound,
    FileNotFound,
    FileAlreadyExists,
    TooManyOpenFiles,
    ReadAlreadyOpen,
    InvalidHandle,
    InvalidState,
    InvalidPath,
    InvalidBufferLength,
    IoError(E),
    OutOfSpace,
}

impl<E> From<ExFatError<E>> for StorageError<E> {
    fn from(error: ExFatError<E>) -> Self {
        Self::Filesystem(error)
    }
}
