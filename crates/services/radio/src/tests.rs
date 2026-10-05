use embassy_time::{Duration, Instant};
use raylar_time_service::{TimeState, UtcStatus, UtcTimestamp};

use super::*;
use crate::link::{
    BandMask, ChannelProfile, CodingRate, LinkConstraints, LinkEstimator, LinkPurpose, LinkRequest,
    LinkTarget, PassiveLinkState, ProfileId, SpreadingFactor, StaticLinkEstimator,
};

fn profile(id: u8, frequency_hz: u32) -> ChannelProfile {
    ChannelProfile::lora(
        ProfileId(id),
        frequency_hz,
        SpreadingFactor::Sf9,
        125_000,
        CodingRate::Cr4_5,
        if frequency_hz >= 2_400_000_000 {
            13
        } else {
            14
        },
        0x12,
    )
    .unwrap()
}

fn epoch_config() -> EpochConfig {
    EpochConfig {
        origin_utc_micros: 1_000_000,
        epoch_duration: Duration::from_secs(300),
        broadcast_window: Duration::from_secs(60),
        slot_duration: Duration::from_secs(5),
    }
}

#[test]
fn frame_round_trip_uses_compact_broadcast_header() {
    let header = FrameHeader {
        frame_type: FrameType::Heartbeat,
        source: NodeId(0x1020_3040),
        boot_id: BootId(0x5060_7080),
        sequence: Sequence(0x90A0),
        destination: None,
    };
    let payload = [1, 2, 3];
    let frame = FrameBuffer::encode(header, &payload).unwrap();

    assert_eq!(frame.len(), FRAME_HEADER_LEN + payload.len());
    assert_eq!(frame.as_slice()[0], 0x11);
    assert_eq!(frame.as_slice()[1], 0);
    let decoded = FrameHeader::decode(frame.as_slice()).unwrap();
    assert_eq!(decoded.header, header);
    assert_eq!(decoded.payload, payload);
}

#[test]
fn directed_data_has_only_four_bytes_of_optional_addressing() {
    let header = FrameHeader {
        frame_type: FrameType::Data,
        source: NodeId(1),
        boot_id: BootId(2),
        sequence: Sequence(3),
        destination: Some(NodeId(4)),
    };
    let frame = FrameBuffer::encode(header, &[]).unwrap();
    assert_eq!(frame.len(), FRAME_HEADER_LEN + 4);
    assert_eq!(
        FrameHeader::decode(frame.as_slice()).unwrap().header,
        header
    );
}

#[test]
fn frame_codec_rejects_unknown_and_malformed_input_without_panicking() {
    assert_eq!(FrameHeader::decode(&[0; 3]), Err(FrameError::Truncated));

    let mut frame = [0u8; FRAME_HEADER_LEN];
    frame[0] = 0x21;
    assert_eq!(
        FrameHeader::decode(&frame),
        Err(FrameError::UnsupportedVersion(2))
    );
    frame[0] = 0x1F;
    assert_eq!(
        FrameHeader::decode(&frame),
        Err(FrameError::UnknownFrameType(15))
    );
    frame[0] = 0x11;
    frame[1] = 0x80;
    assert_eq!(FrameHeader::decode(&frame), Err(FrameError::Malformed));

    let mut directed = [0u8; FRAME_HEADER_LEN + 4];
    directed[0] = 0x13;
    directed[1] = 1;
    for length in 0..directed.len() {
        assert_eq!(
            FrameHeader::decode(&directed[..length]),
            Err(FrameError::Truncated)
        );
    }
}

#[test]
fn frame_codec_accepts_maximum_packet_and_rejects_one_byte_more() {
    let header = FrameHeader {
        frame_type: FrameType::Data,
        source: NodeId(1),
        boot_id: BootId(2),
        sequence: Sequence(3),
        destination: None,
    };
    let maximum = [0xA5; MAX_FRAME_LEN - FRAME_HEADER_LEN];
    assert_eq!(FrameBuffer::encode(header, &maximum).unwrap().len(), 255);
    let too_large = [0xA5; MAX_FRAME_LEN - FRAME_HEADER_LEN + 1];
    assert_eq!(
        FrameBuffer::encode(header, &too_large),
        Err(FrameError::FrameTooLarge)
    );
}

