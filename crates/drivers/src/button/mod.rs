//! Debounced logical user-button driver.

#![cfg(feature = "stm32")]

use embassy_stm32::exti::ExtiInput;
use embassy_stm32::mode::Async;
use embassy_time::{Duration, Timer};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ButtonName { User }

pub struct ButtonResources<'d> { pub user: ExtiInput<'d, Async> }

pub struct ButtonDriver<'d> {
    user: ExtiInput<'d, Async>,
    debounce: Duration,
}

pub fn init(resources: ButtonResources<'static>) -> ButtonDriver<'static> {
    init_with_debounce(resources, Duration::from_millis(30))
}

pub fn init_with_debounce(
    resources: ButtonResources<'static>,
    debounce: Duration,
) -> ButtonDriver<'static> {
    ButtonDriver { user: resources.user, debounce }
}

impl<'d> ButtonDriver<'d> {
    pub async fn wait_for_press(&mut self, button: ButtonName) {
        match button {
            ButtonName::User => loop {
                self.user.wait_for_falling_edge().await;
                Timer::after(self.debounce).await;
                if self.user.is_low() { return; }
            },
        }
    }

    pub async fn wait_for_release(&mut self, button: ButtonName) {
        match button {
            ButtonName::User => loop {
                self.user.wait_for_rising_edge().await;
                Timer::after(self.debounce).await;
                if self.user.is_high() { return; }
            },
        }
    }

    pub fn is_pressed(&self, button: ButtonName) -> bool {
        match button { ButtonName::User => self.user.is_low() }
    }
}
