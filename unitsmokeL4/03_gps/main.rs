//! ADR-L4-008: continuous, read-only u-blox UART/fix/PPS bring-up.
#![no_std]
#![no_main]

use core::cell::RefCell;
use core::sync::atomic::{AtomicU32, Ordering};
use critical_section::Mutex;
use defmt::{assert, info, warn};
use embassy_executor::Spawner;
use embassy_stm32::exti::{ExtiInput, InterruptHandler as ExtiHandler};
use embassy_stm32::gpio::{Input, Level, Output, Pull, Speed};
use embassy_stm32::mode::Async;
use embassy_stm32::rcc::{
    self, AHBPrescaler, APBPrescaler, Hse, HseMode, Pll, PllMul, PllPreDiv, PllRDiv, PllSource,
    Sysclk,
};
use embassy_stm32::usart::{self, BufferedUart};
use embassy_stm32::{bind_interrupts, pac, time::Hertz};
use embassy_time::{with_timeout, Duration, Instant};
use embedded_io_async::{Read, Write};
use raylar_drivers::gps::nmea::{NavigationEvent, NmeaParser};
use raylar_drivers::gps::UtcTime;
use unit_smoke_l4_03_gps::{ascii_field, nav_pvt, poll, version, Frame, Framer, PpsCheck, Pvt};
use {defmt_rtt as _, panic_probe as _};

// The portable driver crate also contains alloc-using storage code. This GPS
// path needs no heap: fail visibly if that assumption ever changes.
struct NoHeap;
unsafe impl core::alloc::GlobalAlloc for NoHeap {
    unsafe fn alloc(&self, _: core::alloc::Layout) -> *mut u8 {
        panic!("L4 GPS unexpected heap allocation");
    }
    unsafe fn dealloc(&self, _: *mut u8, _: core::alloc::Layout) {
        panic!("L4 GPS unexpected heap deallocation");
    }
}
#[global_allocator]
static ALLOCATOR: NoHeap = NoHeap;

bind_interrupts!(struct Irqs {
    USART2 => UartIrq;
    EXTI9_5 => ExtiHandler<embassy_stm32::interrupt::typelevel::EXTI9_5>;
});

static UART_ERROR_IRQS: AtomicU32 = AtomicU32::new(0);
struct UartIrq;
impl embassy_stm32::interrupt::typelevel::Handler<embassy_stm32::interrupt::typelevel::USART2>
    for UartIrq
{
    unsafe fn on_interrupt() {
        let status = pac::USART2.isr().read();
        if status.pe() || status.fe() || status.ne() || status.ore() {
            UART_ERROR_IRQS.fetch_add(1, Ordering::Relaxed);
        }
        <usart::BufferedInterruptHandler<embassy_stm32::peripherals::USART2> as
            embassy_stm32::interrupt::typelevel::Handler<embassy_stm32::interrupt::typelevel::USART2>>::on_interrupt();
    }
}

// SAFETY: the USART2 vector above calls UartIrq, which always delegates to
// this exact Embassy handler after its read-only status observation.
unsafe impl
    embassy_stm32::interrupt::typelevel::Binding<
        embassy_stm32::interrupt::typelevel::USART2,
        usart::BufferedInterruptHandler<embassy_stm32::peripherals::USART2>,
    > for Irqs
{
}

static PPS: Mutex<RefCell<PpsCheck>> = Mutex::new(RefCell::new(PpsCheck {
    edges: 0,
    streak: 0,
    bad_intervals: 0,
    last_us: None,
    minimum_us: None,
    maximum_us: None,
}));

