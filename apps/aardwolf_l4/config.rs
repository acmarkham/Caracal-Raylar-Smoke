//! Explicit Evo capability/budget overrides; no fabricated U5 peripherals.
pub const EPOCH_US: i64 = 1_800_000_000;
pub const LOG_RESERVE_BYTES: u64 = 100_000_000;
pub const LOG_BUDGET_BYTES: u64 = 1_000_000_000;
pub const AUDIO_FILE_BYTES: u64 = crate::FILE_BUDGET;
pub const GPS_OFF_SECONDS: u64 = if cfg!(feature = "bench-gps-cycle") {
    30
} else {
    1800
};
pub const GPS_ON_SECONDS: u64 = 60;

#[inline]
pub fn gps() -> raylar_drivers::gps::GpsConfig {
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
            standby: Some(if cfg!(feature = "gps-m8") {
                &M8_STANDBY
            } else {
                &ublox::STANDBY
            }),
            wake: Some(&ublox::WAKE),
            hot_start: None,
            warm_start: None,
            cold_start: None,
        },
        ..Default::default()
    }
}

const M8_STANDBY: [u8; 16] = raylar_drivers::gps::ublox::packet(2, 0x41, &[0, 0, 0, 0, 2, 0, 0, 0]);
