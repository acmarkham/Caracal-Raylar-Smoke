#![no_std]
#![no_main]

mod audio;
#[path = "../../servicetests/storage/common.rs"]
#[allow(dead_code)]
mod common;
mod diagnostics;
mod power;
mod radio;
mod storage;
mod config {
    pub use aardwolf::config::*;
}
mod policy {
    pub use aardwolf::policy::*;
}

use defmt::{error, info, unwrap};
use embassy_executor::Spawner;
use embassy_stm32::{bind_interrupts, peripherals};
use embassy_time::{Duration, Instant, Timer};
use embedded_alloc::LlffHeap as Heap;
use raylar_board_v1p0::{Board, Leds};
use raylar_drivers::{
    gps::PhaseQualifiedShutdownConfig,
    identity,
    mic_array::stm32::Dma0TimestampHandler,
    stm32_core::{stm32::Stm32CoreDriver, CoreConfig, CoreSupply},
    trng::{stm32::Stm32Trng, TrngConfig},
};
use raylar_location_service::{LocationConfig, LocationResources, LocationService};
use raylar_logging_service::{info as log_info, LoggingResources, LoggingService, ProcessOutcome};
use raylar_radio_service::NodeId;
use raylar_storage_service::StorageService;
use raylar_versioning_service::{
    IdentityConfig, IdentityField, IdentityResources, IdentityVersioningService,
};
use static_cell::StaticCell;
use {defmt_rtt as _, panic_probe as _};

use audio::MICROPHONES;
use storage::{SharedStorage, SystemLogSink, STORAGE_FLAGS};

static SHARED_STORAGE: StaticCell<SharedStorage> = StaticCell::new();
static STORAGE_READY: core::sync::atomic::AtomicBool = core::sync::atomic::AtomicBool::new(false);
static STORAGE_SEVERE: core::sync::atomic::AtomicBool = core::sync::atomic::AtomicBool::new(false);
/// Deliberately uninitialised: any accidental allocation fails closed.
#[global_allocator]
static HEAP: Heap = Heap::empty();
static LOGGING: LoggingResources<384, 32> = LoggingResources::new();
static LOCATION: LocationResources<4> = LocationResources::new();
static VERSIONING: IdentityResources<4> = IdentityResources::new();
const GPS_ON_TIME: Duration = Duration::from_secs(60);
const GPS_STANDBY_TIME: Duration = Duration::from_secs(30 * 60);

bind_interrupts!(struct MicIrqs {
    GPDMA1_CHANNEL0 => embassy_stm32::dma::InterruptHandler<peripherals::GPDMA1_CH0>, Dma0TimestampHandler;
});

