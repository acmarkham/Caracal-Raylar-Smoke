use super::*;
use crate::crcl::crc32c;
use crate::RecordingMetadata;
use raylar_audiosource::AudioFormat;
use raylar_storage_service::StorageLayout;
use std::{
    sync::Arc,
    task::{Context, Poll, Wake, Waker},
    vec,
    vec::Vec,
};

struct Metadata {
    mapped: bool,
}
impl MetadataSource for Metadata {
    fn snapshot(&self) -> Option<RecordingMetadata> {
        let mut m = RecordingMetadata::new(UtcTimestamp::new(1700000040, 0).unwrap());
        m.device_id.push_str("test-board").unwrap();
        m.boot_id = Some(42);
        m.utc_status = 1;
        m.uncertainty_us = 100;
        m.firmware_version.push_str("test").unwrap();
        Some(m)
    }
    fn system_ticks_at(&self, utc: UtcTimestamp) -> Option<u64> {
        self.mapped
            .then(|| (utc.as_micros() - 1700000040000000) as u64)
    }
    fn utc_at_ticks(&self, t: u64) -> Option<UtcTimestamp> {
        Some(UtcTimestamp::from_micros(1700000040000000 + t as i64))
    }
}
#[derive(Default)]
struct Storage {
    files: Vec<Vec<u8>>,
    suffixes: Vec<std::string::String>,
    starts: Vec<UtcTimestamp>,
    closed: Vec<usize>,
    operations: usize,
    fail_at: Option<usize>,
    admitted: Vec<u64>,
}
impl Storage {
    fn operation(&mut self) -> Result<(), ()> {
        self.operations += 1;
        if self.fail_at == Some(self.operations) {
            Err(())
        } else {
            Ok(())
        }
    }
}
impl RecordingStorage for Storage {
    type Error = ();
    type Handle = usize;
    async fn admit_audio_bytes(&mut self, bytes: u64) -> Result<(), ()> {
        self.operation()?;
        self.admitted.push(bytes);
        Ok(())
    }
    async fn begin_audio_stream(
        &mut self,
        _: StorageLayout,
        utc: UtcTimestamp,
        suffix: &str,
    ) -> Result<usize, ()> {
        self.operation()?;
        self.files.push(Vec::new());
        self.starts.push(utc);
        self.suffixes.push(suffix.into());
        Ok(self.files.len() - 1)
    }
    async fn append_audio(&mut self, h: usize, bytes: &[u8]) -> Result<(), ()> {
        self.operation()?;
        self.files[h].extend_from_slice(bytes);
        Ok(())
    }
    async fn rewrite_audio(&mut self, h: usize, offset: u64, bytes: &[u8]) -> Result<(), ()> {
        self.operation()?;
        self.files[h][offset as usize..offset as usize + bytes.len()].copy_from_slice(bytes);
        Ok(())
    }
    async fn finish_audio(&mut self, h: usize) -> Result<(), ()> {
        self.closed.push(h);
        self.operation()
    }
}
struct Wakeup;
impl Wake for Wakeup {
    fn wake(self: Arc<Self>) {}
}
fn block_on<F: core::future::Future>(future: F) -> F::Output {
    let w = Waker::from(Arc::new(Wakeup));
    let mut cx = Context::from_waker(&w);
    let mut f = core::pin::pin!(future);
    loop {
        match f.as_mut().poll(&mut cx) {
            Poll::Ready(v) => return v,
            Poll::Pending => std::thread::yield_now(),
        }
    }
}
fn config(rate: u32, seconds: u32, flac: bool) -> HexaphonicConfig {
    HexaphonicConfig {
        recorder: AudioRecorderConfig {
            recording_seconds: seconds,
            storage_layout: StorageLayout::HourlyFolders,
        },
        spatial: Config {
            block_frames: 256,
            ..Config::default()
        },
        reference: if flac {
            ReferenceFormat::Flac(FlacConfig {
                sample_rate_hz: rate,
                block_samples: 1024,
                ..FlacConfig::default()
            })
        } else {
            ReferenceFormat::Wav
        },
        capture: CaptureParameters {
            pdm_clock_hz: 3200000,
            sinc_order: 5,
            decimation: 100,
            cic_scale: 40,
            hpf_enabled: true,
            pcm_shift: 8,
            gain_tenths_db: -241,
            origin_ticks: None,
        },
        calibration_id: None,
    }
}
fn samples(start: usize, count: usize) -> Vec<i32> {
    (start..start + count)
        .flat_map(|n| (0..6).map(move |c| n as i32 * 13 + c * 157))
        .collect()
}
fn end(file: &[u8]) -> (u64, u32, [u8; 32]) {
    let cheap = u16::from_le_bytes(file[8..10].try_into().unwrap()) == 2;
    let mut p = u16::from_le_bytes(file[10..12].try_into().unwrap()) as usize;
    let mut sequence = 0;
    let mut expected = 0;
    loop {
        assert_eq!(&file[p..p + 4], b"CRBK");
        let length = u32::from_le_bytes(file[p + 8..p + 12].try_into().unwrap()) as usize;
        assert_eq!(
            u32::from_le_bytes(file[p + 12..p + 16].try_into().unwrap()),
            sequence
        );
        sequence += 1;
        assert_eq!(
            u64::from_le_bytes(file[p + 16..p + 24].try_into().unwrap()),
            expected
        );
        if !cheap {
            assert_eq!(
                u32::from_le_bytes(file[p + length - 4..p + length].try_into().unwrap()),
                crc32c(&file[p..p + length - 4])
            );
        }
        match file[p + 4] {
            1 => expected += u16::from_le_bytes(file[p + 24..p + 26].try_into().unwrap()) as u64,
            3 => {
                assert_eq!(p + length, file.len());
                if cheap {
                    assert_eq!(length, 52);
                    assert_eq!(
                        u64::from_le_bytes(file[p + 28..p + 36].try_into().unwrap()),
                        expected
                    );
                    return (
                        expected,
                        u32::from_le_bytes(file[p + 48..p + 52].try_into().unwrap()),
                        [0; 32],
                    );
                }
                return (
                    expected,
                    u32::from_le_bytes(file[p + 52..p + 56].try_into().unwrap()),
                    file[p + 56..p + 88].try_into().unwrap(),
                );
            }
            2 => {}
            _ => panic!(),
        }
        p += length;
    }
}

