use super::*;
use std::vec::Vec;

fn chosen(samples: &[i32], depth: u8) -> Subframe {
    let encoder = Encoder::new(Config {
        bits_per_sample: depth,
        ..Config::default()
    })
    .unwrap();
    let mut output = [0u8; 8224];
    encoder.encode(samples, &mut output).unwrap().subframe
}

#[test]
fn selects_all_fixed_orders_and_safe_verbatim() {
    let noise: Vec<_> = (0..128).map(|i| ((i * 77 + i * i * 39) % 17) - 8).collect();
    assert!(matches!(
        chosen(&noise, 32),
        Subframe::Fixed { order: 0, .. }
    ));
    for order in 1..=4 {
        let mut levels = [0i32; 4];
        levels[order - 1] = 1_000_000;
        let mut random = 0x12345678u32;
        let samples: Vec<_> = (0..128)
            .map(|_| {
                random ^= random << 13;
                random ^= random >> 17;
                random ^= random << 5;
                let mut delta = (random % 3) as i32 - 1;
                for level in &mut levels[..order] {
                    *level += delta;
                    delta = *level;
                }
                delta
            })
            .collect();
        let choice = chosen(&samples, 32);
        assert!(
            matches!(choice, Subframe::Fixed { order: p, .. } if p == order as u8),
            "order {order}: {choice:?}"
        );
    }
    assert_eq!(
        chosen(&[i32::MIN, i32::MAX, i32::MIN, i32::MAX], 32),
        Subframe::Verbatim
    );
    assert_eq!(chosen(&[i32::MIN; 16], 32), Subframe::Constant);
    let samples: Vec<_> = (0..1024u32)
        .map(|i| (i.wrapping_mul(1664525).wrapping_add(1013904223) & 0x3fffff) as i32)
        .collect();
    assert!(matches!(
        chosen(&samples, 32),
        Subframe::Fixed { rice_bits: 5, .. }
    ));
}

#[test]
fn known_crc_vectors_and_capacity_errors() {
    assert_eq!(crc8(b"123456789"), 0xf4);
    assert_eq!(crc16(b"123456789"), 0xfee8);
    let encoder = Encoder::new(Config::default()).unwrap();
    assert_eq!(
        encoder.encode(&[1, 2], &mut [0; 1]),
        Err(Error::OutputTooSmall)
    );
    assert_eq!(
        Encoder::new(Config {
            bits_per_sample: 12,
            ..Config::default()
        })
        .err(),
        Some(Error::InvalidConfig)
    );
    let encoder = Encoder::new(Config {
        bits_per_sample: 8,
        ..Config::default()
    })
    .unwrap();
    assert_eq!(
        encoder.encode(&[128], &mut [0; 64]),
        Err(Error::SampleOutOfRange)
    );
    assert_eq!(encoder.encode(&[], &mut [0; 64]), Err(Error::InvalidFrame));
    assert_eq!(
        encoder.header(&mut [0; 1024], &["INVALID"]),
        Err(Error::InvalidConfig)
    );
    assert_eq!(
        encoder.header(&mut [0; 10], &[]),
        Err(Error::OutputTooSmall)
    );
}

#[test]
fn transactional_frames_and_partial_termination() {
    let mut encoder = Encoder::new(Config::default()).unwrap();
    let mut output = [0; 8224];
    let frame = encoder.encode(&[7; 2048], &mut output).unwrap();
    assert_eq!(encoder.samples(), 0);
    encoder.commit(frame).unwrap();
    assert_eq!(encoder.samples(), 2048);
    assert_eq!(encoder.commit(frame), Err(Error::InvalidFrame));
    let last = encoder.encode(&[8; 3], &mut output).unwrap();
    encoder.commit(last).unwrap();
    assert_eq!(encoder.samples(), 2051);
    assert_eq!(encoder.encode(&[9], &mut output), Err(Error::InvalidFrame));
    let info = encoder.streaminfo();
    let packed = u64::from_be_bytes(info[10..18].try_into().unwrap());
    assert_eq!(packed & ((1u64 << 36) - 1), 2051);
    assert_eq!(&info[18..], &[0u8; 16]);
    encoder.reset();
    encoder.frames = 128;
    let _ = encoder.encode(&[7; 2048], &mut output).unwrap();
    assert_eq!(&output[4..6], &[0xc2, 0x80]);
    encoder.frames = 0x8000_0000;
    assert_eq!(
        encoder.encode(&[7], &mut output),
        Err(Error::CounterOverflow)
    );
}

