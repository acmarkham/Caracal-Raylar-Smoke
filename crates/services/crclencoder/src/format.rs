use crate::{
    bits::{fold, Bits},
    Config, Error, Prediction, Representation, Sha256, Transform,
};

const fn crc_table() -> [u32; 256] {
    let mut table = [0; 256];
    let mut i = 0;
    while i < 256 {
        let mut c = i as u32;
        let mut bit = 0;
        while bit < 8 {
            c = (c >> 1) ^ if c & 1 != 0 { 0x82f63b78 } else { 0 };
            bit += 1;
        }
        table[i] = c;
        i += 1;
    }
    table
}
pub fn crc32c(bytes: &[u8]) -> u32 {
    let _profile = raylar_stage_profiling::Span::new(raylar_stage_profiling::Stage::CrclCrc);
    static TABLE: [u32; 256] = crc_table();
    let mut crc = u32::MAX;
    for &b in bytes {
        crc = (crc >> 8) ^ TABLE[((crc as u8) ^ b) as usize];
    }
    !crc
}

pub fn segment_identity(
    device: &str,
    boot: u32,
    utc_us: u64,
    source_frame: u64,
) -> Result<[u8; 16], Error> {
    if device.is_empty() || device.len() > 32 {
        return Err(Error::InvalidMetadata);
    }
    let mut hash = Sha256::new();
    hash.update(b"CRCL-segment-v1");
    hash.update(&(device.len() as u16).to_le_bytes());
    hash.update(device.as_bytes());
    hash.update(&boot.to_le_bytes());
    hash.update(&utc_us.to_le_bytes());
    hash.update(&source_frame.to_le_bytes());
    let mut id = [0; 16];
    id.copy_from_slice(&hash.finish()[..16]);
    Ok(id)
}

impl Config {
    /// V2 uses the explicit UTC/source tuple, without a digest. Device and boot
    /// identity remain separate mandatory metadata and reference comment fields.
    pub fn segment_identity(
        self,
        device: &str,
        boot: u32,
        utc_us: u64,
        source_frame: u64,
    ) -> Result<[u8; 16], Error> {
        if !self.is_cheap() {
            return segment_identity(device, boot, utc_us, source_frame);
        }
        if device.is_empty() || device.len() > 32 {
            return Err(Error::InvalidMetadata);
        }
        let mut id = [0; 16];
        id[..8].copy_from_slice(&utc_us.to_le_bytes());
        id[8..].copy_from_slice(&source_frame.to_le_bytes());
        Ok(id)
    }
}

#[derive(Clone, Copy, Debug)]
pub struct TimeAnchor {
    pub capture_origin_ticks: u64,
    pub sample_ticks: u64,
    pub sample_utc_us: u64,
    pub uncertainty_us: u64,
    pub utc_status: u8,
    pub calibration_locked: bool,
    pub calibration_ppb: i64,
    pub gps_pps: Option<(u64, u64)>,
    /// latitude E7, longitude E7, age microseconds, validity (1 retained, 2 current).
    pub location: Option<(i32, i32, u64, u8)>,
}

pub struct Header<'a> {
    pub identity: [u8; 16],
    pub sample_rate_hz: u32,
    pub reference_format: u8,
    pub first_source_frame: u64,
    pub intended_utc_us: u64,
    pub planned_frames: u64,
    pub tick_hz: u32,
    pub time: TimeAnchor,
    pub device: &'a str,
    pub boot: u32,
    pub firmware: &'a str,
    pub capture_config: &'a str,
    pub calibration_id: Option<&'a str>,
}

#[derive(Clone, Copy)]
pub enum Completion {
    Boundary = 1,
    EarlyStop = 2,
    Discontinuity = 4,
}

