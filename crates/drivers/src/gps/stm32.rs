use embassy_stm32::Peri;
use embassy_stm32::exti::{ExtiInput, InterruptHandler as ExtiInterruptHandler};
use embassy_stm32::gpio::{Output, Pull};
use embassy_stm32::interrupt::typelevel::Binding;
use embassy_stm32::mode::Async;
use embassy_stm32::peripherals::{EXTI9, PB9, TIM4};
use embassy_stm32::time::hz;
use embassy_stm32::timer::input_capture::{CapturePin, Ch1, Ch2, Ch3, Ch4, InputCapture};
use embassy_stm32::timer::low_level::CountingMode;
use embassy_stm32::timer::{CaptureCompareInterruptHandler, Channel, GeneralInstance1Channel};
use embassy_time::Instant;

use crate::gps::{GpsConfig, GpsPowerControl, PpsCapture, PpsSource, PpsTimingSource};

pub const TIM4_PPS_CAPTURE_FREQUENCY_HZ: u32 = 1_000_000;
const TIM4_COUNTER_MODULUS: u64 = 1 << 16;

pub struct Stm32GpsPower {
    en: Output<'static>,
    rst: Output<'static>,
}

impl Stm32GpsPower {
    pub fn new(en: Output<'static>, rst: Output<'static>) -> Self {
        Self { en, rst }
    }
}

impl GpsPowerControl for Stm32GpsPower {
    type Error = core::convert::Infallible;

    fn set_enabled(&mut self, enabled: bool) -> Result<(), Self::Error> {
        if enabled {
            self.en.set_high();
        } else {
            self.en.set_low();
        }
        Ok(())
    }

    fn set_reset_asserted(&mut self, asserted: bool) -> Result<(), Self::Error> {
        if asserted {
            self.rst.set_low();
        } else {
            self.rst.set_high();
        }
        Ok(())
    }
}

pub struct ExtiPps {
    input: ExtiInput<'static, Async>,
}

impl ExtiPps {
    pub fn new(input: ExtiInput<'static, Async>) -> Self {
        Self { input }
    }
}

impl PpsSource for ExtiPps {
    type Error = core::convert::Infallible;

    async fn wait_for_pps(&mut self) -> Result<PpsCapture, Self::Error> {
        self.input.wait_for_rising_edge().await;
        Ok(PpsCapture {
            timing_source: PpsTimingSource::EmbassyInstant,
            timestamp: Instant::now(),
            capture_ticks: None,
            capture_frequency_hz: None,
        })
    }
}

pub struct Tim4Pps {
    capture: InputCapture<'static, TIM4>,
    previous_raw: Option<u32>,
    previous_observation_time: Option<Instant>,
    extended_ticks: u64,
    reference_capture_ticks: Option<u64>,
    reference_system_time: Option<Instant>,
}

impl Tim4Pps {
    pub fn new(
        timer: Peri<'static, TIM4>,
        pin: Peri<'static, PB9>,
        irq: impl Binding<
            <TIM4 as GeneralInstance1Channel>::CaptureCompareInterrupt,
            CaptureCompareInterruptHandler<TIM4>,
        > + 'static,
    ) -> Self {
        let capture_pin = CapturePin::new(pin, Pull::None);
        let capture = InputCapture::new(
            timer,
            None::<CapturePin<'static, TIM4, Ch1>>,
            None::<CapturePin<'static, TIM4, Ch2>>,
            None::<CapturePin<'static, TIM4, Ch3>>,
            Some::<CapturePin<'static, TIM4, Ch4>>(capture_pin),
            irq,
            hz(TIM4_PPS_CAPTURE_FREQUENCY_HZ),
            CountingMode::EdgeAlignedUp,
        );
        Self {
            capture,
            previous_raw: None,
            previous_observation_time: None,
            extended_ticks: 0,
            reference_capture_ticks: None,
            reference_system_time: None,
        }
    }

