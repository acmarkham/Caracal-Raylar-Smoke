use super::*;
use std::vec;

fn hex(bytes: &[u8]) -> std::string::String {
    bytes.iter().map(|b| std::format!("{b:02x}")).collect()
}

#[test]
fn sha256_and_crc_known_vectors() {
    assert_eq!(crc32c(b"123456789"), 0xe3069283);
    let mut hash = Sha256::new();
    hash.update(b"a");
    hash.update(b"bc");
    assert_eq!(
        hex(&hash.finish()),
        "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad"
    );
    assert_eq!(
        hex(&Sha256::new().finish()),
        "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"
    );
    let mut hash = Sha256::new();
    for _ in 0..1000 {
        hash.update(&[b'a'; 1000]);
    }
    assert_eq!(
        hex(&hash.finish()),
        "cdc76e5c9914fb9281a1c7e284d73e67f1809a48a497200e046d39ccc7112cd0"
    );
}

#[test]
fn quantization_matches_wide_host_reference_at_all_precisions() {
    let mut rng = 123u32;
    for bits in [6, 8, 10, 12] {
        for peak in [1, 2, 31, 255, 65535, 16777216, i32::MAX] {
            for _ in 0..2000 {
                rng = rng.wrapping_mul(1664525).wrapping_add(1013904223);
                let a = (rng % peak as u32) as i32;
                for d in [0, a, -a, peak, -peak] {
                    let exact = (2 * i64::from(d.abs()) * ((1i64 << (bits - 1)) - 1)
                        + i64::from(peak))
                        / (2 * i64::from(peak));
                    assert_eq!(
                        quantize_ratio(d, bits, peak).unwrap(),
                        exact as i32 * d.signum()
                    );
                }
            }
        }
    }
    assert!(quantize_ratio(i32::MIN, 12, i32::MAX).is_err());
}

struct Reader<'a> {
    bytes: &'a [u8],
    bit: usize,
}
impl Reader<'_> {
    fn read(&mut self, n: u8) -> u32 {
        let mut v = 0;
        for _ in 0..n {
            v = v * 2 + u32::from((self.bytes[self.bit / 8] >> (7 - self.bit % 8)) & 1);
            self.bit += 1;
        }
        v
    }
    fn signed(&mut self, b: u8) -> i32 {
        ((self.read(b) << (32 - b)) as i32) >> (32 - b)
    }
    fn rice(&mut self, k: u8) -> i32 {
        let mut q = 0;
        while self.read(1) == 0 {
            q += 1;
        }
        let u = (q << k) | self.read(k);
        if u & 1 == 0 {
            (u / 2) as i32
        } else {
            -((u / 2) as i32) - 1
        }
    }
}
fn u32_at(bytes: &[u8], i: usize) -> u32 {
    u32::from_le_bytes(bytes[i..i + 4].try_into().unwrap())
}

