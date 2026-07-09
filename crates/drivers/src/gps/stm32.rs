use embassy_stm32::exti::ExtiInput;
use embassy_stm32::gpio::Output;
use embassy_stm32::mode::Async;
use embassy_time::Instant;

use crate::gps::{GpsPowerControl, PpsCapture, PpsSource};

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
            timestamp: Instant::now(),
            capture_ticks: None,
        })
    }
}
