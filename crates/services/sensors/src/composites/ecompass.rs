use embassy_time::{Duration, Instant};

use super::{
    acceleration, dependency, instant_skew, magnetic_field, validate_dependency, AxisMap,
    CompositeEvaluation, CompositeSample, CompositeValidationError, MagneticCalibration,
};
use crate::math::{heading_cdeg, vector_magnitude};
use crate::{SensorDescriptor, SensorId, SensorKind, SensorOrigin, SensorSnapshot, SensorValue};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ECompassConfig {
    pub output_id: SensorId,
    pub acceleration_id: SensorId,
    pub magnetic_field_id: SensorId,
    pub acceleration_axis_map: AxisMap,
    pub magnetic_axis_map: AxisMap,
    pub magnetic_calibration: MagneticCalibration,
    pub declination_cdeg: i32,
    pub minimum_gravity_mg: u32,
    pub maximum_gravity_mg: u32,
    pub maximum_age: Duration,
    pub maximum_skew: Duration,
}

impl ECompassConfig {
    pub const fn new(
        output_id: SensorId,
        acceleration_id: SensorId,
        magnetic_field_id: SensorId,
    ) -> Self {
        Self {
            output_id,
            acceleration_id,
            magnetic_field_id,
            acceleration_axis_map: AxisMap::IDENTITY,
            magnetic_axis_map: AxisMap::IDENTITY,
            magnetic_calibration: MagneticCalibration::IDENTITY,
            declination_cdeg: 0,
            minimum_gravity_mg: 700,
            maximum_gravity_mg: 1_300,
            maximum_age: Duration::from_secs(30),
            maximum_skew: Duration::from_secs(2),
        }
    }
}

pub struct ECompassSensor {
    config: ECompassConfig,
    last_sequences: Option<(u64, u64)>,
}

impl ECompassSensor {
    /// Calculates DT0058-style tilt-compensated heading.
    ///
    /// After their independent axis maps, board +X is north, +Y is east,
    /// +Z is up, and heading increases clockwise when viewed from above.
    pub const fn new(config: ECompassConfig) -> Self {
        Self {
            config,
            last_sequences: None,
        }
    }

    pub fn descriptor(&self) -> SensorDescriptor {
        SensorDescriptor {
            id: self.config.output_id,
            kind: SensorKind::TiltCompensatedHeading,
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
        validate_dependency(
            &lookup,
            self.config.magnetic_field_id,
            SensorKind::MagneticField,
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
        let Some((accel_value, accel_at, accel_sequence)) = dependency(
            snapshot,
            self.config.acceleration_id,
            now,
            self.config.maximum_age,
        ) else {
            return CompositeEvaluation::Unavailable;
        };
        let Some((mag_value, mag_at, mag_sequence)) = dependency(
            snapshot,
            self.config.magnetic_field_id,
            now,
            self.config.maximum_age,
        ) else {
            return CompositeEvaluation::Unavailable;
        };
        if instant_skew(accel_at, mag_at) > self.config.maximum_skew {
            return CompositeEvaluation::Unavailable;
        }
        if self.last_sequences == Some((accel_sequence, mag_sequence)) {
            return CompositeEvaluation::NoChange;
        }
        self.last_sequences = Some((accel_sequence, mag_sequence));

        let Some(acceleration) =
            acceleration(accel_value).map(|value| self.config.acceleration_axis_map.apply(value))
        else {
            return CompositeEvaluation::Unavailable;
        };
        let magnitude = vector_magnitude(acceleration.x, acceleration.y, acceleration.z);
        if magnitude < self.config.minimum_gravity_mg || magnitude > self.config.maximum_gravity_mg
        {
            return CompositeEvaluation::Unavailable;
        }
        let Some(field) = magnetic_field(mag_value) else {
            return CompositeEvaluation::Unavailable;
        };
        let field = self
            .config
            .magnetic_calibration
            .apply(self.config.magnetic_axis_map.apply(field));

        let heading = compensated_heading(acceleration, field, self.config.declination_cdeg);
        CompositeEvaluation::Updated(CompositeSample {
            value: SensorValue::HeadingCentiDegrees(heading),
            observed_at: if accel_at >= mag_at { accel_at } else { mag_at },
        })
    }
}

fn compensated_heading(
    acceleration: super::Vector3,
    field: super::Vector3,
    correction: i32,
) -> u16 {
    let ax = acceleration.x as f32;
    let ay = acceleration.y as f32;
    let az = acceleration.z as f32;
    let roll = libm::atan2f(ay, az);
    let pitch = libm::atan2f(-ax, libm::sqrtf(ay * ay + az * az));
    let sin_roll = libm::sinf(roll);
    let cos_roll = libm::cosf(roll);
    let sin_pitch = libm::sinf(pitch);
    let cos_pitch = libm::cosf(pitch);
    let mx = field.x as f32;
    let my = field.y as f32;
    let mz = field.z as f32;
    let horizontal_x = mx * cos_pitch + mz * sin_pitch;
    let horizontal_y = mx * sin_roll * sin_pitch + my * cos_roll - mz * sin_roll * cos_pitch;
    heading_cdeg(horizontal_y, horizontal_x, correction)
}

#[cfg(test)]
mod tests {
    use super::super::Vector3;
    use super::*;

    #[test]
    fn level_compass_preserves_cardinal_heading() {
        let gravity = Vector3 {
            x: 0,
            y: 0,
            z: 1_000,
        };
        assert_eq!(
            compensated_heading(
                gravity,
                Vector3 {
                    x: 1_000,
                    y: 0,
                    z: 0
                },
                0
            ),
            0
        );
        assert_eq!(
            compensated_heading(
                gravity,
                Vector3 {
                    x: 0,
                    y: 1_000,
                    z: 0
                },
                0
            ),
            9_000
        );
    }

    #[test]
    fn roll_is_removed_from_magnetic_vector() {
        let gravity = Vector3 {
            x: 0,
            y: 707,
            z: 707,
        };
        let rotated_north = Vector3 {
            x: 1_000,
            y: 0,
            z: 0,
        };
        assert_eq!(compensated_heading(gravity, rotated_north, 0), 0);
    }
}
