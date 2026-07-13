// PDM MIC1 mono smoke test.
//
// MIC1 is connected to MDF1 filter 0:
// - PB8: MDF1_CCK0, common clock 0 output, AF5
// - PD3: MDF1_SDI0, serial data input, AF6
//
// This is intentionally a polling smoke test. It configures a roughly 3.08 MHz
// PDM clock from the 80 MHz MDF kernel clock and uses SINC5 with decimation
// 192, giving about 16.0 kHz output samples. It logs every 2000th sample.

#![no_std]
#![no_main]

use core::ptr;

use defmt::info;
use embassy_executor::Spawner;
use embassy_stm32::gpio::{AfType, Flex, OutputType, Pull, Speed};
use embassy_stm32::pac::{self, rcc::vals::Mdfsel};
use embassy_stm32::rcc::*;
use embassy_stm32::time::mhz;
use embassy_time::{Duration, Timer};
use raylar_board_v1p0::{Board, PdmMic1};
use {defmt_rtt as _, panic_probe as _};

const MDF1_BASE: usize = 0x4002_5000;
const MDF_GCR: usize = 0x0000;
const MDF_CKGCR: usize = 0x0004;
const MDF_SITFCR0: usize = 0x0080;
const MDF_BSMXCR0: usize = 0x0084;
const MDF_DFLTCR0: usize = 0x0088;
const MDF_DFLTCICR0: usize = 0x008c;
const MDF_DFLTRSFR0: usize = 0x0090;
const MDF_DFLTISR0: usize = 0x00b0;
const MDF_DFLTDR0: usize = 0x00f0;

const RXNEF: u32 = 1 << 3;
const DOVRF: u32 = 1 << 1;
const CKABF: u32 = 1 << 10;

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
    let Board { pdm_mic1, .. } = Board::new(p);

    info!("PDM MIC1 mono smoke test started");
    run_mic_smoke(pdm_mic1).await
}

async fn run_mic_smoke(pdm_mic1: PdmMic1<'static>) -> ! {
    configure_mdf_pins(pdm_mic1);
    configure_mdf1_filter0();

    info!("MDF1 filter0 configured: CCK0 ~= 3.08 MHz, SINC5 decim 192, output ~= 16.0 kHz");

    let mut sample_count = 0u32;
    let mut overrun_count = 0u32;
    let mut clock_absence_count = 0u32;

    loop {
        let isr = mdf_read(MDF_DFLTISR0);

        if (isr & DOVRF) != 0 {
            overrun_count = overrun_count.wrapping_add(1);
        }
        if (isr & CKABF) != 0 {
            clock_absence_count = clock_absence_count.wrapping_add(1);
        }

        if (isr & RXNEF) != 0 {
            let raw = mdf_read(MDF_DFLTDR0);
            let sample = sign_extend_24(raw >> 8);
            sample_count = sample_count.wrapping_add(1);

            if sample_count % 2000 == 0 {
                info!(
                    "MIC1 raw {} sample {} value={} overrun={} ckab={}",
                    raw, sample_count, sample, overrun_count, clock_absence_count,
                );
            }
        } else {
            Timer::after(Duration::from_micros(50)).await;
        }
    }
}

fn configure_mdf_pins(pdm_mic1: PdmMic1<'static>) {
    let PdmMic1 { cck0, sdio } = pdm_mic1;

    let mut cck0 = Flex::new(cck0);
    cck0.set_as_af_unchecked(5, AfType::output(OutputType::PushPull, Speed::VeryHigh));
    core::mem::forget(cck0);

    let mut sdio = Flex::new(sdio);
    sdio.set_as_af_unchecked(6, AfType::input(Pull::None));
    core::mem::forget(sdio);
}

fn configure_mdf1_filter0() {
    let rcc = pac::RCC;

    rcc.ccipr2().modify(|w| w.set_mdf1sel(Mdfsel::HCLK1));
    rcc.ahb1enr().modify(|w| w.set_mdf1en(true));
    rcc.ahb1rstr().modify(|w| w.set_mdf1rst(true));
    rcc.ahb1rstr().modify(|w| w.set_mdf1rst(false));

    mdf_write(MDF_DFLTCR0, 0);
    mdf_write(MDF_SITFCR0, 0);
    mdf_write(MDF_CKGCR, 0);
    mdf_write(MDF_GCR, 0);

    // HCLK1 is 80 MHz. PROC_DIV=1 and CCKDIV=13 gives 80 MHz / 2 / 13 = 3.077 MHz.
    // Enable CCK0 output and the clock generator immediately.
    let ckgcr = (1 << 0) | (1 << 1) | (1 << 5) | (12 << 16) | (1 << 24);
    mdf_write(MDF_CKGCR, ckgcr);

    // SCKSRC=CCK0, SITFMOD=normal SPI/PDM sampling, STH=4, then enable SITF.
    let sitfcr0 = (1 << 0) | (1 << 4) | (4 << 8);
    mdf_write(MDF_SITFCR0, sitfcr0);

    // PD3 maps to MDF1_SDI0. SEL is pulled high, so use the rising-edge stream.
    mdf_write(MDF_BSMXCR0, 0);

    // DATSRC=BSMX, CICMOD=SINC5, decimation=(191+1)=192, modest right shift.
    let dfltcicr0 = (5 << 4) | (191 << 8) | (8 << 20);
    mdf_write(MDF_DFLTCICR0, dfltcicr0);

    // Bypass reshape and high-pass filters for a simple bring-up path.
    mdf_write(MDF_DFLTRSFR0, (1 << 0) | (1 << 7));

    // RXFIFO threshold not-empty, asynchronous continuous acquisition, enable filter.
    mdf_write(MDF_DFLTCR0, 1 << 0);
}

fn mdf_read(offset: usize) -> u32 {
    unsafe { ptr::read_volatile((MDF1_BASE + offset) as *const u32) }
}

fn mdf_write(offset: usize, value: u32) {
    unsafe { ptr::write_volatile((MDF1_BASE + offset) as *mut u32, value) }
}

fn sign_extend_24(value: u32) -> i32 {
    ((value << 8) as i32) >> 8
}
