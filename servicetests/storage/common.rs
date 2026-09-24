use defmt::{error, info, unwrap};
use embassy_executor::Spawner;
use embassy_stm32::gpio::Output;
use embassy_stm32::rcc::mux::Sdmmcsel;
use embassy_stm32::rcc::*;
use embassy_stm32::sdmmc::sd::{CmdBlock, StorageDevice};
use embassy_stm32::sdmmc::{Config as SdmmcConfig, Sdmmc};
use embassy_stm32::time::{Hertz, mhz};
use embassy_stm32::usart::{BufferedUart, Config as UartConfig, DataBits, Parity, StopBits};
use embassy_time::{Duration, Timer};
use raylar_board_v1p0::{Gps, Irqs, SdCard};
use raylar_drivers::gps::stm32::{Stm32GpsPower, Stm32Pps};
use raylar_drivers::gps::{
    GpsCommand, GpsConfig, GpsDriver, GpsResources, PhaseQualifiedShutdownConfig, PpsTimingSource,
};
use raylar_drivers::storage::stm32::Stm32SdBlockDevice;
use raylar_drivers::storage::{
    FileHandle, PartitionedBlockDevice, StorageDeviceIdentity, StorageDriver, detect_exfat_volume,
};
use raylar_storage_service::StorageBackend;
use raylar_time_service::gps::run_gps_time_source;
use raylar_time_service::{
    Anchor, AnchorQuality, TimeConfig, TimeResources, TimeService, TimeSource, UtcTimestamp,
};
use static_cell::StaticCell;

pub static GPS_RESOURCES: GpsResources = GpsResources::new();
pub static TIME_RESOURCES: TimeResources<4, 8> = TimeResources::new();
const SD_TARGET_FREQ: Hertz = mhz(24);

/// Largest measured HSE error accepted by the startup PLL correction.
///
/// This is deliberately much smaller than the range of the fractional PLL.
/// It prevents a bad configuration value from making a large, unsafe clock
/// change while still covering the expected board-to-board crystal spread.
pub const MAX_HSE_ERROR_PPM: i32 = 30;
const PLL_BASE_N: i32 = 54;
const PLL_FRAC_SCALE: i64 = 8192;
const PPM_SCALE: i64 = 1_000_000;
// The hardware timer clock is nominally 144 MHz, so a 1 MHz counter requires
// PSC=143. embassy-stm32 0.6 models M=3/N=54 as `(16 MHz / 3) * 54`, truncates
// SYSCLK to 143_999_991 Hz, and incorrectly selects PSC=142. Keep the truthful
// 16 MHz HSE model (required by OTG-HS validation) and repair only the affected
// exact-1-MHz timers. See `correct_embassy_time_driver_prescaler` and the TIM4
// PPS capture initialization.
const ONE_MHZ_FROM_144_MHZ_PSC: u16 = 143;
// The ST fractional-latch workaround requires a short pause while FRACEN is
// clear. At the 144 MHz startup SYSCLK, 256 spin-loop iterations are safely
// longer than several PLL reference cycles but still only a few microseconds.
const PLL_FRAC_LATCH_DELAY_ITERATIONS: usize = 256;

/// An invalid measured HSE error was supplied to the boot clock setup.
#[derive(Clone, Copy, Debug, defmt::Format, PartialEq, Eq)]
pub enum PllCorrectionError {
    PpmOutsideSafeRange { requested_ppm: i32 },
}

/// Boot-only plan for applying the same fractional correction to PLL1/PLL3.
///
/// `measured_error_ppm` is signed oscillator error: a negative value means
/// the board clock was measured slow against UTC and therefore needs a
/// positive PLL pull. This value is consumed by `apply` to make its intended
/// one-shot startup use clear.
pub struct PllFrequencyCorrection {
    measured_error_ppm: i32,
    integer_n: u16,
    fracn: u16,
}

impl PllFrequencyCorrection {
    pub fn measured_error_ppm(&self) -> i32 {
        self.measured_error_ppm
    }

    pub fn integer_n(&self) -> u16 {
        self.integer_n
    }

    pub fn fracn(&self) -> u16 {
        self.fracn
    }

