//! Paired audio clients share the same serialized exFAT owner as syslog.
use super::*;
#[cfg(feature = "bench-glitches")]
static DELAYED: core::sync::atomic::AtomicBool = core::sync::atomic::AtomicBool::new(false);
#[derive(Debug, defmt::Format)]
pub enum RecordError {
    Storage(StorageError),
    BenchComplete,
    FaultLatched,
}
#[derive(Clone, Copy)]
struct Active {
    handle: StreamHandle,
    bytes: u64,
    hash: u32,
    prepared: bool,
    complete: bool,
}
pub struct Recording {
    pub opened: u32,
    pub starts: [i64; BENCH_SEGMENTS],
    pub lengths: [[u32; 2]; BENCH_SEGMENTS],
    pub hashes: [[u32; 2]; BENCH_SEGMENTS],
    pub streaminfo_hashes: [u32; BENCH_SEGMENTS],
    pub frames: [u64; BENCH_SEGMENTS],
    active: [Option<Active>; 2],
}
impl Recording {
    pub fn new() -> Self {
        Self {
            opened: 0,
            starts: [0; BENCH_SEGMENTS],
            lengths: [[0; 2]; BENCH_SEGMENTS],
            hashes: [[0; 2]; BENCH_SEGMENTS],
            streaminfo_hashes: [0; BENCH_SEGMENTS],
            frames: [0; BENCH_SEGMENTS],
            active: [None; 2],
        }
    }
    pub async fn prepare_at(&mut self, utc: UtcTimestamp) -> Result<(), RecordError> {
        self.admit_audio_bytes(config::AUDIO_FILE_BYTES).await?;
        for suffix in ["flac", "CRCL"] {
            self.begin_audio_stream(StorageLayout::HourlyFolders, utc, suffix)
                .await?;
        }
        for slot in self.active.iter_mut().flatten() {
            slot.prepared = true;
        }
        Ok(())
    }
    pub async fn export_fixture(&mut self) {
        #[cfg(feature = "bench-codec")]
        crate::quad_fixture::export().await;
    }
    fn lane(&self, stream: StreamHandle) -> usize {
        self.active
            .iter()
            .position(|s| s.is_some_and(|s| s.handle == stream))
            .unwrap()
    }
    fn index(&self) -> usize {
        (self.opened as usize - 1) % BENCH_SEGMENTS
    }
}
impl RecordingStorage for Recording {
    type Error = RecordError;
    type Handle = StreamHandle;
    async fn admit_audio_bytes(&mut self, bytes: u64) -> Result<(), RecordError> {
        let mut guard = STORAGE.lock().await;
        let storage = guard.as_mut().unwrap();
        let (_, free, cluster) = storage
            .space_info()
            .await
            .map_err(RecordError::Storage)?
            .unwrap();
        // Two companions can consume one more cluster than rounding their sum.
        let need = bytes.div_ceil(cluster as u64) * cluster as u64 + cluster as u64;
        if free
            < config::LOG_RESERVE_BYTES
                .saturating_add(need)
                .saturating_add(storage.pending_bytes())
        {
            return Err(RecordError::Storage(StorageServiceError::OutOfSpace));
        }
        Ok(())
    }
    async fn begin_audio_stream(
        &mut self,
        layout: StorageLayout,
        utc: UtcTimestamp,
        suffix: &str,
    ) -> Result<StreamHandle, RecordError> {
        if crate::fault::active() {
            return Err(RecordError::FaultLatched);
        }
        let lane = if suffix == "flac" {
            0
        } else {
            defmt::assert!(suffix == "CRCL");
            1
        };
        if let Some(active) = &mut self.active[lane] {
            defmt::assert!(
                active.prepared
                    && self.starts[(self.opened as usize - 1) % BENCH_SEGMENTS] == utc.seconds
            );
            active.prepared = false;
            return Ok(active.handle);
        }
        if lane == 0
            && ((cfg!(any(feature = "bench-codec", feature = "bench-audio-ui"))
                && self.opened >= 2)
                || (cfg!(feature = "bench-gps-cycle") && crate::gnss::BENCH_FINISH.load(Relaxed)))
        {
            return Err(RecordError::BenchComplete);
        }
        defmt::assert!(
            lane != 0 || !cfg!(feature = "bench-gps-cycle") || self.opened < BENCH_SEGMENTS as u32,
            "L4 QUAD bench GPS cycle timeout"
        );
        let handle = STORAGE
            .lock()
            .await
            .as_mut()
            .unwrap()
            .begin_new_audio_with_suffix_at(layout, utc, suffix)
            .await
            .map_err(RecordError::Storage)?;
        if lane == 0 {
            self.opened += 1;
            let index = self.index();
            self.starts[index] = utc.seconds;
        }
        self.active[lane] = Some(Active {
            handle,
            bytes: 0,
            hash: FNV_INITIAL,
            prepared: false,
            complete: false,
        });
        AUDIO_OPEN.store(true, Relaxed);
        opened();
        log(format_args!(
            "segment_begin index={} utc={} stream={} channels={} rate=16000 bits=24",
            self.opened,
            utc.seconds,
            suffix,
            if lane == 0 { 1 } else { 3 }
        ));
        defmt::info!(
            "L4 QUAD begin index={} utc={} stream={} open={}",
            self.opened,
            utc.seconds,
            suffix,
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
        #[cfg(feature = "bench-glitches")]
        if crate::audio::PUBLISHED_SAMPLES.load(Relaxed) >= 59 * 16000 * 4 + 48000
            && !DELAYED.swap(true, Relaxed)
        {
            defmt::warn!("L4 GLITCH inject audio append delay_ms=1500 across minute boundary");
            // Delay the audio sink; keep syslog serviceable. This simulates
            // consumer backpressure, without altering card contents or drivers.
            embassy_time::Timer::after_millis(1500).await;
        }
        STORAGE
            .lock()
            .await
            .as_mut()
            .unwrap()
            .write(stream, bytes)
            .await
            .map_err(RecordError::Storage)?;
        let lane = self.lane(stream);
        #[cfg(feature = "bench-codec")]
        crate::quad_fixture::encoded(lane, bytes);
        let active = self.active[lane].as_mut().unwrap();
        active.hash = if lane == 0 {
            recording_hash(active.hash, active.bytes, bytes)
        } else {
            hash_bytes(active.hash, bytes)
        };
        active.bytes += bytes.len() as u64;
        if lane == 1 && bytes.len() == 52 && &bytes[..4] == b"CRBK" && bytes[4] == 3 {
            active.complete = u32::from_le_bytes(bytes[48..52].try_into().unwrap()) == 1;
        }
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
        defmt::assert!(self.lane(stream) == 0 && offset == 8 && bytes.len() == 34);
        let properties = u64::from_be_bytes(bytes[10..18].try_into().unwrap());
        let frames = properties & ((1 << 36) - 1);
        defmt::assert!(valid_streaminfo(bytes, frames));
        let index = self.index();
        self.streaminfo_hashes[index] = hash_bytes(FNV_INITIAL, bytes);
        self.frames[index] = frames;
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
        let lane = self.lane(stream);
        let active = self.active[lane].take().unwrap();
        let index = self.index();
        self.lengths[index][lane] = active.bytes as u32;
        self.hashes[index][lane] = active.hash;
        closed();
        if lane == 1 {
            defmt::assert!(self.active[0].is_none());
            AUDIO_OPEN.store(false, Relaxed);
            if !active.complete {
                log(format_args!(
                    "segment_incomplete index={} flac_bytes={} crcl_bytes={}",
                    self.opened, self.lengths[index][0], self.lengths[index][1]
                ));
                return Ok(());
            }
            CLOSED.fetch_add(1, Relaxed);
            log(format_args!(
                "segment_closed index={} frames={} flac_bytes={} crcl_bytes={}",
                self.opened, self.frames[index], self.lengths[index][0], self.lengths[index][1]
            ));
            defmt::info!("AARDWOLF L4 closed index={} frames={} flac_bytes={} crcl_bytes={} flac_hash={:08x} crcl_hash={:08x}",self.opened,self.frames[index],self.lengths[index][0],self.lengths[index][1],self.hashes[index][0],self.hashes[index][1]);
        }
        Ok(())
    }
}
