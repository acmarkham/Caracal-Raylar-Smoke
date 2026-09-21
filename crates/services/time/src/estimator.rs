use embassy_time::{Duration, Instant, TICK_HZ};

use crate::{Anchor, TimeConfig, TimeSource, TimeState, UtcStatus, UtcTimestamp};

const FREQUENCY_SAMPLE_CAPACITY: usize = 11;
const FREQUENCY_SLOPE_CAPACITY: usize =
    FREQUENCY_SAMPLE_CAPACITY * (FREQUENCY_SAMPLE_CAPACITY - 1) / 2;

#[derive(Clone, Copy)]
struct FrequencySample {
    system_ticks: u64,
    utc_us: i64,
    source: TimeSource,
}

const EMPTY_FREQUENCY_SAMPLE: FrequencySample = FrequencySample {
    system_ticks: 0,
    utc_us: 0,
    source: TimeSource::None,
};

pub struct TimeEstimator {
    config: TimeConfig,
    state: TimeState,
    frequency_samples: [FrequencySample; FREQUENCY_SAMPLE_CAPACITY],
    frequency_sample_count: usize,
    last_observed_pps_system_time: Option<Instant>,
    last_observed_pps_sequence: Option<u64>,
}

impl TimeEstimator {
    pub const fn new(config: TimeConfig) -> Self {
        Self {
            config,
            state: TimeState::invalid(),
            frequency_samples: [EMPTY_FREQUENCY_SAMPLE; FREQUENCY_SAMPLE_CAPACITY],
            frequency_sample_count: 0,
            last_observed_pps_system_time: None,
            last_observed_pps_sequence: None,
        }
    }

    pub const fn state(&self) -> TimeState {
        self.state
    }

