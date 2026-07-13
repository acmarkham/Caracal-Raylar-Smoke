// Test of the bq25186 charge controller on the SensI2C bus. This is a smoke test to verify that the I2C bus is working and that the charge controller is responding with data.

// Activity: This test will read the charge controller data from the bq25186 sensor, and print the relevant values to the console. The sys_main_red led will be turned on when the I2C bus is being accessed, and turned off when the bus is idle. The sys_main_green led will blink to indicate that the test is running.

// Assumptions: using HSE (16MHz) as the clock source, and the board is powered on and running.

#![no_std]
#![no_main]

use defmt::{info, unwrap};
use embassy_executor::Spawner;
use embassy_stm32::gpio::Output;
use embassy_stm32::mode::Async;
use embassy_time::{Duration, Timer};
use raylar_board_v1p0::{Board, Leds, SensI2C};
use {defmt_rtt as _, panic_probe as _};
// i2c imports
use embassy_stm32::i2c::{Config, I2c};
use embassy_stm32::time::mhz;
use embassy_stm32::time::Hertz;

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
    let Board { leds, sens_i2c, .. } = Board::new(p);
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
async fn i2c_task(sens_i2c: SensI2C<'static>, mut led: Output<'static>) -> ! {
    let SensI2C { i2c, scl, sda } = sens_i2c;
    let mut config = Config::default();
    config.frequency = Hertz(100_000);

    let mut i2c = I2c::new_blocking(i2c, scl, sda, config);

    let mut whoami = [0u8; 1];

    //
    // bq25186
    //
    const BQ25186_ADDR: u8 = 0x6A;
    const REG_STAT0: u8 = 0x00;
    const REG_STAT1: u8 = 0x01;
    const REG_FLAG0: u8 = 0x02;

    const REG_VBAT_CTRL: u8 = 0x03;
    const REG_ICHG_CTRL: u8 = 0x04;

    const REG_CHARGECTRL0: u8 = 0x05;
    const REG_CHARGECTRL1: u8 = 0x06;

    const REG_IC_CTRL: u8 = 0x07;
    const REG_TMR_ILIM: u8 = 0x08;

    const REG_SHIP_RST: u8 = 0x09;
    const REG_SYS_REG: u8 = 0x0A;
    const REG_TS_CONTROL: u8 = 0x0B;

    const REG_MASK_ID: u8 = 0x0C;

    // Check whoami
    match i2c.blocking_write_read(BQ25186_ADDR, &[REG_MASK_ID], &mut whoami) {
        Ok(_) => info!("BQ25186 WHO_AM_I = {=u8:#x}", whoami[0]),
        Err(e) => info!("BQ25186 error: {:?}", e),
    }

    loop {
        led.set_high();

        let mut stat0 = [0u8; 1];

        match i2c.blocking_write_read(BQ25186_ADDR, &[REG_STAT0], &mut stat0) {
            Ok(_) => {
                let v = stat0[0];

                info!("STAT0 = 0x{=u8:02x}", v);
                let ts_open = (v & 0x80) != 0;
                let chg_stat = (v >> 5) & 0x03;
                let ilim_active = (v & 0x10) != 0;
                let vdppm_active = (v & 0x08) != 0;
                let vindpm_active = (v & 0x04) != 0;
                let thermreg_active = (v & 0x02) != 0;
                let vin_pgood = (v & 0x01) != 0;
                let chg_state = match chg_stat {
                    0b00 => "NotCharging",
                    0b01 => "ConstantCurrent",
                    0b10 => "ConstantVoltage",
                    0b11 => "ChargeDone",
                    _ => "Invalid",
                };
                info!(
                    "BQ25186: TS_OPEN={} CHG={} ILIM={} VDPPM={} VINDPM={} THERM={} VIN_PGOOD={}",
                    ts_open,
                    chg_state,
                    ilim_active,
                    vdppm_active,
                    vindpm_active,
                    thermreg_active,
                    vin_pgood,
                );
            }
            Err(e) => info!("BQ25186 STAT0 read failed: {=?}", e),
        }

        let mut stat1 = [0u8; 1];

        match i2c.blocking_write_read(BQ25186_ADDR, &[REG_STAT1], &mut stat1) {
            Ok(_) => {
                let v = stat1[0];

                let vin_ovp = (v & 0x80) != 0;
                let buvlo = (v & 0x40) != 0;

                let ts_stat = (v >> 3) & 0x03;

                let safety_timer_fault = (v & 0x04) != 0;
                let wake1 = (v & 0x02) != 0;
                let wake2 = (v & 0x01) != 0;

                let ts_state = match ts_stat {
                    0b00 => "Normal",
                    0b01 => "HotOrColdSuspend",
                    0b10 => "CoolCurrentReduced",
                    0b11 => "WarmVoltageReduced",
                    _ => "Invalid",
                };

                info!(
                    "STAT1: VIN_OVP={} BUVLO={} TS={} SAFETY_TMR={} WAKE1={} WAKE2={}",
                    vin_ovp, buvlo, ts_state, safety_timer_fault, wake1, wake2,
                );
            }
            Err(e) => {
                info!("BQ25186 STAT1 read failed: {=?}", e);
            }
        }

        led.set_low();

        Timer::after_secs(1).await;
    }
}
