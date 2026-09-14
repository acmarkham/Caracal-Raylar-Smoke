#![no_std]
#![no_main]

extern crate alloc;

#[path = "../../servicetests/storage/common.rs"]
#[allow(dead_code)] // The real and synthetic time paths are feature-exclusive.
mod common;

use core::cell::RefCell;
use core::sync::atomic::{AtomicBool, AtomicU32, Ordering};
use defmt::{error, info, unwrap};
use embassy_executor::Spawner;
use embassy_stm32::gpio::{Input, Pull};
use embassy_stm32::i2c::{mode::Master, Config as I2cConfig, I2c};
use embassy_stm32::mode::Blocking;
use embassy_stm32::peripherals::{PA0, PA1, PB1};
use embassy_stm32::time::Hertz;
use embassy_stm32::{bind_interrupts, peripherals};
use embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex;
use embassy_sync::channel::Channel;
use embassy_sync::mutex::Mutex;
use embassy_time::{Duration, Instant, Timer, TICK_HZ};
use embedded_alloc::LlffHeap as Heap;
use raylar_audio_recorder_service::{
    AudioRecorder, AudioRecorderConfig, AudioRecorderError, RecorderProgress, TimeMetadataSource,
};
use raylar_audiosource::{AudioFormat, AudioSource};
use raylar_board_v1p0::{AdcVoltages, Board, Leds, PdmMicArray, PdmMicDma, SensI2C, UsbCdc};
use raylar_drivers::batterycharger::{ChargerConfig, ChargerDriver, ChargerResources};
use raylar_drivers::mic_array::stm32::{Dma0TimestampHandler, MonoPins, Stm32MonoMicrophoneDriver};
use raylar_drivers::mic_array::{
    MicrophoneConfig, MicrophoneMode, MicrophonePreset, MicrophoneResources,
};
use raylar_drivers::voltagemonitor::stm32::Stm32VoltageMonitor;
use raylar_drivers::voltagemonitor::{VoltageConfig, VoltageMonitorDriver, VoltageResources};
use raylar_drivers::{buzzer, leds};
use raylar_logging_service::{
    info as log_info, LogOutcome, LogSink, LoggerHandle, LoggingResources, LoggingService,
    ProcessOutcome,
};
use raylar_power_management_service::{PowerConfig, PowerManagementService, PowerResources};
use raylar_storage_service::{
    StorageBackend, StorageLayout, StorageService, StorageServiceError, StreamHandle, StreamKind,
};
use raylar_time_service::TimeResources;
use static_cell::StaticCell;
use {defmt_rtt as _, panic_probe as _};

const HEAP_BYTES: usize = 64 * 1024;
const MESSAGE_LENGTH: usize = 256;
const QUEUE_DEPTH: usize = 16;
const LINE_LENGTH: usize = 384;
const MIC_CONFIG: MicrophoneConfig = MicrophoneConfig {
    mode: MicrophoneMode::Mono,
    ..MicrophoneConfig::from_preset(MicrophonePreset::ReferenceSinc5_16KhzHiperf)
};
const SAMPLE_RATE_HZ: usize = MIC_CONFIG.sample_rate.hz() as usize;
const CHANNELS: usize = MIC_CONFIG.mode.channel_count();
// Match the working audiorecorder test: 100 ms per half at the requested
// 16 kHz rate. Each GPDMA linked-list item is 6,400 bytes.
const HALF_SAMPLES: usize = 1_600;
const DMA_SAMPLES: usize = HALF_SAMPLES * 2;
// Eight seconds absorbs SD write latency while the recorder catches up.
const AUDIO_CAPACITY: usize = SAMPLE_RATE_HZ * CHANNELS * 8;

type TestLogger = LoggerHandle<'static, MESSAGE_LENGTH, QUEUE_DEPTH>;
type BoardVoltageMonitor = Stm32VoltageMonitor<
    embassy_stm32::Peri<'static, PA0>,
    embassy_stm32::Peri<'static, PA1>,
    embassy_stm32::Peri<'static, PB1>,
    Input<'static>,
