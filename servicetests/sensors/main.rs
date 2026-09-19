#![no_std]
#![no_main]

use defmt::{info, unwrap};
use embassy_executor::Spawner;
use embassy_stm32::rcc::*;
use embassy_stm32::time::mhz;
use embassy_time::Duration;
use embedded_alloc::LlffHeap as Heap;
use raylar_sensor_service::composites::{
    ECompassConfig, ECompassSensor, HeadingConfig, HeadingSensor, TiltConfig, TiltSensor,
};
use raylar_sensor_service::{
    AbsoluteComparison, SensorDescriptor, SensorId, SensorKind, SensorOrigin, SensorRegistration,
    SensorResources, SensorService, SensorSource, SensorSourceError, SensorValue, ThresholdId,
    ThresholdRule, ValueSelector,
};
use static_cell::StaticCell;
use {defmt_rtt as _, panic_probe as _};

const ACCELERATION: SensorId = SensorId(1);
const MAGNETIC_FIELD: SensorId = SensorId(2);
const ACC_TEMPERATURE: SensorId = SensorId(10);
const MAG_TEMPERATURE: SensorId = SensorId(11);
const CORE_TEMPERATURE: SensorId = SensorId(12);
const TILT: SensorId = SensorId(20);
const HEADING: SensorId = SensorId(21);
const ECOMPASS: SensorId = SensorId(22);
const HEAP_BYTES: usize = 8 * 1024;

#[global_allocator]
static HEAP: Heap = Heap::empty();
static RESOURCES: SensorResources = SensorResources::new();

struct FixedSource(SensorValue);

impl SensorSource for FixedSource {
    fn sample(&mut self) -> Result<SensorValue, SensorSourceError> {
        Ok(self.0)
    }
}

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
    let _peripherals = embassy_stm32::init(config);

    static ACC: StaticCell<FixedSource> = StaticCell::new();
    static MAG: StaticCell<FixedSource> = StaticCell::new();
    static ACC_TEMP: StaticCell<FixedSource> = StaticCell::new();
    static MAG_TEMP: StaticCell<FixedSource> = StaticCell::new();
    static CORE_TEMP: StaticCell<FixedSource> = StaticCell::new();
    let acc = ACC.init(FixedSource(SensorValue::AccelerationMg {
        x: 0,
        y: 0,
        z: 1_000,
    }));
    let mag = MAG.init(FixedSource(SensorValue::MagneticFieldNt {
        x: 25_000,
        y: 0,
        z: 5_000,
    }));
    let acc_temp = ACC_TEMP.init(FixedSource(SensorValue::TemperatureMilliCelsius(31_000)));
    let mag_temp = MAG_TEMP.init(FixedSource(SensorValue::TemperatureMilliCelsius(32_000)));
    let core_temp = CORE_TEMP.init(FixedSource(SensorValue::TemperatureMilliCelsius(41_000)));

    let mut service: SensorService<'static> = SensorService::new(&RESOURCES);
    register(
        &mut service,
        ACCELERATION,
        SensorKind::Acceleration,
        SensorOrigin::Lis2hh12,
        acc,
    );
    register(
        &mut service,
        MAGNETIC_FIELD,
        SensorKind::MagneticField,
        SensorOrigin::Lis2mdl,
        mag,
    );
    register(
        &mut service,
        ACC_TEMPERATURE,
        SensorKind::Temperature,
        SensorOrigin::Lis2hh12,
        acc_temp,
    );
    register(
        &mut service,
        MAG_TEMPERATURE,
        SensorKind::Temperature,
        SensorOrigin::Lis2mdl,
        mag_temp,
    );
    register(
        &mut service,
        CORE_TEMPERATURE,
        SensorKind::Temperature,
        SensorOrigin::Stm32Core,
        core_temp,
    );

    assert!(service
        .register_composite(TiltSensor::new(TiltConfig::new(TILT, ACCELERATION)))
        .is_ok());
    assert!(service
        .register_composite(HeadingSensor::new(HeadingConfig::new(
            HEADING,
            MAGNETIC_FIELD,
        )))
        .is_ok());
    assert!(service
        .register_composite(ECompassSensor::new(ECompassConfig::new(
            ECOMPASS,
            ACCELERATION,
            MAGNETIC_FIELD,
        )))
        .is_ok());
    assert!(service
        .set_threshold(ThresholdRule::Absolute {
            id: ThresholdId(1),
            sensor: CORE_TEMPERATURE,
            selector: ValueSelector::Scalar,
            comparison: AbsoluteComparison::Above,
            threshold: 40_000,
            hysteresis: 1_000,
            trigger_on_initial: true,
        })
        .is_ok());

    spawner.spawn(unwrap!(sensor_task(service)));
    spawner.spawn(unwrap!(state_observer_task()));
    spawner.spawn(unwrap!(event_observer_task()));
    core::future::pending().await
}

fn register(
    service: &mut SensorService<'static>,
    id: SensorId,
    kind: SensorKind,
    origin: SensorOrigin,
    source: &'static mut dyn SensorSource,
) {
    assert!(service
        .register_source(
            SensorRegistration::new(SensorDescriptor { id, kind, origin }, source)
                .with_interval(Duration::from_secs(10))
        )
        .is_ok());
}

#[embassy_executor::task]
async fn sensor_task(service: SensorService<'static>) -> ! {
    service.run().await
}

#[embassy_executor::task]
async fn state_observer_task() -> ! {
    let mut receiver = unwrap!(RESOURCES.state_receiver());
    loop {
        let snapshot = receiver.changed().await;
        info!(
            "sensor snapshot generation={} readings={} successful_polls={}",
            snapshot.generation,
            snapshot.readings.len(),
            snapshot.stats.polls_succeeded
        );
    }
}

#[embassy_executor::task]
async fn event_observer_task() -> ! {
    let receiver = RESOURCES.event_receiver();
    loop {
        let event = receiver.receive().await;
        info!(
            "sensor event sequence={} threshold={} sensor={} current={}",
            event.sequence, event.threshold_id.0, event.sensor_id.0, event.current
        );
    }
}
