use std::sync::Arc;
use std::task::{Context, Poll, Wake, Waker};
use std::vec::Vec;

use raylar_audiosource::{AudioFormat, AudioSource};
use raylar_time_service::UtcTimestamp;

use super::*;
use crate::RecordingMetadata;

struct FixedMetadata;

impl MetadataSource for FixedMetadata {
    fn snapshot(&self) -> Option<RecordingMetadata> {
        Some(RecordingMetadata::new(
            UtcTimestamp::new(1_700_000_000, 0).unwrap(),
        ))
    }
}

#[derive(Default)]
struct MockStorage {
    writes: Vec<Vec<u8>>,
    files: Vec<Vec<u8>>,
    suffixes: Vec<std::string::String>,
    rewrites: Vec<(u64, Vec<u8>)>,
    fail_append: bool,
    fail_rewrite: bool,
    fail_finish: bool,
    layouts: Vec<StorageLayout>,
    starts: Vec<UtcTimestamp>,
    finishes: usize,
    next_handle: u8,
}

impl RecordingStorage for MockStorage {
    type Error = ();
    type Handle = u8;

    async fn begin_audio_stream(
        &mut self,
        layout: StorageLayout,
        started_utc: UtcTimestamp,
        suffix: &str,
    ) -> Result<Self::Handle, Self::Error> {
        self.layouts.push(layout);
        self.starts.push(started_utc);
        self.files.push(Vec::new());
        self.suffixes.push(suffix.into());
        self.next_handle += 1;
        Ok(self.next_handle)
    }

    async fn append_audio(
        &mut self,
        stream: Self::Handle,
        bytes: &[u8],
    ) -> Result<(), Self::Error> {
        if self.fail_append {
            return Err(());
        }
        self.writes.push(bytes.to_vec());
        self.files[stream as usize - 1].extend_from_slice(bytes);
        Ok(())
    }

    async fn finish_audio(&mut self, _stream: Self::Handle) -> Result<(), Self::Error> {
        self.finishes += 1;
        if self.fail_finish {
            Err(())
        } else {
            Ok(())
        }
    }

    async fn rewrite_audio(
        &mut self,
        stream: Self::Handle,
        offset: u64,
        bytes: &[u8],
    ) -> Result<(), Self::Error> {
        if self.fail_rewrite {
            return Err(());
        }
        self.rewrites.push((offset, bytes.to_vec()));
        self.files[stream as usize - 1][offset as usize..offset as usize + bytes.len()]
            .copy_from_slice(bytes);
        Ok(())
    }
}

struct NoopWake;

impl Wake for NoopWake {
    fn wake(self: Arc<Self>) {}
}

fn block_on<F: core::future::Future>(future: F) -> F::Output {
    let waker = Waker::from(Arc::new(NoopWake));
    let mut context = Context::from_waker(&waker);
    let mut future = core::pin::pin!(future);
    loop {
        match future.as_mut().poll(&mut context) {
            Poll::Ready(output) => return output,
            Poll::Pending => std::thread::yield_now(),
        }
    }
}

#[test]
fn recorder_owns_rotation_at_an_exact_sample_boundary() {
    let source = AudioSource::<16, 1>::new(AudioFormat::new(4, 1, 1_000_000));
    let config = AudioRecorderConfig {
        recording_seconds: 1,
        storage_layout: StorageLayout::DailyFolders,
    };
    let mut recorder = AudioRecorder::<_, _, 16, 1, 16>::new(
        &source,
        MockStorage::default(),
        FixedMetadata,
        config,
    )
    .unwrap();

    source.write(&[99, 99], 0).unwrap();
    block_on(recorder.start()).unwrap();
    source.write(&[1, -2, 3, -4], 250).unwrap();
    let progress = block_on(recorder.record_next()).unwrap();

    assert_eq!(progress.pcm_samples, 4);
    assert!(progress.rotated);
    assert_eq!(recorder.storage().finishes, 1);
    assert_eq!(recorder.storage().layouts, [StorageLayout::DailyFolders; 2]);
    assert_eq!(&recorder.storage().writes[0][..4], b"RIFF");
    assert_eq!(
        recorder.storage().writes[1],
        [1i32, -2, 3, -4]
            .into_iter()
            .flat_map(i32::to_le_bytes)
            .collect::<Vec<_>>()
    );
    assert_eq!(&recorder.storage().writes[2][..4], b"RIFF");

    block_on(recorder.stop()).unwrap();
    assert_eq!(recorder.storage().finishes, 2);
}

