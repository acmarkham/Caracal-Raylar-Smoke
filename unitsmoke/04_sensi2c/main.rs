// Test of all connected i2c devices on the SensI2C bus. This is a smoke test to verify that the I2C bus is working and that the devices are responding.

// Activity: This test will scan the I2C bus for devices, and then read the WHO_AM_I register from the LIS2HH12 and LIS2MDL sensors, and read register 0 from the BQ25186 PMIC. The sys_main_red led will be turned on when the I2C bus is being accessed, and turned off when the bus is idle. The sys_main_green led will blink to indicate that the test is running.

// Assumptions: using HSE (16MHz) as the clock source, and the board is powered on and running.

#![no_std]
#![no_main]

use defmt::{info, unwrap};
use embassy_executor::Spawner;
use embassy_stm32::gpio::Output;
use embassy_stm32::mode::Async;
use embassy_time::{Duration, Timer};
use raylar_board_v1p0::{Board, SensI2C, Leds};
use {defmt_rtt as _, panic_probe as _};
// i2c imports
use embassy_stm32::i2c::{Config, I2c};
use embassy_stm32::time::Hertz;
use embassy_stm32::time::mhz;

use embassy_stm32::rcc::*;


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
    //let p = embassy_stm32::init(Default::default());
    let Board { leds, sens_i2c,..} = Board::new(p);
    let Leds {
        sys_main_red,
        sys_main_green,
        ..
    } = leds;
    
    info!("SENS I2C smoke test started");

    spawner.spawn(unwrap!(heartbeat_task(sys_main_green)));
    spawner.spawn(unwrap!(i2c_task(sens_i2c, sys_main_red)));

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
async fn i2c_task(
    sens_i2c: SensI2C<'static>,
    mut led: Output<'static>,
) -> ! {
    let SensI2C{ i2c, scl, sda } = sens_i2c;
    let mut config = Config::default();
    config.frequency = Hertz(100_000);

    let mut i2c = I2c::new_blocking(
        i2c,
        scl,
        sda,
        config,
    );

    let mut whoami = [0u8; 1];

    info!("Scanning I2C bus for devices...");
    for addr in 0x08..0x78 {
    if i2c.blocking_write(addr, &[]).is_ok() {
        info!("Found device at 0x{:02x}", addr);
        }
    }

    loop {
        led.set_high();



        //
        // LIS2HH12
        //
        match i2c.blocking_write_read(0x1D, &[0x0F], &mut whoami) {
            Ok(_) => info!("LIS2HH12 WHO_AM_I = {=u8:#x}", whoami[0]),
            Err(e) => info!("LIS2HH12 error: {:?}", e),
        }

        //
        // LIS2MDL
        //
        match i2c.blocking_write_read(0x1E, &[0x4F], &mut whoami) {
            Ok(_) => info!("LIS2MDL WHO_AM_I = {=u8:#x}", whoami[0]),
            Err(e) => info!("LIS2MDL error: {:?}", e),
        }

        //
        // BQ25186
        //
        match i2c.blocking_write_read(0x6A, &[0x00], &mut whoami) {
            Ok(_) => info!("BQ25186 reg0 = {=u8:#x}", whoami[0]),
            Err(e) => info!("BQ25186 error: {:?}", e),
        }

        led.set_low();

        Timer::after_secs(1).await;
    }
}
