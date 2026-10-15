//! Opt-in diagnostic kernels. No normal-build code or production codec changes.
//! Whole-block kernels use the same opt-level=3 as the encoder. Extra buffers
//! here isolate actions; the production transform still has no scratch arrays.
use crate::{
    bits::{fold, legacy::Bits},
    estimate_rice_k, shift_for_peak, Channel, Config, Encoder, Transform,
};
use core::hint::black_box;
use core::sync::atomic::{compiler_fence, Ordering};

const N: usize = 1024;
const AUX: usize = 5;
const VALUES: usize = N * 6;
const RESIDUALS: usize = (N - 1) * AUX;
const CAP: usize = crate::cheap_record_capacity(N, 10);
const REPEATS: usize = 129;
const CONFIG: Config = Config {
    min_shift: 6,
    ..Config::cheap()
};
const SINE: [i32; 32] = [
    0, 6393, 12539, 18204, 23170, 27245, 30273, 32137, 32767, 32137, 30273, 27245, 23170, 18204,
    12539, 6393, 0, -6393, -12539, -18204, -23170, -27245, -30273, -32137, -32767, -32137, -30273,
    -27245, -23170, -18204, -12539, -6393,
];

pub struct Memory {
    source: [i32; VALUES],
    pcm: [i32; VALUES],
    quant: [i32; VALUES],
    delta: [i32; RESIDUALS],
    folded: [u32; RESIDUALS],
    output: [u8; CAP],
    expected: [u8; CAP],
    channels: [Channel; AUX],
    sums: [u32; AUX],
    lengths: [u32; AUX],
    record_bytes: usize,
}
impl Memory {
    pub const fn new() -> Self {
        Self {
            source: [0; VALUES],
            pcm: [0; VALUES],
            quant: [0; VALUES],
            delta: [0; RESIDUALS],
            folded: [0; RESIDUALS],
            output: [0; CAP],
            expected: [0; CAP],
            channels: [Channel::ZERO; AUX],
            sums: [0; AUX],
            lengths: [0; AUX],
            record_bytes: 0,
        }
    }
}

pub struct Measurement {
    pub name: &'static str,
    /// Scalar stages batch 256 five-channel parameter calculations per timer span.
    pub batch: u32,
    pub min: u32,
    pub median: u32,
    pub p90: u32,
    pub max: u32,
    pub checksum: u32,
}
pub struct Fixture {
    pub input_hash: u32,
    pub record_bytes: usize,
    pub shifts: [u32; AUX],
    pub k: [u8; AUX],
    pub sums: [u32; AUX],
    pub payload_bytes: [u32; AUX],
}

fn prepare(m: &mut Memory) {
    let mut state = 0x12345678u32;
    for n in 0..N {
        for c in 0..6 {
            state ^= state << 13;
            state ^= state >> 17;
            state ^= state << 5;
            m.source[n * 6 + c] = SINE[(n + c * 2) % 32] * (20000 + c as i32 * 1000) / 32768
                + (state & 255) as i32
                - 128;
        }
    }
    raw(m);
    let mut t = Transform::new(&mut m.pcm, CONFIG).unwrap();
    while !t.advance(usize::MAX).unwrap() {}
    m.channels = t.channels;
    m.record_bytes = Encoder::new(CONFIG)
        .unwrap()
        .data(t, &mut m.expected)
        .unwrap();
    m.quant.copy_from_slice(&m.pcm);
    for c in 0..AUX {
        let mut sum = 0;
        for n in 1..N {
            let r = m.quant[n * 6 + c + 1] - m.quant[(n - 1) * 6 + c + 1];
            let i = c * (N - 1) + n - 1;
            m.delta[i] = r;
            m.folded[i] = fold(r);
            sum += m.folded[i];
        }
        m.sums[c] = sum;
        assert_eq!(estimate_rice_k(sum, N - 1), m.channels[c].k);
        m.lengths[c] = u32::from_le_bytes(m.expected[36 + c * 12..40 + c * 12].try_into().unwrap());
    }
    for n in 0..N {
        assert_eq!(m.source[n * 6], m.quant[n * 6]);
    }
}

fn raw(m: &mut Memory) {
    m.pcm.copy_from_slice(&m.source);
}
fn quant(m: &mut Memory) {
    m.pcm.copy_from_slice(&m.quant);
}
fn nothing(_: &mut Memory) {}