#[embassy_executor::task]
async fn pps_task(mut pin: ExtiInput<'static, Async>, mut blue: Output<'static>) -> ! {
    loop {
        pin.wait_for_rising_edge().await;
        let now = Instant::now().as_micros();
        let (snapshot, delta) = critical_section::with(|cs| {
            let mut pps = PPS.borrow(cs).borrow_mut();
            let delta = pps.edge(now);
            (*pps, delta)
        });
        // Diagnostic indication only. Green remains reserved for TimeService.
        blue.toggle();
        if snapshot.edges <= 3 || snapshot.edges % 10 == 0 || snapshot.streak == 0 {
            info!("L4 GPS PPS edge={} mono_us={} delta_us={:?} streak={} bad={} min_us={:?} max_us={:?}",
                snapshot.edges, now, delta, snapshot.streak, snapshot.bad_intervals,
                snapshot.minimum_us, snapshot.maximum_us);
        }
    }
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
    let _green = Output::new(p.PC3, Level::High, Speed::Low);
    let blue = Output::new(p.PC1, Level::High, Speed::Low);
    // PB4's harness-specific enable polarity is unverified. Do not drive or cycle it.
    let enable = Input::new(p.PB4, Pull::None);
    let pps = ExtiInput::new(p.PB5, p.EXTI5, Pull::Down, Irqs);
    info!("L4 GPS boot board=CaracalEvo receiver_label=Quescan_G10A-F30 uart=USART2 tx=PA2 rx=PA3 pps=PB5 enable=PB4_input enable_high={} initial_pps_high={} search_s=900",
        enable.is_high(), pps.is_high());
    let clocks = rcc::clocks(&p.RCC);
    assert!(clocks.hse.to_hertz().unwrap().0 == 16_000_000);
    assert!(clocks.sys.to_hertz().unwrap().0 == 80_000_000);
    assert!(pac::RCC.cr().read().hserdy() && pac::RCC.cr().read().pllrdy());
    assert!(pac::TIM5.psc().read() == 79);
    spawner.spawn(pps_task(pps, blue).unwrap());

    let mut tx_buffer = [0u8; 256];
    let mut rx_buffer = [0u8; 4096];
    let mut serial_config = usart::Config::default();
    serial_config.baudrate = 9600;
    serial_config.eager_reads = Some(1);
    let mut uart = BufferedUart::new(
        p.USART2,
        p.PA3,
        p.PA2,
        &mut tx_buffer,
        &mut rx_buffer,
        Irqs,
        serial_config,
    )
    .unwrap();
    let started = Instant::now();
    let mut framer = Framer::default();
    let mut bytes = [0u8; 256];
    let mut baud = None;
    // Listen only: no incorrectly clocked configuration commands sent to the receiver.
    'scan: for candidate in [9600, 38400, 115200, 19200, 57600, 4800, 230400] {
        uart.set_baudrate(candidate).unwrap();
        framer.reset();
        let until = Instant::now() + Duration::from_secs(4);
        let mut valid = 0u32;
        let mut received = 0usize;
        info!("L4 GPS baud scan baud={}", candidate);
        while Instant::now() < until {
            if let Ok(Ok(n)) = with_timeout(Duration::from_millis(200), uart.read(&mut bytes)).await
            {
                received += n;
                for b in &bytes[..n] {
                    if framer.push(*b).is_some() {
                        valid += 1;
                    }
                }
                if valid >= 2 {
                    baud = Some(candidate);
                    break 'scan;
                }
            }
        }
        info!(
            "L4 GPS baud scan complete baud={} bytes={} valid_frames={}",
            candidate, received, valid
        );
    }
    let baud = baud.expect("L4 GPS FAIL no checksum-valid UART stream; inspect power/harness/baud");
    let scan_error_irqs = UART_ERROR_IRQS.swap(0, Ordering::Relaxed);
    info!(
        "L4 GPS UART PASS baud={} data=8N1 scan_error_irqs={} (receiver baud unchanged)",
        baud, scan_error_irqs
    );
    framer.reset();
    let mut nmea = NmeaParser::new();
    let mut total_bytes = 0u32;
    let mut nmea_frames = 0u32;
    let mut ubx_frames = 0u32;
    let mut parse_errors = 0u32;
    let mut identity = false;
    let mut pubx_position = false;
    let mut pubx_time = false;
    let mut pvt: Option<(Pvt, Instant)> = None;
    let mut pvt_streak = 0u32;
    let mut previous_tow = None;
    let mut gga: Option<(UtcTime, Instant)> = None;
    let mut rmc: Option<(UtcTime, Instant)> = None;
    let mut gga_streak = 0u32;
    let mut first_fix_ms = None;
    let mut next_poll = Instant::now();
    let mut next_report = Instant::now();
    let mut last_rx = Instant::now();
    let mut poll_count = 0u32;
    let mut passed = false;
    loop {
        let now = Instant::now();
        if now >= next_poll {
            if !identity {
                send(&mut uart, &poll(0x0a, 0x04)).await;
            }
            send(&mut uart, &poll(1, 7)).await; // UBX-NAV-PVT, no configuration mutation.
            if poll_count % 5 == 0 {
                send(&mut uart, b"$PUBX,00*33\r\n").await;
                send(&mut uart, b"$PUBX,04*37\r\n").await;
            }
            poll_count += 1;
            next_poll = Instant::now() + Duration::from_secs(2);
        }
        if let Ok(Ok(n)) = with_timeout(Duration::from_millis(100), uart.read(&mut bytes)).await {
            // A truncated binary frame must not consume subsequent messages indefinitely.
            if (Instant::now() - last_rx).as_millis() > 500 {
                framer.reset();
            }
            last_rx = Instant::now();
            total_bytes += n as u32;
            for byte in &bytes[..n] {
                match framer.push(*byte) {
                    Some(Frame::Ubx { class, id, payload }) => {
                        ubx_frames += 1;
                        if class == 0x0a && id == 4 {
                            if let Some(v) = version(payload) {
                                info!("L4 GPS MON-VER sw={} hw={}", v.software, v.hardware);
                                for field in v.extensions.chunks_exact(30) {
                                    if let Some(s) = ascii_field(field) {
                                        info!("L4 GPS MON-VER extension={}", s);
                                    }
                                }
                                identity = true;
                            } else {
                                parse_errors += 1;
                            }
                        } else if class == 1 && id == 7 {
                            if let Some(nav) = nav_pvt(payload) {
                                let good = nav.qualified();
                                if !good {
                                    pvt_streak = 0;
                                } else if previous_tow != Some(nav.tow_ms) {
                                    // A long discontinuity cannot preserve a qualification streak.
                                    if pvt
                                        .is_some_and(|(_, at)| (Instant::now() - at).as_secs() > 5)
                                    {
                                        pvt_streak = 0;
                                    }
                                    pvt_streak += 1;
                                    if first_fix_ms.is_none() {
                                        first_fix_ms = Some((Instant::now() - started).as_millis());
                                    }
                                }
                                previous_tow = Some(nav.tow_ms);
                                info!("L4 GPS NAV-PVT tow_ms={} utc={}-{}-{}T{}:{}:{} valid={:02x} fix_type={} flags={:02x} sv={} hacc_mm={} pdop_centi={} qualified={} streak={}",
                                    nav.tow_ms, nav.year, nav.month, nav.day, nav.hour, nav.minute, nav.second,
                                    nav.valid, nav.fix_type, nav.flags, nav.satellites, nav.horizontal_accuracy_mm,
                                    nav.position_dop_centi, good, pvt_streak);
                                pvt = Some((nav, Instant::now()));
                            } else {
                                parse_errors += 1;
                            }
                        } else if class == 5 && id == 0 {
                            warn!("L4 GPS UBX NAK payload={=[u8]:x}", payload);
                        }
                    }
                    Some(Frame::Nmea(sentence)) => {
                        nmea_frames += 1;
                        let body = core::str::from_utf8(sentence).unwrap(); // Framer checked printable ASCII.
                        if body.starts_with("$PUBX,00,") {
                            if !pubx_position {
                                info!("L4 GPS PUBX position={}", body.trim_end());
                            }
                            pubx_position = true;
                        } else if body.starts_with("$PUBX,04,") {
                            if !pubx_time {
                                info!("L4 GPS PUBX time={}", body.trim_end());
                            }
                            pubx_time = true;
                        }
                        // Reuse upstream's portable NMEA interpretation after local checksum validation.
                        match nmea.parse(sentence) {
                            Ok(Some(NavigationEvent::Fix(fix))) => {
                                let at = Instant::now();
                                if fix.hdop_centi.is_some() {
                                    let good = fix.satellites >= 4
                                        && fix.hdop_centi.is_some_and(|h| h <= 500);
                                    if good {
                                        if gga.is_none_or(|(t, old)| {
                                            t != fix.utc_time.time || (at - old).as_secs() > 3
                                        }) {
                                            if gga.is_some_and(|(_, old)| (at - old).as_secs() > 3)
                                            {
                                                gga_streak = 0;
                                            }
                                            gga_streak += 1;
                                        }
                                        gga = Some((fix.utc_time.time, at));
                                    } else {
                                        gga = None;
                                        gga_streak = 0;
                                    }
                                    if gga_streak <= 3 || nmea_frames % 50 == 0 {
                                        info!("L4 GPS GGA utc={:?} sv={} hdop_centi={:?} accepted={} streak={}", fix.utc_time.time, fix.satellites, fix.hdop_centi, good, gga_streak);
                                    }
                                } else if fix.utc_time.date.is_some() {
                                    rmc = Some((fix.utc_time.time, at));
                                    if gga_streak <= 3 {
                                        info!("L4 GPS RMC utc={:?} status=A", fix.utc_time);
                                    }
                                }
                            }
                            Ok(Some(NavigationEvent::FixStatus { valid: false, .. })) => {
                                // Identify GGA versus RMC; their independent freshness is checked below.
                                if body.as_bytes().get(3..6) == Some(b"GGA") {
                                    gga = None;
                                    gga_streak = 0;
                                }
                                if body.as_bytes().get(3..6) == Some(b"RMC") {
                                    rmc = None;
                                }
                            }
                            Err(_) => {
                                parse_errors += 1;
                            } // Cold-start empty fields may be unparseable upstream.
                            _ => {}
                        }
                    }
                    None => {}
                }
            }
        }
        let now = Instant::now();
        let pps = critical_section::with(|cs| *PPS.borrow(cs).borrow());
        let fixes_ok = pvt.is_some_and(|(nav, at)| {
            let utc = UtcTime {
                hour: nav.hour,
                minute: nav.minute,
                second: nav.second,
            };
            nav.qualified()
                && pvt_streak >= 3
                && (now - at).as_millis() < 3000
                && gga_streak >= 3
                && gga.is_some_and(|(time, at)| time == utc && (now - at).as_millis() < 3000)
                && rmc.is_some_and(|(time, at)| time == utc && (now - at).as_millis() < 3000)
        });
        if !passed
            && identity
            && pubx_position
            && pubx_time
            && fixes_ok
            && pps.qualified(now.as_micros())
        {
            assert!(
                UART_ERROR_IRQS.load(Ordering::Relaxed) == 0,
                "L4 GPS FAIL UART errors after baud lock"
            );
            info!("L4 GPS PASS baud={} identity={} pubx00={} pubx04={} pvt_fixes={} gga_fixes={} pps_edges={} pps_streak={} first_fix_ms={:?} elapsed_ms={} (receiver UTC; no TimeService anchor)",
                baud, identity, pubx_position, pubx_time, pvt_streak, gga_streak, pps.edges, pps.streak,
                first_fix_ms, (now - started).as_millis());
            passed = true;
        }
        if now >= next_report {
            let pvt_valid =
                pvt.is_some_and(|(nav, at)| nav.qualified() && (now - at).as_millis() < 3000);
            info!("L4 GPS status elapsed_s={} bytes={} nmea={} ubx={} rejected={} parse_errors={} uart_error_irqs={} identity={} pubx00={} pubx04={} pvt_valid={} epoch_match_now={} pps_edges={} pps_streak={} passed={}",
                (now - started).as_secs(), total_bytes, nmea_frames, ubx_frames, framer.rejected,
                parse_errors, UART_ERROR_IRQS.load(Ordering::Relaxed), identity, pubx_position, pubx_time, pvt_valid, fixes_ok, pps.edges, pps.streak, passed);
            next_report = now + Duration::from_secs(10);
        }
        assert!(
            passed || (now - started).as_secs() < 900,
            "L4 GPS FAIL acquisition deadline (receiver left powered)"
        );
    }
}
