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
