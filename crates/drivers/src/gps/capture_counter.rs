/// Extend a wrapping hardware counter for a periodic PPS input.
///
/// Executor latency is not a reliable way to resolve counter wraps: a delayed
/// task wake can be hundreds of milliseconds after the capture even though the
/// captured PPS interval is still exactly one second. Use the known PPS cadence
/// to select the wrap count near the coarse elapsed-time estimate instead.
pub(crate) fn resolve_periodic_capture_delta(
    modulo_delta: u64,
    approximate_delta: u64,
    counter_modulus: u64,
    nominal_period: u64,
) -> u64 {
    if counter_modulus == 0 || nominal_period == 0 {
        return modulo_delta;
    }

    let approximate_periods = approximate_delta
        .saturating_add(nominal_period / 2)
        .checked_div(nominal_period)
        .unwrap_or(0)
        .max(1);
    // A sub-second executor stall can make the rounded coarse estimate one
    // period early or late. Two periods of headroom also covers unusually long
    // storage critical sections without weakening the cadence check.
    let first_periods = approximate_periods.saturating_sub(2).max(1);
    let last_periods = approximate_periods.saturating_add(2);
    let mut best_delta = modulo_delta;
    let mut best_period_error = u64::MAX;
    let mut best_observation_error = u64::MAX;

    for periods in first_periods..=last_periods {
        let target = periods.saturating_mul(nominal_period);
        let wraps = target
            .saturating_sub(modulo_delta)
            .saturating_add(counter_modulus / 2)
            / counter_modulus;
        let candidate = modulo_delta.saturating_add(wraps.saturating_mul(counter_modulus));
        let period_error = candidate.abs_diff(target);
        let observation_error = candidate.abs_diff(approximate_delta);
        if period_error < best_period_error
            || (period_error == best_period_error && observation_error < best_observation_error)
        {
            best_delta = candidate;
            best_period_error = period_error;
            best_observation_error = observation_error;
        }
    }

    best_delta
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ignores_delayed_executor_wake() {
        let modulus = 1 << 16;
        let nominal_period = 1_000_000;
        let modulo_delta = nominal_period % modulus;

        assert_eq!(
            resolve_periodic_capture_delta(modulo_delta, 1_343_470, modulus, nominal_period),
            nominal_period
        );
        assert_eq!(
            resolve_periodic_capture_delta(modulo_delta, 1_589_824, modulus, nominal_period),
            nominal_period
        );
    }

    #[test]
    fn preserves_counter_measured_drift() {
        let modulus = 1 << 16;
        let measured_period = 999_989;

        assert_eq!(
            resolve_periodic_capture_delta(
                measured_period % modulus,
                1_343_459,
                modulus,
                1_000_000,
            ),
            measured_period
        );
    }

    #[test]
    fn resolves_wraps_across_a_power_cycle_gap() {
        let modulus = 1 << 16;
        let measured_gap = 30_000_270;

        assert_eq!(
            resolve_periodic_capture_delta(measured_gap % modulus, 30_590_000, modulus, 1_000_000),
            measured_gap
        );
    }
}
