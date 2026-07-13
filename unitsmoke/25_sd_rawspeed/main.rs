// SDMMC1 raw sustained write speed test.
//
// Destructive test: writes raw blocks starting at block 0. Do not run this on a
// card containing a filesystem or data you want to keep.

#![no_std]
#![no_main]

use defmt::{error, info};
use embassy_executor::Spawner;
use embassy_stm32::gpio::Input;
use embassy_stm32::rcc::mux::Sdmmcsel;
use embassy_stm32::rcc::*;
use embassy_stm32::sdmmc::sd::{CmdBlock, DataBlock, StorageDevice};
use embassy_stm32::sdmmc::{Config as SdmmcConfig, Sdmmc};
use embassy_stm32::time::{mhz, Hertz};
use embassy_time::{Instant, Timer};
use raylar_board_v1p0::{Board, Irqs, SdCard};
use sdio_host::sd::CardCapacity;
use {defmt_rtt as _, panic_probe as _};

const SD_TARGET_FREQ: Hertz = mhz(24);
const BLOCK_BYTES: usize = 512;
const TEST_MBYTES: usize = 10;
const TEST_BYTES: usize = TEST_MBYTES * 1024 * 1024;
const TEST_BLOCKS: usize = TEST_BYTES / BLOCK_BYTES;
const START_BLOCK: u32 = 0;
const MAX_BURST_BYTES: usize = 262_144;
const MAX_BURST_BLOCKS: usize = MAX_BURST_BYTES / BLOCK_BYTES;
const BURST_BYTES: [usize; 9] = [512, 1024, 4096, 8192, 16384, 32768, 65536, 131072, 262144];

static mut WRITE_BLOCKS: [DataBlock; MAX_BURST_BLOCKS] =
    [const { DataBlock::new() }; MAX_BURST_BLOCKS];

#[embassy_executor::main]
async fn main(_spawner: Spawner) -> ! {
    let mut config = embassy_stm32::Config::default();

    config.rcc.hse = Some(Hse {
        freq: mhz(16),
        mode: HseMode::Oscillator,
    });

    config.rcc.pll1 = Some(Pll {
        source: PllSource::HSE,
        prediv: PllPreDiv::DIV1,
        mul: PllMul::MUL18,
        divp: Some(PllDiv::DIV6), // 48 MHz SDMMC kernel clock through PLL1_P.
        divq: Some(PllDiv::DIV2),
        divr: Some(PllDiv::DIV2), // 144 MHz SYSCLK.
    });

    config.rcc.sys = Sysclk::PLL1_R;
    config.rcc.hsi48 = Some(Hsi48Config::new());
    config.rcc.mux.sdmmcsel = Sdmmcsel::PLL1_P;

    let p = embassy_stm32::init(config);
    let Board { sd, .. } = Board::new(p);

    info!("SDMMC1 raw sustained write speed test started");
    info!(
        "DESTRUCTIVE: writing {} bytes from raw block {}",
        TEST_BYTES, START_BLOCK
    );

    run_sd_rawspeed(sd).await
}

async fn run_sd_rawspeed(mut sd: SdCard<'static>) -> ! {
    sd.power.set_high();

    if card_absent(&sd.switch) {
        info!("Card absent: SD_SW is high, leaving SD power off");
        pending_forever().await;
    }

    info!("Card present: SD_SW is low");
    let mut sdmmc_config = SdmmcConfig::default();
    sdmmc_config.data_transfer_timeout = 120_000_000;
    let mut sdmmc = Sdmmc::new_4bit(
        sd.sdmmc,
        Irqs,
        sd.clk,
        sd.cmd,
        sd.d0,
        sd.d1,
        sd.d2,
        sd.d3,
        sdmmc_config,
    );
    let mut cmd_block = CmdBlock::new();

    info!("Turning SD power on");
    sd.power.set_low();
    Timer::after_secs(1).await;

    let mut card =
        match StorageDevice::new_sd_card(&mut sdmmc, &mut cmd_block, SD_TARGET_FREQ).await {
            Ok(card) => card,
            Err(e) => {
                error!("SD card init failed: {}", e);
                pending_forever().await;
            }
        };

    log_card_info(&card);
    ensure_capacity(&card).await;

    let write_blocks = unsafe { &mut *core::ptr::addr_of_mut!(WRITE_BLOCKS) };
    fill_dummy_data(write_blocks);

    info!(
        "Starting raw write benchmark: {} blocks, {} bytes",
        TEST_BLOCKS, TEST_BYTES
    );
    for burst_bytes in BURST_BYTES {
        benchmark_burst(&mut card, burst_bytes, write_blocks).await;
        Timer::after_millis(250).await;
    }

    info!("SDMMC1 raw sustained write speed test complete");
    pending_forever().await
}

