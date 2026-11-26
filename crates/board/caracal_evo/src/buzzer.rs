//! Evo PB14 is TIM8_CH2N, not the U5 board's buzzer pin/channel.
use embassy_stm32::{
    gpio::OutputType,
    peripherals::{PB14, TIM8},
    time::Hertz,
    timer::{
        complementary_pwm::{ComplementaryPwm, ComplementaryPwmPin},
        low_level::CountingMode,
        Channel,
    },
    Peri,
};

pub struct Buzzer(ComplementaryPwm<'static, TIM8>);
impl Buzzer {
    pub fn new(timer: Peri<'static, TIM8>, pin: Peri<'static, PB14>) -> Self {
        let pin = ComplementaryPwmPin::new(pin, OutputType::PushPull);
        let mut pwm = ComplementaryPwm::new(
            timer,
            None,
            None,
            None,
            Some(pin),
            None,
            None,
            None,
            None,
            Hertz(1000),
            CountingMode::EdgeAlignedUp,
        );
        pwm.disable(Channel::Ch2);
        Self(pwm)
    }
    /// Hardware PWM continues while the executor sleeps. Zero silences it.
    pub fn set(&mut self, frequency: u32, volume: u8) {
        self.0.disable(Channel::Ch2);
        if frequency != 0 && volume != 0 {
            self.0.set_frequency(Hertz(frequency));
            self.0.set_duty(
                Channel::Ch2,
                self.0.get_max_duty() / 2 * u32::from(volume) / 255,
            );
            self.0.enable(Channel::Ch2);
        }
    }
}
