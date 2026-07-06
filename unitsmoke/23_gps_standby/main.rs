// GPS standby smoke test.
//
// Activity: This test enables and releases reset on the GPS module, listens on
// USART2 at 9600 8N1, and alternates every 30 seconds between normal active
// operation and PMTK standby. Serial output is streamed to defmt RTT so the
// NMEA rate and PMTK acknowledgements can be watched while the mode changes.
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
use embassy_stm32::usart::{
    BufferedUart, BufferedUartRx, BufferedUartTx, Config, DataBits, Parity, StopBits,
};
use embassy_time::{Duration, Timer};
use embedded_io_async::{Read, Write};
use raylar_board_v1p0::{Board, Gps, Irqs};
use {defmt_rtt as _, panic_probe as _};

const GPS_STANDBY_COMMAND: &[u8] = b"$PMTK161,0*28\r\n";
const GPS_WAKE_DUMMY: &[u8] = b"\n\n";
const MODE_INTERVAL: Duration = Duration::from_secs(30);

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

    info!("GPS standby smoke test started");
    gps_standby_test(spawner, gps, leds.sys_gps_green).await
}

async fn gps_standby_test(
    spawner: Spawner,
    gps: Gps<'static>,
    serial_led: Output<'static>,
) -> ! {
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

    static mut TX_BUFFER: [u8; 64] = [0; 64];
    static mut RX_BUFFER: [u8; 512] = [0; 512];

    // This test owns one UART instance for its whole lifetime.
    let tx_buffer = unsafe { &mut *core::ptr::addr_of_mut!(TX_BUFFER) };
    let rx_buffer = unsafe { &mut *core::ptr::addr_of_mut!(RX_BUFFER) };
    let uart = unwrap!(BufferedUart::new(
        usart, rx, tx, tx_buffer, rx_buffer, Irqs, config
    ));
    let (uart_tx, uart_rx) = uart.split();

    info!("Listening on GPS USART2 at 9600 8N1");
    info!("GPS starts active; standby toggle interval is 30 seconds");

    spawner.spawn(unwrap!(gps_serial_logger(uart_rx, serial_led)));
    spawner.spawn(unwrap!(gps_mode_toggler(uart_tx)));

    core::future::pending().await
}

#[embassy_executor::task]
async fn gps_serial_logger(mut uart_rx: BufferedUartRx<'static>, mut serial_led: Output<'static>) {
    let mut line = [0u8; 128];
    let mut len = 0usize;
    let mut buf = [0u8; 32];

    loop {
        match uart_rx.read(&mut buf).await {
            Ok(n) => {
                if n > 0 {
                    serial_led.set_high();
                }

                for &b in &buf[..n] {
                    if b == b'\n' || len == line.len() {
                        log_gps_message(&line[..len]);
                        len = 0;
                    } else if b != b'\r' {
                        line[len] = b;
                        len += 1;
                    }
                }

                serial_led.set_low();
            }
            Err(e) => {
                info!("GPS UART read error: {=?}", e);
            }
        }
    }
}

#[embassy_executor::task]
async fn gps_mode_toggler(mut uart_tx: BufferedUartTx<'static>) {
    let mut active = true;

    loop {
        Timer::after(MODE_INTERVAL).await;

        if active {
            info!("GPS mode: entering standby with PMTK161");
            match uart_tx.write_all(GPS_STANDBY_COMMAND).await {
                Ok(()) => match uart_tx.flush().await {
                    Ok(()) => info!("GPS standby command sent"),
                    Err(e) => info!("GPS UART flush error after standby command: {=?}", e),
                },
                Err(e) => info!("GPS UART write error sending standby command: {=?}", e),
            }
        } else {
            info!("GPS mode: waking from standby with dummy newline bytes");
            match uart_tx.write_all(GPS_WAKE_DUMMY).await {
                Ok(()) => match uart_tx.flush().await {
                    Ok(()) => info!("GPS wake dummy bytes sent"),
                    Err(e) => info!("GPS UART flush error after wake bytes: {=?}", e),
                },
                Err(e) => info!("GPS UART write error sending wake bytes: {=?}", e),
            }
        }

        active = !active;
    }
}

fn log_gps_message(bytes: &[u8]) {
    if bytes.is_empty() {
        return;
    }

    if is_standby_ack(bytes) {
        info!("GPS standby ACK: {=[u8]}", bytes);
    }

    match str::from_utf8(bytes) {
        Ok(message) => info!("GPS: {}", message),
        Err(_) => info!("GPS bytes: {=[u8]}", bytes),
    }
}

fn is_standby_ack(bytes: &[u8]) -> bool {
    bytes.starts_with(b"$PMTK001,161,3*")
}