fn measure(
    m: &mut Memory,
    clock: fn() -> u32,
    name: &'static str,
    batch: u32,
    setup: fn(&mut Memory),
    op: fn(&mut Memory) -> u32,
) -> Measurement {
    let mut values = [0u32; REPEATS];
    let mut checksum = 0;
    for i in 0..REPEATS + 8 {
        setup(m);
        compiler_fence(Ordering::SeqCst);
        let start = clock();
        let result = black_box(op(black_box(m)));
        compiler_fence(Ordering::SeqCst);
        let elapsed = clock().wrapping_sub(start);
        if i >= 8 {
            values[i - 8] = elapsed;
            checksum ^= result;
        }
    }
    values.sort_unstable();
    Measurement {
        name,
        batch,
        min: values[0],
        median: values[REPEATS / 2],
        p90: values[(REPEATS - 1) * 9 / 10],
        max: values[REPEATS - 1],
        checksum,
    }
}

#[inline(never)]
fn empty(_: &mut Memory) -> u32 {
    black_box(0)
}
#[inline(never)]
fn copy(m: &mut Memory) -> u32 {
    raw(m);
    black_box(&m.pcm);
    m.pcm[0] as u32
}
#[inline(never)]
fn dcblock(m: &mut Memory) -> u32 {
    // Optional first-order HPF, a=255/256 (~9.97 Hz at 16 kHz).
    // State reset is fixture setup; production would persist two i32 per channel.
    let mut check = 0;
    for c in 1..6 {
        let (mut previous, mut y) = (0, 0);
        for n in 0..N {
            let i = n * 6 + c;
            let x = m.pcm[i];
            y = x - previous + y - (y >> 8);
            previous = x;
            m.pcm[i] = y;
        }
        check ^= y as u32;
    }
    black_box(&m.pcm);
    check
}
#[inline(never)]
fn peak(m: &mut Memory) -> u32 {
    let mut check = 0;
    for c in 1..6 {
        let mut peak = 0i32;
        for n in 0..N {
            let x = m.pcm[n * 6 + c];
            if x < -CONFIG.max_abs_pcm || x > CONFIG.max_abs_pcm {
                panic!("range");
            }
            peak = peak.max(x.abs());
        }
        check ^= black_box(peak as u32);
    }
    check
}
#[inline(never)]
fn scale(_m: &mut Memory) -> u32 {
    let mut check = 0;
    for _ in 0..256 {
        for c in 1..6 {
            // Equivalent known fixture peaks; black_box prevents constant folding.
            let peak = black_box(20000 + c as u32 * 1000 + 128);
            check ^= black_box(u32::from(
                shift_for_peak(peak, black_box(10)).max(black_box(6)),
            ));
        }
    }
    check
}
#[inline(never)]
fn shift_only(m: &mut Memory) -> u32 {
    for c in 1..6 {
        let shift = black_box(m.channels[c - 1].scale);
        for n in 0..N {
            m.pcm[n * 6 + c] >>= shift;
        }
    }
    black_box(&m.pcm);
    m.pcm[1] as u32
}
#[inline(never)]
fn rounding(m: &mut Memory) -> u32 {
    for c in 1..6 {
        let shift = black_box(m.channels[c - 1].scale);
        let half = 1 << (shift - 1);
        for n in 0..N {
            m.pcm[n * 6 + c] = (m.pcm[n * 6 + c] + half) >> shift;
        }
    }
    black_box(&m.pcm);
    m.pcm[1] as u32
}
#[inline(never)]
fn temporal(m: &mut Memory) -> u32 {
    for c in 0..AUX {
        let mut previous = m.quant[c + 1];
        for n in 1..N {
            let q = m.quant[n * 6 + c + 1];
            m.delta[c * (N - 1) + n - 1] = q - previous;
            previous = q;
        }
    }
    black_box(&m.delta);
    m.delta[0] as u32
}
#[inline(never)]
fn fold_sum(m: &mut Memory) -> u32 {
    let mut check = 0;
    for c in 0..AUX {
        let mut sum = 0;
        for n in 0..N - 1 {
            sum += fold(m.delta[c * (N - 1) + n]);
        }
        check ^= black_box(sum);
    }
    check
}
#[inline(never)]
fn sum_only(m: &mut Memory) -> u32 {
    let mut check = 0;
    for c in 0..AUX {
        let mut sum = 0;
        for n in 0..N - 1 {
            sum += m.folded[c * (N - 1) + n];
        }
        check ^= black_box(sum);
    }
    check
}
#[inline(never)]
fn rice_k(m: &mut Memory) -> u32 {
    let mut check = 0;
    for _ in 0..256 {
        for c in 0..AUX {
            check ^= black_box(u32::from(estimate_rice_k(
                black_box(m.sums[c]),
                black_box(N - 1),
            )));
        }
    }
    check
}
#[inline(never)]
fn fused(m: &mut Memory) -> u32 {
    let mut check = 0;
    for c in 1..6 {
        let shift = black_box(m.channels[c - 1].scale);
        let half = 1 << (shift - 1);
        let mut previous = 0;
        let mut sum = 0;
        for n in 0..N {
            let i = n * 6 + c;
            let q = (m.pcm[i] + half) >> shift;
            m.pcm[i] = q;
            if n != 0 {
                sum += fold(q - previous);
            }
            previous = q;
        }
        check ^= black_box(sum);
    }
    black_box(&m.pcm);
    check
}

