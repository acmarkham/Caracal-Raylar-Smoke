// PDM MIC1 ping-pong DMA smoke test.
//
// MIC1 is connected to MDF1 filter 0:
// - PB8: MDF1_CCK0, common clock 0 output, AF5
// - PD3: MDF1_SDI0, serial data input, AF6
//
// The DMA buffer is 16 kSa x 2. Each half is one second at the configured
// 16 kHz output sample rate. Samples are kept as full 32-bit DFLTDR words.

#![no_std]
#![no_main]

use core::ptr;

use defmt::info;
use embassy_executor::Spawner;
use embassy_stm32::dma::{Channel, ReadableRingBuffer, TransferOptions};
use embassy_stm32::gpio::{AfType, Flex, Level, Output, OutputType, Pull, Speed};
use embassy_stm32::pac::{self, rcc::vals::Mdfsel};
use embassy_stm32::rcc::*;
use embassy_stm32::time::mhz;
use embassy_stm32::{bind_interrupts, peripherals};
use raylar_board_v1p0::PdmMic1;
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

const DOVRF: u32 = 1 << 1;
const SATF: u32 = 1 << 9;
const CKABF: u32 = 1 << 10;
const RFOVRF: u32 = 1 << 11;

const MDF1_FLT0_DMA_REQUEST: u8 = 92;
const MDF_BITSTREAM_SELECT: u32 = 0; // 0 = BS0_R, 1 = BS0_F for MDF1_SDI0.
const MDF_CIC_MODE: u32 = 4; // 4 = SINC4, 5 = SINC5 on STM32U5 MDF/ADF.
const MDF_CIC_SCALE: u32 = 0x27;
const MDF_CIC_DECIMATION: u32 = 191;
const MDF_HPF_BYPASS: bool = false;
const SAMPLE_RATE_HZ: usize = 16_000;
const HALF_BUFFER_LEN: usize = SAMPLE_RATE_HZ;

const DMA_BUFFER_LEN: usize = HALF_BUFFER_LEN * 2;

static mut DMA_BUFFER: [u32; DMA_BUFFER_LEN] = [0; DMA_BUFFER_LEN];
static mut PROCESS_BUFFER: [u32; HALF_BUFFER_LEN] = [0; HALF_BUFFER_LEN];

bind_interrupts!(struct Irqs {
    GPDMA1_CHANNEL0 => embassy_stm32::dma::InterruptHandler<peripherals::GPDMA1_CH0>;
});

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

    let pdm_mic1 = PdmMic1 {
        cck0: p.PB8,
        sdio: p.PD3,
    };
    let red = Output::new(p.PB15, Level::Low, Speed::Medium);
    let green = Output::new(p.PD10, Level::Low, Speed::Medium);
    let dma = Channel::new(p.GPDMA1_CH0, Irqs);

    info!("PDM MIC1 DMA smoke test started");
    run_mic_dma_smoke(pdm_mic1, red, green, dma).await
}

