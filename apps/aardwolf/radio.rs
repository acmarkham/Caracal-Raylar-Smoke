use core::sync::atomic::{AtomicBool, AtomicU32, Ordering};
use defmt::{error, unwrap};
use embassy_executor::Spawner;
use embassy_stm32::{
    exti::ExtiInput,
    gpio::Output,
    mode::{Async, Blocking},
    spi::{mode::Master, Spi},
    time::mhz,
};
use embassy_time::{Duration, Instant, Timer};
use raylar_board_v1p0::EbyteRf;
use raylar_drivers::radio::{DriverTiming, ManualCsSpiDevice, RadioDriver};
use raylar_logging_service::{info as log_info, LoggerHandle};
use raylar_power_management_service::PowerSource;
use raylar_radio_service::heartbeat_v4::{HeartbeatV4, ENERGY_RECOVERED, LOGGING_IMPAIRED};
use raylar_radio_service::{
    FrameBuffer, GpsStatus, JobId, NodeId, RadioEvent, RadioHandle, RadioMode, RadioPriority,
    RadioResources, RadioRxJob, RadioService, RadioServiceError, RadioTxJob, RxPurpose,
    ScheduleError,
};
use raylar_time_service::UtcTimestamp;

use crate::{
    common, config,
    power::{ACTIVITY, POWER},
    storage::STORAGE_FLAGS,
    LOCATION,
};

pub const JOB_DEPTH: usize = 24;
pub const EVENT_DEPTH: usize = 32;
// The service requires 100 ms before a reservation starts. Allow another
// 150 ms for the coordinator period, queue delivery, and task latency.
const SUBMISSION_LEAD: Duration = Duration::from_millis(250);
const RX_MISS_RETRY: Duration = Duration::from_secs(2);
pub static RADIO: RadioResources<JOB_DEPTH, EVENT_DEPTH, 4> = RadioResources::new();
pub static RECOVERY_PENDING: AtomicBool = AtomicBool::new(false);
pub static VALID_RX: AtomicU32 = AtomicU32::new(0);
pub static COMPLETED_TX: AtomicU32 = AtomicU32::new(0);
static PROFILE_RX: [AtomicU32; 12] = [const { AtomicU32::new(0) }; 12];
type SpiDevice = ManualCsSpiDevice<Spi<'static, Blocking, Master>, Output<'static>>;
type BoardRadio =
    RadioDriver<SpiDevice, ExtiInput<'static, Async>, Output<'static>, ExtiInput<'static, Async>>;
type BoardRadioService = RadioService<'static, BoardRadio, JOB_DEPTH, EVENT_DEPTH, 4>;
type Log = LoggerHandle<'static, 384, 32>;

pub fn start(spawner: Spawner, rf: EbyteRf<'static>, node: NodeId, boot: u16, log: Log) {
    let EbyteRf {
        spi,
        sck,
        miso,
        mosi,
        cs,
        busy,
        nrst,
        irq,
    } = rf;
    let mut spi_config = embassy_stm32::spi::Config::default();
    spi_config.frequency = mhz(1);
    let spi = Spi::new_blocking(spi, sck, mosi, miso, spi_config);
    let driver = RadioDriver::with_timing(
        ManualCsSpiDevice::new(spi, cs),
        busy,
        nrst,
        irq,
        DriverTiming {
            preparation_guard: Duration::from_millis(10),
            ..DriverTiming::default()
        },
    );
    RADIO.set_admission_events(true);
    RADIO.set_enabled(false);
    spawner.spawn(unwrap!(service_task(RadioService::new(
        driver,
        &RADIO,
        Duration::from_millis(100)
    ))));
    spawner.spawn(unwrap!(coordinator_task(node, boot, log)));
    spawner.spawn(unwrap!(health_task(log)));
}

#[embassy_executor::task]
async fn service_task(service: BoardRadioService) -> ! {
    service.run().await
}

struct Minute {
    epoch: i64,
    index: usize,
    start: Instant,
    end: Instant,
    tx_start: Instant,
    tx_end: Instant,
    tx_id: Option<JobId>,
    tx_submitted_at: Option<Instant>,
    tx_completed: bool,
    rx_id: Option<JobId>,
    rx_submitted_at: Option<Instant>,
    rx_retry_at: Instant,
    rx_misses: u32,
    rx_at_start: u32,
    sequence: u16,
}

