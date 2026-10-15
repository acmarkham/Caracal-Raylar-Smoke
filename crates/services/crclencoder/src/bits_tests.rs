use super::{legacy, Bits};
use crate::Error;
use std::vec;

#[test]
fn fields_match_legacy_at_every_bit_offset_and_width() {
    for offset in 0..32u8 {
        for byte_alignment in 0..4 {
            let mut reference = vec![0xa5; 4096];
            let mut actual = reference.clone();
            let length;
            {
                let mut old = legacy::Bits::new(&mut reference[byte_alignment..4090]);
                let mut new = Bits::new(&mut actual[byte_alignment..4090]);
                old.write(u32::MAX, offset).unwrap();
                new.write(u32::MAX, offset).unwrap();
                let mut state = 0x12345678u32;
                for _ in 0..8 {
                    for width in 0..=32 {
                        state = state.wrapping_mul(1664525).wrapping_add(1013904223);
                        old.write(state, width).unwrap();
                        new.write(state, width).unwrap();
                    }
                }
                length = old.finish().unwrap();
                assert_eq!(new.finish(), Ok(length));
            }
            assert_eq!(
                actual, reference,
                "offset={offset} alignment={byte_alignment} bytes={length}"
            );
        }
    }
}

#[test]
fn rice_matches_legacy_for_all_k_and_short_long_code_boundaries() {
    for k in 0..=31u8 {
        for offset in 0..32u8 {
            let mut reference = vec![0xa5; 32768];
            let mut actual = reference.clone();
            let mut old = legacy::Bits::new(&mut reference[1..32767]);
            let mut new = Bits::new(&mut actual[1..32767]);
            old.write(u32::MAX, offset).unwrap();
            new.write(u32::MAX, offset).unwrap();
            let boundary = (31 - u32::from(k)) << k;
            for value in [
                0,
                1,
                2,
                31,
                32,
                63,
                64,
                127,
                255,
                511,
                1023,
                4095,
                65535,
                boundary,
                boundary.saturating_add(1u32 << k),
                if k >= 24 { u32::MAX } else { 13 },
            ] {
                old.unary(value >> k).unwrap();
                old.write(value & ((1u32 << k) - 1), k).unwrap();
                new.rice(value, k).unwrap();
            }
            let length = old.finish().unwrap();
            assert_eq!(new.finish(), Ok(length));
            assert_eq!(actual, reference, "k={k}, offset={offset}");
        }
    }
}

#[test]
fn exact_capacity_partial_words_and_failures_preserve_bounds() {
    for bytes in 0..17usize {
        for trailing_bits in 0..8usize {
            if bytes == 0 && trailing_bits != 0 {
                continue;
            }
            let mut storage = vec![0xa5; bytes + 2];
            let mut bits = Bits::new(&mut storage[1..1 + bytes]);
            let count = bytes * 8 - trailing_bits;
            for _ in 0..count {
                bits.write(1, 1).unwrap();
            }
            assert_eq!(bits.write(0, 33), Err(Error::OutputTooSmall));
            assert_eq!(bits.rice(0, 32), Err(Error::InvalidConfig));
            assert_eq!(bits.rice(u32::MAX, 0), Err(Error::OutputTooSmall));
            assert_eq!(
                bits.write(0, (trailing_bits + 1) as u8),
                Err(Error::OutputTooSmall)
            );
            if trailing_bits != 0 {
                bits.write(0, trailing_bits as u8).unwrap();
            }
            assert_eq!(bits.finish(), Ok(bytes));
            assert_eq!(storage[0], 0xa5);
            assert_eq!(storage[bytes + 1], 0xa5);
            for n in 0..count {
                assert_ne!(storage[1 + n / 8] & (1 << (7 - n % 8)), 0);
            }
            for n in count..bytes * 8 {
                assert_eq!(storage[1 + n / 8] & (1 << (7 - n % 8)), 0);
            }
        }
    }
    let mut storage = [0xa5; 3];
    let mut bits = Bits::new(&mut storage[1..2]);
    bits.write(7, 3).unwrap();
    assert_eq!(bits.rice(15, 0), Err(Error::OutputTooSmall));
    bits.write(0, 5).unwrap();
    assert_eq!(bits.finish(), Ok(1));
    assert_eq!(storage, [0xa5, 0xe0, 0xa5]);
}
