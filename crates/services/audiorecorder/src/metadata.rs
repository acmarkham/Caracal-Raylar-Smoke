use heapless::String;
use raylar_time_service::{TimeResources, UtcTimestamp};

pub const DEVICE_ID_CAPACITY: usize = 32;
pub const FIRMWARE_VERSION_CAPACITY: usize = 32;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RecordingMetadata {
    pub started_utc: UtcTimestamp,
    pub latitude_e7: Option<i32>,
    pub longitude_e7: Option<i32>,
    pub device_id: String<DEVICE_ID_CAPACITY>,
    pub firmware_version: String<FIRMWARE_VERSION_CAPACITY>,
}

impl RecordingMetadata {
    pub const fn new(started_utc: UtcTimestamp) -> Self {
        Self {
            started_utc,
            latitude_e7: None,
            longitude_e7: None,
            device_id: String::new(),
            firmware_version: String::new(),
        }
    }
}

pub trait MetadataSource {
    /// Return `None` until the metadata required to start a recording is valid.
    fn snapshot(&self) -> Option<RecordingMetadata>;
}

/// Minimal metadata source which gates recording on valid UTC.
pub struct TimeMetadataSource<'a, const WATCHERS: usize, const ANCHOR_DEPTH: usize> {
    time: &'a TimeResources<WATCHERS, ANCHOR_DEPTH>,
}

impl<'a, const WATCHERS: usize, const ANCHOR_DEPTH: usize>
    TimeMetadataSource<'a, WATCHERS, ANCHOR_DEPTH>
{
    pub const fn new(time: &'a TimeResources<WATCHERS, ANCHOR_DEPTH>) -> Self {
        Self { time }
    }
}

impl<const WATCHERS: usize, const ANCHOR_DEPTH: usize> MetadataSource
    for TimeMetadataSource<'_, WATCHERS, ANCHOR_DEPTH>
{
    fn snapshot(&self) -> Option<RecordingMetadata> {
        self.time.current_utc().ok().map(RecordingMetadata::new)
    }
}
