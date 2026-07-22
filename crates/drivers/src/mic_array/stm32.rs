//! STM32U595 MDF1/GPDMA implementation.

use core::ptr;
use core::sync::atomic::{fence, AtomicU32, Ordering};

use embassy_stm32::dma::{Channel, ReadableRingBuffer, TransferOptions};
use embassy_stm32::gpio::{AfType, AnyPin, Flex, OutputType, Pull, Speed};
use embassy_stm32::pac::{self, rcc::vals::Mdfsel};
use embassy_stm32::peripherals::{PB8, PC2, PD3, PD6, PE4, PE7};
use embassy_stm32::Peri;
use embassy_time::Instant;

use super::stm32_config::*;
use super::{
    cic_output_bits, BitDepth, CaptureState, Decimation, Error, MicrophoneConfig, MicrophoneMode,
    MicrophoneResources, ReshapeFilter, SamplePacking, SincFilter,
};

struct InterruptTimestamp {
    version: AtomicU32,
    low: AtomicU32,
    high: AtomicU32,
}

impl InterruptTimestamp {
    const fn new() -> Self {
        Self {
            version: AtomicU32::new(0),
            low: AtomicU32::new(0),
            high: AtomicU32::new(0),
        }
    }

    fn write(&self, value: u64) {
        self.version.fetch_add(1, Ordering::SeqCst);
        self.low.store(value as u32, Ordering::SeqCst);
        self.high.store((value >> 32) as u32, Ordering::SeqCst);
        self.version.fetch_add(1, Ordering::SeqCst);
    }

    fn read(&self) -> u64 {
        loop {
            let before = self.version.load(Ordering::SeqCst);
            if before & 1 != 0 {
                continue;
            }
            let low = self.low.load(Ordering::SeqCst);
            let high = self.high.load(Ordering::SeqCst);
            let after = self.version.load(Ordering::SeqCst);
            if before == after {
                return (u64::from(high) << 32) | u64::from(low);
            }
        }
    }
}

static DMA0_INTERRUPT_TICKS: InterruptTimestamp = InterruptTimestamp::new();
static DMA5_INTERRUPT_TICKS: InterruptTimestamp = InterruptTimestamp::new();

/// Add this handler alongside Embassy's DMA channel 0 handler in `bind_interrupts!`.
pub struct Dma0TimestampHandler;

impl
    embassy_stm32::interrupt::typelevel::Handler<
        embassy_stm32::interrupt::typelevel::GPDMA1_CHANNEL0,
    > for Dma0TimestampHandler
{
    unsafe fn on_interrupt() {
        DMA0_INTERRUPT_TICKS.write(Instant::now().as_ticks());
    }
}

/// Add this handler alongside Embassy's DMA channel 5 handler in `bind_interrupts!`.
pub struct Dma5TimestampHandler;

impl
    embassy_stm32::interrupt::typelevel::Handler<
        embassy_stm32::interrupt::typelevel::GPDMA1_CHANNEL5,
    > for Dma5TimestampHandler
{
    unsafe fn on_interrupt() {
        DMA5_INTERRUPT_TICKS.write(Instant::now().as_ticks());
    }
}

pub struct Pins<'d> {
    pub cck0: Peri<'d, PB8>,
    pub sd0: Peri<'d, PD3>,
    pub cck1: Peri<'d, PC2>,
    pub sd1: Peri<'d, PD6>,
    pub sd2: Peri<'d, PE7>,
    pub sd3: Peri<'d, PE4>,
}

pub struct DmaChannels<'d> {
    pub ch0: Channel<'d>,
    pub ch1: Channel<'d>,
    pub ch2: Channel<'d>,
    pub ch3: Channel<'d>,
    pub ch4: Channel<'d>,
    pub ch5: Channel<'d>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct ResolvedConfig {
    pub requested: MicrophoneConfig,
    /// CIC decimation (MCICD + 1), before optional reshape filtering.
    pub decimation: u16,
    pub total_decimation: u16,
    pub cic_output_bits: u8,
    pub microphone_clock_hz: u32,
    pub actual_sample_rate_hz: u32,
    pub clock_divider: u16,
}

pub struct Stm32MicrophoneDriver<
    'd,
    const BUFFER: usize,
    const WATCHERS: usize = { super::DEFAULT_WATCHERS },
