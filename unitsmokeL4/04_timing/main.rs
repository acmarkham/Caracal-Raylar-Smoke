//! ADR-L4-009: hardware carry chain feeding the unchanged upstream Time Service.
#![no_std]
#![no_main]
mod capture;

use core::{
    cell::RefCell,
    sync::atomic::{AtomicU32, Ordering},
};
use critical_section::Mutex;
use defmt::{assert, info};
use embassy_executor::Spawner;
use embassy_stm32::{
    bind_interrupts,
    gpio::{Input, Level, Output, Pull, Speed},
    pac,
    rcc::{
        self, AHBPrescaler, APBPrescaler, Hse, HseMode, Pll, PllMul, PllPreDiv, PllRDiv, PllSource,
        Sysclk,
    },
    time::Hertz,
    usart::{self, BufferedUart},
};
use embassy_time::{with_timeout, Duration, Instant, Timer};
use embedded_io_async::{Read, Write};
use gps_wire::{ascii_field, nav_pvt, poll, version, Frame, Framer};
use raylar_drivers::gps::nmea::{NavigationEvent, NmeaParser};
use raylar_time_service::{
    gps::gps_utc_to_timestamp, Anchor, AnchorQuality, TimeConfig, TimeResources, TimeService,
    TimeSource, UtcTimestamp,
};
use unit_smoke_l4_04_timing::Labels;
use {defmt_rtt as _, panic_probe as _};

struct NoHeap;
unsafe impl core::alloc::GlobalAlloc for NoHeap {
    unsafe fn alloc(&self, _: core::alloc::Layout) -> *mut u8 {
        panic!("L4 timing unexpected heap allocation");
    }
    unsafe fn dealloc(&self, _: *mut u8, _: core::alloc::Layout) {
        panic!("L4 timing unexpected heap deallocation");
    }
}
#[global_allocator]
static ALLOCATOR: NoHeap = NoHeap;

bind_interrupts!(struct Irqs {
    USART2 => UartIrq;
    TIM3 => capture::Handler;
});

static UART_ERRORS: AtomicU32 = AtomicU32::new(0);
struct UartIrq;
impl embassy_stm32::interrupt::typelevel::Handler<embassy_stm32::interrupt::typelevel::USART2>
    for UartIrq
{
    unsafe fn on_interrupt() {
        let s = pac::USART2.isr().read();
        if s.pe() || s.fe() || s.ne() || s.ore() {
            UART_ERRORS.fetch_add(1, Ordering::Relaxed);
        }
        <usart::BufferedInterruptHandler<embassy_stm32::peripherals::USART2> as embassy_stm32::interrupt::typelevel::Handler<embassy_stm32::interrupt::typelevel::USART2>>::on_interrupt();
    }
}
// SAFETY: UartIrq always delegates to this exact handler.
unsafe impl
    embassy_stm32::interrupt::typelevel::Binding<
        embassy_stm32::interrupt::typelevel::USART2,
        usart::BufferedInterruptHandler<embassy_stm32::peripherals::USART2>,
    > for Irqs
{
}

static TIME: TimeResources<4, 8> = TimeResources::new();
#[derive(Clone, Copy)]
struct Pps {
    edge: capture::Edge,
    delta: Option<Duration>,
}
static LATEST: Mutex<RefCell<Option<Pps>>> = Mutex::new(RefCell::new(None));
static PPS_MAX_AGE: AtomicU32 = AtomicU32::new(0);

#[embassy_executor::task]
async fn pps_task() -> ! {
    let mut previous: Option<capture::Edge> = None;
    loop {
        let edge = capture::EDGES.receive().await;
        let delta = previous.map(|last| {
            Duration::from_micros(
                edge.ticks
                    .checked_sub(last.ticks)
                    .expect("capture counter went backwards"),
            )
        });
        PPS_MAX_AGE.fetch_max(edge.age_us, Ordering::Relaxed);
        if let Some(delta) = delta {
            assert!(
                (999_980..=1_000_020).contains(&delta.as_micros()),
                "L4 TIMING invalid PPS cadence"
            );
        }
        critical_section::with(|cs| *LATEST.borrow(cs).borrow_mut() = Some(Pps { edge, delta }));
        if edge.sequence <= 3 || edge.sequence % 60 == 0 {
            info!("L4 TIMING PPS seq={} ticks={} high={} low={} delta_us={:?} timestamp_us={} observed_us={} irq_age_us={} max_age_us={} timer_irqs={}",
                edge.sequence, edge.ticks, edge.ticks >> 16, edge.ticks as u16, delta.map(|d| d.as_micros()), edge.timestamp.as_micros(), edge.observed.as_micros(), edge.age_us, PPS_MAX_AGE.load(Ordering::Relaxed), capture::IRQS.load(Ordering::Relaxed));
        }
        previous = Some(edge);
    }
}

