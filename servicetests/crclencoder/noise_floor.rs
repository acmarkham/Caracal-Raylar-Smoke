//! Diagnostic-only raw PCM capture. No codec, SD writes, GPS or radio tasks.
//! Read the reported buffer with `probe-rs read b32 ADDRESS 384000 -f binary -o FILE`.

use defmt::{info, unwrap};
use embassy_time::{with_timeout, Duration, Timer};
use static_cell::ConstStaticCell;

const CHANNELS: usize = 6;
const FRAMES: usize = 64_000;
static NOISE_PCM: ConstStaticCell<[i32; FRAMES * CHANNELS]> =
    ConstStaticCell::new([0; FRAMES * CHANNELS]);

pub async fn run() {
    let pcm = NOISE_PCM.take();
    let mut receiver = unwrap!(super::MICROPHONES.frame_receiver());
    info!("NOISE settling: 2 seconds; production Sinc5/R50, scale=0x27, reshape /4, HPF10Hz; GPS off; SD idle");
    let mut previous = 0;
    let mut written = 0;
    let mut first_sequence = 0;
    while written < FRAMES {
        let state = unwrap!(with_timeout(Duration::from_secs(3), receiver.changed()).await);
        assert!(state.error.is_none());
        if !state.running || state.sequence == 0 {
            continue;
        }
        assert!(previous == 0 || state.sequence == previous + 1);
        previous = state.sequence;
        // Each DMA half is 1600 frames / 100 ms. Discard startup transients.
        if state.sequence <= 20 {
            continue;
        }
        if written == 0 {
            first_sequence = state.sequence;
        }
        let frame = super::MICROPHONES.frame(state);
        let channels = frame.active_channels();
        assert_eq!(channels.len(), CHANNELS);
        let count = channels[0].len().min(FRAMES - written);
        for n in 0..count {
            for c in 0..CHANNELS {
                pcm[(written + n) * CHANNELS + c] = (channels[c][n] as i32) >> 8;
            }
        }
        assert_eq!(super::MICROPHONES.state().sequence, state.sequence);
        written += count;
    }
    super::MICROPHONES.set_enabled(false);
    Timer::after_millis(150).await;
    assert!(!super::MICROPHONES.state().running);
    // Independent target summaries let the host verify the RAM readback.
    for c in 0..CHANNELS {
        let mut sum = 0i64;
        let mut squares = 0u64;
        let mut min = i32::MAX;
        let mut max = i32::MIN;
        for n in 0..FRAMES {
            let value = pcm[n * CHANNELS + c];
            sum += i64::from(value);
            squares += (i64::from(value) * i64::from(value)) as u64;
            min = min.min(value);
            max = max.max(value);
        }
        info!(
            "NOISE channel={} frames={} sum={} squares={} min={} max={}",
            c + 1,
            FRAMES,
            sum,
            squares,
            min,
            max
        );
    }
    info!("NOISE CAPTURE READY address={=u32:#x} words={} frames={} channels={} rate=16000 first_seq={} last_seq={}",
        pcm.as_ptr() as u32, pcm.len(), FRAMES, CHANNELS, first_sequence, previous);
    loop {
        // Keep the buffer alive and untouched until the debugger reads it.
        core::hint::black_box(&*pcm);
        Timer::after_secs(60).await;
    }
}