#[test]
fn paired_rotation_and_tail_preserve_every_reference_sample() {
    for cheap in [false, true] {
        for flac in [false, true] {
            let source = AudioSource::<6144, 1>::new(AudioFormat::new(16000, 6, 1000000));
            let (mut pcm, mut spatial, mut ref_pcm, mut ref_out) =
                ([0; 1536], [0; 4096], [0; 1024], [0; 4128]);
            let mut profile = config(16000, 1, flac);
            if cheap {
                profile.spatial = Config {
                    block_frames: 256,
                    min_shift: 6,
                    ..Config::cheap()
                };
            }
            let mut recorder = HexaphonicRecorder::new(
                &source,
                Storage::default(),
                Metadata { mapped: false },
                profile,
                HexaphonicBuffers {
                    pcm: &mut pcm,
                    spatial_output: &mut spatial,
                    reference_pcm: &mut ref_pcm,
                    reference_output: &mut ref_out,
                },
            )
            .unwrap();
            block_on(recorder.start_at(UtcTimestamp::new(1700000040, 0).unwrap())).unwrap();
            assert!(recorder.storage().files.is_empty());
            let mut original = Vec::new();
            let mut position = 0;
            while position < 16013 {
                let n = (16013 - position).min(317);
                let input = samples(position, n);
                original.extend(input.iter().step_by(6).copied());
                source
                    .write(&input, position as u64 * 1000000 / 16000)
                    .unwrap();
                while recorder.reader.stats().available_samples > 0 {
                    block_on(recorder.record_next()).unwrap();
                }
                position += n;
            }
            block_on(recorder.stop()).unwrap();
            assert_eq!(
                recorder.storage().suffixes,
                if flac {
                    vec!["flac", "CRCL", "flac", "CRCL"]
                } else {
                    vec!["wav", "CRCL", "wav", "CRCL"]
                }
            );
            assert_eq!(recorder.storage().closed, [0, 1, 2, 3]);
            for (segment, count) in [(0, 16000), (1, 13)] {
                let storage = recorder.storage();
                let (total, status, digest) = end(&storage.files[segment * 2 + 1]);
                assert_eq!(total, count as u64);
                assert_eq!(status, if segment == 0 { 1 } else { 2 });
                assert_eq!(storage.starts[segment * 2], storage.starts[segment * 2 + 1]);
                let offset = if segment == 0 { 0 } else { 16000 };
                let raw: Vec<_> = original[offset..offset + count]
                    .iter()
                    .flat_map(|v| v.to_le_bytes())
                    .collect();
                let mut hash = Sha256::new();
                hash.update(&raw);
                if cheap {
                    assert_eq!(digest, [0; 32]);
                } else {
                    assert_eq!(digest, hash.finish());
                }
                if !flac {
                    assert_eq!(&storage.files[segment * 2][1024..], raw);
                }
                if let Some(directory) = std::env::var_os("RAYLAR_CRCL_TEST_OUTPUT") {
                    let dir = std::path::PathBuf::from(directory);
                    std::fs::create_dir_all(&dir).unwrap();
                    let stem = std::format!(
                        "paired-{}{}-{segment}",
                        if cheap { "cheap-" } else { "" },
                        if flac { "flac" } else { "wav" }
                    );
                    std::fs::write(
                        dir.join(std::format!("{stem}.{}", if flac { "flac" } else { "wav" })),
                        &storage.files[segment * 2],
                    )
                    .unwrap();
                    std::fs::write(
                        dir.join(std::format!("{stem}.CRCL")),
                        &storage.files[segment * 2 + 1],
                    )
                    .unwrap();
                    std::fs::write(dir.join(std::format!("{stem}.s32le")), raw).unwrap();
                }
            }
        }
    }
}

