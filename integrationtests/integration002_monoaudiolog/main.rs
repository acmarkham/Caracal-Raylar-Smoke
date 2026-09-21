#![no_std]
#![no_main]

extern crate alloc;

#[path = "../../servicetests/storage/common.rs"]
#[allow(dead_code)] // The real and synthetic time paths are feature-exclusive.
mod common;

use core::cell::RefCell;
use core::future::Future;
use core::pin::Pin;
use core::sync::atomic::{AtomicBool, AtomicU32, Ordering};
use core::task::{Context, Poll};
use defmt::{error, info, unwrap};
use embassy_executor::Spawner;
use embassy_stm32::gpio::{Input, Pull};
use embassy_stm32::i2c::{mode::Master, Config as I2cConfig, I2c};
use embassy_stm32::mode::Blocking;
use embassy_stm32::peripherals::{PA0, PA1, PB1};
use embassy_stm32::time::Hertz;
use embassy_stm32::{bind_interrupts, peripherals};
use embassy_sync::blocking_mutex::raw::{CriticalSectionRawMutex, NoopRawMutex};
use embassy_sync::blocking_mutex::Mutex as BlockingMutex;
use embassy_sync::channel::Channel;
use embassy_sync::mutex::Mutex;
#[cfg(not(feature = "fake-gps-time"))]
use embassy_sync::pubsub::WaitResult;
use embassy_time::{Duration, Instant, Timer, TICK_HZ};
use embedded_alloc::LlffHeap as Heap;
use raylar_audio_recorder_service::{
    AudioRecorder, AudioRecorderConfig, AudioRecorderError, RecorderProgress, TimeMetadataSource,
};
use raylar_audiosource::{AudioFormat, AudioSource};
use raylar_board_v1p0::{AdcVoltages, Board, Leds, PdmMicArray, PdmMicDma, SensI2C, UsbCdc};
use raylar_drivers::batterycharger::{ChargerBus, ChargerConfig, ChargerDriver, ChargerResources};
use raylar_drivers::mic_array::stm32::{Dma0TimestampHandler, MonoPins, Stm32MonoMicrophoneDriver};
use raylar_drivers::mic_array::{
    MicrophoneConfig, MicrophoneMode, MicrophonePreset, MicrophoneResources,
};
use raylar_drivers::stm32_core::stm32::Stm32CoreDriver;
use raylar_drivers::stm32_core::{CoreConfig, CoreSupply, CoreSupplyControl};
use raylar_drivers::voltagemonitor::stm32::Stm32VoltageMonitor;
use raylar_drivers::voltagemonitor::{VoltageConfig, VoltageMonitorDriver, VoltageResources};
use raylar_drivers::{buzzer, leds};
use raylar_drivers::{sensor_acc, sensor_mag};
use raylar_location_service::{LocationConfig, LocationResources, LocationService, LocationState};
use raylar_logging_service::{
    info as log_info, LogOutcome, LogSink, LoggerHandle, LoggingResources, LoggingService,
    ProcessOutcome,
};
use raylar_power_management_service::{PowerConfig, PowerManagementService, PowerResources};
use raylar_sensor_service::{
    ReadingStatus, SensorDescriptor, SensorId, SensorKind, SensorOrigin, SensorRegistration,
    SensorResources, SensorService, SensorSource, SensorSourceError, SensorValue,
};
use raylar_storage_service::{
    StorageBackend, StorageLayout, StorageService, StorageServiceError, StreamHandle, StreamKind,
};
use raylar_time_service::TimeResources;
use raylar_versioning_service::{
    IdentityConfig, IdentityField, IdentityResources, IdentityState, IdentityVersioningService,
};
use static_cell::StaticCell;
use {defmt_rtt as _, panic_probe as _};

const HEAP_BYTES: usize = 64 * 1024;
// Preserve complete raw PPS/correlation records for post-hoc reconstruction.
const MESSAGE_LENGTH: usize = 384;
const QUEUE_DEPTH: usize = 16;
const LINE_LENGTH: usize = 512;
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
const AUDIO_PACKETS_PER_SECOND: u32 = (SAMPLE_RATE_HZ / HALF_SAMPLES) as u32;
const LOCATION_HISTORY: usize = 9;
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
type BoardSensorI2c = I2c<'static, Blocking, Master>;
type SensorBusMutex = BlockingMutex<NoopRawMutex, RefCell<BoardSensorI2c>>;
type AccelerometerDriver = sensor_acc::Lis2hh12<SharedSensorI2c>;
type MagnetometerDriver = sensor_mag::Lis2mdl<SharedSensorI2c>;
type AccelerometerMutex = BlockingMutex<NoopRawMutex, RefCell<AccelerometerDriver>>;
type MagnetometerMutex = BlockingMutex<NoopRawMutex, RefCell<MagnetometerDriver>>;
type BoardChargerDriver = ChargerDriver<SharedSensorI2c>;
type MicDriver = Stm32MonoMicrophoneDriver<'static, DMA_SAMPLES>;

const ACCELERATION_SENSOR: SensorId = SensorId(1);
const MAGNETIC_FIELD_SENSOR: SensorId = SensorId(2);
const ACCELEROMETER_TEMPERATURE_SENSOR: SensorId = SensorId(3);
const MAGNETOMETER_TEMPERATURE_SENSOR: SensorId = SensorId(4);

