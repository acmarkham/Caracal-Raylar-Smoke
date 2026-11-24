use core::cell::RefCell;
use core::sync::atomic::{AtomicU32, Ordering};
use critical_section::Mutex;
use embassy_stm32::gpio::Pull;
use embassy_stm32::interrupt::{InterruptExt, Priority};
use embassy_stm32::peripherals::{PB5, TIM2, TIM3};
use embassy_stm32::timer::{input_capture::CapturePin, low_level::Timer, Ch2};
use embassy_stm32::{pac, Peri};
use embassy_sync::{blocking_mutex::raw::CriticalSectionRawMutex, channel::Channel};
use embassy_time::Instant;
use unit_smoke_l4_04_timing::{captured_count, MASK48};

#[derive(Clone, Copy)]
pub struct Edge {
    pub ticks: u64,
    pub timestamp: Instant,
    pub observed: Instant,
    pub sequence: u64,
    pub age_us: u32,
}

#[derive(Clone, Copy)]
struct State {
    offset: i64,
    sequence: u64,
}
static STATE: Mutex<RefCell<State>> = Mutex::new(RefCell::new(State {
    offset: 0,
    sequence: 0,
}));
pub static EDGES: Channel<CriticalSectionRawMutex, Edge, 8> = Channel::new();
pub static IRQS: AtomicU32 = AtomicU32::new(0);
pub static FAULTS: AtomicU32 = AtomicU32::new(0);

pub struct Chain {
    _master: Timer<'static, TIM3>,
    _upper: Timer<'static, TIM2>,
    _pin: CapturePin<'static, TIM3, Ch2>,
    pub uncertainty_us: u64,
}

/// Retry a torn high/low read and avoid the carry propagation aperture. Both
/// timers run on APB1; eight 1 MHz ticks greatly exceed the synchronizer delay.
pub fn now_ticks() -> Option<u64> {
    for _ in 0..1024 {
        let high1 = pac::TIM2.cnt().read();
        let low = pac::TIM3.cnt().read().cnt();
        let high2 = pac::TIM2.cnt().read();
        if high1 == high2 && (8..=65_527).contains(&low) {
            return Some(((high1 as u64) << 16) | low as u64);
        }
    }
    None
}

pub struct Handler;
impl embassy_stm32::interrupt::typelevel::Handler<embassy_stm32::interrupt::typelevel::TIM3>
    for Handler
{
    unsafe fn on_interrupt() {
        IRQS.fetch_add(1, Ordering::Relaxed);
        let observed = Instant::now();
        let status = pac::TIM3.sr().read();
        if !status.ccif(1) {
            FAULTS.fetch_add(1, Ordering::Relaxed);
            return;
        }
        let low = pac::TIM3.ccr(1).read().ccr(); // Reading CCR2 clears CC2IF.
        let live = now_ticks();
        // rc_w0: preserve all other flags. UIE remains disabled throughout.
        pac::TIM3.sr().write(|w| w.0 = !(1 << 10));
        let Some(live) = live else {
            FAULTS.fetch_add(1, Ordering::Relaxed);
            return;
        };
        if status.ccof(1) {
            FAULTS.fetch_add(1, Ordering::Relaxed);
            return;
        }
        let ticks = captured_count(live, low);
        let age_us = live.wrapping_sub(ticks) & MASK48;
        if age_us > 10_000 {
            FAULTS.fetch_add(1, Ordering::Relaxed);
            return;
        }
        let edge = critical_section::with(|cs| {
            let mut state = STATE.borrow(cs).borrow_mut();
            state.sequence += 1;
            Edge {
                ticks,
                timestamp: Instant::from_micros((ticks as i64 + state.offset) as u64),
                observed,
                sequence: state.sequence,
                age_us: age_us as u32,
            }
        });
        if EDGES.try_send(edge).is_err() {
            FAULTS.fetch_add(1, Ordering::Relaxed);
        }
    }
}