>;
type BoardVoltageDriver = VoltageMonitorDriver<BoardVoltageMonitor>;
type BoardChargerDriver = ChargerDriver<I2c<'static, Blocking, Master>>;
type MicDriver = Stm32MonoMicrophoneDriver<'static, DMA_SAMPLES>;

static VOLTAGES: VoltageResources = VoltageResources::new();
static CHARGER: ChargerResources = ChargerResources::new();
static POWER: PowerResources = PowerResources::new();
static LOGGING: LoggingResources<MESSAGE_LENGTH, QUEUE_DEPTH> = LoggingResources::new();
static MICROPHONES: MicrophoneResources<DMA_SAMPLES> = MicrophoneResources::new();
static AUDIO: AudioSource<AUDIO_CAPACITY, 2> = AudioSource::new(AudioFormat::new(
    SAMPLE_RATE_HZ as u32,
    CHANNELS as u8,
    1_000_000,
));
static ERROR_SIGNAL: embassy_sync::signal::Signal<CriticalSectionRawMutex, ()> =
    embassy_sync::signal::Signal::new();
static LED_COMMANDS: Channel<CriticalSectionRawMutex, LedCommand, 16> = Channel::new();
static AUDIO_RECORDING_ACTIVE: AtomicBool = AtomicBool::new(false);
static CPU_IDLE_OPEN: AtomicBool = AtomicBool::new(false);
static CPU_IDLE_START_TICKS: AtomicU32 = AtomicU32::new(0);
static CPU_IDLE_TICKS: AtomicU32 = AtomicU32::new(0);
static SHARED_STORAGE: StaticCell<SharedStorage<common::BoardStorageBackend>> = StaticCell::new();

#[global_allocator]
static HEAP: Heap = Heap::empty();

bind_interrupts!(struct MicIrqs {
    GPDMA1_CHANNEL0 => embassy_stm32::dma::InterruptHandler<peripherals::GPDMA1_CH0>, Dma0TimestampHandler;
});

// Embassy's thread-mode executor calls these trace hooks immediately before
// polling and after its ready queue is empty. The interval between those
// callbacks is the time spent in WFE (plus any interrupt work that happens
// before the executor is woken), which is the useful idle figure for this
// cooperative integration test.
#[unsafe(export_name = "_embassy_trace_poll_start")]
fn embassy_trace_poll_start(_executor_id: u32) {
    if CPU_IDLE_OPEN.swap(false, Ordering::AcqRel) {
        let now = Instant::now().as_ticks() as u32;
        let start = CPU_IDLE_START_TICKS.load(Ordering::Relaxed);
        CPU_IDLE_TICKS.fetch_add(now.wrapping_sub(start), Ordering::Relaxed);
    }
}

#[unsafe(export_name = "_embassy_trace_executor_idle")]
fn embassy_trace_executor_idle(_executor_id: u32) {
    CPU_IDLE_START_TICKS.store(Instant::now().as_ticks() as u32, Ordering::Relaxed);
    CPU_IDLE_OPEN.store(true, Ordering::Release);
}

#[unsafe(export_name = "_embassy_trace_task_new")]
fn embassy_trace_task_new(_executor_id: u32, _task_id: u32) {}

#[unsafe(export_name = "_embassy_trace_task_end")]
fn embassy_trace_task_end(_executor_id: u32, _task_id: u32) {}

#[unsafe(export_name = "_embassy_trace_task_exec_begin")]
fn embassy_trace_task_exec_begin(_executor_id: u32, _task_id: u32) {}

#[unsafe(export_name = "_embassy_trace_task_exec_end")]
fn embassy_trace_task_exec_end(_executor_id: u32, _task_id: u32) {}

#[unsafe(export_name = "_embassy_trace_task_ready_begin")]
fn embassy_trace_task_ready_begin(_executor_id: u32, _task_id: u32) {}

#[derive(Clone, Copy)]
enum LedCommand {
    On(leds::LedName),
    Off(leds::LedName),
    Toggle(leds::LedName),
}

