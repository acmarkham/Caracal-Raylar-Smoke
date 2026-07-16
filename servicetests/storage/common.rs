use alloc::boxed::Box;
use defmt::{error, info, unwrap};
use embassy_executor::Spawner;
use embassy_stm32::gpio::Output;
use embassy_stm32::rcc::mux::Sdmmcsel;
use embassy_stm32::rcc::*;
use embassy_stm32::sdmmc::sd::{CmdBlock, StorageDevice};
use embassy_stm32::sdmmc::{Config as SdmmcConfig, Sdmmc};
use embassy_stm32::time::{mhz, Hertz};
use embassy_stm32::usart::{BufferedUart, Config as UartConfig, DataBits, Parity, StopBits};
use embassy_time::Timer;
use raylar_board_v1p0::{Gps, Irqs, SdCard};
use raylar_drivers::gps::stm32::{ExtiPps, Stm32GpsPower};
use raylar_drivers::gps::{GpsCommand, GpsConfig, GpsDriver, GpsResources};
use raylar_drivers::storage::stm32::Stm32SdBlockDevice;
use raylar_drivers::storage::{
    detect_exfat_volume, FileHandle, PartitionedBlockDevice, StorageDriver,
};
use raylar_storage_service::StorageBackend;
use raylar_time_service::gps::run_gps_time_source;
use raylar_time_service::{TimeConfig, TimeResources, TimeService};

pub static GPS_RESOURCES: GpsResources = GpsResources::new();
pub static TIME_RESOURCES: TimeResources<4, 8> = TimeResources::new();
const SD_TARGET_FREQ: Hertz = mhz(24);

/// Keeps the active-low SD power GPIO configured for as long as the filesystem
/// backend exists. Dropping an Embassy `Output` disconnects the pin, which can
/// remove card power after volume detection but before filesystem mounting.
struct PoweredStorage<B> {
    inner: B,
    _power: Output<'static>,
}

impl<B, const BLOCK_SIZE: usize> StorageBackend<BLOCK_SIZE> for PoweredStorage<B>
where
    B: StorageBackend<BLOCK_SIZE>,
{
    type Error = B::Error;

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
    config.rcc.sys = Sysclk::PLL1_R;
    config.rcc.hsi48 = Some(Hsi48Config::new());
    config.rcc.mux.sdmmcsel = Sdmmcsel::PLL1_P;
    config
}

pub async fn start_time(spawner: Spawner, gps: Gps<'static>) {
    let Gps {
        usart,
        tx,
        rx,
        pps,
        rst,
        en,
        ..
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

    let driver = GpsDriver::new(
        uart,
        ExtiPps::new(pps),
        Stm32GpsPower::new(en, rst),
        &GPS_RESOURCES,
        GpsConfig::default(),
    );
    let time = TimeService::new(&TIME_RESOURCES, TimeConfig::default());
    let correlations = unwrap!(GPS_RESOURCES.time_receiver()).as_dyn();
    spawner.spawn(unwrap!(gps_driver_task(driver)));
    spawner.spawn(unwrap!(time_service_task(time)));
    spawner.spawn(unwrap!(gps_time_source_task(correlations)));
    GPS_RESOURCES.command_sender().send(GpsCommand::Start).await;
}

pub async fn storage_driver(
    sd: SdCard<'static>,
) -> impl raylar_storage_service::StorageBackend<512, Error: defmt::Format> {
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
        pending_forever().await;
    }
    info!("SD init phase 1 complete: card detected (SD_SW low)");

    info!("SD init phase 2: constructing SDMMC 4-bit peripheral");
    let mut config = SdmmcConfig::default();
    config.data_transfer_timeout = 120_000_000;
    let sdmmc = Box::leak(Box::new(Sdmmc::new_4bit(
        sdmmc, Irqs, clk, cmd, d0, d1, d2, d3, config,
    )));
    let cmd_block = Box::leak(Box::new(CmdBlock::new()));
    info!("SD init phase 2 complete: SDMMC peripheral constructed");

    info!("SD init phase 3: enabling card power and waiting 1 second");
    power.set_low();
    Timer::after_secs(1).await;
    info!("SD init phase 3 complete: card power settling delay elapsed");

    info!(
        "SD init phase 4: initializing SD card at {} Hz",
        SD_TARGET_FREQ.0
    );
    let card = match StorageDevice::new_sd_card(sdmmc, cmd_block, SD_TARGET_FREQ).await {
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
            pending_forever().await
        }
    };

    info!("SD init phase 5: wrapping card as a 512-byte block device");
    let mut device = Stm32SdBlockDevice::new(card);
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
            pending_forever().await
        }
    };
    info!("SD init phase 7: constructing powered partitioned storage driver");
    let inner = StorageDriver::<_>::new(PartitionedBlockDevice::new(device, volume));
    info!("SD init phase 7 complete: driver owns SD power pin and is ready to mount");
    PoweredStorage {
        inner,
        _power: power,
    }
}

#[embassy_executor::task]
async fn gps_driver_task(driver: GpsDriver<BufferedUart<'static>, ExtiPps, Stm32GpsPower>) -> ! {
    driver.run().await
}

#[embassy_executor::task]
async fn time_service_task(service: TimeService<4, 8>) -> ! {
    service.run().await
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
