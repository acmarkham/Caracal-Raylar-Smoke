//! Deferred radio diagnostics: ten records/s at most, no logs in driver calls.
use super::Log;
use embassy_time::{Instant, Timer};
use raylar_drivers::radio::diagnostics::{self, Kind, Record};
use raylar_logging_service::info;

#[embassy_executor::task]
pub(super) async fn task(log: Log) -> ! {
    let mut summary = Instant::now();
    let mut expected = Instant::now();
    let mut late_us = 0;
    let mut late_due = 0;
    let mut late_observed = 0;
    loop {
        let now = Instant::now();
        let late = now.saturating_duration_since(expected).as_micros();
        if late > late_us {
            late_us = late;
            late_due = expected.as_ticks();
            late_observed = now.as_ticks();
        }
        // Formatting must not compete with an observed file transition.
        #[cfg(feature = "sd-diagnostics")]
        let sd_active = raylar_drivers::storage::diagnostics::active();
        #[cfg(not(feature = "sd-diagnostics"))]
        let sd_active = false;
        if !sd_active {
            if let Some(record) = diagnostics::pop() {
                match record {
                    Record::Operation(r) => {
                        let _ = info!(log, "radio timing id={} op={} reason={} job={} start_ticks={} end_ticks={} us={} poll_us={} max_poll_us={} polls={} result={}",
                            r.id, r.op.name(), r.reason, r.job, r.start_ticks, r.end_ticks,
                            r.us, r.poll_us, r.max_poll_us, r.polls, r.result);
                        if let Some(error) = r.driver_error {
                            let _ = info!(log, "radio timing_error id={} error={:?}", r.id, error);
                        }
                        for (kind, io) in [Kind::Command, Kind::Spi, Kind::Busy].iter().zip(r.io) {
                            if io.calls == 0 {
                                continue;
                            }
                            let _ = info!(log, "radio timing_io id={} kind={} calls={} err={} cancelled={} us={} poll_us={} max_poll_us={} max_us={} tag={} error_tag={} start_ticks={} end_ticks={}",
                                r.id, kind.name(), io.calls, io.errors, io.cancelled, io.us,
                                io.poll_us, io.max_poll_us, io.max_us, io.tag, io.error_tag, io.start_ticks, io.end_ticks);
                        }
                    }
                    Record::State { ticks, mode, job } => {
                        let _ = info!(
                            log,
                            "radio timing_state ticks={} mode={} job={}", ticks, mode, job
                        );
                    }
                    Record::Request { ticks, enabled } => {
                        let _ = info!(
                            log,
                            "radio timing_request ticks={} enabled={}", ticks, enabled
                        );
                    }
                }
            }
            if summary.elapsed().as_secs() >= 10 {
                let t = diagnostics::totals();
                let c = t.operations;
                let _ = info!(log, "radio timing_totals init={} prepare={} tx={} rx={} recover={} sleep={} wake={} cancelled={} lost={} observer_late_us={} due_ticks={} observed_ticks={}",
                    c[0], c[1], c[2], c[3], c[4], c[5], c[6], t.cancelled, t.lost,
                    late_us, late_due, late_observed);
                summary = Instant::now();
                late_us = 0;
                late_due = 0;
                late_observed = 0;
            }
        }
        expected = Instant::now() + embassy_time::Duration::from_millis(100);
        Timer::at(expected).await;
    }
}