struct SharedStorage<B: 'static> {
    inner: Mutex<CriticalSectionRawMutex, RefCell<StorageService<B, &'static TimeResources<4, 8>>>>,
}

impl<B> SharedStorage<B>
where
    B: StorageBackend<512> + 'static,
{
    fn new(storage: StorageService<B, &'static TimeResources<4, 8>>) -> Self {
        Self {
            inner: Mutex::new(RefCell::new(storage)),
        }
    }
    async fn begin(
        &self,
        kind: StreamKind,
        layout: StorageLayout,
    ) -> Result<StreamHandle, StorageServiceError<B::Error>> {
        self.inner
            .lock()
            .await
            .borrow_mut()
            .begin_stream(kind, layout)
            .await
    }
    async fn write(
        &self,
        stream: StreamHandle,
        data: &[u8],
    ) -> Result<(), StorageServiceError<B::Error>> {
        self.inner
            .lock()
            .await
            .borrow_mut()
            .write(stream, data)
            .await
    }
    async fn flush(&self, stream: StreamHandle) -> Result<(), StorageServiceError<B::Error>> {
        self.inner.lock().await.borrow_mut().flush(stream).await
    }
    async fn finish(&self, stream: StreamHandle) -> Result<(), StorageServiceError<B::Error>> {
        self.inner.lock().await.borrow_mut().finish(stream).await
    }
}

#[derive(Clone, Copy)]
struct SharedRecording<B: 'static> {
    storage: &'static SharedStorage<B>,
}

impl<B> raylar_audio_recorder_service::RecordingStorage for SharedRecording<B>
where
    B: StorageBackend<512> + 'static,
{
    type Error = StorageServiceError<B::Error>;
    type Handle = StreamHandle;
    async fn begin_audio_stream(
        &mut self,
        layout: StorageLayout,
    ) -> Result<Self::Handle, Self::Error> {
        self.storage.begin(StreamKind::Audio, layout).await
    }
    async fn append_audio(&mut self, stream: Self::Handle, data: &[u8]) -> Result<(), Self::Error> {
        self.storage.write(stream, data).await
    }
    async fn finish_audio(&mut self, stream: Self::Handle) -> Result<(), Self::Error> {
        self.storage.finish(stream).await
    }
}

struct SharedLogSink<B: 'static> {
    storage: &'static SharedStorage<B>,
    stream: StreamHandle,
}

impl<B> SharedLogSink<B>
where
    B: StorageBackend<512> + 'static,
{
    async fn open(
        storage: &'static SharedStorage<B>,
    ) -> Result<Self, StorageServiceError<B::Error>> {
        let stream = storage.begin(StreamKind::Log, StorageLayout::Flat).await?;
        Ok(Self { storage, stream })
    }
}

impl<B> LogSink for SharedLogSink<B>
where
    B: StorageBackend<512> + 'static,
{
    type Error = StorageServiceError<B::Error>;
    async fn append(&mut self, data: &[u8]) -> Result<(), Self::Error> {
        self.storage.write(self.stream, data).await
    }
    async fn flush(&mut self) -> Result<(), Self::Error> {
        self.storage.flush(self.stream).await
    }
}

