use super::*;
use crate::crcl::{segment_identity, Completion, Header, TimeAnchor};
use crate::{container::flac_comment, RecordingMetadata};
use core::fmt::Write;
use heapless::String;

impl<S: RecordingStorage, M: MetadataSource, const C: usize, const R: usize>
    HexaphonicRecorder<'_, S, M, C, R>
{
    pub(super) fn time_anchor(
        &self,
        ticks: u64,
        metadata: &RecordingMetadata,
    ) -> Result<TimeAnchor, AudioRecorderError<S::Error>> {
        let utc = self
            .metadata
            .utc_at_ticks(ticks)
            .ok_or(AudioRecorderError::TimeUnavailable)?;
        let utc_us =
            u64::try_from(utc.as_micros()).map_err(|_| AudioRecorderError::TimeUnavailable)?;
        Ok(TimeAnchor {
            capture_origin_ticks: self.capture_origin,
            sample_ticks: ticks,
            sample_utc_us: utc_us,
            uncertainty_us: metadata.uncertainty_us,
            utc_status: metadata.utc_status,
            calibration_locked: metadata.calibration_locked,
            calibration_ppb: metadata.calibration_ppb.unwrap_or(0),
            gps_pps: metadata
                .last_gps_pps_utc
                .zip(metadata.last_pps_system_ticks)
                .and_then(|(utc, t)| u64::try_from(utc.as_micros()).ok().map(|u| (u, t))),
            // The current metadata interface supplies no location age. Omit instead of inventing it.
            location: None,
        })
    }
    pub(super) async fn open(
        &mut self,
        source_frame: u64,
        ticks: u64,
    ) -> Result<(), AudioRecorderError<S::Error>> {
        let start = self.started_utc.ok_or(AudioRecorderError::NotStarted)?;
        let end = UtcTimestamp::new(
            start
                .seconds
                .checked_add(self.config.recorder.recording_seconds.into())
                .ok_or(AudioRecorderError::InvalidConfig)?,
            start.microseconds,
        )
        .ok_or(AudioRecorderError::InvalidConfig)?;
        let nominal = self.rate as usize * self.config.recorder.recording_seconds as usize;
        self.frames_target = if let Some(end_ticks) = self.metadata.system_ticks_at(end) {
            let duration = end_ticks
                .checked_sub(ticks)
                .ok_or(AudioRecorderError::InvalidConfig)?;
            usize::try_from(
                duration
                    .checked_mul(self.rate.into())
                    .ok_or(AudioRecorderError::InvalidConfig)?
                    .div_ceil(self.tick_hz.into()),
            )
            .map_err(|_| AudioRecorderError::InvalidConfig)?
        } else {
            nominal
        };
        if self.frames_target == 0 || self.frames_target > nominal + self.rate as usize {
            return Err(AudioRecorderError::InvalidConfig);
        }
        self.first_ticks = ticks;
        self.seeking_start = false;
        self.capture_origin = self
            .config
            .capture
            .origin_ticks
            .map(|f| f())
            .unwrap_or(ticks);
        let mut metadata = self
            .metadata
            .snapshot()
            .ok_or(AudioRecorderError::TimeUnavailable)?;
        metadata.started_utc = start;
        metadata.started_system_ticks = Some(ticks);
        let start_us =
            u64::try_from(start.as_micros()).map_err(|_| AudioRecorderError::TimeUnavailable)?;
        let boot = metadata.boot_id.ok_or(AudioRecorderError::InvalidConfig)?;
        let identity = segment_identity(&metadata.device_id, boot, start_us, source_frame)
            .map_err(AudioRecorderError::Spatial)?;
        let capture = self.config.capture;
        let mut capture_text = String::<192>::new();
        write!(capture_text,"pdm_clock_hz={};sinc_order={};decimation={};cic_scale={};hpf_enabled={};pcm_shift={};gain_tenths_db={}",
            capture.pdm_clock_hz,capture.sinc_order,capture.decimation,capture.cic_scale,u8::from(capture.hpf_enabled),capture.pcm_shift,capture.gain_tenths_db)
            .map_err(|_|AudioRecorderError::InvalidConfig)?;
        let mut firmware = String::<128>::new();
        write!(
            firmware,
            "{}{}{}",
            metadata.firmware_version,
            if metadata.firmware_hash.is_empty() {
                ""
            } else {
                ":"
            },
            metadata.firmware_hash
        )
        .map_err(|_| AudioRecorderError::InvalidConfig)?;
        let header = Header {
            identity,
            sample_rate_hz: self.rate,
            reference_format: if self.flac.is_some() { 2 } else { 1 },
            first_source_frame: source_frame,
            intended_utc_us: start_us,
            planned_frames: self.frames_target as u64,
            tick_hz: self.tick_hz,
            time: self.time_anchor(ticks, &metadata)?,
            device: &metadata.device_id,
            boot,
            firmware: &firmware,
            capture_config: &capture_text,
            calibration_id: self.config.calibration_id,
        };
        let spatial_len = self
            .spatial
            .header(&header, self.buffers.spatial_output)
            .map_err(AudioRecorderError::Spatial)?;
        let profile = self
            .flac
            .as_ref()
            .map(|f| f.config())
            .unwrap_or(FlacConfig {
                sample_rate_hz: self.rate,
                ..FlacConfig::default()
            });
        let mut comment =
            flac_comment(&metadata, profile).map_err(|_| AudioRecorderError::InvalidConfig)?;
        comment
            .push_str(";segment_id=")
            .map_err(|_| AudioRecorderError::InvalidConfig)?;
        for byte in identity {
            write!(comment, "{byte:02x}").map_err(|_| AudioRecorderError::InvalidConfig)?;
        }
        write!(
            comment,
            ";first_source_frame={source_frame};companion_format=CRCL"
        )
        .map_err(|_| AudioRecorderError::InvalidConfig)?;
        let reference_len = if let Some(flac) = &self.flac {
            flac.header(self.buffers.reference_output, &[comment.as_str()])
                .map_err(AudioRecorderError::Codec)?
        } else {
            reference::wav_header(
                self.buffers.reference_output,
                self.rate,
                self.frames_target,
                &comment,
            )
            .map_err(|_| AudioRecorderError::InvalidConfig)?
        };
        let bound = self.byte_budget(nominal + self.rate as usize)?;
        #[cfg(feature = "defmt")]
        defmt::info!(
            "paired audio admission frames={} worst_bytes={}",
            self.frames_target,
            bound
        );
        self.storage
            .admit_audio_bytes(bound)
            .await
            .map_err(AudioRecorderError::Storage)?;
        #[cfg(feature = "defmt")]
        defmt::info!("paired audio admission complete");
        let reference = self
            .storage
            .begin_audio_stream(
                self.config.recorder.storage_layout,
                start,
                if self.flac.is_some() { "flac" } else { "wav" },
            )
            .await
            .map_err(AudioRecorderError::Storage)?;
        self.reference_handle = Some(reference);
        let spatial = self
            .storage
            .begin_audio_stream(self.config.recorder.storage_layout, start, "CRCL")
            .await
            .map_err(AudioRecorderError::Storage)?;
        self.spatial_handle = Some(spatial);
        #[cfg(feature = "defmt")]
        defmt::info!("paired audio streams created");
        self.storage
            .append_audio(reference, &self.buffers.reference_output[..reference_len])
            .await
            .map_err(AudioRecorderError::Storage)?;
        self.reference_bytes = reference_len as u64;
        self.storage
            .append_audio(spatial, &self.buffers.spatial_output[..spatial_len])
            .await
            .map_err(AudioRecorderError::Storage)?;
        Ok(())
    }
    fn byte_budget(&self, frames: usize) -> Result<u64, AudioRecorderError<S::Error>> {
        let block = self.config.spatial.block_frames;
        let full = frames / block;
        let tail = frames % block;
        let side = full as u64
            * crate::crcl::record_capacity(block, self.config.spatial.bits) as u64
            + if tail == 0 {
                0
            } else {
                crate::crcl::record_capacity(tail, self.config.spatial.bits) as u64
            }
            + 4096
            + 548 * self.config.recorder.recording_seconds as u64
            + 92;
        let reference = frames as u64 * 4
            + 1024
            + self
                .flac
                .as_ref()
                .map(|f| 32 * (frames as u64).div_ceil(f.config().block_samples as u64))
                .unwrap_or(0);
        reference
            .checked_add(side)
            .ok_or(AudioRecorderError::InvalidConfig)
    }
    pub(super) async fn finish(
        &mut self,
        status: Completion,
    ) -> Result<(), AudioRecorderError<S::Error>> {
        if self.reference_handle.is_none() {
            return Ok(());
        }
        if self.pcm_frames != 0 {
            self.flush_block().await?;
        }
        self.flush_reference().await?;
        if self.reference_frames != self.spatial.frames() as usize
            || self.frames_accepted != self.reference_frames
        {
            return Err(AudioRecorderError::InvalidConfig);
        }
        let reference = self
            .reference_handle
            .ok_or(AudioRecorderError::NotStarted)?;
        let rewrite = if let Some(flac) = &self.flac {
            self.storage
                .rewrite_audio(reference, 8, &flac.streaminfo())
                .await
        } else {
            let bytes = (self.reference_frames as u32)
                .checked_mul(4)
                .ok_or(AudioRecorderError::InvalidConfig)?;
            self.storage
                .rewrite_audio(reference, 4, &(1016 + bytes).to_le_bytes())
                .await
                .map_err(AudioRecorderError::Storage)?;
            self.storage
                .rewrite_audio(reference, 1020, &bytes.to_le_bytes())
                .await
        };
        // A failed rewrite still closes the reference; END is withheld on any failure.
        self.reference_handle = None;
        let close = self.storage.finish_audio(reference).await;
        rewrite.map_err(AudioRecorderError::Storage)?;
        close.map_err(AudioRecorderError::Storage)?;
        let len = self
            .spatial
            .end(
                self.reference_bytes,
                self.hash.clone().finish(),
                status,
                self.buffers.spatial_output,
            )
            .map_err(AudioRecorderError::Spatial)?;
        let side = self.spatial_handle.ok_or(AudioRecorderError::NotStarted)?;
        self.storage
            .append_audio(side, &self.buffers.spatial_output[..len])
            .await
            .map_err(AudioRecorderError::Storage)?;
        self.spatial_handle = None;
        self.storage
            .finish_audio(side)
            .await
            .map_err(AudioRecorderError::Storage)
    }
}
