//! Autodetected u-blox transport. Shared GpsDriver owns duty cycling.
use crate::{gnss_diagnostics, streams, GPS, LOCATION, TIME};
use aardwolf_l4::gnss_profile::{self, Receiver};
use caracal_evo::capture;
use core::convert::Infallible;
use core::sync::atomic::{AtomicBool, AtomicU32, Ordering::Relaxed};
use core::{cell::RefCell, fmt::Write as _};
use embassy_stm32::{
    bind_interrupts,
    usart::{self, BufferedUart},
};
use embassy_time::{with_timeout, Duration, Instant, Timer};
use embedded_io_async::{ErrorType, Read, Write};
use raylar_drivers::gps::{
    self,
    ublox::{self, Frame, Framer},
};

pub static RX_BYTES: AtomicU32 = AtomicU32::new(0);
pub static UART_ERRORS: AtomicU32 = AtomicU32::new(0);
pub static FRAME_ERRORS: AtomicU32 = AtomicU32::new(0);
pub static VERSIONS: AtomicU32 = AtomicU32::new(0);
pub static PVT_FIXES: AtomicU32 = AtomicU32::new(0);
pub static SLEEPS: AtomicU32 = AtomicU32::new(0);
pub static WAKES: AtomicU32 = AtomicU32::new(0);
pub static ASLEEP: AtomicBool = AtomicBool::new(false);
pub static PROFILE_VALID: AtomicBool = AtomicBool::new(false);
pub static DETECTED_BAUD: AtomicU32 = AtomicU32::new(0);
pub static BENCH_FINISH: AtomicBool = AtomicBool::new(false);
pub static VERSION: embassy_sync::blocking_mutex::Mutex<
    embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex,
    RefCell<heapless::String<64>>,
> = embassy_sync::blocking_mutex::Mutex::new(RefCell::new(heapless::String::new()));
bind_interrupts!(pub struct Irqs {
    USART2 => usart::BufferedInterruptHandler<embassy_stm32::peripherals::USART2>;
    TIM3 => capture::Handler;
});

/// The rail is externally supplied; PB4's harness behavior is unverified.
/// Module commands implement standby. Never drive EN or assert hardware reset.
pub struct RetainedPower;
impl gps::GpsPowerControl for RetainedPower {
    type Error = Infallible;
    fn set_enabled(&mut self, enabled: bool) -> Result<(), Infallible> {
        defmt::assert!(enabled);
        Ok(())
    }
    fn set_reset_asserted(&mut self, asserted: bool) -> Result<(), Infallible> {
        defmt::assert!(!asserted);
        Ok(())
    }
}
pub struct Pps {
    pub _chain: capture::Chain,
}
impl gps::PpsSource for Pps {
    type Error = Infallible;
    async fn wait_for_pps(&mut self) -> Result<gps::PpsCapture, Infallible> {
        let edge = capture::EDGES.receive().await;
        defmt::assert!(capture::FAULTS.load(Relaxed) == 0 && edge.age_us < 10_000);
        Ok(gps::PpsCapture {
            timing_source: gps::PpsTimingSource::ChainedTimerCapture,
            timestamp: edge.timestamp,
            capture_ticks: Some(edge.ticks),
            capture_frequency_hz: Some(1_000_000),
        })
    }
}

