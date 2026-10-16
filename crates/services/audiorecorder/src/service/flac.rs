use super::*;

impl<
    'a,
    S,
    M,
    const AUDIO_CAPACITY: usize,
    const READERS: usize,
    const ENCODE_BYTES: usize,
    const PCM_SAMPLES: usize,
    W,
> AudioRecorder<'a, S, M, AUDIO_CAPACITY, READERS, ENCODE_BYTES, PCM_SAMPLES, W>
where
    S: RecordingStorage,
    M: MetadataSource,
    W: RecorderBuffers<ENCODE_BYTES, PCM_SAMPLES>,
{
    pub(super) async fn record_flac(
        &mut self,
        timeout: Option<Duration>,
    ) -> Result<RecorderProgress, AudioRecorderError<S::Error>> {
        if !self.started {
            return Err(AudioRecorderError::NotStarted);
        }
        let block = match &self.container {
            Container::Flac(e) => e.config().block_samples,
            _ => unreachable!(),
        };
        let remaining = self.samples_per_recording - self.samples_in_recording - self.pcm_len;
        let requested = (block - self.pcm_len).min(remaining);
        if let Some(timeout) = timeout {
            with_timeout(timeout, self.reader.wait_for_samples(1))
                .await
                .map_err(|_| AudioRecorderError::SourceTimeout)?
                .map_err(AudioRecorderError::AudioSource)?;
        } else {
            self.reader
                .wait_for_samples(1)
                .await
                .map_err(AudioRecorderError::AudioSource)?;
        }
        let (_, pcm) = self.buffers.split();
        let offset = self.pcm_len;
        let expected = self.expected_position;
        let expected_ticks = self.expected_timestamp;
        let tick_hz = self.timestamp_hz;
        let rate = self.sample_rate_hz;
        let mut gap = false;
        let mut next_position = None;
        let mut next_ticks = None;
        let status = self
            .reader
            .read(requested, |chunk| {
                gap = chunk.dropped_since_last_read != 0
                    || expected.is_some_and(|position| position != chunk.position)
                    || expected_ticks
                        .zip(chunk.timestamp_ticks)
                        .is_some_and(|(a, b)| {
                            a.abs_diff(b) > (u64::from(tick_hz) / u64::from(rate)).max(1) * 2
                        });
                if gap {
                    return 0;
                }
                for (target, sample) in pcm[offset..]
                    .iter_mut()
                    .zip(chunk.first.iter().chain(chunk.second))
                {
                    *target = *sample;
                }
                next_position = Some(chunk.position + chunk.len() as u64);
                next_ticks = chunk
                    .timestamp_ticks
                    .map(|ticks| ticks + chunk.len() as u64 * u64::from(tick_hz) / u64::from(rate));
                chunk.len()
            })
            .map_err(AudioRecorderError::AudioSource)?;
        if gap || status.dropped_since_last_read != 0 {
            if self.stream.is_some() {
                self.finish_recording().await?;
            }
            self.started = false;
            self.pending_start = None;
            self.next_recording_started_utc = None;
            return Err(AudioRecorderError::Discontinuity {
                dropped_samples: status.dropped_since_last_read,
            });
        }
        self.expected_position = next_position;
        self.expected_timestamp = next_ticks;
        self.pcm_len += status.consumed_samples;
        if self.stream.is_none() && self.pcm_len != 0 {
            self.open_flac_stream().await?;
        }
        let mut timing = (0, 0);
        let mut encoded_frames = 0;
        if self.pcm_len == block
            || self.samples_in_recording + self.pcm_len == self.samples_per_recording
        {
            encoded_frames = self.pcm_len;
            timing = self.append_flac_frame().await?;
        }
        let rotated = self.samples_in_recording == self.samples_per_recording;
        if rotated {
            self.finish_recording().await?;
            self.begin_recording().await?;
        }
        Ok(RecorderProgress {
            pcm_samples: status.consumed_samples,
            encoded_frames,
            dropped_samples: 0,
            rotated,
            codec_elapsed_us: timing.0,
            storage_elapsed_us: timing.1,
            backlog_samples: self.reader.stats().available_samples,
        })
    }

    pub(super) async fn append_flac_frame(
        &mut self,
    ) -> Result<(u64, u64), AudioRecorderError<S::Error>> {
        let stream = self.stream.ok_or(AudioRecorderError::NotStarted)?;
        let encoder = match &mut self.container {
            Container::Flac(e) => e,
            _ => unreachable!(),
        };
        let (output, pcm) = self.buffers.split();
        let encode_started = embassy_time::Instant::now();
        let encoded = async {
            let mut analysis = encoder.analyze(&pcm[..self.pcm_len])?;
            while !analysis.advance(256) {
                embassy_futures::yield_now().await;
            }
            encoder.encode_analyzed(analysis, output)
        }
        .await;
        let codec_elapsed_us = encode_started.elapsed().as_micros();
        let frame = match encoded {
            Ok(frame) => frame,
            Err(error) => {
                self.pcm_len = 0;
                let _ = self.storage.finish_audio(stream).await;
                self.stream = None;
                self.started = false;
                return Err(AudioRecorderError::Codec(error));
            }
        };
        let write_started = embassy_time::Instant::now();
        if let Err(error) = self
            .storage
            .append_audio(stream, &output[..frame.bytes])
            .await
        {
            // Source samples have been consumed; a failed append must never be replayed.
            self.pcm_len = 0;
            let _ = self.storage.finish_audio(stream).await;
            self.stream = None;
            self.started = false;
            return Err(AudioRecorderError::Storage(error));
        }
        encoder.commit(frame).map_err(AudioRecorderError::Codec)?;
        self.samples_in_recording += frame.samples;
        self.pcm_len = 0;
        Ok((codec_elapsed_us, write_started.elapsed().as_micros()))
    }

    async fn open_flac_stream(&mut self) -> Result<(), AudioRecorderError<S::Error>> {
        let utc = self.pending_start.ok_or(AudioRecorderError::NotStarted)?;
        let stream = self
            .storage
            .begin_audio_stream(self.storage_layout, utc, self.container.suffix())
            .await
            .map_err(AudioRecorderError::Storage)?;
        if let Err(error) = self
            .storage
            .append_audio(stream, &self.buffers.encoded()[..self.pending_header_len])
            .await
        {
            let _ = self.storage.finish_audio(stream).await;
            self.started = false;
            return Err(AudioRecorderError::Storage(error));
        }
        self.pending_start = None;
        self.pending_header_len = 0;
        self.stream = Some(stream);
        Ok(())
    }
}
