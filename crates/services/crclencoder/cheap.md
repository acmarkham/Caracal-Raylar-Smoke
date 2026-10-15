# Experimental CRCL v2 temporal shift codec

This selectable variant is for measuring CPU cost, compression and downstream quality. V1 remains available. Use `Config::cheap()` in the recorder, `--features cheap-crcl` in Aardwolf, or add `cheap-crcl` to the existing bench profiling features.

Each MIC2..MIC6 block scans only that channel's own PCM peak to select a right shift. It overwrites samples with quantized values in their existing `i32` slots; no sample, spatial-difference or temporal-residual arrays are allocated. MIC1 is never read by this transform and remains unchanged for the lossless reference. Native `i32` arithmetic uses the validated microphone range, leaving headroom for rounding; no PCM arithmetic is promoted to `i64`.

For each channel/block, derive the shift from the absolute peak using CLZ, adjusting once if the rounded positive peak would exceed the symmetric signed precision. Quantization is `q = (pcm + half) >> shift`, where `half = 0` for shift zero or `1 << (shift - 1)` otherwise. Ties round toward positive infinity. Reconstruction is `q << shift`; error is at most half a step, or zero for shift zero. Quantize before temporal differencing so error does not accumulate through a block.

The first quantized sample is a signed `bits`-wide warm-up; subsequent values use `r[n] = q[n] - q[n-1]`. Accumulate only `sum(fold(r))`, with signed folding `2*r` for nonnegative values and `-2*r-1` otherwise. Estimate `k = floor(log2(ceil(sum/(frames-1))))`, or zero for no residuals/all zeros. This is an empirical mean estimate for a geometric residual distribution: one native integer mean calculation and CLZ per channel/block, no histogram or candidate search. Each channel/block gets its own k. No spatial subtraction or reference prediction is performed.

V2 always uses temporal Rice, without a packed fallback. `Config::record_capacity` bounds channel payload to the warm-up plus `(bits+2)` bits per remaining value on average, with byte padding. For `k < bits`, the estimated mean bounds summed quotients below twice the residual count; for `k = bits`, each quotient is at most one. Use `config.workspace_capacity()` for the existing output buffer. The 1024-frame/10-bit DATA bound is 7,768 bytes; this is a bounded output buffer, not additional quantization workspace.

## Binary layout

All integer fields remain little endian and bit payloads remain MSB first. The magic is unchanged, but version `2` and flags `1` identify independent temporal shift coding without CRC or a reference digest. V1 files continue to decode with their original semantics.

| Record | V2 layout |
| --- | --- |
| File header | Fixed 92-byte prefix with the v1 field offsets, followed by metadata TLVs. No trailing CRC. `metadata_bytes = header_bytes - 92`. |
| DATA | 28-byte common prefix plus five 12-byte channel descriptors, then payload: 88-byte header, no header CRC or record trailer. |
| Channel descriptor | Channel ID, representation `2`, reserved zero, Rice k, shift as `u32`, payload bytes as `u32`. Warm-up is signed `bits`; remaining values are Rice temporal residuals. |
| TIME | 28-byte common prefix followed by time metadata TLVs, no CRC. |
| END | 28-byte common prefix followed by total frames `u64`, reference file bytes `u64`, DATA count `u32`, completion status `u32`: 52 bytes total, no SHA-256 or CRC. |

Segment identity is the explicit 16-byte tuple `(UTC microseconds u64, first source frame u64)`. Device and boot identity remain mandatory metadata and reference-comment checks. Metadata, UTC stems/hourly folders, sequence and sample indices, shared rotation, admission and failure handling continue through the paired recorder. The recorder skips reference hashing and hashed segment identity entirely for v2. Standard FLAC frame CRCs remain part of the lossless reference format.

The Python decoder detects the version automatically. V2 retains bounds, sequence, sample-count, metadata and padding checks, but does not detect arbitrary payload corruption or verify reference PCM contents. Reports explicitly contain `crc_present=false` and `digest_verified=false`. `--recover` exposes a structurally checked prefix without claiming checksum validation. Precision and power-of-two scaling need beamforming/angle-estimation evaluation; CPU and byte improvements alone do not establish equivalent fidelity.
