// PDM all-microphone ping-pong DMA smoke test.
//
// MIC1 is connected to MDF1 CCK0:
// - PB8: MDF1_CCK0, common clock 0 output, AF5
// - PD3: MDF1_SDI0, MIC1 rising edge, AF6
//
// MIC2-MIC6 are connected to MDF1 CCK1:
// - PC2: MDF1_CCK1, common clock 1 output, AF6
// - PD6: MDF1_SDI1, MIC2 rising edge, MIC3 falling edge, AF6
// - PE7: MDF1_SDI2, MIC4 rising edge, MIC5 falling edge, AF6
// - PE4: MDF1_SDI3, MIC6 rising edge, AF6
//
// Each microphone has a ping-pong DMA buffer. Each half is controlled by
// HALF_BUFFER_MS at the configured 16 kHz output sample rate.

#![no_std]
#![no_main]

use core::ptr;
use core::sync::atomic::{fence, Ordering};

use defmt::info;
use embassy_executor::Spawner;
use embassy_stm32::dma::{Channel, ReadableRingBuffer, TransferOptions};
use embassy_stm32::gpio::{AfType, Flex, Level, Output, OutputType, Pull, Speed};
use embassy_stm32::pac::{self, rcc::vals::Mdfsel};
use embassy_stm32::peripherals::{PB8, PC2, PD3, PD6, PE4, PE7};
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
const MDF_DFLTDR0: usize = 0x00f0;

const MIC_COUNT: usize = 6;
const MDF_FILTERS: [usize; MIC_COUNT] = [0, 1, 2, 3, 4, 5];
const MDF_DMA_REQUESTS: [u8; MIC_COUNT] = [92, 93, 94, 95, 96, 97];

// BS0_R, BS1_R, BS1_F, BS2_R, BS2_F, BS3_R.
const MDF_BITSTREAM_SELECTS: [u32; MIC_COUNT] = [0, 2, 3, 4, 5, 6];
const MDF_CIC_MODE: u32 = 4; // 4 = SINC4, 5 = SINC5 on STM32U5 MDF/ADF.
const MDF_CIC_SCALE: u32 = 0x27;
const MDF_CIC_DECIMATION: u32 = 191;
const MDF_HPF_BYPASS: bool = false;

const SAMPLE_RATE_HZ: usize = 16_000;
const HALF_BUFFER_MS: usize = 100;
const HALF_BUFFER_LEN: usize = SAMPLE_RATE_HZ * HALF_BUFFER_MS / 1_000;
const DMA_BUFFER_LEN: usize = HALF_BUFFER_LEN * 2;

#[link_section = ".sram3_dma"]
static mut DMA_BUFFER_MIC1: [u32; DMA_BUFFER_LEN] = [0; DMA_BUFFER_LEN];
#[link_section = ".sram3_dma"]
static mut DMA_BUFFER_MIC2: [u32; DMA_BUFFER_LEN] = [0; DMA_BUFFER_LEN];
#[link_section = ".sram3_dma"]
static mut DMA_BUFFER_MIC3: [u32; DMA_BUFFER_LEN] = [0; DMA_BUFFER_LEN];
#[link_section = ".sram3_dma"]
static mut DMA_BUFFER_MIC4: [u32; DMA_BUFFER_LEN] = [0; DMA_BUFFER_LEN];
#[link_section = ".sram3_dma"]
static mut DMA_BUFFER_MIC5: [u32; DMA_BUFFER_LEN] = [0; DMA_BUFFER_LEN];
#[link_section = ".sram3_dma"]
static mut DMA_BUFFER_MIC6: [u32; DMA_BUFFER_LEN] = [0; DMA_BUFFER_LEN];

static mut SYNC_BUFFER: [u32; HALF_BUFFER_LEN] = [0; HALF_BUFFER_LEN];

bind_interrupts!(struct Irqs {
    GPDMA1_CHANNEL0 => embassy_stm32::dma::InterruptHandler<peripherals::GPDMA1_CH0>;
    GPDMA1_CHANNEL1 => embassy_stm32::dma::InterruptHandler<peripherals::GPDMA1_CH1>;
    GPDMA1_CHANNEL2 => embassy_stm32::dma::InterruptHandler<peripherals::GPDMA1_CH2>;
    GPDMA1_CHANNEL3 => embassy_stm32::dma::InterruptHandler<peripherals::GPDMA1_CH3>;
    GPDMA1_CHANNEL4 => embassy_stm32::dma::InterruptHandler<peripherals::GPDMA1_CH4>;
    GPDMA1_CHANNEL5 => embassy_stm32::dma::InterruptHandler<peripherals::GPDMA1_CH5>;
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
        cck0: p.PB8,
        sd0: p.PD3,
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
        ch5: Channel::new(p.GPDMA1_CH5, Irqs),
    };

    info!("PDM MIC all DMA smoke test started");
    run_mic_all_dma_smoke(pins, red, green, dma).await
}