#[inline(never)]
fn encode_folded(m: &mut Memory) -> u32 {
    encode_bits(m, false, false)
}
#[inline(never)]
fn encode_codeword(m: &mut Memory) -> u32 {
    encode_bits(m, true, false)
}
#[inline(never)]
fn encode_payload(m: &mut Memory) -> u32 {
    encode_bits(m, false, true)
}

// Frozen pre-reservoir writer: historical baseline remains directly comparable.
fn encode_bits(m: &mut Memory, single: bool, from_quant: bool) -> u32 {
    let mut pos = 88;
    for c in 0..AUX {
        let k = m.channels[c].k;
        let mut bits = Bits::new(&mut m.output[pos..]);
        bits.write(m.quant[c + 1] as u32, 10).unwrap();
        let mut previous = m.quant[c + 1];
        for n in 1..N {
            let u = if from_quant {
                let q = m.quant[n * 6 + c + 1];
                let u = fold(q - previous);
                previous = q;
                u
            } else {
                m.folded[c * (N - 1) + n - 1]
            };
            let len = (u >> k) + 1 + u32::from(k);
            if single && len <= 32 {
                bits.write((1 << k) | (u & ((1 << k) - 1)), len as u8)
                    .unwrap();
            } else {
                bits.unary(u >> k).unwrap();
                bits.write(u & ((1 << k) - 1), k).unwrap();
            }
        }
        pos += bits.finish().unwrap();
    }
    black_box(&m.output[..pos]);
    pos as u32
}

// Candidate 32-bit MSB reservoir, fixture-only prevalidated output capacity.
// It retains slice bounds checks but omits the production writer's per-field
// checked_add/capacity contract. Production adoption must retain a proven bound.
struct Words<'a> {
    bytes: &'a mut [u8],
    word: u32,
    used: u32,
    pos: usize,
}
impl Words<'_> {
    fn write(&mut self, value: u32, mut width: u32) {
        while width != 0 {
            let take = width.min(32 - self.used);
            let part = if take == 32 {
                value
            } else {
                (value >> (width - take)) & ((1 << take) - 1)
            };
            self.word |= part << (32 - self.used - take);
            self.used += take;
            width -= take;
            if self.used == 32 {
                self.bytes[self.pos..self.pos + 4].copy_from_slice(&self.word.to_be_bytes());
                self.pos += 4;
                self.word = 0;
                self.used = 0;
            }
        }
    }
    fn finish(self) -> usize {
        let tail = self.used.div_ceil(8) as usize;
        self.bytes[self.pos..self.pos + tail].copy_from_slice(&self.word.to_be_bytes()[..tail]);
        self.pos + tail
    }
}
#[inline(never)]
fn encode_words(m: &mut Memory) -> u32 {
    let mut pos = 88;
    for c in 0..AUX {
        let k = u32::from(m.channels[c].k);
        let mut bits = Words {
            bytes: &mut m.output[pos..],
            word: 0,
            used: 0,
            pos: 0,
        };
        bits.write(m.quant[c + 1] as u32, 10);
        for n in 0..N - 1 {
            let u = m.folded[c * (N - 1) + n];
            let len = (u >> k) + 1 + k;
            if len <= 32 {
                bits.write((1 << k) | (u & ((1 << k) - 1)), len);
            } else {
                let mut zeros = u >> k;
                while zeros != 0 {
                    let take = zeros.min(32);
                    bits.write(0, take);
                    zeros -= take;
                }
                bits.write(1, 1);
                bits.write(u & ((1 << k) - 1), k);
            }
        }
        pos += bits.finish();
    }
    black_box(&m.output[..pos]);
    pos as u32
}