#[test]
fn source_loss_finishes_only_the_common_prefix() {
    let source = AudioSource::<6144, 1>::new(AudioFormat::new(16000, 6, 1000000));
    let (mut pcm, mut spatial, mut ref_pcm, mut ref_out) =
        ([0; 1536], [0; 4096], [0; 1024], [0; 4128]);
    let mut r = HexaphonicRecorder::new(
        &source,
        Storage::default(),
        Metadata { mapped: false },
        config(16000, 60, true),
        HexaphonicBuffers {
            pcm: &mut pcm,
            spatial_output: &mut spatial,
            reference_pcm: &mut ref_pcm,
            reference_output: &mut ref_out,
        },
    )
    .unwrap();
    block_on(r.start_at(UtcTimestamp::new(1700000040, 0).unwrap())).unwrap();
    source.write(&samples(0, 7), 0).unwrap();
    block_on(r.record_next()).unwrap();
    source.report_discontinuity(18);
    source.write(&samples(10, 13), 625).unwrap();
    assert!(matches!(
        block_on(r.record_next()),
        Err(AudioRecorderError::Discontinuity {
            dropped_samples: 18
        })
    ));
    assert_eq!(end(&r.storage().files[1]).0, 7);
    assert_eq!(end(&r.storage().files[1]).1, 4);
    block_on(r.stop()).unwrap();
    assert_eq!(r.storage().closed, [0, 1]);
}