static VOLTAGES: VoltageResources = VoltageResources::new();
static CHARGER: ChargerResources = ChargerResources::new();
static POWER: PowerResources = PowerResources::new();
static LOCATION: LocationResources<4> = LocationResources::new();
static VERSIONING: IdentityResources<4> = IdentityResources::new();
static SENSORS: SensorResources = SensorResources::new();
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
static MIC_CAPTURE_PROFILE: CpuProfile = CpuProfile::new();
static AUDIO_FORWARD_PROFILE: CpuProfile = CpuProfile::new();
static AUDIO_RECORDER_PROFILE: CpuProfile = CpuProfile::new();
static AUDIO_STORAGE_PROFILE: CpuProfile = CpuProfile::new();
static LOGGING_PROFILE: CpuProfile = CpuProfile::new();
static LOG_STORAGE_PROFILE: CpuProfile = CpuProfile::new();
// Diagnostic INFO records are best-effort. A temporarily full telemetry queue
// must not turn storage latency at a recording boundary into a fatal error.
static DROPPED_INFO_DIAGNOSTICS: AtomicU32 = AtomicU32::new(0);
static SHARED_STORAGE: StaticCell<SharedStorage<common::BoardStorageBackend>> = StaticCell::new();
static SENSOR_BUS: StaticCell<SensorBusMutex> = StaticCell::new();
static ACCELEROMETER: StaticCell<AccelerometerMutex> = StaticCell::new();
static MAGNETOMETER: StaticCell<MagnetometerMutex> = StaticCell::new();
static ACCELERATION_SOURCE: StaticCell<AccelerometerSource> = StaticCell::new();
static ACCELEROMETER_TEMPERATURE_SOURCE: StaticCell<AccelerometerSource> = StaticCell::new();
static MAGNETIC_FIELD_SOURCE: StaticCell<MagnetometerSource> = StaticCell::new();
static MAGNETOMETER_TEMPERATURE_SOURCE: StaticCell<MagnetometerSource> = StaticCell::new();

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

/// Measures CPU time spent polling an async operation, excluding the time for
/// which it returns `Pending`. This is deliberately different from ordinary
/// elapsed timing: a 20 ms SD transfer that consumes 100 us to submit and
/// complete is charged about 100 us, not 20 ms.
struct CpuProfile {
    active_ticks: AtomicU32,
    polls: AtomicU32,
    completions: AtomicU32,
}

impl CpuProfile {
    const fn new() -> Self {
        Self {
            active_ticks: AtomicU32::new(0),
            polls: AtomicU32::new(0),
            completions: AtomicU32::new(0),
        }
    }

    fn instrument<F>(&'static self, future: F) -> Profiled<F> {
        Profiled {
            future,
            profile: self,
        }
    }

    fn enter(&'static self) -> CpuProfileGuard {
        CpuProfileGuard {
            profile: self,
            started_ticks: Instant::now().as_ticks() as u32,
        }
    }

    fn record(&self, started_ticks: u32, completed: bool) {
        let elapsed = (Instant::now().as_ticks() as u32).wrapping_sub(started_ticks);
        self.active_ticks.fetch_add(elapsed, Ordering::Relaxed);
        self.polls.fetch_add(1, Ordering::Relaxed);
        if completed {
            self.completions.fetch_add(1, Ordering::Relaxed);
        }
    }

    fn take(&self) -> CpuProfileSample {
        CpuProfileSample {
            active_ticks: self.active_ticks.swap(0, Ordering::AcqRel),
            polls: self.polls.swap(0, Ordering::AcqRel),
            completions: self.completions.swap(0, Ordering::AcqRel),
        }
    }
}

struct Profiled<F> {
    future: F,
    profile: &'static CpuProfile,
}

impl<F: Future> Future for Profiled<F> {
    type Output = F::Output;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let started_ticks = Instant::now().as_ticks() as u32;
        // SAFETY: projection never moves `future`; it remains pinned with its
        // containing `Profiled` value for the duration of the poll.
        let this = unsafe { self.get_unchecked_mut() };
        let result = unsafe { Pin::new_unchecked(&mut this.future) }.poll(cx);
        this.profile
            .record(started_ticks, matches!(result, Poll::Ready(_)));
        result
    }
}

struct CpuProfileGuard {
    profile: &'static CpuProfile,
    started_ticks: u32,
}

impl Drop for CpuProfileGuard {
    fn drop(&mut self) {
        self.profile.record(self.started_ticks, true);
    }
}

#[derive(Clone, Copy)]
struct CpuProfileSample {
    active_ticks: u32,
    polls: u32,
    completions: u32,
}

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

/// Synchronous shared access is sufficient here because all clients run on
/// the same thread-mode executor and no I2C operation yields. Interrupts do
/// not access this bus, so the no-op raw mutex avoids masking audio DMA IRQs.
#[derive(Clone, Copy)]
struct SharedSensorI2c {
    bus: &'static SensorBusMutex,
}

impl embedded_hal::i2c::ErrorType for SharedSensorI2c {
    type Error = embassy_stm32::i2c::Error;
}

