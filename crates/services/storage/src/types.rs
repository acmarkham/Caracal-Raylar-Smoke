use raylar_drivers::storage::FileHandle;

pub(crate) const PATH_CAPACITY: usize = 128;

#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum StreamKind {
    Log,
    Audio,
    GpsTiming,
}

#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum StorageLayout {
    Flat,
    DailyFolders,
    HourlyFolders,
    MissionFolders,
    IntervalFolders { interval_seconds: i64 },
}

impl StorageLayout {
    pub(crate) const fn is_valid(self) -> bool {
        match self {
            Self::IntervalFolders { interval_seconds } => interval_seconds > 0,
            _ => true,
        }
    }
}

#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct StreamHandle {
    index: u8,
    generation: u8,
}

impl StreamHandle {
    pub(crate) const fn new(index: usize, generation: u8) -> Self {
        Self {
            index: index as u8,
            generation,
        }
    }

    pub(crate) const fn index(self) -> usize {
        self.index as usize
    }

    pub(crate) const fn generation(self) -> u8 {
        self.generation
    }
}

#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Debug, PartialEq, Eq)]
pub enum StorageServiceError<E> {
    Backend(E),
    InvalidConfig,
    InvalidStream,
    TooManyStreams,
    InvalidTimestamp,
    InvalidPath,
}

pub(crate) struct StreamSlot<const BLOCK_SIZE: usize> {
    pub generation: u8,
    pub file: Option<FileHandle>,
    pub path: heapless::String<PATH_CAPACITY>,
    pub pending: [u8; BLOCK_SIZE],
    pub pending_len: usize,
}

impl<const BLOCK_SIZE: usize> StreamSlot<BLOCK_SIZE> {
    pub fn new(generation: u8) -> Self {
        Self {
            generation,
            file: None,
            path: heapless::String::new(),
            pending: [0; BLOCK_SIZE],
            pending_len: 0,
        }
    }
}
