mod axis;
mod ecompass;
mod heading;
mod odba;
mod tilt;
mod vedba;

use embassy_time::{Duration, Instant};

pub use axis::{AxisMap, MagneticCalibration, SignedAxis, Vector3};
pub use ecompass::{ECompassConfig, ECompassSensor};
pub use heading::{HeadingConfig, HeadingSensor};
pub use odba::{OdbaConfig, OdbaSensor};
pub use tilt::{TiltConfig, TiltSensor};
pub use vedba::{VedbaConfig, VedbaSensor};

use crate::{ReadingStatus, SensorDescriptor, SensorId, SensorKind, SensorSnapshot, SensorValue};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CompositeValidationError {
    MissingDependency(SensorId),
    WrongDependencyKind {
        id: SensorId,
        expected: SensorKind,
        observed: SensorKind,
    },
    InputRateTooLow {
        id: SensorId,
        maximum_interval: Duration,
    },
    InvalidConfiguration,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct CompositeSample {
    pub value: SensorValue,
    pub observed_at: Instant,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum CompositeEvaluation {
    NoChange,
    Unavailable,
    Updated(CompositeSample),
}

pub enum CompositeSensor {
    Tilt(TiltSensor),
    Heading(HeadingSensor),
    ECompass(ECompassSensor),
    Vedba(VedbaSensor),
    Odba(OdbaSensor),
}

impl CompositeSensor {
    pub fn descriptor(&self) -> SensorDescriptor {
        match self {
            Self::Tilt(sensor) => sensor.descriptor(),
            Self::Heading(sensor) => sensor.descriptor(),
            Self::ECompass(sensor) => sensor.descriptor(),
            Self::Vedba(sensor) => sensor.descriptor(),
            Self::Odba(sensor) => sensor.descriptor(),
        }
    }

    pub(crate) fn evaluate<const SENSORS: usize>(
        &mut self,
        snapshot: &SensorSnapshot<SENSORS>,
        now: Instant,
    ) -> CompositeEvaluation {
        match self {
            Self::Tilt(sensor) => sensor.evaluate(snapshot, now),
            Self::Heading(sensor) => sensor.evaluate(snapshot, now),
            Self::ECompass(sensor) => sensor.evaluate(snapshot, now),
            Self::Vedba(sensor) => sensor.evaluate(snapshot, now),
            Self::Odba(sensor) => sensor.evaluate(snapshot, now),
        }
    }

    pub(crate) fn validate<F>(&self, lookup: F) -> Result<(), CompositeValidationError>
    where
        F: Fn(SensorId) -> Option<(SensorKind, Option<Duration>)>,
    {
        match self {
            Self::Tilt(sensor) => sensor.validate(lookup),
            Self::Heading(sensor) => sensor.validate(lookup),
            Self::ECompass(sensor) => sensor.validate(lookup),
            Self::Vedba(sensor) => sensor.validate(lookup),
            Self::Odba(sensor) => sensor.validate(lookup),
        }
    }
}

impl From<TiltSensor> for CompositeSensor {
    fn from(value: TiltSensor) -> Self {
        Self::Tilt(value)
    }
}
impl From<HeadingSensor> for CompositeSensor {
    fn from(value: HeadingSensor) -> Self {
        Self::Heading(value)
    }
}
impl From<ECompassSensor> for CompositeSensor {
    fn from(value: ECompassSensor) -> Self {
        Self::ECompass(value)
    }
}
impl From<VedbaSensor> for CompositeSensor {
    fn from(value: VedbaSensor) -> Self {
        Self::Vedba(value)
    }
}
impl From<OdbaSensor> for CompositeSensor {
    fn from(value: OdbaSensor) -> Self {
        Self::Odba(value)
    }
}

pub(crate) fn dependency<const SENSORS: usize>(
    snapshot: &SensorSnapshot<SENSORS>,
    id: SensorId,
    now: Instant,
    maximum_age: Duration,
) -> Option<(SensorValue, Instant, u64)> {
    let reading = snapshot.reading(id)?;
    if reading.status != ReadingStatus::Current {
        return None;
    }
    let observed_at = reading.last_success?;
    if now >= observed_at && now.duration_since(observed_at) > maximum_age {
        return None;
    }
    Some((reading.value?, observed_at, reading.sequence))
}

pub(crate) fn validate_dependency<F>(
    lookup: &F,
    id: SensorId,
    expected: SensorKind,
) -> Result<Option<Duration>, CompositeValidationError>
where
    F: Fn(SensorId) -> Option<(SensorKind, Option<Duration>)>,
{
    let (observed, interval) = lookup(id).ok_or(CompositeValidationError::MissingDependency(id))?;
    if observed != expected {
        return Err(CompositeValidationError::WrongDependencyKind {
            id,
            expected,
            observed,
        });
    }
    Ok(interval)
}

pub(crate) fn acceleration(value: SensorValue) -> Option<Vector3> {
    match value {
        SensorValue::AccelerationMg { x, y, z } => Some(Vector3 { x, y, z }),
        _ => None,
    }
}

pub(crate) fn magnetic_field(value: SensorValue) -> Option<Vector3> {
    match value {
        SensorValue::MagneticFieldNt { x, y, z } => Some(Vector3 { x, y, z }),
        _ => None,
    }
}

pub(crate) fn instant_skew(a: Instant, b: Instant) -> Duration {
    if a >= b {
        a.duration_since(b)
    } else {
        b.duration_since(a)
    }
}
