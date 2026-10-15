//! Opt-in, heapless audio profiling. Default builds compile the probes away.
//! Synchronous spans and individual future polls measure cycles, never await time.
//! Stages are INCLUSIVE: parent and child totals must not be summed.
#![no_std]

use core::future::Future;

#[derive(Clone, Copy)]
#[repr(usize)]
pub enum Stage {
    RecorderPoll,
    CaptureCheck,
    CapturePublish,
    SourceCopy,
    SpatialTransform,
    SpatialDeltaSample,
    QuantizeFastSample,
    QuantizeWideSample,
    RiceStatsSample,
    RiceSelect,
    CrclSerialize,
    CrclPack,
    CrclCrc,
    ReferenceFeed,
    ReferenceShaCompress,
    FlacAnalysis,
    FlacEncode,
    StorageBuffer,
    FilesystemAppend,
    FilesystemFinalize,
    SdRead,
    SdWrite,
    Probe,
}

pub const STAGES: [Stage; 22] = [
    Stage::RecorderPoll,
    Stage::CaptureCheck,
    Stage::CapturePublish,
    Stage::SourceCopy,
    Stage::SpatialTransform,
    Stage::SpatialDeltaSample,
    Stage::QuantizeFastSample,
    Stage::QuantizeWideSample,
    Stage::RiceStatsSample,
    Stage::RiceSelect,
    Stage::CrclSerialize,
    Stage::CrclPack,
    Stage::CrclCrc,
    Stage::ReferenceFeed,
    Stage::ReferenceShaCompress,
    Stage::FlacAnalysis,
    Stage::FlacEncode,
    Stage::StorageBuffer,
    Stage::FilesystemAppend,
    Stage::FilesystemFinalize,
    Stage::SdRead,
    Stage::SdWrite,
];

impl Stage {
    pub const fn name(self) -> &'static str {
        match self {
            Self::RecorderPoll => "recorder_poll",
            Self::CaptureCheck => "capture_check",
            Self::CapturePublish => "capture_publish",
            Self::SourceCopy => "source_copy",
            Self::SpatialTransform => "spatial_transform",
            Self::SpatialDeltaSample => "spatial_delta_sample",
            Self::QuantizeFastSample => "quantize_fast_sample",
            Self::QuantizeWideSample => "quantize_wide_sample",
            Self::RiceStatsSample => "rice_stats_sample",
            Self::RiceSelect => "rice_select",
            Self::CrclSerialize => "crcl_serialize",
            Self::CrclPack => "crcl_pack",
            Self::CrclCrc => "crcl_crc",
            Self::ReferenceFeed => "reference_feed",
            Self::ReferenceShaCompress => "reference_sha_compress",
            Self::FlacAnalysis => "flac_analysis",
            Self::FlacEncode => "flac_encode",
            Self::StorageBuffer => "storage_buffer",
            Self::FilesystemAppend => "filesystem_append",
            Self::FilesystemFinalize => "filesystem_finalize",
            Self::SdRead => "sd_read",
            Self::SdWrite => "sd_write",
            Self::Probe => "probe",
        }
    }
    pub const fn sampled(self) -> bool {
        matches!(
            self,
            Self::SpatialDeltaSample
                | Self::QuantizeFastSample
                | Self::QuantizeWideSample
                | Self::RiceStatsSample
        )
    }
    #[cfg(feature = "dwt")]
    const fn root(self) -> bool {
        matches!(
            self,
            Self::RecorderPoll | Self::CaptureCheck | Self::CapturePublish | Self::Probe
        )
    }
}

#[derive(Clone, Copy, Default)]
pub struct Snapshot {
    pub calls: u32,
    pub cycles: u64,
    pub max_cycles: u32,
    pub wall_us: u64,
    pub bytes: u32,
}