#[embassy_executor::main]
async fn main(spawner: Spawner) -> ! {
    let reset = Stm32CoreDriver::take_reset_flags();
    let (config, correction) = unwrap!(common::mcu_config_with_hse_error_ppm(
        aardwolf::config::HSE_MEASURED_ERROR_PPM
    ));
    let peripherals = embassy_stm32::init(config);
    let pll_n = correction.integer_n();
    let pll_fracn = correction.fracn();
    correction.apply();
    let supply = if cfg!(feature = "core-smps") {
        CoreSupply::Smps
    } else {
        CoreSupply::Ldo
    };
    let _core = unwrap!(Stm32CoreDriver::init(CoreConfig { supply }));
    let Board {
        leds,
        buzzer,
        gps,
        sd,
        adc_voltages,
        sens_i2c,
        usb_cdc,
        pdm_mic_array,
        ebyte_rf,
        trng,
        ..
    } = Board::new(peripherals);
    let node = NodeId::from(identity::init().serials());
    let mut trng = Stm32Trng::new(trng, raylar_board_v1p0::Irqs, TrngConfig::default());
    let boot = match trng.boot_id().await {
        Ok(value) => value as u16,
        Err(error) => {
            error!("TRNG failure {:?}", error);
            loop {
                Timer::after_secs(60).await;
            }
        }
    };
    let system_log = LOGGING.register("Aardwolf");
    let radio_log = LOGGING.register("Radio");
    let audio_log = LOGGING.register("Audio");
    let power_log = LOGGING.register("Power");
    let _ = log_info!(system_log, "boot node={} boot={} config={} network={} schedule={} band868_hz=868100000 band24_hz=244100000 antenna=unknown firmware={}",
        node.0, boot, aardwolf::config::CONFIG_ID, aardwolf::config::NETWORK_ID, aardwolf::config::SCHEDULE_VERSION, env!("CARGO_PKG_VERSION"));
    let _ = log_info!(
        system_log,
        "reset pin={} brownout={} software={} iwdg={} wwdg={} low_power={} option_byte={}",
        reset.pin,
        reset.brownout,
        reset.software,
        reset.independent_watchdog,
        reset.window_watchdog,
        reset.low_power,
        reset.option_byte
    );
    let _ = log_info!(
        system_log,
        "clock hse_trim_ppm={} pll_n={} pll_fracn={} gps_pps_required=true",
        aardwolf::config::HSE_MEASURED_ERROR_PPM,
        pll_n,
        pll_fracn
    );
    let _ = log_info!(
        system_log,
        "gps duty initial_until_lock=true on_s={} standby_s={} standby_retains_rail=true",
        GPS_ON_TIME.as_secs(),
        GPS_STANDBY_TIME.as_secs()
    );
    for index in 0..aardwolf::config::ACTIVE_MINUTES {
        if let Ok(profile) = aardwolf::config::profile(index) {
            let _ = log_info!(
                system_log,
                "profile={} configuration={:?} airtime_us={} slot_s={}",
                index + 1,
                profile,
                aardwolf::config::airtime_us(index),
                aardwolf::config::slot_seconds(index)
            );
        }
    }
    info!("Aardwolf boot node={} boot={}", node.0, boot);

    common::start_time_stopped_with_phase_qualified_duty_cycle(
        spawner,
        gps,
        GPS_ON_TIME,
        GPS_STANDBY_TIME,
        PhaseQualifiedShutdownConfig {
            maximum_on_time: Duration::from_secs(180),
            residual_threshold_us: 250,
            uncertainty_threshold_us: 500,
            consecutive_anchors: 5,
        },
    )
    .await;
    let location = LocationService::<4, 9>::new(
        &LOCATION,
        unwrap!(common::GPS_RESOURCES.fix_receiver()).as_dyn(),
        LocationConfig::default(),
    );
    spawner.spawn(unwrap!(location_task(location)));
    power::start(spawner, adc_voltages, sens_i2c, usb_cdc, power_log).await;
    radio::start(spawner, ebyte_rf, node, boot, radio_log);
    spawner.spawn(unwrap!(ui_task(leds, buzzer)));

    STORAGE_FLAGS.fetch_or(
        raylar_radio_service::heartbeat_v4::LOGGING_IMPAIRED,
        core::sync::atomic::Ordering::Relaxed,
    );
    let backend = common::storage_driver_with_fatal_handler(sd, mark_storage_failure).await;
    let mut service = unwrap!(StorageService::<_, _, 512, 2, 16_384>::new(
        backend,
        &common::TIME_RESOURCES
    ));
    if let Err(error) = service.mount().await {
        mark_storage_failure();
        STORAGE_FLAGS.fetch_or(
            raylar_radio_service::heartbeat_v4::LOGGING_IMPAIRED,
            core::sync::atomic::Ordering::Relaxed,
        );
        error!("storage mount failed {:?}", error);
        loop {
            Timer::after_secs(60).await;
        }
    }
    let mut versioning = IdentityVersioningService::new(&VERSIONING, IdentityConfig::default());
    versioning.set_sd_card_identity(
        service
            .device_identity()
            .map(|card| IdentityField::Known(card.into()))
            .unwrap_or(IdentityField::Unavailable),
    );
    let trace = versioning.state();
    let _ = log_info!(
        system_log,
        "versioning device={:?} firmware={:?} board={:?}",
        trace.device,
        trace.firmware,
        trace.hardware.board_revision
    );
    let _ = log_info!(
        system_log,
        "versioning sd_card={:?} gps_module={:?} radio_module={:?}",
        trace.hardware.sd_card,
        trace.hardware.gps_module,
        trace.hardware.radio_module
    );
    versioning.publish();
    let shared = SHARED_STORAGE.init(SharedStorage::new(service));
    let sink = match SystemLogSink::open(shared).await {
        Ok(sink) => sink,
        Err(error) => {
            mark_storage_failure();
            STORAGE_FLAGS.fetch_or(
                raylar_radio_service::heartbeat_v4::LOGGING_IMPAIRED,
                core::sync::atomic::Ordering::Relaxed,
            );
            error!("log open failed {:?}", error);
            loop {
                Timer::after_secs(60).await;
            }
        }
    };
    STORAGE_FLAGS.store(0, core::sync::atomic::Ordering::Relaxed);
    STORAGE_READY.store(true, core::sync::atomic::Ordering::Relaxed);
    let logging = LoggingService::<_, 384, 32, 512>::new(&LOGGING, sink);
    spawner.spawn(unwrap!(logging_task(logging)));
    spawner.spawn(unwrap!(diagnostics::time_task(LOGGING.register("Time"))));
    spawner.spawn(unwrap!(diagnostics::gps_task(LOGGING.register("Gps"))));
    spawner.spawn(unwrap!(diagnostics::location_task(
        LOGGING.register("Location")
    )));
    audio::start(spawner, pdm_mic_array, shared, node.0, boot, audio_log);
    loop {
        Timer::after_secs(60).await;
    }
}

#[embassy_executor::task]
async fn location_task(service: LocationService<4, 9>) -> ! {
    service.run().await
}

fn mark_storage_failure() {
    STORAGE_FLAGS.fetch_or(
        raylar_radio_service::heartbeat_v4::STORAGE_UNAVAILABLE
            | raylar_radio_service::heartbeat_v4::LOGGING_IMPAIRED,
        core::sync::atomic::Ordering::Relaxed,
    );
    STORAGE_SEVERE.store(true, core::sync::atomic::Ordering::Relaxed);
}

