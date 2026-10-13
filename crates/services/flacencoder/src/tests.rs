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
