// Test of the magnetometer on the SensI2C bus. This is a smoke test to verify that the I2C bus is working and that the magnetometer is responding with data.

// Activity: This test will read the magnetometer data from the LIS2MDL sensor, and print the X, Y, and Z magnetic field values to the console. The sys_main_red led will be turned on when the I2C bus is being accessed, and turned off when the bus is idle. The sys_main_green led will blink to indicate that the test is running.

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

    //
    // LIS2MDL
    //
    const LIS2MDL_ADDR: u8 = 0x1E;
    const REG_WHO_AM_I: u8 = 0x4F;
    const REG_CTRL1: u8 = 0x60;
    const REG_CTRL2: u8 = 0x61;
    const REG_CTRL3: u8 = 0x62;
    const REG_OUT_X_L: u8 = 0x68;
    const WHO_AM_I_EXPECTED: u8 = 0x40;

    // Check whoami
    match i2c.blocking_write_read(LIS2MDL_ADDR, &[REG_WHO_AM_I], &mut whoami) {
        Ok(_) => info!("LIS2MDL WHO_AM_I = {=u8:#x}", whoami[0]),
        Err(e) => info!("LIS2MDL error: {:?}", e),
    }

    // start:
    // continuous mode
    let ctrl1 = 0x10;

    match i2c.blocking_write(
        LIS2MDL_ADDR,
        &[REG_CTRL1, ctrl1],
    ){
        Ok(_) => info!("Started"),
        Err(e) => info!("CONFIG:LIS2MDL error: {:?}", e),
    }
    // set to continuous mode
    unwrap!(i2c.blocking_write(
        LIS2MDL_ADDR,
        &[REG_CTRL3, 0x00],
    ));

    loop {
        led.set_high();
        let mut raw = [0u8; 6];
        match   i2c.blocking_write_read(
            LIS2MDL_ADDR,
            &[REG_OUT_X_L | 0x80],
            &mut raw,
        ) {
            Ok(_) => {
                let x = i16::from_le_bytes([raw[0], raw[1]]);
                let y = i16::from_le_bytes([raw[2], raw[3]]);
                let z = i16::from_le_bytes([raw[4], raw[5]]);
                info!(
                    "MAG x={} y={} z={}",
                    x,
                    y,
                    z
                );},
            Err(e) => info!("LIS2MDL error: {:?}", e),
        }
        
        led.set_low();

        Timer::after_secs(1).await;
    }
}