#[test]
fn read_is_split_instead_of_crossing_the_recording_boundary() {
    let source = AudioSource::<16, 1>::new(AudioFormat::new(2, 1, 1_000_000));
    let config = AudioRecorderConfig {
        recording_seconds: 1,
        storage_layout: StorageLayout::Flat,
    };
    let mut recorder = AudioRecorder::<_, _, 16, 1, 16>::new(
        &source,
        MockStorage::default(),
        FixedMetadata,
        config,
    )
    .unwrap();

    block_on(recorder.start()).unwrap();
    source.write(&[1, 2, 3, 4], 0).unwrap();
    let first = block_on(recorder.record_next()).unwrap();
    let second = block_on(recorder.record_next()).unwrap();

    assert_eq!(first.pcm_samples, 2);
    assert_eq!(second.pcm_samples, 2);
    assert!(first.rotated);
    assert!(second.rotated);
}

fn flac_count(bytes: &[u8]) -> u64 {
    assert_eq!(&bytes[..4], b"fLaC");
    u64::from_be_bytes(bytes[18..26].try_into().unwrap()) & ((1u64 << 36) - 1)
}

fn desktop_fixture(name: &str, file: &[u8], samples: &[i32]) {
    if let Some(folder) = std::env::var_os("RAYLAR_FLAC_TEST_OUTPUT") {
        let folder = std::path::PathBuf::from(folder);
        std::fs::create_dir_all(&folder).unwrap();
        std::fs::write(folder.join(std::format!("{name}_32_2048.flac")), file).unwrap();
        let raw: Vec<_> = samples
            .iter()
            .flat_map(|sample| sample.to_le_bytes())
            .collect();
        std::fs::write(folder.join(std::format!("{name}_32_2048.s32le")), raw).unwrap();
    }
}

#[test]
fn flac_buffers_packets_and_flushes_the_tail_on_stop() {
    let source = AudioSource::<4096, 1>::new(AudioFormat::new(16000, 1, 1000000));
    let mut output = [0u8; 4128];
    let mut pcm = [0i32; 1024];
    let mut recorder = AudioRecorder::<_, _, 4096, 1, 4128, 1024>::new_flac_with_buffers(
        &source,
        MockStorage::default(),
        FixedMetadata,
        AudioRecorderConfig {
            recording_seconds: 60,
            storage_layout: StorageLayout::HourlyFolders,
        },
        FlacConfig {
            block_samples: 1024,
            ..FlacConfig::default()
        },
        &mut output,
        &mut pcm,
    )
    .unwrap();
    block_on(recorder.start()).unwrap();
    source.write(&[7; 11], 0).unwrap();
    assert_eq!(block_on(recorder.record_next()).unwrap().pcm_samples, 11);
    assert_eq!(recorder.storage().writes.len(), 1);
    source.write(&[-9; 13], 687).unwrap();
    block_on(recorder.record_next()).unwrap();
    block_on(recorder.stop()).unwrap();
    assert_eq!(recorder.storage().suffixes, ["flac"]);
    assert_eq!(flac_count(&recorder.storage().files[0]), 24);
    assert_eq!(recorder.storage().writes.len(), 2);
    assert_eq!(recorder.storage().finishes, 1);
    assert_eq!(recorder.storage().rewrites[0].0, 8);
    desktop_fixture(
        "recorder-tail",
        &recorder.storage().files[0],
        &[&[7; 11][..], &[-9; 13][..]].concat(),
    );
}

