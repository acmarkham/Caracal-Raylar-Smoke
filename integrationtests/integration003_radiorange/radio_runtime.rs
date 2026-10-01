use defmt::{error, info, warn};
use embassy_stm32::spi::{Config as SpiConfig, Spi};
use embassy_stm32::time::mhz;
use embassy_time::{Duration, Instant, Timer};
use heapless::Vec;
use raylar_board_v1p0::EbyteRf;
use raylar_drivers::radio::{
    ChannelConfig, DriverTiming, Error as RadioError, ManualCsSpiDevice, RadioDriver, RxMetadata,
    RxMetrics,
};
use raylar_logging_service::{info as log_info, warn as log_warn, LogLevel};
use raylar_time_service::{TimeSource, UtcStatus};

use crate::packet::{DecodeError, RangePacket, PACKET_LEN};
use crate::position::{self, LocalPosition};
use crate::radio_test_config as config;
use crate::{common, record_log_outcome, TestLogger, LOCATION, RECEIVE_INDICATION};

const MAX_TRACKED_PEERS: usize = 8;

pub async fn run(rf: EbyteRf<'static>, device_id: u64, logger: TestLogger) -> ! {
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
    let mut spi_config = SpiConfig::default();
    spi_config.frequency = mhz(1);
    let spi = Spi::new_blocking(spi, sck, mosi, miso, spi_config);
    let spi_device = ManualCsSpiDevice::new(spi, cs);
    let timing = DriverTiming {
        preparation_guard: Duration::from_millis(10),
        ..DriverTiming::default()
    };
    let mut radio = RadioDriver::with_timing(spi_device, busy, nrst, irq, timing);
    initialize_radio(&mut radio, logger).await;
    wait_for_initial_readiness(logger).await;

    let channel = config::channel();
    record_log_outcome(log_info!(
        logger,
        "radio_configuration channel={:?} tx={:?}",
        channel,
        config::tx_config(),
    ));
    if let Err(value) = radio.prepare_channel(&channel).await {
        record_log_outcome(log_warn!(
            logger,
            "channel_prepare_failed error={:?}",
            value
        ));
        recover_and_prepare(&mut radio, &channel, logger).await;
    }
    record_log_outcome(log_info!(
        logger,
        "radio_started config_id={} modulation={} frequency_hz={} utc_gate=GpsPps/Synchronized location_valid=true",
        config::CONFIGURATION_ID,
        config::MODULATION_NAME,
        config::FREQUENCY_HZ,
    ));

    let seed = (device_id as u32) ^ ((device_id >> 32) as u32) ^ (Instant::now().as_ticks() as u32);
    let mut random = XorShift32::new(seed);
    let mut next_tx = Instant::now() + random_tx_interval(&mut random);
    let mut next_sequence = Some(0u16);
    let mut rx_buffer = [0u8; PACKET_LEN];
    let mut peers: Vec<Peer, MAX_TRACKED_PEERS> = Vec::new();
    let mut rx_count = 0u64;
    let mut utc_suspended = false;
    let mut location_ready = true;

    loop {
        if common::TIME_RESOURCES.time_state().utc_status == UtcStatus::Invalid {
            if !utc_suspended {
                let _ = radio.standby().await;
                record_log_outcome(log_warn!(logger, "radio_suspended reason=utc_invalid"));
                utc_suspended = true;
            }
            Timer::after_secs(1).await;
            continue;
        }
        if utc_suspended {
            record_log_outcome(log_info!(logger, "radio_resumed reason=utc_valid"));
            next_tx = Instant::now() + random_tx_interval(&mut random);
            utc_suspended = false;
        }

        let current_location_ready =
            position::from_location(LOCATION.state(), Instant::now()).is_ok();
        if current_location_ready != location_ready {
            record_log_outcome(if current_location_ready {
                log_info!(logger, "location_state ready=true")
            } else {
                log_warn!(logger, "location_state ready=false tx_suspended=true")
            });
            location_ready = current_location_ready;
        }

        let now = Instant::now();
        if now + config::RX_START_GUARD < next_tx {
            let rx_start = now + config::RX_START_GUARD;
            match radio.receive_at(rx_start, next_tx, &mut rx_buffer).await {
                Ok(packet) => handle_rx(
                    packet.payload,
                    packet.metadata,
                    &mut peers,
                    &mut rx_count,
                    logger,
                ),
                Err(RadioError::RxTimeout { at }) => {
                    record_log_outcome(logger.log_at(
                        at,
                        LogLevel::Debug,
                        format_args!("rx_window_timeout switch_to_tx=true tick={}", at.as_ticks()),
                    ));
                }
                Err(
                    value @ (RadioError::CrcRejected { .. }
                    | RadioError::HeaderRejected { .. }
                    | RadioError::GfskLengthRejected { .. }
                    | RadioError::GfskAddressRejected { .. }),
                ) => {
                    let at = error_time(value).unwrap_or_else(Instant::now);
                    let time = common::TIME_RESOURCES.time_state();
                    let utc = common::TIME_RESOURCES.system_to_utc(at).ok();
                    record_log_outcome(logger.log_at(
                        at,
                        LogLevel::Warn,
                        format_args!(
                            "rx_rejected_by_radio error={:?} tick={} rx_utc={:?} utc_status={:?} utc_source={:?} utc_uncertainty_us={}",
                            value,
                            at.as_ticks(),
                            utc,
                            time.utc_status,
                            time.active_time_source,
                            time.uncertainty_us,
                        ),
                    ));
                }
                Err(value) => {
                    let at = error_time(value).unwrap_or_else(Instant::now);
                    record_log_outcome(logger.log_at(
                        at,
                        LogLevel::Warn,
                        format_args!("rx_failed error={:?} tick={}", value, at.as_ticks()),
                    ));
                    recover_and_prepare(&mut radio, &channel, logger).await;
                }
            }
            continue;
        }

        if let Some(sequence) = next_sequence {
            let attempted = match transmit(&mut radio, device_id, sequence, logger).await {
                Ok(attempted) => attempted,
                Err(()) => {
                    recover_and_prepare(&mut radio, &channel, logger).await;
                    true
                }
            };
            if attempted {
                next_sequence = sequence.checked_add(1);
                if next_sequence.is_none() {
                    record_log_outcome(log_warn!(
                        logger,
                        "tx_sequence_exhausted last_sequence={} transmitter_disabled=true",
                        sequence
                    ));
                }
            }
        }
        next_tx = Instant::now() + random_tx_interval(&mut random);
        info!("Next range-test TX tick={}", next_tx.as_ticks());
    }
}

