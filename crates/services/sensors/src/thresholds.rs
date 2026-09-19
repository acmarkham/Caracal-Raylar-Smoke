use embassy_time::{Duration, Instant};
use heapless::Vec;

use crate::math::vector_magnitude;
use crate::{
    ReadingStatus, SensorEvent, SensorEventKind, SensorId, SensorKind, SensorReading, SensorValue,
};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct ThresholdId(pub u16);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ValueSelector {
    Scalar,
    X,
    Y,
    Z,
    Magnitude,
    Roll,
    Pitch,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum AbsoluteComparison {
    Above,
    Below,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ThresholdRule {
    Absolute {
        id: ThresholdId,
        sensor: SensorId,
        selector: ValueSelector,
        comparison: AbsoluteComparison,
        threshold: i32,
        hysteresis: u32,
        trigger_on_initial: bool,
    },
    Delta {
        id: ThresholdId,
        sensor: SensorId,
        selector: ValueSelector,
        minimum_absolute_change: u32,
        maximum_interval: Option<Duration>,
    },
}

impl ThresholdRule {
    pub const fn id(self) -> ThresholdId {
        match self {
            Self::Absolute { id, .. } | Self::Delta { id, .. } => id,
        }
    }

    pub const fn sensor(self) -> SensorId {
        match self {
            Self::Absolute { sensor, .. } | Self::Delta { sensor, .. } => sensor,
        }
    }

    pub const fn selector(self) -> ValueSelector {
        match self {
            Self::Absolute { selector, .. } | Self::Delta { selector, .. } => selector,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ThresholdError {
    Capacity,
    DuplicateId(ThresholdId),
    UnknownSensor(SensorId),
    IncompatibleSelector {
        sensor: SensorId,
        kind: SensorKind,
        selector: ValueSelector,
    },
    InvalidConfiguration,
}

struct ThresholdState {
    rule: ThresholdRule,
    latched: bool,
    has_sample: bool,
    previous: Option<(i32, Instant)>,
}

pub(crate) struct ThresholdEngine<const RULES: usize> {
    rules: Vec<ThresholdState, RULES>,
    next_event_sequence: u64,
}

impl<const RULES: usize> ThresholdEngine<RULES> {
    pub const fn new() -> Self {
        Self {
            rules: Vec::new(),
            next_event_sequence: 0,
        }
    }

    pub fn add(&mut self, rule: ThresholdRule, kind: SensorKind) -> Result<(), ThresholdError> {
        if self.rules.iter().any(|state| state.rule.id() == rule.id()) {
            return Err(ThresholdError::DuplicateId(rule.id()));
        }
        if !selector_compatible(kind, rule.selector()) {
            return Err(ThresholdError::IncompatibleSelector {
                sensor: rule.sensor(),
                kind,
                selector: rule.selector(),
            });
        }
        if matches!(
            rule,
            ThresholdRule::Delta {
                minimum_absolute_change: 0,
                ..
            }
        ) {
            return Err(ThresholdError::InvalidConfiguration);
        }
        self.rules
            .push(ThresholdState {
                rule,
                latched: false,
                has_sample: false,
                previous: None,
            })
            .map_err(|_| ThresholdError::Capacity)
    }

    pub fn evaluate(&mut self, reading: SensorReading, events: &mut Vec<SensorEvent, RULES>) {
        if reading.status != ReadingStatus::Current {
            return;
        }
        let Some(value) = reading.value else {
            return;
        };
        let Some(observed_at) = reading.last_success else {
            return;
        };

        for state in self
            .rules
            .iter_mut()
            .filter(|state| state.rule.sensor() == reading.descriptor.id)
        {
            let Some(current) = select_value(value, state.rule.selector()) else {
                continue;
            };
            let event = match state.rule {
                ThresholdRule::Absolute {
                    id,
                    sensor,
                    comparison,
                    threshold,
                    hysteresis,
                    trigger_on_initial,
                    ..
                } => evaluate_absolute(
                    state,
                    AbsoluteInput {
                        id,
                        sensor,
                        comparison,
                        threshold,
                        hysteresis,
                        trigger_on_initial,
                        current,
                        observed_at,
                    },
                ),
                ThresholdRule::Delta {
                    id,
                    sensor,
                    minimum_absolute_change,
                    maximum_interval,
                    ..
                } => evaluate_delta(
                    state,
                    DeltaInput {
                        id,
                        sensor,
                        minimum_absolute_change,
                        maximum_interval,
                        current,
                        observed_at,
                        angular: matches!(
                            reading.descriptor.kind,
                            SensorKind::Heading | SensorKind::TiltCompensatedHeading
                        ),
                    },
                ),
            };
            if let Some(mut event) = event {
                event.sequence = self.next_event_sequence;
                self.next_event_sequence = self.next_event_sequence.wrapping_add(1);
                let _ = events.push(event);
            }
        }
    }
}

struct AbsoluteInput {
    id: ThresholdId,
    sensor: SensorId,
    comparison: AbsoluteComparison,
    threshold: i32,
    hysteresis: u32,
    trigger_on_initial: bool,
    current: i32,
    observed_at: Instant,
}

fn evaluate_absolute(state: &mut ThresholdState, input: AbsoluteInput) -> Option<SensorEvent> {
    let violated = match input.comparison {
        AbsoluteComparison::Above => input.current >= input.threshold,
        AbsoluteComparison::Below => input.current <= input.threshold,
    };
    let hysteresis = i64::from(input.hysteresis);
    let cleared = match input.comparison {
        AbsoluteComparison::Above => {
            i64::from(input.current) <= i64::from(input.threshold) - hysteresis
        }
        AbsoluteComparison::Below => {
            i64::from(input.current) >= i64::from(input.threshold) + hysteresis
        }
    };

    if !state.has_sample {
        state.has_sample = true;
        state.latched = violated;
        if !violated || !input.trigger_on_initial {
            return None;
        }
    } else if !state.latched && violated {
        state.latched = true;
    } else if state.latched && cleared {
        state.latched = false;
        return Some(SensorEvent {
            sequence: 0,
            threshold_id: input.id,
            sensor_id: input.sensor,
            kind: SensorEventKind::AbsoluteCleared,
            current: input.current,
            previous: None,
            threshold: input.threshold,
            observed_at: input.observed_at,
        });
    } else {
        return None;
    }

    Some(SensorEvent {
        sequence: 0,
        threshold_id: input.id,
        sensor_id: input.sensor,
        kind: SensorEventKind::AbsoluteEntered,
        current: input.current,
        previous: None,
        threshold: input.threshold,
        observed_at: input.observed_at,
    })
}

struct DeltaInput {
    id: ThresholdId,
    sensor: SensorId,
    minimum_absolute_change: u32,
    maximum_interval: Option<Duration>,
    current: i32,
    observed_at: Instant,
    angular: bool,
}

fn evaluate_delta(state: &mut ThresholdState, input: DeltaInput) -> Option<SensorEvent> {
    let previous = state.previous.replace((input.current, input.observed_at));
    let (previous_value, previous_at) = previous?;
    if matches!(input.maximum_interval, Some(limit) if input.observed_at >= previous_at && input.observed_at.duration_since(previous_at) > limit)
    {
        return None;
    }
    let difference = if input.angular {
        let direct =
            ((i64::from(input.current) - i64::from(previous_value)).unsigned_abs() % 36_000) as u32;
        direct.min(36_000 - direct)
    } else {
        (i64::from(input.current) - i64::from(previous_value)).unsigned_abs() as u32
    };
    if difference < input.minimum_absolute_change {
        return None;
    }
    Some(SensorEvent {
        sequence: 0,
        threshold_id: input.id,
        sensor_id: input.sensor,
        kind: SensorEventKind::DeltaExceeded,
        current: input.current,
        previous: Some(previous_value),
        threshold: input.minimum_absolute_change.min(i32::MAX as u32) as i32,
        observed_at: input.observed_at,
    })
}

fn selector_compatible(kind: SensorKind, selector: ValueSelector) -> bool {
    match selector {
        ValueSelector::Scalar => matches!(
            kind,
            SensorKind::Temperature
                | SensorKind::Heading
                | SensorKind::TiltCompensatedHeading
                | SensorKind::Vedba
                | SensorKind::Odba
        ),
        ValueSelector::X | ValueSelector::Y | ValueSelector::Z | ValueSelector::Magnitude => {
            matches!(kind, SensorKind::Acceleration | SensorKind::MagneticField)
        }
        ValueSelector::Roll | ValueSelector::Pitch => kind == SensorKind::Tilt,
    }
}

fn select_value(value: SensorValue, selector: ValueSelector) -> Option<i32> {
    match (value, selector) {
        (SensorValue::TemperatureMilliCelsius(value), ValueSelector::Scalar) => Some(value),
        (SensorValue::HeadingCentiDegrees(value), ValueSelector::Scalar) => Some(i32::from(value)),
        (SensorValue::DynamicAccelerationMg(value), ValueSelector::Scalar) => {
            Some(value.min(i32::MAX as u32) as i32)
        }
        (SensorValue::AccelerationMg { x, .. }, ValueSelector::X)
        | (SensorValue::MagneticFieldNt { x, .. }, ValueSelector::X) => Some(x),
        (SensorValue::AccelerationMg { y, .. }, ValueSelector::Y)
        | (SensorValue::MagneticFieldNt { y, .. }, ValueSelector::Y) => Some(y),
        (SensorValue::AccelerationMg { z, .. }, ValueSelector::Z)
        | (SensorValue::MagneticFieldNt { z, .. }, ValueSelector::Z) => Some(z),
        (SensorValue::AccelerationMg { x, y, z }, ValueSelector::Magnitude)
        | (SensorValue::MagneticFieldNt { x, y, z }, ValueSelector::Magnitude) => {
            Some(vector_magnitude(x, y, z).min(i32::MAX as u32) as i32)
        }
        (SensorValue::TiltCentiDegrees { roll, .. }, ValueSelector::Roll) => Some(roll),
        (SensorValue::TiltCentiDegrees { pitch, .. }, ValueSelector::Pitch) => Some(pitch),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{SensorDescriptor, SensorOrigin};

    fn reading(kind: SensorKind, value: SensorValue, ticks: u64) -> SensorReading {
        SensorReading {
            descriptor: SensorDescriptor {
                id: SensorId(1),
                kind,
                origin: SensorOrigin::Other(1),
            },
            value: Some(value),
            status: ReadingStatus::Current,
            sequence: ticks,
            last_attempt: Instant::from_ticks(ticks),
            last_success: Some(Instant::from_ticks(ticks)),
            consecutive_errors: 0,
            total_errors: 0,
            last_error: None,
        }
    }

    #[test]
    fn absolute_threshold_hysteresis_rearms() {
        let mut engine = ThresholdEngine::<3>::new();
        engine
            .add(
                ThresholdRule::Absolute {
                    id: ThresholdId(1),
                    sensor: SensorId(1),
                    selector: ValueSelector::Scalar,
                    comparison: AbsoluteComparison::Above,
                    threshold: 40_000,
                    hysteresis: 1_000,
                    trigger_on_initial: true,
                },
                SensorKind::Temperature,
            )
            .unwrap();
        let mut events = Vec::new();
        engine.evaluate(
            reading(
                SensorKind::Temperature,
                SensorValue::TemperatureMilliCelsius(41_000),
                1,
            ),
            &mut events,
        );
        engine.evaluate(
            reading(
                SensorKind::Temperature,
                SensorValue::TemperatureMilliCelsius(40_500),
                2,
            ),
            &mut events,
        );
        engine.evaluate(
            reading(
                SensorKind::Temperature,
                SensorValue::TemperatureMilliCelsius(39_000),
                3,
            ),
            &mut events,
        );
        engine.evaluate(
            reading(
                SensorKind::Temperature,
                SensorValue::TemperatureMilliCelsius(41_000),
                4,
            ),
            &mut events,
        );
        assert_eq!(events.len(), 3);
        assert_eq!(events[0].kind, SensorEventKind::AbsoluteEntered);
        assert_eq!(events[1].kind, SensorEventKind::AbsoluteCleared);
        assert_eq!(events[2].kind, SensorEventKind::AbsoluteEntered);
    }

    #[test]
    fn angular_delta_uses_shortest_distance() {
        let mut engine = ThresholdEngine::<1>::new();
        engine
            .add(
                ThresholdRule::Delta {
                    id: ThresholdId(2),
                    sensor: SensorId(1),
                    selector: ValueSelector::Scalar,
                    minimum_absolute_change: 300,
                    maximum_interval: None,
                },
                SensorKind::Heading,
            )
            .unwrap();
        let mut events = Vec::new();
        engine.evaluate(
            reading(
                SensorKind::Heading,
                SensorValue::HeadingCentiDegrees(35_900),
                1,
            ),
            &mut events,
        );
        engine.evaluate(
            reading(
                SensorKind::Heading,
                SensorValue::HeadingCentiDegrees(100),
                2,
            ),
            &mut events,
        );
        assert!(events.is_empty());
    }

    #[test]
    fn delta_discards_a_stale_baseline() {
        let mut engine = ThresholdEngine::<1>::new();
        engine
            .add(
                ThresholdRule::Delta {
                    id: ThresholdId(3),
                    sensor: SensorId(1),
                    selector: ValueSelector::Scalar,
                    minimum_absolute_change: 500,
                    maximum_interval: Some(Duration::from_ticks(10)),
                },
                SensorKind::Temperature,
            )
            .unwrap();
        let mut events = Vec::new();
        engine.evaluate(
            reading(
                SensorKind::Temperature,
                SensorValue::TemperatureMilliCelsius(0),
                1,
            ),
            &mut events,
        );
        engine.evaluate(
            reading(
                SensorKind::Temperature,
                SensorValue::TemperatureMilliCelsius(1_000),
                100,
            ),
            &mut events,
        );
        assert!(events.is_empty());
        engine.evaluate(
            reading(
                SensorKind::Temperature,
                SensorValue::TemperatureMilliCelsius(1_600),
                105,
            ),
            &mut events,
        );
        assert_eq!(events.len(), 1);
        assert_eq!(events[0].previous, Some(1_000));
    }
}
