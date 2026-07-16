#![no_std]
#![no_main]

extern crate alloc;

mod hardware;
mod producers;

use defmt::{error, info, unwrap};
use embassy_executor::Spawner;
use embassy_stm32::sdmmc::sd::{CmdBlock, StorageDevice};
use embassy_stm32::sdmmc::{Config as SdmmcConfig, Sdmmc};
use embassy_stm32::time::{mhz, Hertz};
use embassy_time::{Duration, Instant, Timer};
use embedded_alloc::LlffHeap as Heap;
use raylar_board_v1p0::{Board, Irqs, SdCard};
use raylar_drivers::storage::stm32::Stm32SdBlockDevice;
use raylar_drivers::storage::{detect_exfat_volume, PartitionedBlockDevice, StorageDriver};
use raylar_logging_service::{
    info as log_info, LoggerHandle, LoggingResources, LoggingService, ProcessOutcome,
    StorageLogSink,
};
use raylar_storage_service::{StorageConfig, StorageService, UtcClock};
use raylar_time_service::UtcTimestamp;
use {defmt_rtt as _, panic_probe as _};

const HEAP_BYTES: usize = 64 * 1024;
const SD_TARGET_FREQ: Hertz = mhz(24);
const MESSAGE_LENGTH: usize = 128;
const QUEUE_DEPTH: usize = 8;
const LINE_LENGTH: usize = 224;
const FLUSH_INTERVAL: Duration = Duration::from_secs(5);

pub type TestLogger = LoggerHandle<'static, MESSAGE_LENGTH, QUEUE_DEPTH>;

static LOGGING_RESOURCES: LoggingResources<MESSAGE_LENGTH, QUEUE_DEPTH> = LoggingResources::new();

#[global_allocator]
static HEAP: Heap = Heap::empty();

struct NoUtcClock;

impl UtcClock for NoUtcClock {
    fn current_utc(&self) -> Option<UtcTimestamp> {
        None
    }
}

#[embassy_executor::main]
async fn main(spawner: Spawner) -> ! {
    unsafe {
        embedded_alloc::init!(HEAP, HEAP_BYTES);
    }
    let peripherals = embassy_stm32::init(hardware::mcu_config());
    let Board { sd, .. } = Board::new(peripherals);
    run_logging_test(spawner, sd).await
}

async fn run_logging_test(spawner: Spawner, sd: SdCard<'static>) -> ! {
    let SdCard {
        sdmmc,
        clk,
        cmd,
        d0,
        d1,
        d2,
        d3,
        switch,
        mut power,
    } = sd;
    power.set_high();
    if switch.is_high() {
        error!("logging service test requires an SD card");
        hardware::pending_forever().await;
    }

    let mut sd_config = SdmmcConfig::default();
    sd_config.data_transfer_timeout = 120_000_000;
    let mut sdmmc = Sdmmc::new_4bit(sdmmc, Irqs, clk, cmd, d0, d1, d2, d3, sd_config);
    let mut command = CmdBlock::new();
    power.set_low();
    Timer::after_secs(1).await;

    let card = match StorageDevice::new_sd_card(&mut sdmmc, &mut command, SD_TARGET_FREQ).await {
        Ok(card) => card,
        Err(error) => {
            error!("SD card initialization failed: {}", error);
            hardware::pending_forever().await
        }
    };
    let mut device = Stm32SdBlockDevice::new(card);
    let volume = match detect_exfat_volume(&mut device).await {
        Ok(volume) => volume,
        Err(error) => {
            error!("exFAT volume detection failed: {}", error);
            hardware::pending_forever().await
        }
    };
    let driver = StorageDriver::<_>::new(PartitionedBlockDevice::new(device, volume));
    let mut storage =
        match StorageService::<_, _>::new(driver, NoUtcClock, StorageConfig::default()) {
            Ok(storage) => storage,
            Err(error) => {
                error!("storage service construction failed: {}", error);
                hardware::pending_forever().await
            }
        };
    if let Err(error) = storage.mount().await {
        error!("storage mount failed: {}", error);
        hardware::pending_forever().await;
    }

    let sink = match StorageLogSink::open(&mut storage).await {
        Ok(sink) => sink,
        Err(error) => {
            error!("logging stream open failed: {}", error);
            hardware::pending_forever().await
        }
    };
    let mut logging = LoggingService::<_, MESSAGE_LENGTH, QUEUE_DEPTH, LINE_LENGTH>::new(
        &LOGGING_RESOURCES,
        sink,
    );
    let system = logging.register("System");
    let gps = logging.register("GPS");
    let audio = logging.register("Audio");
    let battery = logging.register("Battery");

    spawner.spawn(unwrap!(producers::gps(gps)));
    spawner.spawn(unwrap!(producers::audio(audio)));
    spawner.spawn(unwrap!(producers::battery(battery)));
    let _ = log_info!(
        system,
        "logging service test started with {} producers",
        3u8
    );
    info!("logging to /syslog.txt; flush interval is 5 seconds");

    let mut next_flush = Instant::now() + FLUSH_INTERVAL;
    loop {
        let now = Instant::now();
        if now >= next_flush {
            if let Err(error) = logging.flush().await {
                error!("log flush failed: {}", error);
            }
            let stats = logging.stats();
            info!(
                "logging stats: total={} dropped={} depth={} max_depth={} bytes={} truncated={} write_failures={}",
                stats.total_messages,
                stats.dropped_messages,
                stats.queue_depth,
                stats.maximum_queue_depth,
                stats.bytes_written,
                stats.truncated_messages,
                stats.write_failures,
            );
            next_flush = now + FLUSH_INTERVAL;
        }

        match logging.process_one().await {
            Ok(ProcessOutcome::Written) => {}
            Ok(ProcessOutcome::Empty) => Timer::after_millis(10).await,
            Err(error) => {
                error!("log append failed: {}", error);
                Timer::after_millis(100).await;
            }
        }
    }
}
