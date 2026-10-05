use embassy_stm32::gpio::Output;
use embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex;
use embassy_sync::channel::Channel;
use embassy_time::{Duration, Timer};

use integration004_radioheartbeat::policy::Role;

static COMMANDS: Channel<CriticalSectionRawMutex, Command, 16> = Channel::new();

#[derive(Clone, Copy)]
enum Command {
    Transmit,
    Receive,
    Fatal,
}

pub fn transmit() {
    let _ = COMMANDS.try_send(Command::Transmit);
}

pub fn receive() {
    let _ = COMMANDS.try_send(Command::Receive);
}

pub fn fatal() {
    let _ = COMMANDS.try_send(Command::Fatal);
}

#[embassy_executor::task]
pub async fn role_led_task(mut role_led: Output<'static>, role: Role) -> ! {
    if role.is_base_station() {
        role_led.set_high();
    } else {
        role_led.set_low();
    }
    loop {
        // Keep ownership and periodically restore the latched indication.
        Timer::after_secs(60).await;
        if role.is_base_station() {
            role_led.set_high();
        } else {
            role_led.set_low();
        }
    }
}

#[embassy_executor::task]
pub async fn activity_led_task(
    mut tx_led: Output<'static>,
    mut rx_led: Output<'static>,
    mut fatal_led: Output<'static>,
) -> ! {
    tx_led.set_low();
    rx_led.set_low();
    fatal_led.set_low();
    let mut fatal_latched = false;
    loop {
        match COMMANDS.receive().await {
            Command::Transmit => {
                tx_led.set_high();
                Timer::after(Duration::from_millis(40)).await;
                tx_led.set_low();
            }
            Command::Receive => {
                rx_led.set_high();
                Timer::after(Duration::from_millis(40)).await;
                rx_led.set_low();
            }
            Command::Fatal => {
                fatal_latched = true;
                fatal_led.set_high();
            }
        }
        if fatal_latched {
            fatal_led.set_high();
        }
    }
}
