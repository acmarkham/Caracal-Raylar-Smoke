//! Checked-in interoperability and accelerated schedule configuration.

use embassy_time::Duration;
use raylar_radio_service::link::{ChannelProfile, CodingRate, ProfileId, SpreadingFactor};
use raylar_radio_service::{
    Epoch, EpochConfig, LinkError, NodeId, RadioServiceConfig, Rendezvous, RendezvousPurpose,
    ScheduleError, ScheduleVersion,
};
use raylar_time_service::UtcTimestamp;

pub const CONFIGURATION_ID: u16 = 0x0004;
pub const NETWORK_ID: u32 = 0x4954_0004;
pub const SCHEDULE_VERSION: ScheduleVersion = ScheduleVersion(3);
pub const BASE_STATION_CAPABILITY: u16 = 1 << 0;

pub const EPOCH_DURATION: Duration = Duration::from_secs(60);
pub const ACTIVE_WINDOW: Duration = Duration::from_secs(40);
pub const SUBWINDOW_DURATION: Duration = Duration::from_secs(20);
pub const HEARTBEAT_OFFSET: Duration = Duration::from_secs(20);
pub const SLOT_DURATION: Duration = Duration::from_secs(1);
pub const SLOTS_PER_SUBWINDOW: u32 = 20;
pub const TX_RESERVATION: Duration = Duration::from_millis(750);

pub const EXPECTED_REMOTE_UNCERTAINTY: Duration = Duration::from_millis(20);
pub const SCHEDULING_UNCERTAINTY: Duration = Duration::from_millis(10);
pub const PROPAGATION_ALLOWANCE: Duration = Duration::from_millis(1);
pub const ENGINEERING_MARGIN: Duration = Duration::from_millis(30);
pub const NARROW_RENDEZVOUS_THRESHOLD: Duration = Duration::from_millis(100);
pub const MAXIMUM_TX_UNCERTAINTY: Duration = Duration::from_secs(2);
pub const PREPARATION_GUARD: Duration = Duration::from_millis(100);
pub const NEIGHBOUR_EXPIRY: Duration = Duration::from_secs(3 * 60);
pub const PERIODIC_SCAN_EPOCHS: u64 = 5;
pub const STARTUP_SCAN_EPOCHS: u8 = 2;

#[cfg(feature = "channel-868")]
pub const FREQUENCY_HZ: u32 = 868_000_000;
#[cfg(feature = "channel-915")]
pub const FREQUENCY_HZ: u32 = 915_000_000;
#[cfg(feature = "channel-2445")]
pub const FREQUENCY_HZ: u32 = 2_445_000_000;

#[cfg(any(feature = "channel-868", feature = "channel-915"))]
pub const TX_POWER_DBM: i8 = 14;
#[cfg(feature = "channel-2445")]
pub const TX_POWER_DBM: i8 = 10;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TestConfigError {
    InvalidSchedule,
    InvalidIdentifiers,
    InvalidProfile,
}

pub const fn epoch_config() -> EpochConfig {
    EpochConfig {
        origin_utc_micros: 0,
        epoch_duration: EPOCH_DURATION,
        broadcast_window: ACTIVE_WINDOW,
        slot_duration: SLOT_DURATION,
    }
}

pub const fn service_config() -> RadioServiceConfig {
    RadioServiceConfig {
        network_id: NETWORK_ID,
        schedule_version: SCHEDULE_VERSION.0,
        epoch: epoch_config(),
        heartbeat_repetitions: 1,
        heartbeat_minimum_separation_slots: 0,
        expected_remote_uncertainty: EXPECTED_REMOTE_UNCERTAINTY,
        scheduling_uncertainty: SCHEDULING_UNCERTAINTY,
        propagation_allowance: PROPAGATION_ALLOWANCE,
        engineering_margin: ENGINEERING_MARGIN,
        maximum_scheduled_uncertainty: NARROW_RENDEZVOUS_THRESHOLD,
        preparation_guard: PREPARATION_GUARD,
        neighbour_expiry: NEIGHBOUR_EXPIRY,
    }
}

pub fn profile() -> Result<ChannelProfile, LinkError> {
    ChannelProfile::lora(
        ProfileId(CONFIGURATION_ID as u8),
        FREQUENCY_HZ,
        SpreadingFactor::Sf7,
        125_000,
        CodingRate::Cr4_5,
        TX_POWER_DBM,
        0x12,
    )
}

pub fn validate() -> Result<(), TestConfigError> {
    let service = service_config();
    service
        .validate()
        .map_err(|_| TestConfigError::InvalidSchedule)?;
    if NETWORK_ID == 0 || CONFIGURATION_ID == 0 || SCHEDULE_VERSION.0 == 0 {
        return Err(TestConfigError::InvalidIdentifiers);
    }
    if ACTIVE_WINDOW > EPOCH_DURATION
        || HEARTBEAT_OFFSET < SUBWINDOW_DURATION
        || HEARTBEAT_OFFSET + SUBWINDOW_DURATION > ACTIVE_WINDOW
        || SUBWINDOW_DURATION.as_micros() % SLOT_DURATION.as_micros() != 0
        || SUBWINDOW_DURATION.as_micros() / SLOT_DURATION.as_micros()
            != u64::from(SLOTS_PER_SUBWINDOW)
    {
        return Err(TestConfigError::InvalidSchedule);
    }
    profile().map_err(|_| TestConfigError::InvalidProfile)?;
    Ok(())
}