async fn transmit<SPI, BUSY, RESET, IRQ>(
    radio: &mut RadioDriver<SPI, BUSY, RESET, IRQ>,
    device_id: u64,
    sequence: u16,
    logger: TestLogger,
) -> Result<bool, ()>
where
    SPI: embedded_hal_async::spi::SpiDevice<u8>,
    BUSY: embedded_hal::digital::InputPin + embedded_hal_async::digital::Wait,
    RESET: embedded_hal::digital::OutputPin,
    IRQ: embedded_hal::digital::InputPin + embedded_hal_async::digital::Wait,
{
    let now = Instant::now();
    let location = match position::from_location(LOCATION.state(), now) {
        Ok(value) => value,
        Err(value) => {
            record_log_outcome(log_warn!(
                logger,
                "tx_skipped sequence={} reason=location_{:?}",
                sequence,
                value
            ));
            return Ok(false);
        }
    };
    let tx_start = now + config::RX_START_GUARD;
    let tx_utc = match common::TIME_RESOURCES.system_to_utc(tx_start) {
        Ok(value) => value,
        Err(value) => {
            record_log_outcome(log_warn!(
                logger,
                "tx_skipped sequence={} reason=utc_{:?}",
                sequence,
                value
            ));
            return Ok(false);
        }
    };
    let payload = RangePacket {
        sender_id: device_id,
        sequence,
        tx_utc,
        east_10m: location.east_10m,
        north_10m: location.north_10m,
    }
    .encode();
    let time = common::TIME_RESOURCES.time_state();
    record_log_outcome(log_info!(
        logger,
        "tx_attempt sender={:#018x} sequence={} tx_utc={}.{:06} east_10m={} north_10m={} location_age_ms={} request_tick={} utc_status={:?} utc_source={:?} uncertainty_us={} frequency_hz={} power_dbm={}",
        device_id,
        sequence,
        tx_utc.seconds,
        tx_utc.microseconds,
        location.east_10m,
        location.north_10m,
        location.age_ms,
        tx_start.as_ticks(),
        time.utc_status,
        time.active_time_source,
        time.uncertainty_us,
        config::FREQUENCY_HZ,
        config::TX_POWER_DBM,
    ));
    match radio
        .transmit_at(tx_start, &payload, &config::tx_config())
        .await
    {
        Ok(report) => {
            let done_utc = common::TIME_RESOURCES.system_to_utc(report.tx_done_at).ok();
            record_log_outcome(logger.log_at(
                report.tx_done_at,
                LogLevel::Info,
                format_args!(
                    "tx_complete sender={:#018x} sequence={} tx_utc={}.{:06} done_utc={:?} request_tick={} command_start_tick={} command_done_tick={} tx_done_tick={}",
                    device_id,
                    sequence,
                    tx_utc.seconds,
                    tx_utc.microseconds,
                    done_utc,
                    report.requested_start.as_ticks(),
                    report.command_started_at.as_ticks(),
                    report.command_completed_at.as_ticks(),
                    report.tx_done_at.as_ticks(),
                ),
            ));
            info!(
                "TX sender={=u64:#x} sequence={} tx_utc_s={} tx_utc_us={} request_tick={} done_tick={}",
                device_id,
                sequence,
                tx_utc.seconds,
                tx_utc.microseconds,
                report.requested_start.as_ticks(),
                report.tx_done_at.as_ticks(),
            );
            Ok(true)
        }
        Err(value) => {
            let at = error_time(value).unwrap_or_else(Instant::now);
            record_log_outcome(logger.log_at(
                at,
                LogLevel::Warn,
                format_args!(
                    "tx_failed sequence={} error={:?} tick={}",
                    sequence,
                    value,
                    at.as_ticks()
                ),
            ));
            warn!("TX failed sequence={} error={:?}", sequence, value);
            Err(())
        }
    }
}