    /// Apply PLL1/PLL3 FRACN once, immediately after `embassy_stm32::init`.
    ///
    /// RM0456 requires FRACEN to be cleared before FRACN is changed and set
    /// again afterwards. Readbacks plus short delays implement the additional
    /// settling workaround reported for STM32U5 fractional-PLL latching. A
    /// zero fractional value deliberately leaves fractional mode disabled.
    pub fn apply(self) {
        use embassy_stm32::pac::RCC;

        RCC.pll1cfgr().modify(|w| w.set_pllfracen(false));
        RCC.pll3cfgr().modify(|w| w.set_pllfracen(false));
        let _ = RCC.pll1cfgr().read();
        let _ = RCC.pll3cfgr().read();
        pll_fractional_latch_delay();

        RCC.pll1fracr().write(|w| w.set_pllfracn(self.fracn));
        RCC.pll3fracr().write(|w| w.set_pllfracn(self.fracn));
        let _ = RCC.pll1fracr().read();
        let _ = RCC.pll3fracr().read();
        pll_fractional_latch_delay();

        if self.fracn != 0 {
            RCC.pll1cfgr().modify(|w| w.set_pllfracen(true));
            RCC.pll3cfgr().modify(|w| w.set_pllfracen(true));
            let _ = RCC.pll1cfgr().read();
            let _ = RCC.pll3cfgr().read();
        }

        correct_embassy_time_driver_prescaler();
    }
}

/// Correct TIM5, which is selected as this workspace's Embassy time driver.
///
/// This runs immediately after `embassy_stm32::init`, before application code
/// observes `Instant`. Interrupts are masked while the live counter is stopped,
/// rescaled, and restarted so the one-shot prescaler repair remains monotonic.
fn correct_embassy_time_driver_prescaler() {
    cortex_m::interrupt::free(|_| {
        let regs = embassy_stm32::pac::TIM5;
        let old_psc = regs.psc().read();
        if old_psc == ONE_MHZ_FROM_144_MHZ_PSC {
            return;
        }

        // Limit this workaround to the known Embassy M=3 truncation result.
        // If its clock calculation changes, do not silently rewrite a different
        // timer configuration.
        const EMBASSY_TRUNCATED_PSC: u16 = 142;
        if old_psc != EMBASSY_TRUNCATED_PSC {
            error!(
                "TIM5 1MHz prescaler correction skipped: unexpected PSC={}",
                old_psc
            );
            return;
        }

        let was_enabled = regs.cr1().read().cen();
        regs.cr1().modify(|w| w.set_cen(false));
        let old_counter = regs.cnt().read();
        let corrected_counter = ((old_counter as u64)
            .saturating_mul((EMBASSY_TRUNCATED_PSC as u64) + 1)
            / ((ONE_MHZ_FROM_144_MHZ_PSC as u64) + 1)) as u32;

        regs.psc().write_value(ONE_MHZ_FROM_144_MHZ_PSC);
        regs.egr().write(|w| w.set_ug(true));
        regs.sr().modify(|w| w.set_uif(false));
        regs.cnt().write_value(corrected_counter);
        regs.cr1().modify(|w| w.set_cen(was_enabled));
        info!(
            "Corrected Embassy TIM5 1MHz prescaler: PSC {} -> {}",
            old_psc, ONE_MHZ_FROM_144_MHZ_PSC
        );
    });
}

#[inline(never)]
fn pll_fractional_latch_delay() {
    for _ in 0..PLL_FRAC_LATCH_DELAY_ITERATIONS {
        core::hint::spin_loop();
    }
}

/// Keeps the active-low SD power GPIO configured for as long as the filesystem
/// backend exists. Dropping an Embassy `Output` disconnects the pin, which can
/// remove card power after volume detection but before filesystem mounting.
pub struct PoweredStorage<B> {
    inner: B,
    _power: Output<'static>,
}

pub type BoardStorageBackend =
    PoweredStorage<StorageDriver<PartitionedBlockDevice<Stm32SdBlockDevice<'static, 'static>>>>;

static SDMMC: StaticCell<Sdmmc<'static>> = StaticCell::new();

