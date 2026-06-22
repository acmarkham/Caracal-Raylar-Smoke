// Ebyte E80 LR1121 RF module smoke test.
//
// SPI interface:
// - PE13 RF_SCK  -> SPI1_SCK
// - PE14 RF_MISO -> SPI1_MISO
// - PE15 RF_MOSI -> SPI1_MOSI
// - PE8  RF_CS   -> manual chip select
//
// Control/status:
// - PE10 RF_BUSY
// - PE11 RF_NRST
// - PE12 RF_IRQ, LR1121 DIO9 interrupt output

#![no_std]
#![no_main]

use defmt::{error, info};
use embassy_executor::Spawner;
use embassy_stm32::gpio::{Input, Output};
use embassy_stm32::mode::Blocking;
use embassy_stm32::rcc::*;
use embassy_stm32::spi::mode::Master;
use embassy_stm32::spi::{Config as SpiConfig, Spi};
use embassy_stm32::time::mhz;
use embassy_time::{Duration, Instant, Timer};
use raylar_board_v1p0::{Board, EbyteRf};
use {defmt_rtt as _, panic_probe as _};

const LR11XX_GET_VERSION: [u8; 2] = [0x01, 0x01];
const BUSY_TIMEOUT: Duration = Duration::from_millis(500);

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

    let p = embassy_stm32::init(config);
    let Board { ebyte_rf, .. } = Board::new(p);

    info!("Ebyte E80 LR1121 RF smoke test started");
    run_rf_smoke(ebyte_rf).await
}

async fn run_rf_smoke(rf: EbyteRf<'static>) -> ! {
    let EbyteRf {
        spi,
        sck,
        miso,
        mosi,
        mut cs,
        busy,
        mut nrst,
        irq: _irq,
    } = rf;

    cs.set_high();

    info!("Resetting LR1121");
    nrst.set_low();
    Timer::after_millis(10).await;
    nrst.set_high();
    Timer::after_millis(10).await;

    if !wait_busy_low(&busy, BUSY_TIMEOUT).await {
        error!("RF_BUSY stayed high after reset");
        pending_forever().await;
    }
    info!("RF_BUSY low after reset");

    let mut spi_config = SpiConfig::default();
    spi_config.frequency = mhz(1);

    let mut spi = Spi::new_blocking(spi, sck, mosi, miso, spi_config);
    let response = get_version(&mut spi, &mut cs, &busy).await;

    info!(
        "GetVersion raw: {:02x} {:02x} {:02x} {:02x} {:02x} {:02x} {:02x} {:02x}",
        response[0],
        response[1],
        response[2],
        response[3],
        response[4],
        response[5],
        response[6],
        response[7],
    );

    info!(
        "Firmware version decode: hw={} type={} fw={}.{}",
        response[2],
        response[3],
        response[4],
        response[5],
    );

    pending_forever().await
}

async fn get_version(
    spi: &mut Spi<'static, Blocking, Master>,
    cs: &mut Output<'static>,
    busy: &Input<'static>,
) -> [u8; 8] {
    let write = [
        LR11XX_GET_VERSION[0],
        LR11XX_GET_VERSION[1],
        0x00,
        0x00,
        0x00,
        0x00,
        0x00,
        0x00,
    ];
    let mut read = [0u8; 8];

    if !wait_busy_low(busy, BUSY_TIMEOUT).await {
        error!("RF_BUSY high before GetVersion");
        pending_forever().await;
    }

    cs.set_low();
    let result = spi.blocking_transfer(&mut read, &write);
    cs.set_high();

    if let Err(e) = result {
        error!("SPI GetVersion transfer failed: {}", e);
        pending_forever().await;
    }

    if !wait_busy_low(busy, BUSY_TIMEOUT).await {
        error!("RF_BUSY high after GetVersion");
        pending_forever().await;
    }

    read
}

async fn wait_busy_low(busy: &Input<'_>, timeout: Duration) -> bool {
    let start = Instant::now();
    while busy.is_high() {
        if Instant::now().duration_since(start) >= timeout {
            return false;
        }
        Timer::after_millis(1).await;
    }
    true
}

async fn pending_forever() -> ! {
    loop {
        Timer::after_secs(60).await;
    }
}
