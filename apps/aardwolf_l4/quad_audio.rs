//! Four synchronous DFSDM filters, each with an independent circular DMA ring.
use crate::{dfsdm, AUDIO};
use caracal_evo::pcm::{pcm24, Stats, SAMPLE_RATE, WARMUP_BLOCKS};
use core::sync::atomic::{AtomicBool, AtomicU32, Ordering::Relaxed};
use embassy_stm32::{
    bind_interrupts,
    dma::{self, ReadableRingBuffer, TransferOptions},
    interrupt::typelevel,
    pac, peripherals, Peri,
};
use embassy_sync::{blocking_mutex::raw::CriticalSectionRawMutex, signal::Signal};
use embassy_time::{with_timeout, Duration, Instant, Timer};

// Compact builds use 8 ms banks / 80 ms source retention. The continuous
// laboratory build uses freed GPS RAM for 16 ms banks / 96 ms retention.
// The laboratory profile spends RAM otherwise reserved for GPS on service
// latency margin. Preserve the smaller GPS-enabled profile's memory budget.
const HALF: usize = if cfg!(all(feature = "local-time", not(feature = "bench-codec"))) {
    256
} else {
    128
};
const RING: usize = HALF * 2;
static EXECUTOR: embassy_executor::InterruptExecutor = embassy_executor::InterruptExecutor::new();
struct CaptureExecutorIrq;
impl typelevel::Handler<typelevel::TIM6_DAC> for CaptureExecutorIrq {
    unsafe fn on_interrupt() {
        EXECUTOR.on_interrupt();
    }
}
pub fn spawner() -> embassy_executor::SendSpawner {
    use embassy_stm32::interrupt::{InterruptExt, Priority};
    embassy_stm32::interrupt::TIM6_DAC.set_priority(Priority::P1);
    EXECUTOR.start(embassy_stm32::interrupt::TIM6_DAC)
}
pub static STOP: AtomicBool = AtomicBool::new(false);
pub static START: Signal<CriticalSectionRawMutex, Instant> = Signal::new();
pub static DONE: Signal<CriticalSectionRawMutex, ()> = Signal::new();
pub static PUBLISHED_BLOCKS: AtomicU32 = AtomicU32::new(0);
pub static PUBLISHED_SAMPLES: AtomicU32 = AtomicU32::new(0);
pub static DMA_ERRORS: AtomicU32 = AtomicU32::new(0);
pub static DMA_RECOVERIES: AtomicU32 = AtomicU32::new(0);
pub static DMA_SILENCE_FRAMES: AtomicU32 = AtomicU32::new(0);
static COALESCED_IRQS: AtomicU32 = AtomicU32::new(0);
pub static MAX_COPY_US: AtomicU32 = AtomicU32::new(0);
pub static MAX_DMA_BACKLOG: AtomicU32 = AtomicU32::new(0);
pub static MAX_IRQ_JITTER_US: AtomicU32 = AtomicU32::new(0);
static IRQ_COUNT: [AtomicU32; 4] = [const { AtomicU32::new(0) }; 4];
static IRQ_TIME_LO: [AtomicU32; 2] = [const { AtomicU32::new(0) }; 2];
static mut DMA_WORDS: [[u32; RING]; 4] = [[0; RING]; 4];

macro_rules! irq {
    ($name:ident,$vector:ident,$channel:ident,$lane:expr) => {
        struct $name;
        impl typelevel::Handler<typelevel::$vector> for $name {
            unsafe fn on_interrupt() {
                let flags = pac::DMA1.isr().read();
                let half = flags.htif(3 + $lane);
                let full = flags.tcif(3 + $lane);
                if flags.teif(3 + $lane) {
                    DMA_ERRORS.fetch_add(1, Relaxed);
                }
                if half && full {
                    COALESCED_IRQS.fetch_add(1, Relaxed);
                }
                if half || full {
                    let prior = IRQ_COUNT[$lane].fetch_add(1, Relaxed);
                    if $lane == 0 {
                        IRQ_TIME_LO[prior as usize % 2]
                            .store(Instant::now().as_micros() as u32, Relaxed);
                    }
                }
                <dma::InterruptHandler<peripherals::$channel> as typelevel::Handler<
                    typelevel::$vector,
                >>::on_interrupt();
            }
        }
        unsafe impl
            typelevel::Binding<typelevel::$vector, dma::InterruptHandler<peripherals::$channel>>
            for Irqs
        {
        }
    };
}
bind_interrupts!(struct Irqs {
    // TIM6 is unused hardware; its vector schedules only microphone draining.
    TIM6_DAC => CaptureExecutorIrq;
    DMA1_CHANNEL4 => Irq0;
    DMA1_CHANNEL5 => Irq1;
    DMA1_CHANNEL6 => Irq2;
    DMA1_CHANNEL7 => Irq3;
});
irq!(Irq0, DMA1_CHANNEL4, DMA1_CH4, 0);
irq!(Irq1, DMA1_CHANNEL5, DMA1_CH5, 1);
irq!(Irq2, DMA1_CHANNEL6, DMA1_CH6, 2);
irq!(Irq3, DMA1_CHANNEL7, DMA1_CH7, 3);

