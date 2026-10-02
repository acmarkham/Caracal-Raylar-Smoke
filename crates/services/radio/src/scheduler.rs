use embassy_time::{Duration, Instant};
use heapless::Vec;

use crate::{JobId, ScheduleError};

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RadioPriority {
    BestEffort,
    ReliableData,
    Control,
    CriticalControl,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Reservation {
    pub job_id: JobId,
    pub start: Instant,
    pub end: Instant,
    pub priority: RadioPriority,
}

impl Reservation {
    pub fn overlaps(self, other: Self) -> bool {
        self.start < other.end && other.start < self.end
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ReservationOutcome<const CAPACITY: usize> {
    pub evicted: Vec<JobId, CAPACITY>,
}

pub struct Scheduler<const CAPACITY: usize> {
    reservations: Vec<Reservation, CAPACITY>,
    preparation_guard: Duration,
}

impl<const CAPACITY: usize> Scheduler<CAPACITY> {
    pub const fn new(preparation_guard: Duration) -> Self {
        Self {
            reservations: Vec::new(),
            preparation_guard,
        }
    }

    pub fn len(&self) -> usize {
        self.reservations.len()
    }

    pub fn is_empty(&self) -> bool {
        self.reservations.is_empty()
    }

    pub fn reserve(
        &mut self,
        reservation: Reservation,
        now: Instant,
    ) -> Result<ReservationOutcome<CAPACITY>, ScheduleError> {
        if reservation.end <= reservation.start {
            return Err(ScheduleError::InvalidSchedule);
        }
        if reservation.start < now + self.preparation_guard {
            return Err(ScheduleError::MissedSlot);
        }
        if self.reservations.iter().any(|existing| {
            existing.overlaps(reservation) && existing.priority >= reservation.priority
        }) {
            return Err(ScheduleError::Conflict);
        }

        let mut evicted = Vec::new();
        let mut index = self.reservations.len();
        while index > 0 {
            index -= 1;
            let existing = self.reservations[index];
            if existing.overlaps(reservation) {
                evicted
                    .push(existing.job_id)
                    .map_err(|_| ScheduleError::QueueFull)?;
                self.reservations.swap_remove(index);
            }
        }
        self.reservations
            .push(reservation)
            .map_err(|_| ScheduleError::QueueFull)?;
        self.reservations.sort_unstable_by(|left, right| {
            left.start
                .cmp(&right.start)
                .then_with(|| right.priority.cmp(&left.priority))
                .then_with(|| left.job_id.cmp(&right.job_id))
        });
        Ok(ReservationOutcome { evicted })
    }

    pub fn release(&mut self, job_id: JobId) -> bool {
        let Some(index) = self
            .reservations
            .iter()
            .position(|reservation| reservation.job_id == job_id)
        else {
            return false;
        };
        self.reservations.remove(index);
        true
    }

    pub fn preparation_time(&self, job_id: JobId) -> Option<Instant> {
        let start = self
            .reservations
            .iter()
            .find(|reservation| reservation.job_id == job_id)?
            .start;
        Some(Instant::from_ticks(
            start
                .as_ticks()
                .saturating_sub(self.preparation_guard.as_ticks()),
        ))
    }
}

pub fn guard_interval(
    local_uncertainty: Duration,
    expected_remote_uncertainty: Duration,
    scheduling_uncertainty: Duration,
    propagation_allowance: Duration,
    engineering_margin: Duration,
) -> Duration {
    Duration::from_micros(
        local_uncertainty
            .as_micros()
            .saturating_add(expected_remote_uncertainty.as_micros())
            .saturating_add(scheduling_uncertainty.as_micros())
            .saturating_add(propagation_allowance.as_micros())
            .saturating_add(engineering_margin.as_micros()),
    )
}
