use crate::{Config, Encoder, Error, Transform};
use std::{vec, vec::Vec};

fn same_state(a: &Transform<'_>, b: &Transform<'_>) {
    assert_eq!(a.pcm, b.pcm);
    assert_eq!(a.config, b.config);
    assert_eq!(
        (
            a.channel,
            a.offset,
            a.quantizing,
            a.peak,
            a.previous,
            a.shift,
            a.sum_folded
        ),
        (
            b.channel,
            b.offset,
            b.quantizing,
            b.peak,
            b.previous,
            b.shift,
            b.sum_folded
        )
    );
    for (left, right) in a.channels.iter().zip(b.channels.iter()) {
        assert_eq!(
            (left.scale, left.representation as u8, left.k),
            (right.scale, right.representation as u8, right.k)
        );
    }
}

fn progress(t: &Transform<'_>, frames: usize) -> usize {
    (t.channel - 1) * 2 * frames + if t.quantizing { frames } else { 0 } + t.offset
}

fn input(frames: usize, bound: i32, pattern: usize) -> Vec<i32> {
    let mut pcm = vec![0; frames * 6];
    let mut state = 0x73ab192fu32;
    for n in 0..frames {
        pcm[n * 6] = if n % 2 == 0 { i32::MIN } else { i32::MAX };
        for c in 1..6 {
            state = state.wrapping_mul(1664525).wrapping_add(1013904223);
            let amplitude = [0, 0, 31, 511, 32704, bound][c].min(bound);
            pcm[n * 6 + c] = match pattern {
                0 => 0,
                1 => amplitude,
                2 => {
                    if n % 2 == 0 {
                        amplitude
                    } else {
                        -amplitude
                    }
                }
                _ => (state % (2 * amplitude as u32 + 1)) as i32 - amplitude,
            };
        }
    }
    pcm
}

fn compare(input: &[i32], config: Config, budgets: &[usize]) {
    let mut before = input.to_vec();
    let mut after = before.clone();
    let frames = input.len() / 6;
    let mut old = Transform::new(&mut before, config).unwrap();
    let mut new = Transform::new(&mut after, config).unwrap();
    let mut call = 0;
    loop {
        let budget = budgets[call % budgets.len()];
        let previous = progress(&new, frames);
        let expected = old.advance_cheap_reference(budget);
        let actual = new.advance(budget);
        assert_eq!(actual, expected);
        same_state(&old, &new);
        if actual.is_err() {
            return;
        }
        assert_eq!(
            progress(&new, frames) - previous,
            budget.min(10 * frames - previous)
        );
        for n in 0..frames {
            assert_eq!(new.pcm[n * 6], input[n * 6]);
        }
        call += 1;
        assert!(call < 20 * frames + 32);
        if actual == Ok(true) {
            break;
        }
    }
    // Completion remains idempotent, including zero budgets.
    assert_eq!(new.advance(0), Ok(true));
    assert_eq!(new.advance(usize::MAX), Ok(true));
    same_state(&old, &new);
    let encoder = Encoder::new(config).unwrap();
    let mut expected = vec![0xa5; config.workspace_capacity()];
    let mut actual = expected.clone();
    let a = encoder.data(old, &mut expected).unwrap();
    let b = encoder.data(new, &mut actual).unwrap();
    assert_eq!(a, b);
    assert_eq!(expected, actual);
}

#[test]
fn chunked_transform_matches_previous_state_and_bytes_across_budgets() {
    for frames in [1, 7, 255, 256, 257, 1023, 1024, 1025, 4096] {
        for bits in [6, 8, 10, 12] {
            for min_shift in [0, 6, 30] {
                for pattern in 0..4 {
                    let config = Config {
                        block_frames: 4096,
                        bits,
                        min_shift,
                        ..Config::cheap()
                    };
                    let pcm = input(frames, config.max_abs_pcm, pattern);
                    compare(
                        &pcm,
                        config,
                        &[0, 1, 3, 0, 7, 113, 256, 1023, 1024, 1025, usize::MAX],
                    );
                    compare(&pcm, config, if frames < 8 { &[0, 1] } else { &[256] });
                }
            }
        }
    }
    // Largest permitted i32 range still leaves room for one rounding add.
    let config = Config {
        max_abs_pcm: i32::MAX / 2,
        ..Config::cheap()
    };
    for pattern in 1..4 {
        compare(
            &input(257, config.max_abs_pcm, pattern),
            config,
            &[1, 31, 257, 1024],
        );
    }
}

#[test]
fn chunked_peak_failure_preserves_previous_progress_at_chunk_edges() {
    let frames = 257;
    let config = Config::cheap();
    for channel in 1..6 {
        for n in [0, 1, 112, 113, 255, 256] {
            for value in [
                -config.max_abs_pcm - 1,
                config.max_abs_pcm + 1,
                i32::MIN,
                i32::MAX,
            ] {
                let mut pcm = input(frames, config.max_abs_pcm, 3);
                pcm[n * 6 + channel] = value;
                let mut before = pcm.clone();
                let mut after = pcm;
                let mut old = Transform::new(&mut before, config).unwrap();
                let mut new = Transform::new(&mut after, config).unwrap();
                let budgets = [0, 1, 113, 256, 257, usize::MAX];
                let mut call = 0;
                loop {
                    let budget = budgets[call % budgets.len()];
                    let expected = old.advance_cheap_reference(budget);
                    assert_eq!(new.advance(budget), expected);
                    same_state(&old, &new);
                    call += 1;
                    assert!(call < 40);
                    if expected == Err(Error::SampleOutOfRange) {
                        assert_eq!((new.channel, new.offset), (channel, n));
                        assert_eq!(new.advance(0), old.advance_cheap_reference(0));
                        same_state(&old, &new);
                        assert_eq!(new.advance(1), old.advance_cheap_reference(1));
                        same_state(&old, &new);
                        break;
                    }
                }
            }
        }
    }
}