> {
    pins: Pins<'d>,
    dma: DmaChannels<'d>,
    resources: &'static MicrophoneResources<BUFFER, WATCHERS>,
    config: ResolvedConfig,
}

impl<'d, const BUFFER: usize, const WATCHERS: usize> Stm32MicrophoneDriver<'d, BUFFER, WATCHERS> {
    pub fn new(
        pins: Pins<'d>,
        dma: DmaChannels<'d>,
        resources: &'static MicrophoneResources<BUFFER, WATCHERS>,
        config: MicrophoneConfig,
    ) -> Result<Self, Error> {
        if BUFFER < 2 || BUFFER % 2 != 0 {
            return Err(Error::InvalidBufferSize);
        }
        let config = resolve_config(config)?;
        Ok(Self {
            pins,
            dma,
            resources,
            config,
        })
    }

    pub const fn resolved_config(&self) -> ResolvedConfig {
        self.config
    }

    /// Runs continuous capture. DMA errors are published and capture resumes.
    pub async fn run(self) -> ! {
        configure_pins(self.pins);
        configure_mdf(self.config);

        let half = BUFFER / 2;
        let buffers = self.resources.buffers.get().cast::<[u32; BUFFER]>();
        let sync = unsafe { &mut *self.resources.sync.get() };

        // Each ring has exclusive DMA access to one row of the static resource.
        let mut mic1 = make_ring(self.dma.ch0, 0, unsafe { &mut *buffers.add(0) });
        let mut mic2 = make_ring(self.dma.ch1, 1, unsafe { &mut *buffers.add(1) });
        let mut mic3 = make_ring(self.dma.ch2, 2, unsafe { &mut *buffers.add(2) });
        let mut mic4 = make_ring(self.dma.ch3, 3, unsafe { &mut *buffers.add(3) });
        let mut mic5 = make_ring(self.dma.ch4, 4, unsafe { &mut *buffers.add(4) });
        let mut mic6 = make_ring(self.dma.ch5, 5, unsafe { &mut *buffers.add(5) });

        mic1.set_alignment(half);
        mic2.set_alignment(half);
        mic3.set_alignment(half);
        mic4.set_alignment(half);
        mic5.set_alignment(half);
        mic6.set_alignment(half);

        mic1.start();
        if self.config.requested.mode == MicrophoneMode::Hexaphonic {
            mic2.start();
            mic3.start();
            mic4.start();
            mic5.start();
            mic6.start();
        }

        // This timestamp deliberately precedes the enable writes: it is the
        // closest software bound on the instant at which all filters start.
        let started_at_ticks = Instant::now().as_ticks();
        enable_filters(self.config.requested.mode);

        let publisher = self.resources.state.sender();
        let channel_count = self.config.requested.mode.channel_count() as u8;
        let mut sequence = 0u64;
        publisher.send(CaptureState {
            running: true,
            started_at_ticks,
            channel_count,
            ..CaptureState::default()
        });

        loop {
            let result = if self.config.requested.mode == MicrophoneMode::Mono {
                mic1.read_exact(&mut sync[..half]).await.map(|_| ())
            } else {
                // The reference design's channels are synchronous. Waiting for
                // filter 5 therefore emits one coherent six-channel frame.
                mic6.read_exact(&mut sync[..half]).await.map(|_| ())
            };

            let completed_at_ticks = match self.config.requested.mode {
                MicrophoneMode::Mono => DMA0_INTERRUPT_TICKS.read(),
                MicrophoneMode::Hexaphonic => DMA5_INTERRUPT_TICKS.read(),
            };
            match result {
                Ok(()) => {
                    sequence = sequence.wrapping_add(1);
                    fence(Ordering::Acquire);
                    publisher.send(CaptureState {
                        running: true,
                        sequence,
                        half: ((sequence - 1) & 1) as u8,
                        started_at_ticks,
                        completed_at_ticks,
                        channel_count,
                        error: None,
                    });
                }
                Err(_) => {
                    if self.config.requested.mode == MicrophoneMode::Mono {
                        mic1.clear();
                    } else {
                        mic6.clear();
                    }
                    publisher.send(CaptureState {
                        running: true,
                        sequence,
                        half: (sequence & 1) as u8,
                        started_at_ticks,
                        completed_at_ticks,
                        channel_count,
                        error: Some(Error::Dma),
                    });
                }
            }
        }
    }
}

