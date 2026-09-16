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
use embassy_stm32::i2c::{Config as I2cConfig, I2c, mode::Master};
use embassy_stm32::mode::Blocking;
use embassy_stm32::peripherals::{PA0, PA1, PB1};
use embassy_stm32::time::Hertz;
use embassy_stm32::{bind_interrupts, peripherals};
use embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex;
use embassy_sync::channel::Channel;
use embassy_sync::mutex::Mutex;
use embassy_time::{Duration, Instant, TICK_HZ, Timer};
use embedded_alloc::LlffHeap as Heap;
use raylar_audio_recorder_service::{
    AudioRecorder, AudioRecorderConfig, AudioRecorderError, RecorderProgress, TimeMetadataSource,
};
use raylar_audiosource::{AudioFormat, AudioSource};
use raylar_board_v1p0::{AdcVoltages, Board, Leds, PdmMicArray, PdmMicDma, SensI2C, UsbCdc};
use raylar_drivers::batterycharger::{ChargerConfig, ChargerDriver, ChargerResources};
use raylar_drivers::identity;
use raylar_drivers::mic_array::stm32::{Dma0TimestampHandler, MonoPins, Stm32MonoMicrophoneDriver};
use raylar_drivers::mic_array::{
    MicrophoneConfig, MicrophoneMode, MicrophonePreset, MicrophoneResources,
};
use raylar_drivers::voltagemonitor::stm32::Stm32VoltageMonitor;
use raylar_drivers::voltagemonitor::{VoltageConfig, VoltageMonitorDriver, VoltageResources};
use raylar_drivers::{buzzer, leds};
use raylar_logging_service::{
    LogOutcome, LogSink, LoggerHandle, LoggingResources, LoggingService, ProcessOutcome,
    info as log_info,
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
static BUZZER_COMMANDS: Channel<CriticalSectionRawMutex, BuzzerCommand, 4> = Channel::new();
static SEVERE_ERROR_ACTIVE: AtomicBool = AtomicBool::new(false);
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

#[derive(Clone, Copy)]
enum BuzzerCommand {
    #[cfg(not(feature = "fake-gps-time"))]
    GpsPpsAcquired,
    SevereError,
}

fn signal_severe_error() {
    if !SEVERE_ERROR_ACTIVE.swap(true, Ordering::AcqRel) {
        AUDIO_RECORDING_ACTIVE.store(false, Ordering::Release);
        ERROR_SIGNAL.signal(());
    }
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
    async fn checkpoint(&self, stream: StreamHandle) -> Result<(), StorageServiceError<B::Error>> {
        self.inner
            .lock()
            .await
            .borrow_mut()
            .checkpoint(stream)
            .await
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
    async fn checkpoint(&mut self) -> Result<(), Self::Error> {
        self.storage.checkpoint(self.stream).await
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
    spawner.spawn(unwrap!(severe_error_task()));
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
        if SEVERE_ERROR_ACTIVE.load(Ordering::Acquire) {
            break;
        }
        let _ = buzzer_driver
            .play_tone(
                buzzer::PitchHz(1_000),
                Duration::from_millis(250),
                buzzer::Volume(255),
            )
            .await;
        Timer::after_millis(250).await;
    }
    spawner.spawn(unwrap!(buzzer_task(buzzer_driver)));
    if SEVERE_ERROR_ACTIVE.load(Ordering::Acquire) {
        common::pending_forever().await;
    }
    #[cfg(not(feature = "fake-gps-time"))]
    spawner.spawn(unwrap!(gps_pps_trill_task()));
    let backend = common::storage_driver_with_fatal_handler(sd, signal_severe_error).await;
    let mut storage = match StorageService::new(backend, &common::TIME_RESOURCES) {
        Ok(storage) => storage,
        Err(error) => fail_forever("storage service creation failed", error).await,
    };
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
    let gps_log = logging.register("Gps");
    let audio_log = logging.register("Audio");
    log_identity(system_log);
    record_outcome(log_info!(
        system_log,
        "integration002 monoaudiolog started; format={}Hz mono, 60-second WAV files in hourly folders",
        SAMPLE_RATE_HZ
    ));
    // Commit startup records before audio startup. This makes /syslog.txt
    // visible even if GPS acquisition or microphone capture subsequently
    // stalls.
    for _ in 0..2 {
        match logging.process_one().await {
            Ok(ProcessOutcome::Written) => {}
            Ok(ProcessOutcome::Empty) => {
                error!("system log startup record was not queued");
                signal_severe_error();
                break;
            }
            Err(error) => {
                error!("system log startup write failed: {}", error);
                signal_severe_error();
                break;
            }
        }
    }
    if let Err(error) = logging.flush().await {
        error!("system log startup flush failed: {}", error);
        signal_severe_error();
    } else {
        info!("system log stream opened and flushed: /syslog.txt");
    }
    spawner.spawn(unwrap!(status_logger_task(power_log, time_log, gps_log)));
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
    let recorder = match AudioRecorder::<_, _, AUDIO_CAPACITY, 2>::new(
        &AUDIO,
        SharedRecording { storage },
        TimeMetadataSource::new(&common::TIME_RESOURCES),
        AudioRecorderConfig {
            recording_seconds: 60,
            storage_layout: StorageLayout::HourlyFolders,
        },
    ) {
        Ok(recorder) => recorder,
        Err(error) => fail_forever("audio recorder creation failed", error).await,
    };
    run_services(logging, recorder).await
}

fn log_identity(system_log: TestLogger) {
    let identity = identity::init();
    let uid = identity.uid();
    let serials = identity.serials();
    match identity::calculate_firmware_crc32() {
        Ok(crc32) => record_outcome(log_info!(
            system_log,
            "identity uuid={:08X}-{:08X}-{:08X} serial64={:016X} serial48={:012X} serial32={:08X} serial16={:04X} firmware_crc32={:08X}",
            uid.word0,
            uid.word1,
            uid.word2,
            serials.serial_64,
            serials.serial_48,
            serials.serial_32,
            serials.serial_16,
            crc32
        )),
        Err(error) => record_outcome(log_info!(
            system_log,
            "identity uuid={:08X}-{:08X}-{:08X} serial64={:016X} serial48={:012X} serial32={:08X} serial16={:04X} firmware_crc32_error={:?}",
            uid.word0,
            uid.word1,
            uid.word2,
            serials.serial_64,
            serials.serial_48,
            serials.serial_32,
            serials.serial_16,
            error
        )),
    }
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
async fn severe_error_task() -> ! {
    ERROR_SIGNAL.wait().await;
    AUDIO_RECORDING_ACTIVE.store(false, Ordering::Release);
    BUZZER_COMMANDS.send(BuzzerCommand::SevereError).await;
    LED_COMMANDS
        .send(LedCommand::Off(leds::LedName::SysMainGreen))
        .await;
    LED_COMMANDS
        .send(LedCommand::Off(leds::LedName::SysGpsGreen))
        .await;
    LED_COMMANDS
        .send(LedCommand::Off(leds::LedName::SysSdBlue))
        .await;
    loop {
        LED_COMMANDS
            .send(LedCommand::On(leds::LedName::SysMainRed))
            .await;
        LED_COMMANDS
            .send(LedCommand::On(leds::LedName::SysGpsRed))
            .await;
        Timer::after_millis(500).await;
        LED_COMMANDS
            .send(LedCommand::Off(leds::LedName::SysMainRed))
            .await;
        LED_COMMANDS
            .send(LedCommand::Off(leds::LedName::SysGpsRed))
            .await;
        Timer::after_millis(500).await;
    }
}

#[embassy_executor::task]
async fn voltage_task(driver: BoardVoltageDriver) -> ! {
    driver.run().await
}

#[embassy_executor::task]
async fn charger_task(mut driver: BoardChargerDriver) -> ! {
    if driver.initialize().is_err() || driver.enable().is_err() {
        signal_severe_error();
    }
    loop {
        if driver.refresh_state().is_err() {
            // A transient telemetry refresh failure is recoverable. Keep the
            // service running; initialization/enable failure above is fatal.
            error!("charger state refresh failed; retrying");
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
#[cfg(not(feature = "fake-gps-time"))]
async fn gps_pps_trill_task() {
    let mut states = unwrap!(common::TIME_RESOURCES.state_receiver());
    let state = loop {
        let state = states.changed().await;
        if state.active_time_source == raylar_time_service::TimeSource::GpsPps
            && state.accepted_anchors != 0
        {
            break state;
        }
    };
    info!(
        "GPS first PPS anchor accepted; playing acquisition trill: accepted={} residual_us={:?}",
        state.accepted_anchors, state.last_anchor_residual_us
    );
    if !SEVERE_ERROR_ACTIVE.load(Ordering::Acquire) {
        BUZZER_COMMANDS.send(BuzzerCommand::GpsPpsAcquired).await;
    }
}

#[embassy_executor::task]
async fn buzzer_task(mut driver: buzzer::BuzzerDriver<'static>) -> ! {
    loop {
        match BUZZER_COMMANDS.receive().await {
            #[cfg(not(feature = "fake-gps-time"))]
            BuzzerCommand::GpsPpsAcquired => play_gps_pps_trill(&mut driver).await,
            BuzzerCommand::SevereError => loop {
                let next_alarm = Instant::now() + Duration::from_secs(10);
                play_severe_error_signal(&mut driver).await;
                Timer::at(next_alarm).await;
            },
        }
    }
}

#[cfg(not(feature = "fake-gps-time"))]
async fn play_gps_pps_trill(buzzer: &mut buzzer::BuzzerDriver<'static>) {
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

async fn play_severe_error_signal(buzzer: &mut buzzer::BuzzerDriver<'static>) {
    for pitch_hz in [1_200, 800, 400] {
        let _ = buzzer
            .play_tone(
                buzzer::PitchHz(pitch_hz),
                Duration::from_millis(180),
                buzzer::Volume(255),
            )
            .await;
        Timer::after_millis(80).await;
    }
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
    let mut next_checkpoint = Instant::now() + Duration::from_secs(10);
    loop {
        if SEVERE_ERROR_ACTIVE.load(Ordering::Acquire) {
            common::pending_forever().await;
        }
        match recorder.start().await {
            Ok(()) => break,
            Err(AudioRecorderError::TimeUnavailable) => {
                drain_logging(&mut logging).await;
                if Instant::now() >= next_checkpoint {
                    checkpoint_logging(&mut logging).await;
                    next_checkpoint = Instant::now() + Duration::from_secs(10);
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
    if SEVERE_ERROR_ACTIVE.load(Ordering::Acquire) {
        common::pending_forever().await;
    }
    AUDIO_RECORDING_ACTIVE.store(true, Ordering::Release);
    loop {
        match recorder.record_next().await {
            Ok(RecorderProgress {
                pcm_samples,
                dropped_samples,
                rotated,
            }) => {
                if SEVERE_ERROR_ACTIVE.load(Ordering::Acquire) {
                    AUDIO_RECORDING_ACTIVE.store(false, Ordering::Release);
                    common::pending_forever().await;
                }
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
        if Instant::now() >= next_checkpoint {
            checkpoint_logging(&mut logging).await;
            next_checkpoint = Instant::now() + Duration::from_secs(10);
        }
    }
}

async fn checkpoint_logging<B>(
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
    if let Err(error) = logging.checkpoint().await {
        error!("system log checkpoint failed: {}", error);
        signal_severe_error();
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
        signal_severe_error();
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
                signal_severe_error();
                return;
            }
        }
    }
}

#[embassy_executor::task]
async fn status_logger_task(power_log: TestLogger, time_log: TestLogger, gps_log: TestLogger) -> ! {
    loop {
        if SEVERE_ERROR_ACTIVE.load(Ordering::Acquire) {
            common::pending_forever().await;
        }
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
                "UTC {} src={:?} first={:?} valid={} map_ppb={} cal_ppb={} cal_n={} slew_ppb={} residual_us={:?} uncertainty_us={} holdover_us={} anchors={}/{} utc_fix={}",
                utc.seconds,
                time.active_time_source,
                time.first_anchor_source,
                time.utc_valid,
                time.estimated_frequency_error_ppb,
                time.calibrated_frequency_error_ppb,
                time.frequency_calibration_samples,
                time.phase_slew_ppb,
                time.last_anchor_residual_us,
                time.uncertainty_us,
                time.holdover_duration.as_micros(),
                time.accepted_anchors,
                time.rejected_anchors,
                time.utc_second_corrections
            )),
            Err(_) => record_outcome(log_info!(
                time_log,
                "UTC unavailable src={:?} first={:?} valid=false map_ppb={} cal_ppb={} cal_n={} slew_ppb={} residual_us={:?} uncertainty_us={} holdover_us={} anchors={}/{} utc_fix={}",
                time.active_time_source,
                time.first_anchor_source,
                time.estimated_frequency_error_ppb,
                time.calibrated_frequency_error_ppb,
                time.frequency_calibration_samples,
                time.phase_slew_ppb,
                time.last_anchor_residual_us,
                time.uncertainty_us,
                time.holdover_duration.as_micros(),
                time.accepted_anchors,
                time.rejected_anchors,
                time.utc_second_corrections
            )),
        }
        let gps = common::GPS_RESOURCES.stats();
        record_outcome(log_info!(
            gps_log,
            "state={:?} powered={} calibrated={} fixes={} checksum_err={} uart_err={} overflow={} reacq={}/{} search={}/{} pps_events={} pps_source={:?} pps_timeouts={} search_timeouts={}",
            gps.operating_state,
            gps.powered,
            gps.initial_calibration_complete,
            gps.num_fixes,
            gps.num_checksum_errors,
            gps.num_uart_errors,
            gps.num_buffer_overflows,
            gps.num_reacquisition_attempts,
            gps.num_reacquisition_successes,
            gps.num_search_attempts,
            gps.num_search_failures,
            gps.num_pps_events,
            gps.last_pps_timing_source,
            gps.num_pps_timeouts,
            gps.num_search_timeouts
        ));
        let now = Instant::now();
        let latest_pps = common::GPS_RESOURCES.latest_pps();
        let latest_fix = common::GPS_RESOURCES.latest_fix();
        if let Some(pps) = latest_pps {
            record_outcome(log_info!(
                gps_log,
                "PPS count={} age_us={} source={:?} systime_us={} capture_ticks={:?} delta_ticks={:?} capture_hz={:?} system_delta_us={:?}",
                pps.pps_count,
                now.saturating_duration_since(pps.timestamp).as_micros(),
                pps.timing_source,
                pps.timestamp.as_micros(),
                pps.capture_ticks,
                pps.capture_delta_ticks,
                pps.capture_frequency_hz,
                pps.delta_time.map(|delta| delta.as_micros())
            ));
        } else {
            record_outcome(log_info!(gps_log, "PPS none received"));
        }
        if let Some(fix) = latest_fix {
            record_outcome(log_info!(
                gps_log,
                "FIX count={} age_us={} systime_us={} utc={}:{}:{} date={:?} sats={} hdop_centi={:?}",
                gps.num_fixes,
                now.saturating_duration_since(fix.system_timestamp)
                    .as_micros(),
                fix.system_timestamp.as_micros(),
                fix.utc_time.time.hour,
                fix.utc_time.time.minute,
                fix.utc_time.time.second,
                fix.utc_time.date,
                fix.satellites,
                fix.hdop_centi
            ));
        } else {
            record_outcome(log_info!(gps_log, "FIX none received"));
        }
        if let (Some(fix), Some(pps)) = (latest_fix, latest_pps) {
            let candidate_offset_us = signed_instant_delta_us(fix.system_timestamp, pps.timestamp);
            record_outcome(log_info!(
                gps_log,
                "PAIR latest_fix_minus_pps_us={} within_window={} fix_count={} pps_count={}",
                candidate_offset_us,
                (0..=750_000).contains(&candidate_offset_us),
                gps.num_fixes,
                pps.pps_count
            ));
        }
        if let Some(correlation) = common::GPS_RESOURCES.latest_time_correlation() {
            let pps_systime_us = correlation.pps_timestamp.map(|value| value.as_micros());
            let pair_offset_us = correlation
                .pps_timestamp
                .map(|pps| signed_instant_delta_us(correlation.local_timestamp, pps));
            record_outcome(log_info!(
                gps_log,
                "CORR age_us={} nmea_systime_us={} pps_systime_us={:?} offset_us={:?} source={:?} capture_ticks={:?} delta_ticks={:?} capture_hz={:?}",
                now.saturating_duration_since(correlation.local_timestamp)
                    .as_micros(),
                correlation.local_timestamp.as_micros(),
                pps_systime_us,
                pair_offset_us,
                correlation.pps_timing_source,
                correlation.pps_capture_ticks,
                correlation.pps_capture_delta_ticks,
                correlation.pps_capture_frequency_hz
            ));
        } else {
            record_outcome(log_info!(gps_log, "CORR none emitted"));
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

fn signed_instant_delta_us(value: Instant, reference: Instant) -> i64 {
    if value >= reference {
        value
            .saturating_duration_since(reference)
            .as_micros()
            .min(i64::MAX as u64) as i64
    } else {
        -(reference
            .saturating_duration_since(value)
            .as_micros()
            .min(i64::MAX as u64) as i64)
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
            signal_severe_error();
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
        signal_severe_error();
    }
}

async fn fail_forever<E: defmt::Format>(message: &str, value: E) -> ! {
    error!("{}: {}", message, value);
    signal_severe_error();
    common::pending_forever().await
}