#[test]
fn failures_close_both_handles_without_replaying_or_emitting_a_successful_end() {
    for fail in 1..=11 {
        let source = AudioSource::<6144, 1>::new(AudioFormat::new(16000, 6, 1000000));
        let (mut pcm, mut spatial, mut ref_pcm, mut ref_out) =
            ([0; 1536], [0; 4096], [0; 1024], [0; 4128]);
        let storage = Storage {
            fail_at: Some(fail),
            ..Storage::default()
        };
        let mut r = HexaphonicRecorder::new(
            &source,
            storage,
            Metadata { mapped: false },
            config(16000, 60, false),
            HexaphonicBuffers {
                pcm: &mut pcm,
                spatial_output: &mut spatial,
                reference_pcm: &mut ref_pcm,
                reference_output: &mut ref_out,
            },
        )
        .unwrap();
        block_on(r.start_at(UtcTimestamp::new(1700000040, 0).unwrap())).unwrap();
        source.write(&samples(0, 7), 0).unwrap();
        let read = block_on(r.record_next());
        let stop = block_on(r.stop());
        assert!(
            read.is_err() || stop.is_err(),
            "failure {fail} was not exercised"
        );
        let sizes: Vec<_> = r.storage().files.iter().map(Vec::len).collect();
        block_on(r.stop()).unwrap();
        assert_eq!(
            sizes,
            r.storage().files.iter().map(Vec::len).collect::<Vec<_>>()
        );
        for h in 0..r.storage().files.len() {
            assert!(r.storage().closed.contains(&h));
        }
    }
}

#[test]
fn utc_boundaries_skip_pre_start_and_split_at_shared_end() {
    let source = AudioSource::<6144, 1>::new(AudioFormat::new(16000, 6, 1000000));
    let (mut pcm, mut spatial, mut ref_pcm, mut ref_out) =
        ([0; 1536], [0; 4096], [0; 1024], [0; 4128]);
    let mut r = HexaphonicRecorder::new(
        &source,
        Storage::default(),
        Metadata { mapped: true },
        config(16000, 1, false),
        HexaphonicBuffers {
            pcm: &mut pcm,
            spatial_output: &mut spatial,
            reference_pcm: &mut ref_pcm,
            reference_output: &mut ref_out,
        },
    )
    .unwrap();
    block_on(r.start_at(UtcTimestamp::new(1700000041, 0).unwrap())).unwrap();
    source.write(&samples(0, 256), 990000).unwrap();
    block_on(r.record_next()).unwrap();
    assert_eq!(r.pcm_frames, 96);
    assert_eq!(r.first_ticks, 1000000);
    assert_eq!(r.frames_target, 16000);
    block_on(r.stop()).unwrap();
    assert_eq!(end(&r.storage().files[1]).0, 96);
}

#[test]
fn initial_alignment_can_skip_multiple_packets_without_consuming_reference_frames() {
    let source = AudioSource::<6144, 1>::new(AudioFormat::new(16000, 6, 1000000));
    let (mut pcm, mut spatial, mut ref_pcm, mut ref_out) =
        ([0; 1536], [0; 4096], [0; 1024], [0; 4128]);
    let mut recorder = HexaphonicRecorder::new(
        &source,
        Storage::default(),
        Metadata { mapped: true },
        config(16000, 1, false),
        HexaphonicBuffers {
            pcm: &mut pcm,
            spatial_output: &mut spatial,
            reference_pcm: &mut ref_pcm,
            reference_output: &mut ref_out,
        },
    )
    .unwrap();
    block_on(recorder.start_at(UtcTimestamp::new(1700000040, 40000).unwrap())).unwrap();
    for packet in 0..3 {
        source
            .write(&samples(packet * 256, 256), packet as u64 * 16000)
            .unwrap();
        block_on(recorder.record_next()).unwrap();
    }
    assert_eq!(recorder.first_ticks, 40000);
    assert_eq!(recorder.pcm_frames, 128);
    block_on(recorder.stop()).unwrap();
    let expected: Vec<_> = samples(640, 128)
        .iter()
        .step_by(6)
        .flat_map(|v| v.to_le_bytes())
        .collect();
    assert_eq!(&recorder.storage().files[0][1024..], expected);
    assert_eq!(end(&recorder.storage().files[1]).0, 128);
}