    pub fn ingest(&mut self, mut anchor: Anchor) -> bool {
        if self.state.accepted_anchors == 0 {
            if anchor.source == TimeSource::GpsPps {
                self.last_observed_pps_system_time = Some(anchor.system_time);
                self.last_observed_pps_sequence = anchor.pps_sequence;
            }
            self.accept_first(anchor);
            return true;
        }

        if anchor.source == TimeSource::GpsPps
            && !self.pps_reacquisition_ready(
                anchor.system_time,
                anchor.pps_sequence,
                anchor.pps_interval,
            )
        {
            self.reject();
            return false;
        }

        let Some(predicted_us) = mapping_utc_micros(&self.state, anchor.system_time) else {
            self.reject();
            return false;
        };
        #[cfg(feature = "defmt")]
        defmt::info!(
            "anchor system: {:?}, utc: {:?}, predicted utc: {}us",
            anchor.system_time,
            anchor.utc,
            predicted_us
        );
        let mut residual_us = anchor.utc.as_micros() as i128 - predicted_us;
        if anchor.source == crate::TimeSource::GpsPps {
            let one_second_us = 1_000_000i128;
            let tolerance_us = self.config.utc_second_correction_tolerance_us as i128;
            let correction_us = if (residual_us - one_second_us).abs() <= tolerance_us {
                one_second_us
            } else if (residual_us + one_second_us).abs() <= tolerance_us {
                -one_second_us
            } else {
                0
            };
            if correction_us != 0 {
                anchor.utc = UtcTimestamp::from_micros(
                    anchor.utc.as_micros().saturating_sub(correction_us as i64),
                );
                residual_us -= correction_us;
                self.state.utc_second_corrections =
                    self.state.utc_second_corrections.saturating_add(1);
                #[cfg(feature = "defmt")]
                defmt::warn!(
                    "PPS UTC second corrected: correction_us={} corrected_residual_us={} corrections={}",
                    correction_us,
                    residual_us,
                    self.state.utc_second_corrections
                );
            }
        }
        self.state.last_anchor_residual_us =
            Some(residual_us.clamp(i64::MIN as i128, i64::MAX as i128) as i64);
        #[cfg(feature = "defmt")]
        if anchor.source == crate::TimeSource::GpsPps {
            let predicted_seconds = predicted_us.div_euclid(1_000_000);
            let predicted_subsecond_us = predicted_us.rem_euclid(1_000_000);
            defmt::info!(
                "PPS UTC comparison: actual={}s+{}us predicted={}s+{}us error_actual_minus_predicted_us={} system_ticks={}",
                anchor.utc.seconds,
                anchor.utc.microseconds,
                predicted_seconds,
                predicted_subsecond_us,
                residual_us,
                anchor.system_time.as_ticks()
            );
        }
        #[cfg(feature = "defmt")]
        defmt::info!(
            "anchor residual: {}us, uncertainty: {}us",
            residual_us,
            self.state.uncertainty_us
        );
        let mut allowed = self
            .config
            .max_anchor_residual_us
            .saturating_add(anchor.quality.uncertainty_us) as i128;
        if anchor.source != TimeSource::GpsPps {
            allowed = allowed.saturating_add(self.state.uncertainty_us as i128);
        }
        if residual_us.abs() > allowed {
            #[cfg(feature = "defmt")]
            defmt::warn!(
                "anchor rejected: residual_us={} allowed_us={} source={:?}",
                residual_us,
                allowed,
                anchor.source
            );
            self.reject();
            return false;
        }

        self.add_frequency_sample(anchor);
        self.update_frequency_calibration();

        let phase_slew_ppb = phase_slew_ppb(residual_us, &self.config);
        self.state.phase_slew_ppb = phase_slew_ppb;
        self.state.estimated_frequency_error_ppb = self
            .state
            .calibrated_frequency_error_ppb
            .saturating_add(phase_slew_ppb);

        // Rebase at the old mapping's prediction before changing scale. This
        // keeps UTC continuous while the temporary rate correction slews the
        // measured phase residual toward zero.
        let Ok(predicted_i64) = i64::try_from(predicted_us) else {
            self.reject();
            return false;
        };
        self.state.reference_system_time = anchor.system_time;
        self.state.reference_utc = UtcTimestamp::from_micros(predicted_i64);
        self.state.uncertainty_us = anchor
            .quality
            .uncertainty_us
            .saturating_add(abs_i128_to_u64(residual_us));
        self.state.last_anchor_system_time = Some(anchor.system_time);
        self.state.last_anchor_utc = Some(anchor.utc);
        self.state.holdover_duration = Duration::from_ticks(0);
        self.state.active_time_source = anchor.source;
        self.state.accepted_anchors = self.state.accepted_anchors.saturating_add(1);
        self.state.holdover_warning = false;
        self.refresh_utc_status();
        #[cfg(feature = "defmt")]
        defmt::info!(
            "mapping epoch rebased: system_ticks={} utc={}s+{}us source={:?} quality_us={} residual_us={} calibrated_ppb={} phase_slew_ppb={} mapping_scale_ppb={} accepted={}",
            anchor.system_time.as_ticks(),
            anchor.utc.seconds,
            anchor.utc.microseconds,
            anchor.source,
            anchor.quality.uncertainty_us,
            residual_us,
            self.state.calibrated_frequency_error_ppb,
            self.state.phase_slew_ppb,
            1_000_000_000i128 + self.state.estimated_frequency_error_ppb as i128,
            self.state.accepted_anchors
        );
        #[cfg(feature = "defmt")]
        defmt::info!(
            "mapping state: reference_system_ticks={} reference_utc={}s+{}us last_anchor_system_ticks={:?} last_anchor_utc={:?} holdover_duration_ms={} uncertainty_us={} utc_status={:?} active_time_source={:?} calibrated_ppb={} calibration_samples={} phase_slew_ppb={} accepted_anchors={} rejected_anchors={} utc_second_corrections={}",
            self.state.reference_system_time.as_ticks(),
            self.state.reference_utc.seconds,
            self.state.reference_utc.microseconds,
            self.state.last_anchor_system_time.map(|t| t.as_ticks()),
            self.state.last_anchor_utc,
            self.state.holdover_duration.as_millis(),
            self.state.uncertainty_us,
            self.state.utc_status,
            self.state.active_time_source,
            self.state.calibrated_frequency_error_ppb,
            self.state.frequency_calibration_samples,
            self.state.phase_slew_ppb,
            self.state.accepted_anchors,
            self.state.rejected_anchors,
            self.state.utc_second_corrections
        );
        true
    }

