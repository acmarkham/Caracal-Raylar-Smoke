#![no_std]
#![no_main]

extern crate alloc;

#[path = "../../servicetests/storage/common.rs"]
#[allow(dead_code)]
mod common;
mod packet;
mod position;
mod radio_runtime;
mod radio_test_config;

use core::sync::atomic::{AtomicU32, Ordering};

use defmt::{error, info, unwrap, warn};
use embassy_executor::Spawner;
use embassy_stm32::gpio::Output;
use embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex;
use embassy_sync::channel::Channel;
use embassy_time::{Duration, Instant, Timer};
use embedded_alloc::LlffHeap as Heap;
use raylar_board_v1p0::{Board, Leds};
use raylar_drivers::{buzzer, identity};
use raylar_location_service::{LocationConfig, LocationResources, LocationService};
use raylar_logging_service::{
    info as log_info, LogOutcome, LoggerHandle, LoggingResources, LoggingService, ProcessOutcome,
    StorageLogSink,
};
use raylar_storage_service::StorageService;
use raylar_time_service::{TimeResources, TimeSource};
use static_cell::StaticCell;
use {defmt_rtt as _, panic_probe as _};

const HEAP_BYTES: usize = 64 * 1024;
const LOCATION_HISTORY: usize = 9;
const MESSAGE_LENGTH: usize = 640;
const QUEUE_DEPTH: usize = 32;
const LINE_LENGTH: usize = 768;
const LOG_WRITE_BUFFER_BYTES: usize = 4 * 1024;
const LOG_FLUSH_INTERVAL: Duration = Duration::from_secs(10);

pub(crate) type TestLogger = LoggerHandle<'static, MESSAGE_LENGTH, QUEUE_DEPTH>;
type BoardStorage = StorageService<
    common::BoardStorageBackend,
    &'static TimeResources<4, 8>,
    512,
    1,
    LOG_WRITE_BUFFER_BYTES,
>;
type BoardLogSink = StorageLogSink<
    'static,
    common::BoardStorageBackend,
    &'static TimeResources<4, 8>,
    512,
    1,
    LOG_WRITE_BUFFER_BYTES,
>;
type BoardLogging = LoggingService<'static, BoardLogSink, MESSAGE_LENGTH, QUEUE_DEPTH, LINE_LENGTH>;

pub(crate) static LOCATION: LocationResources<4> = LocationResources::new();
static LOGGING: LoggingResources<MESSAGE_LENGTH, QUEUE_DEPTH> = LoggingResources::new();
static INDICATION_COMMANDS: Channel<CriticalSectionRawMutex, IndicationCommand, 16> =
    Channel::new();
static STORAGE: StaticCell<BoardStorage> = StaticCell::new();
static LOGGING_SERVICE: StaticCell<BoardLogging> = StaticCell::new();
static LOG_DROPS: AtomicU32 = AtomicU32::new(0);
static LOG_TRUNCATIONS: AtomicU32 = AtomicU32::new(0);

#[derive(Clone, Copy)]
enum IndicationCommand {
    Startup,
    Receive,
    Transmit,
    Error,
}

#[global_allocator]
static HEAP: Heap = Heap::empty();

