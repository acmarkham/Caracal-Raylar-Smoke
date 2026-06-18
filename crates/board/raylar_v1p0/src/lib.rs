#![no_std]

use embassy_stm32::exti::{self, ExtiInput};
use embassy_stm32::gpio::{Level, Output, Pull, Speed};
use embassy_stm32::mode::Async;
use embassy_stm32::Peripherals;
use embassy_stm32::{bind_interrupts, interrupt};

bind_interrupts!(struct Irqs {
    EXTI2 => exti::InterruptHandler<interrupt::typelevel::EXTI2>;
});

pub struct Board<'d> {
    pub leds: Leds<'d>,
    pub buttons: Buttons<'d>,
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

impl Board<'static> {
    pub fn new(p: Peripherals) -> Self {
        let Peripherals {
            PB4,
            PD7,
            PB15,
            PD10,
            PD5,
            PE2,
            EXTI2,
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
        }
    }
}