#[test]
fn boot_change_resets_volatile_sequence_and_duplicate_key_includes_boot() {
    let mut sequences = SequenceState::new(BootId(10));
    assert_eq!(sequences.take(), Sequence(0));
    assert_eq!(sequences.take(), Sequence(1));
    sequences.set_boot_id(BootId(11));
    assert_eq!(sequences.take(), Sequence(0));
    assert_ne!(
        DuplicateKey {
            node_id: NodeId(1),
            boot_id: BootId(10),
            sequence: Sequence(0),
        },
        DuplicateKey {
            node_id: NodeId(1),
            boot_id: BootId(11),
            sequence: Sequence(0),
        }
    );
}

#[test]
fn sequence_wrap_is_explicit() {
    let mut sequences = SequenceState::new(BootId(1));
    for _ in 0..=u16::MAX {
        sequences.take();
    }
    assert_eq!(sequences.take(), Sequence(0));
}

#[test]
fn epoch_boundaries_and_slots_are_exact() {
    let config = epoch_config();
    let origin = UtcTimestamp::from_micros(1_000_000);
    let at_boundary = UtcTimestamp::from_micros(301_000_000);
    assert_eq!(
        config.position(origin).unwrap(),
        EpochPosition {
            epoch: Epoch(0),
            offset: Duration::from_secs(0),
            slot: 0,
            in_broadcast_window: true,
        }
    );
    assert_eq!(config.position(at_boundary).unwrap().epoch, Epoch(1));
    assert_eq!(
        config.slot_time(Epoch(1), 3).unwrap().as_micros(),
        316_000_000
    );
    assert_eq!(
        config.position(UtcTimestamp::from_micros(999_999)),
        Err(ScheduleError::InvalidSchedule)
    );
}

#[test]
fn epoch_rejects_fractional_slot_windows() {
    let mut config = epoch_config();
    config.broadcast_window = Duration::from_secs(59);
    assert_eq!(config.validate(), Err(ScheduleError::InvalidSchedule));
}

#[test]
fn rendezvous_vectors_are_stable() {
    let rendezvous = Rendezvous::new(0x1122_3344, 1);
    assert_eq!(
        rendezvous
            .slot(
                RendezvousPurpose::Heartbeat,
                NodeId(0xAABB_CCDD),
                Epoch(12345),
                0,
                12,
            )
            .unwrap(),
        11
    );
    assert_eq!(
        rendezvous
            .slot(
                RendezvousPurpose::Presence,
                NodeId(0x0102_0304),
                Epoch(77),
                2,
                60,
            )
            .unwrap(),
        22
    );
}

#[test]
fn heartbeat_payload_round_trips_with_and_without_location() {
    let rich = Heartbeat {
        location: Some(CompactLocation {
            latitude_e7: 515_012_345,
            longitude_e7: -1_234_567,
        }),
        location_age: Some(Duration::from_secs(17)),
        battery_soc: BatterySoc::new(Some(84)),
        charging_state: ChargingState::Solar,
        error_flags: ErrorFlags(0x0102),
        storage_usage: StorageUsage::new(Some(55)),
        gps_status: GpsStatus {
            utc_valid: true,
            fix_quality: FixQuality::Good,
            uncertainty: TimeUncertaintyClass::Under20Ms,
            holdover: false,
        },
    };
    let mut bytes = [0u8; 32];
    let len = rich.encode(&mut bytes).unwrap();
    assert_eq!(len, 17);
    assert_eq!(Heartbeat::decode(&bytes[..len]).unwrap(), rich);

    let mut protocol = HeartbeatProtocol::new(
        NodeId(9),
        BootId(10),
        ChannelProfile::phase_one_eu868_bootstrap(),
        RadioServiceConfig::default(),
    )
    .unwrap();
    let frame = protocol.encode_frame(&rich).unwrap();
    let (header, decoded) = Heartbeat::decode_frame(frame.as_slice()).unwrap();
    assert_eq!(header.source, NodeId(9));
    assert_eq!(decoded, rich);

    let minimal = Heartbeat {
        location: None,
        location_age: None,
        ..rich
    };
    let len = minimal.encode(&mut bytes).unwrap();
    assert_eq!(len, 7);
    assert_eq!(Heartbeat::decode(&bytes[..len]).unwrap(), minimal);
}

