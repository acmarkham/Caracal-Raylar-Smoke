use core::fmt::Write;

use heapless::String;
use raylar_time_service::{TimeResources, UtcTimestamp};

pub const DEVICE_ID_CAPACITY: usize = 32;
pub const FIRMWARE_VERSION_CAPACITY: usize = 32;
pub const IDENTIFIER_CAPACITY: usize = 32;
pub const FIRMWARE_HASH_CAPACITY: usize = 64;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RecordingMetadata {
    pub started_utc: UtcTimestamp,
    pub latitude_e7: Option<i32>,
    pub longitude_e7: Option<i32>,
    pub device_id: String<DEVICE_ID_CAPACITY>,
    pub firmware_version: String<FIRMWARE_VERSION_CAPACITY>,
    pub node_id: Option<u32>,
    pub card_id: Option<String<IDENTIFIER_CAPACITY>>,
    pub firmware_hash: String<FIRMWARE_HASH_CAPACITY>,
    pub boot_id: Option<u32>,
    pub started_system_ticks: Option<u64>,
    pub last_gps_pps_utc: Option<UtcTimestamp>,
    pub calibration_ppb: Option<i64>,
    pub utc_status: u8,
    pub uncertainty_us: u64,
    pub calibration_locked: bool,
    pub last_pps_system_ticks: Option<u64>,
}

impl RecordingMetadata {
    pub const fn new(started_utc: UtcTimestamp) -> Self {
        Self {
            started_utc,
            latitude_e7: None,
            longitude_e7: None,
            device_id: String::new(),
            firmware_version: String::new(),
            node_id: None,
            card_id: None,
            firmware_hash: String::new(),
            boot_id: None,
            started_system_ticks: None,
            last_gps_pps_utc: None,
            calibration_ppb: None,
            utc_status: 0,
            uncertainty_us: u64::MAX,
            calibration_locked: false,
            last_pps_system_ticks: None,
        }
    }
}

pub trait MetadataSource {
    /// Return `None` until the metadata required to start a recording is valid.
    fn snapshot(&self) -> Option<RecordingMetadata>;
    fn system_ticks_at(&self, _utc: UtcTimestamp) -> Option<u64> { None }
}

/// Minimal metadata source which gates recording on valid UTC.
pub struct TimeMetadataSource<'a, const WATCHERS: usize, const ANCHOR_DEPTH: usize> {
    time: &'a TimeResources<WATCHERS, ANCHOR_DEPTH>,
    node_id: Option<u32>,
    boot_id: Option<u32>,
    card_id: Option<String<IDENTIFIER_CAPACITY>>,
    firmware_hash: String<FIRMWARE_HASH_CAPACITY>,
    location: Option<(i32, i32)>,
    location_source: Option<fn() -> Option<(i32, i32)>>,
}

impl<'a, const WATCHERS: usize, const ANCHOR_DEPTH: usize>
    TimeMetadataSource<'a, WATCHERS, ANCHOR_DEPTH>
{
    pub const fn new(time: &'a TimeResources<WATCHERS, ANCHOR_DEPTH>) -> Self {
        Self {
            time,
            node_id: None,
            boot_id: None,
            card_id: None,
            firmware_hash: String::new(),
            location: None,
            location_source: None,
        }
    }

    pub fn with_node_id(mut self, node_id: u32) -> Self {
        self.node_id = Some(node_id);
        self
    }

    pub fn with_boot_id(mut self, boot_id: u32) -> Self {
        self.boot_id = Some(boot_id);
        self
    }

    pub fn with_card_id(mut self, card_id: &str) -> Self {
        let _ = self.card_id.get_or_insert_with(String::new).push_str(card_id);
        self
    }

    pub fn with_firmware_hash(mut self, hash: &str) -> Self {
        let _ = self.firmware_hash.push_str(hash);
        self
    }

    pub fn with_location_e7(mut self, latitude: i32, longitude: i32) -> Self {
        self.location = Some((latitude, longitude));
        self
    }

    pub fn with_location_source(mut self, source: fn() -> Option<(i32, i32)>) -> Self {
        self.location_source = Some(source);
        self
    }

    /// Populate node, SD card and firmware fields using the platform identity
    /// snapshots. Unknown values remain explicitly unavailable in the WAV.
    pub fn with_traceability(
        mut self,
        node_id: Option<u32>,
        card: Option<raylar_drivers::storage::StorageDeviceIdentity>,
        firmware_hash: Option<&str>,
        boot_id: Option<u32>,
    ) -> Self {
        self.node_id = node_id;
        self.boot_id = boot_id;
        if let Some(card) = card {
            let id = self.card_id.get_or_insert_with(String::new);
            let _ = write!(id, "{:02x}-{:08x}", card.manufacturer_id, card.serial_number);
        }
        if let Some(hash) = firmware_hash {
            let _ = self.firmware_hash.push_str(hash);
        }
        self
    }
}

impl<const WATCHERS: usize, const ANCHOR_DEPTH: usize> MetadataSource
    for TimeMetadataSource<'_, WATCHERS, ANCHOR_DEPTH>
{
    fn system_ticks_at(&self, utc: UtcTimestamp) -> Option<u64> {
        self.time.time_state().utc_to_system_holdover(utc).ok().map(|instant| instant.as_ticks())
    }

    fn snapshot(&self) -> Option<RecordingMetadata> {
        let state = self.time.time_state();
        let now = embassy_time::Instant::now();
        let mut metadata = RecordingMetadata::new(state.system_to_utc_holdover(now).ok()?);
        metadata.started_system_ticks = Some(embassy_time::Instant::now().as_ticks());
        metadata.node_id = self.node_id;
        metadata.boot_id = self.boot_id;
        metadata.card_id = self.card_id.clone();
        metadata.firmware_hash = self.firmware_hash.clone();
        if let Some((latitude, longitude)) = self.location.or_else(|| self.location_source.and_then(|source| source())) {
            metadata.latitude_e7 = Some(latitude);
            metadata.longitude_e7 = Some(longitude);
        }
        metadata.calibration_ppb = Some(state.calibrated_frequency_error_ppb);
        metadata.utc_status = match state.utc_status { raylar_time_service::UtcStatus::Invalid => 0, raylar_time_service::UtcStatus::Synchronized => 1, raylar_time_service::UtcStatus::Degraded => 2 };
        metadata.uncertainty_us = state.uncertainty_us;
        metadata.calibration_locked = state.frequency_calibration_locked;
        metadata.last_pps_system_ticks = state.last_anchor_system_time.map(|v| v.as_ticks());
        metadata.last_gps_pps_utc = state
            .last_anchor_utc
            .filter(|_| matches!(state.active_time_source, raylar_time_service::TimeSource::GpsPps));
        Some(metadata)
    }
}

