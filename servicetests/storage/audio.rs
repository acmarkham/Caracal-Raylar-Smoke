#![no_std]
#![no_main]

extern crate alloc;

mod common;

use defmt::{error, info};
use embassy_executor::Spawner;
use embassy_time::Timer;
use embedded_alloc::LlffHeap as Heap;
use raylar_board_v1p0::Board;
use raylar_storage_service::{RollingPolicy, StorageConfig, StorageService, StreamType};
use {defmt_rtt as _, panic_probe as _};

const HEAP_BYTES: usize = 64 * 1024;
const AUDIO_BYTES_PER_SECOND: usize = 16_000 * 2;

#[global_allocator]
static HEAP: Heap = Heap::empty();

static mut AUDIO: [u8; AUDIO_BYTES_PER_SECOND] = [0; AUDIO_BYTES_PER_SECOND];

#[embassy_executor::main]
async fn main(spawner: Spawner) -> ! {
    unsafe {
        embedded_alloc::init!(HEAP, HEAP_BYTES);
    }
    let p = embassy_stm32::init(common::mcu_config());
    let Board { gps, sd, .. } = Board::new(p);
    let driver = common::storage_driver(sd).await;
    let config = StorageConfig {
        audio: RollingPolicy {
            folder_interval_seconds: 600,
            file_interval_seconds: 60,
            startup_alignment_seconds: 60,
        },
        ..StorageConfig::default()
    };
    let mut storage = match StorageService::<_, _>::new(driver, &common::TIME_RESOURCES, config) {
        Ok(storage) => storage,
        Err(_) => {
            error!("storage service construction failed");
            common::pending_forever().await
        }
    };
    if storage.mount().await.is_err() {
        error!("storage service mount failed");
        common::pending_forever().await;
    }

    common::start_time(spawner, gps).await;
    info!("waiting for valid UTC before audio test");
    while common::TIME_RESOURCES.current_utc().is_err() {
        Timer::after_secs(1).await;
    }
    let audio = match storage.create_stream(StreamType::Audio).await {
        Ok(stream) => stream,
        Err(_) => {
            error!("audio stream creation failed");
            common::pending_forever().await
        }
    };
    let data = unsafe { &mut *core::ptr::addr_of_mut!(AUDIO) };
    for (index, byte) in data.iter_mut().enumerate() {
        *byte = (index as u8).wrapping_mul(17);
    }

    info!("storage audio test running: 16 kHz, 16-bit, minute files, ten minute folders");
    loop {
        if storage.append(audio, data).await.is_err() {
            error!("audio append failed");
        }
        Timer::after_secs(1).await;
    }
}
