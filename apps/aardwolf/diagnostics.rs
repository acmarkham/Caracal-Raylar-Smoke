use crate::{common, LOCATION};
use defmt::unwrap;
use embassy_time::{Duration, Instant};
use raylar_logging_service::{info as log_info, LoggerHandle};
use raylar_time_service::UtcStatus;

type Log = LoggerHandle<'static, 384, 32>;

#[embassy_executor::task]
pub async fn time_task(log: Log) -> ! {
    let mut changes = unwrap!(common::TIME_RESOURCES.state_receiver());
    let mut last_status = UtcStatus::Invalid;
    let mut last_lock = false;
    let mut last_warning = false;
    let mut last_rejected = 0u32;
    let mut last_log = Instant::from_ticks(0);
    loop {
        let state = changes.changed().await;
        let now = Instant::now();
        if state.utc_status != last_status
            || state.frequency_calibration_locked != last_lock
            || state.holdover_warning != last_warning
            || state.rejected_anchors != last_rejected
            || now.saturating_duration_since(last_log) >= Duration::from_secs(60)
        {
            let _ = log_info!(log, "time status={:?} source={:?} uncertainty_us={} calibrated_ppb={} phase_slew_ppb={} locked={} samples={} holdover_s={} warning={} accepted={} rejected={} last_residual_us={:?} last_pps_ticks={:?}",
                state.utc_status, state.active_time_source, state.uncertainty_us,
                state.calibrated_frequency_error_ppb, state.phase_slew_ppb,
                state.frequency_calibration_locked, state.frequency_calibration_samples,
                state.holdover_duration.as_secs(), state.holdover_warning,
                state.accepted_anchors, state.rejected_anchors, state.last_anchor_residual_us,
                state.last_anchor_system_time.map(|v| v.as_ticks()));
            last_status = state.utc_status;
            last_lock = state.frequency_calibration_locked;
            last_warning = state.holdover_warning;
            last_rejected = state.rejected_anchors;
            last_log = now;
        }
    }
}

#[embassy_executor::task]
pub async fn location_task(log: Log) -> ! {
    let mut changes = unwrap!(LOCATION.state_receiver());
    let mut was_valid = false;
    let mut last_log = Instant::from_ticks(0);
    loop {
        let location = changes.changed().await;
        let now = Instant::now();
        if location.valid != was_valid
            || now.saturating_duration_since(last_log) >= Duration::from_secs(60)
        {
            let _ = log_info!(log, "location valid={} lat_e7={} lon_e7={} hdop={:?} satellites={:?} uncertainty_m={:?} fix_ticks={} fixes_seen={}",
                location.valid, location.latitude.degrees_e7, location.longitude.degrees_e7,
                location.hdop_centi, location.satellites, location.uncertainty_meters,
                location.last_fix_system_time.as_ticks(), location.total_fix_count_seen);
            was_valid = location.valid;
            last_log = now;
        }
    }
}
