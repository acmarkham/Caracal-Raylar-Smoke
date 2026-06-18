#![no_std]
#![no_main]

use defmt::{info, unwrap};
use embassy_executor::Spawner;
use embassy_stm32::exti::ExtiInput;
use embassy_stm32::gpio::Output;
use embassy_stm32::mode::Async;
use embassy_time::{Duration, Timer};
use raylar_board_v1p0::{Board, Buttons, Leds};
use {defmt_rtt as _, panic_probe as _};

#[embassy_executor::main]
async fn main(spawner: Spawner) -> ! {
    let p = embassy_stm32::init(Default::default());
    let Board { leds, buttons, .. } = Board::new(p);
    let Leds {
        sys_main_red,
        sys_main_green,
        ..
    } = leds;
    let Buttons { user } = buttons;

    info!("User button smoke test started");

    spawner.spawn(unwrap!(heartbeat_task(sys_main_green)));
    spawner.spawn(unwrap!(user_button_task(user, sys_main_red)));

    core::future::pending().await
}

#[embassy_executor::task]
async fn heartbeat_task(mut led: Output<'static>) -> ! {
    loop {
        led.set_high();
        Timer::after_millis(100).await;
        led.set_low();
        Timer::after_millis(900).await;
    }
}

#[embassy_executor::task]
async fn user_button_task(mut button: ExtiInput<'static, Async>, mut led: Output<'static>) -> ! {
    loop {
        button.wait_for_falling_edge().await;
        Timer::after(Duration::from_millis(30)).await;

        if button.is_low() {
            led.toggle();
            info!("User button pressed; sys_main_red toggled");

            button.wait_for_rising_edge().await;
            Timer::after(Duration::from_millis(30)).await;
        }
    }
}