    pub fn update_holdover(&mut self, now: Instant) -> TimeState {
        let Some(last_anchor) = self.state.last_anchor_system_time else {
            self.state.utc_status = UtcStatus::Invalid;
            return self.state;
        };
        let old_growth = uncertainty_growth(
            self.state.holdover_duration,
            self.config.holdover_stability_ppb,
        );
        let base_uncertainty = self.state.uncertainty_us.saturating_sub(old_growth);
        self.state.holdover_duration = now.saturating_duration_since(last_anchor);
        if self.state.holdover_duration >= self.config.pps_loss_timeout {
            self.disable_phase_slew(now);
        }
        let new_growth = uncertainty_growth(
            self.state.holdover_duration,
            self.config.holdover_stability_ppb,
        );
        self.state.uncertainty_us = base_uncertainty.saturating_add(new_growth);
        self.refresh_utc_status();
        if !self.state.holdover_warning
            && self.state.holdover_duration >= self.config.holdover_warning_threshold
        {
            self.state.holdover_warning = true;
            #[cfg(feature = "defmt")]
            defmt::warn!(
                "UTC holdover warning: duration_ms={} uncertainty_us={} status={:?}",
                self.state.holdover_duration.as_millis(),
                self.state.uncertainty_us,
                self.state.utc_status
            );
        }
        self.state
    }

    fn accept_first(&mut self, anchor: Anchor) {
        self.state.reference_system_time = anchor.system_time;
        self.state.reference_utc = anchor.utc;
        self.state.estimated_frequency_error_ppb = 0;
        self.state.calibrated_frequency_error_ppb = 0;
        self.state.phase_slew_ppb = 0;
        self.state.uncertainty_us = anchor.quality.uncertainty_us;
        self.state.last_anchor_system_time = Some(anchor.system_time);
        self.state.last_anchor_utc = Some(anchor.utc);
        self.state.holdover_duration = Duration::from_ticks(0);
        self.state.holdover_warning = false;
        self.state.active_time_source = anchor.source;
        self.state.first_anchor_source = anchor.source;
        self.state.last_anchor_residual_us = None;
        self.state.accepted_anchors = 1;
        self.refresh_utc_status();
        self.add_frequency_sample(anchor);
        #[cfg(feature = "defmt")]
        defmt::info!(
            "mapping epoch initialized with first fix: system_ticks={} utc={}s+{}us source={:?} quality_us={} scale_ppb=0 mapping_scale_ppb=1000000000",
            anchor.system_time.as_ticks(),
            anchor.utc.seconds,
            anchor.utc.microseconds,
            anchor.source,
            anchor.quality.uncertainty_us
        );
    }

    fn reject(&mut self) {
        self.state.rejected_anchors = self.state.rejected_anchors.saturating_add(1);
    }

    fn add_frequency_sample(&mut self, anchor: Anchor) {
        if self.state.frequency_calibration_locked {
            return;
        }
        if self.frequency_sample_count != 0 {
            let last = self.frequency_samples[self.frequency_sample_count - 1];
            if last.source != anchor.source {
                self.frequency_sample_count = 0;
            }
        }
        if self.frequency_sample_count != 0 {
            let last = self.frequency_samples[self.frequency_sample_count - 1];
            if anchor
                .system_time
                .as_ticks()
                .saturating_sub(last.system_ticks)
                < self.config.minimum_frequency_baseline.as_ticks()
            {
                return;
            }
        }

        if self.frequency_sample_count == FREQUENCY_SAMPLE_CAPACITY {
            self.frequency_samples.copy_within(1.., 0);
            self.frequency_sample_count -= 1;
        }
        self.frequency_samples[self.frequency_sample_count] = FrequencySample {
            system_ticks: anchor.system_time.as_ticks(),
            utc_us: anchor.utc.as_micros(),
            source: anchor.source,
        };
        self.frequency_sample_count += 1;
        self.state.frequency_calibration_samples = self.frequency_sample_count as u8;
    }

