use crate::{common, policy::Activity, radio::RADIO, MICROPHONES};
use core::cell::RefCell;
use defmt::{error, unwrap};
use embassy_executor::Spawner;
use embassy_stm32::{
    gpio::{Input, Pull},
    i2c::{mode::Master, Config as I2cConfig, I2c},
    mode::Blocking,
    peripherals::{PA0, PA1, PB1},
    time::Hertz,
};
use embassy_sync::{
    blocking_mutex::{
        raw::{CriticalSectionRawMutex, NoopRawMutex},
        Mutex as BlockingMutex,
    },
    watch::Watch,
};
use embassy_time::{Instant, Timer};
use raylar_board_v1p0::{AdcVoltages, SensI2C, UsbCdc};
use raylar_drivers::voltagemonitor::stm32::Stm32VoltageMonitor;
use raylar_drivers::{
    batterycharger::{ChargerBus, ChargerConfig, ChargerDriver, ChargerResources},
    gps::GpsCommand,
    voltagemonitor::{VoltageConfig, VoltageMonitorDriver, VoltageResources},
};
use raylar_logging_service::{info as log_info, LoggerHandle};
use raylar_power_management_service::{PowerConfig, PowerManagementService, PowerResources};
use static_cell::StaticCell;

pub static VOLTAGES: VoltageResources = VoltageResources::new();
pub static CHARGER: ChargerResources = ChargerResources::new();
pub static POWER: PowerResources = PowerResources::new();
pub static ACTIVITY: Watch<CriticalSectionRawMutex, Activity, 4> =
    Watch::new_with(Activity::EnergyRecovery);
type Bus = I2c<'static, Blocking, Master>;
type BusMutex = BlockingMutex<NoopRawMutex, RefCell<Bus>>;
static SENSOR_BUS: StaticCell<BusMutex> = StaticCell::new();
type VoltageDriver = VoltageMonitorDriver<
    Stm32VoltageMonitor<
        embassy_stm32::Peri<'static, PA0>,
        embassy_stm32::Peri<'static, PA1>,
        embassy_stm32::Peri<'static, PB1>,
        Input<'static>,
    >,
>;