#[embassy_executor::task]
pub async fn capture(
    clock: Peri<'static, peripherals::PC2>,
    a: Peri<'static, peripherals::PB12>,
    b: Peri<'static, peripherals::PC7>,
    c: Peri<'static, peripherals::PB6>,
    d: Peri<'static, peripherals::PB10>,
    dma0: Peri<'static, peripherals::DMA1_CH4>,
    dma1: Peri<'static, peripherals::DMA1_CH5>,
    dma2: Peri<'static, peripherals::DMA1_CH6>,
    dma3: Peri<'static, peripherals::DMA1_CH7>,
) {
    let publish_at = START.wait().await.as_micros();
    let mut microphone = dfsdm::QuadDfsdm::new(clock, a, b, c, d);
    Timer::after_millis(100).await;
    let [buf0, buf1, buf2, buf3] = unsafe { &mut *core::ptr::addr_of_mut!(DMA_WORDS) };
    defmt::assert!(
        (buf0.as_ptr() as usize) >= 0x20000000 && (buf3.as_ptr() as usize) + RING * 4 <= 0x20018000
    );
    let mut r0 = unsafe {
        ReadableRingBuffer::new(
            dma::Channel::new(dma0, Irqs),
            0,
            dfsdm::filter_data_ptr(0),
            buf0,
            TransferOptions::default(),
        )
    };
    let mut r1 = unsafe {
        ReadableRingBuffer::new(
            dma::Channel::new(dma1, Irqs),
            0,
            dfsdm::filter_data_ptr(1),
            buf1,
            TransferOptions::default(),
        )
    };
    let mut r2 = unsafe {
        ReadableRingBuffer::new(
            dma::Channel::new(dma2, Irqs),
            0,
            dfsdm::filter_data_ptr(2),
            buf2,
            TransferOptions::default(),
        )
    };
    let mut r3 = unsafe {
        ReadableRingBuffer::new(
            dma::Channel::new(dma3, Irqs),
            0,
            dfsdm::filter_data_ptr(3),
            buf3,
            TransferOptions::default(),
        )
    };
    r0.set_alignment(HALF);
    r1.set_alignment(HALF);
    r2.set_alignment(HALF);
    r3.set_alignment(HALF);
    r0.start();
    r1.start();
    r2.start();
    r3.start();
    microphone.start();
    let mut process = [[0u32; HALF]; 4];
    let mut block = 0u32;
    let mut epoch = None;
    let mut timeline_first = None;
    let mut published_frames = 0u64;
    let mut coalesced = 0;
    #[cfg(feature = "bench-glitches")]
    let mut injected = false;
    let mut summary = [Stats::default(); 4];
    while !STOP.load(Relaxed) {
        #[cfg(feature = "bench-glitches")]
        if !injected && published_frames >= 3 * u64::from(SAMPLE_RATE) {
            injected = true;
            defmt::warn!("L4 GLITCH inject DMA consumer pause_ms=100");
            Timer::after_millis(100).await;
        }
        let [a, b, c, d] = &mut process;
        let reads = embassy_futures::join::join4(
            r0.read_exact(a),
            r1.read_exact(b),
            r2.read_exact(c),
            r3.read_exact(d),
        );
        let result = with_timeout(Duration::from_secs(1), reads).await;
        let ok = if let Ok((Ok(a), Ok(b), Ok(c), Ok(d))) = result {
            MAX_DMA_BACKLOG.fetch_max(a.max(b).max(c).max(d) as u32, Relaxed);
            true
        } else {
            defmt::warn!("L4 QUAD DMA read result={:?}", result);
            false
        };
        block = block.wrapping_add(1);
        if DMA_ERRORS.load(Relaxed) != 0 || result.is_err() {
            defmt::error!("L4 QUAD DMA deadline block={} irqs={}/{}/{}/{} errors={} poll_us={} codec_us={} append_us={}",block,IRQ_COUNT[0].load(Relaxed),IRQ_COUNT[1].load(Relaxed),IRQ_COUNT[2].load(Relaxed),IRQ_COUNT[3].load(Relaxed),DMA_ERRORS.load(Relaxed),crate::diagnostics::MAX_POLL_US.load(Relaxed),crate::MAX_CODEC_US.load(Relaxed),crate::streams::MAX_APPEND_US.load(Relaxed));
            DMA_ERRORS.fetch_add(1, Relaxed);
            crate::fault::latch(crate::fault::Fault::Audio);
            break;
        }
        if !ok
            || COALESCED_IRQS.load(Relaxed) != coalesced
            || IRQ_COUNT
                .iter()
                .any(|n| n.load(Relaxed).wrapping_sub(block) > 1)
        {
            let Some(first_irq) = epoch else {
                crate::fault::latch(crate::fault::Fault::Audio);
                break;
            };
            let deadline = Instant::now() + Duration::from_secs(1);
            loop {
                let joined = cortex_m::interrupt::free(|_| {
                    let remaining =
                        core::array::from_fn(|i| pac::DMA1.ch(3 + i).ndtr().read().ndt() as usize);
                    let completed = aardwolf_l4::audio_recovery::rejoin_bank(
                        first_irq,
                        Instant::now().as_micros(),
                        remaining,
                        HALF,
                    )?;
                    r0.clear();
                    r1.clear();
                    r2.clear();
                    r3.clear();
                    // Pending flags belong to discarded banks, not the new epoch.
                    pac::DMA1.ifcr().write(|w| {
                        for ch in 3..7 {
                            w.set_htif(ch, true);
                            w.set_tcif(ch, true);
                        }
                    });
                    for n in &IRQ_COUNT {
                        n.store(completed, Relaxed);
                    }
                    coalesced = COALESCED_IRQS.load(Relaxed);
                    Some(completed)
                });
                if let Some(completed) = joined {
                    // clear() skips the partial bank; the next read is the bank
                    // after it. All four lanes rejoin at the same DMA boundary.
                    block = completed + 1;
                    DMA_RECOVERIES.fetch_add(1, Relaxed);
                    defmt::warn!(
                        "L4 QUAD DMA recovered next_bank={} previous_published_frames={}",
                        block + 1,
                        published_frames
                    );
                    break;
                }
                if Instant::now() >= deadline {
                    crate::fault::latch(crate::fault::Fault::Audio);
                    break;
                }
                Timer::after_micros(250).await;
            }
            if crate::fault::active() {
                break;
            }
            continue;
        }
        let begun = Instant::now();
        summary.fill(Stats::default());
        for filter in 0..4 {
            defmt::assert!(
                dfsdm::read(dfsdm::FLT_ISR + filter * 0x80) & ((1 << 3) | (1 << 2) | (1 << 17))
                    == 0,
                "L4 QUAD DFSDM status"
            );
        }
        let now = Instant::now().as_micros();
        let irq_stamp = now
            - (now as u32).wrapping_sub(IRQ_TIME_LO[(block as usize - 1) % 2].load(Relaxed)) as u64;
        // TIM5 and DFSDM share the 80 MHz PLL: each bank spans an exact
        // integer number of local microseconds. Timestamp by sample count after establishing
        // the epoch, so a brief critical section cannot jitter the PCM clock.
        // Actual loss is checked by HAL overwrite checks, flags and bank counts.
        let first_stamp = *epoch.get_or_insert(irq_stamp);
        let stamp =
            first_stamp + (u64::from(block) - 1) * HALF as u64 * 1_000_000 / u64::from(SAMPLE_RATE);
        let jitter = irq_stamp.abs_diff(stamp);
        MAX_IRQ_JITTER_US.fetch_max(jitter as u32, Relaxed);
        defmt::assert!(
            jitter < HALF as u64 * 1_000_000 / u64::from(SAMPLE_RATE),
            "L4 QUAD sample clock epoch changed"
        );
        if block > WARMUP_BLOCKS * 512 / HALF as u32 {
            let first = stamp - (HALF as u64 - 1) * 1_000_000 / u64::from(SAMPLE_RATE);
            let offset = publish_at
                .saturating_sub(first)
                .saturating_mul(u64::from(SAMPLE_RATE))
                .div_ceil(1_000_000)
                .min(HALF as u64) as usize;
            if offset == HALF {
                continue;
            }
            if PUBLISHED_BLOCKS.load(Relaxed) == 0 {
                crate::diagnostics::reset_poll_peak();
                defmt::info!(
                    "L4 QUAD first PCM ticks={} channels=4 order=PB12,PC7,PB6,PB10",
                    first + offset as u64 * 1_000_000 / u64::from(SAMPLE_RATE)
                );
            }
            let samples = (HALF - offset) * 4;
            let packet_first = first + offset as u64 * 1_000_000 / u64::from(SAMPLE_RATE);
            let origin = *timeline_first.get_or_insert(packet_first);
            let packet_frame =
                ((packet_first - origin) * u64::from(SAMPLE_RATE) + 500_000) / 1_000_000;
            let gap_frames = packet_frame.saturating_sub(published_frames);
            if gap_frames != 0 {
                // Preserve the sample timeline. Large gaps cost only one ring's
                // bounded zero fill; the reader pads any overwritten prefix.
                AUDIO
                    .write_from_fn(
                        gap_frames as usize * 4,
                        origin + published_frames * 1_000_000 / u64::from(SAMPLE_RATE),
                        |_| 0,
                    )
                    .unwrap();
                DMA_SILENCE_FRAMES.fetch_add(gap_frames as u32, Relaxed);
                PUBLISHED_SAMPLES.fetch_add(gap_frames as u32 * 4, Relaxed);
                defmt::warn!(
                    "L4 QUAD DMA silence first_frame={} frames={}",
                    published_frames,
                    gap_frames
                );
                crate::streams::log(format_args!(
                    "audio_gap reason=dma first_source_frame={} frames={} fill=zero_all_channels",
                    published_frames, gap_frames
                ));
                published_frames += gap_frames;
            }
            // A recorder call can copy PCM, then spend tens of milliseconds
            // finishing/writing it before returning its progress. Use the
            // actual reader cursor, not the count of completed recorder calls.
            let lag = AUDIO.maximum_reader_backlog() as u32 + samples as u32;
            if crate::ARMED.load(Relaxed) {
                crate::MAX_BACKLOG.fetch_max(lag.min(crate::AUDIO_CAPACITY as u32), Relaxed);
            }
            for lane in 0..4 {
                for word in &process[lane][offset..] {
                    summary[lane].add(*word);
                }
            }
            #[cfg(feature = "bench-codec")]
            crate::quad_fixture::pcm(&process, offset);
            AUDIO
                .write_from_fn(
                    samples,
                    first + offset as u64 * 1_000_000 / u64::from(SAMPLE_RATE),
                    |i| pcm24(process[i % 4][offset + i / 4]),
                )
                .unwrap();
            PUBLISHED_BLOCKS.fetch_add(1, Relaxed);
            PUBLISHED_SAMPLES.fetch_add(samples as u32, Relaxed);
            published_frames += (samples / 4) as u64;
        }
        if block % (60 * SAMPLE_RATE / HALF as u32) == 0 {
            for (lane, stats) in summary.iter().enumerate() {
                defmt::info!(
                    "L4 QUAD PCM lane={} samples={} min={} max={} hash={:08x} bad_metadata={}",
                    lane,
                    stats.count,
                    stats.min,
                    stats.max,
                    caracal_evo::pcm::hash_words(&process[lane]),
                    stats.bad_metadata
                );
                defmt::assert!(stats.bad_metadata == 0 && stats.max - stats.min > 32);
            }
        }
        MAX_COPY_US.fetch_max((Instant::now() - begun).as_micros() as u32, Relaxed);
    }
    microphone.stop();
    drop((r0, r1, r2, r3));
    for channel in 3..7 {
        defmt::assert!(!pac::DMA1.ch(channel).cr().read().en());
    }
    DONE.signal(());
}