    fn update_frequency_calibration(&mut self) {
        if self.frequency_sample_count < 2 {
            return;
        }
        // Median pairwise slope (Theil-Sen) is robust to isolated mistagged
        // UTC samples while still using the complete ten-minute window.
        let mut slopes = [0i64; FREQUENCY_SLOPE_CAPACITY];
        let mut slope_count = 0usize;
        for first_index in 0..self.frequency_sample_count - 1 {
            let first = self.frequency_samples[first_index];
            for second in &self.frequency_samples[first_index + 1..self.frequency_sample_count] {
                let system_ticks = second.system_ticks.saturating_sub(first.system_ticks);
                let nominal_us = system_ticks as i128 * 1_000_000 / TICK_HZ as i128;
                let utc_us = second.utc_us as i128 - first.utc_us as i128;
                if nominal_us <= 0 || utc_us <= 0 {
                    continue;
                }
                let observed_ppb = (utc_us - nominal_us) * 1_000_000_000 / nominal_us;
                if observed_ppb.abs() <= self.config.max_frequency_error_ppb as i128 {
                    slopes[slope_count] = observed_ppb as i64;
                    slope_count += 1;
                }
            }
        }
        if slope_count == 0 {
            return;
        }
        slopes[..slope_count].sort_unstable();
        let calibrated_ppb = if slope_count % 2 == 0 {
            let upper = slopes[slope_count / 2] as i128;
            let lower = slopes[slope_count / 2 - 1] as i128;
            ((lower + upper) / 2) as i64
        } else {
            slopes[slope_count / 2]
        };
        self.state.calibrated_frequency_error_ppb = calibrated_ppb;
        if self.frequency_sample_count == FREQUENCY_SAMPLE_CAPACITY {
            self.state.frequency_calibration_locked = true;
            #[cfg(feature = "defmt")]
            defmt::info!("oscillator calibration locked at {}ppb", calibrated_ppb);
        }
        #[cfg(feature = "defmt")]
        defmt::info!(
            "robust frequency regression: samples={} valid_slopes={} span_s={} calibrated_ppb={}",
            self.frequency_sample_count,
            slope_count,
            (self.frequency_samples[self.frequency_sample_count - 1]
                .system_ticks
                .saturating_sub(self.frequency_samples[0].system_ticks))
                / TICK_HZ,
            calibrated_ppb
        );
    }

    /// Hold UTC continuous while removing the temporary phase correction.
    /// Only the learned oscillator frequency is allowed to run in holdover.
    fn disable_phase_slew(&mut self, at: Instant) {
        if self.state.phase_slew_ppb == 0 {
            return;
        }
        let Some(mapped_us) = mapping_utc_micros(&self.state, at) else {
            return;
        };
        let Ok(mapped_us) = i64::try_from(mapped_us) else {
            return;
        };
        self.state.reference_system_time = at;
        self.state.reference_utc = UtcTimestamp::from_micros(mapped_us);
        self.state.phase_slew_ppb = 0;
        self.state.estimated_frequency_error_ppb = self.state.calibrated_frequency_error_ppb;
        #[cfg(feature = "defmt")]
        defmt::info!(
            "PPS holdover: phase slew removed at system_ticks={}, calibrated_ppb={}",
            at.as_ticks(),
            self.state.calibrated_frequency_error_ppb
        );
    }

