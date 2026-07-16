// Voltage monitor driver proof-of-concept.
//
// This test owns ADC1/ADC4 through raylar-drivers, samples all board voltage
// monitor channels once per second, and logs calibrated millivolt state updates.

#![no_std]
#![no_main]

use defmt::{info, unwrap};
use embassy_executor::Spawner;
use embassy_stm32::gpio::Output;
use embassy_stm32::rcc::*;
use embassy_stm32::time::mhz;
use embassy_time::Timer;
use embedded_alloc::LlffHeap as Heap;
use raylar_board_v1p0::{AdcVoltages, Board, Leds};
use raylar_drivers::voltagemonitor::stm32::Stm32VoltageMonitor;
use raylar_drivers::voltagemonitor::{VoltageConfig, VoltageMonitorDriver, VoltageResources};
use {defmt_rtt as _, panic_probe as _};

static VOLTAGES: VoltageResources = VoltageResources::new();
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
    let Board {
        leds, adc_voltages, ..
    } = Board::new(p);
    let Leds {
        sys_main_red,
        sys_main_green,
        ..
    } = leds;

    info!("Voltage monitor driver test started");
    spawner.spawn(unwrap!(heartbeat_task(sys_main_green)));
    spawner.spawn(unwrap!(voltage_observer_task()));

    run_voltage_driver(adc_voltages, sys_main_red).await
}

async fn run_voltage_driver(
    adc_voltages: AdcVoltages<'static>,
    mut activity_led: Output<'static>,
) -> ! {
    let AdcVoltages {
        adc,
        adc4,
        v_dc,
        v_batt,
        v_solar,
    } = adc_voltages;

    let sampler = Stm32VoltageMonitor::new_without_usb(adc, adc4, v_dc, v_batt, v_solar);
    info!("VREFBUF ready={}", sampler.status().vrefbuf_ready);

    activity_led.set_high();
    let driver = VoltageMonitorDriver::new(sampler, &VOLTAGES, VoltageConfig::default());
    driver.run().await
}

#[embassy_executor::task]
async fn voltage_observer_task() -> ! {
    let mut rx = unwrap!(VOLTAGES.state_receiver());

    loop {
        if let Some(state) = rx.try_changed() {
            info!(
                "Voltage state: batt={}mV solar={}mV ext_dc={}mV usb={}mV vref={}mV",
                state.battery_mv, state.solar_mv, state.ext_dc_mv, state.usb_mv, state.vref_mv
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