#[inline(never)]
fn pack_header(m: &mut Memory) -> u32 {
    // Equivalent v2 DATA header/descriptor stores, over an existing payload.
    // No Rice work or payload copy; compare production serialization separately.
    let out = &mut m.output;
    out[..88].fill(0);
    out[..4].copy_from_slice(b"CRBK");
    out[4] = 1;
    out[6..8].copy_from_slice(&88u16.to_le_bytes());
    out[8..12].copy_from_slice(&(m.record_bytes as u32).to_le_bytes());
    out[24..26].copy_from_slice(&(N as u16).to_le_bytes());
    out[26] = 5;
    for c in 0..AUX {
        let d = 28 + c * 12;
        out[d] = (c + 2) as u8;
        out[d + 1] = 2;
        out[d + 3] = m.channels[c].k;
        out[d + 4..d + 8].copy_from_slice(&m.channels[c].scale.to_le_bytes());
        out[d + 8..d + 12].copy_from_slice(&m.lengths[c].to_le_bytes());
    }
    black_box(&m.output[..88]);
    88
}
#[inline(never)]
fn pack_copy(m: &mut Memory) -> u32 {
    m.output[88..m.record_bytes].copy_from_slice(&m.expected[88..m.record_bytes]);
    pack_header(m);
    black_box(&m.output[..m.record_bytes]);
    m.record_bytes as u32
}
#[inline(never)]
fn transform_production(m: &mut Memory) -> u32 {
    let mut t = Transform::new(&mut m.pcm, black_box(CONFIG)).unwrap();
    while !t.advance(black_box(usize::MAX)).unwrap() {}
    black_box(&t.pcm);
    t.channels[0].scale
}
#[inline(never)]
fn transform_budgeted(m: &mut Memory) -> u32 {
    let mut t = Transform::new(&mut m.pcm, black_box(CONFIG)).unwrap();
    while !t.advance(black_box(256)).unwrap() {}
    black_box(&t.pcm);
    t.channels[0].scale
}
#[inline(never)]
fn serialize_production(m: &mut Memory) -> u32 {
    // Ready transform reconstructed outside the timed arithmetic; equivalent
    // metadata is already proven against a real Transform by prepare().
    let mut t = Transform::new(&mut m.pcm, black_box(CONFIG)).unwrap();
    t.channel = 6;
    t.channels = m.channels;
    let bytes = Encoder::new(black_box(CONFIG))
        .unwrap()
        .data(t, &mut m.output)
        .unwrap();
    black_box(&m.output[..bytes]);
    bytes as u32
}
#[inline(never)]
fn codec(m: &mut Memory) -> u32 {
    let encoder = Encoder::new(black_box(CONFIG)).unwrap();
    let mut t = Transform::new(&mut m.pcm, black_box(CONFIG)).unwrap();
    while !t.advance(black_box(256)).unwrap() {}
    let bytes = encoder.data(t, &mut m.output).unwrap();
    black_box(&m.output[..bytes]);
    bytes as u32
}

fn split_transform(m: &mut Memory, clock: fn() -> u32) -> [Measurement; 2] {
    let mut peaks = [0u32; REPEATS];
    let mut quants = [0u32; REPEATS];
    for repeat in 0..REPEATS + 8 {
        raw(m);
        let mut t = Transform::new(&mut m.pcm, CONFIG).unwrap();
        let (mut p, mut q) = (0u32, 0u32);
        for _ in 0..AUX {
            compiler_fence(Ordering::SeqCst);
            let begin = clock();
            black_box(t.advance(black_box(N)).unwrap());
            compiler_fence(Ordering::SeqCst);
            let middle = clock();
            black_box(t.advance(black_box(N)).unwrap());
            compiler_fence(Ordering::SeqCst);
            let end = clock();
            p += middle.wrapping_sub(begin);
            q += end.wrapping_sub(middle);
        }
        assert_eq!(t.pcm, &m.quant);
        if repeat >= 8 {
            peaks[repeat - 8] = p;
            quants[repeat - 8] = q;
        }
    }
    peaks.sort_unstable();
    quants.sort_unstable();
    core::array::from_fn(|i| {
        let a = if i == 0 { &peaks } else { &quants };
        Measurement {
            name: if i == 0 {
                "production_peak_scale_phases"
            } else {
                "production_quant_stats_k_phases"
            },
            batch: 1,
            min: a[0],
            median: a[REPEATS / 2],
            p90: a[(REPEATS - 1) * 9 / 10],
            max: a[REPEATS - 1],
            checksum: 0,
        }
    })
}

