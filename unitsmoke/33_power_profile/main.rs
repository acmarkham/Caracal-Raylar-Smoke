//! Eleven-state board power profile.
//!
//! States 1-10 provide an eight-second steady measurement window after the red
//! entry marker has completed. State 11 is terminal STOP2 with HSE disabled.
//!
//!
//! #1: unit_smoke_17 settings (gpio analog, RF off, GPS off, SD off, HSE on, 80MHz)
//! #2: mount SD card (i.e. storage service active) but no explicit writes
//! #3: Sustained writing to SD card
//! #4: Write one 512-byte block to SD every second
//! #5: Unmount and turn SD card off with SD_PWR pin
//! #6: Mono mic enabled (MCO output) according to integration_test002 settings along with MDF filters+ DMA (just dump the data, don't process it)
//! #7: All mics enabled (both MCO active) along with MDF filters + DMA
//! #8: All Mics off
//! #9: GPS on
//! #10: GPS in standby
//! #11 (terminal state): HSE off, STM core into STOP mode
//!
//!

#![no_std]
#![no_main]

extern crate alloc;

use core::panic::PanicInfo;

use embassy_executor::Spawner;
use embassy_stm32::dma::Channel;
use embassy_stm32::gpio::Output;
use embassy_stm32::rcc::mux::Sdmmcsel;
use embassy_stm32::rcc::*;
use embassy_stm32::sdmmc::sd::{CmdBlock, StorageDevice};
use embassy_stm32::sdmmc::{Config as SdmmcConfig, Sdmmc};
use embassy_stm32::time::{mhz, Hertz};
use embassy_stm32::usart::{Config as UartConfig, DataBits, Parity, StopBits, UartTx};
use embassy_stm32::{bind_interrupts, peripherals};
use embassy_time::{Duration, Instant, Timer};
use embedded_alloc::LlffHeap as Heap;
use raylar_board_v1p0::{Board, EbyteRf, Gps, Leds, PdmMicArray, PdmMicDma, SdCard};
use raylar_drivers::mic_array::stm32::{
    Dma0TimestampHandler, Dma5TimestampHandler, DmaChannels, Pins, Stm32MicrophoneDriver,
};
use raylar_drivers::mic_array::{MicrophoneConfig, MicrophonePreset, MicrophoneResources};
use raylar_drivers::stm32_core::stm32::Stm32CoreDriver;
use raylar_drivers::stm32_core::{CoreConfig, CoreSupply};
use raylar_drivers::storage::stm32::Stm32SdBlockDevice;
use raylar_drivers::storage::{
    detect_exfat_volume, FileHandle, PartitionedBlockDevice, StorageDeviceIdentity, StorageDriver,
};
use raylar_storage_service::{StorageBackend, StorageLayout, StorageService, StreamKind, UtcClock};
use raylar_time_service::UtcTimestamp;
use static_cell::StaticCell;

const HEAP_BYTES: usize = 64 * 1024;
const STATE_DURATION: Duration = Duration::from_secs(8);
const SD_TARGET_FREQ: Hertz = mhz(24);
const HALF_SAMPLES: usize = 1_600;
const DMA_SAMPLES: usize = HALF_SAMPLES * 2;
const MIC_CONFIG: MicrophoneConfig =
    MicrophoneConfig::from_preset(MicrophonePreset::ReferenceSinc5_16KhzHiperf);
const WRITE_DATA: [u8; 4096] = [0xa5; 4096];
const WRITE_BLOCK: [u8; 512] = [0x5a; 512];

#[global_allocator]
static HEAP: Heap = Heap::empty();

static SDMMC: StaticCell<Sdmmc<'static>> = StaticCell::new();
static MICROPHONES: MicrophoneResources<DMA_SAMPLES> = MicrophoneResources::new();

#[defmt::global_logger]
struct DisabledLogger;