#[embassy_executor::task]
async fn coordinator_task(node: NodeId, boot: u16, log: Log) -> ! {
    let handle = RADIO.handle();
    let events = RADIO.event_receiver();
    let mut minute: Option<Minute> = None;
    let mut join_epoch: Option<i64> = None;
    let mut was_active = false;
    let mut sequence = 0u16;
    let mut last_summary = Instant::now();
    loop {
        let active = ACTIVITY.try_get().unwrap_or_default().active();
        if !active {
            if was_active {
                RECOVERY_PENDING.store(true, Ordering::Relaxed);
            }
            was_active = false;
            join_epoch = None;
            finish_minute(&mut minute, log);
            RADIO.set_enabled(false);
        } else {
            let state = common::TIME_RESOURCES.time_state();
            if let Ok(utc) = state.system_to_utc_holdover(Instant::now()) {
                let now_us = utc.as_micros();
                let epoch = now_us.div_euclid(config::EPOCH_US);
                if !was_active {
                    join_epoch = Some(epoch + 1);
                    was_active = true;
                }
                // Queue the next profile in second 59; the service prepares
                // it before the minute boundary, including slot zero.
                let target_us = now_us.saturating_add(1_000_000);
                let target_epoch = target_us.div_euclid(config::EPOCH_US);
                let phase = target_us.rem_euclid(config::EPOCH_US);
                let index = (phase / config::MINUTE_US) as usize;
                if target_epoch < join_epoch.unwrap_or(i64::MAX) || index >= config::ACTIVE_MINUTES
                {
                    finish_minute(&mut minute, log);
                    RADIO.set_enabled(false);
                } else {
                    RADIO.set_enabled(true);
                    let planned = minute.as_ref().map(|m| (m.epoch, m.index));
                    if planned != Some((target_epoch, index)) {
                        finish_minute(&mut minute, log);
                        minute = schedule_minute(
                            &handle,
                            node,
                            boot,
                            sequence,
                            target_epoch,
                            index,
                            log,
                        );
                        if minute.is_some() {
                            sequence = sequence.wrapping_add(1);
                        }
                    }
                    if let Some(m) = minute.as_mut() {
                        rearm_rx(&handle, m, log);
                    }
                }
            } else {
                RADIO.set_enabled(false);
                finish_minute(&mut minute, log);
            }
        }
        while let Ok(event) = events.try_receive() {
            process_event(event, &mut minute, log);
        }
        if Instant::now().saturating_duration_since(last_summary) >= Duration::from_secs(10) {
            let state = RADIO.state();
            let time = common::TIME_RESOURCES.time_state();
            let power = POWER.state();
            let gps = common::GPS_RESOURCES.stats();
            let location = LOCATION.state();
            let logging = crate::LOGGING.stats();
            let _ = log_info!(log, "summary active={} soc={:?} battery_mv={} solar_mv={} ext_dc_mv={} source={:?} charging={} time={:?} uncertainty_us={} holdover_s={} radio={:?} tx={} rx={} misses={} errors={} storage_flags={}",
                active, power.battery_percent, power.battery_mv, power.solar_mv, power.ext_dc_mv, power.source, power.charging, time.utc_status, time.uncertainty_us,
                time.holdover_duration.as_secs(), state.mode, state.stats.frames_tx, state.stats.frames_rx, state.stats.schedule_misses, state.stats.radio_errors,
                STORAGE_FLAGS.load(Ordering::Relaxed));
            let _ = log_info!(log, "summary2 gps_powered={} fixes={} pps={} location_valid={} lat_e7={} lon_e7={} hdop={:?} audio_losses={} log_drops={} log_truncated={} log_failures={} radio_conflicts={} radio_queue_drops={}",
                gps.powered, gps.num_fixes, gps.num_pps_events, location.valid, location.latitude.degrees_e7,
                location.longitude.degrees_e7, location.hdop_centi, crate::audio::AUDIO_LOSSES.load(Ordering::Relaxed),
                logging.dropped_messages, logging.truncated_messages, logging.write_failures,
                state.stats.scheduler_conflicts, state.stats.queue_drops);
            last_summary = Instant::now();
        }
        Timer::after_millis(25).await;
    }
}

