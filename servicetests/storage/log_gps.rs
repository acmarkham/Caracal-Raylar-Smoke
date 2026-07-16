#![no_std]
#![no_main]

extern crate alloc;

mod common;

use core::fmt::Write;

use defmt::{error, info};
use embassy_executor::Spawner;
use embassy_time::Timer;
use embedded_alloc::LlffHeap as Heap;
use heapless::String;
use raylar_board_v1p0::Board;
use raylar_storage_service::{StorageConfig, StorageService, StreamType};
use {defmt_rtt as _, panic_probe as _};

const HEAP_BYTES: usize = 64 * 1024;

#[global_allocator]
static HEAP: Heap = Heap::empty();

#[embassy_executor::main]
async fn main(spawner: Spawner) -> ! {
    unsafe {
        embedded_alloc::init!(HEAP, HEAP_BYTES);
    }
    let p = embassy_stm32::init(common::mcu_config());
    let Board { gps, sd, .. } = Board::new(p);
    let driver = common::storage_driver(sd).await;
    info!("storage service phase 1: constructing service");
    let mut storage = match StorageService::<_, _>::new(
        driver,
        &common::TIME_RESOURCES,
        StorageConfig::default(),
    ) {
        Ok(storage) => storage,
        Err(e) => {
            error!("storage service construction failed: {}", e);
            common::pending_forever().await
        }
    };
    info!("storage service phase 1 complete: service constructed");
    info!("storage service phase 2: mounting exFAT filesystem");
    match storage.mount().await {
        Ok(()) => info!("storage service phase 2 complete: exFAT filesystem mounted"),
        Err(e) => {
            error!("storage service mount failed: {}", e);
            common::pending_forever().await
        }
    }
    info!("storage service phase 3: opening /syslog.txt for append");
    let log = match storage.create_stream(StreamType::Log).await {
        Ok(stream) => {
            info!("storage service phase 3 complete: log stream opened");
            stream
        }
        Err(e) => {
            error!("log stream creation failed: {}", e);
            common::pending_forever().await
        }
    };
    info!("storage service phase 4: creating GPS timing stream");
    let gps_timing = match storage.create_stream(StreamType::GpsTiming).await {
        Ok(stream) => {
            info!("storage service phase 4 complete: GPS timing stream created");
            stream
        }
        Err(e) => {
            error!("GPS stream creation failed: {}", e);
            common::pending_forever().await
        }
    };
    info!("storage service phase 5: starting GPS driver and time service");
    common::start_time(spawner, gps).await;
    info!("storage service phase 5 complete: GPS and time tasks started");
    let mut pps = match common::GPS_RESOURCES.pps_receiver() {
        Some(receiver) => receiver,
        None => {
            error!("PPS receiver unavailable");
            common::pending_forever().await
        }
    };

    info!("storage log/GPS service test running");
    let mut seconds = 0u64;
    loop {
        Timer::after_secs(1).await;
        seconds = seconds.saturating_add(1);
        let mut line: String<96> = String::new();
        let _ = writeln!(&mut line, "storage service log tick {seconds}");
        if storage.append(log, line.as_bytes()).await.is_err() {
            error!("log append failed");
        }

        if let Some(event) = pps.try_changed() {
            let mut record: String<128> = String::new();
            info!(
                "GPS PPS: count={} source={} timestamp={} capture_ticks={:?} capture_delta_ticks={:?} capture_hz={:?} system_delta_us={:?}",
                event.pps_count,
                event.timing_source,
                event.timestamp,
                event.capture_ticks,
                event.capture_delta_ticks,
                event.capture_frequency_hz,
                event.delta_time.map(|d| d.as_micros()),
            );
            let _ = writeln!(
                &mut record,
                "pps={} system_us={} capture_ticks={:?}",
                event.pps_count,
                event.timestamp.as_micros(),
                event.capture_ticks,
            );
            if storage.append(gps_timing, record.as_bytes()).await.is_err() {
                error!("GPS timing append failed");
            }
        }

        if seconds.is_multiple_of(10) {
            if storage.flush(log).await.is_err() {
                error!("log flush failed");
            }
            // Before UTC becomes valid the GPS stream intentionally has no file.
            if common::TIME_RESOURCES.current_utc().is_ok()
                && storage.flush(gps_timing).await.is_err()
            {
                error!("GPS flush failed");
            }
        }
    }
}
