// PDM microphone array ping-pong DMA smoke test.
//
// MIC2-MIC6 are connected to MDF1 CCK1:
// - PC2: MDF1_CCK1, common clock 1 output, AF6
// - PD6: MDF1_SDI1, MIC2 rising edge, MIC3 falling edge, AF6
// - PE7: MDF1_SDI2, MIC4 rising edge, MIC5 falling edge, AF6
// - PE4: MDF1_SDI3, MIC6 rising edge, AF6
//
// Each microphone has a 16 kSa ping-pong DMA buffer. Each half is 8 kSa,
// which is 500 ms at the configured 16 kHz output sample rate.

#![no_std]
#![no_main]

use core::ptr;

use defmt::info;
use embassy_executor::Spawner;
use embassy_stm32::dma::{Channel, ReadableRingBuffer, TransferOptions};
use embassy_stm32::gpio::{AfType, Flex, Level, Output, OutputType, Pull, Speed};
use embassy_stm32::pac::{self, rcc::vals::Mdfsel};
use embassy_stm32::peripherals::{PC2, PD6, PE4, PE7};
use embassy_stm32::rcc::*;
use embassy_stm32::time::mhz;
use embassy_stm32::{bind_interrupts, peripherals, Peri};
use {defmt_rtt as _, panic_probe as _};

const MDF1_BASE: usize = 0x4002_5000;
const MDF_GCR: usize = 0x0000;
const MDF_CKGCR: usize = 0x0004;
const MDF_FILTER_STRIDE: usize = 0x0080;
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

const MIC_COUNT: usize = 5;
const MIC_NAMES: [&str; MIC_COUNT] = ["MIC2", "MIC3", "MIC4", "MIC5", "MIC6"];
const MDF_FILTERS: [usize; MIC_COUNT] = [1, 2, 3, 4, 5];
const MDF_DMA_REQUESTS: [u8; MIC_COUNT] = [93, 94, 95, 96, 97];

// BS1_R, BS1_F, BS2_R, BS2_F, BS3_R.
const MDF_BITSTREAM_SELECTS: [u32; MIC_COUNT] = [2, 3, 4, 5, 6];
const MDF_CIC_MODE: u32 = 4; // 4 = SINC4, 5 = SINC5 on STM32U5 MDF/ADF.
const MDF_CIC_SCALE: u32 = 0x27;
const MDF_CIC_DECIMATION: u32 = 191;
const MDF_HPF_BYPASS: bool = false;

const SAMPLE_RATE_HZ: usize = 16_000;
const HALF_BUFFER_LEN: usize = SAMPLE_RATE_HZ / 2;
const DMA_BUFFER_LEN: usize = HALF_BUFFER_LEN * 2;

static mut DMA_BUFFER_MIC2: [u32; DMA_BUFFER_LEN] = [0; DMA_BUFFER_LEN];
static mut DMA_BUFFER_MIC3: [u32; DMA_BUFFER_LEN] = [0; DMA_BUFFER_LEN];
static mut DMA_BUFFER_MIC4: [u32; DMA_BUFFER_LEN] = [0; DMA_BUFFER_LEN];
static mut DMA_BUFFER_MIC5: [u32; DMA_BUFFER_LEN] = [0; DMA_BUFFER_LEN];
static mut DMA_BUFFER_MIC6: [u32; DMA_BUFFER_LEN] = [0; DMA_BUFFER_LEN];

static mut PROCESS_MIC2: [u32; HALF_BUFFER_LEN] = [0; HALF_BUFFER_LEN];
static mut PROCESS_MIC3: [u32; HALF_BUFFER_LEN] = [0; HALF_BUFFER_LEN];
static mut PROCESS_MIC4: [u32; HALF_BUFFER_LEN] = [0; HALF_BUFFER_LEN];
static mut PROCESS_MIC5: [u32; HALF_BUFFER_LEN] = [0; HALF_BUFFER_LEN];
static mut PROCESS_MIC6: [u32; HALF_BUFFER_LEN] = [0; HALF_BUFFER_LEN];

