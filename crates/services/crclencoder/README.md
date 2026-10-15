# CRCL encoder

Heapless, `no_std` CRCL v1 encoding for five spatial differences alongside a lossless mono MIC1 reference. See the [ADR](../../../SpecAndDesign/ADR/services/audio/Service%20Multichannel%20Audio%20Recording.md) for the file layout and recording lifecycle.

`Transform::new` borrows one owned interleaved six-channel PCM block. Call `advance(256)` between executor yields until complete, then pass the transform to `Encoder::data`. MIC1 remains unchanged; MIC2..MIC6 become quantized differences in their existing `i32` slots. Scaling uses native 32-bit arithmetic with an overflow-safe fallback. There are no difference, quantization or temporal-residual arrays.

The caller supplies output capacity through `workspace_capacity(block_frames, bits)`, appends each complete record, and commits the codec counters only after a successful append. Direct Rice, first-difference Rice and automatic selection all retain a packed quantized fallback. Input must satisfy `Config::max_abs_pcm`; arbitrary full-range `i32` audio is rejected. The default supports the signed 24-bit microphone values produced by the current eight-bit PCM shift.

Run host checks with:

```powershell
rtk cargo test -p raylar-crcl-encoder-service --target x86_64-pc-windows-msvc
```

The default profiling hooks compile away. The optional `defmt` feature supports firmware diagnostics. In v1, a fixed-state SHA-256 binds the paired reference PCM and CRC-32C protects headers and records.

Select `Config::cheap()` (`Algorithm::TemporalShiftV2`) for the experimental independent-channel variant and size the existing output buffer with `config.workspace_capacity()`. It uses in-place `i32` shift quantization with one scale per channel/block, temporal-only residuals, an estimated Rice parameter, and no CRCL CRC or PCM SHA-256. MIC1 remains unchanged. See the [algorithm and v2 layout](cheap.md) and [hardware results](../../../servicetests/crclencoder/cheap-profiling.md).

Use [crcl_decode.py](../../../scripts/crcl_decode.py) to independently validate and reconstruct recordings:

```powershell
rtk proxy python scripts/crcl_decode.py aud_1792045380.CRCL --reference aud_1792045380.flac --output six-channel.wav
```

WAV references use Python's standard library. FLAC references require FFmpeg on PATH. `--recover` explicitly returns an incomplete validated prefix when a tail is missing or corrupt; it does not invent samples or claim a verified reference digest. Normal decoding verifies segment identity, sample offsets, record CRCs, exact reference sample count and SHA-256 before accepting completion. Reconstructed values retain the quantization error described in the ADR.