#[embassy_executor::main]
async fn main(spawner: Spawner) -> ! {
    unsafe {
        embedded_alloc::init!(HEAP, HEAP_BYTES);
    }
    let Board {
        leds: board_leds,
        buzzer: board_buzzer,
        gps,
        sd,
        adc_voltages,
        sens_i2c,
        usb_cdc,
        pdm_mic_array,
        ..
    } = Board::new(embassy_stm32::init(common::mcu_config()));
    let Leds {
        sys_gps_green,
        sys_gps_red,
        sys_main_red,
        sys_main_green,
        sys_sd_blue,
    } = board_leds;
    spawner.spawn(unwrap!(led_task(leds::init(leds::LedResources {
        sys_gps_green,
        sys_gps_red,
        sys_main_red,
        sys_main_green,
        sys_sd_blue
    }))));
    spawner.spawn(unwrap!(error_latch_task()));
    spawner.spawn(unwrap!(cpu_usage_task()));
    #[cfg(not(feature = "fake-gps-time"))]
    common::start_time(spawner, gps).await;
    #[cfg(feature = "fake-gps-time")]
    {
        let fake_utc = env!("INTEGRATION002_FAKE_UTC_SECONDS")
            .parse::<i64>()
            .expect("build script emits a valid synthetic UTC epoch");
        common::start_fake_time(spawner, gps, fake_utc).await;
        info!("TEST MODE: fake-gps-time enabled; GPS hardware is ignored");
    }
    start_power(spawner, adc_voltages, sens_i2c, usb_cdc).await;
    let mut buzzer_driver = buzzer::init(buzzer::BuzzerResources {
        timer: board_buzzer.tim,
        pin: board_buzzer.pin,
    });
    for _ in 0..3 {
        let _ = buzzer_driver
            .play_tone(
                buzzer::PitchHz(1_000),
                Duration::from_millis(250),
                buzzer::Volume(255),
            )
            .await;
        Timer::after_millis(250).await;
    }
    #[cfg(not(feature = "fake-gps-time"))]
    spawner.spawn(unwrap!(gps_fix_trill_task(buzzer_driver)));
    #[cfg(feature = "fake-gps-time")]
    drop(buzzer_driver);
    let backend = common::storage_driver(sd).await;
    let mut storage = unwrap!(StorageService::new(backend, &common::TIME_RESOURCES));
    if let Err(error) = storage.mount().await {
        fail_forever("storage mount failed", error).await;
    }
    let storage = SHARED_STORAGE.init(SharedStorage::new(storage));
    let sink = match SharedLogSink::open(storage).await {
        Ok(sink) => sink,
        Err(error) => fail_forever("system log stream creation failed", error).await,
    };
    let mut logging =
        LoggingService::<_, MESSAGE_LENGTH, QUEUE_DEPTH, LINE_LENGTH>::new(&LOGGING, sink);
    let system_log = logging.register("System");
    let power_log = logging.register("Power");
    let time_log = logging.register("Time");
    let audio_log = logging.register("Audio");
    record_outcome(log_info!(system_log, "integration002 monoaudiolog started; format={}Hz mono, 60-second WAV files in hourly folders", SAMPLE_RATE_HZ));
    // Commit one record before audio startup. This makes /syslog.txt visible
    // even if GPS acquisition or microphone capture subsequently stalls.
    match logging.process_one().await {
        Ok(ProcessOutcome::Written) => {}
        Ok(ProcessOutcome::Empty) => {
            error!("system log startup record was not queued");
            ERROR_SIGNAL.signal(());
        }
        Err(error) => {
            error!("system log startup write failed: {}", error);
            ERROR_SIGNAL.signal(());
        }
    }
    if let Err(error) = logging.flush().await {
        error!("system log startup flush failed: {}", error);
        ERROR_SIGNAL.signal(());
    } else {
        info!("system log stream opened and flushed: /syslog.txt");
    }
    spawner.spawn(unwrap!(status_logger_task(power_log, time_log)));
    let microphone_driver = microphone_driver(pdm_mic_array);
    let resolved = microphone_driver.resolved_config();
    info!(
        "microphone configured: requested={}Hz calculated={}Hz clock={}Hz kernel={:?} decimation={} total_decimation={} sinc={:?} reshape={:?} hpf={} dma_samples={} half_samples={} half_bytes={}",
        resolved.requested.sample_rate.hz(),
        resolved.actual_sample_rate_hz,
        resolved.microphone_clock_hz,
        resolved.requested.kernel_clock,
        resolved.decimation,
        resolved.total_decimation,
        resolved.requested.sinc_filter,
        resolved.requested.reshape_filter,
        resolved.requested.high_pass_filter,
        DMA_SAMPLES,
        HALF_SAMPLES,
        HALF_SAMPLES * core::mem::size_of::<u32>(),
    );
    spawner.spawn(unwrap!(capture_task(microphone_driver)));
    spawner.spawn(unwrap!(audio_forwarder_task(audio_log)));
    let recorder = unwrap!(AudioRecorder::<_, _, AUDIO_CAPACITY, 2>::new(
        &AUDIO,
        SharedRecording { storage },
        TimeMetadataSource::new(&common::TIME_RESOURCES),
        AudioRecorderConfig {
            recording_seconds: 60,
            storage_layout: StorageLayout::HourlyFolders
        }
    ));
    run_services(logging, recorder).await
}

