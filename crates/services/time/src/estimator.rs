use embassy_time::{Duration, Instant, TICK_HZ};

use crate::{Anchor, TimeConfig, TimeState};

pub struct TimeEstimator {
    config: TimeConfig,
    state: TimeState,
    frequency_reference: Option<Anchor>,
}

impl TimeEstimator {
    pub const fn new(config: TimeConfig) -> Self {
        Self {
            config,
            state: TimeState::invalid(),
            frequency_reference: None,
        }
    }

    pub const fn state(&self) -> TimeState {
        self.state
    }

    pub fn ingest(&mut self, anchor: Anchor) -> bool {
        if self.state.accepted_anchors == 0 {
            self.accept_first(anchor);
            return true;
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
        let residual_us = anchor.utc.as_micros() as i128 - predicted_us;
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
        let allowed = self
            .config
            .max_anchor_residual_us
            .saturating_add(self.state.uncertainty_us)
            .saturating_add(anchor.quality.uncertainty_us) as i128;
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

        if self
            .frequency_reference
            .is_some_and(|reference| reference.source != anchor.source)
        {
            #[cfg(feature = "defmt")]
            {
                let previous = self.frequency_reference.unwrap();
                defmt::info!(
                    "scale baseline reset: source {:?} -> {:?}, system_ticks={} utc_us={} quality_us={}",
                    previous.source,
                    anchor.source,
                    anchor.system_time.as_ticks(),
                    anchor.utc.as_micros(),
                    anchor.quality.uncertainty_us
                );
            }
            self.frequency_reference = Some(anchor);
        } else if let Some(reference) = self.frequency_reference {
            let system_ticks = anchor
                .system_time
                .as_ticks()
                .saturating_sub(reference.system_time.as_ticks());
            let utc_us = anchor.utc.as_micros() as i128 - reference.utc.as_micros() as i128;
            if system_ticks >= self.config.minimum_frequency_baseline.as_ticks() && utc_us > 0 {
                let nominal_us = system_ticks as i128 * 1_000_000 / TICK_HZ as i128;
                if nominal_us > 0 {
                    let frequency_residual_us = utc_us - nominal_us;
                    let observed_ppb = frequency_residual_us * 1_000_000_000 / nominal_us;
                    let previous_ppb = self.state.estimated_frequency_error_ppb as i128;
                    #[cfg(feature = "defmt")]
                    let mapping_scale_ppb = 1_000_000_000i128 + previous_ppb;
                    #[cfg(feature = "defmt")]
                    defmt::info!(
                        "scale sample: ref_ticks={} anchor_ticks={} system_ticks={} nominal_us={} utc_us={} frequency_residual_us={} raw_ppb={} current_ppb={} mapping_scale_ppb={} limit_ppb={}",
                        reference.system_time.as_ticks(),
                        anchor.system_time.as_ticks(),
                        system_ticks,
                        nominal_us,
                        utc_us,
                        frequency_residual_us,
                        observed_ppb,
                        previous_ppb,
                        mapping_scale_ppb,
                        self.config.max_frequency_error_ppb
                    );
                    if observed_ppb.abs() <= self.config.max_frequency_error_ppb as i128 {
                        let weight = self.config.frequency_ewma_weight_per_mille.min(1_000) as i128;
                        let ewma_numerator =
                            previous_ppb * (1_000 - weight) + observed_ppb * weight;
                        let updated_ppb = ewma_numerator / 1_000;
                        self.state.estimated_frequency_error_ppb = updated_ppb as i64;
                        #[cfg(feature = "defmt")]
                        defmt::info!(
                            "scale EWMA applied: previous_ppb={} observed_ppb={} weight_per_mille={} numerator={} updated_ppb={} mapping_scale_ppb={}",
                            previous_ppb,
                            observed_ppb,
                            weight,
                            ewma_numerator,
                            updated_ppb,
                            1_000_000_000i128 + updated_ppb
                        );
                    } else {
                        #[cfg(feature = "defmt")]
                        defmt::warn!(
                            "scale sample rejected: raw_ppb={} exceeds limit_ppb={}",
                            observed_ppb,
                            self.config.max_frequency_error_ppb
                        );
                    }
                }
                self.frequency_reference = Some(anchor);
            }
        }

        self.state.reference_system_time = anchor.system_time;
        self.state.reference_utc = anchor.utc;
        self.state.uncertainty_us = anchor.quality.uncertainty_us;
        self.state.last_anchor_system_time = Some(anchor.system_time);
        self.state.last_anchor_utc = Some(anchor.utc);
        self.state.holdover_duration = Duration::from_ticks(0);
        self.state.active_time_source = anchor.source;
        self.state.accepted_anchors = self.state.accepted_anchors.saturating_add(1);
        self.state.utc_valid = self.state.uncertainty_us <= self.config.max_uncertainty_us;
        #[cfg(feature = "defmt")]
        defmt::info!(
            "mapping epoch updated: system_ticks={} utc={}s+{}us source={:?} quality_us={} residual_us={} scale_ppb={} mapping_scale_ppb={} accepted={}",
            anchor.system_time.as_ticks(),
            anchor.utc.seconds,
            anchor.utc.microseconds,
            anchor.source,
            anchor.quality.uncertainty_us,
            residual_us,
            self.state.estimated_frequency_error_ppb,
            1_000_000_000i128 + self.state.estimated_frequency_error_ppb as i128,
            self.state.accepted_anchors
        );
        #[cfg(feature = "defmt")]
        defmt::info!(
            "mapping state: reference_system_ticks={} reference_utc={}s+{}us last_anchor_system_ticks={:?} last_anchor_utc={:?} holdover_duration_ms={} uncertainty_us={} utc_valid={} active_time_source={:?} accepted_anchors={} rejected_anchors={}",
            self.state.reference_system_time.as_ticks(),
            self.state.reference_utc.seconds,
            self.state.reference_utc.microseconds,
            self.state.last_anchor_system_time.map(|t| t.as_ticks()),
            self.state.last_anchor_utc,
            self.state.holdover_duration.as_millis(),
            self.state.uncertainty_us,
            self.state.utc_valid,
            self.state.active_time_source,
            self.state.accepted_anchors,
            self.state.rejected_anchors
        );
        true
    }

    pub fn update_holdover(&mut self, now: Instant) -> TimeState {
        let Some(last_anchor) = self.state.last_anchor_system_time else {
            self.state.utc_valid = false;
            return self.state;
        };
        let old_growth = uncertainty_growth(
            self.state.holdover_duration,
            self.config.holdover_stability_ppb,
        );
        let base_uncertainty = self.state.uncertainty_us.saturating_sub(old_growth);
        self.state.holdover_duration = now.saturating_duration_since(last_anchor);
        let new_growth = uncertainty_growth(
            self.state.holdover_duration,
            self.config.holdover_stability_ppb,
        );
        self.state.uncertainty_us = base_uncertainty.saturating_add(new_growth);
        self.state.utc_valid = self.state.uncertainty_us <= self.config.max_uncertainty_us;
        self.state
    }

    fn accept_first(&mut self, anchor: Anchor) {
        self.state.reference_system_time = anchor.system_time;
        self.state.reference_utc = anchor.utc;
        self.state.estimated_frequency_error_ppb = 0;
        self.state.uncertainty_us = anchor.quality.uncertainty_us;
        self.state.last_anchor_system_time = Some(anchor.system_time);
        self.state.last_anchor_utc = Some(anchor.utc);
        self.state.holdover_duration = Duration::from_ticks(0);
        self.state.active_time_source = anchor.source;
        self.state.accepted_anchors = 1;
        self.state.utc_valid = self.state.uncertainty_us <= self.config.max_uncertainty_us;
        self.frequency_reference = Some(anchor);
        #[cfg(feature = "defmt")]
        defmt::info!(
            "mapping epoch initialized: system_ticks={} utc={}s+{}us source={:?} quality_us={} scale_ppb=0 mapping_scale_ppb=1000000000",
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
    fn estimates_frequency_error_with_ewma() {
        let mut config = TimeConfig::default();
        config.frequency_ewma_weight_per_mille = 1_000;
        config.minimum_frequency_baseline = Duration::from_secs(1);
        let mut estimator = TimeEstimator::new(config);
        estimator.ingest(anchor(0, 1_700_000_000, 10));
        let mut second = anchor(100, 1_700_000_100, 10);
        second.utc.microseconds = 1_000;
        assert!(estimator.ingest(second));
        assert_eq!(estimator.state().estimated_frequency_error_ppb, 10_000);
    }

    #[test]
    fn frequency_baseline_spans_frequent_anchors() {
        let mut config = TimeConfig::default();
        config.frequency_ewma_weight_per_mille = 1_000;
        config.minimum_frequency_baseline = Duration::from_secs(10);
        let mut estimator = TimeEstimator::new(config);
        estimator.ingest(anchor(0, 1_700_000_000, 10));
        for second in 1..10 {
            assert!(estimator.ingest(anchor(second, 1_700_000_000 + second as i64, 10)));
        }
        let mut tenth = anchor(10, 1_700_000_010, 10);
        tenth.utc.microseconds = 100;
        assert!(estimator.ingest(tenth));
        assert_eq!(estimator.state().estimated_frequency_error_ppb, 10_000);
    }

    #[test]
    fn source_change_restarts_frequency_baseline() {
        let mut config = TimeConfig::default();
        config.frequency_ewma_weight_per_mille = 1_000;
        config.minimum_frequency_baseline = Duration::from_secs(10);
        let mut estimator = TimeEstimator::new(config);
        let mut coarse = anchor(0, 1_700_000_000, 1_000_000);
        coarse.source = TimeSource::GpsNmea;
        estimator.ingest(coarse);
        estimator.ingest(anchor(1, 1_700_000_001, 10));
        let mut fine = anchor(11, 1_700_000_011, 10);
        fine.utc.microseconds = 100;
        estimator.ingest(fine);
        assert_eq!(estimator.state().estimated_frequency_error_ppb, 10_000);
    }

    #[test]
    fn default_guardrail_accepts_software_pps_noise() {
        let mut config = TimeConfig::default();
        config.minimum_frequency_baseline = Duration::from_secs(10);
        let mut estimator = TimeEstimator::new(config);
        estimator.ingest(anchor(0, 1_700_000_000, 10));
        let noisy = Anchor {
            system_time: Instant::from_ticks(10 * TICK_HZ + 66),
            ..anchor(10, 1_700_000_010, 10)
        };
        assert!(estimator.ingest(noisy));
        assert!(estimator.state().estimated_frequency_error_ppb < 0);
    }

    #[test]
    fn rejects_large_discontinuity() {
        let mut estimator = TimeEstimator::new(TimeConfig::default());
        estimator.ingest(anchor(0, 1_700_000_000, 10));
        assert!(!estimator.ingest(anchor(1, 1_700_000_100, 10)));
        assert_eq!(estimator.state().rejected_anchors, 1);
    }

    #[test]
    fn uncertainty_invalidates_during_holdover() {
        let mut config = TimeConfig::default();
        config.max_uncertainty_us = 100;
        config.holdover_stability_ppb = 10_000;
        let mut estimator = TimeEstimator::new(config);
        estimator.ingest(anchor(0, 1_700_000_000, 10));
        let state = estimator.update_holdover(Instant::from_ticks(10 * TICK_HZ));
        assert_eq!(state.uncertainty_us, 110);
        assert!(!state.utc_valid);
    }
}