    /// Reject the gap edge and the first clean intervals after it. This keeps
    /// standby gaps and timer restart artefacts out of phase control and the
    /// oscillator regression.
    fn pps_reacquisition_ready(
        &mut self,
        system_time: Instant,
        pps_sequence: Option<u64>,
        raw_pps_interval: Option<Duration>,
    ) -> bool {
        let previous = self.last_observed_pps_system_time.replace(system_time);
        let previous_sequence = self.last_observed_pps_sequence;
        self.last_observed_pps_sequence = pps_sequence;
        let Some(previous) = previous else {
            return true;
        };
        // Prefer the raw edge-to-edge interval carried by the GPS driver.
        // Correlated anchors can be skipped when an NMEA label is late, so
        // their spacing is not evidence that the PPS stream was interrupted.
        let interval =
            raw_pps_interval.unwrap_or_else(|| system_time.saturating_duration_since(previous));
        let nominal_ticks = TICK_HZ;
        let interval_ticks = interval.as_ticks();
        let error_ticks = interval_ticks.abs_diff(nominal_ticks);
        let clean = error_ticks <= self.config.pps_interval_tolerance.as_ticks();
        let correlation_elapsed_ticks = system_time.saturating_duration_since(previous).as_ticks();
        let sequence_gap = match (previous_sequence, pps_sequence) {
            (Some(previous_sequence), Some(pps_sequence)) => {
                let edge_count = pps_sequence.saturating_sub(previous_sequence);
                correlation_elapsed_ticks
                    > edge_count
                        .saturating_mul(nominal_ticks)
                        .saturating_add(self.config.pps_interval_tolerance.as_ticks())
            }
            _ => false,
        };

        if interval >= self.config.pps_loss_timeout || sequence_gap {
            self.state.pps_reacquisition_active = true;
            self.state.pps_reacquisition_clean_intervals = 0;
            self.disable_phase_slew(system_time);
        } else if self.state.pps_reacquisition_active {
            self.state.pps_reacquisition_clean_intervals = if clean {
                self.state
                    .pps_reacquisition_clean_intervals
                    .saturating_add(1)
            } else {
                0
            };
        } else {
            return true;
        }

        if self.state.pps_reacquisition_clean_intervals < self.config.pps_reacquisition_intervals {
            self.state.pps_reacquisition_rejections =
                self.state.pps_reacquisition_rejections.saturating_add(1);
            #[cfg(feature = "defmt")]
            defmt::info!(
                "PPS reacquisition gate: interval_ticks={} clean={} progress={}/{} rejected={}",
                interval_ticks,
                clean,
                self.state.pps_reacquisition_clean_intervals,
                self.config.pps_reacquisition_intervals,
                self.state.pps_reacquisition_rejections
            );
            return false;
        }

        self.state.pps_reacquisition_active = false;
        #[cfg(feature = "defmt")]
        defmt::info!(
            "PPS reacquisition qualified after {} clean intervals",
            self.state.pps_reacquisition_clean_intervals
        );
        true
    }

    fn refresh_utc_status(&mut self) {
        let _previous_status = self.state.utc_status;
        self.state.utc_status = if self.state.accepted_anchors == 0 {
            UtcStatus::Invalid
        } else if self.state.uncertainty_us > self.config.degraded_uncertainty_us {
            UtcStatus::Degraded
        } else {
            UtcStatus::Synchronized
        };
        #[cfg(feature = "defmt")]
        if _previous_status != self.state.utc_status {
            defmt::warn!(
                "UTC status changed: {:?} -> {:?}, uncertainty_us={}",
                _previous_status,
                self.state.utc_status,
                self.state.uncertainty_us
            );
        }
    }
}

fn phase_slew_ppb(residual_us: i128, config: &TimeConfig) -> i64 {
    let duration_us = config.phase_slew_duration.as_micros() as i128;
    if duration_us == 0 {
        return 0;
    }
    let requested = residual_us.saturating_mul(1_000_000_000) / duration_us;
    requested.clamp(
        -(config.max_phase_slew_ppb as i128),
        config.max_phase_slew_ppb as i128,
    ) as i64
}

fn abs_i128_to_u64(value: i128) -> u64 {
    value.unsigned_abs().min(u64::MAX as u128) as u64
}

fn uncertainty_growth(duration: Duration, stability_ppb: u64) -> u64 {
    ((duration.as_micros() as u128).saturating_mul(stability_ppb as u128) / 1_000_000_000)
        .min(u64::MAX as u128) as u64
}

