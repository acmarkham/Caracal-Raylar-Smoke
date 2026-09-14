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
            extended_ticks: 0,
            reference_capture_ticks: None,
            reference_system_time: None,
        }
    }

    fn extend_ticks(&mut self, raw: u32) -> u64 {
        if let Some(previous_raw) = self.previous_raw {
            self.extended_ticks = self
                .extended_ticks
                .saturating_add(raw.wrapping_sub(previous_raw) as u64);
        } else {
            self.extended_ticks = raw as u64;
        }
        self.previous_raw = Some(raw);
        self.extended_ticks
    }
}

impl PpsSource for Tim4Pps {
    type Error = core::convert::Infallible;

    async fn wait_for_pps(&mut self) -> Result<PpsCapture, Self::Error> {
        let raw: u32 = self.capture.wait_for_rising_edge(Channel::Ch4).await;
        let capture_ticks = self.extend_ticks(raw);
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
                let now = Instant::now();
                self.reference_capture_ticks = Some(capture_ticks);
                self.reference_system_time = Some(now);
                now
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
