// LIS2MDL magnetometer driver hardware test.

#![no_std]
#![no_main]

use defmt::{error, info, warn};
use embassy_executor::Spawner;
use embassy_stm32::i2c::{Config as I2cConfig, I2c};
use embassy_stm32::rcc::*;
use embassy_stm32::time::{mhz, Hertz};
use embassy_time::Timer;
use embedded_alloc::LlffHeap as Heap;
use raylar_board_v1p0::{Board, Leds, SensI2C};
use raylar_drivers::sensor_mag::{Error, Lis2mdl, DEVICE_ID};
use {defmt_rtt as _, panic_probe as _};

const HEAP_BYTES: usize = 8 * 1024;

#[global_allocator]
static HEAP: Heap = Heap::empty();

#[embassy_executor::main]
async fn main(_spawner: Spawner) -> ! {
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
    let Board { sens_i2c, leds, .. } = Board::new(p);
    let Leds {
        mut sys_main_red,
        mut sys_main_green,
        ..
    } = leds;

    let SensI2C { i2c, scl, sda } = sens_i2c;
    let mut i2c_config = I2cConfig::default();
    i2c_config.frequency = Hertz(100_000);
    let i2c = I2c::new_blocking(i2c, scl, sda, i2c_config);

    let mut magnetometer = match Lis2mdl::new(i2c) {
        Ok(driver) => {
            info!("LIS2MDL detected, WHO_AM_I={=u8:#x}", DEVICE_ID);
            driver
        }
        Err(Error::DeviceIdMismatch { observed }) => {
            error!("LIS2MDL identity mismatch: observed={=u8:#x}", observed);
            fail_forever().await
        }
        Err(Error::Bus(error)) => {
            error!("LIS2MDL initial I2C error: {:?}", error);
            fail_forever().await
        }
        Err(Error::NotEnabled) => unreachable!(),
    };

    if let Err(error) = magnetometer.turn_on() {
        error!("LIS2MDL turn_on failed: {:?}", error);
        fail_forever().await;
    }
    info!("LIS2MDL enabled in high-resolution continuous mode at 10Hz");
    Timer::after_millis(120).await;

    loop {
        sys_main_red.set_high();

        match magnetometer.read_raw_magnetic_field() {
            Ok(raw) => {
                let field = raw.to_nanotesla();
                info!(
                    "MAG raw x={} y={} z={}; nT x={} y={} z={}",
                    raw.x, raw.y, raw.z, field.x_nanotesla, field.y_nanotesla, field.z_nanotesla
                );
            }
            Err(error) => warn!("LIS2MDL magnetic-field error: {:?}", error),
        }

        match magnetometer.read_die_temperature() {
            Ok(temp) => info!(
                "LIS2MDL die temp raw={} approx={}.{:03}C",
                temp.raw,
                temp.milli_celsius / 1_000,
                (temp.milli_celsius % 1_000).abs()
            ),
            Err(error) => warn!("LIS2MDL temperature error: {:?}", error),
        }

        sys_main_red.set_low();
        sys_main_green.set_high();
        Timer::after_millis(100).await;
        sys_main_green.set_low();
        Timer::after_millis(900).await;
    }
}

async fn fail_forever() -> ! {
    loop {
        Timer::after_secs(60).await;
    }
}