#[embassy_executor::main]
async fn main(spawner: Spawner) -> ! {
    unsafe {
        embedded_alloc::init!(HEAP, HEAP_BYTES);
    }

    let peripherals = embassy_stm32::init(common::mcu_config());
    let Board {
        leds,
        buzzer: board_buzzer,
        gps,
        sd,
        ebyte_rf,
        ..
    } = Board::new(peripherals);
    let Leds {
        sys_gps_green,
        sys_main_green,
        sys_sd_blue,
        ..
    } = leds;
    let buzzer_driver = buzzer::init(buzzer::BuzzerResources {
        timer: board_buzzer.tim,
        pin: board_buzzer.pin,
    });
    spawner.spawn(unwrap!(indication_task(
        sys_main_green,
        sys_sd_blue,
        buzzer_driver
    )));
    INDICATION_COMMANDS.send(IndicationCommand::Startup).await;

    common::start_time(spawner, gps).await;
    spawner.spawn(unwrap!(pps_led_task(sys_gps_green)));
    let location_service = LocationService::<4, LOCATION_HISTORY>::new(
        &LOCATION,
        unwrap!(common::GPS_RESOURCES.fix_receiver()).as_dyn(),
        LocationConfig::default(),
    );
    spawner.spawn(unwrap!(location_service_task(location_service)));

    let backend = common::storage_driver(sd).await;
    let mut storage: BoardStorage = unwrap!(StorageService::new(backend, &common::TIME_RESOURCES));
    if let Err(value) = storage.mount().await {
        fail_forever("storage mount failed", value).await;
    }
    let storage = STORAGE.init(storage);
    let sink = match StorageLogSink::open(storage).await {
        Ok(sink) => sink,
        Err(value) => fail_forever("log stream open failed", value).await,
    };
    let mut logging = LoggingService::new(&LOGGING, sink);
    let system_log = logging.register("System");
    let radio_log = logging.register("RadioRange");
    let diagnostics_log = logging.register("Diagnostics");
    let device_id = identity::init().serial_64();

    record_log_outcome(log_info!(
        system_log,
        "integration003 started device_id={:#018x} config_id={} protocol={} modulation={} frequency_hz={} tx_power_dbm={} origin_lat_e7={} origin_lon_e7={} interval_s={}..{} packet_bytes={}",
        device_id,
        radio_test_config::CONFIGURATION_ID,
        radio_test_config::PROTOCOL_VERSION,
        radio_test_config::MODULATION_NAME,
        radio_test_config::FREQUENCY_HZ,
        radio_test_config::TX_POWER_DBM,
        radio_test_config::ORIGIN_LATITUDE_E7,
        radio_test_config::ORIGIN_LONGITUDE_E7,
        radio_test_config::MIN_TX_INTERVAL_SECS,
        radio_test_config::MIN_TX_INTERVAL_SECS
            + u64::from(radio_test_config::TX_INTERVAL_SPAN_SECS - 1),
        packet::PACKET_LEN,
    ));
    while matches!(logging.process_one().await, Ok(ProcessOutcome::Written)) {}
    if let Err(value) = logging.flush().await {
        warn!("initial log flush failed: {:?}", value);
    }
    let logging = LOGGING_SERVICE.init(logging);
    spawner.spawn(unwrap!(logging_task(logging, diagnostics_log)));

    info!(
        "Integration003 initialized device={=u64:#x}; waiting for GPS UTC and location",
        device_id
    );
    radio_runtime::run(ebyte_rf, device_id, radio_log).await
}

pub(crate) fn record_log_outcome(outcome: LogOutcome) -> bool {
    match outcome {
        LogOutcome::Enqueued => true,
        LogOutcome::EnqueuedTruncated => {
            LOG_TRUNCATIONS.fetch_add(1, Ordering::Relaxed);
            warn!("range-test log record truncated");
            true
        }
        LogOutcome::DroppedQueueFull => {
            LOG_DROPS.fetch_add(1, Ordering::Relaxed);
            error!("range-test log record dropped: queue full");
            signal_error_indication();
            false
        }
    }
}

pub(crate) fn signal_receive_indication() {
    let _ = INDICATION_COMMANDS.try_send(IndicationCommand::Receive);
}

pub(crate) fn signal_transmit_indication() {
    let _ = INDICATION_COMMANDS.try_send(IndicationCommand::Transmit);
}

pub(crate) fn signal_error_indication() {
    let _ = INDICATION_COMMANDS.try_send(IndicationCommand::Error);
}