bind_interrupts!(struct Irqs {
    GPDMA1_CHANNEL0 => embassy_stm32::dma::InterruptHandler<peripherals::GPDMA1_CH0>;
    GPDMA1_CHANNEL1 => embassy_stm32::dma::InterruptHandler<peripherals::GPDMA1_CH1>;
    GPDMA1_CHANNEL2 => embassy_stm32::dma::InterruptHandler<peripherals::GPDMA1_CH2>;
    GPDMA1_CHANNEL3 => embassy_stm32::dma::InterruptHandler<peripherals::GPDMA1_CH3>;
    GPDMA1_CHANNEL4 => embassy_stm32::dma::InterruptHandler<peripherals::GPDMA1_CH4>;
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

    let pins = PdmMicArrayPins {
        cck1: p.PC2,
        sd1: p.PD6,
        sd2: p.PE7,
        sd3: p.PE4,
    };
    let red = Output::new(p.PB15, Level::Low, Speed::Medium);
    let green = Output::new(p.PD10, Level::Low, Speed::Medium);
    let dma = DmaChannels {
        ch0: Channel::new(p.GPDMA1_CH0, Irqs),
        ch1: Channel::new(p.GPDMA1_CH1, Irqs),
        ch2: Channel::new(p.GPDMA1_CH2, Irqs),
        ch3: Channel::new(p.GPDMA1_CH3, Irqs),
        ch4: Channel::new(p.GPDMA1_CH4, Irqs),
    };

    info!("PDM MIC array DMA smoke test started");
    run_mic_array_dma_smoke(pins, red, green, dma).await
}

struct PdmMicArrayPins<'d> {
    cck1: Peri<'d, PC2>,
    sd1: Peri<'d, PD6>,
    sd2: Peri<'d, PE7>,
    sd3: Peri<'d, PE4>,
}

struct DmaChannels<'d> {
    ch0: Channel<'d>,
    ch1: Channel<'d>,
    ch2: Channel<'d>,
    ch3: Channel<'d>,
    ch4: Channel<'d>,
}