pub struct Serial {
    uart: BufferedUart<'static>,
    framer: Framer,
    raw: [u8; 64],
    position: usize,
    length: usize,
    nmea: [u8; 128],
    nmea_pos: usize,
    nmea_len: usize,
    next_poll: Instant,
    next_identity_poll: Instant,
    wake_versions: u32,
    fresh_fix: Option<Instant>,
    ack: bool,
    nak: bool,
    ram: bool,
    bbr: bool,
    receiver: Option<Receiver>,
    next_diagnostics: Instant,
    diagnostics_until: Instant,
    config_poll: usize,
}
impl Serial {
    pub fn new(uart: BufferedUart<'static>) -> Self {
        Self {
            uart,
            framer: Framer::default(),
            raw: [0; 64],
            position: 0,
            length: 0,
            nmea: [0; 128],
            nmea_pos: 0,
            nmea_len: 0,
            next_poll: Instant::now(),
            next_identity_poll: Instant::now(),
            wake_versions: 0,
            fresh_fix: None,
            ack: false,
            nak: false,
            ram: false,
            bbr: false,
            receiver: None,
            next_diagnostics: Instant::now(),
            diagnostics_until: Instant::now(),
            config_poll: 0,
        }
    }
    async fn send(&mut self, bytes: &[u8]) -> Result<(), usart::Error> {
        self.uart.write_all(bytes).await?;
        self.uart.flush().await
    }
    /// Initialization preserves the running navigation engine and its BBR.
    pub async fn initialize(&mut self) -> Receiver {
        // Only wake and identity polls precede detection. Two bounded sweeps
        // tolerate a receiver still booting during the first visit to its baud.
        let mut detected_baud = None;
        'scan: for sweep in 1..=2 {
            for baud in gnss_profile::BAUD_RATES {
                defmt::info!("AARDWOLF L4 GNSS scan sweep={} baud={}", sweep, baud);
                self.uart.set_baudrate(baud).unwrap();
                self.reset_rx();
                self.send(&ublox::WAKE).await.unwrap();
                Timer::after_millis(500).await;
                self.next_identity_poll = Instant::now();
                let deadline = Instant::now() + Duration::from_secs(2);
                while VERSIONS.load(Relaxed) == 0 && Instant::now() < deadline {
                    if Instant::now() >= self.next_identity_poll {
                        self.send(&ublox::MON_VER).await.unwrap();
                        self.next_identity_poll = Instant::now() + Duration::from_millis(500);
                    }
                    let _ = with_timeout(Duration::from_millis(100), self.pump()).await;
                }
                if VERSIONS.load(Relaxed) > 0 {
                    detected_baud = Some(baud);
                    break 'scan;
                }
            }
        }
        defmt::assert!(detected_baud.is_some(), "GNSS baud/identity scan failed");
        defmt::assert!(self.receiver.is_some(), "Unsupported GNSS MON-VER identity");
        let receiver = self.receiver.unwrap();
        DETECTED_BAUD.store(detected_baud.unwrap(), Relaxed);
        defmt::info!(
            "AARDWOLF L4 GNSS DETECT receiver={} baud={}",
            receiver.name(),
            detected_baud.unwrap()
        );
        // Change at the discovered baud, then verify at the operational baud.
        // A baud-change ACK can arrive at either rate; readback is the proof.
        self.send(if receiver == Receiver::M10 {
            &ublox::UART_PROFILE
        } else {
            &gnss_profile::LEGACY_UART
        })
        .await
        .unwrap();
        Timer::after_millis(100).await;
        self.uart.set_baudrate(38400).unwrap();
        self.reset_rx();
        self.ram = false;
        self.bbr = false;
        if receiver != Receiver::M10 {
            self.until(2).await;
            defmt::info!("AARDWOLF L4 GNSS PROFILE PASS receiver={} baud=38400 CFG-PRT readback=true flash=false", receiver.name());
        } else {
            for layer in [0, 1] {
                self.until(2 + layer).await;
            }
            defmt::assert!(
                self.ram && self.bbr && !self.nak,
                "M10 UART profile readback failed"
            );
            defmt::info!("AARDWOLF L4 GNSS PROFILE PASS receiver=M10 baud=38400 ram=true bbr=true flash=false");
        }
        PROFILE_VALID.store(true, Relaxed);
        receiver
    }
    fn reset_rx(&mut self) {
        self.framer.reset();
        self.position = 0;
        self.length = 0;
        self.fresh_fix = None;
        self.nmea_len = 0;
        self.nmea_pos = 0;
    }
    async fn until(&mut self, condition: u8) {
        let deadline = Instant::now() + Duration::from_secs(5);
        let mut next_request = Instant::now();
        loop {
            let done = match condition {
                2 => self.ram,
                _ => self.bbr,
            };
            if done {
                return;
            }
            defmt::assert!(
                Instant::now() < deadline,
                "GNSS UART profile readback timeout"
            );
            if Instant::now() >= next_request {
                if self.receiver == Some(Receiver::M10) {
                    self.send(&ublox::uart_profile_poll(condition - 2))
                        .await
                        .unwrap();
                } else {
                    self.send(&gnss_profile::LEGACY_UART_POLL).await.unwrap();
                }
                next_request = Instant::now() + Duration::from_millis(500);
            }
            // Read keeps all framing state in self, including across cancellation.
            let _ = with_timeout(Duration::from_millis(200), self.pump()).await;
        }
    }
    async fn pump(&mut self) -> Result<(), usart::Error> {
        if self.position == self.length {
            match self.uart.read(&mut self.raw).await {
                Ok(n) => {
                    self.position = 0;
                    self.length = n;
                    RX_BYTES.fetch_add(n as u32, Relaxed);
                }
                Err(e) => {
                    UART_ERRORS.fetch_add(1, Relaxed);
                    self.framer.reset();
                    return Err(e);
                }
            }
        }
        while self.position < self.length {
            let b = self.raw[self.position];
            self.position += 1;
            if let Some(frame) = self.framer.push(b) {
                if let Frame::Ubx { class, id, payload } = &frame {
                    gnss_diagnostics::frame(*class, *id, payload);
                }
                match frame {
                    Frame::Nmea(bytes) => {
                        if Instant::now() < self.diagnostics_until
                            && bytes.len() >= 8
                            && (&bytes[3..6] == b"GSV" || &bytes[3..6] == b"GSA")
                        {
                            if let Ok(text) = core::str::from_utf8(&bytes[..bytes.len() - 2]) {
                                defmt::info!("GNSS DIAG NMEA {}", text);
                            }
                        }
                        // Only feed the shared parser after fully resolved UBX UTC/fix.
                        if !ASLEEP.load(Relaxed)
                            && self
                                .fresh_fix
                                .is_some_and(|t| Instant::now() - t < Duration::from_secs(3))
                            && bytes.len() <= self.nmea.len()
                        {
                            self.nmea[..bytes.len()].copy_from_slice(bytes);
                            self.nmea_pos = 0;
                            self.nmea_len = bytes.len();
                            return Ok(());
                        }
                    }
                    Frame::Ubx {
                        class: 10,
                        id: 4,
                        payload,
                    } => {
                        if let Some(v) = ublox::version(payload) {
                            let receiver = Receiver::detect(payload);
                            if let Some(previous) = self.receiver {
                                defmt::assert!(
                                    receiver == Some(previous),
                                    "GNSS identity changed; restart after replacing receiver"
                                );
                            }
                            self.receiver = receiver;
                            VERSION.lock(|s| {
                                let mut text = s.borrow_mut();
                                text.clear();
                                let _ = write!(
                                    text,
                                    "{} sw={} hw={}",
                                    receiver.map_or("Unknown", Receiver::name),
                                    v.software,
                                    v.hardware
                                );
                            });
                            defmt::info!(
                                "AARDWOLF L4 GNSS version sw={} hw={}",
                                v.software,
                                v.hardware
                            );
                            for ext in v.extensions.chunks_exact(30) {
                                if let Some(s) = ublox::ascii_field(ext) {
                                    defmt::info!("AARDWOLF L4 GNSS extension {}", s);
                                }
                            }
                            VERSIONS.fetch_add(1, Relaxed);
                        }
                    }
                    Frame::Ubx {
                        class: 1,
                        id: 7,
                        payload,
                    } => {
                        if let Some(p) = self.receiver.and_then(|r| r.nav_pvt(payload)) {
                            if Instant::now() < self.diagnostics_until {
                                defmt::info!("GNSS DIAG PVT tow_ms={} fix={} satellites={} valid={:02x} flags={:02x} year={} month={} day={} hour={} minute={} second={} qualified={} pdop_centi={} horizontal_accuracy_mm={}",p.tow_ms,p.fix_type,p.satellites,p.valid,p.flags,p.year,p.month,p.day,p.hour,p.minute,p.second,p.qualified(),p.position_dop_centi,p.horizontal_accuracy_mm);
                            }
                            if p.qualified() {
                                self.fresh_fix = Some(Instant::now());
                                PVT_FIXES.fetch_add(1, Relaxed);
                            } else {
                                self.fresh_fix = None;
                            }
                        }
                    }
                    Frame::Ubx {
                        class: 6,
                        id: 0,
                        payload,
                    } if self.receiver != Some(Receiver::M10) => {
                        self.ram = gnss_profile::legacy_uart_matches(payload);
                    }
                    Frame::Ubx {
                        class: 5,
                        id,
                        payload,
                    } if payload == [6, 0x8a] => {
                        if id == 1 {
                            self.ack = true;
                        } else if id == 0 {
                            self.nak = true;
                        }
                    }
                    Frame::Ubx {
                        class: 6,
                        id: 0x8b,
                        payload,
                    } => {
                        if ublox::uart_profile_matches(payload, 0) {
                            self.ram = true;
                        }
                        if ublox::uart_profile_matches(payload, 1) {
                            self.bbr = true;
                        }
                    }
                    _ => {}
                }
            }
            FRAME_ERRORS.store(self.framer.rejected, Relaxed);
        }
        Ok(())
    }
}
impl ErrorType for Serial {
    type Error = usart::Error;
}
impl Read for Serial {
    async fn read(&mut self, out: &mut [u8]) -> Result<usize, Self::Error> {
        if out.is_empty() {
            return Ok(0);
        }
        loop {
            if self.nmea_pos < self.nmea_len {
                let n = out.len().min(self.nmea_len - self.nmea_pos);
                out[..n].copy_from_slice(&self.nmea[self.nmea_pos..self.nmea_pos + n]);
                self.nmea_pos += n;
                return Ok(n);
            }
            if !ASLEEP.load(Relaxed) && Instant::now() >= self.next_poll {
                self.send(&ublox::NAV_PVT).await?;
                if self.receiver != Some(Receiver::M10)
                    && self.config_poll < gnss_diagnostics::CONFIG_POLLS.len()
                {
                    self.send(gnss_diagnostics::CONFIG_POLLS[self.config_poll])
                        .await?;
                    self.config_poll += 1;
                }
                if Instant::now() >= self.next_diagnostics {
                    self.diagnostics_until = Instant::now() + Duration::from_secs(2);
                    self.next_diagnostics = Instant::now() + Duration::from_secs(30);
                    self.send(&gnss_diagnostics::STATUS).await?;
                    if self.receiver != Some(Receiver::M10) {
                        self.send(&gnss_diagnostics::SATELLITES).await?;
                        self.send(&gnss_diagnostics::HARDWARE).await?;
                        self.send(&gnss_diagnostics::DOP).await?;
                        self.send(&gnss_diagnostics::GPS_TIME).await?;
                        self.send(&gnss_diagnostics::UTC_TIME).await?;
                        self.send(&gnss_diagnostics::AIDING).await?;
                    }
                }
                self.next_poll = Instant::now() + Duration::from_secs(1);
            }
            if !ASLEEP.load(Relaxed)
                && VERSIONS.load(Relaxed) <= self.wake_versions
                && Instant::now() >= self.next_identity_poll
            {
                self.send(&ublox::MON_VER).await?;
                self.next_identity_poll = Instant::now() + Duration::from_secs(1);
            }
            self.pump().await?;
        }
    }
}
impl Write for Serial {
    async fn write(&mut self, bytes: &[u8]) -> Result<usize, Self::Error> {
        if self.receiver.is_some_and(|r| bytes == r.standby()) {
            defmt::assert!(PROFILE_VALID.load(Relaxed));
            self.send(bytes).await?;
            ASLEEP.store(true, Relaxed);
            self.fresh_fix = None;
            self.nmea_len = 0;
            self.nmea_pos = 0;
            SLEEPS.fetch_add(1, Relaxed);
            defmt::info!(
                "AARDWOLF L4 GNSS standby sent duration=indefinite wake=UART_RX rail=retained"
            );
        } else if bytes == ublox::WAKE {
            // End the silence interval before the first UART wake transition.
            ASLEEP.store(false, Relaxed);
            self.wake_versions = VERSIONS.load(Relaxed);
            self.send(bytes).await?;
            Timer::after_millis(500).await;
            self.fresh_fix = None;
            self.framer.reset();
            self.nmea_len = 0;
            self.nmea_pos = 0;
            self.send(&ublox::MON_VER).await?;
            self.next_identity_poll = Instant::now() + Duration::from_secs(1);
            self.next_poll = Instant::now();
            self.next_diagnostics = Instant::now();
            self.config_poll = 0;
            WAKES.fetch_add(1, Relaxed);
            defmt::info!("AARDWOLF L4 GNSS UART wake sent reset=false aiding=retained");
        } else {
            self.send(bytes).await?;
        }
        Ok(bytes.len())
    }
    async fn flush(&mut self) -> Result<(), Self::Error> {
        self.uart.flush().await
    }
}

