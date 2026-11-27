use super::*;
use crate::crcl::{ChannelTransform, Completion};
use embassy_time::{with_timeout, Instant};

impl<S: RecordingStorage, M: MetadataSource, const C: usize, const R: usize, const N: usize>
    ChannelRecorder<'_, S, M, C, R, N>
{
    pub(super) async fn read_next(
        &mut self,
        timeout: Option<Duration>,
    ) -> Result<RecorderProgress, AudioRecorderError<S::Error>> {
        let start = self.started_utc.ok_or(AudioRecorderError::NotStarted)?;
        if let Some(duration) = timeout {
            with_timeout(duration, self.reader.wait_for_samples(N))
                .await
                .map_err(|_| AudioRecorderError::SourceTimeout)?
                .map_err(AudioRecorderError::AudioSource)?;
        } else {
            self.reader
                .wait_for_samples(N)
                .await
                .map_err(AudioRecorderError::AudioSource)?;
        }
        let remaining = if self.frames_target == 0 {
            self.config.spatial.block_frames
        } else {
            self.frames_target - self.frames_accepted - self.pcm_frames
        };
        let requested = (self.config.spatial.block_frames - self.pcm_frames).min(remaining) * N;
        let expected = self.expected_position;
        let expected_ticks = self.expected_ticks;
        let fill_gaps = self.fill_gaps_with_silence;
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
        let mut silence_samples = 0;
        let mut advanced = false;
        // Keep time-model arithmetic outside the AudioSource callback. An
        // interrupt-safe producer must not wait for it while PCM is borrowed.
        let end_ticks = start
            .seconds
            .checked_add(self.config.recorder.recording_seconds.into())
            .and_then(|s| UtcTimestamp::new(s, start.microseconds))
            .and_then(|u| self.metadata.system_ticks_at(u));
        let pcm = &mut self.buffers.pcm;
        let offset = self.pcm_frames * N;
        let source_profile =
            raylar_stage_profiling::Span::new(raylar_stage_profiling::Stage::SourceCopy);
        let status = self
            .reader
            .read(requested, |chunk| {
                let Some(source_ticks) = chunk.timestamp_ticks else {
                    gap = true;
                    return 0;
                };
                let position = expected.unwrap_or_else(|| {
                    if fill_gaps {
                        chunk.position.saturating_sub(chunk.dropped_since_last_read)
                    } else {
                        chunk.position
                    }
                });
                let missing = chunk.position.saturating_sub(position);
                let padding = fill_gaps && missing != 0;
                let ticks = if padding {
                    expected_ticks.unwrap_or_else(|| {
                        source_ticks.saturating_sub(missing / N as u64 * hz / rate)
                    })
                } else {
                    source_ticks
                };
                // Only a measured forward sample-position gap is recoverable.
                // An epoch reset, unknown gap length or backwards clock is not.
                gap = position > chunk.position
                    || (!padding && (chunk.dropped_since_last_read != 0 || missing != 0))
                    || chunk.dropped_since_last_read > missing
                    || !missing.is_multiple_of(N as u64)
                    || expected_ticks.is_some_and(|t| {
                        let projected = t.saturating_add(missing / N as u64 * hz / rate);
                        projected.abs_diff(source_ticks) > 2 * (hz / rate).max(1)
                    });
                if gap {
                    return 0;
                }
                let position = if padding { position } else { chunk.position };
                let frames = if padding {
                    (missing / N as u64).min((requested / N) as u64) as usize
                } else {
                    chunk.len() / N
                };
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
                let accepted = if self.reference_handle.is_none() {
                    let cutoff = end_ticks
                        .map(|t| t.saturating_sub(ticks).saturating_mul(rate).div_ceil(hz) as usize)
                        .unwrap_or(frames);
                    frames.min(cutoff).saturating_sub(skip)
                } else {
                    frames - skip
                };
                let take = accepted * N;
                if padding {
                    pcm[offset..offset + take].fill(0);
                    silence_samples = take;
                } else {
                    for (destination, value) in pcm[offset..offset + take]
                        .iter_mut()
                        .zip(chunk.first.iter().chain(chunk.second).skip(skip * N))
                    {
                        *destination = *value;
                    }
                }
                copied = accepted;
                if copied != 0 {
                    first = Some((
                        position / N as u64 + skip as u64,
                        ticks + skip as u64 * hz / rate,
                    ));
                }
                let consumed = (skip + accepted) * N;
                next = Some((
                    position + consumed as u64,
                    ticks + (skip + accepted) as u64 * hz / rate,
                ));
                advanced = consumed != 0;
                if padding {
                    0
                } else {
                    consumed
                }
            })
            .map_err(AudioRecorderError::AudioSource)?;
        drop(source_profile);
        if gap || (!fill_gaps && status.dropped_since_last_read != 0) {
            return Err(AudioRecorderError::Discontinuity {
                dropped_samples: status.dropped_since_last_read,
            });
        }
        if let Some((position, ticks)) = next {
            self.expected_position = Some(position);
            self.expected_ticks = Some(ticks);
        }
        if copied == 0 {
            if !advanced {
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
        let mut encoded_frames = 0;
        if self.pcm_frames == self.config.spatial.block_frames
            || self.frames_accepted + self.pcm_frames == self.frames_target
        {
            encoded_frames = self.pcm_frames;
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
            pcm_samples: copied * N,
            encoded_frames,
            dropped_samples: silence_samples as u64,
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
            ChannelTransform::<N>::new(&mut self.buffers.pcm[..frames * N], self.config.spatial)
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
        let reference_codec_us = raylar_stage_profiling::measure(
            raylar_stage_profiling::Stage::ReferenceFeed,
            self.feed_reference(frames),
        )
        .await?;
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