impl<B, const BLOCK_SIZE: usize> StorageBackend<BLOCK_SIZE> for PoweredStorage<B>
where
    B: StorageBackend<BLOCK_SIZE>,
{
    type Error = B::Error;

    fn device_identity(&self) -> Option<StorageDeviceIdentity> {
        self.inner.device_identity()
    }

    async fn mount(&mut self) -> Result<(), Self::Error> {
        self.inner.mount().await
    }

    async fn create_directory(&mut self, path: &str) -> Result<(), Self::Error> {
        self.inner.create_directory(path).await
    }

    async fn open_for_append(&mut self, path: &str) -> Result<FileHandle, Self::Error> {
        self.inner.open_for_append(path).await
    }

    async fn append(&mut self, handle: FileHandle, data: &[u8]) -> Result<(), Self::Error> {
        self.inner.append(handle, data).await
    }

    async fn flush(&mut self, handle: FileHandle) -> Result<(), Self::Error> {
        self.inner.flush(handle).await
    }

    async fn close(
        &mut self,
        handle: FileHandle,
        valid_bytes_last_block: usize,
    ) -> Result<(), Self::Error> {
        self.inner.close(handle, valid_bytes_last_block).await
    }
}

pub fn mcu_config() -> embassy_stm32::Config {
    let mut config = embassy_stm32::Config::default();
    config.rcc.hse = Some(Hse {
        freq: mhz(16),
        mode: HseMode::Oscillator,
    });
    config.rcc.pll1 = Some(Pll {
        source: PllSource::HSE,
        prediv: PllPreDiv::DIV1,
        mul: PllMul::MUL18,
        divp: Some(PllDiv::DIV6),
        divq: Some(PllDiv::DIV2),
        divr: Some(PllDiv::DIV2),
    });
    // MDF high-performance audio presets use PLL3_Q as their 96 MHz kernel
    // clock. Storage-only callers simply leave this additional clock unused.
    config.rcc.pll3 = Some(Pll {
        source: PllSource::HSE,
        prediv: PllPreDiv::DIV1,
        mul: PllMul::MUL12,
        divp: None,
        divq: Some(PllDiv::DIV2),
        divr: None,
    });
    config.rcc.sys = Sysclk::PLL1_R;
    config.rcc.hsi48 = Some(Hsi48Config::new());
    config.rcc.mux.sdmmcsel = Sdmmcsel::PLL1_P;
    config
}

/// Build integrationtest002's common-ratio PLL clock tree and its boot trim.
///
/// Both PLLs use HSE / 3 and the same N + FRACN/8192 multiplier. Consequently
/// SYSCLK/PPS timing and the audio kernel clock receive exactly the same ppm
/// correction. The output dividers retain PLL1_R=144 MHz, PLL1_P=48 MHz and
/// PLL3_Q=96 MHz at the nominal N=54 setting.
pub fn mcu_config_with_hse_error_ppm(
    measured_error_ppm: i32,
) -> Result<(embassy_stm32::Config, PllFrequencyCorrection), PllCorrectionError> {
    if !(-MAX_HSE_ERROR_PPM..=MAX_HSE_ERROR_PPM).contains(&measured_error_ppm) {
        return Err(PllCorrectionError::PpmOutsideSafeRange {
            requested_ppm: measured_error_ppm,
        });
    }

    // Correct the measured source error using the reciprocal scale factor:
    // multiplier = 54 * 1_000_000 / (1_000_000 + measured_error_ppm).
    // Keep the multiplier in 1/8192 units and round to the nearest FRACN step.
    let numerator = i64::from(PLL_BASE_N) * PLL_FRAC_SCALE * PPM_SCALE;
    let denominator = PPM_SCALE + i64::from(measured_error_ppm);
    let multiplier_units = (numerator + denominator / 2) / denominator;
    let integer_n = (multiplier_units / PLL_FRAC_SCALE) as u16;
    let fracn = (multiplier_units % PLL_FRAC_SCALE) as u16;
    let mul = match integer_n {
        53 => PllMul::MUL53,
        54 => PllMul::MUL54,
        _ => unreachable!("the +/-30 ppm guard only permits PLL N=53 or N=54"),
    };

    let mut config = embassy_stm32::Config::default();
    config.rcc.hse = Some(Hse {
        freq: mhz(16),
        mode: HseMode::Oscillator,
    });
    config.rcc.pll1 = Some(Pll {
        source: PllSource::HSE,
        prediv: PllPreDiv::DIV3,
        mul,
        divp: Some(PllDiv::DIV6),
        divq: Some(PllDiv::DIV2),
        divr: Some(PllDiv::DIV2),
    });
    config.rcc.pll3 = Some(Pll {
        source: PllSource::HSE,
        prediv: PllPreDiv::DIV3,
        mul,
        divp: None,
        divq: Some(PllDiv::DIV3),
        divr: None,
    });
    config.rcc.sys = Sysclk::PLL1_R;
    config.rcc.hsi48 = Some(Hsi48Config::new());
    config.rcc.mux.sdmmcsel = Sdmmcsel::PLL1_P;

    Ok((
        config,
        PllFrequencyCorrection {
            measured_error_ppm,
            integer_n,
            fracn,
        },
    ))
}