fn mapping_utc_micros(state: &TimeState, system_time: Instant) -> Option<i128> {
    let delta_ticks =
        system_time.as_ticks() as i128 - state.reference_system_time.as_ticks() as i128;
    let scale = 1_000_000_000i128 + state.estimated_frequency_error_ppb as i128;
    let delta_us = delta_ticks.checked_mul(1_000_000)?.checked_mul(scale)?
        / (TICK_HZ as i128 * 1_000_000_000i128);
    (state.reference_utc.as_micros() as i128).checked_add(delta_us)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{AnchorQuality, TimeError, TimeSource, UtcTimestamp};

    fn anchor(system_seconds: u64, utc_seconds: i64, uncertainty_us: u64) -> Anchor {
        Anchor {
            system_time: Instant::from_ticks(system_seconds * TICK_HZ),
            utc: UtcTimestamp::new(utc_seconds, 0).unwrap(),
            quality: AnchorQuality::new(uncertainty_us),
            source: TimeSource::GpsPps,
            capture_ticks: None,
            pps_sequence: None,
            pps_interval: None,
        }
    }

    #[test]
    fn invalid_before_first_anchor() {
        let estimator = TimeEstimator::new(TimeConfig::default());
        assert_eq!(
            estimator.state().system_to_utc(Instant::from_ticks(0)),
            Err(TimeError::NotValid)
        );
    }

    #[test]
    fn mapping_is_bidirectional() {
        let mut estimator = TimeEstimator::new(TimeConfig::default());
        assert!(estimator.ingest(anchor(10, 1_700_000_000, 10)));
        let system = Instant::from_ticks(15 * TICK_HZ);
        let utc = estimator.state().system_to_utc(system).unwrap();
        assert_eq!(utc, UtcTimestamp::new(1_700_000_005, 0).unwrap());
        assert_eq!(estimator.state().utc_to_system(utc).unwrap(), system);
    }

    #[test]
    fn estimates_frequency_error_with_long_baseline_regression() {
        let mut config = TimeConfig::default();
        config.minimum_frequency_baseline = Duration::from_secs(1);
        config.pps_loss_timeout = Duration::from_secs(1_000);
        let mut estimator = TimeEstimator::new(config);
        estimator.ingest(anchor(0, 1_700_000_000, 10));
        let mut second = anchor(100, 1_700_000_100, 10);
        second.utc.microseconds = 1_000;
        assert!(estimator.ingest(second));
        assert_eq!(estimator.state().calibrated_frequency_error_ppb, 10_000);
    }

    #[test]
    fn frequency_baseline_spans_frequent_anchors() {
        let mut config = TimeConfig::default();
        config.minimum_frequency_baseline = Duration::from_secs(10);
        let mut estimator = TimeEstimator::new(config);
        estimator.ingest(anchor(0, 1_700_000_000, 10));
        for second in 1..10 {
            assert!(estimator.ingest(anchor(second, 1_700_000_000 + second as i64, 10)));
        }
        let mut tenth = anchor(10, 1_700_000_010, 10);
        tenth.utc.microseconds = 100;
        assert!(estimator.ingest(tenth));
        assert_eq!(estimator.state().calibrated_frequency_error_ppb, 10_000);
    }

    #[test]
    fn frequency_regression_rejects_one_in_window_outlier() {
        let mut config = TimeConfig::default();
        config.pps_loss_timeout = Duration::from_secs(1_000);
        let mut estimator = TimeEstimator::new(config);
        estimator.ingest(anchor(0, 1_700_000_000, 10));
        for minute in 1..=10u64 {
            let mut sample = anchor(minute * 60, 1_700_000_000 + (minute * 60) as i64, 10);
            sample.utc.microseconds = minute as u32 * 600;
            if minute == 5 {
                sample.utc.microseconds += 50_000;
            }
            assert!(estimator.ingest(sample));
        }
        assert_eq!(estimator.state().calibrated_frequency_error_ppb, 10_000);
        assert!(estimator.state().frequency_calibration_locked);
    }

    #[test]
    fn corrects_adjacent_utc_second_without_poisoning_phase() {
        let mut estimator = TimeEstimator::new(TimeConfig::default());
        estimator.ingest(anchor(0, 1_700_000_000, 10));
        assert!(estimator.ingest(anchor(1, 1_700_000_000, 10)));
        let state = estimator.state();
        assert_eq!(state.utc_second_corrections, 1);
        assert_eq!(state.last_anchor_residual_us, Some(0));
        assert_eq!(state.rejected_anchors, 0);
    }

    #[test]
    fn hardware_scale_sample_handles_small_capture_quantization() {
        let mut config = TimeConfig::default();
        config.minimum_frequency_baseline = Duration::from_secs(10);
        config.pps_loss_timeout = Duration::from_secs(1_000);
        let mut estimator = TimeEstimator::new(config);
        estimator.ingest(anchor(0, 1_700_000_000, 10));
        let noisy = Anchor {
            system_time: Instant::from_ticks(10 * TICK_HZ + 66),
            ..anchor(10, 1_700_000_010, 10)
        };
        assert!(estimator.ingest(noisy));
        assert!(estimator.state().calibrated_frequency_error_ppb < 0);
    }

    #[test]
    fn phase_slew_rebases_without_a_clock_step_and_covers_residual() {
        let mut estimator = TimeEstimator::new(TimeConfig::default());
        estimator.ingest(anchor(0, 1_700_000_000, 10));
        let system = Instant::from_ticks(TICK_HZ);
        let before = estimator.state().system_to_utc(system).unwrap();
        let mut next = anchor(1, 1_700_000_001, 10);
        next.utc.microseconds = 1_000;
        assert!(estimator.ingest(next));
        let state = estimator.state();
        let after = state.system_to_utc(system).unwrap();
        assert_eq!(after, before);
        assert_eq!(state.last_anchor_residual_us, Some(1_000));
        assert_eq!(state.uncertainty_us, 1_010);
        assert!(state.phase_slew_ppb > 0);
    }

    #[test]
    fn rejects_large_discontinuity() {
        let mut estimator = TimeEstimator::new(TimeConfig::default());
        estimator.ingest(anchor(0, 1_700_000_000, 10));
        assert!(!estimator.ingest(anchor(1, 1_700_000_100, 10)));
        assert_eq!(estimator.state().rejected_anchors, 1);
    }

    #[test]
    fn uncertainty_degrades_during_holdover_but_mapping_remains_available() {
        let mut config = TimeConfig::default();
        config.degraded_uncertainty_us = 100;
        config.holdover_stability_ppb = 10_000;
        let mut estimator = TimeEstimator::new(config);
        estimator.ingest(anchor(0, 1_700_000_000, 10));
        let state = estimator.update_holdover(Instant::from_ticks(10 * TICK_HZ));
        assert_eq!(state.uncertainty_us, 110);
        assert_eq!(state.utc_status, UtcStatus::Degraded);
        assert!(state
            .system_to_utc(Instant::from_ticks(10 * TICK_HZ))
            .is_ok());
    }

    #[test]
    fn holdover_removes_phase_slew_without_stepping_utc() {
        let mut estimator = TimeEstimator::new(TimeConfig::default());
        estimator.ingest(anchor(0, 1_700_000_000, 10));
        let mut next = anchor(1, 1_700_000_001, 10);
        next.utc.microseconds = 1_000;
        assert!(estimator.ingest(next));
        assert_ne!(estimator.state().phase_slew_ppb, 0);

        let holdover_time = Instant::from_ticks(3 * TICK_HZ);
        let before = estimator.state().system_to_utc(holdover_time).unwrap();
        let state = estimator.update_holdover(holdover_time);
        let after = state.system_to_utc(holdover_time).unwrap();
        assert_eq!(after, before);
        assert_eq!(state.phase_slew_ppb, 0);
        assert_eq!(
            state.estimated_frequency_error_ppb,
            state.calibrated_frequency_error_ppb
        );
    }

    #[test]
    fn reacquisition_requires_three_clean_pps_intervals() {
        let mut estimator = TimeEstimator::new(TimeConfig::default());
        assert!(estimator.ingest(anchor(0, 1_700_000_000, 10)));

        assert!(!estimator.ingest(anchor(30, 1_700_000_030, 10)));
        assert!(!estimator.ingest(anchor(31, 1_700_000_031, 10)));
        assert!(!estimator.ingest(anchor(32, 1_700_000_032, 10)));
        assert!(estimator.ingest(anchor(33, 1_700_000_033, 10)));

        let state = estimator.state();
        assert!(!state.pps_reacquisition_active);
        assert_eq!(state.pps_reacquisition_clean_intervals, 3);
        assert_eq!(state.pps_reacquisition_rejections, 3);
        assert_eq!(state.accepted_anchors, 2);
        assert_eq!(state.rejected_anchors, 3);
    }

    #[test]
    fn reacquisition_uses_raw_pps_intervals_when_correlations_are_skipped() {
        let mut estimator = TimeEstimator::new(TimeConfig::default());
        assert!(estimator.ingest(anchor(0, 1_700_000_000, 10)));

        let mut gap = anchor(30, 1_700_000_030, 10);
        gap.pps_interval = Some(Duration::from_secs(30));
        assert!(!estimator.ingest(gap));

        // Correlated anchors are two seconds apart, but each reports that its
        // immediately preceding raw PPS interval was a clean one second.
        for second in [32u64, 34] {
            let mut clean = anchor(second, 1_700_000_000 + second as i64, 10);
            clean.pps_interval = Some(Duration::from_secs(1));
            assert!(!estimator.ingest(clean));
        }
        let mut qualified = anchor(36, 1_700_000_036, 10);
        qualified.pps_interval = Some(Duration::from_secs(1));
        assert!(estimator.ingest(qualified));

        let state = estimator.state();
        assert!(!state.pps_reacquisition_active);
        assert_eq!(state.pps_reacquisition_clean_intervals, 3);
        assert_eq!(state.accepted_anchors, 2);
    }

    #[test]
    fn pps_sequence_rearms_gate_across_a_power_cycle() {
        let mut estimator = TimeEstimator::new(TimeConfig::default());
        let mut first = anchor(0, 1_700_000_000, 10);
        first.pps_sequence = Some(1);
        assert!(estimator.ingest(first));

        // Four edges arrived across thirty elapsed seconds: the PPS stream was
        // stopped between sessions even though this latest raw interval is
        // already clean. Discard this first correlated post-wake anchor.
        let mut resumed = anchor(30, 1_700_000_030, 10);
        resumed.pps_sequence = Some(5);
        resumed.pps_interval = Some(Duration::from_secs(1));
        assert!(!estimator.ingest(resumed));
        assert!(estimator.state().pps_reacquisition_active);
        assert_eq!(estimator.state().pps_reacquisition_clean_intervals, 0);
    }

    #[test]
    fn holdover_warning_is_one_way_until_an_anchor_is_accepted() {
        let mut config = TimeConfig::default();
        config.holdover_warning_threshold = Duration::from_secs(90);
        config.pps_loss_timeout = Duration::from_secs(1_000);
        let mut estimator = TimeEstimator::new(config);
        assert!(estimator.ingest(anchor(0, 1_700_000_000, 10)));

        assert!(
            !estimator
                .update_holdover(Instant::from_ticks(89 * TICK_HZ))
                .holdover_warning
        );
        assert!(
            estimator
                .update_holdover(Instant::from_ticks(90 * TICK_HZ))
                .holdover_warning
        );
        assert!(estimator.ingest(anchor(90, 1_700_000_090, 10)));
        assert!(!estimator.state().holdover_warning);
    }

    #[test]
    fn locked_frequency_calibration_ignores_later_samples() {
        let mut config = TimeConfig::default();
        config.minimum_frequency_baseline = Duration::from_secs(1);
        config.pps_loss_timeout = Duration::from_secs(1_000);
        let mut estimator = TimeEstimator::new(config);
        assert!(estimator.ingest(anchor(0, 1_700_000_000, 10)));
        for second in 1..=10u64 {
            let mut sample = anchor(second, 1_700_000_000 + second as i64, 10);
            sample.utc.microseconds = second as u32 * 10;
            assert!(estimator.ingest(sample));
        }
        let locked_ppb = estimator.state().calibrated_frequency_error_ppb;
        assert!(estimator.state().frequency_calibration_locked);

        let mut later = anchor(11, 1_700_000_011, 10);
        later.utc.microseconds = 1_000;
        assert!(estimator.ingest(later));
        assert_eq!(estimator.state().calibrated_frequency_error_ppb, locked_ppb);
        assert_eq!(estimator.state().frequency_calibration_samples, 11);
    }
}