#[embassy_executor::task]
async fn logging_task(logging: &'static mut BoardLogging, diagnostics: TestLogger) -> ! {
    let mut next_flush = Instant::now() + LOG_FLUSH_INTERVAL;
    let mut last_drops = 0;
    let mut last_truncations = 0;
    loop {
        if Instant::now() >= next_flush {
            if let Err(value) = logging.flush().await {
                error!("range-test log flush failed: {:?}", value);
                signal_error_indication();
            }
            let stats = logging.stats();
            let drops = LOG_DROPS.load(Ordering::Relaxed);
            let truncations = LOG_TRUNCATIONS.load(Ordering::Relaxed);
            info!(
                "Log stats total={} dropped={} local_drops={} truncated={} local_truncated={} failures={} depth={}",
                stats.total_messages,
                stats.dropped_messages,
                drops,
                stats.truncated_messages,
                truncations,
                stats.write_failures,
                stats.queue_depth
            );
            if drops != last_drops || truncations != last_truncations || stats.write_failures != 0 {
                record_log_outcome(log_info!(
                    diagnostics,
                    "logging_health dropped={} enqueue_drops={} truncated={} enqueue_truncated={} write_failures={} max_depth={}",
                    stats.dropped_messages,
                    drops,
                    stats.truncated_messages,
                    truncations,
                    stats.write_failures,
                    stats.maximum_queue_depth,
                ));
                last_drops = drops;
                last_truncations = truncations;
            }
            next_flush = Instant::now() + LOG_FLUSH_INTERVAL;
        }

        match logging.process_one().await {
            Ok(ProcessOutcome::Written) => {}
            Ok(ProcessOutcome::Empty) => Timer::after_millis(10).await,
            Err(value) => {
                error!("range-test log append failed: {:?}", value);
                signal_error_indication();
                Timer::after_millis(100).await;
            }
        }
    }
}

#[embassy_executor::task]
async fn location_service_task(service: LocationService<4, LOCATION_HISTORY>) -> ! {
    service.run().await
}

#[embassy_executor::task]
async fn pps_led_task(mut led: Output<'static>) -> ! {
    let mut states = unwrap!(common::TIME_RESOURCES.state_receiver());
    let mut observed_anchors = common::TIME_RESOURCES.time_state().accepted_anchors;
    led.set_low();
    loop {
        let state = states.changed().await;
        let accepted_gps_pps = state.active_time_source == TimeSource::GpsPps
            && state.accepted_anchors != observed_anchors;
        observed_anchors = state.accepted_anchors;
        if accepted_gps_pps {
            led.set_high();
            Timer::after_millis(50).await;
            led.set_low();
        }
    }
}

#[embassy_executor::task]
async fn indication_task(
    mut receive_led: Output<'static>,
    mut transmit_led: Output<'static>,
    mut buzzer_driver: buzzer::BuzzerDriver<'static>,
) -> ! {
    receive_led.set_low();
    transmit_led.set_low();
    loop {
        match INDICATION_COMMANDS.receive().await {
            IndicationCommand::Startup => play_startup_beeps(&mut buzzer_driver).await,
            IndicationCommand::Receive => {
                receive_led.set_high();
                if let Err(value) = buzzer_driver
                    .play_tone(
                        buzzer::PitchHz(radio_test_config::INDICATION_PITCH_HZ),
                        radio_test_config::INDICATION_DURATION,
                        buzzer::Volume(radio_test_config::INDICATION_VOLUME),
                    )
                    .await
                {
                    warn!("receive beep failed: {:?}", value);
                }
                receive_led.set_low();
            }
            IndicationCommand::Transmit => {
                transmit_led.set_high();
                Timer::after_millis(50).await;
                transmit_led.set_low();
            }
            IndicationCommand::Error => play_error_beeps(&mut buzzer_driver).await,
        }
    }
}

async fn play_startup_beeps(driver: &mut buzzer::BuzzerDriver<'static>) {
    for pitch_hz in [1_047, 1_319, 1_568] {
        if let Err(value) = driver
            .play_tone(
                buzzer::PitchHz(pitch_hz),
                Duration::from_millis(70),
                buzzer::Volume(radio_test_config::INDICATION_VOLUME),
            )
            .await
        {
            warn!("startup beep failed: {:?}", value);
        }
        Timer::after_millis(35).await;
    }
}

async fn play_error_beeps(driver: &mut buzzer::BuzzerDriver<'static>) {
    for pitch_hz in [440, 330, 220] {
        if let Err(value) = driver
            .play_tone(
                buzzer::PitchHz(pitch_hz),
                Duration::from_millis(140),
                buzzer::Volume(radio_test_config::INDICATION_VOLUME),
            )
            .await
        {
            warn!("error beep failed: {:?}", value);
        }
        Timer::after_millis(70).await;
    }
}

async fn fail_forever<E: defmt::Format>(message: &str, value: E) -> ! {
    error!("{}: {:?}", message, value);
    loop {
        INDICATION_COMMANDS.send(IndicationCommand::Error).await;
        Timer::after_secs(10).await;
    }
}
