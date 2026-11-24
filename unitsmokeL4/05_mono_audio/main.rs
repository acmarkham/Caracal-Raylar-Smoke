//! ADR-L4-010: finite DFSDM/DMA qualification and stable RAM audio export.
#![no_std]
#![no_main]
mod dfsdm;

use core::sync::atomic::{AtomicU32, Ordering};
use defmt::{assert, info};
use embassy_executor::Spawner;
use embassy_stm32::{
    bind_interrupts,
    dma::{self, ReadableRingBuffer, TransferOptions},
    gpio::{Level, Output, Speed},
    pac,
    rcc::{
        self, AHBPrescaler, APBPrescaler, Hse, HseMode, Pll, PllMul, PllPreDiv, PllRDiv, PllSource,
        Sysclk,
    },
    time::Hertz,
};
use embassy_time::{with_timeout, Duration, Instant, Timer};
use raylar_audiosource::{AudioFormat, AudioSource, ReaderStart};
use unit_smoke_l4_05_mono_audio::*;
use {defmt_rtt as _, panic_probe as _};

static HT: AtomicU32 = AtomicU32::new(0);
static TC: AtomicU32 = AtomicU32::new(0);
static DMA_ERRORS: AtomicU32 = AtomicU32::new(0);
static FIRST_IRQ: AtomicU32 = AtomicU32::new(0);
static LAST_IRQ: AtomicU32 = AtomicU32::new(0);

bind_interrupts!(struct Irqs { DMA1_CHANNEL4 => DmaIrq; });
struct DmaIrq;
impl
    embassy_stm32::interrupt::typelevel::Handler<embassy_stm32::interrupt::typelevel::DMA1_CHANNEL4>
    for DmaIrq
{
    unsafe fn on_interrupt() {
        let flags = pac::DMA1.isr().read();
        let half = flags.htif(3);
        let full = flags.tcif(3);
        if flags.teif(3) || (half && full) {
            DMA_ERRORS.fetch_add(1, Ordering::Relaxed);
        }
        if half {
            HT.fetch_add(1, Ordering::Relaxed);
        }
        if full {
            TC.fetch_add(1, Ordering::Relaxed);
        }
        if half || full {
            let now = Instant::now().as_micros() as u32;
            let _ = FIRST_IRQ.compare_exchange(0, now, Ordering::Relaxed, Ordering::Relaxed);
            LAST_IRQ.store(now, Ordering::Relaxed);
        }
        <dma::InterruptHandler<embassy_stm32::peripherals::DMA1_CH4> as embassy_stm32::interrupt::typelevel::Handler<embassy_stm32::interrupt::typelevel::DMA1_CHANNEL4>>::on_interrupt();
    }
}
// SAFETY: the vector above always delegates to this exact Embassy handler.
unsafe impl
    embassy_stm32::interrupt::typelevel::Binding<
        embassy_stm32::interrupt::typelevel::DMA1_CHANNEL4,
        dma::InterruptHandler<embassy_stm32::peripherals::DMA1_CH4>,
    > for Irqs
{
}

static AUDIO: AudioSource<RING, 1> = AudioSource::new(AudioFormat::new(SAMPLE_RATE, 1, 1_000_000));
static mut DMA_WORDS: [u32; RING] = [0; RING];
#[no_mangle]
pub static mut AUDIO_SNAPSHOT: [u32; SNAPSHOT_WORDS] = [0; SNAPSHOT_WORDS];
#[no_mangle]
pub static mut AUDIO_SNAPSHOT_META: [u32; 12] = [0; 12];

