// GPS serial and PPS smoke test.
//
// Activity: This test enables and releases reset on the GPS module, listens on
// USART2 at 9600 8N1, and flashes sys_gps_red briefly on each GPS_PPS rising
// edge to indicate that PPS is present.
//
// Clock check: HSE=16 MHz, PLL1_R = 16 MHz / 1 * 10 / 2 = 80 MHz SYSCLK.
// APB1 defaults to DIV1, so USART2 is clocked from an 80 MHz PCLK1. At 9600
// baud Embassy programs BRR ~= 8333, giving about 9600.38 baud (0.004% high).

#![no_std]
#![no_main]

use core::str;

use defmt::{info, unwrap};
use embassy_executor::Spawner;
use embassy_stm32::exti::ExtiInput;
use embassy_stm32::gpio::Output;
use embassy_stm32::mode::Async;
use embassy_stm32::peripherals::{PA2, PA3, USART2};
use embassy_stm32::rcc::*;
use embassy_stm32::time::mhz;
use embassy_stm32::usart::{BufferedUart, Config, DataBits, Parity, StopBits};
use embassy_stm32::Peri;
use embassy_time::{Duration, Timer};
use embedded_io_async::Read;
use raylar_board_v1p0::{Board, Gps, Irqs};
use {defmt_rtt as _, panic_probe as _};

#[embassy_executor::main]
async fn main(spawner: Spawner) -> ! {
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
    let Gps {
        usart,
        tx,
        rx,
        pps,
        mut rst,
        mut en,
        ..
    } = gps;

    info!("GPS serial PPS smoke test started");

    en.set_high();
    rst.set_high();
    info!("GPS_EN=HIGH GPS_RST=HIGH");

    Timer::after(Duration::from_millis(250)).await;

    spawner.spawn(unwrap!(gps_pps_task(pps, leds.sys_gps_red)));
    spawner.spawn(unwrap!(gps_serial_task(usart, tx, rx)));

    core::future::pending().await
}

#[embassy_executor::task]
async fn gps_pps_task(mut pps: ExtiInput<'static, Async>, mut led: Output<'static>) -> ! {
    loop {
        pps.wait_for_rising_edge().await;
        info!("GPS PPS rising edge detected");

        led.set_high();
        Timer::after(Duration::from_millis(75)).await;
        led.set_low();
    }
}

#[embassy_executor::task]
async fn gps_serial_task(
    usart: Peri<'static, USART2>,
    tx: Peri<'static, PA2>,
    rx: Peri<'static, PA3>,
) -> ! {
    let mut config = Config::default();
    config.baudrate = 9_600;
    config.data_bits = DataBits::DataBits8;
    config.parity = Parity::ParityNone;
    config.stop_bits = StopBits::STOP1;

    static mut TX_BUFFER: [u8; 16] = [0; 16];
    static mut RX_BUFFER: [u8; 256] = [0; 256];

    // This task is spawned once, so these buffers are handed to exactly one UART.
    let tx_buffer = unsafe { &mut *core::ptr::addr_of_mut!(TX_BUFFER) };
    let rx_buffer = unsafe { &mut *core::ptr::addr_of_mut!(RX_BUFFER) };

    let mut uart = unwrap!(BufferedUart::new(
        usart, rx, tx, tx_buffer, rx_buffer, Irqs, config
    ));

    let mut line = [0u8; 128];
    let mut len = 0usize;
    let mut buf = [0u8; 32];

    info!("Listening on GPS USART2 at 9600 8N1");

    loop {
        match uart.read(&mut buf).await {
            Ok(n) => {
                for &b in &buf[..n] {
                    if b == b'\n' || len == line.len() {
                        log_gps_message(&line[..len]);
                        len = 0;
                    } else if b != b'\r' {
                        line[len] = b;
                        len += 1;
                    }
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
