//! Pure, host-testable Integration 004 role and receive-window policy.

use embassy_time::Duration;
use heapless::Vec;
use raylar_radio_service::{guard_interval, Epoch, NodeId, RendezvousPurpose, ScheduleError};
use raylar_time_service::{UtcStatus, UtcTimestamp};

use crate::config;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Role {
    BaseStation,
    #[default]
    Node,
}

impl Role {
    pub const fn is_base_station(self) -> bool {
        matches!(self, Self::BaseStation)
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct RoleLatch(Option<Role>);

impl RoleLatch {
    pub const fn new() -> Self {
        Self(None)
    }

    pub fn sample(&mut self, user_pressed: bool) -> Role {
        *self.0.get_or_insert(if user_pressed {
            Role::BaseStation
        } else {
            Role::Node
        })
    }

    pub const fn role(self) -> Option<Role> {
        self.0
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct UtcWindow {
    pub start_us: i64,
    pub end_us: i64,
}

impl UtcWindow {
    pub fn new(start: UtcTimestamp, end: UtcTimestamp) -> Result<Self, ScheduleError> {
        if end.as_micros() <= start.as_micros() {
            return Err(ScheduleError::InvalidSchedule);
        }
        Ok(Self {
            start_us: start.as_micros(),
            end_us: end.as_micros(),
        })
    }

    pub const fn start(self) -> UtcTimestamp {
        UtcTimestamp::from_micros(self.start_us)
    }

    pub const fn end(self) -> UtcTimestamp {
        UtcTimestamp::from_micros(self.end_us)
    }

    pub const fn contains(self, utc: UtcTimestamp) -> bool {
        utc.as_micros() >= self.start_us && utc.as_micros() < self.end_us
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WindowClass {
    Predicted,
    Scan,
    Outside,
    Unverifiable,
    SchedulerConflict,
}

pub fn should_scan(
    completed_epochs: u8,
    epoch: Epoch,
    base_station_known: bool,
    force_scan: bool,
) -> bool {
    completed_epochs < config::STARTUP_SCAN_EPOCHS
        || !base_station_known
        || force_scan
        || epoch.0.is_multiple_of(config::PERIODIC_SCAN_EPOCHS)
}

pub fn rendezvous_guard(local_uncertainty: Duration) -> Duration {
    guard_interval(
        local_uncertainty,
        config::EXPECTED_REMOTE_UNCERTAINTY,
        config::SCHEDULING_UNCERTAINTY,
        config::PROPAGATION_ALLOWANCE,
        config::ENGINEERING_MARGIN,
    )
}

pub fn predicted_window(
    node: NodeId,
    epoch: Epoch,
    purpose: RendezvousPurpose,
    guard: Duration,
) -> Result<UtcWindow, ScheduleError> {
    let slot = config::slot_time(node, epoch, purpose)?.as_micros();
    let guard = i64::try_from(guard.as_micros()).map_err(|_| ScheduleError::InvalidSchedule)?;
    let duration = i64::try_from(config::SLOT_DURATION.as_micros())
        .map_err(|_| ScheduleError::InvalidSchedule)?;
    Ok(UtcWindow {
        start_us: slot.saturating_sub(guard),
        end_us: slot.saturating_add(duration).saturating_add(guard),
    })
}

pub fn classify_observation(
    source: NodeId,
    epoch: Epoch,
    purpose: RendezvousPurpose,
    observed: Option<UtcTimestamp>,
    utc_status: UtcStatus,
    uncertainty: Duration,
    was_scan: bool,
    scheduler_conflict: bool,
) -> WindowClass {
    if scheduler_conflict {
        return WindowClass::SchedulerConflict;
    }
    let Some(observed) = observed else {
        return WindowClass::Unverifiable;
    };
    if utc_status == UtcStatus::Invalid || uncertainty > config::NARROW_RENDEZVOUS_THRESHOLD {
        return WindowClass::Unverifiable;
    }
    if was_scan {
        return WindowClass::Scan;
    }
    match predicted_window(source, epoch, purpose, rendezvous_guard(uncertainty)) {
        Ok(window) if window.contains(observed) => WindowClass::Predicted,
        Ok(_) => WindowClass::Outside,
        Err(_) => WindowClass::Unverifiable,
    }
}

pub fn merge_windows<const INPUT: usize, const OUTPUT: usize>(
    windows: &Vec<UtcWindow, INPUT>,
) -> Result<Vec<UtcWindow, OUTPUT>, ScheduleError> {
    let mut sorted = windows.clone();
    sorted.sort_unstable();
    let mut merged = Vec::<UtcWindow, OUTPUT>::new();
    for window in sorted {
        if window.end_us <= window.start_us {
            return Err(ScheduleError::InvalidSchedule);
        }
        if let Some(last) = merged.last_mut() {
            if window.start_us <= last.end_us {
                last.end_us = last.end_us.max(window.end_us);
                continue;
            }
        }
        merged.push(window).map_err(|_| ScheduleError::QueueFull)?;
    }
    Ok(merged)
}

/// Remove local transmit reservations from receive windows. The returned
/// windows are ordered, non-overlapping, and contain no zero-length entries.
pub fn subtract_windows<const WINDOWS: usize, const BLOCKS: usize, const OUTPUT: usize>(
    windows: &Vec<UtcWindow, WINDOWS>,
    blocks: &Vec<UtcWindow, BLOCKS>,
) -> Result<Vec<UtcWindow, OUTPUT>, ScheduleError> {
    let windows = merge_windows::<WINDOWS, WINDOWS>(windows)?;
    let blocks = merge_windows::<BLOCKS, BLOCKS>(blocks)?;
    let mut result = Vec::<UtcWindow, OUTPUT>::new();
    for window in windows {
        let mut cursor = window.start_us;
        for block in &blocks {
            if block.end_us <= cursor || block.start_us >= window.end_us {
                continue;
            }
            if block.start_us > cursor {
                result
                    .push(UtcWindow {
                        start_us: cursor,
                        end_us: block.start_us.min(window.end_us),
                    })
                    .map_err(|_| ScheduleError::QueueFull)?;
            }
            cursor = cursor.max(block.end_us);
            if cursor >= window.end_us {
                break;
            }
        }
        if cursor < window.end_us {
            result
                .push(UtcWindow {
                    start_us: cursor,
                    end_us: window.end_us,
                })
                .map_err(|_| ScheduleError::QueueFull)?;
        }
    }
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;
    use heapless::Vec;
    use raylar_radio_service::link::PassiveLinkState;
    use raylar_radio_service::{
        BatterySoc, BootId, CapabilityFlags, ChargingState, ErrorFlags, FrameError, GpsStatus,
        Heartbeat, HeartbeatProtocol, NeighbourEntry, NeighbourTable, PresenceAdvert,
        PresenceProtocol, ScheduleVersion, StorageUsage,
    };

    #[test]
    fn role_selection_latches_first_sample() {
        let mut base = RoleLatch::new();
        assert_eq!(base.sample(true), Role::BaseStation);
        assert_eq!(base.sample(false), Role::BaseStation);

        let mut node = RoleLatch::new();
        assert_eq!(node.sample(false), Role::Node);
        assert_eq!(node.sample(true), Role::Node);
    }

    #[test]
    fn scan_policy_covers_bootstrap_loss_recovery_and_periodic_discovery() {
        assert!(should_scan(0, Epoch(1), true, false));
        assert!(should_scan(2, Epoch(1), false, false));
        assert!(should_scan(2, Epoch(1), true, true));
        assert!(should_scan(2, Epoch(10), true, false));
        assert!(!should_scan(2, Epoch(11), true, false));
    }

    #[test]
    fn guard_uses_every_uncertainty_term() {
        assert_eq!(
            rendezvous_guard(Duration::from_millis(9)),
            Duration::from_millis(70)
        );
    }

    #[test]
    fn overlapping_peer_windows_are_merged() {
        let input = Vec::<UtcWindow, 6>::from_slice(&[
            UtcWindow {
                start_us: 10,
                end_us: 20,
            },
            UtcWindow {
                start_us: 30,
                end_us: 40,
            },
            UtcWindow {
                start_us: 18,
                end_us: 32,
            },
            UtcWindow {
                start_us: 50,
                end_us: 60,
            },
        ])
        .unwrap();
        let merged = merge_windows::<6, 6>(&input).unwrap();
        assert_eq!(
            merged.as_slice(),
            &[
                UtcWindow {
                    start_us: 10,
                    end_us: 40
                },
                UtcWindow {
                    start_us: 50,
                    end_us: 60
                },
            ]
        );
    }

    #[test]
    fn local_tx_slots_split_receive_windows_without_overlap() {
        let windows = Vec::<UtcWindow, 1>::from_slice(&[UtcWindow {
            start_us: 0,
            end_us: 20,
        }])
        .unwrap();
        let blocks = Vec::<UtcWindow, 2>::from_slice(&[
            UtcWindow {
                start_us: 3,
                end_us: 4,
            },
            UtcWindow {
                start_us: 13,
                end_us: 14,
            },
        ])
        .unwrap();
        let split = subtract_windows::<1, 2, 5>(&windows, &blocks).unwrap();
        assert_eq!(
            split.as_slice(),
            &[
                UtcWindow {
                    start_us: 0,
                    end_us: 3
                },
                UtcWindow {
                    start_us: 4,
                    end_us: 13
                },
                UtcWindow {
                    start_us: 14,
                    end_us: 20
                },
            ]
        );
    }

    #[test]
    fn observations_are_scan_predicted_outside_or_unverifiable() {
        let source = NodeId(0x0102_0304);
        let epoch = Epoch(77);
        let predicted = config::slot_time(source, epoch, RendezvousPurpose::Presence).unwrap();
        assert_eq!(
            classify_observation(
                source,
                epoch,
                RendezvousPurpose::Presence,
                Some(predicted),
                UtcStatus::Synchronized,
                Duration::from_millis(1),
                false,
                false,
            ),
            WindowClass::Predicted
        );
        assert_eq!(
            classify_observation(
                source,
                epoch,
                RendezvousPurpose::Presence,
                Some(predicted),
                UtcStatus::Synchronized,
                Duration::from_millis(1),
                true,
                false,
            ),
            WindowClass::Scan
        );
        assert_eq!(
            classify_observation(
                source,
                epoch,
                RendezvousPurpose::Presence,
                Some(UtcTimestamp::from_micros(predicted.as_micros() + 2_000_000)),
                UtcStatus::Synchronized,
                Duration::from_millis(1),
                false,
                false,
            ),
            WindowClass::Outside
        );
        assert_eq!(
            classify_observation(
                source,
                epoch,
                RendezvousPurpose::Presence,
                None,
                UtcStatus::Invalid,
                Duration::from_secs(1),
                false,
                false,
            ),
            WindowClass::Unverifiable
        );
    }

    #[test]
    fn integration_frames_have_exact_sizes_and_base_capability() {
        let profile = config::profile().unwrap();
        let mut presence = PresenceProtocol::new(
            NodeId(1),
            BootId(2),
            profile.clone(),
            config::NETWORK_ID,
            config::SCHEDULE_VERSION,
        );
        let advert = PresenceAdvert {
            schedule_version: config::SCHEDULE_VERSION,
            capabilities: CapabilityFlags(config::BASE_STATION_CAPABILITY),
        };
        let presence_frame = presence.encode_frame(advert).unwrap();
        assert_eq!(presence_frame.len(), 15);
        assert_eq!(
            PresenceAdvert::decode_frame(presence_frame.as_slice())
                .unwrap()
                .1,
            advert
        );

        let heartbeat = Heartbeat {
            location: None,
            location_age: None,
            battery_soc: BatterySoc::new(None),
            charging_state: ChargingState::Unknown,
            error_flags: ErrorFlags(0),
            storage_usage: StorageUsage::new(None),
            gps_status: GpsStatus::default(),
        };
        let mut protocol =
            HeartbeatProtocol::new(NodeId(1), BootId(2), profile, config::service_config())
                .unwrap();
        let heartbeat_frame = protocol.encode_frame(&heartbeat).unwrap();
        assert_eq!(heartbeat_frame.len(), 19);
        assert_eq!(
            Heartbeat::decode_frame(heartbeat_frame.as_slice())
                .unwrap()
                .1,
            heartbeat
        );
    }

    fn neighbour(node: u32, boot: u32, seen_us: i64) -> NeighbourEntry {
        NeighbourEntry {
            node_id: NodeId(node),
            boot_id: BootId(boot),
            last_seen_utc: UtcTimestamp::from_micros(seen_us),
            location: None,
            location_uncertainty_meters: None,
            schedule_version: ScheduleVersion(1),
            last_rssi_dbm_x2: None,
            last_snr_db_x4: None,
            link_state: PassiveLinkState::default(),
        }
    }

    #[test]
    fn neighbour_state_is_bounded_and_boot_changes_replace_sessions() {
        let mut table = NeighbourTable::<2>::new(Duration::from_secs(10));
        table.observe(
            neighbour(1, 10, 1_000_000),
            UtcTimestamp::from_micros(1_000_000),
        );
        table.observe(
            neighbour(2, 20, 2_000_000),
            UtcTimestamp::from_micros(2_000_000),
        );
        table.observe(
            neighbour(1, 11, 3_000_000),
            UtcTimestamp::from_micros(3_000_000),
        );
        assert_eq!(table.len(), 2);
        assert_eq!(table.get(NodeId(1)).unwrap().boot_id, BootId(11));
        assert_eq!(table.expire(UtcTimestamp::from_micros(13_000_000)), 2);
    }

    #[test]
    fn malformed_and_unsupported_presence_are_rejected() {
        assert_eq!(PresenceAdvert::decode(&[1, 0]), Err(FrameError::Truncated));
        assert_eq!(
            PresenceAdvert::decode(&[1, 0, 1, 2]),
            Err(FrameError::Malformed)
        );
    }
}
