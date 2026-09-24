use embassy_stm32::Peri;
use embassy_stm32::exti::{ExtiInput, InterruptHandler as ExtiInterruptHandler};
use embassy_stm32::gpio::{Output, Pull};
use embassy_stm32::interrupt::typelevel::Binding;
use embassy_stm32::mode::Async;
use embassy_stm32::peripherals::{EXTI9, PB9, TIM4};
use embassy_stm32::time::hz;
use embassy_stm32::timer::input_capture::{CapturePin, Ch1, Ch2, Ch3, Ch4, InputCapture};
use embassy_stm32::timer::low_level::CountingMode;
use embassy_stm32::timer::{
    CaptureCompareInterruptHandler, Channel, CoreInstance, GeneralInstance1Channel,
};
use embassy_time::Instant;

use crate::gps::{
    resolve_periodic_capture_delta, GpsConfig, GpsPowerControl, PpsCapture, PpsSource,
    PpsTimingSource,
};

pub const TIM4_PPS_CAPTURE_FREQUENCY_HZ: u32 = 1_000_000;
// STM32U59xxx TIM4 is a 32-bit general-purpose timer, unlike TIM4 on many
// other STM32 families. Authoritative sources: DS13633 Rev 3, section 3.44,
// table 19 (p. 80/385), and section 3.44.2 (p. 81/385); RM0456,
// "General-purpose timers (TIM2/TIM3/TIM4/TIM5)" and the TIMx_ARR/TIMx_CCR4
// register definitions. At 1 MHz the counter wraps every 2^32 us, or about
// 71 minutes 35 seconds.
const TIM4_COUNTER_MODULUS: u64 = 1u64 << 32;
// TIM4 is clocked at a nominal 144 MHz on Raylar v1.0. PSC stores divisor-1.
// This explicit value also avoids embassy-stm32 0.6's M=3/N=54 clock-model
// truncation selecting PSC=142 and making the capture clock 144/143 too fast.
const TIM4_ONE_MHZ_PRESCALER: u16 = 143;

fn configure_tim4_32_bit_period() {
    // Embassy InputCapture configures the timer tick prescaler but leaves ARR
    // at its reset value. RM0456's TIMx_ARR definition gives that reset value
    // as 0x0000_FFFF even for this 32-bit timer, which would still make TIM4
    // wrap every 65.536 ms. Program the full period explicitly before any PPS
    // capture is awaited. InputCapture remains the sole owner of TIM4; this
    // narrowly scoped PAC access only completes its hardware initialization.
    let regs = unsafe {
        embassy_stm32::pac::timer::TimGp32::from_ptr(<TIM4 as CoreInstance>::regs())
    };
    regs.cr1().modify(|r| r.set_cen(false));
    regs.psc().write_value(TIM4_ONE_MHZ_PRESCALER);
    regs.arr().write_value(u32::MAX);
    regs.egr().write(|r| r.set_ug(true));
    regs.sr().modify(|r| r.set_uif(false));
    regs.cr1().modify(|r| r.set_cen(true));
    debug_assert_eq!(regs.arr().read(), u32::MAX);
    debug_assert_eq!(regs.psc().read(), TIM4_ONE_MHZ_PRESCALER);
}

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
        configure_tim4_32_bit_period();
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
        // TIM4_CH4 captures the full STM32U595 32-bit counter (DS13633 Rev 3,
        // section 3.44, table 19). A normal PPS interval or GPS standby cycle
        // is far shorter than its approximately 71-minute wrap period. Use the
        // coarse monotonic elapsed time only to extend the rare full 32-bit
        // wrap; fine timing remains entirely from the capture register.
        if let Some(previous_raw) = self.previous_raw {
            let modulo_delta = raw.wrapping_sub(previous_raw) as u64;
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
            let capture_delta = resolve_periodic_capture_delta(
                modulo_delta,
                approximate_delta,
                TIM4_COUNTER_MODULUS,
                TIM4_PPS_CAPTURE_FREQUENCY_HZ as u64,
            );
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
        // embassy-stm32 0.6's InputCaptureFuture uses the TimGp16 register
        // view when it returns the captured value, truncating CCR4 to 16 bits
        // even though STM32U59xxx TIM4 and T::Word are 32-bit. Use the future
        // only to await the hardware edge, then re-read the still-latched CCR4
        // through get_capture_value(), whose low-level path uses TimGp32.
        let _truncated_capture = self.capture.wait_for_rising_edge(Channel::Ch4).await;
        let raw: u32 = self.capture.get_capture_value(Channel::Ch4);
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