struct PdmMicArrayPins<'d> {
    cck0: Peri<'d, PB8>,
    sd0: Peri<'d, PD3>,
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
    ch5: Channel<'d>,
}

async fn run_mic_all_dma_smoke(
    pins: PdmMicArrayPins<'static>,
    mut red: Output<'static>,
    mut green: Output<'static>,
    dma: DmaChannels<'static>,
) -> ! {
    configure_mdf_pins(pins);
    configure_mdf1_all();

    let dma_buffer_mic1_ptr = core::ptr::addr_of!(DMA_BUFFER_MIC1) as *const u32;
    let dma_buffer_mic2_ptr = core::ptr::addr_of!(DMA_BUFFER_MIC2) as *const u32;
    let dma_buffer_mic3_ptr = core::ptr::addr_of!(DMA_BUFFER_MIC3) as *const u32;
    let dma_buffer_mic4_ptr = core::ptr::addr_of!(DMA_BUFFER_MIC4) as *const u32;
    let dma_buffer_mic5_ptr = core::ptr::addr_of!(DMA_BUFFER_MIC5) as *const u32;
    let dma_buffer_mic6_ptr = core::ptr::addr_of!(DMA_BUFFER_MIC6) as *const u32;

    let dma_buffer_mic1 = unsafe { &mut *core::ptr::addr_of_mut!(DMA_BUFFER_MIC1) };
    let dma_buffer_mic2 = unsafe { &mut *core::ptr::addr_of_mut!(DMA_BUFFER_MIC2) };
    let dma_buffer_mic3 = unsafe { &mut *core::ptr::addr_of_mut!(DMA_BUFFER_MIC3) };
    let dma_buffer_mic4 = unsafe { &mut *core::ptr::addr_of_mut!(DMA_BUFFER_MIC4) };
    let dma_buffer_mic5 = unsafe { &mut *core::ptr::addr_of_mut!(DMA_BUFFER_MIC5) };
    let dma_buffer_mic6 = unsafe { &mut *core::ptr::addr_of_mut!(DMA_BUFFER_MIC6) };

    let sync_buffer = unsafe { &mut *core::ptr::addr_of_mut!(SYNC_BUFFER) };

    let mut mic1_ring = unsafe {
        ReadableRingBuffer::new(
            dma.ch0,
            MDF_DMA_REQUESTS[0],
            dfltdr_ptr(MDF_FILTERS[0]),
            dma_buffer_mic1,
            TransferOptions::default(),
        )
    };
    let mut mic2_ring = unsafe {
        ReadableRingBuffer::new(
            dma.ch1,
            MDF_DMA_REQUESTS[1],
            dfltdr_ptr(MDF_FILTERS[1]),
            dma_buffer_mic2,
            TransferOptions::default(),
        )
    };
    let mut mic3_ring = unsafe {
        ReadableRingBuffer::new(
            dma.ch2,
            MDF_DMA_REQUESTS[2],
            dfltdr_ptr(MDF_FILTERS[2]),
            dma_buffer_mic3,
            TransferOptions::default(),
        )
    };
    let mut mic4_ring = unsafe {
        ReadableRingBuffer::new(
            dma.ch3,
            MDF_DMA_REQUESTS[3],
            dfltdr_ptr(MDF_FILTERS[3]),
            dma_buffer_mic4,
            TransferOptions::default(),
        )
    };
    let mut mic5_ring = unsafe {
        ReadableRingBuffer::new(
            dma.ch4,
            MDF_DMA_REQUESTS[4],
            dfltdr_ptr(MDF_FILTERS[4]),
            dma_buffer_mic5,
            TransferOptions::default(),
        )
    };
    let mut mic6_ring = unsafe {
        ReadableRingBuffer::new(
            dma.ch5,
            MDF_DMA_REQUESTS[5],
            dfltdr_ptr(MDF_FILTERS[5]),
            dma_buffer_mic6,
            TransferOptions::default(),
        )
    };

    mic1_ring.set_alignment(HALF_BUFFER_LEN);
    mic2_ring.set_alignment(HALF_BUFFER_LEN);
    mic3_ring.set_alignment(HALF_BUFFER_LEN);
    mic4_ring.set_alignment(HALF_BUFFER_LEN);
    mic5_ring.set_alignment(HALF_BUFFER_LEN);
    mic6_ring.set_alignment(HALF_BUFFER_LEN);

    mic1_ring.start();
    mic2_ring.start();
    mic3_ring.start();
    mic4_ring.start();
    mic5_ring.start();
    mic6_ring.start();

    enable_mdf1_all_dma();

    info!(
        "MDF1 all mic DMA configured: {} ms / {} sample halves, CCK0/CCK1 ~= 3.08 MHz, SINC{} scale={}, decim={}, hpf_bypass={}",
        HALF_BUFFER_MS,
        HALF_BUFFER_LEN,
        MDF_CIC_MODE,
        MDF_CIC_SCALE,
        MDF_CIC_DECIMATION + 1,
        MDF_HPF_BYPASS,
    );

    let mut half_count = 0u32;

    loop {
        let remaining = match mic6_ring.read_exact(sync_buffer).await {
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

        let half_offset = if is_ping { 0 } else { HALF_BUFFER_LEN };
        let mic1 = sample_min_max(dma_buffer_mic1_ptr, half_offset, HALF_BUFFER_LEN);
        let mic2 = sample_min_max(dma_buffer_mic2_ptr, half_offset, HALF_BUFFER_LEN);
        let mic3 = sample_min_max(dma_buffer_mic3_ptr, half_offset, HALF_BUFFER_LEN);
        let mic4 = sample_min_max(dma_buffer_mic4_ptr, half_offset, HALF_BUFFER_LEN);
        let mic5 = sample_min_max(dma_buffer_mic5_ptr, half_offset, HALF_BUFFER_LEN);
        let mic6 = sample_min_max(dma_buffer_mic6_ptr, half_offset, HALF_BUFFER_LEN);

        info!(
            "MIC all DMA {} half={} MIC1=[{},{}] MIC2=[{},{}] MIC3=[{},{}] MIC4=[{},{}] MIC5=[{},{}] MIC6=[{},{}] remaining={}",
            if is_ping { "ping" } else { "pong" },
            half_count,
            mic1.min,
            mic1.max,
            mic2.min,
            mic2.max,
            mic3.min,
            mic3.max,
            mic4.min,
            mic4.max,
            mic5.min,
            mic5.max,
            mic6.min,
            mic6.max,
            remaining,
        );
    }
}

fn configure_mdf_pins(pins: PdmMicArrayPins<'static>) {
    let PdmMicArrayPins {
        cck0,
        sd0,
        cck1,
        sd1,
        sd2,
        sd3,
    } = pins;

    let mut cck0 = Flex::new(cck0);
    cck0.set_as_af_unchecked(5, AfType::output(OutputType::PushPull, Speed::VeryHigh));
    core::mem::forget(cck0);

    let mut sd0 = Flex::new(sd0);
    sd0.set_as_af_unchecked(6, AfType::input(Pull::None));
    core::mem::forget(sd0);

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

fn configure_mdf1_all() {
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
    // Use CCK0 for MIC1 and CCK1 for MIC2-MIC6.
    let ckgcr = (1 << 0) | (1 << 1) | (1 << 2) | (1 << 5) | (1 << 6) | (12 << 16) | (1 << 24);
    mdf_write(MDF_CKGCR, ckgcr);

    // Enable serial interface 0 using CCK0 and interfaces 1, 2 and 3 using CCK1.
    let sitfcr_cck0 = (1 << 0) | (1 << 4) | (4 << 8);
    let sitfcr_cck1 = (1 << 0) | (1 << 1) | (1 << 4) | (4 << 8);
    mdf_write(sitfcr_offset(0), sitfcr_cck0);
    mdf_write(sitfcr_offset(1), sitfcr_cck1);
    mdf_write(sitfcr_offset(2), sitfcr_cck1);
    mdf_write(sitfcr_offset(3), sitfcr_cck1);

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

fn enable_mdf1_all_dma() {
    for filter in MDF_FILTERS {
        mdf_write(dfltcr_offset(filter), (1 << 0) | (1 << 1));
    }
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

fn dfltdr_ptr(filter: usize) -> *mut u32 {
    (MDF1_BASE + MDF_DFLTDR0 + filter * MDF_FILTER_STRIDE) as *mut u32
}

fn mdf_write(offset: usize, value: u32) {
    unsafe { ptr::write_volatile((MDF1_BASE + offset) as *mut u32, value) }
}

struct MinMax {
    min: i32,
    max: i32,
}

fn sample_min_max(buffer: *const u32, offset: usize, len: usize) -> MinMax {
    fence(Ordering::Acquire);

    let mut min = i32::MAX;
    let mut max = i32::MIN;

    for index in 0..len {
        let raw = unsafe { ptr::read_volatile(buffer.add(offset + index)) };
        let value = dflt_sample(raw);
        min = min.min(value);
        max = max.max(value);
    }

    MinMax { min, max }
}

fn dflt_sample(raw: u32) -> i32 {
    sign_extend_24(raw >> 8)
}

fn sign_extend_24(value: u32) -> i32 {
    ((value << 8) as i32) >> 8
}
