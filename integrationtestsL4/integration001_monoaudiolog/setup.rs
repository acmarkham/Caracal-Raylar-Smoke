//! Setup and ownership transfer are separate polling phases to bound stack use.
use crate::{sd, streams, Phase, TIME};
use core::fmt::Write;
use core::sync::atomic::Ordering::Relaxed;
use defmt::{assert, info};
use embassy_stm32::{
    sdmmc::{
        sd::{CmdBlock, StorageDevice},
        Sdmmc,
    },
    time::Hertz,
};
use embassy_time::{Instant, Timer};
use exfat_slim::asynchronous::{file::OpenOptions, file_system::FileSystem};
use integration_test_l4_001_monoaudiolog::*;
use raylar_drivers::storage::{detect_exfat_volume, PartitionedBlockDevice, StorageDriver};
use raylar_storage_service::{StorageLayout, StreamHandle, StreamKind};
use raylar_time_service::{Anchor, AnchorQuality, TimeSource, UtcTimestamp};

pub struct Setup {
    pub epoch: i64,
    pub old_log_length: u64,
    pub serial: u32,
    pub log_stream: StreamHandle,
}
pub async fn storage(sdmmc: &'static mut Sdmmc<'static>) -> Setup {
    let mut command = CmdBlock::new();
    let card = StorageDevice::new_sd_card(sdmmc, &mut command, Hertz(24_000_000))
        .await
        .unwrap();
    let serial = card.card().cid.serial();
    let mut raw = sd::L4Sd::new(card);
    let volume = detect_exfat_volume(&mut raw).await.unwrap();
    let mut fs: FileSystem<_, 512, 1> = FileSystem::new(PartitionedBlockDevice::new(raw, volume));
    Phase(fs.mount()).await.unwrap();
    let (_, free, cluster) = Phase(fs.space_info()).await.unwrap();
    assert!(
        free >= FILE_BYTES * BENCH_SEGMENTS as u64 + u64::from(cluster) * 8 + 1024 * 1024,
        "L4 INTEGRATION FAIL insufficient free space"
    );
    let mut epoch = None;
    for sequence in 0..4096i64 {
        let candidate = FAKE_BASE + sequence * 3600;
        let mut path = heapless::String::<32>::new();
        write!(path, "/{candidate}").unwrap();
        if !Phase(fs.exists(&path)).await.unwrap() {
            epoch = Some(candidate);
            break;
        }
    }
    let epoch = epoch.expect("L4 INTEGRATION FAIL no unused fake hour");
    let old_log_length = if Phase(fs.exists("/syslog.txt")).await.unwrap() {
        let file = fs
            .open("/syslog.txt", OpenOptions::new().read(true))
            .await
            .unwrap();
        let length = file.metadata().len();
        file.close(&mut fs).await.unwrap();
        length
    } else {
        0
    };
    info!("L4 RECORD SD ready serial={:08x} free_bytes={} cluster_bytes={} fake_epoch={} old_syslog_bytes={}", serial, free, cluster, epoch, old_log_length);
    install_service(fs.unmount());
    Phase(mount_global()).await;
    crate::stack_probe::check("storage_installed");
    sd::WRITE_ALLOWED.store(true, Relaxed);
    TIME.anchor_sender()
        .send(Anchor {
            system_time: Instant::now(),
            utc: UtcTimestamp::new(epoch, 0).unwrap(),
            quality: AnchorQuality::new(1_000_000),
            source: TimeSource::Laboratory,
            capture_ticks: None,
            pps_sequence: None,
            pps_interval: None,
        })
        .await;
    while TIME.current_utc().is_err() {
        Timer::after_millis(1).await;
    }
    let log_stream = Phase(open_log()).await;
    crate::stack_probe::check("syslog_open");
    Setup {
        epoch,
        old_log_length,
        serial,
        log_stream,
    }
}

#[inline(always)]
fn make_service(device: PartitionedBlockDevice<sd::L4Sd<'static, 'static>>) -> streams::Service {
    streams::Service::new(StorageDriver::new(device), &TIME).unwrap()
}

#[inline(never)]
fn install_service(device: PartitionedBlockDevice<sd::L4Sd<'static, 'static>>) {
    *streams::STORAGE.try_lock().unwrap() = Some(make_service(device));
}

async fn mount_global() {
    streams::STORAGE
        .lock()
        .await
        .as_mut()
        .unwrap()
        .mount()
        .await
        .unwrap();
}

async fn open_log() -> StreamHandle {
    streams::STORAGE
        .lock()
        .await
        .as_mut()
        .unwrap()
        .begin_stream(StreamKind::Log, StorageLayout::Flat)
        .await
        .unwrap()
}

pub async fn unmount() -> PartitionedBlockDevice<sd::L4Sd<'static, 'static>> {
    let service = streams::STORAGE.lock().await.take().unwrap();
    service.into_inner().into_inner()
}
