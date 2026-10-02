use embassy_time::Duration;

use crate::epoch::EpochConfig;

pub const DEFAULT_EPOCH_DURATION: Duration = Duration::from_secs(5 * 60);
pub const DEFAULT_BROADCAST_WINDOW: Duration = Duration::from_secs(60);
pub const DEFAULT_SLOT_DURATION: Duration = Duration::from_secs(5);
pub const MAX_HEARTBEAT_REPETITIONS: u8 = 8;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct RadioServiceConfig {
    pub network_id: u32,
    pub schedule_version: u8,
    pub epoch: EpochConfig,
    pub heartbeat_repetitions: u8,
    pub heartbeat_minimum_separation_slots: u16,
    pub expected_remote_uncertainty: Duration,
    pub scheduling_uncertainty: Duration,
    pub propagation_allowance: Duration,
    pub engineering_margin: Duration,
    pub maximum_scheduled_uncertainty: Duration,
    pub preparation_guard: Duration,
    pub neighbour_expiry: Duration,
}

impl Default for RadioServiceConfig {
    fn default() -> Self {
        Self {
            network_id: 0,
            schedule_version: 1,
            epoch: EpochConfig {
                origin_utc_micros: 0,
                epoch_duration: DEFAULT_EPOCH_DURATION,
                broadcast_window: DEFAULT_BROADCAST_WINDOW,
                slot_duration: DEFAULT_SLOT_DURATION,
            },
            heartbeat_repetitions: 2,
            heartbeat_minimum_separation_slots: 2,
            expected_remote_uncertainty: Duration::from_secs(1),
            scheduling_uncertainty: Duration::from_millis(10),
            propagation_allowance: Duration::from_millis(1),
            engineering_margin: Duration::from_millis(20),
            maximum_scheduled_uncertainty: Duration::from_secs(2),
            preparation_guard: Duration::from_secs(1),
            neighbour_expiry: Duration::from_secs(15 * 60),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ConfigError {
    InvalidEpoch,
    InvalidHeartbeatRepetitions,
    ImpossibleHeartbeatSeparation,
}

impl RadioServiceConfig {
    pub fn validate(&self) -> Result<(), ConfigError> {
        self.epoch
            .validate()
            .map_err(|_| ConfigError::InvalidEpoch)?;
        if self.heartbeat_repetitions == 0 || self.heartbeat_repetitions > MAX_HEARTBEAT_REPETITIONS
        {
            return Err(ConfigError::InvalidHeartbeatRepetitions);
        }
        let slots = self.epoch.broadcast_slot_count().unwrap_or(0);
        let required = u32::from(self.heartbeat_repetitions.saturating_sub(1))
            .saturating_mul(u32::from(self.heartbeat_minimum_separation_slots));
        if required >= slots {
            return Err(ConfigError::ImpossibleHeartbeatSeparation);
        }
        Ok(())
    }
}