pub struct Encoder {
    config: Config,
    frames: u64,
    sequence: u32,
    data_records: u32,
}
impl Encoder {
    pub fn new(config: Config) -> Result<Self, Error> {
        config.validate()?;
        Ok(Self {
            config,
            frames: 0,
            sequence: 0,
            data_records: 0,
        })
    }
    pub fn reset(&mut self) {
        self.frames = 0;
        self.sequence = 0;
        self.data_records = 0;
    }
    pub fn frames(&self) -> u64 {
        self.frames
    }
    pub fn header(&self, header: &Header<'_>, output: &mut [u8]) -> Result<usize, Error> {
        if output.len() < crate::HEADER_CAPACITY {
            return Err(Error::OutputTooSmall);
        }
        if header.sample_rate_hz == 0
            || header.tick_hz == 0
            || header.planned_frames == 0
            || !matches!(header.reference_format, 1 | 2)
            || header.device.is_empty()
            || header.device.len() > 32
            || header.firmware.len() > 128
            || header.capture_config.is_empty()
            || header.capture_config.len() > 512
            || header
                .calibration_id
                .is_some_and(|s| s.is_empty() || s.len() > 128)
        {
            return Err(Error::InvalidMetadata);
        }
        if header.identity
            != self.config.segment_identity(
                header.device,
                header.boot,
                header.intended_utc_us,
                header.first_source_frame,
            )?
        {
            return Err(Error::InvalidMetadata);
        }
        output[..92].fill(0);
        output[..8].copy_from_slice(b"CRCL\r\n\x1a\n");
        put16(output, 8, self.config.version());
        put32(output, 12, u32::from(self.config.is_cheap()));
        output[16..32].copy_from_slice(&header.identity);
        put32(output, 32, header.sample_rate_hz);
        output[36..40].copy_from_slice(&[32, 1, 6, 5]);
        put16(output, 40, self.config.block_frames as u16);
        output[42] = self.config.bits;
        output[43] = header.reference_format;
        put64(output, 44, header.first_source_frame);
        put64(output, 52, header.intended_utc_us);
        put64(output, 60, header.planned_frames);
        put32(output, 72, header.tick_hz);
        let mut writer = Tlvs::new(&mut output[92..4092]);
        time_tlvs(&mut writer, header.time)?;
        writer.add(3, true, header.device.as_bytes())?;
        writer.add(4, true, &header.boot.to_le_bytes())?;
        writer.add(
            5,
            true,
            if header.firmware.is_empty() {
                b"unknown"
            } else {
                header.firmware.as_bytes()
            },
        )?;
        writer.add(6, true, &[1, 2, 3, 4, 5, 6])?;
        writer.add(7, true, header.capture_config.as_bytes())?;
        if let Some(id) = header.calibration_id {
            writer.add(10, false, id.as_bytes())?;
        }
        writer.add(
            11,
            true,
            &[match self.config.prediction {
                Prediction::DirectRice => 0,
                Prediction::TemporalRice => 1,
                Prediction::Auto => 2,
            }],
        )?;
        let metadata_bytes = writer.position;
        let len = 92 + metadata_bytes + if self.config.is_cheap() { 0 } else { 4 };
        put16(output, 10, len as u16);
        put32(output, 68, metadata_bytes as u32);
        if !self.config.is_cheap() {
            put32(output, len - 4, crc32c(&output[..len - 4]));
        }
        Ok(len)
    }
    /// The caller appends the complete returned record, then calls commit_data.
    pub fn data(&self, transformed: Transform<'_>, output: &mut [u8]) -> Result<usize, Error> {
        use raylar_stage_profiling::{Span, Stage};
        let _profile = Span::new(Stage::CrclSerialize);
        if !transformed.ready() || transformed.config != self.config {
            return Err(Error::NotReady);
        }
        let frames = transformed.pcm.len() / 6;
        if output.len() < self.config.record_capacity(frames) {
            return Err(Error::OutputTooSmall);
        }
        self.validate_next(frames)?;
        let data_header = if self.config.is_cheap() { 88 } else { 92 };
        output[..data_header].fill(0);
        let mut position = data_header;
        for (c, info) in transformed.channels.iter().enumerate() {
            let _pack_profile = Span::new(Stage::CrclPack);
            raylar_stage_profiling::representation(info.representation as u8);
            let descriptor = 28 + c * 12;
            output[descriptor] = (c + 2) as u8;
            output[descriptor + 1] = info.representation as u8;
            output[descriptor + 3] = info.k;
            put32(output, descriptor + 4, info.scale);
            let mut bits = Bits::new(&mut output[position..]);
            let mut previous = 0;
            for n in 0..frames {
                let q = transformed.pcm[n * 6 + c + 1];
                match info.representation {
                    Representation::Zero => {}
                    Representation::Packed => bits.write(q as u32, self.config.bits)?,
                    Representation::Temporal if n == 0 => bits.write(q as u32, self.config.bits)?,
                    Representation::Direct | Representation::Temporal => {
                        let r = if info.representation == Representation::Temporal {
                            q - previous
                        } else {
                            q
                        };
                        let u = fold(r);
                        bits.unary(u >> info.k)?;
                        bits.write(u & ((1 << info.k) - 1), info.k)?;
                    }
                }
                previous = q;
            }
            let bytes = bits.finish()?;
            raylar_stage_profiling::bytes(Stage::CrclPack, bytes);
            put32(output, descriptor + 8, bytes as u32);
            position += bytes;
        }
        finish_record(
            self.config.is_cheap(),
            output,
            1,
            self.sequence,
            self.frames,
            frames as u16,
            position,
        )
    }
    pub fn time(&self, time: TimeAnchor, output: &mut [u8]) -> Result<usize, Error> {
        self.validate_next(0)?;
        if output.len() < 548 {
            return Err(Error::OutputTooSmall);
        }
        let header = if self.config.is_cheap() { 28 } else { 32 };
        output[..header].fill(0);
        let mut writer = Tlvs::new(&mut output[header..544]);
        time_tlvs(&mut writer, time)?;
        let end = header + writer.position;
        finish_record(
            self.config.is_cheap(),
            output,
            2,
            self.sequence,
            self.frames,
            0,
            end,
        )
    }
    pub fn end(
        &self,
        reference_bytes: u64,
        digest: [u8; 32],
        status: Completion,
        output: &mut [u8],
    ) -> Result<usize, Error> {
        self.validate_next(0)?;
        let header = if self.config.is_cheap() { 28 } else { 32 };
        let payload_end = if self.config.is_cheap() { 52 } else { 88 };
        if output.len() < payload_end + if self.config.is_cheap() { 0 } else { 4 } {
            return Err(Error::OutputTooSmall);
        }
        output[..payload_end].fill(0);
        put64(output, header, self.frames);
        put64(output, header + 8, reference_bytes);
        put32(output, header + 16, self.data_records);
        put32(output, header + 20, status as u32);
        if !self.config.is_cheap() {
            output[56..88].copy_from_slice(&digest);
        }
        finish_record(
            self.config.is_cheap(),
            output,
            3,
            self.sequence,
            self.frames,
            0,
            payload_end,
        )
    }
    pub fn commit_data(&mut self, frames: usize) -> Result<(), Error> {
        if frames == 0 || frames > self.config.block_frames {
            return Err(Error::InvalidConfig);
        }
        self.validate_next(frames)?;
        self.frames += frames as u64;
        self.data_records += 1;
        self.sequence += 1;
        Ok(())
    }
    pub fn commit_time(&mut self) -> Result<(), Error> {
        self.validate_next(0)?;
        self.sequence += 1;
        Ok(())
    }
    fn validate_next(&self, frames: usize) -> Result<(), Error> {
        if self.sequence == u32::MAX
            || self.data_records == u32::MAX
            || self.frames.checked_add(frames as u64).is_none()
        {
            return Err(Error::CounterOverflow);
        }
        Ok(())
    }
}

