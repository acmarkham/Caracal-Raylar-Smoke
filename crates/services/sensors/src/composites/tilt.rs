use embassy_time::{Duration, Instant};

use super::{
    acceleration, dependency, validate_dependency, AxisMap, CompositeEvaluation, CompositeSample,
    CompositeValidationError,
};
use crate::math::{atan2_cdeg, vector_magnitude};
use crate::{SensorDescriptor, SensorId, SensorKind, SensorOrigin, SensorSnapshot, SensorValue};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TiltConfig {
    pub output_id: SensorId,
    pub acceleration_id: SensorId,
    pub axis_map: AxisMap,
    pub minimum_gravity_mg: u32,
    pub maximum_gravity_mg: u32,
    pub maximum_age: Duration,
}

impl TiltConfig {
    pub const fn new(output_id: SensorId, acceleration_id: SensorId) -> Self {
        Self {
            output_id,
            acceleration_id,
            axis_map: AxisMap::IDENTITY,
            minimum_gravity_mg: 700,
            maximum_gravity_mg: 1_300,
            maximum_age: Duration::from_secs(30),
        }
    }
}

pub struct TiltSensor {
    config: TiltConfig,
    last_sequence: Option<u64>,
}

impl TiltSensor {
    /// Calculates board tilt after `axis_map` is applied.
    ///
    /// Roll is `atan2(+Y, +Z)` and pitch is
    /// `atan2(-X, sqrt(Y² + Z²))`; both outputs are centi-degrees.
    pub const fn new(config: TiltConfig) -> Self {
        Self {
            config,
            last_sequence: None,
        }
    }

    pub fn descriptor(&self) -> SensorDescriptor {
        SensorDescriptor {
            id: self.config.output_id,
            kind: SensorKind::Tilt,
            origin: SensorOrigin::Composite,
        }
    }

    pub(crate) fn validate<F>(&self, lookup: F) -> Result<(), CompositeValidationError>
    where
        F: Fn(SensorId) -> Option<(SensorKind, Option<Duration>)>,
    {
        validate_dependency(
            &lookup,
            self.config.acceleration_id,
            SensorKind::Acceleration,
        )?;
        if self.config.minimum_gravity_mg >= self.config.maximum_gravity_mg {
            return Err(CompositeValidationError::InvalidConfiguration);
        }
        Ok(())
    }

    pub(crate) fn evaluate<const SENSORS: usize>(
        &mut self,
        snapshot: &SensorSnapshot<SENSORS>,
        now: Instant,
    ) -> CompositeEvaluation {
        let Some((value, observed_at, sequence)) = dependency(
            snapshot,
            self.config.acceleration_id,
            now,
            self.config.maximum_age,
        ) else {
            return CompositeEvaluation::Unavailable;
        };
        if self.last_sequence == Some(sequence) {
            return CompositeEvaluation::NoChange;
        }
        self.last_sequence = Some(sequence);

        let Some(acceleration) = acceleration(value).map(|value| self.config.axis_map.apply(value))
        else {
            return CompositeEvaluation::Unavailable;
        };
        let magnitude = vector_magnitude(acceleration.x, acceleration.y, acceleration.z);
        if magnitude < self.config.minimum_gravity_mg || magnitude > self.config.maximum_gravity_mg
        {
            return CompositeEvaluation::Unavailable;
        }

        let (roll, pitch) = calculate_tilt(acceleration.x, acceleration.y, acceleration.z);
        CompositeEvaluation::Updated(CompositeSample {
            value: SensorValue::TiltCentiDegrees { roll, pitch },
            observed_at,
        })
    }
}

fn calculate_tilt(x: i32, y: i32, z: i32) -> (i32, i32) {
    let x = x as f32;
    let y = y as f32;
    let z = z as f32;
    let roll = atan2_cdeg(y, z);
    let pitch = atan2_cdeg(-x, libm::sqrtf(y * y + z * z));
    (roll, pitch)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn known_gravity_vectors_produce_expected_tilt() {
        assert_eq!(calculate_tilt(0, 0, 1_000), (0, 0));
        assert_eq!(calculate_tilt(0, 1_000, 0), (9_000, 0));
        assert_eq!(calculate_tilt(-1_000, 0, 0), (0, 9_000));
    }
}