#[embassy_executor::main]
async fn main(_spawner: Spawner) -> ! {
    let mut config = embassy_stm32::Config::default();
    config.rcc.hse = Some(Hse {
        freq: Hertz(16_000_000),
        mode: HseMode::Oscillator,
    });
    config.rcc.pll = Some(Pll {
        source: PllSource::HSE,
        prediv: PllPreDiv::DIV2,
        mul: PllMul::MUL20,
        divp: None,
        divq: None,
        divr: Some(PllRDiv::DIV2),
    });
    config.rcc.sys = Sysclk::PLL1_R;
    config.rcc.ahb_pre = AHBPrescaler::DIV1;
    config.rcc.apb1_pre = APBPrescaler::DIV1;
    config.rcc.apb2_pre = APBPrescaler::DIV1;
    config.rcc.ls = rcc::LsConfig::off();
    let p = embassy_stm32::init(config);
    let _red = Output::new(p.PB11, Level::High, Speed::Low);
    let _green = Output::new(p.PC3, Level::High, Speed::Low);
    let mut blue = Output::new(p.PC1, Level::High, Speed::Low);
    assert!(
        rcc::clocks(&p.RCC).sys.to_hertz().unwrap().0 == SYS_HZ && pac::TIM5.psc().read() == 79
    );
    info!("L4 AUDIO boot chip=STM32L476RG sys_hz={} mic_clock=PC2_AF6 data=PB12_AF6 channel=1 edge=rising pdm_hz={} sinc={} fosr={} iosr={} shift={} rate_hz={} valid_bits=24 dma_word_bits=32",SYS_HZ,PDM_HZ,SINC_ORDER,FOSR,IOSR,RIGHT_SHIFT,SAMPLE_RATE);
    let mut microphone = dfsdm::Dfsdm::new(p.PC2, p.PB12);
    Timer::after_millis(100).await; // Clock/startup settle before conversion/DMA.
    let buffer = unsafe { &mut *core::ptr::addr_of_mut!(DMA_WORDS) };
    let snapshot = unsafe { &mut *core::ptr::addr_of_mut!(AUDIO_SNAPSHOT) };
    let addr = buffer.as_ptr() as usize;
    let dma_bytes = core::mem::size_of_val(buffer);
    assert!(addr >= 0x2000_0000 && addr + dma_bytes <= 0x2001_8000 && addr % 4 == 0);
    let mut reader = AUDIO.register(ReaderStart::Latest).unwrap();
    let mut ring = unsafe {
        ReadableRingBuffer::new(
            dma::Channel::new(p.DMA1_CH4, Irqs),
            0,
            dfsdm::data_ptr(),
            buffer,
            TransferOptions::default(),
        )
    };
    ring.set_alignment(HALF);
    ring.start();
    microphone.start();
    let dma_cr = pac::DMA1.ch(3).cr().read().0;
    assert!(pac::DMA1.cselr().read().cs(3) == 0);
    assert!((dma_cr & (1 << 5)) != 0 && (dma_cr & (1 << 7)) != 0); // Circular / memory increment.
    assert!(((dma_cr >> 8) & 3) == 2 && ((dma_cr >> 10) & 3) == 2); // 32-bit peripheral / memory.
    info!("L4 AUDIO DMA configured dma=DMA1_CH4 request=0 cr={:08x} ndtr={} buffer={:08x} bytes={} half_samples={} half_us=32000 snapshot={:08x} snapshot_words={} meta={:08x}",dma_cr,pac::DMA1.ch(3).ndtr().read().ndt(),addr,dma_bytes,HALF,snapshot.as_ptr() as usize,SNAPSHOT_WORDS,core::ptr::addr_of!(AUDIO_SNAPSHOT_META) as usize);
    let mut process = [0u32; HALF];
    let mut min = i32::MAX;
    let mut max = i32::MIN;
    let mut positive = 0u64;
    let mut negative = 0u64;
    let mut mean_power_sum = 0u64;
    let mut low_or = 0u8;
    let mut low_and = 0xffu8;
    let mut max_processing_us = 0u64;
    let mut snapshot_cursor = SNAPSHOT_WORDS;
    let mut snapshot_complete = false;
    let mut snapshot_start = 0u64;
    let mut best_power = 0u64;
    let mut burst_power = 0u64;
    let mut next_report = Instant::now();
    let mut seen_samples = 0u64;
    let total_blocks = WARMUP_BLOCKS + QUALIFICATION_BLOCKS;
    for block in 1..=total_blocks {
        let remaining = with_timeout(Duration::from_secs(1), ring.read_exact(&mut process))
            .await
            .expect("L4 AUDIO FAIL DMA stalled")
            .expect("L4 AUDIO FAIL DMA overwrite");
        let begun = Instant::now();
        let completions = HT.load(Ordering::Relaxed) + TC.load(Ordering::Relaxed);
        assert!(
            DMA_ERRORS.load(Ordering::Relaxed) == 0 && completions == block,
            "L4 AUDIO FAIL DMA service deadline/error"
        );
        let status = dfsdm::read(dfsdm::FLT_ISR);
        assert!(
            status & ((1 << 3) | (1 << 2) | (1 << 17)) == 0,
            "L4 AUDIO FAIL filter overrun/clock absence"
        );
        let mut stats = Stats::default();
        for word in &process {
            stats.add(*word);
        }
        if block == 1 {
            info!(
                "L4 AUDIO first DMA words={:08x},{:08x},{:08x},{:08x}",
                process[0], process[1], process[2], process[3]
            );
        }
        assert!(
            stats.bad_metadata == 0,
            "L4 AUDIO FAIL unexpected pending/reserved bits"
        );
        AUDIO
            .write_from_fn(HALF, LAST_IRQ.load(Ordering::Relaxed) as u64, |i| {
                pcm24(process[i])
            })
            .unwrap();
        let result = reader
            .read(HALF, |chunk| {
                assert!(chunk.position == seen_samples && chunk.len() == HALF);
                assert!(chunk.dropped_since_last_read == 0);
                for (i, value) in chunk.first.iter().chain(chunk.second.iter()).enumerate() {
                    assert!(*value == pcm24(process[i]));
                }
                HALF
            })
            .unwrap();
        assert!(
            result.consumed_samples == HALF
                && result.dropped_since_last_read == 0
                && result.overruns_since_last_read == 0
        );
        seen_samples += HALF as u64;
        if block > WARMUP_BLOCKS {
            assert!(stats.clipped == 0, "L4 AUDIO FAIL PCM rail clipping");
            min = min.min(stats.min);
            max = max.max(stats.max);
            positive += stats.positive as u64;
            negative += stats.negative as u64;
            mean_power_sum += stats.mean_power();
            low_or |= stats.low_or;
            low_and &= stats.low_and;
            // Retain a contiguous 0.512 s burst around a newly stronger event.
            if snapshot_cursor == SNAPSHOT_WORDS
                && (!snapshot_complete || stats.ac_mean_power() > best_power.saturating_mul(2))
                && block + 16 <= total_blocks
            {
                snapshot_cursor = 0;
                snapshot_start = seen_samples - HALF as u64;
                burst_power = 0;
            }
            if snapshot_cursor < SNAPSHOT_WORDS {
                snapshot[snapshot_cursor..snapshot_cursor + HALF].copy_from_slice(&process);
                snapshot_cursor += HALF;
                burst_power = burst_power.max(stats.ac_mean_power());
                if snapshot_cursor == SNAPSHOT_WORDS {
                    snapshot_complete = true;
                    best_power = burst_power;
                }
            }
        }
        max_processing_us = max_processing_us.max((Instant::now() - begun).as_micros());
        if Instant::now() >= next_report {
            info!("L4 AUDIO progress block={} samples={} irq_halves={} irq_full={} remaining={} filter_status={:08x} min={} max={} mean={} power={} low_or={:02x} low_and={:02x} max_processing_us={} first_words={:08x},{:08x},{:08x},{:08x}",block,seen_samples,HT.load(Ordering::Relaxed),TC.load(Ordering::Relaxed),remaining,status,stats.min,stats.max,stats.sum/HALF as i64,stats.mean_power(),stats.low_or,stats.low_and,max_processing_us,process[0],process[1],process[2],process[3]);
            next_report = Instant::now() + Duration::from_secs(2);
        }
    }
    let elapsed = LAST_IRQ.load(Ordering::Relaxed) - FIRST_IRQ.load(Ordering::Relaxed);
    let expected = (total_blocks as u64 - 1) * HALF as u64 * 1_000_000 / SAMPLE_RATE as u64;
    assert!(
        (elapsed as i64 - expected as i64).unsigned_abs() < 1000,
        "L4 AUDIO FAIL nominal sample rate"
    );
    assert!(
        max as i64 - min as i64 > 32 && (low_or ^ low_and) != 0,
        "L4 AUDIO FAIL stuck/truncated PCM"
    );
    assert!(max_processing_us < 32_000 && snapshot_complete && snapshot_cursor == SNAPSHOT_WORDS);
    let final_status = dfsdm::read(dfsdm::FLT_ISR);
    microphone.stop();
    ring.request_reset();
    drop(ring);
    assert!(!pac::DMA1.ch(3).cr().read().en());
    let hash = hash_words(snapshot);
    let meta = unsafe { &mut *core::ptr::addr_of_mut!(AUDIO_SNAPSHOT_META) };
    let snapshot_meta = [
        u32::from_le_bytes(*b"L4AU"),
        1,
        SAMPLE_RATE,
        SNAPSHOT_WORDS as u32,
        1,
        24,
        32,
        1,
        snapshot_start as u32,
        (snapshot_start >> 32) as u32,
        hash,
        0,
    ];
    unsafe {
        core::ptr::write_volatile(meta, snapshot_meta);
    }
    info!("L4 AUDIO SNAPSHOT READY words={} start_sample={} raw_fnv32={:08x} address={:08x} meta={:08x} capture_stopped=true",SNAPSHOT_WORDS,snapshot_start,hash,snapshot.as_ptr() as usize,meta.as_ptr() as usize);
    info!("L4 AUDIO PASS rate_hz={} qualified_samples={} qualified_seconds=60 total_samples={} irq_halves={} irq_full={} dma_errors={} filter_status={:08x} elapsed_us={} expected_us={} min={} max={} positive={} negative={} mean_power={} low_byte_var={:02x} max_processing_us={} audiosource_drops=0",SAMPLE_RATE,QUALIFICATION_BLOCKS as usize*HALF,seen_samples,HT.load(Ordering::Relaxed),TC.load(Ordering::Relaxed),DMA_ERRORS.load(Ordering::Relaxed),final_status,elapsed,expected,min,max,positive,negative,mean_power_sum/QUALIFICATION_BLOCKS as u64,low_or^low_and,max_processing_us);
    loop {
        blue.toggle();
        Timer::after_millis(500).await;
    }
}