fn make_ring<'d, const BUFFER: usize>(
    channel: Channel<'d>,
    index: usize,
    buffer: &'static mut [u32; BUFFER],
) -> ReadableRingBuffer<'d, u32> {
    unsafe {
        ReadableRingBuffer::new(
            channel,
            DMA_REQUESTS[index],
            dfltdr_ptr(index),
            buffer,
            TransferOptions::default(),
        )
    }
}

pub fn resolve_config(config: MicrophoneConfig) -> Result<ResolvedConfig, Error> {
    if config.bit_depth == BitDepth::Bits24 && config.sample_packing == SamplePacking::Bits16 {
        return Err(Error::InvalidSamplePacking);
    }

    let decimation = match config.decimation {
        Decimation::Auto => auto_decimation(config)?,
        Decimation::Ratio(value) if (2..=512).contains(&value) => value,
        Decimation::Ratio(_) => return Err(Error::InvalidDecimation),
    };
    let output_bits = cic_output_bits(config.sinc_filter, decimation);
    if decimation > config.sinc_filter.max_pdm_decimation() || output_bits > 26 {
        return Err(Error::CicOutputTooWide);
    }

    let total_decimation = decimation * config.reshape_filter.decimation();
    let wanted_clock = config.sample_rate.hz() * u32::from(total_decimation);
    let divider = ((MDF_KERNEL_HZ + wanted_clock) / (wanted_clock * 2)).clamp(1, 256);
    let microphone_clock_hz = MDF_KERNEL_HZ / (2 * divider);
    if !valid_operating_clock(microphone_clock_hz) {
        return Err(Error::MicrophoneClockOutOfRange);
    }

    Ok(ResolvedConfig {
        requested: config,
        decimation,
        total_decimation,
        cic_output_bits: output_bits,
        microphone_clock_hz,
        actual_sample_rate_hz: microphone_clock_hz / u32::from(total_decimation),
        clock_divider: divider as u16,
    })
}

fn auto_decimation(config: MicrophoneConfig) -> Result<u16, Error> {
    let reshape = u32::from(config.reshape_filter.decimation());
    let mut decimation = config.sinc_filter.max_pdm_decimation();
    while decimation >= 2 {
        let wanted = config.sample_rate.hz() * u32::from(decimation) * reshape;
        let divider = ((MDF_KERNEL_HZ + wanted) / (wanted * 2)).clamp(1, 256);
        let clock = MDF_KERNEL_HZ / (2 * divider);
        if valid_operating_clock(clock) {
            return Ok(decimation);
        }
        decimation -= 1;
    }
    Err(Error::MicrophoneClockOutOfRange)
}

fn valid_operating_clock(hz: u32) -> bool {
    matches!(hz, 380_000..=1_020_000 | 1_170_000..=1_700_000 | 1_900_000..=3_400_000)
}

fn configure_pins(pins: Pins<'_>) {
    let mut cck0 = Flex::new(pins.cck0);
    cck0.set_as_af_unchecked(5, AfType::output(OutputType::PushPull, Speed::VeryHigh));
    core::mem::forget(cck0);
    let mut sd0 = Flex::new(pins.sd0);
    sd0.set_as_af_unchecked(6, AfType::input(Pull::None));
    core::mem::forget(sd0);
    let mut cck1 = Flex::new(pins.cck1);
    cck1.set_as_af_unchecked(6, AfType::output(OutputType::PushPull, Speed::VeryHigh));
    core::mem::forget(cck1);
    for pin in [
        pins.sd1.into::<AnyPin>(),
        pins.sd2.into::<AnyPin>(),
        pins.sd3.into::<AnyPin>(),
    ] {
        let mut pin = Flex::new(pin);
        pin.set_as_af_unchecked(6, AfType::input(Pull::None));
        core::mem::forget(pin);
    }
}