fn finish_record(
    cheap: bool,
    out: &mut [u8],
    kind: u8,
    sequence: u32,
    offset: u64,
    frames: u16,
    payload_end: usize,
) -> Result<usize, Error> {
    let (header, channels) = if kind == 1 {
        (if cheap { 88 } else { 92 }, 5)
    } else {
        (if cheap { 28 } else { 32 }, 0)
    };
    let len = payload_end + if cheap { 0 } else { 4 };
    if len > out.len() {
        return Err(Error::OutputTooSmall);
    }
    out[..4].copy_from_slice(b"CRBK");
    out[4] = kind;
    out[5] = 0;
    put16(out, 6, header as u16);
    put32(out, 8, len as u32);
    put32(out, 12, sequence);
    put64(out, 16, offset);
    put16(out, 24, frames);
    out[26] = channels;
    out[27] = 0;
    if !cheap {
        put32(out, header - 4, crc32c(&out[..header - 4]));
        put32(out, payload_end, crc32c(&out[..payload_end]));
    }
    Ok(len)
}

struct Tlvs<'a> {
    output: &'a mut [u8],
    position: usize,
}
impl<'a> Tlvs<'a> {
    fn new(output: &'a mut [u8]) -> Self {
        Self {
            output,
            position: 0,
        }
    }
    fn add(&mut self, kind: u16, required: bool, bytes: &[u8]) -> Result<(), Error> {
        let end = self.position + 8 + bytes.len();
        if end > self.output.len() {
            return Err(Error::InvalidMetadata);
        }
        put16(self.output, self.position, kind);
        put16(self.output, self.position + 2, u16::from(required));
        put32(self.output, self.position + 4, bytes.len() as u32);
        self.output[self.position + 8..end].copy_from_slice(bytes);
        self.position = end;
        Ok(())
    }
}
fn time_tlvs(writer: &mut Tlvs<'_>, time: TimeAnchor) -> Result<(), Error> {
    if !matches!(time.utc_status, 1 | 2) {
        return Err(Error::InvalidMetadata);
    }
    let mut origin = [0; 16];
    put64(&mut origin, 0, time.capture_origin_ticks);
    put64(&mut origin, 8, time.sample_ticks);
    writer.add(1, true, &origin)?;
    let mut anchor = [0; 28];
    put64(&mut anchor, 0, time.sample_utc_us);
    put64(&mut anchor, 8, time.uncertainty_us);
    anchor[16] = time.utc_status;
    anchor[17] = u8::from(time.calibration_locked);
    anchor[20..28].copy_from_slice(&time.calibration_ppb.to_le_bytes());
    writer.add(2, true, &anchor)?;
    if let Some((utc, ticks)) = time.gps_pps {
        let mut pps = [0; 16];
        put64(&mut pps, 0, utc);
        put64(&mut pps, 8, ticks);
        writer.add(8, false, &pps)?;
    }
    if let Some((lat, lon, age, valid)) = time.location {
        if !matches!(valid, 0..=2) {
            return Err(Error::InvalidMetadata);
        }
        let mut location = [0; 24];
        location[..4].copy_from_slice(&lat.to_le_bytes());
        location[4..8].copy_from_slice(&lon.to_le_bytes());
        put64(&mut location, 8, age);
        location[16] = valid;
        writer.add(9, false, &location)?;
    }
    Ok(())
}
fn put16(out: &mut [u8], offset: usize, value: u16) {
    out[offset..offset + 2].copy_from_slice(&value.to_le_bytes());
}
fn put32(out: &mut [u8], offset: usize, value: u32) {
    out[offset..offset + 4].copy_from_slice(&value.to_le_bytes());
}
fn put64(out: &mut [u8], offset: usize, value: u64) {
    out[offset..offset + 8].copy_from_slice(&value.to_le_bytes());
}