pub fn derived_slot(
    node: NodeId,
    epoch: Epoch,
    purpose: RendezvousPurpose,
) -> Result<u32, ScheduleError> {
    Rendezvous::new(NETWORK_ID, SCHEDULE_VERSION.0)
        .permuted_slot::<{ SLOTS_PER_SUBWINDOW as usize }>(purpose, node, epoch, 0)
}

pub fn absolute_slot(
    node: NodeId,
    epoch: Epoch,
    purpose: RendezvousPurpose,
) -> Result<u32, ScheduleError> {
    let slot = derived_slot(node, epoch, purpose)?;
    match purpose {
        RendezvousPurpose::Presence => Ok(slot),
        RendezvousPurpose::Heartbeat => Ok(SLOTS_PER_SUBWINDOW + slot),
        _ => Err(ScheduleError::InvalidSchedule),
    }
}

pub fn slot_time(
    node: NodeId,
    epoch: Epoch,
    purpose: RendezvousPurpose,
) -> Result<UtcTimestamp, ScheduleError> {
    epoch_config().slot_time(epoch, absolute_slot(node, epoch, purpose)?)
}

#[cfg(test)]
mod tests {
    use super::*;
    use raylar_radio_service::{EpochPosition, RendezvousPurpose};

    #[test]
    fn accelerated_epoch_and_subwindow_boundaries_are_exact() {
        validate().unwrap();
        let epoch = epoch_config();
        assert_eq!(epoch.broadcast_slot_count().unwrap(), 40);
        for (micros, expected_epoch, expected_offset, in_window) in [
            (0, 0, 0, true),
            (19_999_999, 0, 19_999_999, true),
            (20_000_000, 0, 20_000_000, true),
            (39_999_999, 0, 39_999_999, true),
            (40_000_000, 0, 40_000_000, false),
            (59_999_999, 0, 59_999_999, false),
            (60_000_000, 1, 0, true),
        ] {
            let position = epoch.position(UtcTimestamp::from_micros(micros)).unwrap();
            assert_eq!(
                position,
                EpochPosition {
                    epoch: Epoch(expected_epoch),
                    offset: Duration::from_micros(expected_offset),
                    slot: (expected_offset / 1_000_000) as u32,
                    in_broadcast_window: in_window,
                }
            );
        }
    }

    #[test]
    fn fixed_presence_and_heartbeat_vectors_use_distinct_subwindows() {
        let node = NodeId(0xAABB_CCDD);
        let epoch = Epoch(12_345);
        assert_eq!(
            derived_slot(node, epoch, RendezvousPurpose::Presence),
            Ok(11)
        );
        assert_eq!(
            derived_slot(node, epoch, RendezvousPurpose::Heartbeat),
            Ok(0)
        );
        assert_eq!(
            absolute_slot(node, epoch, RendezvousPurpose::Presence),
            Ok(11)
        );
        assert_eq!(
            absolute_slot(node, epoch, RendezvousPurpose::Heartbeat),
            Ok(20)
        );
    }

    #[test]
    fn observed_campaign_pair_has_distinct_heartbeat_slots() {
        let base_station = NodeId(0x381f_e484);
        let peer = NodeId(3_423_884_720);
        for epoch in 29_853_200..29_853_203 {
            let epoch = Epoch(epoch);
            assert_ne!(
                absolute_slot(base_station, epoch, RendezvousPurpose::Heartbeat),
                absolute_slot(peer, epoch, RendezvousPurpose::Heartbeat),
            );
        }
    }

    #[test]
    fn every_node_and_purpose_visits_all_slots_without_lockstep() {
        let nodes = [
            NodeId(0x381f_e484),
            NodeId(0xcc14_55b0),
            NodeId(0xAABB_CCDD),
        ];
        for node in nodes {
            for purpose in [RendezvousPurpose::Presence, RendezvousPurpose::Heartbeat] {
                let mut seen = [false; SLOTS_PER_SUBWINDOW as usize];
                for epoch in 0..u64::from(SLOTS_PER_SUBWINDOW) {
                    let slot = derived_slot(node, Epoch(epoch), purpose).unwrap() as usize;
                    assert!(!seen[slot], "node={node:?} purpose={purpose:?} slot={slot}");
                    seen[slot] = true;
                }
                assert!(seen.into_iter().all(|visited| visited));
            }
        }

        let first: [u32; 20] = core::array::from_fn(|epoch| {
            derived_slot(nodes[0], Epoch(epoch as u64), RendezvousPurpose::Presence).unwrap()
        });
        let second: [u32; 20] = core::array::from_fn(|epoch| {
            derived_slot(nodes[1], Epoch(epoch as u64), RendezvousPurpose::Presence).unwrap()
        });
        assert_ne!(first, second);
    }
}