#[embassy_executor::task]
pub async fn time_task() {
    raylar_time_service::TimeService::new(&TIME, Default::default())
        .run()
        .await;
}
#[embassy_executor::task]
pub async fn anchors_task() {
    raylar_time_service::gps::run_gps_time_source(
        GPS.time_receiver().unwrap().as_dyn(),
        TIME.anchor_sender(),
    )
    .await;
}
#[embassy_executor::task]
pub async fn location_task() {
    raylar_location_service::LocationService::<2, 9>::new(
        &LOCATION,
        GPS.fix_receiver().unwrap().as_dyn(),
        Default::default(),
    )
    .run()
    .await;
}
#[embassy_executor::task]
pub async fn driver_task(
    mut serial: Serial,
    pps: Pps,
    enable: embassy_stm32::gpio::Input<'static>,
) {
    let _enable = enable; // Pin remains input throughout all states.
    let receiver = serial.initialize().await;
    let config = aardwolf_l4::config::gps(receiver);
    gps::GpsDriver::new(serial, pps, RetainedPower, &GPS, config)
        .run()
        .await;
}

/// Same calibration/phase feedback contract as U5 common time composition.
#[embassy_executor::task]
pub async fn quality_task() {
    use gps::{GpsCommand, OperatingState};
    let mut watch = TIME.state_receiver().unwrap();
    let mut locked = false;
    let mut observed_accepted = 0;
    let mut observed_rejected = 0;
    loop {
        let state = watch.changed().await;
        if state.frequency_calibration_locked && !locked {
            GPS.command_sender()
                .send(GpsCommand::FrequencyCalibrationLocked)
                .await;
            locked = true;
            defmt::info!(
                "AARDWOLF L4 CALIBRATION LOCK samples={} correction_ppb={}",
                state.frequency_calibration_samples,
                state.calibrated_frequency_error_ppb
            );
        }
        let accepted = state.accepted_anchors != observed_accepted;
        let rejected = state.rejected_anchors != observed_rejected;
        let observation = u64::from(state.accepted_anchors) + u64::from(state.rejected_anchors);
        if locked
            && (accepted || rejected)
            && GPS.stats().operating_state == OperatingState::Acquired
        {
            GPS.command_sender()
                .send(GpsCommand::PhaseQuality {
                    observation_sequence: observation,
                    accepted: accepted && !rejected,
                    residual_us: state.last_anchor_residual_us,
                    uncertainty_us: state.uncertainty_us,
                    pps_gate_active: state.pps_reacquisition_active,
                })
                .await;
        }
        observed_accepted = state.accepted_anchors;
        observed_rejected = state.rejected_anchors;
    }
}