#[test]
fn flac_minute_rotation_is_exact_and_preserves_the_next_packet() {
    let source = AudioSource::<4096, 1>::new(AudioFormat::new(16000, 1, 1000000));
    let mut recorder = AudioRecorder::<_, _, 4096, 1, 8224, 2048>::new_flac(
        &source,
        MockStorage::default(),
        FixedMetadata,
        AudioRecorderConfig {
            recording_seconds: 60,
            storage_layout: StorageLayout::HourlyFolders,
        },
        FlacConfig::default(),
    )
    .unwrap();
    block_on(recorder.start_at(UtcTimestamp::new(1700000040, 0).unwrap())).unwrap();
    let mut position = 0;
    while position < 960000 {
        let n = (960000 - position).min(1600);
        source
            .write(&[123; 1600][..n], position as u64 * 1000000 / 16000)
            .unwrap();
        let mut consumed = 0;
        while consumed < n {
            consumed += block_on(recorder.record_next()).unwrap().pcm_samples;
        }
        position += n;
    }
    assert_eq!(recorder.storage().finishes, 1);
    assert_eq!(flac_count(&recorder.storage().files[0]), 960000);
    assert_eq!(recorder.storage().starts.len(), 1);
    assert_eq!(recorder.storage().writes.len(), 470); // 1 header + 469 frames
    source.write(&[456; 3], 60000000).unwrap();
    block_on(recorder.record_next()).unwrap();
    assert_eq!(recorder.storage().starts[1].seconds, 1700000100);
    block_on(recorder.stop()).unwrap();
    assert_eq!(flac_count(&recorder.storage().files[1]), 3);
    desktop_fixture(
        "recorder-minute",
        &recorder.storage().files[0],
        &std::vec![123;960000],
    );
    desktop_fixture("recorder-next", &recorder.storage().files[1], &[456; 3]);
}

#[test]
fn flac_loss_closes_only_the_continuous_prefix() {
    let source = AudioSource::<4096, 1>::new(AudioFormat::new(16000, 1, 1000000));
    let mut recorder = AudioRecorder::<_, _, 4096, 1, 8224, 2048>::new_flac(
        &source,
        MockStorage::default(),
        FixedMetadata,
        AudioRecorderConfig::default(),
        FlacConfig::default(),
    )
    .unwrap();
    block_on(recorder.start()).unwrap();
    source.write(&[1; 7], 0).unwrap();
    block_on(recorder.record_next()).unwrap();
    source.report_discontinuity(9);
    source.write(&[2; 11], 5000).unwrap();
    assert_eq!(
        block_on(recorder.record_next()),
        Err(AudioRecorderError::Discontinuity { dropped_samples: 9 })
    );
    assert_eq!(flac_count(&recorder.storage().files[0]), 7);
    assert_eq!(recorder.storage().finishes, 1);
    desktop_fixture("recorder-gap-prefix", &recorder.storage().files[0], &[1; 7]);
    block_on(recorder.stop()).unwrap();
    assert_eq!(recorder.storage().finishes, 1);
}

#[test]
fn flac_append_and_finalization_errors_close_without_replaying_samples() {
    for failure in 0..3 {
        let source = AudioSource::<4096, 1>::new(AudioFormat::new(16000, 1, 1000000));
        let mut recorder = AudioRecorder::<_, _, 4096, 1, 8224, 2048>::new_flac(
            &source,
            MockStorage::default(),
            FixedMetadata,
            AudioRecorderConfig::default(),
            FlacConfig::default(),
        )
        .unwrap();
        block_on(recorder.start()).unwrap();
        source.write(&[1; 7], 0).unwrap();
        block_on(recorder.record_next()).unwrap();
        recorder.storage_mut().fail_append = failure == 0;
        recorder.storage_mut().fail_rewrite = failure == 1;
        recorder.storage_mut().fail_finish = failure == 2;
        assert_eq!(
            block_on(recorder.stop()),
            Err(AudioRecorderError::Storage(()))
        );
        let writes = recorder.storage().writes.len();
        block_on(recorder.stop()).unwrap();
        assert_eq!(recorder.storage().writes.len(), writes);
        assert_eq!(recorder.storage().finishes, 1);
    }
}