#[cfg(feature = "benchmark")]
#[test]
fn benchmark_exact_searches_match_production_and_bound_forced_choices() {
    use crate::{BenchmarkOptions, RiceMethod, Search};
    for depth in [8u8, 16, 24, 32] {
        let encoder = Encoder::new(Config {
            bits_per_sample: depth,
            ..Config::default()
        })
        .unwrap();
        let mut random = 0x12345678u32;
        let samples: Vec<_> = (0..2048)
            .map(|_| {
                random ^= random << 13;
                random ^= random >> 17;
                random ^= random << 5;
                (random as i32) >> (32 - depth)
            })
            .collect();
        let mut production = [0; 8224];
        let expected = encoder.encode(&samples, &mut production).unwrap();
        for search in [Search::BitCounts, Search::Exhaustive] {
            let analysis = encoder
                .analyze_variant(
                    &samples,
                    BenchmarkOptions {
                        order: None,
                        rice: RiceMethod::Auto,
                        search,
                    },
                )
                .unwrap();
            let mut output = [0; 8224];
            let frame = encoder.encode_analyzed(analysis, &mut output).unwrap();
            assert_eq!(frame, expected);
            assert_eq!(&output[..frame.bytes], &production[..frame.bytes]);
        }
        for order in 0..=4 {
            for rice in [RiceMethod::Auto, RiceMethod::Four, RiceMethod::Five] {
                for search in [
                    Search::BitCounts,
                    Search::Exhaustive,
                    Search::MeanNeighbors,
                    Search::MeanNeighborsFused,
                ] {
                    let analysis = encoder
                        .analyze_variant(
                            &samples,
                            BenchmarkOptions {
                                order: Some(order),
                                rice,
                                search,
                            },
                        )
                        .unwrap();
                    let mut output = [0; 8224];
                    let frame = encoder.encode_analyzed(analysis, &mut output).unwrap();
                    assert!(frame.bytes <= samples.len() * (depth as usize / 8) + 32);
                    if let Subframe::Fixed {
                        order: selected,
                        rice_bits,
                        ..
                    } = frame.subframe
                    {
                        assert_eq!(selected, order);
                        if rice == RiceMethod::Four {
                            assert_eq!(rice_bits, 4);
                        }
                        if rice == RiceMethod::Five {
                            assert_eq!(rice_bits, 5);
                        }
                    }
                }
            }
        }
    }
}

/// Materialize every benchmark variant for independent desktop decoding.
#[cfg(feature = "benchmark")]
#[test]
#[ignore = "requires prepared captured corpus and FLAC_CPU_FIXTURES output directory"]
fn write_benchmark_desktop_fixtures() {
    use crate::{BenchmarkOptions, RiceMethod, Search};
    use std::{fs, path::PathBuf};
    let root = PathBuf::from(std::env::var("FLAC_CPU_FIXTURES").unwrap());
    fs::create_dir_all(&root).unwrap();
    let bytes = fs::read(
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../../target/flac-cpu/corpus.s32le"),
    )
    .unwrap();
    let source: Vec<i32> = bytes
        .chunks_exact(4)
        .map(|b| i32::from_le_bytes(b.try_into().unwrap()))
        .collect();
    let width = fs::read_to_string(
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../../target/flac-cpu/signed_width.txt"),
    )
    .unwrap()
    .trim()
    .parse::<u8>()
    .unwrap();
    for depth in [8u8, 16, 24, 32] {
        let samples: Vec<_> = source
            .iter()
            .map(|x| x >> width.saturating_sub(depth))
            .collect();
        let raw: Vec<_> = samples
            .iter()
            .flat_map(|x| x.to_le_bytes()[..depth as usize / 8].to_vec())
            .collect();
        fs::write(root.join(std::format!("expected-{depth}.raw")), raw).unwrap();
        for block in [1024, 2048, 4096] {
            for search in [
                Search::BitCounts,
                Search::Exhaustive,
                Search::MeanNeighbors,
                Search::MeanNeighborsFused,
            ] {
                for order in [None, Some(0), Some(1), Some(2), Some(3), Some(4)] {
                    for rice in [RiceMethod::Auto, RiceMethod::Four, RiceMethod::Five] {
                        let mut encoder = Encoder::new(Config {
                            bits_per_sample: depth,
                            block_samples: block,
                            sample_rate_hz: 16000,
                        })
                        .unwrap();
                        let mut output = [0; 16416];
                        let header = encoder.header(&mut output, &[]).unwrap();
                        let mut file = output[..header].to_vec();
                        for chunk in samples.chunks(block) {
                            let analysis = encoder
                                .analyze_variant(
                                    chunk,
                                    BenchmarkOptions {
                                        order,
                                        rice,
                                        search,
                                    },
                                )
                                .unwrap();
                            let frame = encoder.encode_analyzed(analysis, &mut output).unwrap();
                            if order.is_none()
                                && rice == RiceMethod::Auto
                                && matches!(search, Search::BitCounts | Search::Exhaustive)
                            {
                                let mut production = [0; 16416];
                                let expected = encoder.encode(chunk, &mut production).unwrap();
                                assert_eq!(frame, expected);
                                assert_eq!(&output[..frame.bytes], &production[..expected.bytes]);
                            }
                            if search == Search::MeanNeighborsFused {
                                let repeated = encoder
                                    .analyze_variant(
                                        chunk,
                                        BenchmarkOptions {
                                            order,
                                            rice,
                                            search: Search::MeanNeighbors,
                                        },
                                    )
                                    .unwrap();
                                let mut previous = [0; 16416];
                                let expected =
                                    encoder.encode_analyzed(repeated, &mut previous).unwrap();
                                assert_eq!(frame, expected);
                                assert_eq!(&output[..frame.bytes], &previous[..expected.bytes]);
                            }
                            file.extend_from_slice(&output[..frame.bytes]);
                            encoder.commit(frame).unwrap();
                        }
                        file[8..42].copy_from_slice(&encoder.streaminfo());
                        fs::write(
                            root.join(std::format!(
                                "depth{depth}-block{block}-order{}-{rice:?}-{search:?}.flac",
                                order.unwrap_or(255)
                            )),
                            file,
                        )
                        .unwrap();
                    }
                }
            }
        }
    }
}
