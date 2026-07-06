// Simple power-down smoke test.
//
// Activity:
// - Keep GPS disabled.
// - Keep the microSD card power rail off.
// - Hold the LR1121 radio in reset with chip select inactive.
// - Turn the main red LED on for 5 seconds while the CPU is awake.
// - Stay awake while exercising the GPS UART standby command.

#![no_std]
#![no_main]

use defmt::{info, unwrap};
use embassy_executor::Spawner;
use embassy_stm32::gpio::{Flex, Level, Output, Pin, Speed};
use embassy_stm32::rcc::*;
use embassy_stm32::time::mhz;
use embassy_stm32::usart::{Config as UartConfig, DataBits, Parity, StopBits, UartTx};
use embassy_stm32::Peri;
use embassy_time::{Duration, Timer};
use {defmt_rtt as _, panic_probe as _};

fn park_analog(pin: Peri<'static, impl Pin>) -> Flex<'static> {
    let mut pin = Flex::new(pin);
    pin.set_as_analog();
    pin
}

#[embassy_executor::main]
async fn main(_spawner: Spawner) -> ! {
    info!("17_sleep_test main entered");
    defmt::flush();

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
    info!("STM32 init complete");
    defmt::flush();
    let mut sys_gps_green = Output::new(p.PB4, Level::Low, Speed::Low);
    let mut sys_gps_red = Output::new(p.PD7, Level::Low, Speed::Low);
    let mut sys_main_red = Output::new(p.PB15, Level::Low, Speed::Low);
    let mut sys_main_green = Output::new(p.PD10, Level::Low, Speed::Low);
    let mut sys_sd_blue = Output::new(p.PD5, Level::Low, Speed::Low);

    let mut gps_rst = Output::new(p.PE3, Level::Low, Speed::Low);
    let mut gps_en = Output::new(p.PC13, Level::Low, Speed::Low);
    let mut sd_power = Output::new(p.PE0, Level::High, Speed::Low);
    let mut rf_cs = Output::new(p.PE8, Level::High, Speed::Low);
    let mut rf_nrst = Output::new(p.PE11, Level::High, Speed::Low);

    gps_en.set_low();
    gps_rst.set_high();
    info!("GPS_EN=HIGH GPS_RST=HIGH, sending L86 standby command");
    Timer::after_millis(250).await;
    info!("GPS standby command settle complete");

    let mut gps_uart_config = UartConfig::default();
    gps_uart_config.baudrate = 9_600;
    gps_uart_config.data_bits = DataBits::DataBits8;
    gps_uart_config.parity = Parity::ParityNone;
    gps_uart_config.stop_bits = StopBits::STOP1;

    let mut gps_uart = unwrap!(UartTx::new_blocking(p.USART2, p.PA2, gps_uart_config));
    unwrap!(gps_uart.blocking_write(b"$PMTK161,0*28\r\n"));
    Timer::after(Duration::from_millis(250)).await;
    unwrap!(gps_uart.blocking_write(b"$PMTK161,0*28\r\n"));
    Timer::after(Duration::from_millis(250)).await;
    unwrap!(gps_uart.blocking_write(b"$PMTK161,0*28\r\n"));
    Timer::after(Duration::from_millis(250)).await;
    unwrap!(gps_uart.blocking_write(b"$PMTK161,0*28\r\n"));
    Timer::after(Duration::from_millis(250)).await;
    unwrap!(gps_uart.blocking_write(b"$PMTK161,0*28\r\n"));
    unwrap!(gps_uart.blocking_flush());
    drop(gps_uart);
    info!("GPS standby command sent");
    defmt::flush();

    let _parked_unused_pins = (
        park_analog(p.PE2),
        park_analog(p.PA5),
        park_analog(p.PD0),
        park_analog(p.PD1),
        park_analog(p.PA3),
        park_analog(p.PB9),
        park_analog(p.PB8),
        park_analog(p.PD3),
        park_analog(p.PC12),
        park_analog(p.PD2),
        park_analog(p.PC8),
        park_analog(p.PC9),
        park_analog(p.PC10),
        park_analog(p.PC11),
        park_analog(p.PD4),
        park_analog(p.PE10),
        park_analog(p.PE12),
        park_analog(p.PE13),
        park_analog(p.PE14),
        park_analog(p.PE15),
    );

    sys_gps_green.set_low();
    sys_gps_red.set_low();
    sys_main_green.set_low();
    sys_sd_blue.set_low();
    gps_en.set_low(); // GPS enabled
    gps_rst.set_high(); // GPS enabled
    sd_power.set_high();
    rf_cs.set_low();
    rf_nrst.set_low();

    info!("17_sleep_test started");
    info!("GPS standby requested, SD off, radio held in reset");
    info!("Unused GPIOs parked in analog mode");
    info!("STM32 sleep/standby disabled while debugging GPS UART");
    defmt::flush();

    loop {
        sys_main_red.set_high();
        info!("CPU awake, red LED on");
        defmt::flush();
        Timer::after_secs(1).await;

        sys_main_red.set_low();
        info!("CPU awake, red LED off");
        defmt::flush();
        Timer::after_secs(1).await;
    }
}