#[embassy_executor::task]
async fn time_task() -> ! {
    TimeService::new(&TIME, TimeConfig::default()).run().await
}

#[embassy_executor::task]
async fn green_task(mut green: Output<'static>) -> ! {
    let mut states = TIME.state_receiver().unwrap();
    let mut accepted = 0;
    loop {
        let state = states.changed().await;
        if state.active_time_source == TimeSource::GpsPps && state.accepted_anchors > accepted {
            accepted = state.accepted_anchors;
            green.set_low();
            Timer::after_millis(50).await;
            green.set_high();
        }
    }
}

enum Phase {
    Calibrating,
    Holdover {
        utc: i64,
        system: u64,
        predicted: i64,
        accepted: u32,
    },
    Reacquiring {
        accepted: u32,
        raw_error: i64,
        corrected_error: i64,
    },
    Passed,
}

async fn send(uart: &mut BufferedUart<'_>, bytes: &[u8]) {
    with_timeout(Duration::from_secs(1), uart.write_all(bytes))
        .await
        .unwrap()
        .unwrap();
    with_timeout(Duration::from_secs(1), uart.flush())
        .await
        .unwrap()
        .unwrap();
}

#[embassy_executor::main]
async fn main(spawner: Spawner) -> ! {
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
    let _blue = Output::new(p.PC1, Level::High, Speed::Low);
    let green = Output::new(p.PC3, Level::High, Speed::Low);
    let enable = Input::new(p.PB4, Pull::None);
    assert!(
        rcc::clocks(&p.RCC).sys.to_hertz().unwrap().0 == 80_000_000 && pac::TIM5.psc().read() == 79
    );
    info!("L4 TIMING boot chip=STM32L476RG hse_hz=16000000 sys_hz=80000000 tick_hz=1000000 gps_enable_input_high={} calibration=TimeConfig_default baseline_s=60 samples=11 holdover_test_s=60", enable.is_high());
    let chain = capture::start(p.TIM3, p.TIM2, p.PB5);
    let quality_us = chain.uncertainty_us;
    // Keep exclusive timer/pin ownership for the entire diagnostic.
    let _chain = chain;
    spawner.spawn(pps_task().unwrap());
    spawner.spawn(time_task().unwrap());
    spawner.spawn(green_task(green).unwrap());
    let mut tx = [0; 256];
    let mut rx = [0; 4096];
    let mut cfg = usart::Config::default();
    cfg.baudrate = 38400;
    cfg.eager_reads = Some(1);
    let mut uart = BufferedUart::new(p.USART2, p.PA3, p.PA2, &mut tx, &mut rx, Irqs, cfg).unwrap();
    let mut bytes = [0; 256];
    let mut framer = Framer::default();
    let mut nmea = NmeaParser::new();
    let mut labels = Labels::default();
    let mut identity = false;
    let mut uart_locked = false;
    let mut valid_frames = 0u32;
    let mut nav_valid_at: Option<Instant> = None;
    let mut next_poll = Instant::now();
    let mut next_report = Instant::now();
    let started = Instant::now();
    let mut first: Option<(u64, i64)> = None;
    let mut last_pair: Option<(u64, i64)> = None;
    let mut phase = Phase::Calibrating;
    let mut matched = 0u32;
    let mut withheld = 0u32;
    let mut rejected_labels = 0u32;
    let anchors = TIME.anchor_sender();
    loop {
        if Instant::now() >= next_poll {
            if !identity {
                send(&mut uart, &poll(10, 4)).await;
            }
            send(&mut uart, &poll(1, 7)).await;
            next_poll = Instant::now() + Duration::from_secs(2);
        }
        if let Ok(Ok(n)) = with_timeout(Duration::from_millis(100), uart.read(&mut bytes)).await {
            for byte in &bytes[..n] {
                let frame = framer.push(*byte);
                if frame.is_some() && !uart_locked {
                    valid_frames += 1;
                    if valid_frames >= 2 {
                        let startup_errors = UART_ERRORS.swap(0, Ordering::Relaxed);
                        info!("L4 TIMING UART PASS baud=38400 startup_error_irqs={} (mid-stream startup excluded)", startup_errors);
                        uart_locked = true;
                    }
                }
                match frame {
                    Some(Frame::Ubx {
                        class: 10,
                        id: 4,
                        payload,
                    }) => {
                        if let Some(v) = version(payload) {
                            info!("L4 TIMING MON-VER sw={} hw={}", v.software, v.hardware);
                            for f in v.extensions.chunks_exact(30) {
                                if let Some(s) = ascii_field(f) {
                                    info!("L4 TIMING MON-VER extension={}", s);
                                }
                            }
                            identity = true;
                        }
                    }
                    Some(Frame::Ubx {
                        class: 1,
                        id: 7,
                        payload,
                    }) => {
                        nav_valid_at = nav_pvt(payload)
                            .filter(|p| p.qualified())
                            .map(|_| Instant::now());
                    }
                    Some(Frame::Nmea(sentence)) => {
                        let Ok(Some(NavigationEvent::Fix(fix))) = nmea.parse(sentence) else {
                            continue;
                        };
                        if !uart_locked
                            || fix.utc_time.date.is_none()
                            || !identity
                            || !nav_valid_at
                                .is_some_and(|at| (Instant::now() - at).as_millis() < 3000)
                        {
                            continue;
                        }
                        let Some(utc) = gps_utc_to_timestamp(fix.utc_time) else {
                            continue;
                        };
                        let Some(pps) = critical_section::with(|cs| *LATEST.borrow(cs).borrow())
                        else {
                            continue;
                        };
                        if !labels.accept(
                            pps.edge.sequence,
                            pps.edge.timestamp.as_micros(),
                            Instant::now().as_micros(),
                            utc.seconds,
                        ) {
                            rejected_labels += 1;
                            continue;
                        }
                        let system = pps.edge.timestamp.as_micros();
                        let actual = utc.as_micros();
                        let state = TIME.time_state();
                        let predicted = state
                            .system_to_utc(pps.edge.timestamp)
                            .ok()
                            .map(UtcTimestamp::as_micros);
                        let residual = predicted.map(|predicted| actual - predicted);
                        if first.is_none() {
                            first = Some((system, actual));
                        }
                        last_pair = Some((system, actual));
                        matched += 1;
                        let mut deliver = true;
                        match phase {
                            Phase::Calibrating if state.frequency_calibration_locked => {
                                assert!(
                                    state.frequency_calibration_samples == 11
                                        && state.utc_second_corrections == 0
                                );
                                info!("L4 TIMING CALIBRATION PASS elapsed_s={} calibrated_ppb={} samples={} accepted={} rejected={} residual_us={:?} uncertainty_us={}",
                                    (Instant::now()-started).as_secs(), state.calibrated_frequency_error_ppb, state.frequency_calibration_samples, state.accepted_anchors, state.rejected_anchors, residual, state.uncertainty_us);
                                phase = Phase::Holdover {
                                    utc: actual,
                                    system,
                                    predicted: predicted.unwrap(),
                                    accepted: state.accepted_anchors,
                                };
                                deliver = false;
                                info!("L4 TIMING holdover start: withholding anchors; receiver remains powered and captures continue");
                            }
                            Phase::Holdover {
                                utc: begin_utc,
                                system: begin_system,
                                predicted: begin_predicted,
                                accepted,
                            } => {
                                deliver = false;
                                assert!(
                                    state.accepted_anchors == accepted
                                        && state.frequency_calibration_locked
                                );
                                if actual - begin_utc >= 60_000_000 {
                                    let raw_error =
                                        (actual - begin_utc) - (system - begin_system) as i64;
                                    let corrected_error = (actual - begin_utc)
                                        - (predicted.unwrap() - begin_predicted);
                                    assert!(state.holdover_duration.as_secs() >= 58);
                                    assert!(
                                        corrected_error.unsigned_abs() <= 50,
                                        "holdover corrected elapsed error exceeds 50 us"
                                    );
                                    assert!(
                                        corrected_error.unsigned_abs()
                                            <= raw_error.unsigned_abs() / 4 + 10,
                                        "holdover failed to improve raw drift"
                                    );
                                    info!("L4 TIMING HOLDOVER PASS utc_span_us={} raw_error_us={} corrected_error_us={} calibrated_ppb={} uncertainty_us={} duration_ms={} accepted_unchanged={}",
                                        actual-begin_utc, raw_error, corrected_error, state.calibrated_frequency_error_ppb, state.uncertainty_us, state.holdover_duration.as_millis(), accepted);
                                    phase = Phase::Reacquiring {
                                        accepted,
                                        raw_error,
                                        corrected_error,
                                    };
                                    deliver = true;
                                }
                            }
                            Phase::Reacquiring {
                                accepted,
                                raw_error,
                                corrected_error,
                            } if state.accepted_anchors >= accepted + 20 => {
                                assert!(
                                    !state.pps_reacquisition_active
                                        && state.utc_second_corrections == 0
                                );
                                assert!(residual.is_some_and(|r| r.unsigned_abs() <= 50));
                                info!("L4 TIMING PASS elapsed_s={} calibrated_ppb={} samples={} accepted={} rejected={} reacq_rejections={} raw_holdover_error_us={} corrected_holdover_error_us={} residual_us={:?} max_irq_age_us={} timer_irqs={} overflow_irq=false",
                                    (Instant::now()-started).as_secs(), state.calibrated_frequency_error_ppb, state.frequency_calibration_samples, state.accepted_anchors, state.rejected_anchors, state.pps_reacquisition_rejections,
                                    raw_error, corrected_error, residual, PPS_MAX_AGE.load(Ordering::Relaxed), capture::IRQS.load(Ordering::Relaxed));
                                phase = Phase::Passed;
                            }
                            _ => {}
                        }
                        if matched <= 3 || matched % 30 == 0 {
                            let (begin_system, begin_utc) = first.unwrap();
                            info!("L4 TIMING anchor seq={} utc_s={} capture_ticks={} timestamp_us={} nmea_age_us={} quality_us={} raw_total_error_us={} corrected_residual_us={:?} deliver={}",
                                pps.edge.sequence, utc.seconds, pps.edge.ticks, system, Instant::now().as_micros()-system, quality_us,
                                (actual-begin_utc)-(system-begin_system) as i64, residual, deliver);
                        }
                        if deliver {
                            anchors
                                .send(Anchor {
                                    system_time: pps.edge.timestamp,
                                    utc,
                                    quality: AnchorQuality::new(quality_us),
                                    source: TimeSource::GpsPps,
                                    capture_ticks: Some(pps.edge.ticks),
                                    pps_sequence: Some(pps.edge.sequence),
                                    pps_interval: pps.delta,
                                })
                                .await;
                        } else {
                            withheld += 1;
                        }
                    }
                    _ => {}
                }
            }
        }
        let now = Instant::now();
        assert!(
            !uart_locked || UART_ERRORS.load(Ordering::Relaxed) == 0,
            "UART error at verified 38400 baud"
        );
        assert!(
            capture::FAULTS.load(Ordering::Relaxed) == 0,
            "capture overrun/latency/queue fault"
        );
        assert!(!pac::TIM3.dier().read().uie() && pac::TIM2.dier().read().0 == 0);
        assert!(
            matches!(phase, Phase::Passed) || (now - started).as_secs() < 1800,
            "L4 TIMING deadline exceeded"
        );
        if now >= next_report {
            let state = TIME.time_state();
            let phase_name = match phase {
                Phase::Calibrating => "calibrating",
                Phase::Holdover { .. } => "holdover",
                Phase::Reacquiring { .. } => "reacquiring",
                Phase::Passed => "passed",
            };
            info!("L4 TIMING status elapsed_s={} phase={} utc_status={:?} calibrated_ppb={} slew_ppb={} samples={} locked={} accepted={} rejected={} residual_us={:?} uncertainty_us={} holdover_ms={} matched={} withheld={} label_rejects={} timer_irqs={} capture_faults={} uart_errors={} frame_rejects={} last_pair={:?}",
                (now-started).as_secs(), phase_name, state.utc_status, state.calibrated_frequency_error_ppb, state.phase_slew_ppb,
                state.frequency_calibration_samples, state.frequency_calibration_locked, state.accepted_anchors, state.rejected_anchors,
                state.last_anchor_residual_us, state.uncertainty_us, state.holdover_duration.as_millis(), matched, withheld, rejected_labels,
                capture::IRQS.load(Ordering::Relaxed), capture::FAULTS.load(Ordering::Relaxed), UART_ERRORS.load(Ordering::Relaxed), framer.rejected, last_pair);
            next_report = now + Duration::from_secs(10);
        }
    }
}
