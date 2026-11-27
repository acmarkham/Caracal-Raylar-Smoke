//! Fresh-mount, whole-file readback after all three writers and DMA stop.
use crate::{sd, streams};
use aardwolf_l4::*;
use aligned::{Aligned, A4};
use core::{fmt::Write, sync::atomic::Ordering::Relaxed};
use exfat_slim::asynchronous::{file::OpenOptions, file_system::FileSystem};
use heapless::String;
use raylar_drivers::storage::PartitionedBlockDevice;
pub async fn files(
    device: PartitionedBlockDevice<sd::L4Sd<'static, 'static>>,
    recording: &mut streams::Recording,
    log: &streams::Syslog,
) {
    sd::WRITE_ALLOWED.store(false, Relaxed);
    let mut fs: FileSystem<_, 512, 1> = FileSystem::new(device);
    fs.mount().await.unwrap();
    let mut buffer = Aligned::<A4, _>([0u8; 1024]);
    for n in 0..recording.opened as usize {
        for lane in 0..2 {
            let start = recording.starts[n];
            let suffix = if lane == 0 { "flac" } else { "CRCL" };
            let mut path = String::<96>::new();
            write!(
                path,
                "/{}/aud_{}.{}",
                start.div_euclid(3600) * 3600,
                start,
                suffix
            )
            .unwrap();
            let mut file = fs.open(&path, OpenOptions::new().read(true)).await.unwrap();
            defmt::assert!(file.metadata().len() == recording.lengths[n][lane] as u64);
            let mut total = 0;
            let mut hash = FNV_INITIAL;
            let mut tail = [0u8; 52];
            let mut tail_len = 0;
            loop {
                let count = file
                    .read(&mut fs, &mut buffer[..])
                    .await
                    .unwrap()
                    .unwrap_or(0);
                if count == 0 {
                    break;
                }
                if total == 0 {
                    if lane == 0 {
                        defmt::assert!(
                            &buffer[..8] == b"fLaC\0\0\0\x22"
                                && valid_streaminfo(&buffer[8..42], recording.frames[n])
                        );
                        defmt::assert!(
                            hash_bytes(FNV_INITIAL, &buffer[8..42])
                                == recording.streaminfo_hashes[n]
                        );
                    } else {
                        defmt::assert!(&buffer[..4] == b"CRCL" && buffer[36..40] == [32, 1, 4, 3]);
                    }
                }
                hash = if lane == 0 {
                    recording_hash(hash, total, &buffer[..count])
                } else {
                    hash_bytes(hash, &buffer[..count])
                };
                total += count as u64;
                if count >= 52 {
                    tail.copy_from_slice(&buffer[count - 52..count]);
                } else {
                    tail.rotate_left(count);
                    tail[52 - count..].copy_from_slice(&buffer[..count]);
                }
                tail_len = (tail_len + count).min(52);
            }
            file.close(&mut fs).await.unwrap();
            defmt::assert!(
                total == recording.lengths[n][lane] as u64 && hash == recording.hashes[n][lane]
            );
            if lane == 1 {
                defmt::assert!(
                    tail_len == 52
                        && &tail[..4] == b"CRBK"
                        && tail[4] == 3
                        && u64::from_le_bytes(tail[28..36].try_into().unwrap())
                            == recording.frames[n]
                );
            }
            defmt::info!(
                "L4 QUAD VERIFY PASS index={} stream={} bytes={} frames={} hash={:08x}",
                n + 1,
                suffix,
                total,
                recording.frames[n],
                hash
            );
        }
    }
    let mut file = fs
        .open("/syslog.txt", OpenOptions::new().read(true))
        .await
        .unwrap();
    defmt::assert!(file.metadata().len() == log.old_length + log.bytes);
    file.seek(&mut fs, log.old_length).await.unwrap();
    let mut hash = FNV_INITIAL;
    let mut total = 0u64;
    let mut cpu = 0;
    let mut closed = 0;
    let mut complete = false;
    let mut lines = 0;
    #[cfg(feature = "bench-glitches")]
    let mut gap_lines = 0;
    let mut line = String::<320>::new();
    loop {
        let count = file
            .read(&mut fs, &mut buffer[..])
            .await
            .unwrap()
            .unwrap_or(0);
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
                cpu += u32::from(line.contains("cpu usage="));
                closed += u32::from(line.contains("segment_closed"));
                complete |= line.contains("bench_complete");
                #[cfg(feature = "bench-glitches")]
                if line.contains("audio_gap ") {
                    gap_lines += 1;
                    defmt::info!("L4 GLITCH SYSLOG {}", line.as_str());
                }
                line.clear();
            } else {
                line.push(*byte as char).unwrap();
            }
        }
    }
    file.close(&mut fs).await.unwrap();
    #[cfg(feature = "bench-glitches")]
    defmt::assert!(
        gap_lines >= 2,
        "DMA and retention gaps must reach SD syslog"
    );
    defmt::assert!(
        line.is_empty()
            && total == log.bytes
            && hash == log.hash
            && closed == recording.opened
            && cpu >= 2
            && complete
    );
    defmt::info!(
        "L4 QUAD VERIFY SYSLOG PASS bytes={} lines={} cpu_reports={} checkpoints={}",
        total,
        lines,
        cpu,
        streams::LOG_CHECKPOINTS_DURING_AUDIO.load(Relaxed)
    );
}