pub async fn start_time(spawner: Spawner, gps: Gps<'static>) {
    let defaults = GpsConfig::default();
    start_time_with_duty_cycle(spawner, gps, defaults.gps_on_time, defaults.gps_off_time).await;
}

/// Start GPS/PPS time with a caller-selected post-calibration duty cycle.
/// Initial acquisition still remains continuous until frequency calibration
/// locks; these durations apply only to subsequent tracking and standby.
pub async fn start_time_with_duty_cycle(
    spawner: Spawner,
    gps: Gps<'static>,
    gps_on_time: Duration,
    gps_off_time: Duration,
) {
    start_time_inner(spawner, gps, gps_on_time, gps_off_time, None).await;
}

/// Start GPS/PPS time with phase-qualified post-calibration shutdown.
pub async fn start_time_with_phase_qualified_duty_cycle(
    spawner: Spawner,
    gps: Gps<'static>,
    gps_on_time: Duration,
    gps_off_time: Duration,
    phase_shutdown: PhaseQualifiedShutdownConfig,
) {
    start_time_inner(
        spawner,
        gps,
        gps_on_time,
        gps_off_time,
        Some(phase_shutdown),
    )
    .await;
}

async fn start_time_inner(
    spawner: Spawner,
    gps: Gps<'static>,
    gps_on_time: Duration,
    gps_off_time: Duration,
    phase_qualified_shutdown: Option<PhaseQualifiedShutdownConfig>,
) {
    let Gps {
        usart,
        tx,
        rx,
        pps,
        pps_exti,
        pps_capture_timer,
        rst,
        en,
    } = gps;
    let mut uart_config = UartConfig::default();
    uart_config.baudrate = 9_600;
    uart_config.data_bits = DataBits::DataBits8;
    uart_config.parity = Parity::ParityNone;
    uart_config.stop_bits = StopBits::STOP1;

    static mut TX_BUFFER: [u8; 64] = [0; 64];
    static mut RX_BUFFER: [u8; 512] = [0; 512];
    let tx_buffer = unsafe { &mut *core::ptr::addr_of_mut!(TX_BUFFER) };
    let rx_buffer = unsafe { &mut *core::ptr::addr_of_mut!(RX_BUFFER) };
    let uart = unwrap!(BufferedUart::new(
        usart,
        rx,
        tx,
        tx_buffer,
        rx_buffer,
        Irqs,
        uart_config
    ));

    let gps_config = GpsConfig {
        gps_on_time,
        gps_off_time,
        phase_qualified_shutdown,
        pps_timing_source: PpsTimingSource::Tim4Capture,
        wait_for_frequency_calibration_lock: true,
        ..GpsConfig::default()
    };
    let pps = Stm32Pps::from_config(&gps_config, pps, pps_exti, pps_capture_timer, Irqs, Irqs);
    let driver = GpsDriver::new(
        uart,
        pps,
        Stm32GpsPower::new(en, rst),
        &GPS_RESOURCES,
        gps_config,
    );
    let time = TimeService::new(&TIME_RESOURCES, TimeConfig::default());
    let correlations = unwrap!(GPS_RESOURCES.time_receiver()).as_dyn();
    spawner.spawn(unwrap!(gps_driver_task(driver)));
    spawner.spawn(unwrap!(time_service_task(time)));
    spawner.spawn(unwrap!(gps_time_source_task(correlations)));
    spawner.spawn(unwrap!(gps_frequency_calibration_lock_task()));
    if phase_qualified_shutdown.is_some() {
        spawner.spawn(unwrap!(gps_phase_quality_task()));
    }
    GPS_RESOURCES.command_sender().send(GpsCommand::Start).await;
}

