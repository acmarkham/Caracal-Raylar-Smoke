//! Low-level asynchronous buzzer driver.

#![cfg(feature = "stm32")]

use embassy_stm32::gpio::OutputType;
use embassy_stm32::peripherals::{PA5, TIM8};
use embassy_stm32::time::hz;
use embassy_stm32::timer::complementary_pwm::{ComplementaryPwm, ComplementaryPwmPin};
use embassy_stm32::timer::low_level::CountingMode;
use embassy_stm32::timer::Channel;
use embassy_stm32::Peri;
use embassy_time::{Duration, Timer};

const MIN_FREQUENCY_HZ: u32 = 20;
const MAX_FREQUENCY_HZ: u32 = 100_000;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PitchHz(pub u32);

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Volume(pub u8);

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum BuzzerError {
    UnsupportedPitch,
}

/// Board-independent ownership bundle for the Raylar buzzer wiring.
pub struct BuzzerResources<'d> {
    pub timer: Peri<'d, TIM8>,
    pub pin: Peri<'d, PA5>,
}

pub struct BuzzerDriver<'d> {
    pwm: ComplementaryPwm<'d, TIM8>,
}

pub fn init(resources: BuzzerResources<'static>) -> BuzzerDriver<'static> {
    let pin = ComplementaryPwmPin::new(resources.pin, OutputType::PushPull);
    let mut pwm = ComplementaryPwm::new(
        resources.timer,
        None, Some(pin), None, None, None, None, None, None,
        hz(1_000), CountingMode::EdgeAlignedUp,
    );
    pwm.disable(Channel::Ch1);
    BuzzerDriver { pwm }
}

impl<'d> BuzzerDriver<'d> {
    pub async fn play_tone(
        &mut self,
        pitch: PitchHz,
        duration: Duration,
        volume: Volume,
    ) -> Result<(), BuzzerError> {
        if !(MIN_FREQUENCY_HZ..=MAX_FREQUENCY_HZ).contains(&pitch.0) {
            self.pwm.disable(Channel::Ch1);
            return Err(BuzzerError::UnsupportedPitch);
        }

        self.pwm.set_frequency(hz(pitch.0));
        let duty = (self.pwm.get_max_duty() / 2 * u32::from(volume.0)) / 255;
        if volume.0 == 0 {
            self.pwm.disable(Channel::Ch1);
        } else {
            self.pwm.set_duty(Channel::Ch1, duty);
            self.pwm.enable(Channel::Ch1);
        }

        Timer::after(duration).await;
        self.pwm.disable(Channel::Ch1);
        Ok(())
    }
}
