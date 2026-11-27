//! Full readback after both writers close and a fresh filesystem is mounted.
use crate::{sd, streams};
use aardwolf_l4::*;
use aligned::{Aligned, A4};
use core::fmt::Write;
use core::sync::atomic::Ordering::Relaxed;
use exfat_slim::asynchronous::{file::OpenOptions, file_system::FileSystem};
use heapless::String;
use raylar_drivers::storage::PartitionedBlockDevice;
static BUFFER: static_cell::ConstStaticCell<Aligned<A4, [u8; VERIFY_BYTES]>> =
    static_cell::ConstStaticCell::new(Aligned([0; VERIFY_BYTES]));
pub fn workspace() -> &'static mut [u8; VERIFY_BYTES] {
    BUFFER.take()
}

pub async fn files(
    device: PartitionedBlockDevice<sd::L4Sd<'static, 'static>>,
    recording: &mut streams::Recording,
    log: &streams::Syslog,
) {
    sd::WRITE_ALLOWED.store(false, Relaxed);
    let mut fs: FileSystem<PartitionedBlockDevice<_>, 512, 1> = FileSystem::new(device);

    fs.mount().await.unwrap();
    let buffer = recording.workspace.take().unwrap();
    for n in 0..recording.opened as usize {
        let start = recording.starts[n];
        let mut path = String::<96>::new();
        write!(
            path,
            "/{}/aud_{}.{}",
            start.div_euclid(3600) * 3600,
            start,
            CODEC
        )
        .unwrap();
        let mut file = fs.open(&path, OpenOptions::new().read(true)).await.unwrap();
        defmt::assert!(file.metadata().len() == recording.lengths[n] as u64);
        let header_size = if cfg!(feature = "wav") { 512 } else { 1024 };
        let mut first = 0;
        while first < header_size {
            let count = file
                .read(&mut fs, &mut buffer[first..header_size])
                .await
                .unwrap()
                .unwrap_or(0);
            if count == 0 {
                break;
            }
            first += count;
        }
        defmt::assert!(first == header_size);
        if cfg!(feature = "wav") {
            defmt::assert!(valid_minute_header(&buffer[..512]));
        } else {
            defmt::assert!(
                &buffer[..8] == b"fLaC\0\0\0\x22"
                    && valid_streaminfo(&buffer[8..42], SEGMENT_SAMPLES as u64)
            );
            defmt::assert!(
                hash_bytes(FNV_INITIAL, &buffer[8..42]) == recording.streaminfo_hashes[n]
            );
            defmt::assert!(buffer[..first].windows(6).any(|b| b == b"L4Evo:"));
        }
        let mut hash = recording_hash(FNV_INITIAL, 0, &buffer[..first]);
        let mut total = first as u64;
        let mut minimum = i32::MAX;
        let mut maximum = i32::MIN;
        let mut low_or = 0u8;
        let mut low_and = 255u8;
        let mut tones = [
            tone::Tone::new(tone::HZ800),
            tone::Tone::new(tone::HZ1000),
            tone::Tone::new(tone::HZ1200),
            tone::Tone::new(tone::HZ2000),
            tone::Tone::new(tone::HZ2200),
            tone::Tone::new(tone::HZ3000),
            tone::Tone::new(tone::HZ3200),
        ];
        let mut dc = 0;
        let mut sum = 0i64;
        let mut squares = 0u128;
        let mut sample_index = 0u32;
        loop {
            let count = file
                .read(&mut fs, &mut buffer[..4096])
                .await
                .unwrap()
                .unwrap_or(0);
            if count == 0 {
                break;
            }
            if cfg!(feature = "wav") {
                defmt::assert!(count % 4 == 0);
            }
            hash = recording_hash(hash, total, &buffer[..count]);
            total += count as u64;
            if !cfg!(feature = "wav") {
                continue;
            }
            for bytes in buffer[..count].chunks_exact(4) {
                let packed = i32::from_le_bytes(bytes.try_into().unwrap());
                defmt::assert!(packed & 255 == 0);
                let sample = packed >> 8;
                minimum = minimum.min(sample);
                maximum = maximum.max(sample);
                sum += i64::from(sample);
                squares += (i64::from(sample) * i64::from(sample)) as u128;
                low_or |= sample as u8;
                low_and &= sample as u8;
                if cfg!(feature = "bench-audio-ui")
                    && n == 0
                    && (120_000..123_200).contains(&sample_index)
                {
                    if sample_index == 120_000 {
                        dc = sample;
                    }
                    for tone in &mut tones {
                        tone.add(sample - dc);
                    }
                }
                sample_index += 1;
            }
        }
        file.close(&mut fs).await.unwrap();
        defmt::assert!(total == recording.lengths[n] as u64 && hash == recording.hashes[n]);
        if !cfg!(feature = "wav") {
            defmt::info!("AARDWOLF L4 VERIFY FLAC PASS index={} path={} bytes={} samples=960000 normalized_hash={:08x} streaminfo_hash={:08x}",n+1,path.as_str(),total,hash,recording.streaminfo_hashes[n]);
            continue;
        }
        defmt::assert!(maximum as i64 - minimum as i64 > 32 && low_or ^ low_and != 0);
        let count = u128::from(sample_index);
        let ac_power = (squares * count - sum.unsigned_abs() as u128 * sum.unsigned_abs() as u128)
            / (count * count);
        defmt::info!("AARDWOLF L4 VERIFY WAV PASS index={} path={} bytes={} samples=960000 hash={:08x} min={} max={} low_byte_var={:02x}", n + 1, path.as_str(), total, hash, minimum, maximum, low_or ^ low_and);
        let clipped = minimum == -8_388_608 || maximum == 8_388_607;
        defmt::assert!(!clipped);
        defmt::info!(
            "AARDWOLF L4 PCM24 mean={} ac_mean_power={} clipped={}",
            sum / i64::from(sample_index),
            ac_power as u64,
            clipped
        );
        if cfg!(feature = "bench-audio-ui") && n == 0 {
            let powers = tones.map(|tone| tone.power());
            defmt::info!(
                "AARDWOLF L4 ACOUSTIC 800Hz={} 1000Hz={} 1200Hz={} 2000Hz={} 2200Hz={} 3000Hz={} 3200Hz={} window_samples=3200",
                powers[0],
                powers[1],
                powers[2], powers[3], powers[4], powers[5], powers[6]
            );
            let peak = powers[1].max(powers[3]).max(powers[5]);
            let noise = powers[0]
                .max(powers[2])
                .max(powers[4])
                .max(powers[6])
                .max(1);
            defmt::info!(
                "AARDWOLF L4 ACOUSTIC strong={} peak_to_neighbor={} listening_required=true",
                peak > 10 * noise,
                peak / noise
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
    let mut lines = 0u32;
    let mut heartbeats = 0u32;
    let mut closed = 0u32;
    let mut real_marker = false;
    let mut complete_marker = false;
    let mut cpu = 0;
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
                real_marker |= line.contains(if cfg!(feature = "local-time") {
                    "LOCAL-TIME-BENCH"
                } else {
                    "REAL-GPS"
                });
                complete_marker |= line.contains("bench_complete");
                cpu += u32::from(line.contains("cpu usage="));
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
        heartbeats
            >= if cfg!(any(feature = "bench-audio-ui", feature = "bench-codec")) {
                120
            } else {
                175
            }
            && closed == recording.opened
            && real_marker
            && complete_marker
            && cpu >= 2
    );
    defmt::info!("AARDWOLF L4 VERIFY SYSLOG PASS path=/syslog.txt old_bytes={} appended_bytes={} hash={:08x} lines={} heartbeats={} closed_markers={} real_gps=true", log.old_length, total, hash, lines, heartbeats, closed);
}
