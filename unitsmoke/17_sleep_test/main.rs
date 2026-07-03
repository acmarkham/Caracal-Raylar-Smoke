// Simple power-down smoke test.
//
// Activity:
// - Keep GPS disabled.
// - Keep the microSD card power rail off.
// - Hold the LR1121 radio in reset with chip select inactive.
// - Turn the main red LED on for 5 seconds while the CPU is awake.
// - Turn the LED off and let the STM32 enter low-power sleep for 10 seconds.

#![no_std]
#![no_main]

use defmt::{info, unwrap};
use embassy_executor::Spawner;
use embassy_stm32::gpio::{Flex, Level, Output, Pin, Speed};
use embassy_stm32::rcc::*;
use embassy_stm32::time::mhz;
use embassy_stm32::Peri;
use embassy_time::{Duration, Timer};
use {defmt_rtt as _, panic_probe as _};

fn park_analog(pin: Peri<'static, impl Pin>) -> Flex<'static> {
    let mut pin = Flex::new(pin);
    pin.set_as_analog();
    pin
}

#[embassy_executor::task]
async fn app_task() -> ! {
    info!("17_sleep_test app_task entered");
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
    config.min_stop_pause = Duration::from_millis(100);
    config.enable_debug_during_sleep = true;
    config.enable_independent_analog_supply = false;

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
    let mut rf_nrst = Output::new(p.PE11, Level::Low, Speed::Low);

    let _parked_unused_pins = (
        park_analog(p.PE2),
        park_analog(p.PA5),
        park_analog(p.PD0),
        park_analog(p.PD1),
        park_analog(p.PA2),
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
    gps_en.set_low();
    gps_rst.set_low();
    sd_power.set_high();
    rf_cs.set_high();
    rf_nrst.set_low();

    info!("17_sleep_test started");
    info!("GPS off, SD off, radio held in reset");
    info!("Unused GPIOs parked in analog mode");
    info!("Debug probe kept enabled during sleep");
    defmt::flush();

    loop {
        info!("CPU awake for 5 seconds");
        defmt::flush();
        sys_main_red.set_high();
        Timer::after_secs(5).await;

        sys_main_red.set_low();
        info!("Entering low-power sleep for 10 seconds");
        defmt::flush();
        Timer::after_secs(10).await;
    }
}

#[cortex_m_rt::entry]
fn main() -> ! {
    info!("17_sleep_test reset entry");
    defmt::flush();

    let mut executor = embassy_stm32::executor::Executor::new();
    let executor = unsafe {
        core::mem::transmute::<
            &mut embassy_stm32::executor::Executor,
            &'static mut embassy_stm32::executor::Executor,
        >(&mut executor)
    };
    executor.run(|spawner: Spawner| {
        spawner.spawn(unwrap!(app_task()));
    })
}
