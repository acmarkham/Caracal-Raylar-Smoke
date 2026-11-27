//! Explicit Evo capability/budget overrides; no fabricated U5 peripherals.
pub const EPOCH_US: i64 = 1_800_000_000;
pub const LOG_RESERVE_BYTES: u64 = 100_000_000;
pub const LOG_BUDGET_BYTES: u64 = 1_000_000_000;
pub const AUDIO_FILE_BYTES: u64 = crate::FILE_BUDGET
    + if cfg!(feature = "mono") {
        0
    } else {
        // Allow a whole extra second in the UTC cutoff bound for both files.
        // Admission separately rounds for both companions' final clusters.
        crate::SAMPLE_RATE as u64 * 4
            + 32 * (crate::SAMPLE_RATE as u64).div_ceil(crate::FLAC_BLOCK as u64)
            + 4096
            + 61 * 548
            + 92
            + (61 * 16_000u64).div_ceil(crate::SPATIAL_BLOCK as u64)
                * (64 + 3 * (10 + (crate::SPATIAL_BLOCK as u64 - 1) * 12).div_ceil(8))
    };
pub const GPS_OFF_SECONDS: u64 = if cfg!(feature = "bench-gps-cycle") {
    30
} else {
    1800
};
pub const GPS_ON_SECONDS: u64 = 60;

#[inline]
pub fn gps(receiver: crate::gnss_profile::Receiver) -> raylar_drivers::gps::GpsConfig {
    use embassy_time::Duration;
    use raylar_drivers::gps::{
        ublox, GpsConfig, GpsModuleCommands, PhaseQualifiedShutdownConfig, PpsTimingSource,
    };
    GpsConfig {
        gps_on_time: Duration::from_secs(GPS_ON_SECONDS),
        gps_off_time: Duration::from_secs(GPS_OFF_SECONDS),
        wait_for_frequency_calibration_lock: true,
        phase_qualified_shutdown: Some(PhaseQualifiedShutdownConfig {
            maximum_on_time: Duration::from_secs(180),
            minimum_clean_pps_time: Duration::from_secs(120),
            residual_threshold_us: 250,
            uncertainty_threshold_us: 500,
            consecutive_anchors: 5,
        }),
        pps_timing_source: PpsTimingSource::ChainedTimerCapture,
        module_commands: GpsModuleCommands {
            standby: Some(receiver.standby()),
            wake: Some(&ublox::WAKE),
            hot_start: None,
            warm_start: None,
            cold_start: None,
        },
        ..Default::default()
    }
}