unsafe impl defmt::Logger for DisabledLogger {
    fn acquire() {}
    unsafe fn flush() {}
    unsafe fn release() {}
    unsafe fn write(_bytes: &[u8]) {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {
        cortex_m::asm::wfi();
    }
}

bind_interrupts!(struct MicIrqs {
    GPDMA1_CHANNEL0 => embassy_stm32::dma::InterruptHandler<peripherals::GPDMA1_CH0>, Dma0TimestampHandler;
    GPDMA1_CHANNEL1 => embassy_stm32::dma::InterruptHandler<peripherals::GPDMA1_CH1>;
    GPDMA1_CHANNEL2 => embassy_stm32::dma::InterruptHandler<peripherals::GPDMA1_CH2>;
    GPDMA1_CHANNEL3 => embassy_stm32::dma::InterruptHandler<peripherals::GPDMA1_CH3>;
    GPDMA1_CHANNEL4 => embassy_stm32::dma::InterruptHandler<peripherals::GPDMA1_CH4>;
    GPDMA1_CHANNEL5 => embassy_stm32::dma::InterruptHandler<peripherals::GPDMA1_CH5>, Dma5TimestampHandler;
});

type BoardStorageInner =
    StorageDriver<PartitionedBlockDevice<Stm32SdBlockDevice<'static, 'static>>>;
type BoardStorage = PoweredStorage<BoardStorageInner>;

struct PoweredStorage<B> {
    inner: B,
    power: Output<'static>,
}

impl<B> PoweredStorage<B> {
    fn power_off(self) -> Output<'static> {
        let Self { inner, mut power } = self;
        drop(inner);
        shutdown_sdmmc();
        power.set_high();
        power
    }
}

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

struct NoUtc;

impl UtcClock for NoUtc {
    fn current_utc(&self) -> Option<UtcTimestamp> {
        None
    }
}

