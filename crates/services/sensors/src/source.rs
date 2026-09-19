use embassy_time::{Duration, Instant};

use crate::{SensorDescriptor, SensorValue};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct SensorSourceError {
    pub code: u16,
}

impl SensorSourceError {
    pub const fn new(code: u16) -> Self {
        Self { code }
    }
}

pub trait SensorSource {
    fn sample(&mut self) -> Result<SensorValue, SensorSourceError>;
}

/// A statically borrowed physical sensor and its polling policy.
pub struct SensorRegistration<'a> {
    pub descriptor: SensorDescriptor,
    pub poll_interval: Duration,
    pub source: &'a mut dyn SensorSource,
}

impl<'a> SensorRegistration<'a> {
    pub fn new(descriptor: SensorDescriptor, source: &'a mut dyn SensorSource) -> Self {
        Self {
            descriptor,
            poll_interval: Duration::from_secs(10),
            source,
        }
    }

    pub fn with_interval(mut self, poll_interval: Duration) -> Self {
        self.poll_interval = poll_interval;
        self
    }
}

pub(crate) struct RegisteredSource<'a> {
    pub descriptor: SensorDescriptor,
    pub poll_interval: Duration,
    pub next_due: Instant,
    pub source: &'a mut dyn SensorSource,
}

impl<F> SensorSource for F
where
    F: FnMut() -> Result<SensorValue, SensorSourceError>,
{
    fn sample(&mut self) -> Result<SensorValue, SensorSourceError> {
        self()
    }
}