impl embedded_hal::i2c::I2c for SharedSensorI2c {
    fn transaction(
        &mut self,
        address: u8,
        operations: &mut [embedded_hal::i2c::Operation<'_>],
    ) -> Result<(), Self::Error> {
        self.bus.lock(|bus| {
            embedded_hal::i2c::I2c::transaction(&mut *bus.borrow_mut(), address, operations)
        })
    }
}

impl ChargerBus for SharedSensorI2c {
    type Error = embassy_stm32::i2c::Error;

    fn read_register(&mut self, register: u8) -> Result<u8, Self::Error> {
        let mut value = [0];
        embedded_hal::i2c::I2c::write_read(
            self,
            raylar_drivers::batterycharger::BQ25186_ADDRESS,
            &[register],
            &mut value,
        )?;
        Ok(value[0])
    }

    fn write_register(&mut self, register: u8, value: u8) -> Result<(), Self::Error> {
        embedded_hal::i2c::I2c::write(
            self,
            raylar_drivers::batterycharger::BQ25186_ADDRESS,
            &[register, value],
        )
    }
}

#[derive(Clone, Copy)]
enum AccelerometerMeasurement {
    Acceleration,
    Temperature,
}

struct AccelerometerSource {
    driver: &'static AccelerometerMutex,
    measurement: AccelerometerMeasurement,
}

impl SensorSource for AccelerometerSource {
    fn sample(&mut self) -> Result<SensorValue, SensorSourceError> {
        self.driver.lock(|driver| {
            let mut driver = driver.borrow_mut();
            match self.measurement {
                AccelerometerMeasurement::Acceleration => driver
                    .read_acceleration()
                    .map(|value| SensorValue::AccelerationMg {
                        x: value.x_mg,
                        y: value.y_mg,
                        z: value.z_mg,
                    })
                    .map_err(|_| SensorSourceError::new(0x0101)),
                AccelerometerMeasurement::Temperature => driver
                    .read_die_temperature()
                    .map(|value| SensorValue::TemperatureMilliCelsius(value.milli_celsius))
                    .map_err(|_| SensorSourceError::new(0x0102)),
            }
        })
    }
}

#[derive(Clone, Copy)]
enum MagnetometerMeasurement {
    MagneticField,
    Temperature,
}

struct MagnetometerSource {
    driver: &'static MagnetometerMutex,
    measurement: MagnetometerMeasurement,
}

impl SensorSource for MagnetometerSource {
    fn sample(&mut self) -> Result<SensorValue, SensorSourceError> {
        self.driver.lock(|driver| {
            let mut driver = driver.borrow_mut();
            match self.measurement {
                MagnetometerMeasurement::MagneticField => driver
                    .read_magnetic_field()
                    .map(|value| SensorValue::MagneticFieldNt {
                        x: value.x_nanotesla,
                        y: value.y_nanotesla,
                        z: value.z_nanotesla,
                    })
                    .map_err(|_| SensorSourceError::new(0x0201)),
                MagnetometerMeasurement::Temperature => driver
                    .read_die_temperature()
                    .map(|value| SensorValue::TemperatureMilliCelsius(value.milli_celsius))
                    .map_err(|_| SensorSourceError::new(0x0202)),
            }
        })
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
        let started = Instant::now();
        info!("RTT audio boundary: begin stream start");
        let result = AUDIO_STORAGE_PROFILE
            .instrument(self.storage.begin(StreamKind::Audio, layout))
            .await;
        let elapsed_us = Instant::now()
            .saturating_duration_since(started)
            .as_micros();
        match &result {
            Ok(_) => info!(
                "RTT audio boundary: begin stream complete elapsed_us={}",
                elapsed_us
            ),
            Err(_) => error!(
                "RTT audio boundary: begin stream failed elapsed_us={}",
                elapsed_us
            ),
        }
        result
    }
    async fn append_audio(&mut self, stream: Self::Handle, data: &[u8]) -> Result<(), Self::Error> {
        // WAV headers are one storage block; PCM writes use the larger
        // recorder buffer. Trace only this first append, not every audio write.
        if data.len() != 512 {
            return AUDIO_STORAGE_PROFILE
                .instrument(self.storage.write(stream, data))
                .await;
        }

        let started = Instant::now();
        info!(
            "RTT audio boundary: header append start bytes={}",
            data.len()
        );
        let result = AUDIO_STORAGE_PROFILE
            .instrument(self.storage.write(stream, data))
            .await;
        let elapsed_us = Instant::now()
            .saturating_duration_since(started)
            .as_micros();
        match &result {
            Ok(_) => info!(
                "RTT audio boundary: header append complete elapsed_us={}",
                elapsed_us
            ),
            Err(_) => error!(
                "RTT audio boundary: header append failed elapsed_us={}",
                elapsed_us
            ),
        }
        result
    }
    async fn finish_audio(&mut self, stream: Self::Handle) -> Result<(), Self::Error> {
        let started = Instant::now();
        info!("RTT audio boundary: finish stream start");
        let result = AUDIO_STORAGE_PROFILE
            .instrument(self.storage.finish(stream))
            .await;
        let elapsed_us = Instant::now()
            .saturating_duration_since(started)
            .as_micros();
        match &result {
            Ok(_) => info!(
                "RTT audio boundary: finish stream complete elapsed_us={}",
                elapsed_us
            ),
            Err(_) => error!(
                "RTT audio boundary: finish stream failed elapsed_us={}",
                elapsed_us
            ),
        }
        result
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
        LOG_STORAGE_PROFILE
            .instrument(self.storage.write(self.stream, data))
            .await
    }
    async fn flush(&mut self) -> Result<(), Self::Error> {
        LOG_STORAGE_PROFILE
            .instrument(self.storage.flush(self.stream))
            .await
    }
    async fn checkpoint(&mut self) -> Result<(), Self::Error> {
        LOG_STORAGE_PROFILE
            .instrument(self.storage.checkpoint(self.stream))
            .await
    }
}

#[embassy_executor::main]
async fn main(spawner: Spawner) -> ! {
    unsafe {
        embedded_alloc::init!(HEAP, HEAP_BYTES);
    }
    let peripherals = embassy_stm32::init(common::mcu_config());
    let core_supply = if cfg!(feature = "core-smps") {
        CoreSupply::Smps
    } else {
        CoreSupply::Ldo
    };
    let core_driver = unwrap!(Stm32CoreDriver::init(CoreConfig {
        supply: core_supply,
    }));
    info!(
        "STM32 core supply selected: {:?}",
        core_driver.selected_supply()
    );
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
    } = Board::new(peripherals);
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
    let location_service = LocationService::<4, LOCATION_HISTORY>::new(
        &LOCATION,
        unwrap!(common::GPS_RESOURCES.fix_receiver()).as_dyn(),
        LocationConfig::default(),
    );
    spawner.spawn(unwrap!(location_service_task(location_service)));
    let sensor_bus = build_sensor_bus(sens_i2c);
    let sensor_service = build_sensor_service(sensor_bus).await;
    start_power(spawner, adc_voltages, sensor_bus, usb_cdc).await;
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
    let mut versioning_service =
        IdentityVersioningService::new(&VERSIONING, IdentityConfig::default());
    let sd_card_identity = storage
        .device_identity()
        .map(|identity| IdentityField::Known(identity.into()))
        .unwrap_or(IdentityField::Unavailable);
    versioning_service.set_sd_card_identity(sd_card_identity);
    // Publish the complete device/firmware/card snapshot before opening the
    // system log. The service owns future GPS/radio/card identity updates.
    versioning_service.publish();
    spawner.spawn(unwrap!(identity_versioning_task(versioning_service)));
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
    let location_log = logging.register("Location");
    let sensor_log = logging.register("Sensor");
    #[cfg(not(feature = "fake-gps-time"))]
    let pps_log = logging.register("Pps");
    #[cfg(not(feature = "fake-gps-time"))]
    let correlation_log = logging.register("GpsCorr");
    let audio_log = logging.register("Audio");
    log_versioning(system_log, VERSIONING.state());
    record_outcome(log_info!(
        system_log,
        "integration002 monoaudiolog started; format={}Hz mono, 60-second WAV files in hourly folders",
        SAMPLE_RATE_HZ
    ));
    // Commit startup records before audio startup. This makes /syslog.txt
    // visible even if GPS acquisition or microphone capture subsequently
    // stalls.
    loop {
        match logging.process_one().await {
            Ok(ProcessOutcome::Written) => {}
            Ok(ProcessOutcome::Empty) => break,
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
    spawner.spawn(unwrap!(location_logger_task(location_log)));
    spawner.spawn(unwrap!(sensor_logger_task(sensor_log)));
    spawner.spawn(unwrap!(sensor_service_task(sensor_service)));
    #[cfg(not(feature = "fake-gps-time"))]
    {
        spawner.spawn(unwrap!(pps_logger_task(pps_log)));
        spawner.spawn(unwrap!(correlation_logger_task(correlation_log)));
    }
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

fn log_versioning(system_log: TestLogger, state: IdentityState) {
    match (
        state.device.stm32_uid_96,
        state.device.serial_64,
        state.device.serial_48,
        state.device.serial_32,
        state.device.serial_16,
    ) {
        (
            IdentityField::Known(uid),
            IdentityField::Known(serial_64),
            IdentityField::Known(serial_48),
            IdentityField::Known(serial_32),
            IdentityField::Known(serial_16),
        ) => record_outcome(log_info!(
            system_log,
            "versioning device uuid={:08X}-{:08X}-{:08X} serial64={:016X} serial48={:012X} serial32={:08X} serial16={:04X} stm32={:?}",
            uid.word0,
            uid.word1,
            uid.word2,
            serial_64,
            serial_48,
            serial_32,
            serial_16,
            state.device.stm32_device_code
        )),
        _ => record_outcome(log_info!(
            system_log,
            "versioning device uid={:?} serial64={:?} serial48={:?} serial32={:?} serial16={:?} stm32={:?}",
            state.device.stm32_uid_96,
            state.device.serial_64,
            state.device.serial_48,
            state.device.serial_32,
            state.device.serial_16,
            state.device.stm32_device_code
        )),
    }
    record_outcome(log_info!(
        system_log,
        "versioning firmware version={:?} git_hash={:?} build_timestamp={:?} profile={:?} runtime_crc32={:?} build_crc32={:?}",
        state.firmware.version,
        state.firmware.git_hash,
        state.firmware.build_timestamp,
        state.firmware.build_profile,
        state.firmware.runtime_crc32,
        state.firmware.build_crc32
    ));
    record_outcome(log_info!(
        system_log,
        "versioning board_revision={:?}",
        state.hardware.board_revision
    ));
    record_outcome(log_info!(
        system_log,
        "versioning sd_card={:?}",
        state.hardware.sd_card
    ));
    record_outcome(log_info!(
        system_log,
        "versioning gps_module={:?}",
        state.hardware.gps_module
    ));
    record_outcome(log_info!(
        system_log,
        "versioning radio_module={:?}",
        state.hardware.radio_module
    ));
}

async fn start_power(
    spawner: Spawner,
    adc: AdcVoltages<'static>,
    sensor_bus: SharedSensorI2c,
    usb: UsbCdc<'static>,
) {
    let service = PowerManagementService::new(
        &POWER,
        unwrap!(VOLTAGES.state_receiver()).as_dyn(),
        unwrap!(CHARGER.state_receiver()).as_dyn(),
        PowerConfig::default(),
    );
    spawner.spawn(unwrap!(voltage_task(build_voltage_driver(adc, usb))));
    spawner.spawn(unwrap!(charger_task(build_charger_driver(sensor_bus))));
    spawner.spawn(unwrap!(power_service_task(service)));
}

fn build_sensor_bus(sens: SensI2C<'static>) -> SharedSensorI2c {
    let SensI2C { i2c, scl, sda } = sens;
    let mut config = I2cConfig::default();
    config.frequency = Hertz(100_000);
    let bus = SENSOR_BUS.init(BlockingMutex::new(RefCell::new(I2c::new_blocking(
        i2c, scl, sda, config,
    ))));
    SharedSensorI2c { bus }
}

async fn build_sensor_service(sensor_bus: SharedSensorI2c) -> SensorService<'static> {
    let mut accelerometer = match sensor_acc::Lis2hh12::new(sensor_bus) {
        Ok(driver) => driver,
        Err(error) => fail_forever("LIS2HH12 initialization failed", error).await,
    };
    if let Err(error) = accelerometer.turn_on() {
        fail_forever("LIS2HH12 enable failed", error).await;
    }
    let mut magnetometer = match sensor_mag::Lis2mdl::new(sensor_bus) {
        Ok(driver) => driver,
        Err(error) => fail_forever("LIS2MDL initialization failed", error).await,
    };
    if let Err(error) = magnetometer.turn_on() {
        fail_forever("LIS2MDL enable failed", error).await;
    }

    let accelerometer = ACCELEROMETER.init(BlockingMutex::new(RefCell::new(accelerometer)));
    let magnetometer = MAGNETOMETER.init(BlockingMutex::new(RefCell::new(magnetometer)));
    let acceleration = ACCELERATION_SOURCE.init(AccelerometerSource {
        driver: accelerometer,
        measurement: AccelerometerMeasurement::Acceleration,
    });
    let accelerometer_temperature = ACCELEROMETER_TEMPERATURE_SOURCE.init(AccelerometerSource {
        driver: accelerometer,
        measurement: AccelerometerMeasurement::Temperature,
    });
    let magnetic_field = MAGNETIC_FIELD_SOURCE.init(MagnetometerSource {
        driver: magnetometer,
        measurement: MagnetometerMeasurement::MagneticField,
    });
    let magnetometer_temperature = MAGNETOMETER_TEMPERATURE_SOURCE.init(MagnetometerSource {
        driver: magnetometer,
        measurement: MagnetometerMeasurement::Temperature,
    });

    let mut service: SensorService<'static> = SensorService::new(&SENSORS);
    assert!(service
        .register_source(
            SensorRegistration::new(
                SensorDescriptor {
                    id: ACCELERATION_SENSOR,
                    kind: SensorKind::Acceleration,
                    origin: SensorOrigin::Lis2hh12,
                },
                acceleration,
            )
            .with_interval(Duration::from_secs(10)),
        )
        .is_ok());
    assert!(service
        .register_source(
            SensorRegistration::new(
                SensorDescriptor {
                    id: MAGNETIC_FIELD_SENSOR,
                    kind: SensorKind::MagneticField,
                    origin: SensorOrigin::Lis2mdl,
                },
                magnetic_field,
            )
            .with_interval(Duration::from_secs(10)),
        )
        .is_ok());
    assert!(service
        .register_source(
            SensorRegistration::new(
                SensorDescriptor {
                    id: ACCELEROMETER_TEMPERATURE_SENSOR,
                    kind: SensorKind::Temperature,
                    origin: SensorOrigin::Lis2hh12,
                },
                accelerometer_temperature,
            )
            .with_interval(Duration::from_secs(30)),
        )
        .is_ok());
    assert!(service
        .register_source(
            SensorRegistration::new(
                SensorDescriptor {
                    id: MAGNETOMETER_TEMPERATURE_SENSOR,
                    kind: SensorKind::Temperature,
                    origin: SensorOrigin::Lis2mdl,
                },
                magnetometer_temperature,
            )
            .with_interval(Duration::from_secs(30)),
        )
        .is_ok());
    service
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

fn build_charger_driver(sensor_bus: SharedSensorI2c) -> BoardChargerDriver {
    let mut charger_config = ChargerConfig::default();
    charger_config.default_charge_current_ma = 200;
    charger_config.default_input_current_limit_ma = 200;
    ChargerDriver::new(sensor_bus, &CHARGER, charger_config)
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
    MIC_CAPTURE_PROFILE.instrument(driver.run()).await
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
    let mut report_windows = 0u32;
    let mut report_elapsed_ticks = 0u32;
    let mut report_active_ticks = 0u32;
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
        report_windows += 1;
        report_elapsed_ticks = report_elapsed_ticks.saturating_add(elapsed_ticks);
        report_active_ticks = report_active_ticks.saturating_add(active_ticks);
        if report_windows == 5 {
            report_cpu_profiles(report_elapsed_ticks, report_active_ticks);
            report_windows = 0;
            report_elapsed_ticks = 0;
            report_active_ticks = 0;
        }
        previous_ticks = now;
    }
}

fn report_cpu_profiles(elapsed_ticks: u32, active_ticks: u32) {
    let mic = MIC_CAPTURE_PROFILE.take();
    let forward = AUDIO_FORWARD_PROFILE.take();
    let recorder = AUDIO_RECORDER_PROFILE.take();
    let logging = LOGGING_PROFILE.take();
    let audio_storage = AUDIO_STORAGE_PROFILE.take();
    let log_storage = LOG_STORAGE_PROFILE.take();

    // The first four profiles are disjoint executor work. Storage profiles are
    // nested subsets and are intentionally not added again.
    let attributed_ticks = mic
        .active_ticks
        .saturating_add(forward.active_ticks)
        .saturating_add(recorder.active_ticks)
        .saturating_add(logging.active_ticks);
    let other_ticks = active_ticks.saturating_sub(attributed_ticks);
    info!(
        "cpu profile window_ms={} active={}.{}% mic_dma={}.{}%/{}/{} audio_forward={}.{}%/{}/{} audio_recorder={}.{}%/{}/{} logging={}.{}%/{}/{} other={}.{}% nested_audio_storage={}.{}%/{}/{} nested_log_storage={}.{}%/{}/{} (percent/calls/polls)",
        ticks_to_millis(elapsed_ticks),
        tenths_percent(active_ticks, elapsed_ticks) / 10,
        tenths_percent(active_ticks, elapsed_ticks) % 10,
        tenths_percent(mic.active_ticks, elapsed_ticks) / 10,
        tenths_percent(mic.active_ticks, elapsed_ticks) % 10,
        mic.completions,
        mic.polls,
        tenths_percent(forward.active_ticks, elapsed_ticks) / 10,
        tenths_percent(forward.active_ticks, elapsed_ticks) % 10,
        forward.completions,
        forward.polls,
        tenths_percent(recorder.active_ticks, elapsed_ticks) / 10,
        tenths_percent(recorder.active_ticks, elapsed_ticks) % 10,
        recorder.completions,
        recorder.polls,
        tenths_percent(logging.active_ticks, elapsed_ticks) / 10,
        tenths_percent(logging.active_ticks, elapsed_ticks) % 10,
        logging.completions,
        logging.polls,
        tenths_percent(other_ticks, elapsed_ticks) / 10,
        tenths_percent(other_ticks, elapsed_ticks) % 10,
        tenths_percent(audio_storage.active_ticks, elapsed_ticks) / 10,
        tenths_percent(audio_storage.active_ticks, elapsed_ticks) % 10,
        audio_storage.completions,
        audio_storage.polls,
        tenths_percent(log_storage.active_ticks, elapsed_ticks) / 10,
        tenths_percent(log_storage.active_ticks, elapsed_ticks) % 10,
        log_storage.completions,
        log_storage.polls,
    );
}

fn tenths_percent(ticks: u32, elapsed_ticks: u32) -> u32 {
    if elapsed_ticks == 0 {
        0
    } else {
        ((u64::from(ticks) * 1_000) / u64::from(elapsed_ticks)) as u32
    }
}

fn ticks_to_millis(ticks: u32) -> u64 {
    u64::from(ticks).saturating_mul(1_000) / TICK_HZ
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
        match AUDIO_RECORDER_PROFILE
            .instrument(recorder.record_next())
            .await
        {
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
    if let Err(error) = LOGGING_PROFILE.instrument(logging.checkpoint()).await {
        error!("system log checkpoint failed: {}", error);
        signal_severe_error();
    }
    let stats = logging.stats();
    let dropped_info = DROPPED_INFO_DIAGNOSTICS.swap(0, Ordering::AcqRel);
    info!(
        "system log stats: total={} dropped={} depth={} max_depth={} bytes={} truncated={} write_failures={} nonfatal_info_drops_since_checkpoint={}",
        stats.total_messages,
        stats.dropped_messages,
        stats.queue_depth,
        stats.maximum_queue_depth,
        stats.bytes_written,
        stats.truncated_messages,
        stats.write_failures,
        dropped_info,
    );
    if stats.write_failures != 0 {
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
        match LOGGING_PROFILE.instrument(logging.process_one()).await {
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
async fn location_service_task(service: LocationService<4, LOCATION_HISTORY>) -> ! {
    service.run().await
}

#[embassy_executor::task]
async fn identity_versioning_task(service: IdentityVersioningService<4>) -> ! {
    service.run().await
}

#[embassy_executor::task]
async fn sensor_service_task(service: SensorService<'static>) -> ! {
    service.run().await
}

#[embassy_executor::task]
async fn sensor_logger_task(sensor_log: TestLogger) -> ! {
    let mut snapshots = unwrap!(SENSORS.state_receiver());
    let ids = [
        ACCELERATION_SENSOR,
        MAGNETIC_FIELD_SENSOR,
        ACCELEROMETER_TEMPERATURE_SENSOR,
        MAGNETOMETER_TEMPERATURE_SENSOR,
    ];
    let mut observed_attempts = [0u64; 4];
    loop {
        let snapshot = snapshots.changed().await;
        for (index, id) in ids.iter().enumerate() {
            let Some(reading) = snapshot.reading(*id) else {
                continue;
            };
            let attempts = reading.sequence.saturating_add(reading.total_errors);
            if attempts == observed_attempts[index] {
                continue;
            }
            observed_attempts[index] = attempts;
            match (reading.status, reading.value) {
                (
                    ReadingStatus::Current,
                    Some(SensorValue::AccelerationMg { x, y, z }),
                ) => record_outcome(log_info!(
                    sensor_log,
                    "raw acceleration x_mg={} y_mg={} z_mg={} sample_ticks={}",
                    x,
                    y,
                    z,
                    reading.last_attempt.as_ticks()
                )),
                (
                    ReadingStatus::Current,
                    Some(SensorValue::MagneticFieldNt { x, y, z }),
                ) => record_outcome(log_info!(
                    sensor_log,
                    "raw magnetic_field x_nt={} y_nt={} z_nt={} sample_ticks={}",
                    x,
                    y,
                    z,
                    reading.last_attempt.as_ticks()
                )),
                (ReadingStatus::Current, Some(SensorValue::TemperatureMilliCelsius(value))) => {
                    record_outcome(log_info!(
                        sensor_log,
                        "die_temperature origin={:?} milli_celsius={} sample_ticks={}",
                        reading.descriptor.origin,
                        value,
                        reading.last_attempt.as_ticks()
                    ));
                }
                _ => record_outcome(log_info!(
                    sensor_log,
                    "sample_failed id={} origin={:?} kind={:?} status={:?} consecutive_errors={} total_errors={} last_error={:?} sample_ticks={}",
                    reading.descriptor.id.0,
                    reading.descriptor.origin,
                    reading.descriptor.kind,
                    reading.status,
                    reading.consecutive_errors,
                    reading.total_errors,
                    reading.last_error,
                    reading.last_attempt.as_ticks()
                )),
            }
        }
    }
}

#[embassy_executor::task]
async fn location_logger_task(location_log: TestLogger) -> ! {
    let mut states = unwrap!(LOCATION.state_receiver());
    let first = loop {
        if SEVERE_ERROR_ACTIVE.load(Ordering::Acquire) {
            common::pending_forever().await;
        }

        let current = LOCATION.state();
        if current.valid {
            break current;
        }
        let changed = states.changed().await;
        if changed.valid {
            break changed;
        }
    };

    log_location(location_log, "acquired", first);
    loop {
        Timer::after_secs(60).await;
        if SEVERE_ERROR_ACTIVE.load(Ordering::Acquire) {
            common::pending_forever().await;
        }
        log_location(location_log, "periodic", LOCATION.state());
    }
}

fn log_location(location_log: TestLogger, event: &'static str, state: LocationState) {
    let fix_age_us = Instant::now()
        .saturating_duration_since(state.last_fix_system_time)
        .as_micros();
    record_outcome(log_info!(
        location_log,
        "event={} valid={} source={:?} lat_e7={} lon_e7={} fix_age_us={} fixes_used={} fixes_seen={} sats={:?} hdop_centi={:?} uncertainty_m={:?} fix_utc={:?}",
        event,
        state.valid,
        state.source,
        state.latitude.degrees_e7,
        state.longitude.degrees_e7,
        fix_age_us,
        state.fix_count_used,
        state.total_fix_count_seen,
        state.satellites,
        state.hdop_centi,
        state.uncertainty_meters,
        state.last_fix_utc_time
    ));
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
                "UTC {} src={:?} first={:?} status={:?} map_ppb={} cal_ppb={} cal_n={} cal_lock={} slew_ppb={} residual_us={:?} uncertainty_us={} holdover_us={} holdover_warn={} anchors={}/{} utc_fix={}",
                utc.seconds,
                time.active_time_source,
                time.first_anchor_source,
                time.utc_status,
                time.estimated_frequency_error_ppb,
                time.calibrated_frequency_error_ppb,
                time.frequency_calibration_samples,
                time.frequency_calibration_locked,
                time.phase_slew_ppb,
                time.last_anchor_residual_us,
                time.uncertainty_us,
                time.holdover_duration.as_micros(),
                time.holdover_warning,
                time.accepted_anchors,
                time.rejected_anchors,
                time.utc_second_corrections
            )),
            Err(_) => record_outcome(log_info!(
                time_log,
                "UTC unavailable src={:?} first={:?} status={:?} map_ppb={} cal_ppb={} cal_n={} cal_lock={} slew_ppb={} residual_us={:?} uncertainty_us={} holdover_us={} holdover_warn={} anchors={}/{} utc_fix={}",
                time.active_time_source,
                time.first_anchor_source,
                time.utc_status,
                time.estimated_frequency_error_ppb,
                time.calibrated_frequency_error_ppb,
                time.frequency_calibration_samples,
                time.frequency_calibration_locked,
                time.phase_slew_ppb,
                time.last_anchor_residual_us,
                time.uncertainty_us,
                time.holdover_duration.as_micros(),
                time.holdover_warning,
                time.accepted_anchors,
                time.rejected_anchors,
                time.utc_second_corrections
            )),
        }
        record_outcome(log_info!(
            time_log,
            "PPS_GATE active={} clean_intervals={} gate_rejections={}",
            time.pps_reacquisition_active,
            time.pps_reacquisition_clean_intervals,
            time.pps_reacquisition_rejections
        ));
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

/// Persist every PPS edge, including edges for which no NMEA sentence is
/// matched. These raw stamps permit offline oscillator and UTC reconstruction.
#[embassy_executor::task]
#[cfg(not(feature = "fake-gps-time"))]
async fn pps_logger_task(pps_log: TestLogger) -> ! {
    let mut events = unwrap!(common::GPS_RESOURCES.pps_event_subscriber());
    loop {
        let pps = match events.next_message().await {
            WaitResult::Message(pps) => pps,
            WaitResult::Lagged(missed) => {
                record_outcome(log_info!(pps_log, "LOSS missed_edges={}", missed));
                continue;
            }
        };
        record_outcome(log_info!(
            pps_log,
            "EDGE count={} systime_us={} source={:?} capture_ticks={:?} delta_ticks={:?} capture_hz={:?} system_delta_us={:?}",
            pps.pps_count,
            pps.timestamp.as_micros(),
            pps.timing_source,
            pps.capture_ticks,
            pps.capture_delta_ticks,
            pps.capture_frequency_hz,
            pps.delta_time.map(|delta| delta.as_micros())
        ));
    }
}

/// Persist every NMEA time correlation rather than only the ten-second status
/// snapshot. Unmatched records are retained to diagnose second-label errors.
#[embassy_executor::task]
#[cfg(not(feature = "fake-gps-time"))]
async fn correlation_logger_task(correlation_log: TestLogger) -> ! {
    let mut correlations = unwrap!(common::GPS_RESOURCES.time_event_subscriber());
    let mut sequence = 0u64;
    loop {
        let correlation = match correlations.next_message().await {
            WaitResult::Message(correlation) => correlation,
            WaitResult::Lagged(missed) => {
                record_outcome(log_info!(
                    correlation_log,
                    "LOSS missed_correlations={}",
                    missed
                ));
                continue;
            }
        };
        sequence = sequence.saturating_add(1);
        let pps_systime_us = correlation.pps_timestamp.map(|value| value.as_micros());
        let offset_us = correlation
            .pps_timestamp
            .map(|pps| signed_instant_delta_us(correlation.local_timestamp, pps));
        record_outcome(log_info!(
            correlation_log,
            "PAIR seq={} pps_count={:?} utc_date={:?} utc={}:{}:{} nmea_us={} pps_us={:?} offset_us={:?} source={:?} capture_ticks={:?} delta_ticks={:?} capture_hz={:?}",
            sequence,
            correlation.pps_count,
            correlation.utc_time.date,
            correlation.utc_time.time.hour,
            correlation.utc_time.time.minute,
            correlation.utc_time.time.second,
            correlation.local_timestamp.as_micros(),
            pps_systime_us,
            offset_us,
            correlation.pps_timing_source,
            correlation.pps_capture_ticks,
            correlation.pps_capture_delta_ticks,
            correlation.pps_capture_frequency_hz
        ));
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
    let mut stamp_packets = 0u32;
    let mut stamp_samples = 0usize;
    let mut stamp_first_ticks = 0u64;
    loop {
        let state = frames.changed().await;
        let _cpu_profile = AUDIO_FORWARD_PROFILE.enter();
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
            stamp_packets = 0;
            stamp_samples = 0;
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
        if stamp_packets == 0 {
            stamp_first_ticks = state.completed_at_ticks;
        }
        stamp_packets = stamp_packets.saturating_add(1);
        stamp_samples = stamp_samples.saturating_add(channel.len());
        if stamp_packets >= AUDIO_PACKETS_PER_SECOND {
            record_outcome(audio_log.log_at(
                Instant::from_ticks(state.completed_at_ticks),
                raylar_logging_service::LogLevel::Info,
                format_args!(
                    "audio packets first_timestamp_ticks={} last_timestamp_ticks={} packets={} samples={}",
                    stamp_first_ticks,
                    state.completed_at_ticks,
                    stamp_packets,
                    stamp_samples
                ),
            ));
            stamp_packets = 0;
            stamp_samples = 0;
        }
    }
}

fn record_outcome(outcome: LogOutcome) {
    if matches!(outcome, LogOutcome::DroppedQueueFull) {
        DROPPED_INFO_DIAGNOSTICS.fetch_add(1, Ordering::Relaxed);
    }
}

async fn fail_forever<E: defmt::Format>(message: &str, value: E) -> ! {
    error!("{}: {}", message, value);
    signal_severe_error();
    common::pending_forever().await
}