#[embassy_executor::main]
async fn main(_spawner: Spawner) -> ! {
    unsafe {
        embedded_alloc::init!(HEAP, HEAP_BYTES);
    }

    let mut config = embassy_stm32::Config::default();
    config.rcc.msis = None;
    config.rcc.msik = None;
    config.rcc.hsi48 = None;
    config.enable_debug_during_sleep = false;
    config.rcc.hse = Some(Hse {
        freq: mhz(16),
        mode: HseMode::Oscillator,
    });
    config.rcc.pll1 = Some(Pll {
        source: PllSource::HSE,
        prediv: PllPreDiv::DIV1,
        mul: PllMul::MUL10,
        divp: Some(PllDiv::DIV2),
        divq: Some(PllDiv::DIV2),
        divr: Some(PllDiv::DIV2),
    });
    // Integration test 002's high-performance microphone preset needs a
    // 96 MHz PLL3-Q kernel clock. SYSCLK remains PLL1-R at 80 MHz.
    config.rcc.pll3 = Some(Pll {
        source: PllSource::HSE,
        prediv: PllPreDiv::DIV1,
        mul: PllMul::MUL12,
        divp: None,
        divq: Some(PllDiv::DIV2),
        divr: None,
    });
    config.rcc.sys = Sysclk::PLL1_R;
    config.rcc.mux.sdmmcsel = Sdmmcsel::PLL1_P;

    let board = Board::new(embassy_stm32::init(config));
    let core_supply = if cfg!(feature = "core-smps") {
        CoreSupply::Smps
    } else {
        CoreSupply::Ldo
    };
    let _core_driver = match Stm32CoreDriver::init(CoreConfig {
        supply: core_supply,
    }) {
        Ok(driver) => driver,
        Err(_) => panic!(),
    };
    set_pll3_enabled(false);

    let Board {
        leds,
        gps,
        pdm_mic_array,
        sd,
        ebyte_rf,
        ..
    } = board;
    let Leds {
        sys_gps_green,
        sys_gps_red,
        mut sys_main_red,
        sys_main_green,
        sys_sd_blue,
    } = leds;
    drop((sys_gps_green, sys_gps_red, sys_main_green, sys_sd_blue));

    let EbyteRf {
        mut cs, mut nrst, ..
    } = ebyte_rf;
    cs.set_high();
    nrst.set_low();

    let Gps {
        usart: gps_usart,
        tx: gps_tx,
        rst: mut gps_rst,
        en: mut gps_en,
        ..
    } = gps;
    gps_en.set_low();
    gps_rst.set_high();
    set_sd_detect_active(false);

    // Match unit smoke 17: give the GPS rail/control state time to settle,
    // then repeatedly request standby before taking the baseline reading.
    Timer::after_millis(2_500).await;
    // SAFETY: these duplicate tokens are used only by this short-lived UART.
    // It is dropped before the original USART2/PA2 tokens are used in state 10.
    let startup_usart = unsafe { gps_usart.clone_unchecked() };
    let startup_tx = unsafe { gps_tx.clone_unchecked() };
    let mut startup_uart = match UartTx::new_blocking(startup_usart, startup_tx, gps_uart_config())
    {
        Ok(uart) => uart,
        Err(_) => fail(&mut sys_main_red).await,
    };
    for attempt in 0..5 {
        if startup_uart.blocking_write(b"$PMTK161,0*28\r\n").is_err() {
            fail(&mut sys_main_red).await;
        }
        if attempt != 4 {
            Timer::after_millis(250).await;
        }
    }
    if startup_uart.blocking_flush().is_err() {
        fail(&mut sys_main_red).await;
    }
    drop(startup_uart);

    // #1: test-17 baseline, including SD/GPS/RF off and 80 MHz HSE SYSCLK.
    enter_state(&mut sys_main_red, 1).await;
    Timer::after(STATE_DURATION).await;

    // #2: the storage service is mounted and idle.
    let backend = match storage_driver(sd).await {
        Ok(backend) => backend,
        Err(()) => fail(&mut sys_main_red).await,
    };
    let mut storage: StorageService<BoardStorage, NoUtc> = match StorageService::new(backend, NoUtc)
    {
        Ok(storage) => storage,
        Err(_) => fail(&mut sys_main_red).await,
    };
    if storage.mount().await.is_err() {
        fail(&mut sys_main_red).await;
    }
    enter_state(&mut sys_main_red, 2).await;
    Timer::after(STATE_DURATION).await;

    // #3: continuously append full blocks for the complete measurement window.
    let stream = match storage
        .begin_stream(StreamKind::Log, StorageLayout::Flat)
        .await
    {
        Ok(stream) => stream,
        Err(_) => fail(&mut sys_main_red).await,
    };
    enter_state(&mut sys_main_red, 3).await;
    let write_deadline = Instant::now() + STATE_DURATION;
    while Instant::now() < write_deadline {
        if storage.write(stream, &WRITE_DATA).await.is_err() {
            fail(&mut sys_main_red).await;
        }
    }

    // #4: append exactly one service block per second for eight seconds.
    enter_state(&mut sys_main_red, 4).await;
    let periodic_start = Instant::now();
    for second in 0..STATE_DURATION.as_secs() {
        if second != 0 {
            Timer::at(periodic_start + Duration::from_secs(second)).await;
        }
        if storage.write(stream, &WRITE_BLOCK).await.is_err() {
            fail(&mut sys_main_red).await;
        }
    }
    Timer::at(periodic_start + STATE_DURATION).await;

    // #5: close the stream, release SDMMC/filesystem state, then remove card power.
    if storage.finish(stream).await.is_err() {
        fail(&mut sys_main_red).await;
    }
    let _sd_power = storage.into_inner().power_off();
    enter_state(&mut sys_main_red, 5).await;
    Timer::after(STATE_DURATION).await;

    // #6/#7: integration-002 audio settings, first CCK0/filter0/DMA0 only,
    // then both clocks and all six MDF filters/DMA channels. Data is discarded.
    set_pll3_enabled(true);
    let microphone = match microphone_driver(pdm_mic_array) {
        Ok(driver) => driver,
        Err(_) => fail(&mut sys_main_red).await,
    };
    enter_state(&mut sys_main_red, 6).await;
    microphone
        .run_staged_discarding(
            STATE_DURATION,
            || enter_state(&mut sys_main_red, 7),
            STATE_DURATION,
        )
        .await;
    set_pll3_enabled(false);

    // #8: the finite capture has disabled MDF/GPDMA and parked all mic pins.
    enter_state(&mut sys_main_red, 8).await;
    Timer::after(STATE_DURATION).await;

    // #9: GPS powered and released from reset.
    gps_en.set_high();
    gps_rst.set_high();
    enter_state(&mut sys_main_red, 9).await;
    Timer::after(STATE_DURATION).await;

    // #10: repeatedly request GPS standby, then drop USART so only GPS standby remains.
    let mut gps_uart = match UartTx::new_blocking(gps_usart, gps_tx, gps_uart_config()) {
        Ok(uart) => uart,
        Err(_) => fail(&mut sys_main_red).await,
    };
    for attempt in 0..4 {
        if gps_uart.blocking_write(b"$PMTK161,0*28\r\n").is_err() {
            fail(&mut sys_main_red).await;
        }
        if attempt != 3 {
            Timer::after_millis(250).await;
        }
    }
    if gps_uart.blocking_flush().is_err() {
        fail(&mut sys_main_red).await;
    }
    drop(gps_uart);
    enter_state(&mut sys_main_red, 10).await;
    Timer::after(STATE_DURATION).await;

    // #11: terminal marker, then move SYSCLK off HSE and enter STOP2 forever.
    enter_state(&mut sys_main_red, 11).await;
    sys_main_red.set_low();
    drop(sys_main_red);
    enter_terminal_stop2()
}

