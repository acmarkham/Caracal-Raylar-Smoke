//! Bench-only first-block evidence. Export only after capture and storage stop.
use caracal_evo::pcm::pcm24;
static mut PCM: [i32; 256 * 4] = [0; 256 * 4];
static mut REFERENCE: [u8; 1024 + aardwolf_l4::FLAC_ENCODE_BYTES] =
    [0; 1024 + aardwolf_l4::FLAC_ENCODE_BYTES];
static mut SPATIAL: [u8; 3072] = [0; 3072];
static mut PCM_FRAMES: usize = 0;
static mut LENGTHS: [usize; 2] = [0; 2];
static mut HEADERS: [usize; 2] = [0; 2];
static mut APPENDS: [usize; 2] = [0; 2];

// Single executor: these synchronous calls cannot interleave. Each writer
// exclusively accesses its own array; export runs after all writers stop.
pub fn pcm<const H: usize>(words: &[[u32; H]; 4], offset: usize) {
    unsafe {
        let take = (256 - PCM_FRAMES).min(H - offset);
        let target = &mut *core::ptr::addr_of_mut!(PCM);
        for n in 0..take {
            for c in 0..4 {
                target[(PCM_FRAMES + n) * 4 + c] = pcm24(words[c][offset + n]);
            }
        }
        PCM_FRAMES += take;
    }
}
pub fn encoded(lane: usize, bytes: &[u8]) {
    unsafe {
        if APPENDS[lane] >= 2 {
            return;
        }
        let target: &mut [u8] = if lane == 0 {
            &mut *core::ptr::addr_of_mut!(REFERENCE)
        } else {
            &mut *core::ptr::addr_of_mut!(SPATIAL)
        };
        let start = LENGTHS[lane];
        defmt::assert!(start + bytes.len() <= target.len());
        target[start..start + bytes.len()].copy_from_slice(bytes);
        LENGTHS[lane] += bytes.len();
        if APPENDS[lane] == 0 {
            HEADERS[lane] = bytes.len();
        }
        APPENDS[lane] += 1;
    }
}
pub async fn export() {
    let (pcm, reference, spatial) = unsafe {
        defmt::assert!(PCM_FRAMES == 256 && APPENDS == [2, 2]);
        let reference = &mut *core::ptr::addr_of_mut!(REFERENCE);
        let frame = (LENGTHS[0] - HEADERS[0]) as u32;
        reference[12..15].copy_from_slice(&frame.to_be_bytes()[1..]);
        reference[15..18].copy_from_slice(&frame.to_be_bytes()[1..]);
        let properties = (16_000u64 << 44) | (23u64 << 36) | aardwolf_l4::FLAC_BLOCK as u64;
        reference[18..26].copy_from_slice(&properties.to_be_bytes());
        (
            core::slice::from_raw_parts(core::ptr::addr_of!(PCM).cast::<u8>(), 4096),
            &reference[..LENGTHS[0]],
            &(&*core::ptr::addr_of!(SPATIAL))[..LENGTHS[1]],
        )
    };
    for (kind, bytes) in [("pcm", pcm), ("flac", reference), ("CRCL", spatial)] {
        for (n, chunk) in bytes.chunks(64).enumerate() {
            defmt::info!(
                "L4 QUAD FIXTURE kind={} offset={} data={=[u8]:02x}",
                kind,
                n * 64,
                chunk
            );
            embassy_time::Timer::after_millis(50).await;
        }
    }
}
