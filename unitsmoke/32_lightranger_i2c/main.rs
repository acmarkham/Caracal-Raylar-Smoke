//! LightRanger 14 Click / TMF8829 I2C smoke test.
//!
//! Wiring uses the Raylar MikroBUS connector: PC0/I2C3_SCL, PC1/I2C3_SDA,
//! PB2/enable (PWM), and PB5/active-low data-ready (INT). The test resets the
//! sensor, downloads the vendor RAM firmware, selects the default 8x8 map,
//! starts 500 ms measurements, and prints frames over RTT.

#![no_std]
#![no_main]

mod firmware;
mod tmf8829;

use defmt::{error, info, unwrap, warn};
use embassy_executor::Spawner;
use embassy_stm32::gpio::Output;
use embassy_stm32::i2c::{Config as I2cConfig, I2c};
use embassy_stm32::rcc::{Hse, HseMode, Pll, PllDiv, PllMul, PllPreDiv, PllSource, Sysclk};
use embassy_stm32::time::{mhz, Hertz};
use embassy_time::Timer;
use raylar_board_v1p0::{Board, Leds, Mbus};
use tmf8829::{Frame, Tmf8829, MAP_HEIGHT, MAP_WIDTH};
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

    let peripherals = embassy_stm32::init(config);
    let Board { leds, mbus, .. } = Board::new(peripherals);
    let Leds {
        sys_main_red,
        sys_main_green,
        ..
    } = leds;

    info!("LightRanger 14 / TMF8829 I2C smoke test started");
    info!("MBUS I2C3: SCL=PC0 SDA=PC1 EN=PB2 INT=PB5 address=0x41");

    spawner.spawn(unwrap!(heartbeat_task(sys_main_green)));
    spawner.spawn(unwrap!(lightranger_task(mbus, sys_main_red)));

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
async fn lightranger_task(mbus: Mbus<'static>, mut error_led: Output<'static>) -> ! {
    let Mbus {
        i2c,
        scl,
        sda,
        mut enable,
        mut interrupt,
    } = mbus;

    let mut i2c_config = I2cConfig::default();
    i2c_config.frequency = Hertz(400_000);
    let mut i2c = I2c::new_blocking(i2c, scl, sda, i2c_config);
    let mut sensor = Tmf8829::new(&mut i2c);

    let info = loop {
        error_led.set_high();
        info!(
            "Resetting TMF8829 and loading {}-byte RAM firmware",
            firmware::IMAGE.len()
        );
        match sensor.initialize(&mut enable).await {
            Ok(info) => break info,
            Err(err) => {
                error!("TMF8829 initialization failed: {}; retrying in 2 s", err);
                Timer::after_secs(2).await;
            }
        }
    };
    error_led.set_low();

    info!(
        "Firmware application version: {}.{}.{}.{}",
        info.app_version[0], info.app_version[1], info.app_version[2], info.app_version[3]
    );
    info!(
        "Chip version: {}.{}",
        info.chip_version[0], info.chip_version[1]
    );
    info!("Serial number: 0x{:08x}", info.serial_number);
    info!("8x8 ranging started; distance rows are reported in 0.1 mm units");

    loop {
        // TMF8829 INT is active low. wait_for_low also handles a frame that
        // became ready before the task reached this point.
        interrupt.wait_for_low().await;

        if let Err(err) = sensor.clear_interrupts() {
            error_led.set_high();
            warn!("Failed to clear TMF8829 interrupt: {}", err);
            Timer::after_millis(10).await;
            continue;
        }

        match sensor.read_frame() {
            Ok(frame) => {
                error_led.set_low();
                log_frame(&frame);
            }
            Err(err) => {
                error_led.set_high();
                warn!("Failed to read TMF8829 frame: {}", err);
            }
        }
    }
}

fn log_frame(frame: &Frame) {
    info!("SysTick: {}", frame.systick);
    info!("Frame number: {}", frame.frame_number);

    if !frame.is_valid_result() {
        warn!(
            "Ignoring non-result/invalid frame: id=0x{:02x} status=0x{:02x}",
            frame.frame_id, frame.status
        );
    } else {
        for row in 0..MAP_HEIGHT {
            let mut distances_x10_mm = [-1_i32; MAP_WIDTH];
            let mut confidences = [0_u32; MAP_WIDTH];

            for column in 0..MAP_WIDTH {
                // The click mounts the sensor 90 degrees clockwise. This is
                // the vendor example's 90-degree counter-clockwise rotation.
                let source_pixel = column * MAP_WIDTH + (MAP_HEIGHT - 1 - row);
                let pixel = frame.pixel(source_pixel);
                confidences[column] = tmf8829::confidence(pixel.confidence_code);

                if confidences[column] > tmf8829::CONFIDENCE_THRESHOLD {
                    distances_x10_mm[column] =
                        ((u32::from(pixel.distance_quarter_mm) * 10 + 2) / 4) as i32;
                }
            }

            info!(
                "row {} distance_x10_mm (-1=invalid): {:?}",
                row, &distances_x10_mm
            );
            info!("row {} confidence: {:?}", row, &confidences);
        }
    }

    let temperature_x10 = (u32::from(frame.temperature[0])
        + u32::from(frame.temperature[1])
        + u32::from(frame.temperature[2]))
        * 10
        / 3;
    info!(
        "Temperature: {}.{} degC",
        temperature_x10 / 10,
        temperature_x10 % 10
    );
    info!("Status: 0x{:02x}", frame.status);
    info!("-------------------------------");
}