fn microphone_driver(
    pdm: PdmMicArray<'static>,
) -> Result<Stm32MicrophoneDriver<'static, DMA_SAMPLES>, raylar_drivers::mic_array::Error> {
    let PdmMicArray {
        cck0,
        sd0,
        cck1,
        sd1,
        sd2,
        sd3,
        dma:
            PdmMicDma {
                ch0,
                ch1,
                ch2,
                ch3,
                ch4,
                ch5,
            },
    } = pdm;
    Stm32MicrophoneDriver::new(
        Pins {
            cck0,
            sd0,
            cck1,
            sd1,
            sd2,
            sd3,
        },
        DmaChannels {
            ch0: Channel::new(ch0, MicIrqs),
            ch1: Channel::new(ch1, MicIrqs),
            ch2: Channel::new(ch2, MicIrqs),
            ch3: Channel::new(ch3, MicIrqs),
            ch4: Channel::new(ch4, MicIrqs),
            ch5: Channel::new(ch5, MicIrqs),
        },
        &MICROPHONES,
        MIC_CONFIG,
    )
}

async fn storage_driver(sd: SdCard<'static>) -> Result<BoardStorage, ()> {
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
    power.set_high();
    set_sd_detect_active(true);
    if switch.is_high() {
        return Err(());
    }

    let mut config = SdmmcConfig::default();
    config.data_transfer_timeout = 120_000_000;
    let sdmmc = SDMMC.init(Sdmmc::new_4bit(
        sdmmc,
        raylar_board_v1p0::Irqs,
        clk,
        cmd,
        d0,
        d1,
        d2,
        d3,
        config,
    ));
    let mut cmd_block = CmdBlock::new();
    power.set_low();
    Timer::after_secs(1).await;
    let card = StorageDevice::new_sd_card(sdmmc, &mut cmd_block, SD_TARGET_FREQ)
        .await
        .map_err(|_| ())?;
    let mut device = Stm32SdBlockDevice::new(card);
    let identity = device.device_identity();
    let volume = detect_exfat_volume(&mut device).await.map_err(|_| ())?;
    let inner = StorageDriver::new_with_device_identity(
        PartitionedBlockDevice::new(device, volume),
        Some(identity),
    );
    Ok(PoweredStorage { inner, power })
}

async fn enter_state(led: &mut Output<'static>, state: u8) {
    led.set_low();
    for _ in 0..state {
        led.set_high();
        Timer::after_millis(80).await;
        led.set_low();
        Timer::after_millis(120).await;
    }
    Timer::after_millis(500).await;
}

async fn fail(led: &mut Output<'static>) -> ! {
    loop {
        led.set_high();
        Timer::after_millis(40).await;
        led.set_low();
        Timer::after_millis(40).await;
    }
}