/// Starts the time service from a synthetic UTC anchor without powering GPS.
///
/// This is deliberately separate from `start_time` so production callers
/// cannot accidentally mix laboratory and GPS anchors.
#[allow(dead_code)]
pub async fn start_fake_time(spawner: Spawner, _gps: Gps<'static>, utc_seconds: i64) {
    let time = TimeService::new(&TIME_RESOURCES, TimeConfig::default());
    spawner.spawn(unwrap!(time_service_task(time)));
    TIME_RESOURCES
        .anchor_sender()
        .send(Anchor {
            system_time: embassy_time::Instant::now(),
            utc: UtcTimestamp::new(utc_seconds, 0).expect("whole-second UTC is valid"),
            quality: AnchorQuality::new(1),
            source: TimeSource::Laboratory,
            capture_ticks: None,
            pps_sequence: None,
            pps_interval: None,
        })
        .await;
    info!(
        "TEST ONLY: synthetic UTC anchor injected; source=Laboratory utc_seconds={}",
        utc_seconds
    );
}

pub async fn storage_driver(sd: SdCard<'static>) -> BoardStorageBackend {
    storage_driver_inner(sd, None).await
}

/// Creates the board storage driver and invokes `fatal_handler` before
/// entering the terminal wait used for unrecoverable media failures.
#[allow(dead_code)]
pub async fn storage_driver_with_fatal_handler(
    sd: SdCard<'static>,
    fatal_handler: fn(),
) -> BoardStorageBackend {
    storage_driver_inner(sd, Some(fatal_handler)).await
}

async fn storage_driver_inner(
    sd: SdCard<'static>,
    fatal_handler: Option<fn()>,
) -> BoardStorageBackend {
    let SdCard {
        sdmmc,
        clk,
        cmd,
        d0,
        d1,
        d2,
        d3,
        switch,
        mut power,
    } = sd;
    info!("SD init phase 1: disabling card power and checking card-detect");
    power.set_high();
    if switch.is_high() {
        error!("SD init failed at card-detect: SD_SW is high (no card detected)");
        notify_fatal(fatal_handler);
        pending_forever().await;
    }
    info!("SD init phase 1 complete: card detected (SD_SW low)");

    info!("SD init phase 2: constructing SDMMC 4-bit peripheral");
    let mut config = SdmmcConfig::default();
    config.data_transfer_timeout = 120_000_000;
    let sdmmc = SDMMC.init(Sdmmc::new_4bit(
        sdmmc, Irqs, clk, cmd, d0, d1, d2, d3, config,
    ));
    // CmdBlock is only scratch space for card acquisition; it does not need
    // to be leaked for the lifetime of the mounted device.
    let mut cmd_block = CmdBlock::new();
    info!("SD init phase 2 complete: SDMMC peripheral constructed");

    info!("SD init phase 3: enabling card power and waiting 1 second");
    power.set_low();
    Timer::after_secs(1).await;
    info!("SD init phase 3 complete: card power settling delay elapsed");

    info!(
        "SD init phase 4: initializing SD card at {} Hz",
        SD_TARGET_FREQ.0
    );
    let card = match StorageDevice::new_sd_card(sdmmc, &mut cmd_block, SD_TARGET_FREQ).await {
        Ok(card) => {
            let card_info = card.card();
            info!(
                "SD init phase 4 complete: card initialized, blocks={} size_bytes={}",
                card_info.csd.block_count(),
                card_info.csd.card_size(),
            );
            card
        }
        Err(e) => {
            error!("SD init failed during card initialization: {}", e);
            notify_fatal(fatal_handler);
            pending_forever().await
        }
    };

    info!("SD init phase 5: wrapping card as a 512-byte block device");
    let mut device = Stm32SdBlockDevice::new(card);
    let device_identity = device.device_identity();
    info!(
        "SD card identity: mid={} oid={:?} product={:?} revision={} serial={} manufactured={}-{} capacity_bytes={}",
        device_identity.manufacturer_id,
        device_identity.oem_id,
        device_identity.product_name,
        device_identity.product_revision,
        device_identity.serial_number,
        device_identity.manufacture_year,
        device_identity.manufacture_month,
        device_identity.capacity_bytes,
    );
    info!("SD init phase 5 complete: block device ready");

    info!("SD init phase 6: detecting raw or MBR-partitioned exFAT volume");
    let volume = match detect_exfat_volume(&mut device).await {
        Ok(volume) => {
            info!(
                "SD init phase 6 complete: exFAT volume start_lba={} block_count={}",
                volume.start_lba, volume.block_count,
            );
            volume
        }
        Err(e) => {
            error!("SD init failed during exFAT volume detection: {}", e);
            notify_fatal(fatal_handler);
            pending_forever().await
        }
    };
    info!("SD init phase 7: constructing powered partitioned storage driver");
    let inner = StorageDriver::<_>::new_with_device_identity(
        PartitionedBlockDevice::new(device, volume),
        Some(device_identity),
    );
    info!("SD init phase 7 complete: driver owns SD power pin and is ready to mount");
    PoweredStorage {
        inner,
        _power: power,
    }
}

