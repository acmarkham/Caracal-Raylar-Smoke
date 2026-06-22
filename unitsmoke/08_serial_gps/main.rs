// GPS serial smoke test.
//
// Activity: This test enables and releases reset on the GPS module, then listens
// on USART2 at 9600 8N1, flashes sys_gps_green while serial data is arriving,
// and prints received serial messages via defmt.
//
// Clock check: HSE=16 MHz, PLL1_R = 16 MHz / 1 * 10 / 2 = 80 MHz SYSCLK.
// APB1 defaults to DIV1, so USART2 is clocked from an 80 MHz PCLK1. At 9600
// baud Embassy programs BRR ~= 8333, giving about 9600.38 baud (0.004% high).

#![no_std]
#![no_main]

use core::str;

use defmt::{info, unwrap};
use embassy_executor::Spawner;
use embassy_stm32::gpio::Output;
use embassy_stm32::rcc::*;
use embassy_stm32::time::mhz;
use embassy_stm32::usart::{Config, DataBits, Parity, StopBits, Uart};
use embassy_time::{Duration, Timer};
use raylar_board_v1p0::{Board, Gps};
use {defmt_rtt as _, panic_probe as _};

#[embassy_executor::main]
async fn main(_spawner: Spawner) -> ! {
    let mut config = embassy_stm32::Config::default();

    config.rcc.hse = Some(Hse {
        freq: mhz(16),
        mode: HseMode::Oscillator,
    });

    config.rcc.pll1 = Some(Pll {
        source: PllSource::HSE,
        prediv: PllPreDiv::DIV1,
        mul: PllMul::MUL10,
        divp: Some(PllDiv::DIV1),
        divq: Some(PllDiv::DIV2),
        divr: Some(PllDiv::DIV2),
    });

    config.rcc.sys = Sysclk::PLL1_R;

    let p = embassy_stm32::init(config);
    let Board { gps, leds, .. } = Board::new(p);

    info!("GPS serial smoke test started");
    gps_serial_listener(gps, leds.sys_gps_green).await
}

async fn gps_serial_listener(gps: Gps<'static>, mut serial_led: Output<'static>) -> ! {
    let Gps {
        usart,
        tx,
        rx,
        pps: _pps,
        mut rst,
        mut en,
    } = gps;

    en.set_high();
    rst.set_high();
    info!("GPS_EN=HIGH GPS_RST=HIGH");

    Timer::after(Duration::from_millis(250)).await;

    let mut config = Config::default();
    config.baudrate = 9_600;
    config.data_bits = DataBits::DataBits8;
    config.parity = Parity::ParityNone;
    config.stop_bits = StopBits::STOP1;

    let mut uart = unwrap!(Uart::new_blocking(usart, rx, tx, config));
    let mut line = [0u8; 128];
    let mut len = 0usize;
    let mut byte = [0u8; 1];

    info!("Listening on GPS USART2 at 9600 8N1");

    loop {
        match uart.blocking_read(&mut byte) {
            Ok(()) => {
                let b = byte[0];
                serial_led.set_high();

                if b == b'\n' || len == line.len() {
                    log_gps_message(&line[..len]);
                    serial_led.set_low();
                    len = 0;
                } else if b != b'\r' {
                    line[len] = b;
                    len += 1;
                }
            }
            Err(e) => {
                info!("GPS UART read error: {=?}", e);
            }
        }
    }
}

fn log_gps_message(bytes: &[u8]) {
    if bytes.is_empty() {
        return;
    }

    match str::from_utf8(bytes) {
        Ok(message) => info!("GPS: {}", message),
        Err(_) => info!("GPS bytes: {=[u8]}", bytes),
    }
}
