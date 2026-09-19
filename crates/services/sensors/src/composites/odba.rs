use embassy_time::{Duration, Instant};

use super::{
    acceleration, dependency, validate_dependency, AxisMap, CompositeEvaluation, CompositeSample,
    CompositeValidationError,
};
use crate::math::abs_i32;
use crate::{SensorDescriptor, SensorId, SensorKind, SensorOrigin, SensorSnapshot, SensorValue};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct OdbaConfig {
    pub output_id: SensorId,
    pub acceleration_id: SensorId,
    pub axis_map: AxisMap,
    pub static_filter_alpha_permille: u16,
    pub maximum_input_interval: Duration,
    pub maximum_age: Duration,
}

impl OdbaConfig {
    pub const fn new(output_id: SensorId, acceleration_id: SensorId) -> Self {
        Self {
            output_id,
            acceleration_id,
            axis_map: AxisMap::IDENTITY,
            static_filter_alpha_permille: 50,
            maximum_input_interval: Duration::from_millis(100),
            maximum_age: Duration::from_secs(1),
        }
    }
}

pub struct OdbaSensor {
    config: OdbaConfig,
    gravity: Option<[i32; 3]>,
    last_sequence: Option<u64>,
}

impl OdbaSensor {
    pub const fn new(config: OdbaConfig) -> Self {
        Self {
            config,
            gravity: None,
            last_sequence: None,
        }
    }

    pub fn descriptor(&self) -> SensorDescriptor {
        SensorDescriptor {
            id: self.config.output_id,
            kind: SensorKind::Odba,
            origin: SensorOrigin::Composite,
        }
    }

    pub(crate) fn validate<F>(&self, lookup: F) -> Result<(), CompositeValidationError>
    where
        F: Fn(SensorId) -> Option<(SensorKind, Option<Duration>)>,
    {
        let interval = validate_dependency(
            &lookup,
            self.config.acceleration_id,
            SensorKind::Acceleration,
        )?;
        if self.config.static_filter_alpha_permille == 0
            || self.config.static_filter_alpha_permille > 1_000
            || self.config.maximum_input_interval.as_ticks() == 0
        {
            return Err(CompositeValidationError::InvalidConfiguration);
        }
        if !matches!(interval, Some(value) if value <= self.config.maximum_input_interval) {
            return Err(CompositeValidationError::InputRateTooLow {
                id: self.config.acceleration_id,
                maximum_interval: self.config.maximum_input_interval,
            });
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
        let Some(value) = acceleration(value).map(|value| self.config.axis_map.apply(value)) else {
            return CompositeEvaluation::Unavailable;
        };
        let dynamic = self.dynamic_components([value.x, value.y, value.z]);
        let odba = abs_i32(dynamic[0])
            .saturating_add(abs_i32(dynamic[1]))
            .saturating_add(abs_i32(dynamic[2]));
        CompositeEvaluation::Updated(CompositeSample {
            value: SensorValue::DynamicAccelerationMg(odba),
            observed_at,
        })
    }

    fn dynamic_components(&mut self, measured: [i32; 3]) -> [i32; 3] {
        let Some(mut gravity) = self.gravity else {
            self.gravity = Some(measured);
            return [0; 3];
        };
        let alpha = i32::from(self.config.static_filter_alpha_permille);
        for index in 0..3 {
            gravity[index] += ((measured[index] - gravity[index]) * alpha) / 1_000;
        }
        self.gravity = Some(gravity);
        [
            measured[0] - gravity[0],
            measured[1] - gravity[1],
            measured[2] - gravity[2],
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn odba_sums_absolute_dynamic_axes() {
        let mut sensor = OdbaSensor::new(OdbaConfig {
            static_filter_alpha_permille: 100,
            ..OdbaConfig::new(SensorId(2), SensorId(1))
        });
        assert_eq!(sensor.dynamic_components([0, 0, 1_000]), [0, 0, 0]);
        let dynamic = sensor.dynamic_components([300, -400, 1_100]);
        assert_eq!(dynamic, [270, -360, 90]);
        assert_eq!(
            dynamic.iter().map(|value| abs_i32(*value)).sum::<u32>(),
            720
        );
    }
}