#[derive(Clone, Copy)]
struct SharedI2c {
    bus: &'static BusMutex,
}
impl embedded_hal::i2c::ErrorType for SharedI2c {
    type Error = embassy_stm32::i2c::Error;
}
impl embedded_hal::i2c::I2c for SharedI2c {
    fn transaction(
        &mut self,
        address: u8,
        operations: &mut [embedded_hal::i2c::Operation<'_>],
    ) -> Result<(), Self::Error> {
        self.bus.lock(|bus| {
            embedded_hal::i2c::I2c::transaction(&mut *bus.borrow_mut(), address, operations)
        })
    }
}
impl ChargerBus for SharedI2c {
    type Error = embassy_stm32::i2c::Error;
    fn read_register(&mut self, register: u8) -> Result<u8, Self::Error> {
        let mut value = [0];
        embedded_hal::i2c::I2c::write_read(
            self,
            raylar_drivers::batterycharger::BQ25186_ADDRESS,
            &[register],
            &mut value,
        )?;
        Ok(value[0])
    }
    fn write_register(&mut self, register: u8, value: u8) -> Result<(), Self::Error> {
        embedded_hal::i2c::I2c::write(
            self,
            raylar_drivers::batterycharger::BQ25186_ADDRESS,
            &[register, value],
        )
    }
}

pub async fn start(
    spawner: Spawner,
    adc: AdcVoltages<'static>,
    sens: SensI2C<'static>,
    usb: UsbCdc<'static>,
    log: LoggerHandle<'static, 384, 32>,
) {
    let SensI2C { i2c, scl, sda } = sens;
    let mut i2c_config = I2cConfig::default();
    i2c_config.frequency = Hertz(100_000);
    let bus = SharedI2c {
        bus: SENSOR_BUS.init(BlockingMutex::new(RefCell::new(I2c::new_blocking(
            i2c, scl, sda, i2c_config,
        )))),
    };
    let AdcVoltages {
        adc,
        adc4,
        v_dc,
        v_batt,
        v_solar,
    } = adc;
    let UsbCdc { vbus, .. } = usb;
    let voltage = VoltageMonitorDriver::new(
        Stm32VoltageMonitor::new(
            adc,
            adc4,
            v_dc,
            v_batt,
            v_solar,
            Input::new(vbus, Pull::None),
        ),
        &VOLTAGES,
        VoltageConfig::default(),
    );
    let mut charger_config = ChargerConfig::default();
    charger_config.default_charge_current_ma = 200;
    charger_config.default_input_current_limit_ma = 200;
    let charger = ChargerDriver::new(bus, &CHARGER, charger_config);
    let service = PowerManagementService::new(
        &POWER,
        unwrap!(VOLTAGES.state_receiver()).as_dyn(),
        unwrap!(CHARGER.state_receiver()).as_dyn(),
        PowerConfig::default(),
    );
    RADIO.set_enabled(false);
    MICROPHONES.set_enabled(false);
    spawner.spawn(unwrap!(voltage_task(voltage)));
    spawner.spawn(unwrap!(charger_task(charger)));
    spawner.spawn(unwrap!(power_task(service)));
    spawner.spawn(unwrap!(battery_policy_task(log)));
}

#[embassy_executor::task]
async fn voltage_task(driver: VoltageDriver) -> ! {
    driver.run().await
}
#[embassy_executor::task]
async fn charger_task(mut driver: ChargerDriver<SharedI2c>) -> ! {
    if driver.initialize().is_err() || driver.enable().is_err() {
        error!("charger startup failed");
    }
    loop {
        if driver.refresh_state().is_err() {
            error!("charger telemetry failed");
        }
        Timer::after_secs(1).await;
    }
}
#[embassy_executor::task]
async fn power_task(service: PowerManagementService) -> ! {
    service.run().await
}

#[embassy_executor::task]
async fn battery_policy_task(log: LoggerHandle<'static, 384, 32>) -> ! {
    let mut state = if cfg!(feature = "bench-external-power") {
        Activity::Active
    } else {
        Activity::EnergyRecovery
    };
    let mut recovery_started = Instant::now();
    let mut receiver = unwrap!(POWER.state_receiver());
    if state.active() {
        ACTIVITY.sender().send(state);
        RADIO.set_enabled(true);
        MICROPHONES.set_enabled(true);
        common::GPS_RESOURCES
            .command_sender()
            .send(GpsCommand::Start)
            .await;
        let _ = log_info!(log, "bench external power startup gate bypass active=true");
    } else {
        common::GPS_RESOURCES
            .command_sender()
            .send(GpsCommand::Stop)
            .await;
    }
    loop {
        let power = receiver.changed().await;
        if state.update(power.battery_percent) {
            let now = Instant::now();
            let recovery_seconds = now.saturating_duration_since(recovery_started).as_secs();
            if !state.active() {
                recovery_started = now;
            }
            let utc_us = common::TIME_RESOURCES
                .time_state()
                .system_to_utc_holdover(now)
                .ok()
                .map(|v| v.as_micros());
            ACTIVITY.sender().send(state);
            let active = state.active();
            RADIO.set_enabled(active);
            if active || !crate::audio::recording_active() {
                MICROPHONES.set_enabled(active && crate::audio::capture_allowed());
            }
            common::GPS_RESOURCES
                .command_sender()
                .send(if active {
                    GpsCommand::Start
                } else {
                    GpsCommand::Stop
                })
                .await;
            let _ = log_info!(
                log,
                "energy active={} soc={:?} battery_mv={} solar_mv={} source={:?} charging={} ticks={} utc_us={:?} recovery_s={}",
                active,
                power.battery_percent,
                power.battery_mv,
                power.solar_mv,
                power.source,
                power.charging, now.as_ticks(), utc_us, recovery_seconds
            );
        }
    }
}
