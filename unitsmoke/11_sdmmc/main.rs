// SDMMC1 microSD smoke test.
//
// Uses SDMMC1 default 4-bit pins:
// - PC12 SDMMC1_CK
// - PD2  SDMMC1_CMD
// - PC8  SDMMC1_D0
// - PC9  SDMMC1_D1
// - PC10 SDMMC1_D2
// - PC11 SDMMC1_D3
//
// Board control pins:
// - PD4 SD_SW, high when no card is present, low when inserted
// - PE0 SD_PWR, active low P-channel FET gate

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
use embassy_time::Timer;
use raylar_board_v1p0::{Board, Irqs, SdCard};
use sdio_host::sd::CardCapacity;
use {defmt_rtt as _, panic_probe as _};

const SD_TARGET_FREQ: Hertz = mhz(24);

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
        mul: PllMul::MUL10,
        divp: Some(PllDiv::DIV1),
        divq: Some(PllDiv::DIV2),
        divr: Some(PllDiv::DIV2),
    });

    config.rcc.sys = Sysclk::PLL1_R;
    config.rcc.hsi48 = Some(Hsi48Config::new());
    config.rcc.mux.sdmmcsel = Sdmmcsel::ICLK;

    let p = embassy_stm32::init(config);
    let Board { sd, .. } = Board::new(p);

    info!("SDMMC1 microSD smoke test started");
    info!("WARNING: this test overwrites block 0 twice");

    run_sd_smoke(sd).await
}

async fn run_sd_smoke(mut sd: SdCard<'static>) -> ! {
    sd.power.set_high();

    if card_absent(&sd.switch) {
        info!("Card absent: SD_SW is high, leaving SD power off");
        pending_forever().await;
    }

    info!("Card present: SD_SW is low");
    // init
        let mut sdmmc = Sdmmc::new_4bit(
        sd.sdmmc,
        Irqs,
        sd.clk,
        sd.cmd,
        sd.d0,
        sd.d1,
        sd.d2,
        sd.d3,
        SdmmcConfig::default(),
    );
    let mut cmd_block = CmdBlock::new();

    info!("Card is off - should not respond to commands");
    let _card = match StorageDevice::new_sd_card(&mut sdmmc, &mut cmd_block, SD_TARGET_FREQ).await {
        Ok(_card) => {
            error!("SD card init succeeded when it should have been off");
            pending_forever().await;
        },
        Err(e) => {
            info!("SD card init cannot cannot be completed (correct behavior when card is off): {}", e);
        }
    };
    // Now turn the card on properly and try again
    info!("Turning SD power on");
    sd.power.set_low();
    Timer::after_secs(1).await;

    let mut card = match StorageDevice::new_sd_card(&mut sdmmc, &mut cmd_block, SD_TARGET_FREQ).await {
        Ok(card) => card,
        Err(e) => {
            error!("SD card init failed: {}", e);
            pending_forever().await;
        }
    };

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

    write_read_validate(&mut card, 0, Pattern::First).await;
    write_read_validate(&mut card, 0, Pattern::Second).await;

    info!("SDMMC1 block 0 write/read smoke test complete");
    pending_forever().await
}

async fn write_read_validate(
    card: &mut StorageDevice<'_, '_, embassy_stm32::sdmmc::sd::Card>,
    block_idx: u32,
    pattern: Pattern,
) {
    let mut write_block = DataBlock::new();
    fill_pattern(&mut write_block, pattern);

    info!("Writing block {} with pattern {}", block_idx, pattern.name());
    if let Err(e) = card.write_block(block_idx, &write_block).await {
        error!("Write block {} failed: {}", block_idx, e);
        pending_forever().await;
    }

    let mut read_block = DataBlock::new();
    info!("Reading block {} back", block_idx);
    if let Err(e) = card.read_block(block_idx, &mut read_block).await {
        error!("Read block {} failed: {}", block_idx, e);
        pending_forever().await;
    }

    if write_block.0 == read_block.0 {
        info!("Validated block {} pattern {}", block_idx, pattern.name());
        return;
    }

    let mut mismatch = 0usize;
    while mismatch < write_block.0.len() {
        if write_block.0[mismatch] != read_block.0[mismatch] {
            break;
        }
        mismatch += 1;
    }

    error!(
        "Validation failed block {} pattern {} word={} expected={=u32:#010x} actual={=u32:#010x}",
        block_idx,
        pattern.name(),
        mismatch,
        write_block.0[mismatch],
        read_block.0[mismatch],
    );
    pending_forever().await;
}

fn card_absent(switch: &Input<'_>) -> bool {
    switch.is_high()
}

fn fill_pattern(block: &mut DataBlock, pattern: Pattern) {
    for (idx, word) in block.0.iter_mut().enumerate() {
        let idx = idx as u32;
        *word = match pattern {
            Pattern::First => 0xA5A5_0000 ^ idx.rotate_left(5),
            Pattern::Second => 0x5A5A_FF00 ^ idx.wrapping_mul(0x0101_0101),
        };
    }
}

#[derive(Clone, Copy)]
enum Pattern {
    First,
    Second,
}

impl Pattern {
    fn name(self) -> &'static str {
        match self {
            Pattern::First => "first",
            Pattern::Second => "second",
        }
    }
}

async fn pending_forever() -> ! {
    loop {
        Timer::after_secs(60).await;
    }
}