async fn run_mic_dma_smoke(
    pdm_mic1: PdmMic1<'static>,
    mut red: Output<'static>,
    mut green: Output<'static>,
    dma: Channel<'static>,
) -> ! {
    configure_mdf_pins(pdm_mic1);
    configure_mdf1_filter0();

    let dma_buffer = unsafe { &mut *core::ptr::addr_of_mut!(DMA_BUFFER) };
    let process_buffer = unsafe { &mut *core::ptr::addr_of_mut!(PROCESS_BUFFER) };

    let mut ring = unsafe {
        ReadableRingBuffer::new(
            dma,
            MDF1_FLT0_DMA_REQUEST,
            (MDF1_BASE + MDF_DFLTDR0) as *mut u32,
            dma_buffer,
            TransferOptions::default(),
        )
    };
    ring.set_alignment(HALF_BUFFER_LEN);
    ring.start();

    enable_mdf1_filter0_dma();

    info!(
        "MDF1 filter0 DMA configured: {} sample ping-pong halves, CCK0 ~= 3.08 MHz, BSSEL={}, SINC{} scale={}, hpf_bypass={}, output ~= 16.0 kHz",
        HALF_BUFFER_LEN,
        MDF_BITSTREAM_SELECT,
        MDF_CIC_MODE,
        MDF_CIC_SCALE,
        MDF_HPF_BYPASS,
    );

    let mut half_count = 0u32;
    let mut overrun_count = 0u32;
    let mut saturation_count = 0u32;
    let mut clock_absence_count = 0u32;
    let mut reshape_overrun_count = 0u32;

    loop {
        match ring.read_exact(process_buffer).await {
            Ok(remaining) => {
                let is_ping = (half_count & 1) == 0;
                set_ping_pong_leds(&mut red, &mut green, is_ping);

                let status = mdf_read(MDF_DFLTISR0);
                if (status & DOVRF) != 0 {
                    overrun_count = overrun_count.wrapping_add(1);
                }
                if (status & SATF) != 0 {
                    saturation_count = saturation_count.wrapping_add(1);
                }
                if (status & CKABF) != 0 {
                    clock_absence_count = clock_absence_count.wrapping_add(1);
                }
                if (status & RFOVRF) != 0 {
                    reshape_overrun_count = reshape_overrun_count.wrapping_add(1);
                }

                let stats = sample_stats(process_buffer);
                let db_cent = rms_power_db_cent(process_buffer);
                let (db_whole, db_frac) = db_parts(db_cent);
                half_count = half_count.wrapping_add(1);

                info!(
                    "MIC1 DMA {} half={} rms_power={}.{=u8:02} dB min={} max={} peak={} nz={} remaining={} overrun={} sat={} ckab={} rfovr={}",
                    if is_ping { "ping" } else { "pong" },
                    half_count,
                    db_whole,
                    db_frac,
                    stats.min,
                    stats.max,
                    stats.peak,
                    stats.nonzero_count,
                    remaining,
                    overrun_count,
                    saturation_count,
                    clock_absence_count,
                    reshape_overrun_count,
                );
                info!(
                    "MIC1 DMA {} first4 {=u32:#010x} {=u32:#010x} {=u32:#010x} {=u32:#010x}",
                    if is_ping { "ping" } else { "pong" },
                    process_buffer[0],
                    process_buffer[1],
                    process_buffer[2],
                    process_buffer[3],
                );
                info!(
                    "MIC1 DMA {} next4 {=u32:#010x} {=u32:#010x} {=u32:#010x} {=u32:#010x}",
                    if is_ping { "ping" } else { "pong" },
                    process_buffer[4],
                    process_buffer[5],
                    process_buffer[6],
                    process_buffer[7],
                );
            }
            Err(e) => {
                info!("MIC1 DMA read error: {=?}", e);
                ring.clear();
            }
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
    let ckgcr = (1 << 0) | (1 << 1) | (1 << 5) | (12 << 16) | (1 << 24);
    mdf_write(MDF_CKGCR, ckgcr);

    // SCKSRC=CCK0, SITFMOD=normal SPI/PDM sampling, STH=4, then enable SITF.
    let sitfcr0 = (1 << 0) | (1 << 4) | (4 << 8);
    mdf_write(MDF_SITFCR0, sitfcr0);

    // PD3 maps to MDF1_SDI0. Try the falling-edge bitstream; BS0_R was railing
    // at 0x7fffff00/0x80000000 with the probed PDM clock/data pair.
    mdf_write(MDF_BSMXCR0, MDF_BITSTREAM_SELECT);

    // DATSRC=BSMX, CICMOD=SINC4, decimation=(191+1)=192, with enough scale
    // headroom to avoid railing while preserving quiet-signal resolution.
    // DMA still reads the full 32-bit DFLTDR word; SCALE affects the 24-bit DR
    // field before it is packed into DFLTDR[31:8].
    let dfltcicr0 = (MDF_CIC_MODE << 4) | (MDF_CIC_DECIMATION << 8) | (MDF_CIC_SCALE << 20);
    mdf_write(MDF_DFLTCICR0, dfltcicr0);

    // Bypass reshape. Enable HPF during audio bring-up to remove the PDM DC
    // component before RMS calculation.
    let dfltrsfr0 = (1 << 0) | ((MDF_HPF_BYPASS as u32) << 7);
    mdf_write(MDF_DFLTRSFR0, dfltrsfr0);
}

fn enable_mdf1_filter0_dma() {
    // DFLTEN | DMAEN.
    mdf_write(MDF_DFLTCR0, (1 << 0) | (1 << 1));
}

fn set_ping_pong_leds(red: &mut Output<'static>, green: &mut Output<'static>, ping: bool) {
    if ping {
        red.set_high();
        green.set_low();
    } else {
        red.set_low();
        green.set_high();
    }
}

fn mdf_read(offset: usize) -> u32 {
    unsafe { ptr::read_volatile((MDF1_BASE + offset) as *const u32) }
}

fn mdf_write(offset: usize, value: u32) {
    unsafe { ptr::write_volatile((MDF1_BASE + offset) as *mut u32, value) }
}

struct SampleStats {
    min: i32,
    max: i32,
    peak: i32,
    nonzero_count: u32,
}

fn sample_stats(samples: &[u32]) -> SampleStats {
    let mut min = i32::MAX;
    let mut max = i32::MIN;
    let mut peak = 0i32;
    let mut nonzero_count = 0u32;

    for &sample in samples {
        let value = dflt_sample(sample);

        min = min.min(value);
        max = max.max(value);
        peak = peak.max(value.saturating_abs());
        if value != 0 {
            nonzero_count = nonzero_count.wrapping_add(1);
        }
    }

    SampleStats {
        min,
        max,
        peak,
        nonzero_count,
    }
}

fn rms_power_db_cent(samples: &[u32]) -> i32 {
    let mut sum_squares = 0u128;

    for &sample in samples {
        let signed = dflt_sample(sample) as i64;
        let square = signed.wrapping_mul(signed) as u128;
        sum_squares = sum_squares.wrapping_add(square);
    }

    if sum_squares == 0 {
        return -99_900;
    }

    let mean_square = sum_squares / samples.len() as u128;
    db_cent_from_power(mean_square)
}

fn db_cent_from_power(power: u128) -> i32 {
    if power == 0 {
        return -99_900;
    }

    let leading_bit = 127 - power.leading_zeros();
    let normalized = power << (127 - leading_bit);
    let frac_q8 = ((normalized >> 119) & 0xff) as u32;
    let log2_q8 = (leading_bit << 8) + frac_q8;

    ((log2_q8 as u64 * 30_103) / 25_600) as i32
}

fn db_parts(db_cent: i32) -> (i32, u8) {
    let whole = db_cent / 100;
    let frac = db_cent.rem_euclid(100) as u8;
    (whole, frac)
}

fn dflt_sample(raw: u32) -> i32 {
    sign_extend_24(raw >> 8)
}

fn sign_extend_24(value: u32) -> i32 {
    ((value << 8) as i32) >> 8
}