fn handle_rx(
    payload: &[u8],
    metadata: RxMetadata,
    peers: &mut Vec<Peer, MAX_TRACKED_PEERS>,
    rx_count: &mut u64,
    logger: TestLogger,
) {
    let packet = match RangePacket::decode(payload) {
        Ok(value) => value,
        Err(value) => {
            log_invalid_rx(value, payload.len(), metadata, logger);
            return;
        }
    };
    let sequence = observe_sequence(peers, packet.sender_id, packet.sequence);
    if matches!(
        sequence,
        SequenceObservation::Duplicate | SequenceObservation::Stale
    ) {
        let time = common::TIME_RESOURCES.time_state();
        let rx_utc = common::TIME_RESOURCES
            .system_to_utc(metadata.packet_complete_at)
            .ok();
        record_log_outcome(logger.log_at(
            metadata.packet_complete_at,
            LogLevel::Warn,
            format_args!(
                "rx_rejected sender={:#018x} sequence={} reason={:?} rx_utc={:?} frequency_hz={} metrics={:?} utc_status={:?} utc_source={:?} utc_uncertainty_us={}",
                packet.sender_id,
                packet.sequence,
                sequence,
                rx_utc,
                metadata.frequency_hz,
                metadata.metrics,
                time.utc_status,
                time.active_time_source,
                time.uncertainty_us,
            ),
        ));
        return;
    }
    let rx_utc = common::TIME_RESOURCES
        .system_to_utc(metadata.packet_complete_at)
        .ok();
    let receiver = position::from_location(LOCATION.state(), metadata.packet_complete_at).ok();
    let distance =
        receiver.map(|value| position::distance_metres(value, packet.east_10m, packet.north_10m));
    let uncertainty = receiver.map(|value| {
        value
            .uncertainty_m
            .unwrap_or(0)
            .saturating_add(config::REMOTE_POSITION_ERROR_BUDGET_METRES)
    });
    let packet_age_us =
        rx_utc.map(|value| value.as_micros().saturating_sub(packet.tx_utc.as_micros()));
    let time = common::TIME_RESOURCES.time_state();
    *rx_count = rx_count.saturating_add(1);
    log_valid_rx(
        packet,
        metadata,
        receiver,
        distance,
        uncertainty,
        packet_age_us,
        rx_utc,
        sequence,
        *rx_count,
        time.utc_status,
        time.active_time_source,
        time.uncertainty_us,
        logger,
    );
    RECEIVE_INDICATION.signal(());
    display_rx(packet, metadata, *rx_count, distance, rx_utc);
}

