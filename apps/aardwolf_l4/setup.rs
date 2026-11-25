//! Bound setup polling frames independently from concurrent stream operations.
use crate::{sd, streams, Phase, TIME};
use core::sync::atomic::Ordering::Relaxed;
use embassy_stm32::{
    sdmmc::{
        sd::{CmdBlock, StorageDevice},
        Sdmmc,
    },
    time::Hertz,
};
use exfat_slim::asynchronous::{file::OpenOptions, file_system::FileSystem};
use raylar_drivers::storage::{detect_exfat_volume, PartitionedBlockDevice, StorageDriver};
use raylar_storage_service::{StorageLayout, StreamHandle, StreamKind};
pub struct Setup {
    pub old_log_length: u64,
    pub serial: u32,
    pub log_stream: StreamHandle,
}
type Fs = FileSystem<PartitionedBlockDevice<sd::L4Sd<'static, 'static>>, 512, 1>;
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
    defmt::assert!(
        aardwolf_l4::policy::can_start_audio(free, cluster, 0),
        "AARDWOLF L4 insufficient free space"
    );
    let (mut old_log_length, needs_separator) = Phase(inspect_syslog(&mut fs)).await;
    defmt::assert!(old_log_length < aardwolf_l4::config::LOG_BUDGET_BYTES);
    defmt::info!(
        "AARDWOLF L4 SD ready serial={:08x} free_bytes={} cluster_bytes={} old_syslog_bytes={}",
        serial,
        free,
        cluster,
        old_log_length
    );
    install_service(fs.unmount());
    Phase(mount_global()).await;
    crate::stack_probe::check("storage_installed");
    sd::WRITE_ALLOWED.store(true, Relaxed);
    let log_stream = Phase(open_log()).await;
    // A reset can leave a checkpoint at a sector boundary within a log line.
    // Append a separator; never rewrite or remove the existing suffix.
    if needs_separator {
        let mut guard = streams::STORAGE.lock().await;
        let service = guard.as_mut().unwrap();
        Phase(service.write(log_stream, b"\n")).await.unwrap();
        Phase(service.flush(log_stream)).await.unwrap();
        old_log_length += 1;
        defmt::info!("AARDWOLF L4 syslog appended separator after interrupted line");
    }
    Setup {
        old_log_length,
        serial,
        log_stream,
    }
}

// File seek/read futures must not enlarge every setup poll, including the
// unrelated later StorageService::begin_stream call. Keep this frame separate.
async fn inspect_syslog(fs: &mut Fs) -> (u64, bool) {
    if Phase(fs.exists("/syslog.txt")).await.unwrap() {
        let mut file = fs
            .open("/syslog.txt", OpenOptions::new().read(true))
            .await
            .unwrap();
        let length = file.metadata().len();
        let mut tail = [b'\n'];
        if length > 0 {
            file.seek(fs, length - 1).await.unwrap();
            defmt::assert!(file.read(fs, &mut tail).await.unwrap() == Some(1));
        }
        file.close(fs).await.unwrap();
        (length, tail[0] != b'\n')
    } else {
        (0, false)
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
    streams::STORAGE
        .lock()
        .await
        .take()
        .unwrap()
        .into_inner()
        .into_inner()
}
