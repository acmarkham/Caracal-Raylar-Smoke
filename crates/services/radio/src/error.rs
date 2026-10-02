#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FrameError {
    FrameTooLarge,
    BufferTooSmall,
    Truncated,
    Malformed,
    UnsupportedVersion(u8),
    UnknownFrameType(u8),
    InvalidDestination,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ScheduleError {
    UtcUnavailable,
    UtcUncertaintyTooHigh,
    InvalidSchedule,
    MissedSlot,
    Conflict,
    QueueFull,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum LinkError {
    NoAcceptableProfile,
    InvalidProfile,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RadioServiceError {
    Schedule(ScheduleError),
    Frame(FrameError),
    Link(LinkError),
    RadioDriver,
    MessageExpired,
    RetryLimitExceeded,
    NeighbourUnknown,
}

impl From<FrameError> for RadioServiceError {
    fn from(value: FrameError) -> Self {
        Self::Frame(value)
    }
}

impl From<ScheduleError> for RadioServiceError {
    fn from(value: ScheduleError) -> Self {
        Self::Schedule(value)
    }
}

impl From<LinkError> for RadioServiceError {
    fn from(value: LinkError) -> Self {
        Self::Link(value)
    }
}