#[test]
fn heartbeat_slots_are_deterministic_and_respect_minimum_separation() {
    let config = RadioServiceConfig::default();
    let protocol = HeartbeatProtocol::new(
        NodeId(0xAABB_CCDD),
        BootId(123),
        ChannelProfile::phase_one_eu868_bootstrap(),
        config,
    )
    .unwrap();
    let slots = protocol
        .slots(Epoch(42), config.epoch.broadcast_slot_count().unwrap())
        .unwrap();
    assert_eq!(slots, protocol.slots(Epoch(42), 12).unwrap());
    assert_eq!(slots.len(), 2);
    let direct = slots[0].abs_diff(slots[1]);
    let circular = direct.min(12 - direct);
    assert!(circular >= u32::from(config.heartbeat_minimum_separation_slots));
    let next = protocol
        .next_opportunity(UtcTimestamp::from_micros(1_000_000), &epoch_config())
        .unwrap();
    assert!(next.as_micros() > 1_000_000);
}

#[test]
fn presence_payload_is_three_bytes() {
    let advert = PresenceAdvert {
        schedule_version: ScheduleVersion(1),
        capabilities: CapabilityFlags(0x1234),
    };
    let mut bytes = [0u8; 3];
    assert_eq!(advert.encode(&mut bytes).unwrap(), 3);
    assert_eq!(PresenceAdvert::decode(&bytes).unwrap(), advert);
}

