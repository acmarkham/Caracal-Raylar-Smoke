// Battery charger driver proof-of-concept.
//
// This test owns the SENS_I2C BQ25186 through raylar-drivers, applies the
// default 200 mA charge current, and logs published charger state once per
// second.

#![no_std]
#![no_main]

use defmt::{info, unwrap, warn};
use embassy_executor::Spawner;
use embassy_stm32::gpio::Output;
use embassy_stm32::i2c::{Config as I2cConfig, I2c};
use embassy_stm32::rcc::*;
use embassy_stm32::time::{mhz, Hertz};
use embassy_time::Timer;
use embedded_alloc::LlffHeap as Heap;
use raylar_board_v1p0::{Board, Leds, SensI2C};
use raylar_drivers::batterycharger::{ChargerConfig, ChargerDriver, ChargerResources};
use {defmt_rtt as _, panic_probe as _};

static CHARGER: ChargerResources = ChargerResources::new();
const HEAP_BYTES: usize = 8 * 1024;

#[global_allocator]
static HEAP: Heap = Heap::empty();

#[embassy_executor::main]
async fn main(spawner: Spawner) -> ! {
    unsafe {
        embedded_alloc::init!(HEAP, HEAP_BYTES);
    }

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
    let Board { leds, sens_i2c, .. } = Board::new(p);
    let Leds {
        sys_main_red,
        sys_main_green,
        ..
    } = leds;

    info!("Battery charger driver test started");
    spawner.spawn(unwrap!(heartbeat_task(sys_main_green)));
    spawner.spawn(unwrap!(charger_observer_task()));

    run_charger_driver(sens_i2c, sys_main_red).await
}

async fn run_charger_driver(sens_i2c: SensI2C<'static>, mut activity_led: Output<'static>) -> ! {
    let SensI2C { i2c, scl, sda } = sens_i2c;
    let mut i2c_config = I2cConfig::default();
    i2c_config.frequency = Hertz(100_000);
    let i2c = I2c::new_blocking(i2c, scl, sda, i2c_config);

    let mut driver = ChargerDriver::new(i2c, &CHARGER, ChargerConfig::default());

    match driver.initialize() {
        Ok(state) => info!(
            "BQ25186 initialized: charging={} state={} fault={}",
            state.charging, state.state, state.fault
        ),
        Err(_) => warn!("BQ25186 initialization failed"),
    }

    loop {
        activity_led.set_high();
        match driver.refresh_state() {
            Ok(state) => info!(
                "BQ25186 refresh: charging={} state={} fault={}",
                state.charging, state.state, state.fault
            ),
            Err(_) => warn!("BQ25186 status refresh failed"),
        }
        activity_led.set_low();

        Timer::after_secs(1).await;
    }
}

#[embassy_executor::task]
async fn charger_observer_task() -> ! {
    let mut rx = unwrap!(CHARGER.state_receiver());

    loop {
        if let Some(state) = rx.try_changed() {
            info!(
                "Charger state: charging={} state={} fault={}",
                state.charging, state.state, state.fault
            );
        }
        Timer::after_millis(100).await;
    }
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
