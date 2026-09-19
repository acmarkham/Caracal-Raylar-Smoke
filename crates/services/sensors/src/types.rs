use embassy_time::Instant;
use heapless::Vec;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct SensorId(pub u16);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SensorKind {
    Acceleration,
    MagneticField,
    Temperature,
    Tilt,
    Heading,
    TiltCompensatedHeading,
    Vedba,
    Odba,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SensorOrigin {
    Lis2hh12,
    Lis2mdl,
    Stm32Core,
    Radio,
    ExtensionBus(u8),
    Composite,
    Other(u16),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct SensorDescriptor {
    pub id: SensorId,
    pub kind: SensorKind,
    pub origin: SensorOrigin,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SensorValue {
    AccelerationMg { x: i32, y: i32, z: i32 },
    MagneticFieldNt { x: i32, y: i32, z: i32 },
    TemperatureMilliCelsius(i32),
    TiltCentiDegrees { roll: i32, pitch: i32 },
    HeadingCentiDegrees(u16),
    DynamicAccelerationMg(u32),
}

impl SensorValue {
    pub(crate) fn compatible_with(self, kind: SensorKind) -> bool {
        match self {
            Self::AccelerationMg { .. } => kind == SensorKind::Acceleration,
            Self::MagneticFieldNt { .. } => kind == SensorKind::MagneticField,
            Self::TemperatureMilliCelsius(_) => kind == SensorKind::Temperature,
            Self::TiltCentiDegrees { .. } => kind == SensorKind::Tilt,
            Self::HeadingCentiDegrees(value) => {
                value < 36_000
                    && matches!(
                        kind,
                        SensorKind::Heading | SensorKind::TiltCompensatedHeading
                    )
            }
            Self::DynamicAccelerationMg(_) => matches!(kind, SensorKind::Vedba | SensorKind::Odba),
        }
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ReadingStatus {
    #[default]
    Unavailable,
    Current,
    Stale,
    Fault,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ReadingError {
    Source(u16),
    IncompatibleValue,
    CompositeInputUnavailable,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SensorReading {
    pub descriptor: SensorDescriptor,
    pub value: Option<SensorValue>,
    pub status: ReadingStatus,
    pub sequence: u64,
    pub last_attempt: Instant,
    pub last_success: Option<Instant>,
    pub consecutive_errors: u32,
    pub total_errors: u64,
    pub last_error: Option<ReadingError>,
}

impl SensorReading {
    pub const fn unavailable(descriptor: SensorDescriptor) -> Self {
        Self {
            descriptor,
            value: None,
            status: ReadingStatus::Unavailable,
            sequence: 0,
            last_attempt: Instant::from_ticks(0),
            last_success: None,
            consecutive_errors: 0,
            total_errors: 0,
            last_error: None,
        }
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct SensorServiceStats {
    pub polls_attempted: u64,
    pub polls_succeeded: u64,
    pub polls_failed: u64,
    pub composites_evaluated: u64,
    pub threshold_events_emitted: u64,
    pub event_queue_high_water: u16,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SensorSnapshot<const SENSORS: usize> {
    pub readings: Vec<SensorReading, SENSORS>,
    pub generation: u64,
    pub published_at: Instant,
    pub stats: SensorServiceStats,
}

impl<const SENSORS: usize> SensorSnapshot<SENSORS> {
    pub const fn empty() -> Self {
        Self {
            readings: Vec::new(),
            generation: 0,
            published_at: Instant::from_ticks(0),
            stats: SensorServiceStats {
                polls_attempted: 0,
                polls_succeeded: 0,
                polls_failed: 0,
                composites_evaluated: 0,
                threshold_events_emitted: 0,
                event_queue_high_water: 0,
            },
        }
    }

    pub fn reading(&self, id: SensorId) -> Option<&SensorReading> {
        self.readings
            .iter()
            .find(|reading| reading.descriptor.id == id)
    }

    pub fn readings_of_kind(&self, kind: SensorKind) -> impl Iterator<Item = &SensorReading> {
        self.readings
            .iter()
            .filter(move |reading| reading.descriptor.kind == kind)
    }

    pub(crate) fn reading_mut(&mut self, id: SensorId) -> Option<&mut SensorReading> {
        self.readings
            .iter_mut()
            .find(|reading| reading.descriptor.id == id)
    }
}

impl<const SENSORS: usize> Default for SensorSnapshot<SENSORS> {
    fn default() -> Self {
        Self::empty()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SensorEventKind {
    AbsoluteEntered,
    AbsoluteCleared,
    DeltaExceeded,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SensorEvent {
    pub sequence: u64,
    pub threshold_id: crate::ThresholdId,
    pub sensor_id: SensorId,
    pub kind: SensorEventKind,
    pub current: i32,
    pub previous: Option<i32>,
    pub threshold: i32,
    pub observed_at: Instant,
}