fn schedule_minute(
    handle: &RadioHandle<'static, JOB_DEPTH>,
    node: NodeId,
    boot: u16,
    sequence: u16,
    epoch: i64,
    index: usize,
    log: Log,
) -> Option<Minute> {
    let profile = config::profile(index).ok()?;
    let state = common::TIME_RESOURCES.time_state();
    let start_us = epoch * config::EPOCH_US + index as i64 * config::MINUTE_US;
    let start = state
        .utc_to_system_holdover(UtcTimestamp::from_micros(start_us))
        .ok()?;
    let end = state
        .utc_to_system_holdover(UtcTimestamp::from_micros(start_us + 59_000_000))
        .ok()?;
    let slot = config::slot(node, epoch as u64, index).ok()?;
    let tx_start = start + Duration::from_secs(u64::from(slot) * config::slot_seconds(index));
    let tx_end = tx_start + Duration::from_secs(config::slot_seconds(index));
    let power = POWER.state();
    let time = common::TIME_RESOURCES.time_state();
    let charging = match power.source {
        PowerSource::Solar => 1,
        PowerSource::Usb => 2,
        PowerSource::ExternalDc => 3,
        PowerSource::Battery => 0,
        PowerSource::Unknown => 4,
    };
    let mut flags = STORAGE_FLAGS.load(Ordering::Relaxed);
    let logging = crate::LOGGING.stats();
    if logging.dropped_messages != 0 || logging.write_failures != 0 {
        flags |= LOGGING_IMPAIRED;
    }
    if RECOVERY_PENDING.load(Ordering::Relaxed) {
        flags |= ENERGY_RECOVERED;
    }
    let heartbeat = HeartbeatV4 {
        node,
        boot,
        sequence,
        soc: power.battery_percent,
        charging,
        errors: flags,
        storage_percent: None,
        gps_status: GpsStatus::from_states(time, LOCATION.state()).wire(),
    };
    let frame = FrameBuffer::from_slice(&heartbeat.encode().ok()?).ok()?;
    let frequency_hz = profile.frequency_hz();
    let submitted_at = Instant::now();
    let tx_late = tx_start < submitted_at + SUBMISSION_LEAD;
    let tx_id = if tx_late {
        None
    } else {
        match handle.try_submit_tx(RadioTxJob {
            earliest: tx_start,
            deadline: tx_end,
            profile,
            priority: RadioPriority::Control,
            payload: frame,
        }) {
            Ok(id) => Some(id),
            Err(error) => {
                let _ = log_info!(
                    log,
                    "tx enqueue failed epoch={} profile={} error={:?}",
                    epoch,
                    index + 1,
                    error
                );
                None
            }
        }
    };
    let slot_utc_us = start_us + i64::from(slot) * config::slot_seconds(index) as i64 * 1_000_000;
    let _ = log_info!(log, "minute epoch={} profile={} frequency_hz={} slot={} slot_s={} tx_id={:?} sequence={} payload_bytes=16 airtime_us={} slot_utc_us={} slot_ticks={} submit_ticks={} lead_us={} utc_status={:?} uncertainty_us={} holdover_s={}",
        epoch, index + 1, frequency_hz, slot, config::slot_seconds(index), tx_id.map(|id| id.0), sequence,
        config::airtime_us(index), slot_utc_us, tx_start.as_ticks(), submitted_at.as_ticks(),
        lead_us(tx_start, submitted_at), time.utc_status, time.uncertainty_us, time.holdover_duration.as_secs());
    if tx_late {
        let _ = log_info!(
            log,
            "tx skipped epoch={} profile={} slot={} lead_us={} required_us={}",
            epoch,
            index + 1,
            slot,
            lead_us(tx_start, submitted_at),
            SUBMISSION_LEAD.as_micros()
        );
    }
    Some(Minute {
        epoch,
        index,
        start,
        end,
        tx_start,
        tx_end,
        tx_id,
        tx_submitted_at: tx_id.map(|_| submitted_at),
        tx_completed: false,
        rx_id: None,
        rx_submitted_at: None,
        rx_retry_at: submitted_at,
        rx_misses: 0,
        rx_at_start: PROFILE_RX[index].load(Ordering::Relaxed),
        sequence,
    })
}

fn finish_minute(minute: &mut Option<Minute>, log: Log) {
    if let Some(m) = minute.take() {
        let received = PROFILE_RX[m.index]
            .load(Ordering::Relaxed)
            .saturating_sub(m.rx_at_start);
        let _ = log_info!(
            log,
            "minute end epoch={} profile={} sequence={} tx_completed={} valid_rx={} rx_misses={}",
            m.epoch,
            m.index + 1,
            m.sequence,
            m.tx_completed,
            received,
            m.rx_misses
        );
    }
}

