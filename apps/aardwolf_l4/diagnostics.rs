//! Executor load, including the priority microphone executor without overlap.
//! Bare DMA/PPS interrupt work before an executor wakes remains unattributed.
use core::sync::atomic::{AtomicBool, AtomicU32, Ordering};
use embassy_time::{Duration, Instant, Ticker};
static IDLE_OPEN: AtomicBool = AtomicBool::new(false);
static IDLE_START: AtomicU32 = AtomicU32::new(0);
static IDLE_TICKS: AtomicU32 = AtomicU32::new(0);
static POLL_START: AtomicU32 = AtomicU32::new(0);
static THREAD_EXECUTOR: AtomicU32 = AtomicU32::new(0);
static EXECUTOR_DEPTH: AtomicU32 = AtomicU32::new(0);
pub static MAX_POLL_US: AtomicU32 = AtomicU32::new(0);
pub static MAX_POLL_TASK: AtomicU32 = AtomicU32::new(0);

#[unsafe(export_name = "_embassy_trace_poll_start")]
fn poll_start(executor: u32) {
    // The first poll starts main, before main can create the audio executor.
    let _ = THREAD_EXECUTOR.compare_exchange(0, executor, Ordering::Relaxed, Ordering::Relaxed);
    cortex_m::interrupt::free(|_| {
        let depth = EXECUTOR_DEPTH.load(Ordering::Relaxed);
        EXECUTOR_DEPTH.store(depth + 1, Ordering::Relaxed);
        if depth == 0 && IDLE_OPEN.swap(false, Ordering::AcqRel) {
            IDLE_TICKS.fetch_add(
                (Instant::now().as_ticks() as u32).wrapping_sub(IDLE_START.load(Ordering::Relaxed)),
                Ordering::Relaxed,
            );
        }
    });
}
#[unsafe(export_name = "_embassy_trace_executor_idle")]
fn idle(_: u32) {
    cortex_m::interrupt::free(|_| {
        let depth = EXECUTOR_DEPTH.load(Ordering::Relaxed);
        EXECUTOR_DEPTH.store(depth - 1, Ordering::Relaxed);
        if depth != 1 {
            return;
        }
        IDLE_START.store(Instant::now().as_ticks() as u32, Ordering::Relaxed);
        IDLE_OPEN.store(true, Ordering::Release);
    });
}
#[unsafe(export_name = "_embassy_trace_task_new")]
fn task_new(_: u32, _: u32) {}
#[unsafe(export_name = "_embassy_trace_task_end")]
fn task_end(_: u32, _: u32) {}
#[unsafe(export_name = "_embassy_trace_task_exec_begin")]
fn exec_begin(executor: u32, _: u32) {
    if executor != THREAD_EXECUTOR.load(Ordering::Relaxed) {
        return;
    }
    if !crate::ARMED.load(Ordering::Relaxed) {
        return;
    }
    POLL_START.store(Instant::now().as_ticks() as u32, Ordering::Relaxed);
}
#[unsafe(export_name = "_embassy_trace_task_exec_end")]
fn exec_end(executor: u32, task: u32) {
    if executor != THREAD_EXECUTOR.load(Ordering::Relaxed) {
        return;
    }
    if !crate::ARMED.load(Ordering::Relaxed) {
        return;
    }
    let elapsed =
        (Instant::now().as_ticks() as u32).wrapping_sub(POLL_START.load(Ordering::Relaxed));
    if MAX_POLL_US.fetch_max(elapsed, Ordering::Relaxed) < elapsed {
        MAX_POLL_TASK.store(task, Ordering::Relaxed);
    }
}
pub fn reset_poll_peak() {
    MAX_POLL_US.store(0, Ordering::Relaxed);
    MAX_POLL_TASK.store(0, Ordering::Relaxed);
}
#[unsafe(export_name = "_embassy_trace_task_ready_begin")]
fn ready_begin(_: u32, _: u32) {}

#[embassy_executor::task]
pub async fn cpu_task() -> ! {
    IDLE_TICKS.store(0, Ordering::Relaxed);
    let mut previous = Instant::now().as_ticks() as u32;
    let mut ticker = Ticker::every(Duration::from_secs(60));
    loop {
        ticker.next().await;
        if crate::streams::STOP_LOG.load(Ordering::Relaxed) {
            continue;
        }
        let now = Instant::now().as_ticks() as u32;
        let elapsed = now.wrapping_sub(previous);
        let idle = IDLE_TICKS.swap(0, Ordering::AcqRel).min(elapsed);
        let active = elapsed - idle;
        let usage = u64::from(active) * 1000 / u64::from(elapsed.max(1));
        crate::streams::log(format_args!(
            "cpu usage={}.{}% active_ms={} idle_ms={} window_ms={} cadence_s=60 basis=executors",
            usage / 10,
            usage % 10,
            active / 1000,
            idle / 1000,
            elapsed / 1000
        ));
        let log = crate::LOGGING.stats();
        crate::streams::log(format_args!(
            "health log_dropped={} log_truncated={} log_errors={} queue_max={} append_us={} copy_us={} codec_max_us={}",
            log.dropped_messages, log.truncated_messages, log.write_failures, log.maximum_queue_depth,
            crate::streams::MAX_APPEND_US.load(Ordering::Relaxed), crate::audio::MAX_COPY_US.load(Ordering::Relaxed), crate::MAX_CODEC_US.load(Ordering::Relaxed)));
        #[cfg(not(feature = "mono"))]
        {
            crate::streams::log(format_args!(
                "audio_dma backlog_words={} irq_jitter_us={} heap_used={} heap_free={} channels=4 producer=TIM6_DAC",
                crate::audio::MAX_DMA_BACKLOG.load(Ordering::Relaxed),
                crate::audio::MAX_IRQ_JITTER_US.load(Ordering::Relaxed),crate::HEAP.used(),crate::HEAP.free()
            ));
            defmt::info!(
                "L4 QUAD DMA backlog_words={} irq_jitter_us={} heap_used={} heap_free={}",
                crate::audio::MAX_DMA_BACKLOG.load(Ordering::Relaxed),
                crate::audio::MAX_IRQ_JITTER_US.load(Ordering::Relaxed),
                crate::HEAP.used(),
                crate::HEAP.free()
            );
        }
        defmt::info!("AARDWOLF L4 CPU executor_permille={} active_ms={} idle_ms={} window_ms={} log_dropped={} log_truncated={} log_errors={} codec_max_us={} poll_max_us={}",
            usage, active / 1000, idle / 1000, elapsed / 1000, log.dropped_messages, log.truncated_messages, log.write_failures, crate::MAX_CODEC_US.load(Ordering::Relaxed), MAX_POLL_US.load(Ordering::Relaxed));
        previous = now;
    }
}