fn gps_uart_config() -> UartConfig {
    let mut config = UartConfig::default();
    config.baudrate = 9_600;
    config.data_bits = DataBits::DataBits8;
    config.parity = Parity::ParityNone;
    config.stop_bits = StopBits::STOP1;
    config
}

fn set_pll3_enabled(enabled: bool) {
    use embassy_stm32::pac::RCC;

    RCC.cr().modify(|w| w.set_pllon(2, enabled));
    while RCC.cr().read().pllrdy(2) != enabled {}
}

fn set_sd_detect_active(active: bool) {
    use embassy_stm32::pac::gpio::vals::{Moder, Pupdr};
    use embassy_stm32::pac::GPIOD;

    if active {
        GPIOD.pupdr().modify(|w| w.set_pupdr(4, Pupdr::PULL_UP));
        GPIOD.moder().modify(|w| w.set_moder(4, Moder::INPUT));
    } else {
        GPIOD.pupdr().modify(|w| w.set_pupdr(4, Pupdr::FLOATING));
        GPIOD.moder().modify(|w| w.set_moder(4, Moder::ANALOG));
    }
}

fn shutdown_sdmmc() {
    use embassy_stm32::pac::gpio::vals::{Moder, Pupdr};
    use embassy_stm32::pac::{GPIOC, GPIOD, RCC};

    RCC.ahb2enr1().modify(|w| w.set_sdmmc1en(false));
    for pin in 8..=12 {
        GPIOC.pupdr().modify(|w| w.set_pupdr(pin, Pupdr::FLOATING));
        GPIOC.moder().modify(|w| w.set_moder(pin, Moder::ANALOG));
    }
    GPIOD.pupdr().modify(|w| w.set_pupdr(2, Pupdr::FLOATING));
    GPIOD.moder().modify(|w| w.set_moder(2, Moder::ANALOG));
}

fn enter_terminal_stop2() -> ! {
    use embassy_stm32::pac::pwr::vals::Lpms;
    use embassy_stm32::pac::rcc::vals::Sw;
    use embassy_stm32::pac::{PWR, RCC};

    cortex_m::interrupt::disable();

    // MSIS is used only as the short-lived bridge needed to turn PLL1/HSE off.
    RCC.cr().modify(|w| w.set_msison(true));
    while !RCC.cr().read().msisrdy() {}
    RCC.cfgr1().modify(|w| w.set_sw(Sw::MSIS));
    while RCC.cfgr1().read().sws() != Sw::MSIS {}

    RCC.cr().modify(|w| {
        w.set_pllon(0, false);
        w.set_pllon(2, false);
    });
    while RCC.cr().read().pllrdy(0) || RCC.cr().read().pllrdy(2) {}
    RCC.cr().modify(|w| w.set_hseon(false));
    while RCC.cr().read().hserdy() {}

    PWR.cr1().modify(|w| w.set_lpms(Lpms::STOP2));
    let mut core = unsafe { cortex_m::Peripherals::steal() };
    core.SYST.disable_interrupt();
    core.SYST.disable_counter();
    cortex_m::peripheral::SCB::clear_pendsv();
    cortex_m::peripheral::SCB::clear_pendst();
    disable_and_clear_nvic();
    core.SCB.set_sleepdeep();
    loop {
        cortex_m::asm::dsb();
        cortex_m::asm::isb();
        cortex_m::asm::wfi();
    }
}

fn disable_and_clear_nvic() {
    // Cortex-M33 implements at most 240 external interrupt lines. Disable and
    // clear all eight 32-bit banks so a stale Embassy peripheral interrupt
    // cannot make terminal WFI return immediately instead of entering STOP2.
    const BANKS: usize = 8;
    const NVIC_ICER: *mut u32 = 0xe000_e180 as *mut u32;
    const NVIC_ICPR: *mut u32 = 0xe000_e280 as *mut u32;
    for bank in 0..BANKS {
        unsafe {
            core::ptr::write_volatile(NVIC_ICER.add(bank), u32::MAX);
            core::ptr::write_volatile(NVIC_ICPR.add(bank), u32::MAX);
        }
    }
}
