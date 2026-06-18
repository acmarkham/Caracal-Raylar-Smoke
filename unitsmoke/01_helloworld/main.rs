// Test of leds on the Raylar board. This is a smoke test to verify that the leds are working and that the board can control them. This is the most basic test of the board and is a good starting point for testing the board's functionality.

// Activity: This test will blink all the leds on the board in a sequence, with a short delay between each led. The leds will be turned on for 100ms and then turned off for 100ms, with a total cycle time of 800ms. The leds will be blinked in the following order: sys_main_red, sys_main_green, sys_gps_green, sys_gps_red, sys_sd_blue.

// Assumptions: using MSI as the clock source, and the board is powered on and running.

#![no_std]
#![no_main]

use defmt::info;
use embassy_executor::Spawner;
use embassy_time::Timer;
use raylar_board_v1p0::Board;
use {defmt_rtt as _, panic_probe as _};

#[embassy_executor::main]
async fn main(_spawner: Spawner) -> ! {
    let p = embassy_stm32::init(Default::default());
    let mut board = Board::new(p);

    info!("Hello World!");

    loop {
        info!("on!");

        board.leds.sys_main_red.set_high();
        Timer::after_millis(100).await;
        board.leds.sys_main_red.set_low();
        Timer::after_millis(100).await;

        board.leds.sys_main_green.set_high();
        Timer::after_millis(100).await;
        board.leds.sys_main_green.set_low();
        Timer::after_millis(100).await;

        board.leds.sys_gps_green.set_high();
        Timer::after_millis(100).await;
        board.leds.sys_gps_green.set_low();
        Timer::after_millis(100).await;

        board.leds.sys_gps_red.set_high();
        Timer::after_millis(100).await;
        board.leds.sys_gps_red.set_low();
        Timer::after_millis(100).await;

        board.leds.sys_sd_blue.set_high();
        Timer::after_millis(100).await;
        board.leds.sys_sd_blue.set_low();
        Timer::after_millis(100).await;

        info!("off!");
        Timer::after_millis(800).await;
    }
}