pub fn run(
    m: &mut Memory,
    clock: fn() -> u32,
    mut report: impl FnMut(Measurement),
    mut fixture: impl FnMut(Fixture),
) {
    prepare(m);
    let hash = m
        .source
        .iter()
        .flat_map(|x| x.to_le_bytes())
        .fold(2166136261u32, |h, b| {
            (h ^ u32::from(b)).wrapping_mul(16777619)
        });
    fixture(Fixture {
        input_hash: hash,
        record_bytes: m.record_bytes,
        shifts: m.channels.map(|c| c.scale),
        k: m.channels.map(|c| c.k),
        sums: m.sums,
        payload_bytes: m.lengths,
    });
    let rows: [(&str, u32, fn(&mut Memory), fn(&mut Memory) -> u32); 20] = [
        ("empty", 1, nothing, empty),
        ("copy_6ch", 1, nothing, copy),
        ("optional_hpf_5ch", 1, raw, dcblock),
        ("peak_validate_5ch", 1, raw, peak),
        ("scale_select_5ch", 256, nothing, scale),
        ("truncate_shift_5ch", 1, raw, shift_only),
        ("round_quantize_5ch", 1, raw, rounding),
        ("temporal_delta_5ch", 1, nothing, temporal),
        ("sum_pre_folded_5ch", 1, nothing, sum_only),
        ("fold_sum_residual_5ch", 1, nothing, fold_sum),
        ("rice_k_5ch", 256, nothing, rice_k),
        ("quantize_delta_sum_fused_5ch", 1, raw, fused),
        ("rice_bytes_pre_folded_5ch", 1, nothing, encode_folded),
        ("rice_single_codeword_5ch", 1, nothing, encode_codeword),
        ("rice_words_codeword_5ch", 1, nothing, encode_words),
        ("payload_from_quant_5ch", 1, nothing, encode_payload),
        ("crcl_header_only", 1, nothing, pack_header),
        ("crcl_header_payload_copy", 1, nothing, pack_copy),
        ("production_transform", 1, raw, transform_production),
        ("production_transform_budget256", 1, raw, transform_budgeted),
    ];
    for (name, batch, setup, op) in rows {
        let row = measure(m, clock, name, batch, setup, op);
        // Verify actual candidate bytes and transforms outside the measured span.
        if name == "round_quantize_5ch"
            || name == "quantize_delta_sum_fused_5ch"
            || name.starts_with("production_transform")
        {
            assert_eq!(m.pcm, m.quant);
        }
        if name.starts_with("rice_") && name.ends_with("folded_5ch")
            || name == "rice_single_codeword_5ch"
            || name == "rice_words_codeword_5ch"
            || name == "payload_from_quant_5ch"
        {
            assert_eq!(
                &m.output[88..m.record_bytes],
                &m.expected[88..m.record_bytes]
            );
        }
        if name == "crcl_header_payload_copy" {
            assert_eq!(&m.output[..m.record_bytes], &m.expected[..m.record_bytes]);
        }
        report(row);
    }
    for (name, setup, op) in [
        (
            "production_serialize",
            quant as fn(&mut Memory),
            serialize_production as fn(&mut Memory) -> u32,
        ),
        ("production_codec_budget256", raw, codec),
    ] {
        let row = measure(m, clock, name, 1, setup, op);
        assert_eq!(&m.output[..m.record_bytes], &m.expected[..m.record_bytes]);
        report(row);
    }
    for row in split_transform(m, clock) {
        report(row);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn word_reservoir_matches_checked_writer_across_word_boundaries() {
        let mut reference = [0xa5; 2048];
        let mut candidate = [0xa5; 2048];
        let mut bytes = Bits::new(&mut reference);
        let mut words = Words {
            bytes: &mut candidate,
            word: 0,
            used: 0,
            pos: 0,
        };
        for pass in 0..4u32 {
            for width in 0..=32u8 {
                let value = 0x9e3779b9u32.wrapping_mul(pass * 33 + u32::from(width) + 1);
                bytes.write(value, width).unwrap();
                words.write(value, u32::from(width));
            }
        }
        let expected = bytes.finish().unwrap();
        let actual = words.finish();
        assert_eq!(actual, expected);
        assert_eq!(&candidate[..actual], &reference[..expected]);
    }

    #[test]
    fn fixture_and_candidate_payloads_match_production() {
        let mut m = Memory::new();
        prepare(&mut m);
        assert_eq!(m.record_bytes, 5183);
        assert_eq!(m.channels.map(|c| c.scale), [6; 5]);
        assert_eq!(m.channels.map(|c| c.k), [6; 5]);
        for emit in [encode_folded, encode_codeword, encode_words, encode_payload] {
            m.output.fill(0xa5);
            assert_eq!(emit(&mut m), 5183);
            assert_eq!(
                &m.output[88..m.record_bytes],
                &m.expected[88..m.record_bytes]
            );
        }
        pack_header(&mut m);
        assert_eq!(&m.output[..m.record_bytes], &m.expected[..m.record_bytes]);
    }
}
