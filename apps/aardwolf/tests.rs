use crate::{config, policy::*};
use embassy_time::{Instant, TICK_HZ};
use raylar_radio_service::NodeId;
use raylar_time_service::{TimeError, TimeState, UtcStatus, UtcTimestamp};

#[test]
fn calibrated_utc_mapping_survives_quality_loss_without_changing_status() {
    let anchor = Instant::from_ticks(5 * TICK_HZ);
    let mut state = TimeState::invalid();
    assert_eq!(
        state.system_to_utc_holdover(anchor),
        Err(TimeError::NotValid)
    );
    state.reference_system_time = anchor;
    state.reference_utc = UtcTimestamp::new(1_700_000_000, 0).unwrap();
    state.last_anchor_system_time = Some(anchor);
    state.estimated_frequency_error_ppb = 1_000;
    let later = Instant::from_ticks(6 * TICK_HZ);
    let mapped = state.system_to_utc_holdover(later).unwrap();
    assert_eq!(mapped, UtcTimestamp::new(1_700_000_001, 1).unwrap());
    assert_eq!(state.utc_to_system_holdover(mapped).unwrap(), later);
    assert_eq!(state.system_to_utc(later), Err(TimeError::NotValid));
    assert_eq!(state.utc_status, UtcStatus::Invalid);
}

#[test]
fn hysteresis_latches_across_boundaries_missing_data_and_rebound() {
    let mut mode = Activity::default();
    for soc in [None, Some(10), Some(20)] {
        assert!(!mode.update(soc));
    }
    assert!(mode.update(Some(21)));
    for soc in [Some(20), Some(10), None] {
        assert!(!mode.update(soc));
    }
    assert!(mode.update(Some(9)));
    assert!(!mode.update(Some(20)));
    assert!(mode.update(Some(100)));
}

#[test]
fn each_profile_permutation_visits_every_legal_slot() {
    for profile in 0..12 {
        let count = 59 / config::slot_seconds(profile) as usize;
        let mut seen = [false; 59];
        for epoch in 0..count {
            let slot = config::slot(NodeId(0x12345678), epoch as u64, profile).unwrap() as usize;
            assert!(slot < count && !seen[slot]);
            seen[slot] = true;
            assert!((slot as u64 + 1) * config::slot_seconds(profile) <= 59);
        }
        assert!(config::profile(profile).is_ok());
        assert!(config::airtime_us(profile) + 100_000 < config::slot_seconds(profile) * 1_000_000);
    }
    assert_eq!(config::airtime_us(3), 1_449_984);
    assert_eq!(config::airtime_us(8), 864);
    assert_eq!(config::airtime_us(10), 45_000);
}

#[test]
fn reserve_accounts_for_cluster_rounding_and_pending_log() {
    assert!(!can_start_audio(103_840_512, 32768, 65536));
    assert!(can_start_audio(104_000_000, 32768, 65536));
    assert!(!can_start_audio(100_000_000, 32768, 0));
    assert_eq!(next_epoch(1_800_000_000), 3_600_000_000);
}
