//! Host replay/benchmark of captured signed little-endian 32-bit PCM.
use raylar_flac_encoder_service::{frame_capacity, Config, Encoder};
use std::{fs, path::PathBuf, time::Instant};

fn main() {
    let mut arguments = std::env::args_os().skip(1);
    let input = PathBuf::from(arguments.next().expect("input .s32le"));
    let output = PathBuf::from(arguments.next().expect("output .flac"));
    let raw = fs::read(input).unwrap();
    assert!(raw.len().is_multiple_of(4));
    let pcm: Vec<_> = raw
        .chunks_exact(4)
        .map(|x| i32::from_le_bytes(x.try_into().unwrap()))
        .collect();
    let mut encoder = Encoder::new(Config::default()).unwrap();
    let mut buffer = vec![0u8; frame_capacity(2048)];
    let mut file = Vec::new();
    let started = Instant::now();
    for pass in 0..10 {
        encoder.reset();
        if pass == 0 {
            let n = encoder
                .header(&mut buffer, &["COMMENT=captured aardwolf PCM replay"])
                .unwrap();
            file.extend_from_slice(&buffer[..n]);
        }
        for chunk in pcm.chunks(2048) {
            let frame = encoder.encode(chunk, &mut buffer).unwrap();
            if pass == 0 {
                file.extend_from_slice(&buffer[..frame.bytes]);
            }
            encoder.commit(frame).unwrap();
        }
    }
    let elapsed = started.elapsed();
    file[8..42].copy_from_slice(&encoder.streaminfo());
    fs::write(output, &file).unwrap();
    println!(
        "10 replays: samples={} frames_per_pass={} bytes={} elapsed_us={} us_per_frame={:.1}",
        pcm.len(),
        pcm.len().div_ceil(2048),
        file.len(),
        elapsed.as_micros(),
        elapsed.as_micros() as f64 / (10 * pcm.len().div_ceil(2048)) as f64
    );
}