fn configure_mdf(config: ResolvedConfig) {
    let rcc = pac::RCC;
    rcc.ccipr2().modify(|w| w.set_mdf1sel(Mdfsel::HCLK1));
    rcc.ahb1enr().modify(|w| w.set_mdf1en(true));
    rcc.ahb1rstr().modify(|w| w.set_mdf1rst(true));
    rcc.ahb1rstr().modify(|w| w.set_mdf1rst(false));

    write(MDF_CKGCR, 0);
    write(MDF_GCR, 0);
    for filter in FILTERS {
        write(register(MDF_DFLTCR0, filter), 0);
        write(register(MDF_SITFCR0, filter), 0);
    }

    let divider = u32::from(config.clock_divider - 1);
    let ckgcr = (1 << 0) | (1 << 1) | (1 << 2) | (1 << 5) | (1 << 6) | (divider << 16) | (1 << 24);
    write(MDF_CKGCR, ckgcr);

    let cck0 = (1 << 0) | (1 << 4) | (4 << 8);
    let cck1 = (1 << 0) | (1 << 1) | (1 << 4) | (4 << 8);
    write(register(MDF_SITFCR0, 0), cck0);
    if config.requested.mode == MicrophoneMode::Hexaphonic {
        for interface in 1..=3 {
            write(register(MDF_SITFCR0, interface), cck1);
        }
    }

    let mode = match config.requested.sinc_filter {
        SincFilter::Sinc4 => 4,
        SincFilter::Sinc5 => 5,
    };
    for index in 0..config.requested.mode.channel_count() {
        write(register(MDF_BSMXCR0, index), BITSTREAM_SELECTS[index]);
        let scale = u32::from(config.requested.cic_scale.bits());
        let cic = mode << 4 | (u32::from(config.decimation) - 1) << 8 | scale << 20;
        write(register(MDF_DFLTCICR0, index), cic);
        let hpf_bypass = u32::from(!config.requested.high_pass_filter);
        let reshape_bypass = u32::from(matches!(
            config.requested.reshape_filter,
            ReshapeFilter::Bypass
        ));
        // RSFLTD=0 selects /4 when the reshape filter is enabled.
        write(
            register(MDF_DFLTRSFR0, index),
            reshape_bypass | (hpf_bypass << 7),
        );
    }
}

fn enable_filters(mode: MicrophoneMode) {
    for filter in 0..mode.channel_count() {
        write(register(MDF_DFLTCR0, filter), (1 << 0) | (1 << 1));
    }
}

fn dfltdr_ptr(filter: usize) -> *mut u32 {
    (MDF1_BASE + register(MDF_DFLTDR0, filter)) as *mut u32
}

fn write(offset: usize, value: u32) {
    unsafe { ptr::write_volatile((MDF1_BASE + offset) as *mut u32, value) }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mic_array::{MicrophonePreset, SampleRate};

    #[test]
    fn all_preset_rates_resolve_to_valid_microphone_clocks() {
        for rate in [
            SampleRate::Hz8000,
            SampleRate::Hz16000,
            SampleRate::Hz32000,
            SampleRate::Hz44100,
            SampleRate::Hz96000,
        ] {
            let resolved = resolve_config(MicrophoneConfig {
                sample_rate: rate,
                decimation: Decimation::Auto,
                ..MicrophoneConfig::default()
            })
            .unwrap();
            assert!(valid_operating_clock(resolved.microphone_clock_hz));
        }
    }

    #[test]
    fn table_376_limits_are_enforced() {
        for (filter, maximum) in [(SincFilter::Sinc4, 76), (SincFilter::Sinc5, 32)] {
            let valid = resolve_config(MicrophoneConfig {
                sinc_filter: filter,
                decimation: Decimation::Ratio(maximum),
                reshape_filter: ReshapeFilter::Bypass,
                sample_rate: SampleRate::Hz16000,
                ..MicrophoneConfig::default()
            });
            assert!(valid.is_ok());
            assert_eq!(cic_output_bits(filter, maximum), 26);

            let invalid = resolve_config(MicrophoneConfig {
                sinc_filter: filter,
                decimation: Decimation::Ratio(maximum + 1),
                ..MicrophoneConfig::default()
            });
            assert_eq!(invalid, Err(Error::CicOutputTooWide));
        }
    }

    #[test]
    fn selected_table_384_presets_resolve() {
        for preset in [
            MicrophonePreset::Table384Config1_8Khz,
            MicrophonePreset::Table384Config2_16Khz,
            MicrophonePreset::Table384Config3_8Khz,
            MicrophonePreset::Table384Config7_16Khz,
            MicrophonePreset::Table384Config8_16Khz,
        ] {
            assert!(resolve_config(MicrophoneConfig::from_preset(preset)).is_ok());
        }
    }
}
