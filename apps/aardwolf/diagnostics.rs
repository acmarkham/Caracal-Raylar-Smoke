use crate::{common, LOCATION};
use core::sync::atomic::{AtomicBool, AtomicU32, Ordering};
use defmt::unwrap;
use embassy_time::{Duration, Instant, Ticker};
use raylar_drivers::gps::OperatingState;
use raylar_drivers::{sensor_acc::Lis2hh12, sensor_mag::Lis2mdl};
use raylar_logging_service::{info as log_info, warn as log_warn, LoggerHandle};
use raylar_time_service::UtcStatus;

type Log = LoggerHandle<'static, 384, 32>;
const TELEMETRY_INTERVAL: Duration = Duration::from_secs(60);

static CPU_IDLE_OPEN: AtomicBool = AtomicBool::new(false);
static CPU_IDLE_START_TICKS: AtomicU32 = AtomicU32::new(0);
static CPU_IDLE_TICKS: AtomicU32 = AtomicU32::new(0);

// Match Integration 002's thread-executor idle measurement. Interrupt work
// before the executor wakes is included in idle time; this measures executor
// load rather than attributing individual interrupt cycles.
#[unsafe(export_name = "_embassy_trace_poll_start")]
fn embassy_trace_poll_start(_executor_id: u32) {
    if CPU_IDLE_OPEN.swap(false, Ordering::AcqRel) {
        let now = Instant::now().as_ticks() as u32;
        let start = CPU_IDLE_START_TICKS.load(Ordering::Relaxed);
        CPU_IDLE_TICKS.fetch_add(now.wrapping_sub(start), Ordering::Relaxed);
    }
}

#[unsafe(export_name = "_embassy_trace_executor_idle")]
fn embassy_trace_executor_idle(_executor_id: u32) {
    CPU_IDLE_START_TICKS.store(Instant::now().as_ticks() as u32, Ordering::Relaxed);
    CPU_IDLE_OPEN.store(true, Ordering::Release);
}

#[unsafe(export_name = "_embassy_trace_task_new")]
fn embassy_trace_task_new(_executor_id: u32, _task_id: u32) {}
#[unsafe(export_name = "_embassy_trace_task_end")]
fn embassy_trace_task_end(_executor_id: u32, _task_id: u32) {}
#[unsafe(export_name = "_embassy_trace_task_exec_begin")]
fn embassy_trace_task_exec_begin(_executor_id: u32, _task_id: u32) {}
#[unsafe(export_name = "_embassy_trace_task_exec_end")]
fn embassy_trace_task_exec_end(_executor_id: u32, _task_id: u32) {}
#[unsafe(export_name = "_embassy_trace_task_ready_begin")]
fn embassy_trace_task_ready_begin(_executor_id: u32, _task_id: u32) {}

#[embassy_executor::task]
pub async fn cpu_task(log: Log) -> ! {
    // Discard boot time and report only complete minute windows.
    CPU_IDLE_TICKS.store(0, Ordering::Relaxed);
    let mut previous_ticks = Instant::now().as_ticks() as u32;
    let mut ticker = Ticker::every(TELEMETRY_INTERVAL);
    loop {
        ticker.next().await;
        let now = Instant::now().as_ticks() as u32;
        let elapsed_ticks = now.wrapping_sub(previous_ticks);
        let idle_ticks = CPU_IDLE_TICKS.swap(0, Ordering::AcqRel).min(elapsed_ticks);
        let active_ticks = elapsed_ticks - idle_ticks;
        let usage_permille = u64::from(active_ticks) * 1_000 / u64::from(elapsed_ticks.max(1));
        let _ = log_info!(
            log,
            "cpu usage={}.{}% active_ms={} idle_ms={} window_ms={} cadence_s=60",
            usage_permille / 10,
            usage_permille % 10,
            Duration::from_ticks(u64::from(active_ticks)).as_millis(),
            Duration::from_ticks(u64::from(idle_ticks)).as_millis(),
            Duration::from_ticks(u64::from(elapsed_ticks)).as_millis()
        );
        previous_ticks = now;
    }
}

