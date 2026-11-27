//! Bench-only oscillator clock. Build time is a label, not measured UTC.
use embassy_time::{Instant, Timer};
use raylar_time_service::{Anchor, AnchorQuality, TimeSource, UtcTimestamp};
#[embassy_executor::task]
pub async fn task() {
    let now = Instant::now();
    let build: i64 = env!("L4_BUILD_UNIX").parse().unwrap();
    // Laboratory labels occupy a boot-specific namespace. Reflashing the same
    // cached ELF must never collide with (or overwrite) an earlier recording.
    // These labels deliberately do not claim to be present-day UTC.
    let epoch = build
        + i64::from(crate::streams::BOOT_ID.load(core::sync::atomic::Ordering::Relaxed)) * 180;
    crate::TIME
        .anchor_sender()
        .send(Anchor {
            system_time: now,
            utc: UtcTimestamp::new(epoch, 0).unwrap(),
            quality: AnchorQuality::new(i64::MAX as u64),
            source: TimeSource::Laboratory,
            capture_ticks: None,
            pps_sequence: None,
            pps_interval: None,
        })
        .await;
    defmt::warn!(
        "L4 QUAD LOCAL-TIME-BENCH: synthetic boot epoch={}, GPS disabled, UTC unknown",
        epoch
    );
    loop {
        Timer::after_secs(60).await;
    }
}
