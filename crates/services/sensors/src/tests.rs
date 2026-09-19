use embassy_time::{Duration, Instant};

use crate::composites::{
    CompositeValidationError, TiltConfig, TiltSensor, VedbaConfig, VedbaSensor,
};
use crate::*;

struct SequenceSource<const N: usize> {
    samples: [Result<SensorValue, SensorSourceError>; N],
    index: usize,
}

impl<const N: usize> SequenceSource<N> {
    fn new(samples: [Result<SensorValue, SensorSourceError>; N]) -> Self {
        Self { samples, index: 0 }
    }
}

impl<const N: usize> SensorSource for SequenceSource<N> {
    fn sample(&mut self) -> Result<SensorValue, SensorSourceError> {
        let index = self.index.min(N - 1);
        self.index = self.index.saturating_add(1);
        self.samples[index]
    }
}

fn descriptor(id: u16, kind: SensorKind) -> SensorDescriptor {
    SensorDescriptor {
        id: SensorId(id),
        kind,
        origin: SensorOrigin::Other(id),
    }
}

#[test]
fn sources_keep_independent_schedules_and_publish_generations() {
    let resources = SensorResources::<4, 1, 2>::new();
    let mut first = SequenceSource::new([Ok(SensorValue::TemperatureMilliCelsius(20_000))]);
    let mut second = SequenceSource::new([Ok(SensorValue::TemperatureMilliCelsius(21_000))]);
    let mut service: SensorService<'_, 8, 4, 8, 1, 8, 2> = SensorService::new(&resources);
    service
        .register_source(SensorRegistration::new(
            descriptor(1, SensorKind::Temperature),
            &mut first,
        ))
        .unwrap();
    service
        .register_source(
            SensorRegistration::new(descriptor(2, SensorKind::Temperature), &mut second)
                .with_interval(Duration::from_secs(20)),
        )
        .unwrap();

    let start = Instant::from_ticks(1);
    assert!(service.poll_one_due(start).is_some());
    assert!(service.poll_one_due(start).is_some());
    assert!(service
        .poll_one_due(start + Duration::from_secs(9))
        .is_none());
    assert!(service
        .poll_one_due(start + Duration::from_secs(10))
        .is_some());
    assert_eq!(resources.state().generation, 3);
    assert_eq!(resources.state().stats.polls_succeeded, 3);
}

#[test]
fn multiple_temperature_sources_are_retained_by_id() {
    let resources = SensorResources::<3, 1, 1>::new();
    let mut first = SequenceSource::new([Ok(SensorValue::TemperatureMilliCelsius(10_000))]);
    let mut second = SequenceSource::new([Ok(SensorValue::TemperatureMilliCelsius(20_000))]);
    let mut service: SensorService<'_, 8, 3, 8, 1, 8, 1> = SensorService::new(&resources);
    service
        .register_source(SensorRegistration::new(
            descriptor(10, SensorKind::Temperature),
            &mut first,
        ))
        .unwrap();
    service
        .register_source(SensorRegistration::new(
            descriptor(11, SensorKind::Temperature),
            &mut second,
        ))
        .unwrap();
    let now = Instant::from_ticks(1);
    service.poll_one_due(now);
    service.poll_one_due(now);

    assert_eq!(
        service
            .state()
            .readings_of_kind(SensorKind::Temperature)
            .count(),
        2
    );
    assert_eq!(
        service.state().reading(SensorId(10)).unwrap().value,
        Some(SensorValue::TemperatureMilliCelsius(10_000))
    );
    assert_eq!(
        service.state().reading(SensorId(11)).unwrap().value,
        Some(SensorValue::TemperatureMilliCelsius(20_000))
    );
}

#[test]
fn duplicate_and_capacity_errors_are_explicit() {
    let resources = SensorResources::<2, 1, 1>::new();
    let mut first = SequenceSource::new([Ok(SensorValue::TemperatureMilliCelsius(1))]);
    let mut duplicate = SequenceSource::new([Ok(SensorValue::TemperatureMilliCelsius(2))]);
    let mut excess = SequenceSource::new([Ok(SensorValue::TemperatureMilliCelsius(3))]);
    let mut service: SensorService<'_, 1, 2, 1, 1, 1, 1> = SensorService::new(&resources);
    service
        .register_source(SensorRegistration::new(
            descriptor(1, SensorKind::Temperature),
            &mut first,
        ))
        .unwrap();
    assert_eq!(
        service.register_source(SensorRegistration::new(
            descriptor(1, SensorKind::Temperature),
            &mut duplicate,
        )),
        Err(RegistrationError::DuplicateId(SensorId(1)))
    );
    assert_eq!(
        service.register_source(SensorRegistration::new(
            descriptor(2, SensorKind::Temperature),
            &mut excess,
        )),
        Err(RegistrationError::SourceCapacity)
    );
}

