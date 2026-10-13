//! Host-only fixtures for independent libFLAC and FFmpeg validation.
use raylar_flac_encoder_service::{Config, Encoder, Prediction, frame_capacity};
use std::{fs, path::PathBuf};

fn save(folder: &std::path::Path, name: &str, depth: u8, block: usize, samples: &[i32]) {
    save_with_prediction(folder, name, depth, block, samples, Prediction::Auto);
}

fn save_with_prediction(
    folder: &std::path::Path,
    name: &str,
    depth: u8,
    block: usize,
    samples: &[i32],
    prediction: Prediction,
) {
    let mut encoder = Encoder::new(Config {
        bits_per_sample: depth,
        block_samples: block,
        prediction,
        ..Config::default()
    })
    .unwrap();
    let mut buffer = vec![0u8; frame_capacity(block)];
    let length = encoder
        .header(&mut buffer, &["COMMENT=desktop compatibility fixture"])
        .unwrap();
    let mut file = buffer[..length].to_vec();
    let mut choices = Vec::new();
    for chunk in samples.chunks(block) {
        let frame = encoder.encode(chunk, &mut buffer).unwrap();
        choices.push(format!("{:?}", frame.subframe));
        file.extend_from_slice(&buffer[..frame.bytes]);
        encoder.commit(frame).unwrap();
    }
    // Preserve an unfinalized copy to verify the unknown-count streaming header.
    if name == "minute" {
        fs::write(
            folder.join(format!("{name}_{depth}_{block}_stream.flac")),
            &file,
        )
        .unwrap();
    }
    file[8..42].copy_from_slice(&encoder.streaminfo());
    let stem = format!("{name}_{depth}_{block}");
    fs::write(folder.join(format!("{stem}.flac")), file).unwrap();
    let raw: Vec<_> = samples
        .iter()
        .flat_map(|x| x.wrapping_shl((32 - depth).into()).to_le_bytes())
        .collect();
    fs::write(folder.join(format!("{stem}.s32le")), raw).unwrap();
    fs::write(folder.join(format!("{stem}.choices")), choices.join("\n")).unwrap();
}

fn main() {
    let folder = PathBuf::from(
        std::env::args_os()
            .nth(1)
            .expect("fixture output directory"),
    );
    fs::create_dir_all(&folder).unwrap();
    for depth in [8, 16, 24, 32] {
        let limit = (1i64 << (depth - 1)) - 1;
        for block in [1024, 2048, 4096] {
            for n in 1..=16 {
                save(&folder, &format!("short{n}"), depth, block, &vec![1; n]);
            }
            for length in [block - 1, block, block + 1, block * 3 + 7] {
                let mut state = 0x12345678u32;
                let random: Vec<_> = (0..length)
                    .map(|_| {
                        state ^= state << 13;
                        state ^= state >> 17;
                        state ^= state << 5;
                        if depth == 32 {
                            state as i32
                        } else {
                            (state as i32) >> (32 - depth)
                        }
                    })
                    .collect();
                save(&folder, &format!("noise{length}"), depth, block, &random);
            }
            let extremes: Vec<_> = (0..block + 3)
                .map(|i| {
                    if i % 2 == 0 {
                        (-limit - 1) as i32
                    } else {
                        limit as i32
                    }
                })
                .collect();
            save(&folder, "extremes", depth, block, &extremes);
            save(&folder, "silence", depth, block, &vec![0; block + 5]);
            let tone: Vec<_> = (0..block * 3 + 9)
                .map(|i| ((i as f64 * 0.075).sin() * limit as f64 * 0.75) as i32)
                .collect();
            save(&folder, "tone", depth, block, &tone);
            for (name, samples) in [
                ("fixed1-tone", &tone[..]),
                ("fixed1-extremes", &extremes[..]),
                ("fixed1-short", &[1, -1, 2][..]),
                ("fixed1-constant", &[(-limit - 1) as i32; 16][..]),
            ] {
                save_with_prediction(&folder, name, depth, block, samples, Prediction::Fixed(1));
            }
        }
    }
    for order in 0..=4 {
        let samples: Vec<_> = (0i32..128)
            .map(|i| {
                if order == 0 {
                    ((i * 77 + i * i * 39) % 17) - 8
                } else {
                    (i + 10).pow(order)
                }
            })
            .collect();
        save(&folder, &format!("order{order}"), 32, 2048, &samples);
    }
    // Cross both UTF-8 frame-number boundaries 127 and 2047.
    save(&folder, "numbers", 8, 1024, &vec![0; 2050 * 1024 + 1]);
    let minute: Vec<_> = (0..960_000)
        .map(|i| (((i as f64 * 0.013).sin() + (i as f64 * 0.017).cos()) * 100_000.0) as i32)
        .collect();
    save(&folder, "minute", 32, 2048, &minute);
    println!("Fixtures written to {}", folder.display());
}
