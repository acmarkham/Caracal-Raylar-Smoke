use crate::bitstream::Bits;
use crate::{Encoder, Error, METADATA_BYTES};

impl Encoder {
    /// The fixed-size STREAMINFO payload, for initial output and final rewrite.
    pub fn streaminfo(&self) -> [u8; 34] {
        let mut bytes = [0u8; 34];
        let b = self.config.block_samples as u16;
        bytes[..2].copy_from_slice(&b.to_be_bytes());
        bytes[2..4].copy_from_slice(&b.to_be_bytes());
        bytes[4..7].copy_from_slice(&self.min_frame.to_be_bytes()[1..]);
        bytes[7..10].copy_from_slice(&self.max_frame.to_be_bytes()[1..]);
        let properties = (u64::from(self.config.sample_rate_hz) << 44)
            | (u64::from(self.config.bits_per_sample - 1) << 36)
            | self.samples;
        bytes[10..18].copy_from_slice(&properties.to_be_bytes());
        // MD5 unavailable (zero), deliberately omitted by the minimal profile.
        bytes
    }

    /// Complete native header; comments must be bounded UTF-8 KEY=VALUE fields.
    pub fn header(&self, output: &mut [u8], comments: &[&str]) -> Result<usize, Error> {
        const VENDOR: &[u8] = b"raylar-flac-encoder";
        let comment_bytes = comments.iter().try_fold(8 + VENDOR.len(), |n, text| {
            if !text.split_once('=').is_some_and(|(key, _)| {
                !key.is_empty() && key.bytes().all(|b| (0x20..=0x7d).contains(&b) && b != b'=')
            }) {
                return Err(Error::InvalidConfig);
            }
            n.checked_add(4 + text.len()).ok_or(Error::OutputTooSmall)
        })?;
        let total = 46 + comment_bytes;
        if total > METADATA_BYTES || output.len() < total || comments.len() > u32::MAX as usize {
            return Err(Error::OutputTooSmall);
        }
        output[..4].copy_from_slice(b"fLaC");
        output[4..8].copy_from_slice(&[0, 0, 0, 34]);
        output[8..42].copy_from_slice(&self.streaminfo());
        let mut bits = Bits::new(&mut output[42..46]);
        bits.write(0x84, 8)?; // last metadata block: Vorbis comment
        bits.write(comment_bytes as u64, 24)?;
        let mut at = 46;
        output[at..at + 4].copy_from_slice(&(VENDOR.len() as u32).to_le_bytes());
        at += 4;
        output[at..at + VENDOR.len()].copy_from_slice(VENDOR);
        at += VENDOR.len();
        output[at..at + 4].copy_from_slice(&(comments.len() as u32).to_le_bytes());
        at += 4;
        for text in comments {
            output[at..at + 4].copy_from_slice(&(text.len() as u32).to_le_bytes());
            at += 4;
            output[at..at + text.len()].copy_from_slice(text.as_bytes());
            at += text.len();
        }
        Ok(at)
    }
}
