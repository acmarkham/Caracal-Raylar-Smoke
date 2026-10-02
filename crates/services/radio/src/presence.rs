use embassy_time::Duration;
use raylar_time_service::{TimeState, UtcStatus, UtcTimestamp};

use crate::link::ChannelProfile;
use crate::{
    BootId, EpochConfig, FrameError, FrameHeader, FrameType, NodeId, RadioServiceConfig,
    SequenceState,
};
use crate::{Epoch, RadioPriority, RadioTxJob, Rendezvous, RendezvousPurpose, ScheduleError};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct ScheduleVersion(pub u8);

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct CapabilityFlags(pub u16);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PresenceAdvert {
    pub schedule_version: ScheduleVersion,
    pub capabilities: CapabilityFlags,
}

impl PresenceAdvert {
    pub const ENCODED_LEN: usize = 3;

    pub fn encode(self, output: &mut [u8]) -> Result<usize, FrameError> {
        if output.len() < Self::ENCODED_LEN {
            return Err(FrameError::BufferTooSmall);
        }
        output[0] = self.schedule_version.0;
        output[1..3].copy_from_slice(&self.capabilities.0.to_be_bytes());
        Ok(Self::ENCODED_LEN)
    }

    pub fn decode(input: &[u8]) -> Result<Self, FrameError> {
        if input.len() < Self::ENCODED_LEN {
            return Err(FrameError::Truncated);
        }
        if input.len() != Self::ENCODED_LEN {
            return Err(FrameError::Malformed);
        }
        Ok(Self {
            schedule_version: ScheduleVersion(input[0]),
            capabilities: CapabilityFlags(u16::from_be_bytes([input[1], input[2]])),
        })
    }

    pub fn decode_frame(frame: &[u8]) -> Result<(FrameHeader, Self), FrameError> {
        let decoded = FrameHeader::decode(frame)?;
        if decoded.header.frame_type != FrameType::Presence || decoded.header.destination.is_some()
        {
            return Err(FrameError::Malformed);
        }
        Ok((decoded.header, Self::decode(decoded.payload)?))
    }
}

pub struct PresenceProtocol {
    node_id: NodeId,
    sequence: SequenceState,
    profile: ChannelProfile,
    rendezvous: Rendezvous,
}

impl PresenceProtocol {
    pub const fn new(
        node_id: NodeId,
        boot_id: BootId,
        profile: ChannelProfile,
        network_id: u32,
        schedule_version: ScheduleVersion,
    ) -> Self {
        Self {
            node_id,
            sequence: SequenceState::new(boot_id),
            profile,
            rendezvous: Rendezvous::new(network_id, schedule_version.0),
        }
    }

    pub fn from_config(
        node_id: NodeId,
        boot_id: BootId,
        profile: ChannelProfile,
        config: &RadioServiceConfig,
    ) -> Self {
        Self::new(
            node_id,
            boot_id,
            profile,
            config.network_id,
            ScheduleVersion(config.schedule_version),
        )
    }

    pub fn profile(&self) -> &ChannelProfile {
        &self.profile
    }

    pub fn slot(&self, epoch: Epoch, slot_count: u32) -> Result<u32, ScheduleError> {
        self.rendezvous.slot(
            RendezvousPurpose::Presence,
            self.node_id,
            epoch,
            0,
            slot_count,
        )
    }

    pub fn next_opportunity(
        &self,
        after: UtcTimestamp,
        epoch_config: &EpochConfig,
    ) -> Result<UtcTimestamp, ScheduleError> {
        let position = epoch_config.position(after)?;
        let slot_count = epoch_config.broadcast_slot_count()?;
        for candidate_epoch in [position.epoch, Epoch(position.epoch.0.saturating_add(1))] {
            let slot = self.slot(candidate_epoch, slot_count)?;
            let candidate = epoch_config.slot_time(candidate_epoch, slot)?;
            if candidate.as_micros() > after.as_micros() {
                return Ok(candidate);
            }
        }
        Err(ScheduleError::InvalidSchedule)
    }

    pub fn encode_frame(
        &mut self,
        advert: PresenceAdvert,
    ) -> Result<crate::FrameBuffer, FrameError> {
        let mut payload = [0u8; PresenceAdvert::ENCODED_LEN];
        advert.encode(&mut payload)?;
        crate::FrameBuffer::encode(
            FrameHeader {
                frame_type: FrameType::Presence,
                source: self.node_id,
                boot_id: self.sequence.boot_id(),
                sequence: self.sequence.take(),
                destination: None,
            },
            &payload,
        )
    }

    pub fn tx_job(
        &mut self,
        advert: PresenceAdvert,
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
            .encode_frame(advert)
            .map_err(|_| ScheduleError::InvalidSchedule)?;
        Ok(RadioTxJob {
            earliest: start,
            deadline: start + slot_duration,
            profile: self.profile.clone(),
            priority: RadioPriority::Control,
            payload: frame,
        })
    }
}