#[cfg(feature = "dwt")]
mod active {
    use super::*;
    use core::sync::atomic::{AtomicBool, AtomicU32, Ordering::Relaxed};
    pub static RUNNING: AtomicBool = AtomicBool::new(false);
    pub static DETAILED: AtomicBool = AtomicBool::new(true);
    pub struct Counter {
        pub calls: AtomicU32,
        pub low: AtomicU32,
        pub high: AtomicU32,
        pub maximum: AtomicU32,
        pub wall_low: AtomicU32,
        pub wall_high: AtomicU32,
        pub bytes: AtomicU32,
    }
    impl Counter {
        const fn new() -> Self {
            Self {
                calls: AtomicU32::new(0),
                low: AtomicU32::new(0),
                high: AtomicU32::new(0),
                maximum: AtomicU32::new(0),
                wall_low: AtomicU32::new(0),
                wall_high: AtomicU32::new(0),
                bytes: AtomicU32::new(0),
            }
        }
        pub fn cycles(&self, cycles: u32) {
            let previous = self.low.fetch_add(cycles, Relaxed);
            if previous.checked_add(cycles).is_none() {
                self.high.fetch_add(1, Relaxed);
            }
            self.maximum.fetch_max(cycles, Relaxed);
            self.calls.fetch_add(1, Relaxed);
        }
    }
    pub static COUNTERS: [Counter; 23] = [const { Counter::new() }; 23];
    pub static REPRESENTATIONS: [AtomicU32; 4] = [const { AtomicU32::new(0) }; 4];
    #[inline(always)]
    pub fn clock() -> u32 {
        #[cfg(target_arch = "arm")]
        {
            cortex_m::peripheral::DWT::cycle_count()
        }
        #[cfg(not(target_arch = "arm"))]
        {
            0
        }
    }
    #[inline(always)]
    pub fn wanted(stage: Stage) -> bool {
        if !cfg!(feature = "detail") && !stage.root() {
            return false;
        }
        RUNNING.load(Relaxed) && (stage.root() || DETAILED.load(Relaxed))
    }
}

pub struct Span {
    #[cfg(feature = "dwt")]
    start: Option<(Stage, u32)>,
}
impl Span {
    #[inline(always)]
    pub fn new(stage: Stage) -> Self {
        #[cfg(feature = "dwt")]
        {
            Self {
                start: active::wanted(stage).then(|| (stage, active::clock())),
            }
        }
        #[cfg(not(feature = "dwt"))]
        {
            let _ = stage;
            Self {}
        }
    }
}
impl Drop for Span {
    #[inline(always)]
    fn drop(&mut self) {
        #[cfg(feature = "dwt")]
        if let Some((stage, start)) = self.start {
            active::COUNTERS[stage as usize].cycles(active::clock().wrapping_sub(start));
        }
    }
}

/// Rotate the sample phase between microphone channels. No per-sample atomics
/// or DWT reads on the other 31/32 samples. These totals are sampled estimates.
#[inline(always)]
pub fn sample(index: usize) -> bool {
    #[cfg(all(feature = "dwt", feature = "detail"))]
    {
        use core::sync::atomic::Ordering::Relaxed;
        index & 31 == 0 && active::RUNNING.load(Relaxed) && active::DETAILED.load(Relaxed)
    }
    #[cfg(not(all(feature = "dwt", feature = "detail")))]
    {
        let _ = index;
        false
    }
}

#[inline]
pub async fn measure<F: Future>(stage: Stage, future: F) -> F::Output {
    #[cfg(feature = "dwt")]
    {
        let enabled = active::wanted(stage);
        let start = enabled.then(embassy_time::Instant::now);
        let mut future = core::pin::pin!(future);
        let result = core::future::poll_fn(|cx| {
            let _span = Span::new(stage);
            future.as_mut().poll(cx)
        })
        .await;
        if let Some(start) = start {
            use core::sync::atomic::Ordering::Relaxed;
            let elapsed = start.elapsed().as_micros();
            let counter = &active::COUNTERS[stage as usize];
            let previous = counter.wall_low.fetch_add(elapsed as u32, Relaxed);
            counter.wall_high.fetch_add(
                (elapsed >> 32) as u32 + u32::from(previous.checked_add(elapsed as u32).is_none()),
                Relaxed,
            );
        }
        result
    }
    #[cfg(not(feature = "dwt"))]
    {
        let _ = stage;
        future.await
    }
}

