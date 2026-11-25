//! Local composition of the verified 05 DFSDM adapter and Embassy DMA ring.
use crate::{dfsdm, AUDIO};
use caracal_evo::pcm::{pcm24, start_offset, Stats, HALF, RING, SAMPLE_RATE, WARMUP_BLOCKS};
use core::sync::atomic::{AtomicBool, AtomicU32, Ordering::Relaxed};
use embassy_stm32::{
    bind_interrupts,
    dma::{self, ReadableRingBuffer, TransferOptions},
    interrupt::typelevel,
    pac, peripherals, Peri,
};
use embassy_sync::{blocking_mutex::raw::CriticalSectionRawMutex, signal::Signal};
use embassy_time::{with_timeout, Duration, Instant, Timer};

pub static STOP: AtomicBool = AtomicBool::new(false);
pub static START: Signal<CriticalSectionRawMutex, Instant> = Signal::new();
pub static DONE: Signal<CriticalSectionRawMutex, ()> = Signal::new();
pub static PUBLISHED_BLOCKS: AtomicU32 = AtomicU32::new(0);
pub static DMA_ERRORS: AtomicU32 = AtomicU32::new(0);
pub static MAX_COPY_US: AtomicU32 = AtomicU32::new(0);
static IRQ_COUNT: AtomicU32 = AtomicU32::new(0);
static IRQ_TIME_LO: AtomicU32 = AtomicU32::new(0);
static mut DMA_WORDS: [u32; RING] = [0; RING];

bind_interrupts!(struct Irqs { DMA1_CHANNEL4 => DmaIrq; });
struct DmaIrq;
impl typelevel::Handler<typelevel::DMA1_CHANNEL4> for DmaIrq {
    unsafe fn on_interrupt() {
        let flags = pac::DMA1.isr().read();
        let half = flags.htif(3);
        let full = flags.tcif(3);
        if flags.teif(3) || (half && full) {
            DMA_ERRORS.fetch_add(1, Relaxed);
        }
        if half || full {
            IRQ_COUNT.fetch_add(1, Relaxed);
            IRQ_TIME_LO.store(Instant::now().as_micros() as u32, Relaxed);
        }
        <dma::InterruptHandler<peripherals::DMA1_CH4> as typelevel::Handler<
            typelevel::DMA1_CHANNEL4,
        >>::on_interrupt();
    }
}
// SAFETY: our vector always invokes the exact Embassy channel handler above.
unsafe impl
    typelevel::Binding<typelevel::DMA1_CHANNEL4, dma::InterruptHandler<peripherals::DMA1_CH4>>
    for Irqs
{
}

#[embassy_executor::task]
pub async fn capture(
    clock: Peri<'static, peripherals::PC2>,
    data: Peri<'static, peripherals::PB12>,
    channel: Peri<'static, peripherals::DMA1_CH4>,
) {
    let publish_at = START.wait().await.as_micros();
    let mut microphone = dfsdm::Dfsdm::new(clock, data);
    Timer::after_millis(100).await;
    let buffer = unsafe { &mut *core::ptr::addr_of_mut!(DMA_WORDS) };
    let address = buffer.as_ptr() as usize;
    defmt::assert!(address >= 0x20000000 && address + RING * 4 <= 0x20018000);
    let mut ring = unsafe {
        ReadableRingBuffer::new(
            dma::Channel::new(channel, Irqs),
            0,
            dfsdm::data_ptr(),
            buffer,
            TransferOptions::default(),
        )
    };
    ring.set_alignment(HALF);
    ring.start();
    microphone.start();
    let mut block = 0u32;
    let mut process = [0u32; HALF];
    let mut prior_stamp = None;
    while !STOP.load(Relaxed) {
        with_timeout(Duration::from_secs(1), ring.read_exact(&mut process))
            .await
            .expect("AARDWOLF L4 FAIL audio stalled")
            .expect("AARDWOLF L4 FAIL audio DMA overwrite");
        let begun = Instant::now();
        block = block.wrapping_add(1);
        defmt::assert!(
            IRQ_COUNT.load(Relaxed) == block && DMA_ERRORS.load(Relaxed) == 0,
            "AARDWOLF L4 FAIL audio IRQ deadline"
        );
        defmt::assert!(
            dfsdm::read(dfsdm::FLT_ISR) & ((1 << 3) | (1 << 2) | (1 << 17)) == 0,
            "AARDWOLF L4 FAIL DFSDM status"
        );
        let now = Instant::now().as_micros();
        // Extend the recent low-word ISR timestamp across the 71-minute wrap.
        let stamp = now - (now as u32).wrapping_sub(IRQ_TIME_LO.load(Relaxed)) as u64;
        if let Some(prior) = prior_stamp {
            defmt::assert!(
                (31_900..=32_100).contains(&(stamp - prior)),
                "AARDWOLF L4 FAIL audio cadence"
            );
        }
        prior_stamp = Some(stamp);
        if block > WARMUP_BLOCKS {
            // DMA completion stamps the last sample. AudioSource timestamps
            // identify the first sample, including a trimmed initial packet.
            let first = stamp - (HALF as u64 - 1) * 1_000_000 / u64::from(SAMPLE_RATE);
            let offset = start_offset(first, publish_at);
            if offset == HALF {
                continue;
            }
            let mut stats = Stats::default();
            for word in &process {
                stats.add(*word);
            }
            defmt::assert!(stats.bad_metadata == 0);
            // Conservatively include the recorder's current SD append in its
            // unconsumed backlog. This proves the source ring never overwrites,
            // including the final read whose planned bench exit returns no progress.
            // Count a partial first packet as a whole half: conservative by
            // at most 511 samples, never an underestimate of retained audio.
            let lag = PUBLISHED_BLOCKS
                .load(Relaxed)
                .wrapping_sub(crate::CONSUMED_HALVES.load(Relaxed))
                + 1;
            if crate::ARMED.load(Relaxed) {
                defmt::assert!(
                    lag as usize * HALF <= crate::AUDIO_CAPACITY,
                    "AARDWOLF L4 FAIL AudioSource retention exhausted"
                );
                crate::MAX_BACKLOG.fetch_max(lag * HALF as u32, Relaxed);
            }
            AUDIO
                .write_from_fn(
                    HALF - offset,
                    first + offset as u64 * 1_000_000 / u64::from(SAMPLE_RATE),
                    |i| pcm24(process[i + offset]),
                )
                .unwrap();
            PUBLISHED_BLOCKS.fetch_add(1, Relaxed);
        }
        MAX_COPY_US.fetch_max((Instant::now() - begun).as_micros() as u32, Relaxed);
    }
    microphone.stop();
    ring.request_reset();
    drop(ring);
    defmt::assert!(!pac::DMA1.ch(3).cr().read().en());
    DONE.signal(());
}
