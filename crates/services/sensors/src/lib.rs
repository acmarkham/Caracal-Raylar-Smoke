#![no_std]
//! Heapless polling, latest-state storage, composites, and threshold events.

#[cfg(test)]
extern crate std;

pub mod composites;
mod math;
mod service;
mod source;
mod thresholds;
mod types;

pub use service::{
    RegistrationError, SensorEventReceiver, SensorMutex, SensorResources, SensorService,
    SensorStateReceiver, DEFAULT_COMPOSITES, DEFAULT_EVENT_DEPTH, DEFAULT_FAILURES_BEFORE_FAULT,
    DEFAULT_SENSOR_CAPACITY, DEFAULT_SENSOR_POLL_INTERVAL, DEFAULT_SENSOR_SOURCES,
    DEFAULT_SENSOR_THRESHOLDS, DEFAULT_SENSOR_WATCHERS,
};
pub use source::{SensorRegistration, SensorSource, SensorSourceError};
pub use thresholds::{
    AbsoluteComparison, ThresholdError, ThresholdId, ThresholdRule, ValueSelector,
};
pub use types::{
    ReadingError, ReadingStatus, SensorDescriptor, SensorEvent, SensorEventKind, SensorId,
    SensorKind, SensorOrigin, SensorReading, SensorServiceStats, SensorSnapshot, SensorValue,
};

#[cfg(test)]
mod tests;