async fn start_power(
    spawner: Spawner,
    adc: AdcVoltages<'static>,
    sens: SensI2C<'static>,
    usb: UsbCdc<'static>,
) {
    let service = PowerManagementService::new(
        &POWER,
        unwrap!(VOLTAGES.state_receiver()).as_dyn(),
        unwrap!(CHARGER.state_receiver()).as_dyn(),
        PowerConfig::default(),
    );
    spawner.spawn(unwrap!(voltage_task(build_voltage_driver(adc, usb))));
    spawner.spawn(unwrap!(charger_task(build_charger_driver(sens))));
    spawner.spawn(unwrap!(power_service_task(service)));
}

fn build_voltage_driver(adc: AdcVoltages<'static>, usb: UsbCdc<'static>) -> BoardVoltageDriver {
    let AdcVoltages {
        adc,
        adc4,
        v_dc,
        v_batt,
        v_solar,
    } = adc;
    let UsbCdc { vbus, .. } = usb;
    VoltageMonitorDriver::new(
        Stm32VoltageMonitor::new(
            adc,
            adc4,
            v_dc,
            v_batt,
            v_solar,
            Input::new(vbus, Pull::None),
        ),
        &VOLTAGES,
        VoltageConfig::default(),
    )
}

fn build_charger_driver(sens: SensI2C<'static>) -> BoardChargerDriver {
    let SensI2C { i2c, scl, sda } = sens;
    let mut config = I2cConfig::default();
    config.frequency = Hertz(100_000);
    let mut charger_config = ChargerConfig::default();
    charger_config.default_charge_current_ma = 200;
    charger_config.default_input_current_limit_ma = 200;
    ChargerDriver::new(
        I2c::new_blocking(i2c, scl, sda, config),
        &CHARGER,
        charger_config,
    )
}

fn microphone_driver(pdm: PdmMicArray<'static>) -> MicDriver {
    let PdmMicArray {
        cck0,
        sd0,
        dma: PdmMicDma { ch0, .. },
        ..
    } = pdm;
    unwrap!(Stm32MonoMicrophoneDriver::new(
        MonoPins { cck0, sd0 },
        embassy_stm32::dma::Channel::new(ch0, MicIrqs),
        &MICROPHONES,
        MIC_CONFIG,
    ))
}

#[embassy_executor::task]
async fn led_task(mut driver: leds::LedDriver<'static>) -> ! {
    loop {
        match LED_COMMANDS.receive().await {
            LedCommand::On(led) => driver.on(led),
            LedCommand::Off(led) => driver.off(led),
            LedCommand::Toggle(led) => driver.toggle(led),
        }
    }
}

#[embassy_executor::task]
async fn error_latch_task() -> ! {
    ERROR_SIGNAL.wait().await;
    LED_COMMANDS
        .send(LedCommand::On(leds::LedName::SysMainRed))
        .await;
    loop {
        Timer::after_secs(60).await;
    }
}

#[embassy_executor::task]
async fn voltage_task(driver: BoardVoltageDriver) -> ! {
    driver.run().await
}

#[embassy_executor::task]
async fn charger_task(mut driver: BoardChargerDriver) -> ! {
    if driver.initialize().is_err() || driver.enable().is_err() {
        ERROR_SIGNAL.signal(());
    }
    loop {
        if driver.refresh_state().is_err() {
            ERROR_SIGNAL.signal(());
        }
        Timer::after_secs(1).await;
    }
}