#[test]
fn all_modes_roundtrip_quantized_values_and_preserve_reference_in_place() {
    let mut representations = [false; 4];
    for bits in [6, 8, 10, 12] {
        for frames in [1, 255, 256, 257, 1024, 4096] {
            for prediction in [
                Prediction::DirectRice,
                Prediction::TemporalRice,
                Prediction::Auto,
            ] {
                for pattern in 0..4 {
                    let config = Config {
                        block_frames: 4096,
                        bits,
                        prediction,
                        ..Config::default()
                    };
                    let mut original = vec![0; frames * 6];
                    for n in 0..frames {
                        for c in 0..6 {
                            original[n * 6 + c] = n as i32 * 13
                                + match pattern {
                                    0 => 0,
                                    1 => c as i32 * 1000,
                                    2 => n as i32 * c as i32 * 7,
                                    _ => ((n as i32 * 7919 + c as i32 * 104729) % 100000) - 50000,
                                };
                        }
                    }
                    let mut pcm = original.clone();
                    let mut out = vec![0; workspace_capacity(4096, bits)];
                    let mut transform = Transform::new(&mut pcm, config).unwrap();
                    while !transform.advance(113).unwrap() {}
                    let expected = transform.pcm.to_vec();
                    let encoder = Encoder::new(config).unwrap();
                    let len = encoder.data(transform, &mut out).unwrap();
                    assert!(len <= record_capacity(frames, bits));
                    assert_eq!(u32_at(&out, 88), crc32c(&out[..88]));
                    assert_eq!(u32_at(&out, len - 4), crc32c(&out[..len - 4]));
                    for n in 0..frames {
                        assert_eq!(pcm[n * 6], original[n * 6]);
                    }
                    let mut offset = 92;
                    for c in 0..5 {
                        let descriptor = 28 + c * 12;
                        let mode = out[descriptor + 1];
                        representations[mode as usize] = true;
                        let bytes = u32_at(&out, descriptor + 8) as usize;
                        let mut reader = Reader {
                            bytes: &out[offset..offset + bytes],
                            bit: 0,
                        };
                        let mut previous = 0;
                        for n in 0..frames {
                            let q = match mode {
                                0 => 0,
                                3 => reader.signed(bits),
                                2 if n == 0 => reader.signed(bits),
                                2 => previous + reader.rice(out[descriptor + 3]),
                                1 => reader.rice(out[descriptor + 3]),
                                _ => panic!(),
                            };
                            assert_eq!(q, expected[n * 6 + c + 1]);
                            previous = q;
                            let scale = i64::from(u32_at(&out, descriptor + 4));
                            let limit = (1i64 << (bits - 1)) - 1;
                            let reconstructed = (i64::from(q.abs()) * scale * 2 + limit)
                                / (2 * limit)
                                * i64::from(q.signum());
                            let delta =
                                i64::from(original[n * 6 + c + 1]) - i64::from(original[n * 6]);
                            assert!((reconstructed - delta).abs() * 2 * limit <= scale + limit);
                        }
                        while reader.bit < bytes * 8 {
                            assert_eq!(reader.read(1), 0);
                        }
                        offset += bytes;
                    }
                    assert_eq!(offset, len - 4);
                }
            }
        }
    }
    assert_eq!(representations, [true; 4]);
}

#[test]
fn capacity_and_input_bounds_are_explicit() {
    let config = Config::default();
    let mut pcm = [0; 6];
    pcm[1] = i32::MAX;
    let mut t = Transform::new(&mut pcm, config).unwrap();
    assert_eq!(t.advance(256), Err(Error::SampleOutOfRange));
    let mut pcm = [0; 6];
    let mut t = Transform::new(&mut pcm, config).unwrap();
    assert!(t.advance(256).unwrap());
    assert_eq!(
        Encoder::new(config).unwrap().data(t, &mut [0; 8]),
        Err(Error::OutputTooSmall)
    );
}

#[test]
fn cheap_temporal_roundtrip_scales_channels_independently_and_ignores_reference() {
    for bits in [6, 8, 10, 12] {
        for frames in [1, 255, 256, 257, 1024, 4096] {
            for pattern in 0..4 {
                let config = Config {
                    block_frames: 4096,
                    bits,
                    ..Config::cheap()
                };
                let mut original = vec![0; frames * 6];
                let mut rng = 7u32;
                for n in 0..frames {
                    original[n * 6] = if n % 2 == 0 { i32::MAX } else { i32::MIN };
                    for c in 1..6 {
                        rng = rng.wrapping_mul(1664525).wrapping_add(1013904223);
                        let amplitude = [0, 7, 1000, 1 << 23, 31, 50000][c];
                        original[n * 6 + c] = match pattern {
                            0 => 0,
                            1 => {
                                if n % 2 == 0 {
                                    amplitude
                                } else {
                                    -amplitude
                                }
                            }
                            2 => (rng % (2 * amplitude as u32 + 1)) as i32 - amplitude,
                            _ => (n as i32 % 100 - 50) * (amplitude / 50),
                        };
                    }
                }
                let mut pcm = original.clone();
                let mut out = vec![0; config.workspace_capacity()];
                let mut t = Transform::new(&mut pcm, config).unwrap();
                while !t.advance(113).unwrap() {}
                let descriptors = t.channels;
                let encoder = Encoder::new(config).unwrap();
                let len = encoder.data(t, &mut out).unwrap();
                assert!(len <= config.record_capacity(frames));
                assert_eq!(u16::from_le_bytes(out[6..8].try_into().unwrap()), 88);
                let mut payload = 88;
                for c in 0..5 {
                    let d = 28 + c * 12;
                    assert_eq!(out[d + 1], 2);
                    let shift = u32_at(&out, d + 4);
                    let peak = (0..frames)
                        .map(|n| original[n * 6 + c + 1].unsigned_abs())
                        .max()
                        .unwrap();
                    assert_eq!(shift, u32::from(shift_for_peak(peak, bits)));
                    let bytes = u32_at(&out, d + 8) as usize;
                    let mut reader = Reader {
                        bytes: &out[payload..payload + bytes],
                        bit: 0,
                    };
                    let mut previous = 0;
                    for n in 0..frames {
                        let q = if n == 0 {
                            reader.signed(bits)
                        } else {
                            previous + reader.rice(out[d + 3])
                        };
                        previous = q;
                        assert_eq!(q, pcm[n * 6 + c + 1]);
                        let reconstructed = i64::from(q) << shift;
                        let error = (reconstructed - i64::from(original[n * 6 + c + 1])).abs();
                        assert!(error <= if shift == 0 { 0 } else { 1i64 << (shift - 1) });
                    }
                    while reader.bit < bytes * 8 {
                        assert_eq!(reader.read(1), 0);
                    }
                    payload += bytes;
                    assert_eq!(descriptors[c].k, out[d + 3]);
                }
                assert_eq!(payload, len);
                for n in 0..frames {
                    assert_eq!(pcm[n * 6], original[n * 6]);
                    original[n * 6] = 123;
                }
                let mut t = Transform::new(&mut original, config).unwrap();
                while !t.advance(257).unwrap() {}
                let mut other = vec![0; config.workspace_capacity()];
                let other_len = encoder.data(t, &mut other).unwrap();
                assert_eq!(&out[..len], &other[..other_len]);
            }
        }
    }
}

