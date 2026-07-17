#![no_std]
#![no_main]

use defmt::{info, unwrap, warn};
use embassy_executor::Spawner;
use embassy_stm32::gpio::{Input, Output, Pull};
use embassy_stm32::i2c::{mode::Master, Config as I2cConfig, I2c};
use embassy_stm32::mode::Blocking;
use embassy_stm32::peripherals::{PA0, PA1, PB1};
use embassy_stm32::rcc::*;
use embassy_stm32::time::{mhz, Hertz};
use embassy_stm32::Peri;
use embassy_time::Timer;
use embedded_alloc::LlffHeap as Heap;
use raylar_board_v1p0::{AdcVoltages, Board, Leds, SensI2C, UsbCdc};
use raylar_drivers::batterycharger::{ChargerConfig, ChargerDriver, ChargerResources};
use raylar_drivers::voltagemonitor::stm32::Stm32VoltageMonitor;
use raylar_drivers::voltagemonitor::{VoltageConfig, VoltageMonitorDriver, VoltageResources};
use raylar_power_management_service::{PowerConfig, PowerManagementService, PowerResources};
use {defmt_rtt as _, panic_probe as _};

static VOLTAGES: VoltageResources = VoltageResources::new();
static CHARGER: ChargerResources = ChargerResources::new();
static POWER: PowerResources = PowerResources::new();
const HEAP_BYTES: usize = 8 * 1024;

type BoardVoltageMonitor =
    Stm32VoltageMonitor<Peri<'static, PA0>, Peri<'static, PA1>, Peri<'static, PB1>, Input<'static>>;
type BoardVoltageDriver = VoltageMonitorDriver<BoardVoltageMonitor>;
type BoardChargerDriver = ChargerDriver<I2c<'static, Blocking, Master>>;

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
        leds,
        adc_voltages,
        sens_i2c,
        usb_cdc,
        ..
    } = Board::new(p);
    let Leds { sys_main_green, .. } = leds;

    info!("Power management service test started");
    spawner.spawn(unwrap!(heartbeat_task(sys_main_green)));
    start_services(spawner, adc_voltages, sens_i2c, usb_cdc).await
}

async fn start_services(
    spawner: Spawner,
    adc_voltages: AdcVoltages<'static>,
    sens_i2c: SensI2C<'static>,
    usb_cdc: UsbCdc<'static>,
) -> ! {
    let voltage_driver = build_voltage_driver(adc_voltages, usb_cdc);
    let charger_driver = build_charger_driver(sens_i2c);
    let power_service = PowerManagementService::new(
        &POWER,
        unwrap!(VOLTAGES.state_receiver()).as_dyn(),
        unwrap!(CHARGER.state_receiver()).as_dyn(),
        PowerConfig::default(),
    );

    spawner.spawn(unwrap!(voltage_driver_task(voltage_driver)));
    spawner.spawn(unwrap!(charger_driver_task(charger_driver)));
    spawner.spawn(unwrap!(power_service_task(power_service)));
    spawner.spawn(unwrap!(power_observer_task()));
    core::future::pending().await
}

fn build_voltage_driver(
    adc_voltages: AdcVoltages<'static>,
    usb_cdc: UsbCdc<'static>,
) -> BoardVoltageDriver {
    let AdcVoltages {
        adc,
        adc4,
        v_dc,
        v_batt,
        v_solar,
    } = adc_voltages;
    let UsbCdc { vbus, .. } = usb_cdc;
    let usb_present = Input::new(vbus, Pull::None);
    let sampler = Stm32VoltageMonitor::new(adc, adc4, v_dc, v_batt, v_solar, usb_present);

    VoltageMonitorDriver::new(sampler, &VOLTAGES, VoltageConfig::default())
}

fn build_charger_driver(sens_i2c: SensI2C<'static>) -> BoardChargerDriver {
    let SensI2C { i2c, scl, sda } = sens_i2c;
    let mut i2c_config = I2cConfig::default();
    i2c_config.frequency = Hertz(100_000);
    let i2c = I2c::new_blocking(i2c, scl, sda, i2c_config);

    ChargerDriver::new(i2c, &CHARGER, ChargerConfig::default())
}

#[embassy_executor::task]
async fn voltage_driver_task(driver: BoardVoltageDriver) -> ! {
    driver.run().await
}

#[embassy_executor::task]
async fn charger_driver_task(mut driver: BoardChargerDriver) -> ! {
    match driver.initialize() {
        Ok(state) => info!(
            "BQ25186 initialized: charging={} state={} fault={}",
            state.charging, state.state, state.fault
        ),
        Err(_) => warn!("BQ25186 initialization failed"),
    }

    loop {
        if driver.refresh_state().is_err() {
            warn!("BQ25186 status refresh failed");
        }
        Timer::after_secs(1).await;
    }
}

#[embassy_executor::task]
async fn power_service_task(service: PowerManagementService) -> ! {
    service.run().await
}

#[embassy_executor::task]
async fn power_observer_task() -> ! {
    loop {
        let state = POWER.state();
        info!(
            "Power: source={} batt={}mV solar={}mV ext_dc={}mV charging={} percent={} health={} charger_state={} charger_fault={}",
            state.source,
            state.battery_mv,
            state.solar_mv,
            state.ext_dc_mv,
            state.charging,
            state.battery_percent,
            state.health,
            state.charger.state,
            state.charger.fault
        );
        Timer::after_secs(1).await;
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