fn rearm_rx(handle: &RadioHandle<'static, JOB_DEPTH>, minute: &mut Minute, log: Log) {
    if minute.rx_id.is_some() {
        return;
    }
    let now = Instant::now();
    if now < minute.rx_retry_at {
        return;
    }
    let guard = Duration::from_millis(120);
    let earliest = now + SUBMISSION_LEAD;
    let before_start = earliest.max(minute.start);
    let before_end = minute.tx_start.saturating_sub(guard);
    let (start, end) = if before_start + Duration::from_millis(30) < before_end {
        (before_start, before_end)
    } else {
        (earliest.max(minute.tx_end + guard), minute.end)
    };
    if start + Duration::from_millis(30) >= end {
        return;
    }
    let Ok(profile) = config::profile(minute.index) else {
        return;
    };
    let submitted_at = Instant::now();
    match handle.try_reserve_rx(RadioRxJob {
        start,
        end,
        profile,
        priority: RadioPriority::BestEffort,
        purpose: RxPurpose::Broadcast,
    }) {
        Ok(id) => {
            minute.rx_id = Some(id);
            minute.rx_submitted_at = Some(submitted_at);
        }
        Err(error) => {
            minute.rx_retry_at = submitted_at + RX_MISS_RETRY;
            let _ = log_info!(
                log,
                "rx enqueue failed profile={} error={:?}",
                minute.index + 1,
                error
            );
        }
    }
}

fn process_event(event: RadioEvent, minute: &mut Option<Minute>, log: Log) {
    match event {
        RadioEvent::Admitted {
            id,
            start,
            decided_at,
        } => {
            if let Some(m) = minute.as_ref() {
                let (kind, submitted_at) = if m.tx_id == Some(id) {
                    ("tx", m.tx_submitted_at)
                } else if m.rx_id == Some(id) {
                    ("rx", m.rx_submitted_at)
                } else {
                    ("old", None)
                };
                if let Some(submitted_at) = submitted_at {
                    let time = common::TIME_RESOURCES.time_state();
                    let _ = log_info!(log, "radio admitted id={} kind={} start_ticks={} submit_ticks={} decision_ticks={} submit_to_decision_us={} decision_lead_us={} uncertainty_us={} holdover_s={}",
                        id.0, kind, start.as_ticks(), submitted_at.as_ticks(), decided_at.as_ticks(),
                        decided_at.saturating_duration_since(submitted_at).as_micros(), lead_us(start, decided_at),
                        time.uncertainty_us, time.holdover_duration.as_secs());
                }
            }
        }
        RadioEvent::Received {
            id,
            frame,
            metadata,
        } => {
            if let Some(m) = minute.as_mut() {
                if m.rx_id == Some(id) {
                    m.rx_id = None;
                    m.rx_submitted_at = None;
                }
            }
            match HeartbeatV4::decode(frame.as_slice()) {
                Ok(h) => {
                    let time = common::TIME_RESOURCES.time_state();
                    let utc_us = time
                        .system_to_utc_holdover(metadata.packet_complete_at)
                        .ok()
                        .map(|v| v.as_micros());
                    let position = utc_us.map(|utc| {
                        let epoch = utc.div_euclid(config::EPOCH_US);
                        let phase = utc.rem_euclid(config::EPOCH_US);
                        let index = (phase / config::MINUTE_US) as usize;
                        let start = epoch * config::EPOCH_US + index as i64 * config::MINUTE_US;
                        let observed = (utc - start - config::airtime_us(index) as i64).max(0)
                            / (config::slot_seconds(index) as i64 * 1_000_000);
                        let predicted = config::slot(h.node, epoch as u64, index).ok();
                        (epoch, index + 1, observed, predicted)
                    });
                    let _ = log_info!(log, "rx id={} node={} boot={} seq={} epoch_min_slot_pred={:?} soc={:?} flags={} charge={} storage={:?} gps={}",
                        id.0, h.node.0, h.boot, h.sequence, position, h.soc, h.errors, h.charging, h.storage_percent, h.gps_status);
                    let _ = log_info!(log, "rxmeta id={} ticks={} utc_us={:?} utc_status={:?} uncertainty_us={} freq_hz={} rssi_x2={} snr_x4={:?} gfsk={:?}",
                        id.0, metadata.packet_complete_at.as_ticks(), utc_us, time.utc_status, time.uncertainty_us,
                        metadata.frequency_hz, metadata.rssi_dbm_x2, metadata.snr_db_x4, metadata.gfsk_status);
                    VALID_RX.fetch_add(1, Ordering::Relaxed);
                    if let Some((_, profile, _, _)) = position {
                        if let Some(counter) = PROFILE_RX.get(profile - 1) {
                            counter.fetch_add(1, Ordering::Relaxed);
                        }
                    }
                }
                Err(error) => {
                    let _ = log_info!(log, "rx malformed id={} error={:?}", id.0, error);
                }
            }
        }
        RadioEvent::Completed { id } => {
            if let Some(m) = minute.as_mut() {
                if m.tx_id == Some(id) {
                    if RECOVERY_PENDING.load(Ordering::Relaxed) {
                        RECOVERY_PENDING.store(false, Ordering::Relaxed);
                    }
                    let _ = log_info!(
                        log,
                        "tx complete id={} epoch={} profile={} seq={} event_ticks={} airtime_us={}",
                        id.0,
                        m.epoch,
                        m.index + 1,
                        m.sequence,
                        Instant::now().as_ticks(),
                        config::airtime_us(m.index)
                    );
                    m.tx_id = None;
                    m.tx_submitted_at = None;
                    m.tx_completed = true;
                    COMPLETED_TX.fetch_add(1, Ordering::Relaxed);
                }
            }
        }
        RadioEvent::RxWindowClosed { id } => {
            clear_job(minute, id);
        }
        RadioEvent::PacketRejected { id } => {
            clear_job(minute, id);
            let _ = log_info!(log, "radio packet rejected id={}", id.0);
        }
        RadioEvent::Cancelled { id } => {
            clear_job(minute, id);
            let _ = log_info!(log, "radio job cancelled id={}", id.0);
        }
        RadioEvent::Rejected {
            id,
            error,
            start,
            decided_at,
        } => {
            let mut kind = "old";
            let mut submitted_at = None;
            let mut log_rejection = true;
            if let Some(m) = minute.as_mut() {
                if m.tx_id == Some(id) {
                    kind = "tx";
                    submitted_at = m.tx_submitted_at;
                } else if m.rx_id == Some(id) {
                    kind = "rx";
                    submitted_at = m.rx_submitted_at;
                    if error == RadioServiceError::Schedule(ScheduleError::MissedSlot) {
                        m.rx_misses = m.rx_misses.saturating_add(1);
                        m.rx_retry_at = Instant::now() + RX_MISS_RETRY;
                        log_rejection = m.rx_misses == 1;
                    }
                }
            }
            clear_job(minute, id);
            if log_rejection {
                let time = common::TIME_RESOURCES.time_state();
                let _ = log_info!(log, "radio scheduler rejected id={} kind={} error={:?} start_ticks={} submit_ticks={:?} decision_ticks={} decision_lead_us={} event_ticks={} uncertainty_us={} holdover_s={}",
                    id.0, kind, error, start.as_ticks(), submitted_at.map(|at| at.as_ticks()), decided_at.as_ticks(),
                    lead_us(start, decided_at), Instant::now().as_ticks(), time.uncertainty_us,
                    time.holdover_duration.as_secs());
            }
        }
        RadioEvent::Failed { id, error } => {
            let mut log_failure = true;
            if let Some(m) = minute.as_mut() {
                if m.rx_id == Some(id)
                    && error == RadioServiceError::Schedule(ScheduleError::MissedSlot)
                {
                    m.rx_misses = m.rx_misses.saturating_add(1);
                    m.rx_retry_at = Instant::now() + RX_MISS_RETRY;
                    log_failure = m.rx_misses == 1;
                }
            }
            clear_job(minute, id);
            if log_failure {
                let _ = log_info!(
                    log,
                    "radio job failed id={} error={:?} event_ticks={}",
                    id.0,
                    error,
                    Instant::now().as_ticks()
                );
            }
        }
    }
}