fn notify_fatal(fatal_handler: Option<fn()>) {
    if let Some(handler) = fatal_handler {
        handler();
    }
}

#[embassy_executor::task]
async fn gps_driver_task(driver: GpsDriver<BufferedUart<'static>, Stm32Pps, Stm32GpsPower>) -> ! {
    driver.run().await
}

#[embassy_executor::task]
async fn time_service_task(service: TimeService<4, 8>) -> ! {
    service.run().await
}

/// Release the GPS driver into duty cycling only after the Time Service has
/// collected and locked its full frequency-calibration baseline.
#[embassy_executor::task]
async fn gps_frequency_calibration_lock_task() {
    loop {
        if TIME_RESOURCES.time_state().frequency_calibration_locked {
            GPS_RESOURCES
                .command_sender()
                .send(GpsCommand::FrequencyCalibrationLocked)
                .await;
            return;
        }
        Timer::after_secs(1).await;
    }
}

/// Forward each newly evaluated post-calibration GPS PPS anchor to the GPS
/// manager. Both accepted and rejected observations are forwarded: an accepted
/// anchor can extend the good streak, while a rejected/gated edge breaks it.
/// Periodic watch publications with no evaluated anchor are ignored.
#[embassy_executor::task]
async fn gps_phase_quality_task() {
    let mut states = unwrap!(TIME_RESOURCES.state_receiver());
    let initial = TIME_RESOURCES.time_state();
    let mut observed_accepted = initial.accepted_anchors;
    let mut observed_rejected = initial.rejected_anchors;
    let mut calibration_was_locked = initial.frequency_calibration_locked;

    loop {
        let state = states.changed().await;
        let accepted = state.accepted_anchors != observed_accepted;
        let rejected = state.rejected_anchors != observed_rejected;
        let has_new_observation = accepted || rejected;
        observed_accepted = state.accepted_anchors;
        observed_rejected = state.rejected_anchors;
        if calibration_was_locked
            && has_new_observation
            && state.active_time_source == TimeSource::GpsPps
        {
            GPS_RESOURCES
                .command_sender()
                .send(GpsCommand::PhaseQuality {
                    observation_sequence: u64::from(state.accepted_anchors)
                        + u64::from(state.rejected_anchors),
                    // If the watch ever coalesces multiple publications, do
                    // not count the resulting ambiguous observation as good.
                    accepted: accepted && !rejected,
                    residual_us: state.last_anchor_residual_us,
                    uncertainty_us: state.uncertainty_us,
                    pps_gate_active: state.pps_reacquisition_active,
                })
                .await;
        }
        calibration_was_locked |= state.frequency_calibration_locked;
    }
}

#[embassy_executor::task]
async fn gps_time_source_task(
    correlations: embassy_sync::watch::DynReceiver<'static, raylar_drivers::gps::TimeCorrelation>,
) -> ! {
    run_gps_time_source(correlations, TIME_RESOURCES.anchor_sender()).await
}

pub async fn pending_forever() -> ! {
    loop {
        Timer::after_secs(60).await;
    }
}