    fn extend_ticks(&mut self, raw: u32, observation_time: Instant) -> u64 {
        // TIM4 on STM32U595 is a 16-bit timer even though Embassy exposes its
        // capture value as u32. At 1 MHz it wraps about fifteen times between
        // 1 Hz PPS edges. Use the coarse monotonic elapsed time only to resolve
        // that integer wrap ambiguity; the sub-wrap phase still comes directly
        // from the hardware capture register.
        let raw = raw & 0xffff;
        if let Some(previous_raw) = self.previous_raw {
            let modulo_delta = raw.wrapping_sub(previous_raw) as u64 & 0xffff;
            let approximate_delta = self
                .previous_observation_time
                .map(|previous_time| {
                    let elapsed_system_ticks = observation_time
                        .saturating_duration_since(previous_time)
                        .as_ticks();
                    ((elapsed_system_ticks as u128)
                        .saturating_mul(TIM4_PPS_CAPTURE_FREQUENCY_HZ as u128)
                        / embassy_time::TICK_HZ as u128)
                        .min(u64::MAX as u128) as u64
                })
                .unwrap_or(modulo_delta);
            let whole_wraps = approximate_delta
                .saturating_sub(modulo_delta)
                .saturating_add(TIM4_COUNTER_MODULUS / 2)
                / TIM4_COUNTER_MODULUS;
            let capture_delta =
                modulo_delta.saturating_add(whole_wraps.saturating_mul(TIM4_COUNTER_MODULUS));
            self.extended_ticks = self.extended_ticks.saturating_add(capture_delta);
        } else {
            self.extended_ticks = raw as u64;
        }
        self.previous_raw = Some(raw);
        self.previous_observation_time = Some(observation_time);
        self.extended_ticks
    }
}

impl PpsSource for Tim4Pps {
    type Error = core::convert::Infallible;

    async fn wait_for_pps(&mut self) -> Result<PpsCapture, Self::Error> {
        let raw: u32 = self.capture.wait_for_rising_edge(Channel::Ch4).await;
        let observation_time = Instant::now();
        let capture_ticks = self.extend_ticks(raw, observation_time);
        // The first edge establishes the cross-domain epoch. Later timestamps
        // are reconstructed from the hardware capture counter, so interrupt
        // wake-up latency cannot appear as PPS jitter.
        let timestamp = match (self.reference_capture_ticks, self.reference_system_time) {
            (Some(reference_ticks), Some(reference_time)) => {
                let elapsed_capture_ticks = capture_ticks.saturating_sub(reference_ticks);
                let elapsed_system_ticks = (elapsed_capture_ticks as u128)
                    .saturating_mul(embassy_time::TICK_HZ as u128)
                    / TIM4_PPS_CAPTURE_FREQUENCY_HZ as u128;
                Instant::from_ticks(
                    reference_time
                        .as_ticks()
                        .saturating_add(elapsed_system_ticks.min(u64::MAX as u128) as u64),
                )
            }
            _ => {
                self.reference_capture_ticks = Some(capture_ticks);
                self.reference_system_time = Some(observation_time);
                observation_time
            }
        };
        Ok(PpsCapture {
            timing_source: PpsTimingSource::Tim4Capture,
            timestamp,
            capture_ticks: Some(capture_ticks),
            capture_frequency_hz: Some(TIM4_PPS_CAPTURE_FREQUENCY_HZ),
        })
    }
}

pub enum Stm32Pps {
    Exti(ExtiPps),
    Tim4(Tim4Pps),
}

impl Stm32Pps {
    pub fn from_config(
        config: &GpsConfig,
        pin: Peri<'static, PB9>,
        exti: Peri<'static, EXTI9>,
        timer: Peri<'static, TIM4>,
        exti_irq: impl Binding<
            embassy_stm32::interrupt::typelevel::EXTI9,
            ExtiInterruptHandler<embassy_stm32::interrupt::typelevel::EXTI9>,
        > + 'static,
        timer_irq: impl Binding<
            <TIM4 as GeneralInstance1Channel>::CaptureCompareInterrupt,
            CaptureCompareInterruptHandler<TIM4>,
        > + 'static,
    ) -> Self {
        match config.pps_timing_source {
            PpsTimingSource::EmbassyInstant => Self::Exti(ExtiPps::new(ExtiInput::new(
                pin,
                exti,
                Pull::None,
                exti_irq,
            ))),
            PpsTimingSource::Tim4Capture => Self::Tim4(Tim4Pps::new(timer, pin, timer_irq)),
        }
    }
}

impl PpsSource for Stm32Pps {
    type Error = core::convert::Infallible;

    async fn wait_for_pps(&mut self) -> Result<PpsCapture, Self::Error> {
        match self {
            Self::Exti(source) => source.wait_for_pps().await,
            Self::Tim4(source) => source.wait_for_pps().await,
        }
    }
}