async fn run_mic_array_dma_smoke(
    pins: PdmMicArrayPins<'static>,
    mut red: Output<'static>,
    mut green: Output<'static>,
    dma: DmaChannels<'static>,
) -> ! {
    configure_mdf_pins(pins);
    configure_mdf1_array();

    let dma_buffer_mic2 = unsafe { &mut *core::ptr::addr_of_mut!(DMA_BUFFER_MIC2) };
    let dma_buffer_mic3 = unsafe { &mut *core::ptr::addr_of_mut!(DMA_BUFFER_MIC3) };
    let dma_buffer_mic4 = unsafe { &mut *core::ptr::addr_of_mut!(DMA_BUFFER_MIC4) };
    let dma_buffer_mic5 = unsafe { &mut *core::ptr::addr_of_mut!(DMA_BUFFER_MIC5) };
    let dma_buffer_mic6 = unsafe { &mut *core::ptr::addr_of_mut!(DMA_BUFFER_MIC6) };

    let process_mic2 = unsafe { &mut *core::ptr::addr_of_mut!(PROCESS_MIC2) };
    let process_mic3 = unsafe { &mut *core::ptr::addr_of_mut!(PROCESS_MIC3) };
    let process_mic4 = unsafe { &mut *core::ptr::addr_of_mut!(PROCESS_MIC4) };
    let process_mic5 = unsafe { &mut *core::ptr::addr_of_mut!(PROCESS_MIC5) };
    let process_mic6 = unsafe { &mut *core::ptr::addr_of_mut!(PROCESS_MIC6) };

    let mut mic2_ring = unsafe {
        ReadableRingBuffer::new(dma.ch0, MDF_DMA_REQUESTS[0], dfltdr_ptr(MDF_FILTERS[0]), dma_buffer_mic2, TransferOptions::default())
    };
    let mut mic3_ring = unsafe {
        ReadableRingBuffer::new(dma.ch1, MDF_DMA_REQUESTS[1], dfltdr_ptr(MDF_FILTERS[1]), dma_buffer_mic3, TransferOptions::default())
    };
    let mut mic4_ring = unsafe {
        ReadableRingBuffer::new(dma.ch2, MDF_DMA_REQUESTS[2], dfltdr_ptr(MDF_FILTERS[2]), dma_buffer_mic4, TransferOptions::default())
    };
    let mut mic5_ring = unsafe {
        ReadableRingBuffer::new(dma.ch3, MDF_DMA_REQUESTS[3], dfltdr_ptr(MDF_FILTERS[3]), dma_buffer_mic5, TransferOptions::default())
    };
    let mut mic6_ring = unsafe {
        ReadableRingBuffer::new(dma.ch4, MDF_DMA_REQUESTS[4], dfltdr_ptr(MDF_FILTERS[4]), dma_buffer_mic6, TransferOptions::default())
    };

    mic2_ring.set_alignment(HALF_BUFFER_LEN);
    mic3_ring.set_alignment(HALF_BUFFER_LEN);
    mic4_ring.set_alignment(HALF_BUFFER_LEN);
    mic5_ring.set_alignment(HALF_BUFFER_LEN);
    mic6_ring.set_alignment(HALF_BUFFER_LEN);

    mic2_ring.start();
    mic3_ring.start();
    mic4_ring.start();
    mic5_ring.start();
    mic6_ring.start();

    enable_mdf1_array_dma();

    info!(
        "MDF1 mic array DMA configured: {} sample halves, CCK1 ~= 3.08 MHz, SINC{} scale={}, decim={}, hpf_bypass={}",
        HALF_BUFFER_LEN,
        MDF_CIC_MODE,
        MDF_CIC_SCALE,
        MDF_CIC_DECIMATION + 1,
        MDF_HPF_BYPASS,
    );

    let mut half_count = 0u32;
    let mut counters = [StatusCounters::default(); MIC_COUNT];

    loop {
        let rem2 = match mic2_ring.read_exact(process_mic2).await {
            Ok(remaining) => remaining,
            Err(e) => {
                info!("MIC2 DMA read error: {=?}", e);
                mic2_ring.clear();
                continue;
            }
        };
        let rem3 = match mic3_ring.read_exact(process_mic3).await {
            Ok(remaining) => remaining,
            Err(e) => {
                info!("MIC3 DMA read error: {=?}", e);
                mic3_ring.clear();
                continue;
            }
        };
        let rem4 = match mic4_ring.read_exact(process_mic4).await {
            Ok(remaining) => remaining,
            Err(e) => {
                info!("MIC4 DMA read error: {=?}", e);
                mic4_ring.clear();
                continue;
            }
        };
        let rem5 = match mic5_ring.read_exact(process_mic5).await {
            Ok(remaining) => remaining,
            Err(e) => {
                info!("MIC5 DMA read error: {=?}", e);
                mic5_ring.clear();
                continue;
            }
        };
        let rem6 = match mic6_ring.read_exact(process_mic6).await {
            Ok(remaining) => remaining,
            Err(e) => {
                info!("MIC6 DMA read error: {=?}", e);
                mic6_ring.clear();
                continue;
            }
        };

        let is_ping = (half_count & 1) == 0;
        set_ping_pong_leds(&mut red, &mut green, is_ping);
        half_count = half_count.wrapping_add(1);

        update_status_counters(&mut counters);

        log_mic_stats(MIC_NAMES[0], is_ping, half_count, process_mic2, rem2, counters[0]);
        log_mic_stats(MIC_NAMES[1], is_ping, half_count, process_mic3, rem3, counters[1]);
        log_mic_stats(MIC_NAMES[2], is_ping, half_count, process_mic4, rem4, counters[2]);
        log_mic_stats(MIC_NAMES[3], is_ping, half_count, process_mic5, rem5, counters[3]);
        log_mic_stats(MIC_NAMES[4], is_ping, half_count, process_mic6, rem6, counters[4]);
    }
}