#[test]
fn received_presence_refreshes_neighbour_and_passive_link_state() {
    let channel = ChannelProfile::phase_one_eu868_bootstrap();
    let mut protocol = PresenceProtocol::new(
        NodeId(77),
        BootId(88),
        channel.clone(),
        0x1234,
        ScheduleVersion(1),
    );
    let frame = protocol
        .encode_frame(PresenceAdvert {
            schedule_version: ScheduleVersion(1),
            capabilities: CapabilityFlags(0x0001),
        })
        .unwrap();
    let (header, decoded_advert) = PresenceAdvert::decode_frame(frame.as_slice()).unwrap();
    assert_eq!(header.source, NodeId(77));
    assert_eq!(decoded_advert.schedule_version, ScheduleVersion(1));
    assert!(
        protocol
            .next_opportunity(UtcTimestamp::from_micros(1_000_000), &epoch_config())
            .unwrap()
            .as_micros()
            > 1_000_000
    );
    let mut table = NeighbourTable::<2>::new(Duration::from_secs(10));
    let source = table
        .observe_presence_frame(
            frame.as_slice(),
            UtcTimestamp::from_micros(2_000_000),
            Instant::from_ticks(123),
            channel.id(),
            -180,
            Some(24),
        )
        .unwrap();
    let entry = table.get(source).unwrap();
    assert_eq!(source, NodeId(77));
    assert_eq!(entry.boot_id, BootId(88));
    assert_eq!(entry.last_rssi_dbm_x2, Some(-180));
    assert_eq!(entry.link_state.received_packets, 1);
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
fn neighbour_table_updates_boot_and_replaces_oldest_at_capacity() {
    let mut table = NeighbourTable::<2>::new(Duration::from_secs(10));
    table.observe(
        neighbour(1, 1, 1_000_000),
        UtcTimestamp::from_micros(1_000_000),
    );
    table.observe(
        neighbour(2, 1, 2_000_000),
        UtcTimestamp::from_micros(2_000_000),
    );
    table.observe(
        neighbour(1, 2, 3_000_000),
        UtcTimestamp::from_micros(3_000_000),
    );
    assert_eq!(table.get(NodeId(1)).unwrap().boot_id, BootId(2));

    table.observe(
        neighbour(3, 1, 4_000_000),
        UtcTimestamp::from_micros(4_000_000),
    );
    assert!(table.get(NodeId(2)).is_none());
    assert!(table.get(NodeId(1)).is_some());
    assert!(table.get(NodeId(3)).is_some());
}

#[test]
fn neighbour_expiry_and_prediction_are_derived() {
    let mut table = NeighbourTable::<2>::new(Duration::from_secs(10));
    table.observe(
        neighbour(7, 1, 1_000_000),
        UtcTimestamp::from_micros(1_000_000),
    );
    let next = table
        .next_presence_time(
            NodeId(7),
            UtcTimestamp::from_micros(1_000_000),
            0x1234,
            &epoch_config(),
        )
        .unwrap();
    assert!(next.as_micros() > 1_000_000);
    assert_eq!(table.expire(UtcTimestamp::from_micros(11_000_000)), 1);
}

#[test]
fn static_link_estimator_selects_profiles_and_honours_band_constraints() {
    let estimator = StaticLinkEstimator::new(profile(1, 868_100_000), profile(2, 2_445_000_000));
    let broadcast = LinkRequest {
        target: LinkTarget::Broadcast,
        purpose: LinkPurpose::BootstrapBroadcast,
        constraints: LinkConstraints::default(),
        system_slot: None,
    };
    assert_eq!(
        estimator.select_profile(&broadcast).unwrap().id(),
        ProfileId(1)
    );

    let impossible = LinkRequest {
        constraints: LinkConstraints {
            allowed_bands: BandMask::GHZ_2_4,
            maximum_airtime: None,
        },
        ..broadcast
    };
    assert_eq!(
        estimator.select_profile(&impossible),
        Err(LinkError::NoAcceptableProfile)
    );
}

#[test]
fn built_in_common_profile_is_interoperable_and_static() {
    let estimator = StaticLinkEstimator::phase_one_eu868();
    let request = LinkRequest {
        target: LinkTarget::Broadcast,
        purpose: LinkPurpose::BootstrapBroadcast,
        constraints: LinkConstraints::default(),
        system_slot: None,
    };
    let selected = estimator.select_profile(&request).unwrap();
    assert_eq!(selected.id(), crate::link::PHASE_ONE_BOOTSTRAP_PROFILE_ID);
    assert_eq!(selected.frequency_hz(), 868_000_000);
}

#[test]
fn scheduler_accepts_non_overlapping_and_rejects_equal_priority_conflicts() {
    let now = Instant::from_ticks(1_000);
    let mut scheduler = Scheduler::<4>::new(Duration::from_ticks(10));
    let first = Reservation {
        job_id: JobId(1),
        start: Instant::from_ticks(2_000),
        end: Instant::from_ticks(3_000),
        priority: RadioPriority::Control,
    };
    scheduler.reserve(first, now).unwrap();
    scheduler
        .reserve(
            Reservation {
                job_id: JobId(2),
                start: Instant::from_ticks(3_000),
                end: Instant::from_ticks(4_000),
                priority: RadioPriority::Control,
            },
            now,
        )
        .unwrap();
    assert_eq!(
        scheduler.reserve(
            Reservation {
                job_id: JobId(3),
                start: Instant::from_ticks(2_500),
                end: Instant::from_ticks(3_500),
                priority: RadioPriority::Control,
            },
            now,
        ),
        Err(ScheduleError::Conflict)
    );
}

#[test]
fn higher_priority_reservation_evicts_lower_priority() {
    let now = Instant::from_ticks(1_000);
    let mut scheduler = Scheduler::<4>::new(Duration::from_ticks(10));
    scheduler
        .reserve(
            Reservation {
                job_id: JobId(1),
                start: Instant::from_ticks(2_000),
                end: Instant::from_ticks(3_000),
                priority: RadioPriority::BestEffort,
            },
            now,
        )
        .unwrap();
    let outcome = scheduler
        .reserve(
            Reservation {
                job_id: JobId(2),
                start: Instant::from_ticks(2_500),
                end: Instant::from_ticks(3_500),
                priority: RadioPriority::CriticalControl,
            },
            now,
        )
        .unwrap();
    assert_eq!(outcome.evicted.as_slice(), &[JobId(1)]);
}

#[test]
fn scheduler_detects_missed_preparation_and_capacity() {
    let now = Instant::from_ticks(1_000);
    let mut scheduler = Scheduler::<1>::new(Duration::from_ticks(100));
    assert_eq!(
        scheduler.reserve(
            Reservation {
                job_id: JobId(1),
                start: Instant::from_ticks(1_050),
                end: Instant::from_ticks(2_000),
                priority: RadioPriority::Control,
            },
            now,
        ),
        Err(ScheduleError::MissedSlot)
    );
    scheduler
        .reserve(
            Reservation {
                job_id: JobId(2),
                start: Instant::from_ticks(2_000),
                end: Instant::from_ticks(3_000),
                priority: RadioPriority::Control,
            },
            now,
        )
        .unwrap();
    assert_eq!(
        scheduler.reserve(
            Reservation {
                job_id: JobId(3),
                start: Instant::from_ticks(4_000),
                end: Instant::from_ticks(5_000),
                priority: RadioPriority::Control,
            },
            now,
        ),
        Err(ScheduleError::QueueFull)
    );
}

#[test]
fn uncertainty_guard_sums_all_sources() {
    assert_eq!(
        guard_interval(
            Duration::from_millis(10),
            Duration::from_millis(20),
            Duration::from_millis(3),
            Duration::from_millis(1),
            Duration::from_millis(6),
        ),
        Duration::from_millis(40)
    );
}

#[test]
fn utc_receive_window_consumes_time_service_uncertainty() {
    let mut time = TimeState::invalid();
    time.utc_status = UtcStatus::Synchronized;
    time.reference_system_time = Instant::from_ticks(10_000_000);
    time.reference_utc = UtcTimestamp::from_micros(100_000_000);
    time.uncertainty_us = 10_000;

    let config = RadioServiceConfig::default();
    let nominal_start = time
        .utc_to_system(UtcTimestamp::from_micros(110_000_000))
        .unwrap();
    let nominal_end = time
        .utc_to_system(UtcTimestamp::from_micros(115_000_000))
        .unwrap();
    let expected_guard = guard_interval(
        Duration::from_micros(time.uncertainty_us),
        config.expected_remote_uncertainty,
        config.scheduling_uncertainty,
        config.propagation_allowance,
        config.engineering_margin,
    );
    let job = RadioRxJob::guarded_utc_window(
        UtcTimestamp::from_micros(110_000_000),
        UtcTimestamp::from_micros(115_000_000),
        &time,
        ChannelProfile::phase_one_eu868_bootstrap(),
        RadioPriority::Control,
        RxPurpose::Presence,
        &config,
    )
    .unwrap();
    assert_eq!(
        job.start.as_ticks(),
        nominal_start.as_ticks() - expected_guard.as_ticks()
    );
    assert_eq!(
        job.end.as_ticks(),
        nominal_end.as_ticks() + expected_guard.as_ticks()
    );
}

#[test]
fn neighbour_count_is_published_through_latest_state() {
    let resources = RadioResources::<1, 1, 1>::new();
    resources.set_neighbour_count(3);
    assert_eq!(resources.state().stats.neighbour_count, 3);
}

#[test]
fn handle_enqueues_explicit_best_effort_cancellation() {
    let resources = RadioResources::<1, 1, 1>::new();
    let handle = resources.handle();
    assert_eq!(handle.try_cancel(JobId(42)), Ok(()));
    assert_eq!(handle.try_cancel(JobId(43)), Err(ScheduleError::QueueFull));
}