#[test]
fn cheap_rice_estimate_and_rounding_bounds_cover_boundary_values() {
    assert_eq!(estimate_rice_k(0, 0), 0);
    for count in [1usize, 255, 1023, 4095] {
        for sum in [0, 1, count as u32, count as u32 * 2, count as u32 * 2044] {
            let k = estimate_rice_k(sum, count);
            if sum != 0 {
                let mean = sum.div_ceil(count as u32);
                assert!((1u32 << k) <= mean && mean < (1u32 << (k + 1)));
            }
        }
    }
    for bits in [6, 8, 10, 12] {
        let limit = (1u32 << (bits - 1)) - 1;
        for peak in [0, 1, limit, limit + 1, 65535, 1 << 23, i32::MAX as u32 / 2] {
            let s = shift_for_peak(peak, bits);
            let half = if s == 0 { 0 } else { 1u32 << (s - 1) };
            assert!((peak + half) >> s <= limit);
        }
    }
}

#[test]
fn header_time_and_end_lengths_and_checksums() {
    let mut encoder = Encoder::new(Config::default()).unwrap();
    let mut out = vec![0; 4096];
    let time = TimeAnchor {
        capture_origin_ticks: 100,
        sample_ticks: 123,
        sample_utc_us: 1700000000000000,
        uncertainty_us: 50,
        utc_status: 1,
        calibration_locked: true,
        calibration_ppb: -9000,
        gps_pps: Some((1700000000000000, 123)),
        location: None,
    };
    let id = segment_identity("board", 42, time.sample_utc_us, 5).unwrap();
    let header = Header {
        identity: id,
        sample_rate_hz: 16000,
        reference_format: 2,
        first_source_frame: 5,
        intended_utc_us: time.sample_utc_us,
        planned_frames: 960000,
        tick_hz: 1000000,
        time,
        device: "board",
        boot: 42,
        firmware: "test",
        capture_config: "test",
        calibration_id: None,
    };
    let len = encoder.header(&header, &mut out).unwrap();
    assert_eq!(&out[..8], b"CRCL\r\n\x1a\n");
    assert_eq!(u32_at(&out, len - 4), crc32c(&out[..len - 4]));
    let len = encoder.time(time, &mut out).unwrap();
    assert!(len <= 548);
    encoder.commit_time().unwrap();
    let len = encoder
        .end(1024, [1; 32], Completion::EarlyStop, &mut out)
        .unwrap();
    assert_eq!(len, 92);
    assert_eq!(u32_at(&out, 12), 1);
    assert_eq!(u32_at(&out, 88), crc32c(&out[..88]));
}