fn configure_mdf_pins(pins: PdmMicArrayPins<'static>) {
    let PdmMicArrayPins { cck1, sd1, sd2, sd3 } = pins;

    let mut cck1 = Flex::new(cck1);
    cck1.set_as_af_unchecked(6, AfType::output(OutputType::PushPull, Speed::VeryHigh));
    core::mem::forget(cck1);

    let mut sd1 = Flex::new(sd1);
    sd1.set_as_af_unchecked(6, AfType::input(Pull::None));
    core::mem::forget(sd1);

    let mut sd2 = Flex::new(sd2);
    sd2.set_as_af_unchecked(6, AfType::input(Pull::None));
    core::mem::forget(sd2);

    let mut sd3 = Flex::new(sd3);
    sd3.set_as_af_unchecked(6, AfType::input(Pull::None));
    core::mem::forget(sd3);
}

fn configure_mdf1_array() {
    let rcc = pac::RCC;

    rcc.ccipr2().modify(|w| w.set_mdf1sel(Mdfsel::HCLK1));
    rcc.ahb1enr().modify(|w| w.set_mdf1en(true));
    rcc.ahb1rstr().modify(|w| w.set_mdf1rst(true));
    rcc.ahb1rstr().modify(|w| w.set_mdf1rst(false));

    mdf_write(MDF_CKGCR, 0);
    mdf_write(MDF_GCR, 0);

    for filter in MDF_FILTERS {
        mdf_write(dfltcr_offset(filter), 0);
        mdf_write(sitfcr_offset(filter), 0);
    }

    // HCLK1 is 80 MHz. PROC_DIV=1 and CCKDIV=13 gives 80 MHz / 2 / 13 = 3.077 MHz.
    // Use CCK1 for the array microphones.
    let ckgcr = (1 << 0) | (1 << 2) | (1 << 6) | (12 << 16) | (1 << 24);
    mdf_write(MDF_CKGCR, ckgcr);

    // Enable serial interfaces 1, 2 and 3 using CCK1 in normal SPI/PDM mode.
    let sitfcr = (1 << 0) | (1 << 1) | (1 << 4) | (4 << 8);
    mdf_write(sitfcr_offset(1), sitfcr);
    mdf_write(sitfcr_offset(2), sitfcr);
    mdf_write(sitfcr_offset(3), sitfcr);

    for index in 0..MIC_COUNT {
        let filter = MDF_FILTERS[index];

        mdf_write(bsmxcr_offset(filter), MDF_BITSTREAM_SELECTS[index]);

        let dfltcicr = (MDF_CIC_MODE << 4) | (MDF_CIC_DECIMATION << 8) | (MDF_CIC_SCALE << 20);
        mdf_write(dfltcicr_offset(filter), dfltcicr);

        // Bypass reshape. Keep HPF enabled unless MDF_HPF_BYPASS is set.
        let dfltrsfr = (1 << 0) | ((MDF_HPF_BYPASS as u32) << 7);
        mdf_write(dfltrsfr_offset(filter), dfltrsfr);
    }
}

fn enable_mdf1_array_dma() {
    for filter in MDF_FILTERS {
        mdf_write(dfltcr_offset(filter), (1 << 0) | (1 << 1));
    }
}

fn update_status_counters(counters: &mut [StatusCounters; MIC_COUNT]) {
    for index in 0..MIC_COUNT {
        let status = mdf_read(dfltisr_offset(MDF_FILTERS[index]));
        counters[index].update(status);
    }
}