#[test]
fn wav_is_default_and_early_stop_updates_sizes_in_the_recorder() {
    let source = AudioSource::<16, 1>::new(AudioFormat::new(4, 1, 1000000));
    let mut recorder = AudioRecorder::<_, _, 16, 1, 16>::new(
        &source,
        MockStorage::default(),
        FixedMetadata,
        AudioRecorderConfig::default(),
    )
    .unwrap();
    block_on(recorder.start()).unwrap();
    source.write(&[1, 2, 3, 4], 0).unwrap();
    block_on(recorder.record_next()).unwrap();
    block_on(recorder.stop()).unwrap();
    let file = &recorder.storage().files[0];
    assert_eq!(recorder.storage().suffixes, ["wav"]);
    assert_eq!(u32::from_le_bytes(file[4..8].try_into().unwrap()), 520);
    assert_eq!(u32::from_le_bytes(file[508..512].try_into().unwrap()), 16);
}

#[test]
fn flac_does_not_materialize_an_empty_file_and_rejects_invalid_pcm() {
    let source = AudioSource::<4096, 1>::new(AudioFormat::new(16000, 1, 1000000));
    let mut recorder = AudioRecorder::<_, _, 4096, 1, 8224, 2048>::new_flac(
        &source,
        MockStorage::default(),
        FixedMetadata,
        AudioRecorderConfig::default(),
        FlacConfig {
            bits_per_sample: 8,
            ..FlacConfig::default()
        },
    )
    .unwrap();
    block_on(recorder.start()).unwrap();
    block_on(recorder.stop()).unwrap();
    assert!(recorder.storage().files.is_empty());
    block_on(recorder.start()).unwrap();
    source.write(&[128], 0).unwrap();
    block_on(recorder.record_next()).unwrap();
    assert_eq!(
        block_on(recorder.stop()),
        Err(AudioRecorderError::Codec(FlacError::SampleOutOfRange))
    );
    assert_eq!(recorder.storage().finishes, 1);
    block_on(recorder.stop()).unwrap();
}

struct PartialMinuteMetadata;

impl MetadataSource for PartialMinuteMetadata {
    fn snapshot(&self) -> Option<RecordingMetadata> {
        Some(RecordingMetadata::new(
            UtcTimestamp::new(1_700_000_038, 500_000).unwrap(),
        ))
    }
}

#[test]
fn first_recording_is_shortened_then_successors_start_on_the_boundary() {
    let source = AudioSource::<128, 1>::new(AudioFormat::new(4, 1, 1_000_000));
    let config = AudioRecorderConfig {
        recording_seconds: 60,
        storage_layout: StorageLayout::HourlyFolders,
    };
    let mut recorder = AudioRecorder::<_, _, 128, 1, 64>::new(
        &source,
        MockStorage::default(),
        PartialMinuteMetadata,
        config,
    )
    .unwrap();

    block_on(recorder.start()).unwrap();
    // 1_700_000_040 is the next multiple of 60: 1.5 seconds, or six frames.
    source.write(&[1, 2, 3, 4, 5, 6], 0).unwrap();
    let progress = block_on(recorder.record_next()).unwrap();

    assert!(progress.rotated);
    assert_eq!(recorder.storage().starts.len(), 2);
    assert_eq!(recorder.storage().starts[0].seconds, 1_700_000_038);
    assert_eq!(recorder.storage().starts[0].microseconds, 500_000);
    assert_eq!(recorder.storage().starts[1].seconds, 1_700_000_040);
    assert_eq!(recorder.storage().starts[1].microseconds, 0);
    assert_eq!(recorder.storage().starts[1].seconds % 60, 0);
    assert_eq!(
        u32::from_le_bytes(recorder.storage().writes[0][508..512].try_into().unwrap()),
        24
    );
    assert_eq!(
        u32::from_le_bytes(recorder.storage().writes[2][508..512].try_into().unwrap()),
        960
    );
}