#[embassy_executor::task]
async fn power_service_task(service: PowerManagementService) -> ! {
    service.run().await
}

#[embassy_executor::task]
async fn capture_task(driver: MicDriver) -> ! {
    driver.run().await
}

#[embassy_executor::task]
async fn gps_fix_trill_task(mut buzzer: buzzer::BuzzerDriver<'static>) {
    let mut fixes = unwrap!(common::GPS_RESOURCES.fix_receiver());
    let fix = fixes.changed().await;
    info!(
        "GPS first fix attained; playing acquisition trill: satellites={} hdop_centi={:?}",
        fix.satellites, fix.hdop_centi
    );

    // A quick alternating arpeggio followed by a high resolve: distinctive
    // from the three slow startup beeps, but short enough not to be intrusive.
    for pitch_hz in [1_319, 1_568, 1_319, 1_568, 1_319, 1_568] {
        let _ = buzzer
            .play_tone(
                buzzer::PitchHz(pitch_hz),
                Duration::from_millis(45),
                buzzer::Volume(180),
            )
            .await;
        Timer::after_millis(12).await;
    }
    let _ = buzzer
        .play_tone(
            buzzer::PitchHz(2_093),
            Duration::from_millis(110),
            buzzer::Volume(200),
        )
        .await;
}

#[embassy_executor::task]
async fn cpu_usage_task() -> ! {
    // Discard startup idle time so the first report describes a complete
    // one-second window after the task has been scheduled.
    let _ = CPU_IDLE_TICKS.swap(0, Ordering::AcqRel);
    let mut previous_ticks = Instant::now().as_ticks() as u32;
    loop {
        Timer::after_secs(1).await;
        let now = Instant::now().as_ticks() as u32;
        let elapsed_ticks = now.wrapping_sub(previous_ticks);
        let idle_ticks = CPU_IDLE_TICKS.swap(0, Ordering::AcqRel);
        let idle_ticks = idle_ticks.min(elapsed_ticks);
        let active_ticks = elapsed_ticks.saturating_sub(idle_ticks);
        let usage_percent = if elapsed_ticks == 0 {
            0
        } else {
            active_ticks.saturating_mul(100) / elapsed_ticks
        };
        info!(
            "cpu usage={}%, active_ms={} idle_ms={} window_ms={} update_hz=1",
            usage_percent,
            active_ticks / 1_000,
            idle_ticks / 1_000,
            elapsed_ticks / 1_000,
        );
        previous_ticks = now;
    }
}

async fn run_services<B>(
    mut logging: LoggingService<
        'static,
        SharedLogSink<B>,
        MESSAGE_LENGTH,
        QUEUE_DEPTH,
        LINE_LENGTH,
    >,
    mut recorder: AudioRecorder<
        'static,
        SharedRecording<B>,
        TimeMetadataSource<'static, 4, 8>,
        AUDIO_CAPACITY,
        2,
    >,
) -> !
where
    B: StorageBackend<512> + 'static,
    B::Error: defmt::Format,
{
    let mut next_flush = Instant::now() + Duration::from_secs(10);
    loop {
        match recorder.start().await {
            Ok(()) => break,
            Err(AudioRecorderError::TimeUnavailable) => {
                drain_logging(&mut logging).await;
                if Instant::now() >= next_flush {
                    flush_logging(&mut logging).await;
                    next_flush = Instant::now() + Duration::from_secs(10);
                }
                Timer::after_millis(100).await
            }
            Err(error) => fail_forever("audio recorder start failed", error).await,
        }
    }
    info!(
        "audio recording started after valid UTC time; source={:?}",
        common::TIME_RESOURCES.time_state().active_time_source
    );
    AUDIO_RECORDING_ACTIVE.store(true, Ordering::Release);
    loop {
        match recorder.record_next().await {
            Ok(RecorderProgress {
                pcm_samples,
                dropped_samples,
                rotated,
            }) => {
                LED_COMMANDS
                    .send(LedCommand::Toggle(leds::LedName::SysSdBlue))
                    .await;
                if dropped_samples != 0 {
                    error!("audio source dropped {} samples", dropped_samples);
                }
                if rotated {
                    info!("audio WAV rotated; packet_samples={}", pcm_samples);
                }
            }
            Err(error) => fail_forever("audio recorder failed", error).await,
        }
        drain_logging(&mut logging).await;
        if Instant::now() >= next_flush {
            flush_logging(&mut logging).await;
            next_flush = Instant::now() + Duration::from_secs(10);
        }
    }
}

