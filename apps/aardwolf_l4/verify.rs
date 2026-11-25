//! Full readback after both writers close and a fresh filesystem is mounted.
use crate::{sd, streams};
use aardwolf_l4::*;
use aligned::{Aligned, A4};
use core::fmt::Write;
use core::sync::atomic::Ordering::Relaxed;
use exfat_slim::asynchronous::{file::OpenOptions, file_system::FileSystem};
use heapless::String;
use raylar_drivers::storage::PartitionedBlockDevice;
static BUFFER: static_cell::ConstStaticCell<Aligned<A4, [u8; 4096]>> =
    static_cell::ConstStaticCell::new(Aligned([0; 4096]));

pub async fn files(
    device: PartitionedBlockDevice<sd::L4Sd<'static, 'static>>,
    recording: &streams::Recording,
    log: &streams::Syslog,
) {
    sd::WRITE_ALLOWED.store(false, Relaxed);
    let mut fs: FileSystem<PartitionedBlockDevice<_>, 512, 1> = FileSystem::new(device);

    fs.mount().await.unwrap();
    let buffer: &mut [u8; 4096] = BUFFER.take();
    for n in 0..recording.opened as usize {
        let start = recording.starts[n];
        let mut path = String::<96>::new();
        write!(path, "/{}/aud_{}.wav", start.div_euclid(3600) * 3600, start).unwrap();
        let mut file = fs.open(&path, OpenOptions::new().read(true)).await.unwrap();
        defmt::assert!(file.metadata().len() == FILE_BYTES);
        let first = file
            .read(&mut fs, &mut buffer[..512])
            .await
            .unwrap()
            .unwrap_or(0);
        defmt::assert!(first == 512 && valid_minute_header(&buffer[..512]));
        let mut hash = hash_bytes(FNV_INITIAL, &buffer[..512]);
        let mut total = 512u64;
        let mut minimum = i32::MAX;
        let mut maximum = i32::MIN;
        let mut low_or = 0u8;
        let mut low_and = 255u8;
        loop {
            let count = file.read(&mut fs, buffer).await.unwrap().unwrap_or(0);
            if count == 0 {
                break;
            }
            defmt::assert!(count % 4 == 0);
            total += count as u64;
            hash = hash_bytes(hash, &buffer[..count]);
            for bytes in buffer[..count].chunks_exact(4) {
                let sample = i32::from_le_bytes(bytes.try_into().unwrap());
                defmt::assert!((-8_388_608..=8_388_607).contains(&sample));
                minimum = minimum.min(sample);
                maximum = maximum.max(sample);
                low_or |= sample as u8;
                low_and &= sample as u8;
            }
        }
        file.close(&mut fs).await.unwrap();
        defmt::assert!(total == FILE_BYTES && hash == recording.hashes[n]);
        defmt::assert!(maximum as i64 - minimum as i64 > 32 && low_or ^ low_and != 0);
        defmt::info!("AARDWOLF L4 VERIFY WAV PASS index={} path={} bytes={} samples=960000 hash={:08x} min={} max={} low_byte_var={:02x}", n + 1, path.as_str(), total, hash, minimum, maximum, low_or ^ low_and);
    }
    let mut file = fs
        .open("/syslog.txt", OpenOptions::new().read(true))
        .await
        .unwrap();
    defmt::assert!(file.metadata().len() == log.old_length + log.bytes);
    file.seek(&mut fs, log.old_length).await.unwrap();
    let mut hash = FNV_INITIAL;
    let mut total = 0u64;
    let mut lines = 0u32;
    let mut heartbeats = 0u32;
    let mut closed = 0u32;
    let mut real_marker = false;
    let mut complete_marker = false;
    let mut line = String::<320>::new();
    loop {
        let count = file.read(&mut fs, buffer).await.unwrap().unwrap_or(0);
        if count == 0 {
            break;
        }
        hash = hash_bytes(hash, &buffer[..count]);
        total += count as u64;
        for byte in &buffer[..count] {
            if *byte == b'\n' {
                let sequence: u32 = line
                    .split_ascii_whitespace()
                    .next()
                    .unwrap()
                    .parse()
                    .unwrap();
                defmt::assert!(sequence == lines);
                lines += 1;
                heartbeats += u32::from(line.contains("heartbeat="));
                closed += u32::from(line.contains("segment_closed"));
                real_marker |= line.contains("REAL-GPS");
                complete_marker |= line.contains("bench_complete");
                line.clear();
            } else {
                defmt::assert!(byte.is_ascii());
                line.push(*byte as char).unwrap();
            }
        }
    }
    file.close(&mut fs).await.unwrap();
    defmt::assert!(line.is_empty() && total == log.bytes && hash == log.hash);
    defmt::assert!(
        heartbeats >= 175 && closed == recording.opened && real_marker && complete_marker
    );
    defmt::info!("AARDWOLF L4 VERIFY SYSLOG PASS path=/syslog.txt old_bytes={} appended_bytes={} hash={:08x} lines={} heartbeats={} closed_markers={} real_gps=true", log.old_length, total, hash, lines, heartbeats, closed);
}
