#![no_std]

use embassy_stm32::exti::{self, ExtiInput};
use embassy_stm32::gpio::{Level, Output, Pull, Speed};
use embassy_stm32::mode::Async;
use embassy_stm32::Peripherals;
use embassy_stm32::Peri;
// buzzer imports
use embassy_stm32::peripherals::{PA5, TIM8};
// i2c imports
use embassy_stm32::peripherals::{I2C5, PD0, PD1};
use embassy_stm32::{bind_interrupts, interrupt};

bind_interrupts!(struct Irqs {
    EXTI2 => exti::InterruptHandler<interrupt::typelevel::EXTI2>;
});

pub struct Board<'d> {
    pub leds: Leds<'d>,
    pub buttons: Buttons<'d>,
    pub buzzer: Buzzer<'d>,
    pub sens_i2c: SensI2C<'d>,
}

pub struct Leds<'d> {
    pub sys_gps_green: Output<'d>,
    pub sys_gps_red: Output<'d>,
    pub sys_main_red: Output<'d>,
    pub sys_main_green: Output<'d>,
    pub sys_sd_blue: Output<'d>,
}

pub struct Buttons<'d> {
    pub user: ExtiInput<'d, Async>,
}

// Buzzer on PA5, TIM8_CH1N (can also be exposed as DAC1_OUT2)
pub struct Buzzer<'d> {
    pub tim: Peri<'d, TIM8>,
    pub pin: Peri<'d, PA5>,
}

// SENS_I2C (accelerometer, magnetometer and BMC charge controller) on PD0 (SDA) and PD1 (SCL) on I2C5.
pub struct SensI2C<'d> {
    pub i2c: Peri<'d, I2C5>,
    pub sda: Peri<'d, PD0>,
    pub scl: Peri<'d, PD1>,
}


impl Board<'static> {
    pub fn new(p: Peripherals) -> Self {
        let Peripherals {
            // leds
            PB4,
            PD7,
            PB15,
            PD10,
            PD5,
            // user button
            PE2,
            EXTI2,
            // buzzer
            PA5,
            TIM8,
            // sens_i2c 
            PD0,
            PD1,
            I2C5,
            ..
        } = p;

        Self {
            leds: Leds {
                sys_gps_green: Output::new(PB4, Level::Low, Speed::Medium),
                sys_gps_red: Output::new(PD7, Level::Low, Speed::Medium),
                sys_main_red: Output::new(PB15, Level::Low, Speed::Medium),
                sys_main_green: Output::new(PD10, Level::Low, Speed::Medium),
                sys_sd_blue: Output::new(PD5, Level::Low, Speed::Medium),
            },
            buttons: Buttons {
                user: ExtiInput::new(PE2, EXTI2, Pull::Up, Irqs),
            },
            buzzer: Buzzer {
                tim: TIM8,
                pin: PA5,
            },
            sens_i2c: SensI2C {
                i2c: I2C5,
                sda: PD0,
                scl: PD1,
            },
        }
    }
}
