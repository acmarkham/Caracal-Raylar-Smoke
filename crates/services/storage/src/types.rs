use raylar_drivers::storage::FileHandle;

pub(crate) const PATH_CAPACITY: usize = 128;

#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum StreamType {
    Log,
    Audio,
    GpsTiming,
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
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AppendOutcome {
    Written,
    DroppedUtcUnavailable,
}

#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct RollingPolicy {
    pub folder_interval_seconds: i64,
    pub file_interval_seconds: i64,
    pub startup_alignment_seconds: i64,
}

impl RollingPolicy {
    pub const fn audio_default() -> Self {
        Self {
            folder_interval_seconds: 86_400,
            file_interval_seconds: 3_600,
            startup_alignment_seconds: 60,
        }
    }

    pub const fn gps_default() -> Self {
        Self {
            folder_interval_seconds: 86_400,
            file_interval_seconds: 86_400,
            startup_alignment_seconds: 86_400,
        }
    }

    pub(crate) const fn is_valid(self) -> bool {
        self.folder_interval_seconds > 0
            && self.file_interval_seconds > 0
            && self.startup_alignment_seconds > 0
    }
}

#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct StorageConfig {
    pub audio: RollingPolicy,
    pub gps_timing: RollingPolicy,
}

impl Default for StorageConfig {
    fn default() -> Self {
        Self {
            audio: RollingPolicy::audio_default(),
            gps_timing: RollingPolicy::gps_default(),
        }
    }
}

#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Debug, PartialEq, Eq)]
pub enum StorageServiceError<E> {
    Backend(E),
    InvalidConfig,
    InvalidStream,
    StreamAlreadyOpen,
    TooManyStreams,
    InvalidTimestamp,
    InvalidPath,
}

pub(crate) struct StreamSlot<const BLOCK_SIZE: usize> {
    pub kind: StreamType,
    pub generation: u8,
    pub file: Option<FileHandle>,
    pub file_start: Option<i64>,
    pub rollover_at: Option<i64>,
    pub path: heapless::String<PATH_CAPACITY>,
    pub pending: [u8; BLOCK_SIZE],
    pub pending_len: usize,
}

impl<const BLOCK_SIZE: usize> StreamSlot<BLOCK_SIZE> {
    pub fn new(kind: StreamType, generation: u8) -> Self {
        Self {
            kind,
            generation,
            file: None,
            file_start: None,
            rollover_at: None,
            path: heapless::String::new(),
            pending: [0; BLOCK_SIZE],
            pending_len: 0,
        }
    }
}
