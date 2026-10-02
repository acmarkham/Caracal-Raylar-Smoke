use embassy_time::{Duration, Instant};
use heapless::Vec;
use raylar_location_service::LocationState;
use raylar_time_service::{TimeState, UtcStatus, UtcTimestamp};

use crate::link::ChannelProfile;
use crate::{
    BootId, Epoch, EpochConfig, FrameError, FrameHeader, FrameType, NodeId, RadioServiceConfig,
    RadioTxJob, Rendezvous, RendezvousPurpose, ScheduleError, SequenceState,
};

const FLAG_LOCATION: u8 = 1 << 0;
const BASE_LEN: usize = 7;
const LOCATION_LEN: usize = 10;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct BatterySoc(pub Option<u8>);

impl BatterySoc {
    pub const fn new(percent: Option<u8>) -> Self {
        Self(match percent {
            Some(value) if value <= 100 => Some(value),
            _ => None,
        })
    }

    const fn wire(self) -> u8 {
        match self.0 {
            Some(value) if value <= 100 => value,
            _ => u8::MAX,
        }
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
#[repr(u8)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ChargingState {
    None = 0,
    Solar = 1,
    Usb = 2,
    External = 3,
    #[default]
    Unknown = 4,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct ErrorFlags(pub u16);

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct StorageUsage(pub Option<u8>);

impl StorageUsage {
    pub const fn new(percent: Option<u8>) -> Self {
        Self(match percent {
            Some(value) if value <= 100 => Some(value),
            _ => None,
        })
    }

    const fn wire(self) -> u8 {
        match self.0 {
            Some(value) if value <= 100 => value,
            _ => u8::MAX,
        }
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
#[repr(u8)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FixQuality {
    #[default]
    None = 0,
    Coarse = 1,
    Good = 2,
    Excellent = 3,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
#[repr(u8)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum TimeUncertaintyClass {
    Under1Ms = 0,
    Under20Ms = 1,
    Under1S = 2,
    #[default]
    Over1S = 3,
}

impl TimeUncertaintyClass {
    pub const fn from_micros(micros: u64) -> Self {
        match micros {
            0..=999 => Self::Under1Ms,
            1_000..=19_999 => Self::Under20Ms,
            20_000..=999_999 => Self::Under1S,
            _ => Self::Over1S,
        }
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct GpsStatus {
    pub utc_valid: bool,
    pub fix_quality: FixQuality,
    pub uncertainty: TimeUncertaintyClass,
    pub holdover: bool,
}

impl GpsStatus {
    pub fn from_states(time: TimeState, location: LocationState) -> Self {
        let fix_quality = if !location.valid {
            FixQuality::None
        } else {
            match location.hdop_centi {
                Some(0..=100) => FixQuality::Excellent,
                Some(101..=250) => FixQuality::Good,
                _ => FixQuality::Coarse,
            }
        };
        Self {
            utc_valid: time.utc_status != UtcStatus::Invalid,
            fix_quality,
            uncertainty: TimeUncertaintyClass::from_micros(time.uncertainty_us),
            holdover: time.holdover_duration.as_ticks() != 0,
        }
    }

    const fn wire(self) -> u8 {
        self.utc_valid as u8
            | ((self.fix_quality as u8) << 1)
            | ((self.uncertainty as u8) << 3)
            | ((self.holdover as u8) << 5)
    }

    fn from_wire(value: u8) -> Result<Self, FrameError> {
        if value & 0b1100_0000 != 0 {
            return Err(FrameError::Malformed);
        }
        let fix_quality = match (value >> 1) & 0x03 {
            0 => FixQuality::None,
            1 => FixQuality::Coarse,
            2 => FixQuality::Good,
            3 => FixQuality::Excellent,
            _ => FixQuality::Excellent,
        };
        let uncertainty = match (value >> 3) & 0x03 {
            0 => TimeUncertaintyClass::Under1Ms,
            1 => TimeUncertaintyClass::Under20Ms,
            2 => TimeUncertaintyClass::Under1S,
            3 => TimeUncertaintyClass::Over1S,
            _ => TimeUncertaintyClass::Over1S,
        };
        Ok(Self {
            utc_valid: value & 1 != 0,
            fix_quality,
            uncertainty,
            holdover: value & (1 << 5) != 0,
        })
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct CompactLocation {
    pub latitude_e7: i32,
    pub longitude_e7: i32,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Heartbeat {
    pub location: Option<CompactLocation>,
    pub location_age: Option<Duration>,
    pub battery_soc: BatterySoc,
    pub charging_state: ChargingState,
    pub error_flags: ErrorFlags,
    pub storage_usage: StorageUsage,
    pub gps_status: GpsStatus,
}

impl Heartbeat {
    pub fn from_service_states(
        time: TimeState,
        location: LocationState,
        now: Instant,
        battery_soc: BatterySoc,
        charging_state: ChargingState,
        error_flags: ErrorFlags,
        storage_usage: StorageUsage,
    ) -> Self {
        let compact = location.valid.then_some(CompactLocation {
            latitude_e7: location.latitude.degrees_e7,
            longitude_e7: location.longitude.degrees_e7,
        });
        let age = compact.map(|_| now.saturating_duration_since(location.last_fix_system_time));
        Self {
            location: compact,
            location_age: age,
            battery_soc,
            charging_state,
            error_flags,
            storage_usage,
            gps_status: GpsStatus::from_states(time, location),
        }
    }

    pub const fn encoded_len(&self) -> usize {
        BASE_LEN
            + if self.location.is_some() {
                LOCATION_LEN
            } else {
                0
            }
    }

    pub fn encode(&self, output: &mut [u8]) -> Result<usize, FrameError> {
        let len = self.encoded_len();
        if output.len() < len {
            return Err(FrameError::BufferTooSmall);
        }
        output[0] = u8::from(self.location.is_some()) * FLAG_LOCATION;
        output[1] = self.battery_soc.wire();
        output[2] = self.charging_state as u8;
        output[3..5].copy_from_slice(&self.error_flags.0.to_be_bytes());
        output[5] = self.storage_usage.wire();
        output[6] = self.gps_status.wire();
        if let Some(location) = self.location {
            output[7..11].copy_from_slice(&location.latitude_e7.to_be_bytes());
            output[11..15].copy_from_slice(&location.longitude_e7.to_be_bytes());
            let age_seconds = self
                .location_age
                .map(|age| age.as_secs().min(u64::from(u16::MAX)) as u16)
                .unwrap_or(u16::MAX);
            output[15..17].copy_from_slice(&age_seconds.to_be_bytes());
        }
        Ok(len)
    }

    pub fn decode(input: &[u8]) -> Result<Self, FrameError> {
        if input.len() < BASE_LEN {
            return Err(FrameError::Truncated);
        }
        let flags = input[0];
        if flags & !FLAG_LOCATION != 0 {
            return Err(FrameError::Malformed);
        }
        let expected = BASE_LEN + usize::from(flags & FLAG_LOCATION != 0) * LOCATION_LEN;
        if input.len() != expected {
            return Err(if input.len() < expected {
                FrameError::Truncated
            } else {
                FrameError::Malformed
            });
        }
        let charging_state = match input[2] {
            0 => ChargingState::None,
            1 => ChargingState::Solar,
            2 => ChargingState::Usb,
            3 => ChargingState::External,
            4 => ChargingState::Unknown,
            _ => return Err(FrameError::Malformed),
        };
        let battery_soc = BatterySoc::new((input[1] != u8::MAX).then_some(input[1]));
        let storage_usage = StorageUsage::new((input[5] != u8::MAX).then_some(input[5]));
        if input[1] != u8::MAX && battery_soc.0.is_none()
            || input[5] != u8::MAX && storage_usage.0.is_none()
        {
            return Err(FrameError::Malformed);
        }
        let (location, location_age) = if flags & FLAG_LOCATION != 0 {
            let age = u16::from_be_bytes([input[15], input[16]]);
            let latitude_e7 = i32::from_be_bytes([input[7], input[8], input[9], input[10]]);
            let longitude_e7 = i32::from_be_bytes([input[11], input[12], input[13], input[14]]);
            if !(-900_000_000..=900_000_000).contains(&latitude_e7)
                || !(-1_800_000_000..=1_800_000_000).contains(&longitude_e7)
            {
                return Err(FrameError::Malformed);
            }
            (
                Some(CompactLocation {
                    latitude_e7,
                    longitude_e7,
                }),
                (age != u16::MAX).then_some(Duration::from_secs(u64::from(age))),
            )
        } else {
            (None, None)
        };
        Ok(Self {
            location,
            location_age,
            battery_soc,
            charging_state,
            error_flags: ErrorFlags(u16::from_be_bytes([input[3], input[4]])),
            storage_usage,
            gps_status: GpsStatus::from_wire(input[6])?,
        })
    }

    pub fn decode_frame(frame: &[u8]) -> Result<(FrameHeader, Self), FrameError> {
        let decoded = FrameHeader::decode(frame)?;
        if decoded.header.frame_type != FrameType::Heartbeat || decoded.header.destination.is_some()
        {
            return Err(FrameError::Malformed);
        }
        Ok((decoded.header, Self::decode(decoded.payload)?))
    }
}

pub struct HeartbeatProtocol {
    node_id: NodeId,
    sequence: SequenceState,
    rendezvous: Rendezvous,
    profile: ChannelProfile,
    repetitions: u8,
    minimum_separation_slots: u16,
}

impl HeartbeatProtocol {
    pub fn new(
        node_id: NodeId,
        boot_id: BootId,
        profile: ChannelProfile,
        config: RadioServiceConfig,
    ) -> Result<Self, ScheduleError> {
        config
            .validate()
            .map_err(|_| ScheduleError::InvalidSchedule)?;
        Ok(Self {
            node_id,
            sequence: SequenceState::new(boot_id),
            rendezvous: Rendezvous::new(config.network_id, config.schedule_version),
            profile,
            repetitions: config.heartbeat_repetitions,
            minimum_separation_slots: config.heartbeat_minimum_separation_slots,
        })
    }

    pub fn slots(&self, epoch: Epoch, slot_count: u32) -> Result<Vec<u32, 8>, ScheduleError> {
        if usize::from(self.repetitions) > 8 {
            return Err(ScheduleError::InvalidSchedule);
        }
        let mut slots = Vec::<u32, 8>::new();
        for occurrence in 0..self.repetitions {
            let mut candidate = self.rendezvous.slot(
                RendezvousPurpose::Heartbeat,
                self.node_id,
                epoch,
                occurrence,
                slot_count,
            )?;
            for _ in 0..slot_count {
                if slots.iter().all(|other| {
                    circular_distance(candidate, *other, slot_count)
                        >= u32::from(self.minimum_separation_slots)
                }) {
                    break;
                }
                candidate = (candidate + 1) % slot_count;
            }
            if slots.iter().any(|other| {
                circular_distance(candidate, *other, slot_count)
                    < u32::from(self.minimum_separation_slots)
            }) {
                return Err(ScheduleError::InvalidSchedule);
            }
            slots
                .push(candidate)
                .map_err(|_| ScheduleError::InvalidSchedule)?;
        }
        slots.sort_unstable();
        Ok(slots)
    }

    pub fn next_opportunity(
        &self,
        after: UtcTimestamp,
        epoch_config: &EpochConfig,
    ) -> Result<UtcTimestamp, ScheduleError> {
        let position = epoch_config.position(after)?;
        let slot_count = epoch_config.broadcast_slot_count()?;
        for candidate_epoch in [position.epoch, Epoch(position.epoch.0.saturating_add(1))] {
            for slot in self.slots(candidate_epoch, slot_count)? {
                let candidate = epoch_config.slot_time(candidate_epoch, slot)?;
                if candidate.as_micros() > after.as_micros() {
                    return Ok(candidate);
                }
            }
        }
        Err(ScheduleError::InvalidSchedule)
    }

    pub fn encode_frame(
        &mut self,
        heartbeat: &Heartbeat,
    ) -> Result<crate::FrameBuffer, FrameError> {
        let mut payload = [0u8; 32];
        let payload_len = heartbeat.encode(&mut payload)?;
        let header = FrameHeader {
            frame_type: FrameType::Heartbeat,
            source: self.node_id,
            boot_id: self.sequence.boot_id(),
            sequence: self.sequence.take(),
            destination: None,
        };
        crate::FrameBuffer::encode(header, &payload[..payload_len])
    }

    pub fn tx_job(
        &mut self,
        heartbeat: &Heartbeat,
        slot_utc: UtcTimestamp,
        slot_duration: Duration,
        time: &TimeState,
        maximum_uncertainty: Duration,
    ) -> Result<RadioTxJob, ScheduleError> {
        if time.utc_status == UtcStatus::Invalid {
            return Err(ScheduleError::UtcUnavailable);
        }
        if time.uncertainty_us > maximum_uncertainty.as_micros() {
            return Err(ScheduleError::UtcUncertaintyTooHigh);
        }
        let start = time
            .utc_to_system(slot_utc)
            .map_err(|_| ScheduleError::UtcUnavailable)?;
        let frame = self
            .encode_frame(heartbeat)
            .map_err(|_| ScheduleError::InvalidSchedule)?;
        Ok(RadioTxJob {
            earliest: start,
            deadline: start + slot_duration,
            profile: self.profile.clone(),
            priority: crate::RadioPriority::Control,
            payload: frame,
        })
    }
}

fn circular_distance(a: u32, b: u32, modulus: u32) -> u32 {
    let direct = a.abs_diff(b);
    direct.min(modulus - direct)
}