fn display_rx(
    packet: RangePacket,
    metadata: RxMetadata,
    count: u64,
    distance_m: Option<u32>,
    rx_utc: Option<raylar_time_service::UtcTimestamp>,
) {
    match metadata.metrics {
        RxMetrics::LoRa {
            rssi_dbm_x2,
            signal_rssi_dbm_x2,
            snr_db_x4,
        } => info!(
            "RX LoRa count={} sender={=u64:#x} sequence={} distance_m={:?} tx_utc_s={} tx_utc_us={} rx_utc={:?} complete_tick={} rssi_x2={} signal_rssi_x2={} snr_x4={}",
            count,
            packet.sender_id,
            packet.sequence,
            distance_m,
            packet.tx_utc.seconds,
            packet.tx_utc.microseconds,
            rx_utc,
            metadata.packet_complete_at.as_ticks(),
            rssi_dbm_x2,
            signal_rssi_dbm_x2,
            snr_db_x4,
        ),
        RxMetrics::Gfsk {
            rssi_dbm_x2,
            status,
        } => info!(
            "RX GFSK count={} sender={=u64:#x} sequence={} distance_m={:?} tx_utc_s={} tx_utc_us={} rx_utc={:?} complete_tick={} rssi_x2={} sync_rssi_x2={} len_err={} crc_err={} abort_err={} address_err={} sync_err={}",
            count,
            packet.sender_id,
            packet.sequence,
            distance_m,
            packet.tx_utc.seconds,
            packet.tx_utc.microseconds,
            rx_utc,
            metadata.packet_complete_at.as_ticks(),
            rssi_dbm_x2,
            status.sync_rssi_dbm_x2,
            status.length_error,
            status.crc_error,
            status.abort_error,
            status.address_error,
            status.sync_error,
        ),
    }
}

fn log_valid_rx(
    packet: RangePacket,
    metadata: RxMetadata,
    receiver: Option<LocalPosition>,
    distance_m: Option<u32>,
    distance_uncertainty_m: Option<u32>,
    packet_age_us: Option<i64>,
    rx_utc: Option<raylar_time_service::UtcTimestamp>,
    sequence: SequenceObservation,
    rx_count: u64,
    utc_status: UtcStatus,
    utc_source: TimeSource,
    utc_uncertainty_us: u64,
    logger: TestLogger,
) {
    record_log_outcome(logger.log_at(
        metadata.packet_complete_at,
        LogLevel::Info,
        format_args!(
            "rx_packet count={} sender={:#018x} sequence={} sequence_state={:?} tx_utc={}.{:06} rx_utc={:?} packet_age_us={:?} tx_east_10m={} tx_north_10m={} receiver={:?} distance_m={:?} distance_uncertainty_m={:?} complete_tick={} frequency_hz={} modulation={} metrics={:?} utc_status={:?} utc_source={:?} utc_uncertainty_us={}",
            rx_count,
            packet.sender_id,
            packet.sequence,
            sequence,
            packet.tx_utc.seconds,
            packet.tx_utc.microseconds,
            rx_utc,
            packet_age_us,
            packet.east_10m,
            packet.north_10m,
            receiver,
            distance_m,
            distance_uncertainty_m,
            metadata.packet_complete_at.as_ticks(),
            metadata.frequency_hz,
            config::MODULATION_NAME,
            metadata.metrics,
            utc_status,
            utc_source,
            utc_uncertainty_us,
        ),
    ));
}

fn log_invalid_rx(
    error_value: DecodeError,
    bytes: usize,
    metadata: RxMetadata,
    logger: TestLogger,
) {
    let time = common::TIME_RESOURCES.time_state();
    let rx_utc = common::TIME_RESOURCES
        .system_to_utc(metadata.packet_complete_at)
        .ok();
    record_log_outcome(logger.log_at(
        metadata.packet_complete_at,
        LogLevel::Warn,
        format_args!(
            "rx_decode_error reason={:?} bytes={} complete_tick={} rx_utc={:?} frequency_hz={} metrics={:?} utc_status={:?} utc_source={:?} utc_uncertainty_us={}",
            error_value,
            bytes,
            metadata.packet_complete_at.as_ticks(),
            rx_utc,
            metadata.frequency_hz,
            metadata.metrics,
            time.utc_status,
            time.active_time_source,
            time.uncertainty_us,
        ),
    ));
}

async fn wait_for_initial_readiness(logger: TestLogger) {
    let mut last_report = Instant::from_ticks(0);
    loop {
        let now = Instant::now();
        let time = common::TIME_RESOURCES.time_state();
        let location = LOCATION.state();
        let location_ready = position::from_location(location, now).is_ok();
        if time.utc_status == UtcStatus::Synchronized
            && time.active_time_source == TimeSource::GpsPps
            && location_ready
        {
            record_log_outcome(log_info!(
                logger,
                "startup_gate_open utc_status={:?} source={:?} uncertainty_us={} accepted_anchors={} location_fixes={} satellites={:?} hdop_centi={:?} location_uncertainty_m={:?}",
                time.utc_status,
                time.active_time_source,
                time.uncertainty_us,
                time.accepted_anchors,
                location.fix_count_used,
                location.satellites,
                location.hdop_centi,
                location.uncertainty_meters,
            ));
            return;
        }
        if now.saturating_duration_since(last_report) >= Duration::from_secs(5) {
            record_log_outcome(log_info!(
                logger,
                "startup_gate_wait utc_status={:?} source={:?} uncertainty_us={} location_valid={} location_ready={} fixes={} satellites={:?}",
                time.utc_status,
                time.active_time_source,
                time.uncertainty_us,
                location.valid,
                location_ready,
                location.fix_count_used,
                location.satellites,
            ));
            last_report = now;
        }
        Timer::after_millis(250).await;
    }
}