#[test]
fn failures_keep_last_good_value_and_progress_to_fault() {
    let resources = SensorResources::<1, 1, 1>::new();
    let error = Err(SensorSourceError::new(7));
    let mut source = SequenceSource::new([
        Ok(SensorValue::TemperatureMilliCelsius(25_000)),
        error,
        error,
    ]);
    let mut service: SensorService<'_, 8, 1, 8, 1, 8, 1> = SensorService::new(&resources);
    service.set_failures_before_fault(2);
    service
        .register_source(
            SensorRegistration::new(descriptor(1, SensorKind::Temperature), &mut source)
                .with_interval(Duration::from_secs(1)),
        )
        .unwrap();
    let start = Instant::from_ticks(1);
    service.poll_one_due(start);
    service.poll_one_due(start + Duration::from_secs(1));
    assert_eq!(
        service.state().reading(SensorId(1)).unwrap().status,
        ReadingStatus::Stale
    );
    service.poll_one_due(start + Duration::from_secs(2));
    let reading = service.state().reading(SensorId(1)).unwrap();
    assert_eq!(reading.status, ReadingStatus::Fault);
    assert_eq!(reading.total_errors, 2);
    assert_eq!(reading.last_error, Some(ReadingError::Source(7)));
    assert_eq!(
        reading.value,
        Some(SensorValue::TemperatureMilliCelsius(25_000))
    );
}

#[test]
fn raw_update_and_tilt_are_published_together() {
    let resources = SensorResources::<2, 1, 1>::new();
    let mut source = SequenceSource::new([Ok(SensorValue::AccelerationMg {
        x: 0,
        y: 0,
        z: 1_000,
    })]);
    let mut service: SensorService<'_, 8, 2, 8, 1, 8, 1> = SensorService::new(&resources);
    service
        .register_source(SensorRegistration::new(
            descriptor(1, SensorKind::Acceleration),
            &mut source,
        ))
        .unwrap();
    service
        .register_composite(TiltSensor::new(TiltConfig::new(SensorId(2), SensorId(1))))
        .unwrap();
    service.poll_one_due(Instant::from_ticks(1));

    let snapshot = resources.state();
    assert_eq!(snapshot.generation, 1);
    assert_eq!(
        snapshot.reading(SensorId(2)).unwrap().value,
        Some(SensorValue::TiltCentiDegrees { roll: 0, pitch: 0 })
    );
}

#[test]
fn dynamic_acceleration_rejects_a_slow_source() {
    let resources = SensorResources::<2, 1, 1>::new();
    let mut source = SequenceSource::new([Ok(SensorValue::AccelerationMg {
        x: 0,
        y: 0,
        z: 1_000,
    })]);
    let mut service: SensorService<'_, 8, 2, 8, 1, 8, 1> = SensorService::new(&resources);
    service
        .register_source(SensorRegistration::new(
            descriptor(1, SensorKind::Acceleration),
            &mut source,
        ))
        .unwrap();
    assert_eq!(
        service.register_composite(VedbaSensor::new(
            VedbaConfig::new(SensorId(2), SensorId(1),)
        )),
        Err(RegistrationError::Composite(
            CompositeValidationError::InputRateTooLow {
                id: SensorId(1),
                maximum_interval: Duration::from_millis(100),
            }
        ))
    );
}

#[test]
fn threshold_events_are_returned_for_ordered_delivery() {
    let resources = SensorResources::<1, 1, 2>::new();
    let mut source = SequenceSource::new([Ok(SensorValue::TemperatureMilliCelsius(41_000))]);
    let mut service: SensorService<'_, 8, 1, 8, 1, 8, 2> = SensorService::new(&resources);
    service
        .register_source(SensorRegistration::new(
            descriptor(1, SensorKind::Temperature),
            &mut source,
        ))
        .unwrap();
    service
        .set_threshold(ThresholdRule::Absolute {
            id: ThresholdId(1),
            sensor: SensorId(1),
            selector: ValueSelector::Scalar,
            comparison: AbsoluteComparison::Above,
            threshold: 40_000,
            hysteresis: 1_000,
            trigger_on_initial: true,
        })
        .unwrap();
    service
        .set_threshold(ThresholdRule::Absolute {
            id: ThresholdId(2),
            sensor: SensorId(1),
            selector: ValueSelector::Scalar,
            comparison: AbsoluteComparison::Above,
            threshold: 30_000,
            hysteresis: 1_000,
            trigger_on_initial: true,
        })
        .unwrap();

    let events = service.poll_one_due(Instant::from_ticks(1)).unwrap();
    assert_eq!(events.len(), 2);
    assert_eq!(events[0].kind, SensorEventKind::AbsoluteEntered);
    assert_eq!(events[0].threshold_id, ThresholdId(1));
    assert_eq!(events[0].sequence, 0);
    assert_eq!(events[1].threshold_id, ThresholdId(2));
    assert_eq!(events[1].sequence, 1);
    assert_eq!(resources.state().stats.threshold_events_emitted, 2);
}
