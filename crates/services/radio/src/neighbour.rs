use embassy_time::{Duration, Instant};
use heapless::Vec;
use raylar_time_service::UtcTimestamp;

use crate::link::{LinkObservation, LinkOutcome, PassiveLinkState, ProfileId};
use crate::{
    BootId, CompactLocation, EpochConfig, FrameError, FrameHeader, FrameType, NodeId,
    PresenceAdvert, Rendezvous, RendezvousPurpose, ScheduleError, ScheduleVersion,
};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct NeighbourEntry {
    pub node_id: NodeId,
    pub boot_id: BootId,
    pub last_seen_utc: UtcTimestamp,
    pub location: Option<CompactLocation>,
    pub location_uncertainty_meters: Option<u32>,
    pub schedule_version: ScheduleVersion,
    pub last_rssi_dbm_x2: Option<i16>,
    pub last_snr_db_x4: Option<i16>,
    pub link_state: PassiveLinkState,
}

pub struct NeighbourTable<const CAPACITY: usize> {
    entries: Vec<NeighbourEntry, CAPACITY>,
    expiry: Duration,
}

impl<const CAPACITY: usize> NeighbourTable<CAPACITY> {
    pub const fn new(expiry: Duration) -> Self {
        Self {
            entries: Vec::new(),
            expiry,
        }
    }

    pub fn len(&self) -> usize {
        self.entries.len()
    }

    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    pub fn iter(&self) -> impl Iterator<Item = &NeighbourEntry> {
        self.entries.iter()
    }

    pub fn get(&self, node_id: NodeId) -> Option<&NeighbourEntry> {
        self.entries.iter().find(|entry| entry.node_id == node_id)
    }

    /// Insert or refresh a peer. When full, an expired entry is replaced
    /// first; otherwise the least recently heard entry is replaced.
    pub fn observe(&mut self, entry: NeighbourEntry, now: UtcTimestamp) {
        if let Some(existing) = self
            .entries
            .iter_mut()
            .find(|existing| existing.node_id == entry.node_id)
        {
            if existing.boot_id != entry.boot_id {
                *existing = entry;
            } else {
                existing.last_seen_utc = entry.last_seen_utc;
                existing.location = entry.location.or(existing.location);
                existing.location_uncertainty_meters = entry
                    .location_uncertainty_meters
                    .or(existing.location_uncertainty_meters);
                existing.schedule_version = entry.schedule_version;
                existing.last_rssi_dbm_x2 = entry.last_rssi_dbm_x2.or(existing.last_rssi_dbm_x2);
                existing.last_snr_db_x4 = entry.last_snr_db_x4.or(existing.last_snr_db_x4);
                if let Some(observation) = entry.link_state.last_observation {
                    existing.link_state.observe(observation);
                }
            }
            return;
        }

        if !self.entries.is_full() {
            let _ = self.entries.push(entry);
            return;
        }

        let expiry_us = self.expiry.as_micros().min(i64::MAX as u64) as i64;
        let replace = self
            .entries
            .iter()
            .enumerate()
            .filter(|(_, candidate)| {
                now.as_micros()
                    .saturating_sub(candidate.last_seen_utc.as_micros())
                    >= expiry_us
            })
            .min_by_key(|(_, candidate)| candidate.last_seen_utc)
            .or_else(|| {
                self.entries
                    .iter()
                    .enumerate()
                    .min_by_key(|(_, candidate)| candidate.last_seen_utc)
            })
            .map(|(index, _)| index);
        if let Some(index) = replace {
            self.entries[index] = entry;
        }
    }

    pub fn observe_link(&mut self, observation: LinkObservation) -> bool {
        let Some(entry) = self
            .entries
            .iter_mut()
            .find(|entry| entry.node_id == observation.peer)
        else {
            return false;
        };
        entry.last_rssi_dbm_x2 = observation.rssi_dbm_x2;
        entry.last_snr_db_x4 = observation.snr_db_x4;
        entry.link_state.observe(observation);
        true
    }

    /// Decode a received presence frame, refresh the peer's soft state, and
    /// retain its passive PHY observation.
    #[allow(clippy::too_many_arguments)]
    pub fn observe_presence_frame(
        &mut self,
        frame: &[u8],
        received_utc: UtcTimestamp,
        received_at: Instant,
        profile: ProfileId,
        rssi_dbm_x2: i16,
        snr_db_x4: Option<i16>,
    ) -> Result<NodeId, FrameError> {
        let decoded = FrameHeader::decode(frame)?;
        if decoded.header.frame_type != FrameType::Presence || decoded.header.destination.is_some()
        {
            return Err(FrameError::Malformed);
        }
        let advert = PresenceAdvert::decode(decoded.payload)?;
        let observation = LinkObservation {
            peer: decoded.header.source,
            profile,
            rssi_dbm_x2: Some(rssi_dbm_x2),
            snr_db_x4,
            outcome: LinkOutcome::Received,
            observed_at: received_at,
        };
        let mut link_state = PassiveLinkState::default();
        link_state.observe(observation);
        self.observe(
            NeighbourEntry {
                node_id: decoded.header.source,
                boot_id: decoded.header.boot_id,
                last_seen_utc: received_utc,
                location: None,
                location_uncertainty_meters: None,
                schedule_version: advert.schedule_version,
                last_rssi_dbm_x2: Some(rssi_dbm_x2),
                last_snr_db_x4: snr_db_x4,
                link_state,
            },
            received_utc,
        );
        Ok(decoded.header.source)
    }

    pub fn expire(&mut self, now: UtcTimestamp) -> usize {
        let before = self.entries.len();
        let expiry_us = self.expiry.as_micros().min(i64::MAX as u64) as i64;
        self.entries.retain(|entry| {
            now.as_micros()
                .saturating_sub(entry.last_seen_utc.as_micros())
                < expiry_us
        });
        before - self.entries.len()
    }

    pub fn next_presence_time(
        &self,
        peer: NodeId,
        after: UtcTimestamp,
        network_id: u32,
        epoch: &EpochConfig,
    ) -> Result<UtcTimestamp, ScheduleError> {
        let entry = self.get(peer).ok_or(ScheduleError::InvalidSchedule)?;
        let position = epoch.position(after)?;
        let slots = epoch.broadcast_slot_count()?;
        let rendezvous = Rendezvous::new(network_id, entry.schedule_version.0);
        for candidate_epoch in [
            position.epoch,
            crate::Epoch(position.epoch.0.saturating_add(1)),
        ] {
            let slot =
                rendezvous.slot(RendezvousPurpose::Presence, peer, candidate_epoch, 0, slots)?;
            let candidate = epoch.slot_time(candidate_epoch, slot)?;
            if candidate.as_micros() > after.as_micros() {
                return Ok(candidate);
            }
        }
        Err(ScheduleError::InvalidSchedule)
    }
}