pub fn start(
    master: Peri<'static, TIM3>,
    upper: Peri<'static, TIM2>,
    pin: Peri<'static, PB5>,
) -> Chain {
    let master = Timer::new(master);
    let upper = Timer::new(upper);
    pac::TIM3.psc().write(|w| *w = 79);
    pac::TIM3.arr().write(|w| w.set_arr(u16::MAX));
    pac::TIM3.egr().write(|w| w.set_ug(true));
    pac::TIM3
        .ccmr_input(0)
        .write(|w| w.set_ccs(1, pac::timer::vals::CcmrInputCcs::from_bits(1)));
    pac::TIM2.psc().write(|w| *w = 0);
    pac::TIM2.arr().write(|w| *w = u32::MAX);
    pac::TIM2.egr().write(|w| w.set_ug(true));
    pac::TIM3
        .cr2()
        .write(|w| w.set_mms(pac::timer::vals::Mms::from_bits(2)));
    pac::TIM2.smcr().write(|w| {
        w.set_ts(pac::timer::vals::Ts::from_bits(2));
        w.set_sms(pac::timer::vals::Sms::from_bits(7));
    });
    pac::TIM2.cr1().modify(|w| w.set_cen(true));
    pac::TIM3.ccer().write(|w| w.set_cce(1, true));
    // Exercise software-generated capture and a natural hardware carry while
    // the external channel and NVIC remain disabled. No fake edge reaches UTC.
    for (high, low) in [(7u32, 2u16), (7, 65_534), (u32::MAX, 65_534)] {
        pac::TIM3.cr1().modify(|w| w.set_cen(false));
        pac::TIM2.cnt().write(|w| *w = high);
        pac::TIM3.cnt().write(|w| w.set_cnt(low));
        pac::TIM3.sr().write(|w| w.0 = 0);
        pac::TIM3.egr().write(|w| w.set_ccg(1, true));
        cortex_m::asm::dsb();
        let captured = pac::TIM3.ccr(1).read().ccr();
        defmt::info!(
            "L4 TIMING synthetic input ccmr={:08x} ccer={:08x} cnt={} ccr2={} expected={}",
            pac::TIM3.ccmr_input(0).read().0,
            pac::TIM3.ccer().read().0,
            pac::TIM3.cnt().read().cnt(),
            captured,
            low
        );
        defmt::assert!(captured == low);
        pac::TIM3.cr1().modify(|w| w.set_cen(true));
        cortex_m::asm::delay(8_000); // ~100 us: crosses carry for low=65534.
        let live = now_ticks().unwrap();
        let expected = ((high as u64) << 16) | low as u64;
        defmt::assert!(captured_count(live, low) == expected);
        let elapsed = live.wrapping_sub(expected) & MASK48;
        defmt::assert!((50..=2000).contains(&elapsed));
        defmt::info!(
            "L4 TIMING carry selftest high={:08x} ccr2={} live={} elapsed_us={} PASS",
            high,
            low,
            live,
            elapsed
        );
    }
    pac::TIM3.cr1().modify(|w| w.set_cen(false));
    pac::TIM2.cnt().write(|w| *w = 0);
    pac::TIM3.cnt().write(|w| w.set_cnt(0));
    pac::TIM3.sr().write(|w| w.0 = 0);
    pac::TIM2.sr().write(|w| w.0 = 0);
    pac::TIM3.cr1().modify(|w| w.set_cen(true));
    let mut alignment = None;
    for _ in 0..32 {
        let before = Instant::now();
        let ticks = now_ticks().unwrap();
        let after = Instant::now();
        let span = (after - before).as_micros();
        if span <= 4 {
            alignment = Some((
                before.as_micros() as i64 + span as i64 / 2 - ticks as i64,
                span.div_ceil(2) + 2,
            ));
            break;
        }
    }
    let (offset, bound_us) = alignment.expect("TIM3/TIM5 cross-clock bracket too wide");
    critical_section::with(|cs| {
        *STATE.borrow(cs).borrow_mut() = State {
            offset,
            sequence: 0,
        }
    });
    // Freeze the entire chain together if a debugger halts the CPU. Bench
    // evidence still uses attached reset runs, never a live mid-test attach.
    pac::DBGMCU.apb1fzr1().modify(|w| {
        w.set_tim2(true);
        w.set_tim3(true);
        w.set_tim5(true);
    });
    let pin = CapturePin::new(pin, Pull::Down);
    pac::TIM3.ccer().write(|w| w.set_cce(1, true));
    pac::TIM3.dier().write(|w| w.set_ccie(1, true));
    embassy_stm32::interrupt::TIM3.set_priority(Priority::P0);
    embassy_stm32::interrupt::TIM3.unpend();
    unsafe {
        embassy_stm32::interrupt::TIM3.enable();
    }
    defmt::assert!(!pac::TIM3.dier().read().uie() && pac::TIM2.dier().read().0 == 0);
    defmt::info!("L4 TIMING CHAIN PASS low=TIM3_CH2_PB5_AF2 psc={} arr={} cr2={:08x} high=TIM2 smcr={:08x} arr={:08x} offset_us={} uncertainty_us={} update_irq=false",
        pac::TIM3.psc().read(), pac::TIM3.arr().read().arr(), pac::TIM3.cr2().read().0,
        pac::TIM2.smcr().read().0, pac::TIM2.arr().read(), offset, bound_us);
    Chain {
        _master: master,
        _upper: upper,
        _pin: pin,
        uncertainty_us: bound_us,
    }
}