fn log_mic_stats(
    name: &str,
    is_ping: bool,
    half_count: u32,
    samples: &[u32],
    remaining: usize,
    counters: StatusCounters,
) {
    let stats = sample_stats(samples);
    let (snr_whole, snr_frac) = db_parts(stats.snr_db_cent);
    let (rms_whole, rms_frac) = db_parts(stats.rms_db_cent);

    info!(
        "{} DMA {} half={} snr={}.{=u8:02} dB rms={}.{=u8:02} dB min={} max={} peak={} nz={} remaining={} overrun={} sat={} ckab={} rfovr={}",
        name,
        if is_ping { "ping" } else { "pong" },
        half_count,
        snr_whole,
        snr_frac,
        rms_whole,
        rms_frac,
        stats.min,
        stats.max,
        stats.peak,
        stats.nonzero_count,
        remaining,
        counters.overrun,
        counters.saturation,
        counters.clock_absence,
        counters.reshape_overrun,
    );
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

fn sitfcr_offset(filter: usize) -> usize {
    MDF_SITFCR0 + filter * MDF_FILTER_STRIDE
}

fn bsmxcr_offset(filter: usize) -> usize {
    MDF_BSMXCR0 + filter * MDF_FILTER_STRIDE
}

fn dfltcr_offset(filter: usize) -> usize {
    MDF_DFLTCR0 + filter * MDF_FILTER_STRIDE
}

fn dfltcicr_offset(filter: usize) -> usize {
    MDF_DFLTCICR0 + filter * MDF_FILTER_STRIDE
}

fn dfltrsfr_offset(filter: usize) -> usize {
    MDF_DFLTRSFR0 + filter * MDF_FILTER_STRIDE
}

fn dfltisr_offset(filter: usize) -> usize {
    MDF_DFLTISR0 + filter * MDF_FILTER_STRIDE
}

fn dfltdr_ptr(filter: usize) -> *mut u32 {
    (MDF1_BASE + MDF_DFLTDR0 + filter * MDF_FILTER_STRIDE) as *mut u32
}

fn mdf_read(offset: usize) -> u32 {
    unsafe { ptr::read_volatile((MDF1_BASE + offset) as *const u32) }
}

fn mdf_write(offset: usize, value: u32) {
    unsafe { ptr::write_volatile((MDF1_BASE + offset) as *mut u32, value) }
}

#[derive(Clone, Copy, Default)]
struct StatusCounters {
    overrun: u32,
    saturation: u32,
    clock_absence: u32,
    reshape_overrun: u32,
}

impl StatusCounters {
    fn update(&mut self, status: u32) {
        if (status & DOVRF) != 0 {
            self.overrun = self.overrun.wrapping_add(1);
        }
        if (status & SATF) != 0 {
            self.saturation = self.saturation.wrapping_add(1);
        }
        if (status & CKABF) != 0 {
            self.clock_absence = self.clock_absence.wrapping_add(1);
        }
        if (status & RFOVRF) != 0 {
            self.reshape_overrun = self.reshape_overrun.wrapping_add(1);
        }
    }
}

struct SampleStats {
    min: i32,
    max: i32,
    peak: i32,
    nonzero_count: u32,
    rms_db_cent: i32,
    snr_db_cent: i32,
}

fn sample_stats(samples: &[u32]) -> SampleStats {
    let mut min = i32::MAX;
    let mut max = i32::MIN;
    let mut peak = 0i32;
    let mut nonzero_count = 0u32;
    let mut total_sum_squares = 0u128;
    let mut noise_power = u128::MAX;
    let mut signal_power = 0u128;

    let chunk_len = samples.len() / 8;
    let mut chunk_sum_squares = 0u128;
    let mut chunk_count = 0usize;

    for &sample in samples {
        let value = dflt_sample(sample);
        let square = (value as i64).wrapping_mul(value as i64) as u128;

        min = min.min(value);
        max = max.max(value);
        peak = peak.max(value.saturating_abs());
        if value != 0 {
            nonzero_count = nonzero_count.wrapping_add(1);
        }

        total_sum_squares = total_sum_squares.wrapping_add(square);
        chunk_sum_squares = chunk_sum_squares.wrapping_add(square);
        chunk_count += 1;

        if chunk_count == chunk_len {
            let power = chunk_sum_squares / chunk_len as u128;
            noise_power = noise_power.min(power);
            signal_power = signal_power.max(power);
            chunk_sum_squares = 0;
            chunk_count = 0;
        }
    }

    let rms_power = total_sum_squares / samples.len() as u128;
    let rms_db_cent = db_cent_from_power(rms_power);
    let snr_db_cent = snr_db_cent(signal_power, noise_power);

    SampleStats {
        min,
        max,
        peak,
        nonzero_count,
        rms_db_cent,
        snr_db_cent,
    }
}

fn snr_db_cent(signal_power: u128, noise_power: u128) -> i32 {
    if signal_power == 0 {
        0
    } else if noise_power == 0 || noise_power == u128::MAX {
        99_900
    } else {
        db_cent_from_power(signal_power) - db_cent_from_power(noise_power)
    }
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
