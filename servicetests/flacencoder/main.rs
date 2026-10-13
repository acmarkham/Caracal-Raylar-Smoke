//! Writes deterministic mono FLAC fixtures to SD for desktop round-trip tests.
#![no_std]
#![no_main]
extern crate alloc;

#[path = "../storage/common.rs"]
#[allow(dead_code)]
mod common;

use defmt::{info, unwrap};
use embassy_executor::Spawner;
use embassy_time::{Instant, Timer};
use embedded_alloc::LlffHeap;
use raylar_board_v1p0::Board;
use raylar_flac_encoder_service::{Config, Encoder};
use raylar_storage_service::{StorageLayout, StorageService};
use raylar_time_service::UtcTimestamp;
use static_cell::ConstStaticCell;
use {defmt_rtt as _, panic_probe as _};

// As in aardwolf, leave the allocator empty: an unexpected allocation fails.
#[global_allocator]
static HEAP: LlffHeap = LlffHeap::empty();
static PCM: ConstStaticCell<[i32; 4096]> = ConstStaticCell::new([0; 4096]);
static FRAME: ConstStaticCell<[u8; 16416]> = ConstStaticCell::new([0; 16416]);

#[embassy_executor::main]
async fn main(spawner: Spawner) -> ! {
    let board = Board::new(embassy_stm32::init(common::mcu_config()));
    let backend = common::storage_driver(board.sd).await;
    let mut storage = unwrap!(StorageService::<_, _>::new(
        backend,
        &common::TIME_RESOURCES
    ));
    unwrap!(storage.mount().await);
    common::start_time(spawner, board.gps).await;
    while common::TIME_RESOURCES.current_utc().is_err() {
        Timer::after_secs(1).await;
    }
    let pcm = PCM.take();
    let output = FRAME.take();
    for depth in [8u8, 16, 24, 32] {
        for block in [1024usize, 2048, 4096] {
            // Unique second timestamps retain storage's standard naming contract.
            let now = unwrap!(common::TIME_RESOURCES.current_utc());
            let utc = unwrap!(UtcTimestamp::new(now.seconds + 1, 0));
            Timer::after_secs(1).await;
            let stream = unwrap!(
                storage
                    .begin_new_audio_with_suffix_at(StorageLayout::HourlyFolders, utc, "flac")
                    .await
            );
            let mut encoder = unwrap!(Encoder::new(Config {
                bits_per_sample: depth,
                block_samples: block,
                sample_rate_hz: 16000
            }));
            let length = unwrap!(encoder.header(output, &["COMMENT=synthetic xorshift32 seed 0x12345678, arithmetic shift to configured bit depth; 960000 samples"]));
            unwrap!(storage.write(stream, &output[..length]).await);
            let mut state = 0x12345678u32;
            let mut remaining = 960000usize;
            let mut worst_us = 0;
            while remaining != 0 {
                let n = block.min(remaining);
                for sample in &mut pcm[..n] {
                    state ^= state << 13;
                    state ^= state >> 17;
                    state ^= state << 5;
                    *sample = (state as i32) >> (32 - depth);
                }
                let started = Instant::now();
                let frame = unwrap!(encoder.encode(&pcm[..n], output));
                worst_us = worst_us.max(started.elapsed().as_micros());
                unwrap!(storage.write(stream, &output[..frame.bytes]).await);
                unwrap!(encoder.commit(frame));
                remaining -= n;
            }
            unwrap!(storage.rewrite(stream, 8, &encoder.streaminfo()).await);
            unwrap!(storage.finish(stream).await);
            info!("FLAC fixture utc={} depth={} block={} samples={} worst_encode_us={} frame_budget_us={}",
                utc.seconds, depth, block, encoder.samples(), worst_us, block as u64 * 1000000 / 16000);
        }
    }
    info!("FLAC SD fixtures complete; decode on desktop and compare deterministic PCM");
    loop {
        Timer::after_secs(60).await;
    }
}
