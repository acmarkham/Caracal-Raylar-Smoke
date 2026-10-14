use super::*;
use crate::crcl::{Completion, Transform};
use embassy_time::{with_timeout, Instant};

impl<S: RecordingStorage, M: MetadataSource, const C: usize, const R: usize>
    HexaphonicRecorder<'_, S, M, C, R>
{
    pub(super) async fn read_next(
        &mut self,
        timeout: Option<Duration>,
    ) -> Result<RecorderProgress, AudioRecorderError<S::Error>> {
        let start = self.started_utc.ok_or(AudioRecorderError::NotStarted)?;
        if let Some(duration) = timeout {
            with_timeout(duration, self.reader.wait_for_samples(6))
                .await
                .map_err(|_| AudioRecorderError::SourceTimeout)?
                .map_err(AudioRecorderError::AudioSource)?;
        } else {
            self.reader
                .wait_for_samples(6)
                .await
                .map_err(AudioRecorderError::AudioSource)?;
        }
        let remaining = if self.frames_target == 0 {
            self.config.spatial.block_frames
        } else {
            self.frames_target - self.frames_accepted - self.pcm_frames
        };
        let requested = (self.config.spatial.block_frames - self.pcm_frames).min(remaining) * 6;
        let expected = self.expected_position;
        let expected_ticks = self.expected_ticks;
        let start_ticks = if self.seeking_start {
            self.metadata.system_ticks_at(start)
        } else {
            None
        };
        let rate = self.rate as u64;
        let hz = self.tick_hz as u64;
        let mut first = None;
        let mut copied = 0;
        let mut gap = false;
        let mut next = None;
        let pcm = &mut self.buffers.pcm;
        let offset = self.pcm_frames * 6;
        let status = self
            .reader
            .read(requested, |chunk| {
                gap = chunk.dropped_since_last_read != 0
                    || expected.is_some_and(|p| p != chunk.position)
                    || expected_ticks
                        .zip(chunk.timestamp_ticks)
                        .is_some_and(|(a, b)| a.abs_diff(b) > 2 * (hz / rate).max(1));
                if gap {
                    return 0;
                }
                let Some(ticks) = chunk.timestamp_ticks else {
                    gap = true;
                    return 0;
                };
                let frames = chunk.len() / 6;
                let skip = start_ticks
                    .map(|t| {
                        t.saturating_sub(ticks) * rate / hz
                            + u64::from(
                                t > ticks && !(t - ticks).saturating_mul(rate).is_multiple_of(hz),
                            )
                    })
                    .unwrap_or(0)
                    .min(frames as u64) as usize;
                // When opening, limit this first read to the shared UTC end cutoff too.
                let end_ticks = start
                    .seconds
                    .checked_add(self.config.recorder.recording_seconds.into())
                    .and_then(|s| UtcTimestamp::new(s, start.microseconds))
                    .and_then(|u| self.metadata.system_ticks_at(u));
                let accepted = if self.reference_handle.is_none() {
                    let cutoff = end_ticks
                        .map(|t| t.saturating_sub(ticks).saturating_mul(rate).div_ceil(hz) as usize)
                        .unwrap_or(frames);
                    frames.min(cutoff).saturating_sub(skip)
                } else {
                    frames - skip
                };
                let take = accepted * 6;
                for (destination, value) in pcm[offset..offset + take]
                    .iter_mut()
                    .zip(chunk.first.iter().chain(chunk.second).skip(skip * 6))
                {
                    *destination = *value;
                }
                copied = accepted;
                if copied != 0 {
                    first = Some((
                        chunk.position / 6 + skip as u64,
                        ticks + skip as u64 * hz / rate,
                    ));
                }
                let consumed = (skip + accepted) * 6;
                next = Some((
                    chunk.position + consumed as u64,
                    ticks + (skip + accepted) as u64 * hz / rate,
                ));
                consumed
            })
            .map_err(AudioRecorderError::AudioSource)?;
        if gap || status.dropped_since_last_read != 0 {
            return Err(AudioRecorderError::Discontinuity {
                dropped_samples: status.dropped_since_last_read,
            });
        }
        if let Some((position, ticks)) = next {
            self.expected_position = Some(position);
            self.expected_ticks = Some(ticks);
        }
        if copied == 0 {
            if status.consumed_samples == 0 {
                // The requested segment has already expired; do not spin on
                // an unconsumable packet after a late start or invalid mapping.
                return Err(AudioRecorderError::TimeUnavailable);
            }
            return Ok(RecorderProgress {
                backlog_samples: self.reader.stats().available_samples,
                ..RecorderProgress::default()
            });
        }
        if self.reference_handle.is_none() {
            let (source, ticks) = first.ok_or(AudioRecorderError::InvalidConfig)?;
            self.open(source, ticks).await?;
        }
        self.pcm_frames += copied;
        let mut codec_us = 0;
        let mut storage_us = 0;
        if self.pcm_frames == self.config.spatial.block_frames
            || self.frames_accepted + self.pcm_frames == self.frames_target
        {
            let started = Instant::now();
            let timing = self.flush_block().await?;
            codec_us = timing.0;
            storage_us = started.elapsed().as_micros().saturating_sub(codec_us);
        }
        let rotated = self.frames_accepted == self.frames_target;
        if rotated {
            self.finish(Completion::Boundary).await?;
            let next = UtcTimestamp::new(
                start.seconds + self.config.recorder.recording_seconds as i64,
                start.microseconds,
            )
            .ok_or(AudioRecorderError::InvalidConfig)?;
            self.reset_segment(next);
        }
        Ok(RecorderProgress {
            pcm_samples: copied * 6,
            dropped_samples: 0,
            rotated,
            codec_elapsed_us: codec_us,
            storage_elapsed_us: storage_us,
            backlog_samples: self.reader.stats().available_samples,
        })
    }
    pub(super) async fn flush_block(&mut self) -> Result<(u64, u64), AudioRecorderError<S::Error>> {
        let frames = self.pcm_frames;
        let started = Instant::now();
        let mut transform =
            Transform::new(&mut self.buffers.pcm[..frames * 6], self.config.spatial)
                .map_err(AudioRecorderError::Spatial)?;
        while !transform
            .advance(256)
            .map_err(AudioRecorderError::Spatial)?
        {
            embassy_futures::yield_now().await;
        }
        let len = self
            .spatial
            .data(transform, self.buffers.spatial_output)
            .map_err(AudioRecorderError::Spatial)?;
        let codec_us = started.elapsed().as_micros();
        let write_started = Instant::now();
        let side = self.spatial_handle.ok_or(AudioRecorderError::NotStarted)?;
        self.storage
            .append_audio(side, &self.buffers.spatial_output[..len])
            .await
            .map_err(AudioRecorderError::Storage)?;
        self.spatial
            .commit_data(frames)
            .map_err(AudioRecorderError::Spatial)?;
        let reference_codec_us = self.feed_reference(frames).await?;
        self.frames_accepted += frames;
        self.pcm_frames = 0;
        let second = self.frames_accepted as u64 / u64::from(self.rate);
        if second > self.last_checkpoint_second && self.frames_accepted < self.frames_target {
            if let Some(metadata) = self.metadata.snapshot() {
                let ticks = self.first_ticks
                    + self.frames_accepted as u64 * u64::from(self.tick_hz) / u64::from(self.rate);
                let anchor = self.time_anchor(ticks, &metadata)?;
                let len = self
                    .spatial
                    .time(anchor, self.buffers.spatial_output)
                    .map_err(AudioRecorderError::Spatial)?;
                self.storage
                    .append_audio(side, &self.buffers.spatial_output[..len])
                    .await
                    .map_err(AudioRecorderError::Storage)?;
                self.spatial
                    .commit_time()
                    .map_err(AudioRecorderError::Spatial)?;
            }
            self.last_checkpoint_second = second;
        }
        Ok((
            codec_us + reference_codec_us,
            write_started
                .elapsed()
                .as_micros()
                .saturating_sub(reference_codec_us),
        ))
    }
}