#[embassy_executor::task]
pub async fn sensors_task(bus: crate::power::SharedI2c, log: Log) -> ! {
    let mut accelerometer = None;
    let mut magnetometer = None;
    let mut ticker = Ticker::every(TELEMETRY_INTERVAL);
    loop {
        // Failed initialization is retried on the next minute without
        // preventing the other sensor or the rest of the app from running.
        if accelerometer.is_none() {
            match Lis2hh12::new(bus).and_then(|mut driver| {
                driver.turn_on()?;
                Ok(driver)
            }) {
                Ok(driver) => accelerometer = Some(driver),
                Err(error) => {
                    let _ = log_warn!(log, "accelerometer startup error={:?}", error);
                }
            }
        }
        if magnetometer.is_none() {
            match Lis2mdl::new(bus).and_then(|mut driver| {
                driver.turn_on()?;
                Ok(driver)
            }) {
                Ok(driver) => magnetometer = Some(driver),
                Err(error) => {
                    let _ = log_warn!(log, "magnetometer startup error={:?}", error);
                }
            }
        }
        // Both devices default to 10 Hz. Wait for fresh data before reading.
        ticker.next().await;
        if let Some(driver) = accelerometer.as_mut() {
            match driver.read_acceleration() {
                Ok(value) => {
                    let _ = log_info!(
                        log,
                        "acceleration x_mg={} y_mg={} z_mg={}",
                        value.x_mg,
                        value.y_mg,
                        value.z_mg
                    );
                }
                Err(error) => {
                    let _ = log_warn!(log, "accelerometer read error={:?}", error);
                }
            }
            match driver.read_die_temperature() {
                Ok(value) => {
                    let _ = log_info!(
                        log,
                        "die_temperature origin=Accelerometer milli_celsius={}",
                        value.milli_celsius
                    );
                }
                Err(error) => {
                    let _ = log_warn!(
                        log,
                        "die_temperature origin=Accelerometer error={:?}",
                        error
                    );
                }
            }
        }
        if let Some(driver) = magnetometer.as_mut() {
            match driver.read_magnetic_field() {
                Ok(value) => {
                    let _ = log_info!(
                        log,
                        "magnetic_field x_nt={} y_nt={} z_nt={}",
                        value.x_nanotesla,
                        value.y_nanotesla,
                        value.z_nanotesla
                    );
                }
                Err(error) => {
                    let _ = log_warn!(log, "magnetometer read error={:?}", error);
                }
            }
            match driver.read_die_temperature() {
                Ok(value) => {
                    let _ = log_info!(
                        log,
                        "die_temperature origin=Magnetometer milli_celsius={}",
                        value.milli_celsius
                    );
                }
                Err(error) => {
                    let _ = log_warn!(log, "die_temperature origin=Magnetometer error={:?}", error);
                }
            }
        }
    }
}

#[embassy_executor::task]
pub async fn gps_task(log: Log) -> ! {
    let mut changes = unwrap!(common::GPS_RESOURCES.stats_receiver());
    let mut last_state: Option<OperatingState> = None;
    let mut last_rail_powered = false;
    loop {
        let gps = changes.changed().await;
        if last_state != Some(gps.operating_state) || last_rail_powered != gps.powered {
            let _ = log_info!(log, "gps state={:?} rail_powered={} tracking={} calibration_complete={} fixes={} pps={} search_attempts={} reacq_attempts={} reacq_successes={} phase_shutdowns={} phase_timeouts={} ticks={}",
                gps.operating_state, gps.powered, gps.operating_state.is_tracking(),
                gps.initial_calibration_complete, gps.num_fixes, gps.num_pps_events,
                gps.num_search_attempts, gps.num_reacquisition_attempts,
                gps.num_reacquisition_successes, gps.num_phase_qualified_shutdowns,
                gps.num_phase_convergence_timeouts, Instant::now().as_ticks());
            last_state = Some(gps.operating_state);
            last_rail_powered = gps.powered;
        }
    }
}

#[embassy_executor::task]
pub async fn time_task(log: Log) -> ! {
    let mut changes = unwrap!(common::TIME_RESOURCES.state_receiver());
    let mut last_status = UtcStatus::Invalid;
    let mut last_lock = false;
    let mut last_calibrated_ppb = 0i64;
    let mut last_tracking_updates = 0u32;
    let mut last_warning = false;
    let mut last_rejected = 0u32;
    let mut last_log = Instant::from_ticks(0);
    loop {
        let state = changes.changed().await;
        let now = Instant::now();
        if state.utc_status != last_status
            || state.frequency_calibration_locked != last_lock
            || state.calibrated_frequency_error_ppb != last_calibrated_ppb
            || state.frequency_tracking_updates != last_tracking_updates
            || state.holdover_warning != last_warning
            || state.rejected_anchors != last_rejected
            || now.saturating_duration_since(last_log) >= Duration::from_secs(60)
        {
            let _ = log_info!(log, "time status={:?} source={:?} uncertainty_us={} calibrated_ppb={} observed_ppb={:?} tracking_updates={} phase_slew_ppb={} locked={} samples={} holdover_s={} warning={} accepted={} rejected={} last_residual_us={:?} last_pps_ticks={:?}",
                state.utc_status, state.active_time_source, state.uncertainty_us,
                state.calibrated_frequency_error_ppb, state.last_frequency_observation_ppb,
                state.frequency_tracking_updates, state.phase_slew_ppb,
                state.frequency_calibration_locked, state.frequency_calibration_samples,
                state.holdover_duration.as_secs(), state.holdover_warning,
                state.accepted_anchors, state.rejected_anchors, state.last_anchor_residual_us,
                state.last_anchor_system_time.map(|v| v.as_ticks()));
            last_status = state.utc_status;
            last_lock = state.frequency_calibration_locked;
            last_calibrated_ppb = state.calibrated_frequency_error_ppb;
            last_tracking_updates = state.frequency_tracking_updates;
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
