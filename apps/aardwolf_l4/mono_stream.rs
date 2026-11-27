use super::*;
#[derive(Debug, defmt::Format)]
pub enum RecordError {
    Storage(StorageError),
    BenchComplete,
    FaultLatched,
}
pub struct Recording {
    pub opened: u32,
    pub hashes: [u32; BENCH_SEGMENTS],
    pub starts: [i64; BENCH_SEGMENTS],
    pub lengths: [u32; BENCH_SEGMENTS],
    pub streaminfo_hashes: [u32; BENCH_SEGMENTS],
    pub workspace: Option<&'static mut [u8; VERIFY_BYTES]>,
    fixture_header: usize,
    fixture_length: usize,
    prepared: Option<(i64, StreamHandle)>,
    bytes: u64,
    hash: u32,
}
impl Recording {
    pub fn new() -> Self {
        Self {
            opened: 0,
            hashes: [0; BENCH_SEGMENTS],
            starts: [0; BENCH_SEGMENTS],
            lengths: [0; BENCH_SEGMENTS],
            streaminfo_hashes: [0; BENCH_SEGMENTS],
            workspace: Some(crate::verify::workspace()),
            fixture_header: 0,
            fixture_length: 0,
            prepared: None,
            bytes: 0,
            hash: FNV_INITIAL,
        }
    }
    pub async fn prepare_at(&mut self, utc: UtcTimestamp) -> Result<(), RecordError> {
        let handle = self
            .begin_audio_stream(StorageLayout::HourlyFolders, utc, CODEC)
            .await?;
        self.prepared = Some((utc.seconds, handle));
        Ok(())
    }
    pub async fn export_fixture(&mut self) {
        if !cfg!(feature = "bench-codec") || cfg!(feature = "wav") {
            return;
        }
        defmt::assert!(self.fixture_length > self.fixture_header && self.fixture_header >= 42);
        let bytes = self.workspace.as_mut().unwrap();
        let frame_bytes = (self.fixture_length - self.fixture_header) as u32;
        // The captured first frame is frame zero. Finalize a standalone
        // one-block fixture without changing any of its encoded frame bytes.
        bytes[12..15].copy_from_slice(&frame_bytes.to_be_bytes()[1..]);
        bytes[15..18].copy_from_slice(&frame_bytes.to_be_bytes()[1..]);
        let properties =
            (u64::from(SAMPLE_RATE) << 44) | (u64::from(SOURCE_BITS - 1) << 36) | FLAC_BLOCK as u64;
        bytes[18..26].copy_from_slice(&properties.to_be_bytes());
        defmt::info!(
            "AARDWOLF L4 FIXTURE bytes={} pcm_samples={} pcm_hash={:08x}",
            self.fixture_length,
            FLAC_BLOCK,
            crate::audio::FIRST_PCM_HASH.load(Relaxed)
        );
        for (n, chunk) in bytes[..self.fixture_length].chunks(64).enumerate() {
            defmt::info!(
                "AARDWOLF L4 FIXTURE offset={} data={=[u8]:02x}",
                n * 64,
                chunk
            );
            embassy_time::Timer::after_millis(100).await;
        }
    }
}
impl RecordingStorage for Recording {
    type Error = RecordError;
    type Handle = StreamHandle;
    async fn begin_audio_stream(
        &mut self,
        layout: StorageLayout,
        utc: UtcTimestamp,
        suffix: &str,
    ) -> Result<StreamHandle, RecordError> {
        if let Some((prepared_utc, handle)) = self.prepared.take() {
            defmt::assert!(utc.seconds == prepared_utc && suffix == CODEC);
            return Ok(handle);
        }
        // The preceding segment has already been finalized by AudioRecorder.
        // Stop the bench before allocating another file, without cancelling I/O.
        if (cfg!(feature = "bench-gps-cycle") && crate::gnss::BENCH_FINISH.load(Relaxed))
            || (cfg!(any(feature = "bench-audio-ui", feature = "bench-codec")) && self.opened >= 2)
        {
            return Err(RecordError::BenchComplete);
        }
        if crate::fault::active() {
            return Err(RecordError::FaultLatched);
        }
        defmt::assert!(
            !cfg!(feature = "bench-gps-cycle") || self.opened < BENCH_SEGMENTS as u32,
            "AARDWOLF L4 bench GPS cycle timeout"
        );
        let mut guard = STORAGE.lock().await;
        let storage = guard.as_mut().unwrap();
        let (_, free, cluster) = storage
            .space_info()
            .await
            .map_err(RecordError::Storage)?
            .unwrap();
        if !aardwolf_l4::policy::can_start_audio(free, cluster, storage.pending_bytes()) {
            return Err(RecordError::Storage(StorageServiceError::OutOfSpace));
        }
        let handle = storage
            .begin_new_audio_with_suffix_at(layout, utc, suffix)
            .await
            .map_err(RecordError::Storage)?;
        if (self.opened as usize) < BENCH_SEGMENTS {
            self.starts[self.opened as usize] = utc.seconds;
        }
        self.opened += 1;
        self.bytes = 0;
        self.hash = FNV_INITIAL;
        AUDIO_OPEN.store(true, Relaxed);
        opened();
        log(format_args!(
            "segment_begin index={} utc={} sample_rate=16000 channels=1 codec={} source_bits=24",
            self.opened, utc.seconds, CODEC
        ));
        defmt::info!(
            "AARDWOLF L4 begin index={} utc={} open_streams={}",
            self.opened,
            utc.seconds,
            OPEN.load(Relaxed)
        );
        Ok(handle)
    }
    async fn append_audio(
        &mut self,
        stream: StreamHandle,
        bytes: &[u8],
    ) -> Result<(), RecordError> {
        if crate::fault::active() {
            return Err(RecordError::FaultLatched);
        }
        let start = Instant::now();
        STORAGE
            .lock()
            .await
            .as_mut()
            .unwrap()
            .write(stream, bytes)
            .await
            .map_err(RecordError::Storage)?;
        if cfg!(feature = "bench-codec") && !cfg!(feature = "wav") && self.opened == 1 {
            let workspace = self.workspace.as_mut().unwrap();
            if self.bytes == 0 {
                defmt::assert!(bytes.len() <= 1024);
                workspace[..bytes.len()].copy_from_slice(bytes);
                self.fixture_header = bytes.len();
                self.fixture_length = bytes.len();
            } else if self.fixture_length == self.fixture_header {
                let end = self.fixture_length + bytes.len();
                defmt::assert!(end <= VERIFY_BYTES);
                workspace[self.fixture_length..end].copy_from_slice(bytes);
                self.fixture_length = end;
            }
        }
        self.hash = recording_hash(self.hash, self.bytes, bytes);
        self.bytes += bytes.len() as u64;
        MAX_APPEND_US.fetch_max((Instant::now() - start).as_micros() as u32, Relaxed);
        Ok(())
    }
    async fn rewrite_audio(
        &mut self,
        stream: StreamHandle,
        offset: u64,
        bytes: &[u8],
    ) -> Result<(), RecordError> {
        if crate::fault::active() {
            return Err(RecordError::FaultLatched);
        }
        // Every finished segment must be a complete minute, so these RIFF
        // patches equal the original header and do not change its running hash.
        if cfg!(feature = "wav") {
            defmt::assert!(
                (offset == 4 && bytes == ((FILE_BYTES - 8) as u32).to_le_bytes())
                    || (offset == 508 && bytes == (PAYLOAD_BYTES as u32).to_le_bytes())
            );
        } else {
            defmt::assert!(offset == 8 && valid_streaminfo(bytes, SEGMENT_SAMPLES as u64));
            if self.opened as usize <= BENCH_SEGMENTS {
                self.streaminfo_hashes[self.opened as usize - 1] = hash_bytes(FNV_INITIAL, bytes);
            }
        }
        STORAGE
            .lock()
            .await
            .as_mut()
            .unwrap()
            .rewrite(stream, offset, bytes)
            .await
            .map_err(RecordError::Storage)
    }
    async fn finish_audio(&mut self, stream: StreamHandle) -> Result<(), RecordError> {
        if crate::fault::active() {
            return Err(RecordError::FaultLatched);
        }
        STORAGE
            .lock()
            .await
            .as_mut()
            .unwrap()
            .finish(stream)
            .await
            .map_err(RecordError::Storage)?;
        defmt::assert!(
            LOG_OPEN.load(Relaxed)
                && self.bytes <= FILE_BUDGET
                && if cfg!(feature = "wav") {
                    self.bytes == FILE_BYTES
                } else {
                    self.bytes > 42
                }
        );
        if (self.opened as usize) <= BENCH_SEGMENTS {
            self.hashes[self.opened as usize - 1] = self.hash;
            self.lengths[self.opened as usize - 1] = self.bytes as u32;
        }
        CLOSED.fetch_add(1, Relaxed);
        AUDIO_OPEN.store(false, Relaxed);
        closed();
        log(format_args!(
            "segment_closed index={} bytes={} samples=960000 normalized={} hash={:08x}",
            self.opened,
            self.bytes,
            !cfg!(feature = "wav"),
            self.hash
        ));
        defmt::info!(
            "AARDWOLF L4 closed index={} bytes={} samples=960000 normalized={} hash={:08x}",
            self.opened,
            self.bytes,
            !cfg!(feature = "wav"),
            self.hash
        );
        Ok(())
    }
}
