use embassy_time::Duration;
use raylar_time_service::UtcTimestamp;

use crate::ScheduleError;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Epoch(pub u64);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct EpochPosition {
    pub epoch: Epoch,
    pub offset: Duration,
    pub slot: u32,
    pub in_broadcast_window: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct EpochConfig {
    pub origin_utc_micros: i64,
    pub epoch_duration: Duration,
    pub broadcast_window: Duration,
    pub slot_duration: Duration,
}

impl EpochConfig {
    pub fn validate(&self) -> Result<(), ScheduleError> {
        let epoch = self.epoch_duration.as_micros();
        let window = self.broadcast_window.as_micros();
        let slot = self.slot_duration.as_micros();
        if epoch == 0 || window == 0 || slot == 0 || window > epoch || window < slot {
            return Err(ScheduleError::InvalidSchedule);
        }
        if !window.is_multiple_of(slot) {
            return Err(ScheduleError::InvalidSchedule);
        }
        Ok(())
    }

    pub fn broadcast_slot_count(&self) -> Result<u32, ScheduleError> {
        self.validate()?;
        u32::try_from(self.broadcast_window.as_micros() / self.slot_duration.as_micros())
            .map_err(|_| ScheduleError::InvalidSchedule)
    }

    pub fn position(&self, utc: UtcTimestamp) -> Result<EpochPosition, ScheduleError> {
        self.validate()?;
        let elapsed = utc
            .as_micros()
            .checked_sub(self.origin_utc_micros)
            .ok_or(ScheduleError::InvalidSchedule)?;
        if elapsed < 0 {
            return Err(ScheduleError::InvalidSchedule);
        }
        let elapsed = elapsed as u64;
        let epoch_us = self.epoch_duration.as_micros();
        let offset_us = elapsed % epoch_us;
        let slot = offset_us / self.slot_duration.as_micros();
        Ok(EpochPosition {
            epoch: Epoch(elapsed / epoch_us),
            offset: Duration::from_micros(offset_us),
            slot: u32::try_from(slot).map_err(|_| ScheduleError::InvalidSchedule)?,
            in_broadcast_window: offset_us < self.broadcast_window.as_micros(),
        })
    }

    pub fn epoch_start(&self, epoch: Epoch) -> Result<UtcTimestamp, ScheduleError> {
        self.validate()?;
        let offset = epoch
            .0
            .checked_mul(self.epoch_duration.as_micros())
            .ok_or(ScheduleError::InvalidSchedule)?;
        let offset = i64::try_from(offset).map_err(|_| ScheduleError::InvalidSchedule)?;
        let micros = self
            .origin_utc_micros
            .checked_add(offset)
            .ok_or(ScheduleError::InvalidSchedule)?;
        Ok(UtcTimestamp::from_micros(micros))
    }

    pub fn slot_time(&self, epoch: Epoch, slot: u32) -> Result<UtcTimestamp, ScheduleError> {
        let slot_count = self.broadcast_slot_count()?;
        if slot >= slot_count {
            return Err(ScheduleError::InvalidSchedule);
        }
        let start = self.epoch_start(epoch)?.as_micros();
        let slot_offset = u64::from(slot)
            .checked_mul(self.slot_duration.as_micros())
            .ok_or(ScheduleError::InvalidSchedule)?;
        let slot_offset = i64::try_from(slot_offset).map_err(|_| ScheduleError::InvalidSchedule)?;
        Ok(UtcTimestamp::from_micros(
            start
                .checked_add(slot_offset)
                .ok_or(ScheduleError::InvalidSchedule)?,
        ))
    }
}