fn clear_job(minute: &mut Option<Minute>, id: JobId) {
    if let Some(m) = minute.as_mut() {
        if m.rx_id == Some(id) {
            m.rx_id = None;
            m.rx_submitted_at = None;
        }
        if m.tx_id == Some(id) {
            m.tx_id = None;
            m.tx_submitted_at = None;
        }
    }
}

fn lead_us(start: Instant, now: Instant) -> i64 {
    let magnitude = Duration::from_ticks(start.as_ticks().abs_diff(now.as_ticks()))
        .as_micros()
        .min(i64::MAX as u64) as i64;
    if start >= now {
        magnitude
    } else {
        -magnitude
    }
}

#[embassy_executor::task]
async fn health_task(log: Log) -> ! {
    let mut since = None;
    loop {
        let mode = RADIO.state().mode;
        if ACTIVITY.try_get().unwrap_or_default().active()
            && matches!(mode, RadioMode::Recovering | RadioMode::Initializing)
        {
            let start = since.get_or_insert_with(Instant::now);
            if Instant::now().saturating_duration_since(*start) > Duration::from_secs(60) {
                let _ = log_info!(log, "radio unrecoverable for 60s; software reset");
                error!("radio unrecoverable; resetting");
                raylar_drivers::stm32_core::stm32::Stm32CoreDriver::reset();
            }
        } else {
            since = None;
        }
        Timer::after_secs(1).await;
    }
}
