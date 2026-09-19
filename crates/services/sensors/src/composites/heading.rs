use embassy_time::{Duration, Instant};

use super::{
    dependency, magnetic_field, validate_dependency, AxisMap, CompositeEvaluation, CompositeSample,
    CompositeValidationError, MagneticCalibration,
};
use crate::math::heading_cdeg;
use crate::{SensorDescriptor, SensorId, SensorKind, SensorOrigin, SensorSnapshot, SensorValue};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct HeadingConfig {
    pub output_id: SensorId,
    pub magnetic_field_id: SensorId,
    pub axis_map: AxisMap,
    pub calibration: MagneticCalibration,
    pub declination_cdeg: i32,
    pub maximum_age: Duration,
}

impl HeadingConfig {
    pub const fn new(output_id: SensorId, magnetic_field_id: SensorId) -> Self {
        Self {
            output_id,
            magnetic_field_id,
            axis_map: AxisMap::IDENTITY,
            calibration: MagneticCalibration::IDENTITY,
            declination_cdeg: 0,
            maximum_age: Duration::from_secs(30),
        }
    }
}

pub struct HeadingSensor {
    config: HeadingConfig,
    last_sequence: Option<u64>,
}

impl HeadingSensor {
    /// Calculates level-board heading after axis mapping and calibration.
    ///
    /// Board +X is zero/north, +Y is 90 degrees/east, and heading increases
    /// clockwise when viewed from above.
    pub const fn new(config: HeadingConfig) -> Self {
        Self {
            config,
            last_sequence: None,
        }
    }

    pub fn descriptor(&self) -> SensorDescriptor {
        SensorDescriptor {
            id: self.config.output_id,
            kind: SensorKind::Heading,
            origin: SensorOrigin::Composite,
        }
    }

    pub(crate) fn validate<F>(&self, lookup: F) -> Result<(), CompositeValidationError>
    where
        F: Fn(SensorId) -> Option<(SensorKind, Option<Duration>)>,
    {
        validate_dependency(
            &lookup,
            self.config.magnetic_field_id,
            SensorKind::MagneticField,
        )?;
        Ok(())
    }

    pub(crate) fn evaluate<const SENSORS: usize>(
        &mut self,
        snapshot: &SensorSnapshot<SENSORS>,
        now: Instant,
    ) -> CompositeEvaluation {
        let Some((value, observed_at, sequence)) = dependency(
            snapshot,
            self.config.magnetic_field_id,
            now,
            self.config.maximum_age,
        ) else {
            return CompositeEvaluation::Unavailable;
        };
        if self.last_sequence == Some(sequence) {
            return CompositeEvaluation::NoChange;
        }
        self.last_sequence = Some(sequence);

        let Some(field) = magnetic_field(value) else {
            return CompositeEvaluation::Unavailable;
        };
        let field = self
            .config
            .calibration
            .apply(self.config.axis_map.apply(field));
        CompositeEvaluation::Updated(CompositeSample {
            value: SensorValue::HeadingCentiDegrees(heading_cdeg(
                field.y as f32,
                field.x as f32,
                self.config.declination_cdeg,
            )),
            observed_at,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cardinal_headings_and_declination_wrap() {
        assert_eq!(heading_cdeg(0.0, 1.0, 0), 0);
        assert_eq!(heading_cdeg(1.0, 0.0, 0), 9_000);
        assert_eq!(heading_cdeg(0.0, -1.0, 0), 18_000);
        assert_eq!(heading_cdeg(-1.0, 0.0, 0), 27_000);
        assert_eq!(heading_cdeg(-1.0, 100.0, 200), 143);
    }
}
