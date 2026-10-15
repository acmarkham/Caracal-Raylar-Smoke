use super::*;

pub(super) fn wav_header(
    out: &mut [u8],
    rate: u32,
    frames: usize,
    comment: &str,
) -> Result<usize, ()> {
    if out.len() < 1024 || comment.len() > 960 || rate.checked_mul(4).is_none() {
        return Err(());
    }
    let bytes = u32::try_from(frames)
        .map_err(|_| ())?
        .checked_mul(4)
        .ok_or(())?;
    out[..1024].fill(0);
    out[..4].copy_from_slice(b"RIFF");
    out[4..8].copy_from_slice(&(1016 + bytes).to_le_bytes());
    out[8..12].copy_from_slice(b"WAVE");
    out[12..16].copy_from_slice(b"fmt ");
    out[16..20].copy_from_slice(&16u32.to_le_bytes());
    out[20..24].copy_from_slice(&[1, 0, 1, 0]);
    out[24..28].copy_from_slice(&rate.to_le_bytes());
    out[28..32].copy_from_slice(&(rate * 4).to_le_bytes());
    out[32..36].copy_from_slice(&[4, 0, 32, 0]);
    out[36..40].copy_from_slice(b"LIST");
    out[40..44].copy_from_slice(&972u32.to_le_bytes());
    out[44..48].copy_from_slice(b"INFO");
    out[48..52].copy_from_slice(b"ICMT");
    out[52..56].copy_from_slice(&960u32.to_le_bytes());
    out[56..56 + comment.len()].copy_from_slice(comment.as_bytes());
    out[1016..1020].copy_from_slice(b"data");
    out[1020..1024].copy_from_slice(&bytes.to_le_bytes());
    Ok(1024)
}

impl<S: RecordingStorage, M: MetadataSource, const C: usize, const R: usize>
    HexaphonicRecorder<'_, S, M, C, R>
{
    pub(super) async fn feed_reference(
        &mut self,
        frames: usize,
    ) -> Result<u64, AudioRecorderError<S::Error>> {
        let mut codec_us = 0;
        if self.flac.is_none() {
            let capacity = self.buffers.reference_output.len() / 4;
            let reference = self
                .reference_handle
                .ok_or(AudioRecorderError::NotStarted)?;
            for start in (0..frames).step_by(capacity) {
                let count = capacity.min(frames - start);
                for n in 0..count {
                    let bytes = self.buffers.pcm[(start + n) * 6].to_le_bytes();
                    if !self.config.spatial.is_cheap() {
                        self.hash.update(&bytes);
                    }
                    self.buffers.reference_output[n * 4..n * 4 + 4].copy_from_slice(&bytes);
                }
                self.storage
                    .append_audio(reference, &self.buffers.reference_output[..count * 4])
                    .await
                    .map_err(AudioRecorderError::Storage)?;
                self.reference_bytes += (count * 4) as u64;
                self.reference_frames += count;
            }
        } else {
            let block = self.flac.as_ref().unwrap().config().block_samples;
            for n in 0..frames {
                let value = self.buffers.pcm[n * 6];
                if !self.config.spatial.is_cheap() {
                    self.hash.update(&value.to_le_bytes());
                }
                self.buffers.reference_pcm[self.reference_len] = value;
                self.reference_len += 1;
                if self.reference_len == block {
                    codec_us += self.flush_reference().await?;
                }
            }
        }
        Ok(codec_us)
    }
    pub(super) async fn flush_reference(&mut self) -> Result<u64, AudioRecorderError<S::Error>> {
        if self.reference_len == 0 {
            return Ok(0);
        }
        let started = embassy_time::Instant::now();
        let flac = self
            .flac
            .as_mut()
            .ok_or(AudioRecorderError::InvalidConfig)?;
        let mut analysis = flac
            .analyze(&self.buffers.reference_pcm[..self.reference_len])
            .map_err(AudioRecorderError::Codec)?;
        while !{
            let _profile =
                raylar_stage_profiling::Span::new(raylar_stage_profiling::Stage::FlacAnalysis);
            analysis.advance(256)
        } {
            embassy_futures::yield_now().await;
        }
        let frame = {
            let _profile =
                raylar_stage_profiling::Span::new(raylar_stage_profiling::Stage::FlacEncode);
            flac.encode_analyzed(analysis, self.buffers.reference_output)
                .map_err(AudioRecorderError::Codec)?
        };
        let codec_us = started.elapsed().as_micros();
        let handle = self
            .reference_handle
            .ok_or(AudioRecorderError::NotStarted)?;
        self.storage
            .append_audio(handle, &self.buffers.reference_output[..frame.bytes])
            .await
            .map_err(AudioRecorderError::Storage)?;
        flac.commit(frame).map_err(AudioRecorderError::Codec)?;
        self.reference_frames += frame.samples;
        self.reference_bytes += frame.bytes as u64;
        self.reference_len = 0;
        Ok(codec_us)
    }
}