async fn flush_logging<B>(
    logging: &mut LoggingService<
        'static,
        SharedLogSink<B>,
        MESSAGE_LENGTH,
        QUEUE_DEPTH,
        LINE_LENGTH,
    >,
) where
    B: StorageBackend<512> + 'static,
    B::Error: defmt::Format,
{
    if let Err(error) = logging.flush().await {
        error!("system log flush failed: {}", error);
        ERROR_SIGNAL.signal(());
    }
    let stats = logging.stats();
    info!(
        "system log stats: total={} dropped={} depth={} max_depth={} bytes={} truncated={} write_failures={}",
        stats.total_messages,
        stats.dropped_messages,
        stats.queue_depth,
        stats.maximum_queue_depth,
        stats.bytes_written,
        stats.truncated_messages,
        stats.write_failures,
    );
    if stats.dropped_messages != 0 || stats.write_failures != 0 {
        ERROR_SIGNAL.signal(());
    }
}

async fn drain_logging<B>(
    logging: &mut LoggingService<
        'static,
        SharedLogSink<B>,
        MESSAGE_LENGTH,
        QUEUE_DEPTH,
        LINE_LENGTH,
    >,
) where
    B: StorageBackend<512> + 'static,
    B::Error: defmt::Format,
{
    loop {
        match logging.process_one().await {
            Ok(ProcessOutcome::Written) => {}
            Ok(ProcessOutcome::Empty) => return,
            Err(error) => {
                error!("system log write failed: {}", error);
                ERROR_SIGNAL.signal(());
                return;
            }
        }
    }
}

#[embassy_executor::task]
async fn status_logger_task(power_log: TestLogger, time_log: TestLogger) -> ! {
    loop {
        LED_COMMANDS
            .send(LedCommand::On(leds::LedName::SysMainGreen))
            .await;
        LED_COMMANDS
            .send(LedCommand::On(leds::LedName::SysGpsGreen))
            .await;
        let power = POWER.state();
        record_outcome(log_info!(
            power_log,
            "source={:?} batt={}mV solar={}mV ext_dc={}mV charging={} percent={:?} health={:?} charger_state={:?} charger_fault={:?}",
            power.source,
            power.battery_mv,
            power.solar_mv,
            power.ext_dc_mv,
            power.charging,
            power.battery_percent,
            power.health,
            power.charger.state,
            power.charger.fault
        ));
        let time = common::TIME_RESOURCES.time_state();
        match common::TIME_RESOURCES.current_utc() {
            Ok(utc) => record_outcome(log_info!(
                time_log,
                "UTC {} source={:?} valid={} drift_ppb={} uncertainty_us={} holdover_us={}",
                utc.seconds,
                time.active_time_source,
                time.utc_valid,
                time.estimated_frequency_error_ppb,
                time.uncertainty_us,
                time.holdover_duration.as_micros()
            )),
            Err(_) => record_outcome(log_info!(
                time_log,
                "UTC unavailable source={:?} valid=false drift_ppb={} uncertainty_us={} holdover_us={}",
                time.active_time_source,
                time.estimated_frequency_error_ppb,
                time.uncertainty_us,
                time.holdover_duration.as_micros()
            )),
        }
        Timer::after_millis(100).await;
        LED_COMMANDS
            .send(LedCommand::Off(leds::LedName::SysMainGreen))
            .await;
        LED_COMMANDS
            .send(LedCommand::Off(leds::LedName::SysGpsGreen))
            .await;
        Timer::after_secs(10).await;
    }
}