/// Observe physical silence and resumed anchors; manager state alone is not proof.
pub struct CycleCheck {
    previous_sleep: bool,
    since: Instant,
    quiet: Option<((u32, u32), Instant)>,
    slept: bool,
    quiet_sleeps: u32,
    sleep_versions: u32,
    anchors_at_wake: u32,
    pub verified: bool,
}
impl CycleCheck {
    pub fn new() -> Self {
        Self {
            previous_sleep: false,
            since: Instant::now(),
            quiet: None,
            slept: false,
            quiet_sleeps: 0,
            sleep_versions: 0,
            anchors_at_wake: 0,
            verified: false,
        }
    }
    pub fn update(&mut self) {
        let sleeping = ASLEEP.load(Relaxed);
        if sleeping != self.previous_sleep {
            self.since = Instant::now();
            self.quiet = None;
            if sleeping {
                self.sleep_versions = VERSIONS.load(Relaxed);
            }
            if !sleeping && self.slept {
                self.anchors_at_wake = TIME.time_state().accepted_anchors;
            }
            self.previous_sleep = sleeping;
        }
        let age = Instant::now() - self.since;
        let counters = (RX_BYTES.load(Relaxed), capture::IRQS.load(Relaxed));
        if sleeping && age >= Duration::from_secs(3) {
            if let Some((previous, quiet_since)) = self.quiet {
                defmt::assert!(
                    previous == counters,
                    "GNSS failed standby: UART or PPS still active"
                );
                let sleeps = SLEEPS.load(Relaxed);
                if Instant::now() - quiet_since >= Duration::from_secs(7)
                    && self.quiet_sleeps != sleeps
                {
                    self.slept = true;
                    self.quiet_sleeps = sleeps;
                    streams::log(format_args!(
                        "gps_standby_verified uart_quiet=true pps_quiet=true rails=retained"
                    ));
                    defmt::info!("AARDWOLF L4 GNSS STANDBY PASS uart_quiet=true pps_quiet=true");
                    if cfg!(feature = "bench-gps-cycle")
                        && self.verified
                        && GPS.stats().num_phase_qualified_shutdowns > 0
                    {
                        BENCH_FINISH.store(true, Relaxed);
                    }
                }
            } else {
                self.quiet = Some((counters, Instant::now()));
            }
        }
        let time = TIME.time_state();
        if !sleeping
            && self.slept
            && !self.verified
            && time.accepted_anchors >= self.anchors_at_wake + 20
            && !time.pps_reacquisition_active
            && GPS.stats().num_reacquisition_successes > 0
            && VERSIONS.load(Relaxed) > self.sleep_versions
        {
            self.verified = true;
            streams::log(format_args!(
                "gps_wake_verified fixes={} accepted={} calibrated_ppb={} reset=false",
                GPS.stats().num_reacquisition_successes,
                time.accepted_anchors,
                time.calibrated_frequency_error_ppb
            ));
            defmt::info!("AARDWOLF L4 GNSS WAKE PASS elapsed_ms={} accepted_since_wake={} reacquired={} correction_ppb={}",age.as_millis(),time.accepted_anchors-self.anchors_at_wake,GPS.stats().num_reacquisition_successes,time.calibrated_frequency_error_ppb);
        }
    }
}