async fn benchmark_burst(
    card: &mut StorageDevice<'_, '_, embassy_stm32::sdmmc::sd::Card>,
    burst_bytes: usize,
    write_blocks: &[DataBlock; MAX_BURST_BLOCKS],
) {
    let burst_blocks = burst_bytes / BLOCK_BYTES;
    let burst = &write_blocks[..burst_blocks];
    let mut block_idx = START_BLOCK;
    let mut remaining_blocks = TEST_BLOCKS;

    info!(
        "Burst {} bytes: writing {} bytes as {} transfers of {} blocks",
        burst_bytes,
        TEST_BYTES,
        TEST_BLOCKS / burst_blocks,
        burst_blocks,
    );

    let started = Instant::now();
    while remaining_blocks > 0 {
        if let Err(e) = card.write_blocks(block_idx, burst).await {
            error!(
                "Write failed at block {} burst_bytes={}: {}",
                block_idx, burst_bytes, e
            );
            pending_forever().await;
        }

        block_idx += burst_blocks as u32;
        remaining_blocks -= burst_blocks;
    }
    let elapsed = Instant::now().duration_since(started);
    let elapsed_us = elapsed.as_micros().max(1);
    let speed_x10 = ((TEST_BYTES as u64) * 10 * 1_000_000) / (1024 * 1024) / elapsed_us;

    info!(
        "RESULT burst_bytes={} elapsed_ms={} elapsed_us={} speed={}.{} Mbytes/sec",
        burst_bytes,
        elapsed.as_millis(),
        elapsed_us,
        speed_x10 / 10,
        speed_x10 % 10,
    );
}

fn log_card_info(card: &StorageDevice<'_, '_, embassy_stm32::sdmmc::sd::Card>) {
    let info = card.card();
    let capacity_kind = match info.card_type {
        CardCapacity::StandardCapacity => "SDSC",
        CardCapacity::HighCapacity => "SDHC/SDXC",
        _ => "unknown",
    };

    info!(
        "Card initialized: type={} rca={} blocks={} csd_bytes={} size_bytes={}",
        capacity_kind,
        info.rca,
        info.csd.block_count(),
        info.csd.card_size(),
        info.csd.card_size(),
    );
    info!(
        "CID: mid={} oem={} product={} rev={} serial={} mfg_month={} mfg_year={}",
        info.cid.manufacturer_id(),
        info.cid.oem_id(),
        info.cid.product_name(),
        info.cid.product_revision(),
        info.cid.serial(),
        info.cid.manufacturing_date().0,
        info.cid.manufacturing_date().1,
    );
}

async fn ensure_capacity(card: &StorageDevice<'_, '_, embassy_stm32::sdmmc::sd::Card>) {
    let required_blocks = START_BLOCK as usize + TEST_BLOCKS;
    let available_blocks = card.card().csd.block_count() as usize;

    if available_blocks < required_blocks {
        error!(
            "Card too small: available_blocks={} required_blocks={}",
            available_blocks, required_blocks
        );
        pending_forever().await;
    }
}

fn card_absent(switch: &Input<'_>) -> bool {
    switch.is_high()
}

fn fill_dummy_data(blocks: &mut [DataBlock]) {
    for (block_idx, block) in blocks.iter_mut().enumerate() {
        for (word_idx, word) in block.0.iter_mut().enumerate() {
            *word = 0xA5A5_0000 ^ ((block_idx as u32) << 16) ^ word_idx as u32;
        }
    }
}

async fn pending_forever() -> ! {
    loop {
        Timer::after_secs(60).await;
    }
}