#[embassy_executor::task]
async fn audio_forwarder_task(audio_log: TestLogger) -> ! {
    let mut frames = unwrap!(MICROPHONES.frame_receiver());
    let mut last_sequence = 0u64;
    let mut dma_error_count = 0u32;
    let mut rate_sequence = 0u64;
    let mut rate_ticks = 0u64;
    loop {
        let state = frames.changed().await;
        if let Some(dma_error) = state.error {
            dma_error_count = dma_error_count.wrapping_add(1);
            error!(
                "microphone DMA state error={:?} count={} sequence={} half={} running={} channels={} started_ticks={} completed_ticks={} irq_count={} filter_status={=u32:#010x}",
                dma_error,
                dma_error_count,
                state.sequence,
                state.half,
                state.running,
                state.channel_count,
                state.started_at_ticks,
                state.completed_at_ticks,
                state.dma_interrupt_count,
                state.filter_status,
            );
            continue;
        }
        if !state.running || state.sequence == 0 || state.sequence <= last_sequence {
            continue;
        }
        if last_sequence != 0 && state.sequence - last_sequence > 1 {
            error!(
                "microphone DMA frame notification gap: previous={} current={} missed={}",
                last_sequence,
                state.sequence,
                state.sequence - last_sequence - 1,
            );
        }
        last_sequence = state.sequence;
        if rate_sequence == 0 {
            rate_sequence = state.sequence;
            rate_ticks = state.completed_at_ticks;
        } else if state.sequence - rate_sequence >= 10 {
            let elapsed = state.completed_at_ticks.saturating_sub(rate_ticks);
            if elapsed != 0 {
                let samples = (state.sequence - rate_sequence) * HALF_SAMPLES as u64;
                let measured_hz = samples.saturating_mul(TICK_HZ) / elapsed;
                info!(
                    "microphone measured rate={}Hz frames={} samples={} elapsed_ticks={} avg_frame_ticks={} irq_count={} filter_status={=u32:#010x} dma_overrun={} clock_absent={}",
                    measured_hz,
                    state.sequence - rate_sequence,
                    samples,
                    elapsed,
                    elapsed / (state.sequence - rate_sequence),
                    state.dma_interrupt_count,
                    state.filter_status,
                    (state.filter_status & (1 << 1)) != 0,
                    (state.filter_status & (1 << 10)) != 0,
                );
            } else {
                error!(
                    "microphone DMA timestamp did not advance: sequence={} completed_ticks={}",
                    state.sequence, state.completed_at_ticks,
                );
            }
            rate_sequence = state.sequence;
            rate_ticks = state.completed_at_ticks;
        }
        // Keep validating the DMA cadence while GPS is unavailable, but do not
        // fill the bounded recorder queue with pre-fix audio. Recording begins
        // with the first frame captured after a valid UTC anchor exists.
        if !AUDIO_RECORDING_ACTIVE.load(Ordering::Acquire) {
            continue;
        }
        let frame = MICROPHONES.frame(state);
        let channel = frame.active_channels()[0];
        if AUDIO
            .write_from_fn(channel.len(), state.completed_at_ticks, |index| {
                (channel[index] as i32) >> 8
            })
            .is_err()
        {
            ERROR_SIGNAL.signal(());
            continue;
        }
        record_outcome(audio_log.log_at(
            Instant::from_ticks(state.completed_at_ticks),
            raylar_logging_service::LogLevel::Info,
            format_args!(
                "audio packet timestamp_ticks={} samples={}",
                state.completed_at_ticks,
                channel.len()
            ),
        ));
    }
}

fn record_outcome(outcome: LogOutcome) {
    if matches!(outcome, LogOutcome::DroppedQueueFull) {
        ERROR_SIGNAL.signal(());
    }
}

async fn fail_forever<E: defmt::Format>(message: &str, value: E) -> ! {
    error!("{}: {}", message, value);
    ERROR_SIGNAL.signal(());
    loop {
        Timer::after_secs(60).await;
    }
}