#[inline]
pub fn bytes(stage: Stage, bytes: usize) {
    #[cfg(feature = "dwt")]
    if active::wanted(stage) {
        active::COUNTERS[stage as usize]
            .bytes
            .fetch_add(bytes as u32, core::sync::atomic::Ordering::Relaxed);
    }
    #[cfg(not(feature = "dwt"))]
    {
        let _ = (stage, bytes);
    }
}

#[inline]
pub fn representation(kind: u8) {
    #[cfg(feature = "dwt")]
    if active::wanted(Stage::CrclPack) {
        if let Some(c) = active::REPRESENTATIONS.get(kind as usize) {
            c.fetch_add(1, core::sync::atomic::Ordering::Relaxed);
        }
    }
    #[cfg(not(feature = "dwt"))]
    {
        let _ = kind;
    }
}
#[cfg(feature = "dwt")]
pub fn representations() -> [u32; 4] {
    core::array::from_fn(|i| active::REPRESENTATIONS[i].load(core::sync::atomic::Ordering::Relaxed))
}

#[cfg(feature = "dwt")]
pub fn set_running(running: bool) {
    active::RUNNING.store(running, core::sync::atomic::Ordering::Relaxed);
}
#[cfg(feature = "dwt")]
pub fn set_detailed(detailed: bool) {
    active::DETAILED.store(detailed, core::sync::atomic::Ordering::Relaxed);
}

/// Reset/snapshot only between recording windows, with probes quiescent.
#[cfg(feature = "dwt")]
pub fn reset() {
    use core::sync::atomic::Ordering::Relaxed;
    for c in &active::COUNTERS {
        for a in [
            &c.calls,
            &c.low,
            &c.high,
            &c.maximum,
            &c.wall_low,
            &c.wall_high,
            &c.bytes,
        ] {
            a.store(0, Relaxed);
        }
    }
    for c in &active::REPRESENTATIONS {
        c.store(0, Relaxed);
    }
}
#[cfg(feature = "dwt")]
pub fn snapshot(stage: Stage) -> Snapshot {
    use core::sync::atomic::Ordering::Relaxed;
    let c = &active::COUNTERS[stage as usize];
    Snapshot {
        calls: c.calls.load(Relaxed),
        cycles: (u64::from(c.high.load(Relaxed)) << 32) | u64::from(c.low.load(Relaxed)),
        max_cycles: c.maximum.load(Relaxed),
        wall_us: (u64::from(c.wall_high.load(Relaxed)) << 32) | u64::from(c.wall_low.load(Relaxed)),
        bytes: c.bytes.load(Relaxed),
    }
}

#[cfg(all(test, feature = "dwt"))]
mod tests {
    use super::active::Counter;
    use core::sync::atomic::Ordering::Relaxed;
    #[test]
    fn accumulated_cycles_survive_a_32_bit_wrap() {
        let c = Counter {
            calls: 0.into(),
            low: (u32::MAX - 5).into(),
            high: 0.into(),
            maximum: 0.into(),
            wall_low: 0.into(),
            wall_high: 0.into(),
            bytes: 0.into(),
        };
        c.cycles(10);
        assert_eq!(c.low.load(Relaxed), 4);
        assert_eq!(c.high.load(Relaxed), 1);
        assert_eq!(c.maximum.load(Relaxed), 10);
        assert_eq!(c.calls.load(Relaxed), 1);
    }

    #[test]
    fn pending_future_records_polls_without_holding_a_span_across_suspension() {
        use super::{measure, reset, set_running, snapshot, Stage};
        use core::{
            future::Future,
            task::{Context, Poll, Waker},
        };
        reset();
        set_running(true);
        let mut first = true;
        let inner = core::future::poll_fn(|_| {
            if first {
                first = false;
                Poll::Pending
            } else {
                Poll::Ready(42)
            }
        });
        let mut future = core::pin::pin!(measure(Stage::RecorderPoll, inner));
        let mut cx = Context::from_waker(Waker::noop());
        assert!(future.as_mut().poll(&mut cx).is_pending());
        assert_eq!(snapshot(Stage::RecorderPoll).calls, 1);
        assert_eq!(future.as_mut().poll(&mut cx), Poll::Ready(42));
        assert_eq!(snapshot(Stage::RecorderPoll).calls, 2);
        set_running(false);
    }
}