async fn initialize_radio<SPI, BUSY, RESET, IRQ>(
    radio: &mut RadioDriver<SPI, BUSY, RESET, IRQ>,
    logger: TestLogger,
) where
    SPI: embedded_hal_async::spi::SpiDevice<u8>,
    BUSY: embedded_hal::digital::InputPin + embedded_hal_async::digital::Wait,
    RESET: embedded_hal::digital::OutputPin,
    IRQ: embedded_hal::digital::InputPin + embedded_hal_async::digital::Wait,
{
    loop {
        match radio.initialize().await {
            Ok(()) => return,
            Err(value) => {
                record_log_outcome(log_warn!(
                    logger,
                    "radio_initialize_failed error={:?}",
                    value
                ));
                error!("Radio initialization failed: {:?}", value);
                Timer::after_secs(1).await;
            }
        }
    }
}

async fn recover_and_prepare<SPI, BUSY, RESET, IRQ>(
    radio: &mut RadioDriver<SPI, BUSY, RESET, IRQ>,
    channel: &ChannelConfig,
    logger: TestLogger,
) where
    SPI: embedded_hal_async::spi::SpiDevice<u8>,
    BUSY: embedded_hal::digital::InputPin + embedded_hal_async::digital::Wait,
    RESET: embedded_hal::digital::OutputPin,
    IRQ: embedded_hal::digital::InputPin + embedded_hal_async::digital::Wait,
{
    loop {
        let result = match radio.recover().await {
            Ok(()) => radio.prepare_channel(channel).await,
            Err(value) => Err(value),
        };
        match result {
            Ok(()) => {
                record_log_outcome(log_info!(logger, "radio_recovery_complete"));
                return;
            }
            Err(value) => {
                record_log_outcome(log_warn!(logger, "radio_recovery_failed error={:?}", value));
                Timer::after_secs(1).await;
            }
        }
    }
}

fn error_time(value: RadioError) -> Option<Instant> {
    match value {
        RadioError::RxTimeout { at }
        | RadioError::TxTimeout { at }
        | RadioError::CrcRejected { at }
        | RadioError::HeaderRejected { at }
        | RadioError::GfskLengthRejected { at }
        | RadioError::GfskAddressRejected { at }
        | RadioError::CommandIrq { at }
        | RadioError::DeviceIrq { at } => Some(at),
        _ => None,
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Peer {
    sender_id: u64,
    last_sequence: u16,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum SequenceObservation {
    First,
    Advanced { gap: u16 },
    Duplicate,
    Stale,
    Untracked,
}

fn observe_sequence(
    peers: &mut Vec<Peer, MAX_TRACKED_PEERS>,
    sender_id: u64,
    sequence: u16,
) -> SequenceObservation {
    if let Some(peer) = peers.iter_mut().find(|peer| peer.sender_id == sender_id) {
        if sequence == peer.last_sequence {
            return SequenceObservation::Duplicate;
        }
        if sequence < peer.last_sequence {
            return SequenceObservation::Stale;
        }
        let gap = sequence - peer.last_sequence - 1;
        peer.last_sequence = sequence;
        return SequenceObservation::Advanced { gap };
    }
    if peers
        .push(Peer {
            sender_id,
            last_sequence: sequence,
        })
        .is_err()
    {
        SequenceObservation::Untracked
    } else {
        SequenceObservation::First
    }
}

fn random_tx_interval(random: &mut XorShift32) -> Duration {
    Duration::from_secs(
        config::MIN_TX_INTERVAL_SECS + u64::from(random.next() % config::TX_INTERVAL_SPAN_SECS),
    )
}

struct XorShift32 {
    state: u32,
}

impl XorShift32 {
    fn new(seed: u32) -> Self {
        Self {
            state: if seed == 0 { 0xA341_316C } else { seed },
        }
    }

    fn next(&mut self) -> u32 {
        let mut value = self.state;
        value ^= value << 13;
        value ^= value >> 17;
        value ^= value << 5;
        self.state = value;
        value
    }
}