#[embassy_executor::task]
async fn logging_task(mut service: LoggingService<'static, SystemLogSink, 384, 32, 512>) -> ! {
    let mut next_flush = Instant::now() + Duration::from_secs(10);
    loop {
        match service.process_one().await {
            Ok(ProcessOutcome::Written) => {}
            Ok(ProcessOutcome::Empty) => Timer::after_millis(100).await,
            Err(error) => {
                STORAGE_FLAGS.fetch_or(
                    raylar_radio_service::heartbeat_v4::LOGGING_IMPAIRED,
                    core::sync::atomic::Ordering::Relaxed,
                );
                error!("logging write failed {:?}", error);
                Timer::after_secs(1).await;
            }
        }
        if Instant::now() >= next_flush {
            if service.checkpoint().await.is_err() {
                STORAGE_FLAGS.fetch_or(
                    raylar_radio_service::heartbeat_v4::LOGGING_IMPAIRED,
                    core::sync::atomic::Ordering::Relaxed,
                );
            }
            next_flush = Instant::now() + Duration::from_secs(10);
        }
    }
}

#[embassy_executor::task]
async fn ui_task(leds: Leds<'static>, buzzer: raylar_board_v1p0::Buzzer<'static>) -> ! {
    use raylar_drivers::{buzzer as bz, leds as ld};
    let Leds {
        sys_gps_green,
        sys_gps_red,
        sys_main_red,
        sys_main_green,
        sys_sd_blue,
    } = leds;
    let mut leds = ld::init(ld::LedResources {
        sys_gps_green,
        sys_gps_red,
        sys_main_red,
        sys_main_green,
        sys_sd_blue,
    });
    let mut buzzer = bz::init(bz::BuzzerResources {
        timer: buzzer.tim,
        pin: buzzer.pin,
    });
    let _ = buzzer
        .play_tone(bz::PitchHz(900), Duration::from_millis(80), bz::Volume(80))
        .await;
    let mut last_pps = 0u64;
    let mut last_rx = 0u32;
    let mut last_tx = 0u32;
    let mut last_audio = 0u32;
    let mut first_fix_signalled = false;
    let mut lock_signalled = false;
    let mut severe_signalled = false;
    loop {
        let gps = common::GPS_RESOURCES.stats();
        let rx = radio::VALID_RX.load(core::sync::atomic::Ordering::Relaxed);
        let tx = radio::COMPLETED_TX.load(core::sync::atomic::Ordering::Relaxed);
        let audio = audio::AUDIO_PACKETS.load(core::sync::atomic::Ordering::Relaxed);
        let pps = gps.num_pps_events;
        if gps.operating_state.is_tracking() && pps != last_pps {
            leds.on(ld::LedName::SysGpsGreen);
        } else {
            leds.off(ld::LedName::SysGpsGreen);
        }
        if rx != last_rx {
            leds.on(ld::LedName::SysMainGreen);
            if cfg!(feature = "bench-rx-beep") {
                let _ = buzzer
                    .play_tone(
                        bz::PitchHz(1_500),
                        Duration::from_millis(20),
                        bz::Volume(35),
                    )
                    .await;
            }
        } else {
            leds.off(ld::LedName::SysMainGreen);
        }
        if tx != last_tx {
            leds.on(ld::LedName::SysGpsRed);
        } else {
            leds.off(ld::LedName::SysGpsRed);
        }
        if audio != last_audio {
            leds.on(ld::LedName::SysSdBlue);
        } else {
            leds.off(ld::LedName::SysSdBlue);
        }
        last_pps = pps;
        last_rx = rx;
        last_tx = tx;
        last_audio = audio;
        let severe = STORAGE_SEVERE.load(core::sync::atomic::Ordering::Relaxed)
            || (STORAGE_READY.load(core::sync::atomic::Ordering::Relaxed)
                && STORAGE_FLAGS.load(core::sync::atomic::Ordering::Relaxed) != 0)
            || audio::AUDIO_LOSSES.load(core::sync::atomic::Ordering::Relaxed) != 0;
        if severe {
            leds.on(ld::LedName::SysMainRed);
        } else {
            leds.off(ld::LedName::SysMainRed);
        }
        if !first_fix_signalled && gps.got_first_fix {
            first_fix_signalled = true;
            let _ = buzzer
                .play_tone(
                    bz::PitchHz(1_100),
                    Duration::from_millis(70),
                    bz::Volume(80),
                )
                .await;
        }
        if !lock_signalled
            && common::TIME_RESOURCES
                .time_state()
                .frequency_calibration_locked
        {
            lock_signalled = true;
            let _ = buzzer
                .play_tone(
                    bz::PitchHz(1_350),
                    Duration::from_millis(70),
                    bz::Volume(80),
                )
                .await;
        }
        if severe && !severe_signalled {
            severe_signalled = true;
            let _ = buzzer
                .play_tone(
                    bz::PitchHz(380),
                    Duration::from_millis(150),
                    bz::Volume(100),
                )
                .await;
        }
        Timer::after_millis(50).await;
    }
}
