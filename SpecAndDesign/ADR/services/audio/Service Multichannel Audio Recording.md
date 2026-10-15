# ADR: Multichannel Audio Recording with a Reference Channel

- **Status:** Implemented; host and laboratory board checks passed, deployment quality and concurrency validation pending
- **Date:** 2026-10-14
- **Decision owners:** Firmware team

## Context

The recorder currently persists one microphone as mono 32-bit integer WAV or lossless FLAC. Six-microphone acquisition has been exercised in `unitsmoke/20_pdm_mic_all_dma`, and the microphone driver already represents `MicrophoneMode::Hexaphonic`. Extend recording to retain spatial information from all six microphones without storing six full-resolution PCM channels.

Keep MIC1 as the full-resolution reference. Store the other five microphones as quantized differences from MIC1 in a custom `.CRCL` sidecar. Downstream tools reconstruct the five channels using the decoded reference and these differences, then perform beamforming or angle-of-arrival estimation.

Sample alignment must be exact. The reconstructed amplitudes will be approximate: quantization can alter interchannel phase estimates, coherence and localization performance. Suitability for downstream tasks therefore determines the production precision, rather than file size alone. Spatial differences are not necessarily small, particularly for diffuse noise, uncorrelated signals or wavelengths comparable to microphone spacing.

Follow the [common constraints](../../../common/AGENTS.md), [Audio policy](../../../Policy/audio.yaml), [Storage policy](../../../Policy/storage.yaml), [Time policy](../../../Policy/time.yaml), and [policy hierarchy](../../../Policy/policy_hierarchy.md). This ADR extends the [Audio Recorder Service](Service%20Audio.md), [AudioSource](../../metadrivers/audiosource/MetaDriver%20AudioSource.md), and [FLAC encoder](Service%20FLAC%20Encoder.md).

## Decision

Add a recording mode independent of the reference container:

| Mode | Capture and persisted output |
| --- | --- |
| `Mono` | Existing one-microphone acquisition and mono 32-bit WAV or FLAC |
| `Hexaphonic` | Six synchronized microphones; mono 32-bit MIC1 WAV or FLAC plus five spatial-difference channels in `.CRCL` |

`Mono` remains the default mode. MIC1 remains the reference in both modes. Preserve the current driver-to-PCM conversion and all resulting reference PCM values; a 32-bit container does not imply 32 effective microphone bits. Hexaphonic mode must not introduce gain normalization, resampling or quantization into the reference path.

Introduce a small, synchronous, heapless codec at `crates/services/crclencoder`, invoked by the recorder. It owns difference quantization, entropy coding and CRCL serialization. It has no independent recording task, source subscription, storage handle or rotation clock. The recorder owns the paired recording session, configuration, metadata and sample boundaries. Storage continues to own paths and persist opaque bytes.

### Configuration

| Parameter | Proposed choice |
| --- | --- |
| Recording mode | `Mono` or `Hexaphonic`; default `Mono` |
| Reference container | Existing WAV or FLAC selection; signed 32-bit mono |
| Reference microphone | MIC1, fixed for version 1 |
| Sample rate | Common to all six channels; initial target 16,000 Hz |
| Segment duration | Default 60 seconds for the profile defined here |
| CRCL block length | 256, 512, 1024, 2048 or 4096 sample frames; initially 1024 |
| Difference precision | 6, 8, 10 or 12 signed bits; initially evaluate 10 |
| Prediction policy | `DirectRice`, `TemporalRice` or `Auto`; initially evaluate `DirectRice` |
| Rice parameter | One fixed parameter per channel block, selected by exact bit cost |

The block length, 10-bit precision and prediction policy are starting evaluation settings. Freeze the deployment settings only after downstream quality and target timing validation. All channels use the configured precision throughout a file; scaling and Rice parameters vary by channel block. A change of mode, precision, rate, block length, prediction policy or reference container takes effect only at a new segment boundary. A mode change may require stopping and restarting capture; account for that gap explicitly.

### Policy overrides

These entries apply to the recording profile defined by this ADR, including its mono option. They do not change repository-wide defaults or activate hexaphonic recording in an existing application. Existing applications adopt the profile explicitly in their target configuration and ADR.

```yaml
policy_overrides:
  - policy: audio
    key: recording_seconds
    value: 60
    reason: Use matching minute-long reference and spatial recordings.
  - policy: audio
    key: container
    applies_when: reference_container_flac
    value: FLAC
    reason: Preserve MIC1 losslessly using the existing mono encoder.
  - policy: audio
    key: finalization
    applies_when: reference_container_flac
    value: Finalize FLAC STREAMINFO and finish its Storage stream before opening the next segment; in hexaphonic mode also finalize and finish the paired CRCL stream.
    reason: The selected reference container and paired recording share one lifecycle.
  - policy: audio
    key: finalization
    applies_when: reference_container_wav
    value: Finalize the WAV header and finish its Storage stream before opening the next segment; in hexaphonic mode also finalize and finish the paired CRCL stream.
    reason: Close both members of a hexaphonic recording together.
  - policy: storage
    key: naming.audio
    applies_when: mono_flac_or_hexaphonic
    value: "aud_{start_utc_s}.{suffix}"
    reason: Use wav or flac for the reference and uppercase CRCL for its companion, with the same timestamp stem.
```

`reference_container_wav` and `reference_container_flac` are mutually exclusive. `mono_flac_or_hexaphonic` means mono with FLAC, or hexaphonic with either reference container. UTC admission, hourly folders, collision handling, protected storage space and error accounting retain their governing policies and target requirements.

## Sample alignment and acquisition

A **sample frame** is one simultaneous PCM value from each active microphone. Every CRCL count and offset is measured in sample frames, equivalent to samples in the mono reference file. AudioSource currently counts interleaved scalar samples; convert its positions and loss counts explicitly at the recorder boundary. Six scalar samples form one hexaphonic frame. Never use a scalar-sample count as a mono reference offset.

The microphone adapter publishes complete frames in stable MIC1 through MIC6 order. It uses the same clock and compatible MDF filter, decimation, packing and gain settings for all channels. Retain capture-start ticks, frame sequence and DMA timing diagnostics. The all-microphone smoke test is the hardware starting point, not proof that the production path has verified relative delay.

Validate synchronized filter startup, CCK0/CCK1 behavior, rising/falling-edge microphones, DMA half selection and fixed filter latency. Do not infer simultaneous samples merely from matching DMA notification counts or completion times. Detect a missing, stale or mismatched microphone buffer before publishing its frame. A fault in any channel invalidates that shared frame.

Use one AudioSource reader in the recorder to extract the reference and all five differences from the same frames. Copy a bounded block into recorder workspace and release the source borrow before compression or asynchronous storage access. Mono-only consumers of a hexaphonic source select MIC1 using a channel view; they must not interpret interleaved array PCM as mono.

Choose the first retained sample using the acquisition timeline and Time Service mapping, rather than metadata-snapshot time or SD write time. Store both the intended UTC segment boundary and the measured first-sample timing anchor. Where a UTC boundary lies between samples, use the first sample at or after that boundary, record the offset and uncertainty, and make the same choice for both files.

Apply the same rule at the scheduled end boundary, using one shared source-frame cutoff. Calibrated clock rate and timing uncertainty can make the actual frame count differ slightly from nominal rate times duration. Derive that cutoff once from the capture/time mapping, record the actual count, and use it for both codecs; do not independently round their durations or discard samples to force a nominal count. Reserve capacity using a conservative maximum frame count for the configured rate uncertainty.

The reference and sidecar cover the identical half-open interval `[0, S)` of file-relative frames. Each DATA record identifies its first reference sample and frame count. Its first offset is zero; successive offsets are contiguous. Both files receive precisely the same frames, with no inserted, duplicated, independently skipped or reordered samples.

CRCL blocks may differ from FLAC blocks and DMA packets. Maintain bounded accumulators and one segment frame counter. Split incoming data at the common segment boundary, flush each codec's final partial block independently, and reset prediction state before the next segment. No block crosses a file boundary.

For a nominal 16 kHz, 60-second segment of 960,000 frames, CRCL blocks of 1024 produce 937 full blocks and a final 512-frame block; FLAC blocks of 2048 produce 468 full blocks and a final 1536-sample frame. Their encoded boundaries need not coincide.

## Difference compression

### Scaling and quantization

Scaling and quantization must operate **in-place in the recorder-owned PCM block**, with no additional sample, difference or quantization buffers. Preserve MIC1 unchanged. For each of MIC2..MIC6, overwrite its samples with spatial differences while finding the block peak, then overwrite those differences with quantized values in a second pass. Reuse these same `i32` slots for entropy coding. Only scalar working state and the per-channel scale parameter are needed. Do not mutate shared AudioSource data or live DMA buffers.

This is a tightly constrained embedded path: minimize RAM, memory traffic and per-sample computation. Use native 32-bit integer arithmetic for difference formation, peak detection, scaling and quantization. The microphone PCM occupies a bounded subset of the 32-bit container range, leaving headroom for subtraction; **do not promote samples or differences to `i64`**.

For each additional microphone `c` and each block of `N` frames, using the existing channel slots:

```text
channel[n] = channel[n] - reference[n]       # i32, in-place first pass
M_c = max(abs(channel[n]))
Q = 2^(b - 1) - 1

if M_c == 0:
    channel[n] = 0
else:
    channel[n] = quantize_ratio_i32(channel[n], Q, M_c)

# Downstream reconstruction, separate from the firmware encoding path:
decoded_delta_c[n] = round_away_from_zero_on_ties(channel[n] * M_c / Q)
decoded_x_c[n] = reference[n] + decoded_delta_c[n]
```

Store `M_c` as the scale parameter. With precision `b`, the exact scale is the rational number `M_c / Q` PCM units per quantization step. `quantize_ratio_i32(d, Q, M)` computes the mathematically rounded ratio `d*Q/M`, with ties away from zero, using overflow-safe 32-bit operations. The mathematical product does not require materializing an intermediate multiplication. Publish vectors for positive and negative ties.

Quantized values use the symmetric range `[-Q, Q]`; the most negative value of a signed `b`-bit integer is reserved and invalid. Peak scaling therefore introduces no encoder clipping. Silence or identical reference/channel blocks use `M_c=0` and an explicit zero representation. Scale each channel separately; one microphone's outlier must not reduce the precision of all five.

Establish the supported PCM bounds from the resolved microphone packing, shift and gain settings. Those bounds must guarantee that subtraction and absolute-value peak detection fit `i32`, excluding `i32::MIN` as a difference. Reject incompatible capture configurations and account for out-of-range samples explicitly. Arbitrary full-range `i32` PCM is outside this firmware codec's input contract.

Headroom for subtraction does not automatically make the intermediate product `d*Q` safe. Keep scaling within 32 bits as well: use a native 32-bit multiply/divide path where the product fits, otherwise an exact bounded quotient/remainder routine. For `Q=2^(b-1)-1`, the latter uses at most `b-1` modular-doubling steps, followed by subtracting the original magnitude and rounding. Comparing a remainder with `M-remainder` avoids overflowing a doubled remainder. This needs only scalar state, at most eleven steps at 12-bit precision, and no per-sample wide arithmetic or wide division. Compute reusable block constants once; do not add general-purpose wide-integer routines, floating-point normalization or sample-sized temporaries.

The absolute reconstructed channel error is at most `M_c/(2*Q) + 0.5` PCM units, including reconstruction rounding. A large transient increases the error bound for its whole channel block. This is one reason to evaluate shorter blocks. The codec does not adjust scale to improve apparent compression or silently drop low-energy differences.

The decoder exposes reconstructed channels in a wider integer or floating representation. Approximation can take their values outside the original `i32` range. An optional 32-bit audio export saturates explicitly and reports saturation; the codec reconstruction itself never wraps or silently clips. Exporting six 32-bit channels does not restore the discarded precision.

### Fixed Rice coding options

Support both requested representations after quantization:

| Representation | Encoded values |
| --- | --- |
| Direct Rice | Signed quantized differences `q[0..N]` |
| Temporal Rice | First quantized value `q[0]` in `b` bits, then `q[n] - q[n-1]` using Rice |

Temporal prediction operates on the **quantized** values already stored in-place. Compute temporal residuals as scalars during cost analysis and emission; do not allocate a residual array or destroy the quantized values needed for packed fallback. Reset prediction for every channel block; do not predict across different scales or across files. The decoder recovers the quantized sequence exactly before applying the stored scale. Temporal residuals range from `-2Q` to `2Q` and fit comfortably in `i32` at every supported precision.

Here, **fixed Rice** means one `k` for the entire channel block, without partitions or per-sample adaptation. Select `k` from `0..b` by exact encoded length and store it. This is separate from fixed-order temporal prediction.

Use the existing signed folding convention:

```text
u = 2*r                  if r >= 0
u = -2*r - 1             otherwise
code = (u >> k) zeros, one terminating bit, then k MSB-first remainder bits
cost(r, k) = (u >> k) + 1 + k
```

Count costs without emitting trial streams. Include the temporal warm-up value and byte padding. `DirectRice` evaluates only direct Rice; `TemporalRice` evaluates only first differences; `Auto` compares both. All policies retain a **packed quantized fallback** using exactly `N*b` payload bits. This fallback stores the same quantized differences, never full-resolution extra channels.

Choose Rice only when it saves at least one payload byte over packed values. Prefer packed on a tie, then direct over temporal, then the smaller `k`. A one-frame block can use packed storage. The zero representation has no payload. This gives bounded output size and prevents long unary codes on unsuitable signals. Decode with explicit record bounds and reject unterminated or excessive unary runs.

## CRCL version 1 binary format

The file is a versioned header followed by DATA and optional TIME records, ending with one END record on successful finalization. It contains only the five additional channels' differences. Decoding all six channels requires the matching reference file.

All byte-aligned numeric fields are little-endian. Bit payloads are MSB-first; packed signed integers use two's complement with exactly `b` bits. Each channel payload ends on a byte boundary with zero padding, validated by the decoder. No native Rust structure is written directly to disk.

Use CRC-32C throughout: reflected polynomial `0x82F63B78`, initial value `0xffffffff`, final XOR `0xffffffff`. The check value for ASCII `123456789` is `0xe3069283`. CRCs detect corruption; they do not make two filesystem writes atomic.

### File header

The fixed prefix is 92 bytes. Fields appear in this order, followed by metadata and a four-byte CRC:

| Field | Bytes | Meaning |
| --- | ---: | --- |
| Magic | 8 | `43 52 43 4c 0d 0a 1a 0a` |
| Version | 2 | `1` |
| Header bytes | 2 | Whole header length, including metadata and CRC; maximum 4096 |
| Flags | 4 | Required version-1 flags are zero |
| Segment identity | 16 | Identifier shared with the reference metadata |
| Sample rate | 4 | Nominal samples per second per microphone |
| Reference depth | 1 | `32` |
| Reference channel ID | 1 | `1`, meaning MIC1 |
| Total channel count | 1 | `6` |
| Difference channel count | 1 | `5` |
| Nominal CRCL block frames | 2 | Configured block capacity |
| Quantized precision | 1 | `6`, `8`, `10` or `12` |
| Reference format | 1 | `1` WAV, `2` FLAC |
| First source frame index | 8 | Capture-timeline index of reference sample zero |
| Intended start UTC | 8 | Non-negative Unix microseconds at the scheduled segment boundary |
| Planned frame count | 8 | Segment target; actual count is in END |
| Metadata bytes | 4 | Exact following metadata length |
| System tick frequency | 4 | Units for all stored system ticks |
| Reserved | 16 | Zero |
| Metadata | Variable | Bounded typed fields described below |
| Header CRC-32C | 4 | Covers every preceding header byte |

`header_bytes = 92 + metadata_bytes + 4`. Reject inconsistent lengths, unsupported versions, nonzero reserved fields or unsupported flags before reading payloads.

Derive the 16-byte segment identity as the first 16 bytes of SHA-256 over ASCII `CRCL-segment-v1`, then device identity length `u16` and its UTF-8 bytes, boot identity `u32`, intended start UTC `u64`, and first source frame index `u64`. Numeric inputs are little-endian, with no separators or padding. Include the identity as 32 lowercase hexadecimal characters, the source index and the companion format in the reference's WAV metadata or FLAC comment. Missing required identity or metadata capacity is a setup error, never truncation.

Metadata uses TLVs: `type:u16`, `flags:u16`, `value_bytes:u32`, then that many value bytes, without padding. Flag bit zero means required; all other bits are reserved. Skip unknown optional types by length and reject unknown required types. Known fields cannot repeat. Required types 1 through 7 and 11 must be present in the file header:

| Type | Value encoding |
| ---: | --- |
| 1 | Capture-origin system ticks `u64`, first-sample system ticks `u64` |
| 2 | First-sample UTC microseconds `u64`, uncertainty microseconds `u64`, UTC status `u8` (`1` synchronized, `2` degraded), calibration locked `u8` (`0` or `1`), two reserved zero bytes, calibration error ppb `i64`; 28 bytes |
| 3 | Nonempty device identity, UTF-8, at most 32 bytes |
| 4 | Boot identity `u32` |
| 5 | Firmware version/build identity, UTF-8, at most 128 bytes; use `unknown` when unavailable |
| 6 | Six channel IDs: bytes `01 02 03 04 05 06` |
| 7 | Resolved capture configuration, UTF-8, at most 512 bytes, as described below |
| 8 | Optional last admitted GPS PPS UTC microseconds `u64` and corresponding system ticks `u64`; 16 bytes |
| 9 | Optional location: latitude E7 `i32`, longitude E7 `i32`, estimate age microseconds `u64`, validity `u8` (`0` unavailable, `1` retained, `2` current), seven reserved zero bytes; 24 bytes |
| 10 | Optional geometry/calibration artifact identity, UTF-8, at most 128 bytes; absence means unavailable |
| 11 | Prediction policy `u8`: `0` direct, `1` temporal, `2` auto |

Type 7 contains semicolon-separated decimal `key=value` fields in this order: `pdm_clock_hz`, `sinc_order`, `decimation`, `cic_scale`, `hpf_enabled`, `pcm_shift`, `gain_tenths_db`. Boolean values are `0` or `1`; signed values use an optional minus sign, with no spaces or leading plus sign. It records the resolved common capture configuration, including the existing PCM conversion. More capture diagnostics can use optional future TLVs.

The calibration artifact identifies microphone positions, coordinate axes and units, plus available gain/delay calibration, and is retained with downstream analysis inputs. Sample-index alignment does not establish calibrated acoustic phase or microphone position. Time checkpoints use types 1, 2 and optionally 8 and 9, with type 1's first-sample ticks referring to the checkpoint frame; the capture origin remains unchanged. TIME requires types 1 and 2 and carries no other version-1 types.

### Record framing

Each record has a 28-byte prefix, optional descriptors, and a four-byte header CRC, followed by its payload and a four-byte record CRC:

| Prefix field | Bytes | Meaning |
| --- | ---: | --- |
| Sync | 4 | ASCII `CRBK` |
| Type | 1 | `1` DATA, `2` TIME, `3` END |
| Flags | 1 | Zero in version 1 |
| Header bytes | 2 | Prefix, descriptors and header CRC |
| Record bytes | 4 | Entire record including both CRCs |
| Sequence | 4 | Contiguous from zero across all record types |
| First frame | 8 | File-relative reference sample offset |
| Frame count | 2 | DATA length; zero for TIME and END |
| Channel count | 1 | Five for DATA; zero otherwise |
| Reserved | 1 | Zero |

The header CRC covers the prefix and descriptors. The record CRC covers the entire record before its trailing CRC, including the header CRC. Validate the header CRC and size bounds before allocating or traversing a payload. Implementations remain bounded even for corrupt input.

A DATA record has five 12-byte descriptors in MIC2..MIC6 order:

| Descriptor field | Bytes | Meaning |
| --- | ---: | --- |
| Channel ID | 1 | `2` through `6` |
| Representation | 1 | `0` zero, `1` direct Rice, `2` temporal Rice, `3` packed |
| Reserved | 1 | Zero; precision comes from the file header |
| Rice `k` | 1 | `0..b` for Rice; zero otherwise |
| Scale `M` | 4 | Peak absolute unquantized difference |
| Payload bytes | 4 | Length of this channel's padded payload |

The DATA header is therefore 92 bytes, and total overhead is 96 bytes including its trailing CRC. Payloads follow in descriptor order. All channels describe the same `N` frames. A zero channel requires `M=0` and zero payload; other representations require `M>0`. Every decoded quantized value must be in `[-Q,Q]`. DATA lengths are `1..B`, with only the final DATA record allowed to be short. The maximum DATA record is `96 + 5*ceil(B*b/8)` bytes.

TIME records use bounded TLVs with the file header's time units. Their first-frame field identifies the next DATA frame offset; they do not insert samples. Emit a checkpoint at most once per recorded second and on a significant time-quality change, coalescing changes within that second. Capture a fresh Time Service/PPS mapping, status and uncertainty; GPS serial-arrival time never becomes a sample clock. A TIME record has at most 512 payload bytes. Sample indices remain authoritative even if UTC uncertainty changes.

END uses the next frame offset, zero frame/channel counts and a fixed 56-byte payload: actual frames `u64`, finalized reference file bytes `u64`, DATA record count `u32`, completion status `u32`, and reference PCM SHA-256 `[u8;32]`. Status bits are `1` normal boundary, `2` deliberate early stop, `4` source discontinuity; exactly one is set. Hash the ordered reference PCM values as signed 32-bit little-endian bytes, incrementally. Offline decoding verifies the reference's sample count and hash before accepting a complete six-channel recording. This binds the sidecar to the actual reference samples, including when filenames or metadata have been copied incorrectly.

Missing END, malformed records, failed integrity checks, an unmatched reference or mismatched totals make the recording incomplete. A recovery tool may expose a validated common prefix with an explicit incomplete result; it must not infer or fabricate missing samples. The version-1 decoder rejects unknown record types and bytes after END. Future incompatible coding or layouts require a new format version.

## Recorder and storage integration

```text
Six-channel microphone driver
          |
          v
AudioSource with interleaved frames and one timeline
          |
          v
Audio Recorder with one reader and segment coordinator
          |                              |
      MIC1 PCM                  MIC2..MIC6 minus MIC1
          |                              |
   WAV or FLAC codec                CRCL codec
          |                              |
          +------ opaque bytes ----------+
                         |
                    Storage Service
```

Keep CRCL codec work outside the core recorder task. Small modules own quantization, Rice selection, record serialization and paired-session coordination. Firmware uses `#![no_std]`, static or caller-owned buffers and no allocator. The codec and paired recorder contain no unsafe code; hardware ownership and the required unsafe interrupt-trait entry points stay in the existing STM32 driver. Shared control/status uses `embassy_sync::watch`; PCM continues to use AudioSource's timeline.

The existing `RecordingStorage` interface already supports explicit start UTC, a format suffix, byte append, generic rewrite and finish. Use two handles in hexaphonic mode, with one metadata snapshot and segment identity. The filesystem layout is, for example:

```text
/{floor(start_utc_s / 3600) * 3600}/aud_{start_utc_s}.flac
/{floor(start_utc_s / 3600) * 3600}/aud_{start_utc_s}.CRCL
```

The WAV variant substitutes `.wav` for `.flac`. These are the same duration and sample interval; their byte lengths differ. Hourly directories derive from the shared start timestamp and never control rotation.

Required adapter changes are narrow but explicit:

- The suffix validator currently permits lowercase ASCII and digits only. Permit uppercase ASCII too, keeping length and path-character restrictions, so Storage can create `.CRCL` without learning its semantics.
- Aardwolf currently configures two open streams. Hexaphonic recording plus its log needs at least three. Budget the additional handle and buffering with other enabled producers.
- Check space once for the whole pair, including both worst-case codec bounds, metadata, record overhead, filesystem allocation and the target's protected log reserve. Do not count on an average Rice or FLAC ratio, or perform two independent reference-only reserve checks. If needed, add a generic byte-budget admission operation to Storage.
- Extend reference metadata serialization within its checked capacity. The current WAV metadata/header and bounded FLAC comment may need more space for pairing fields; do not silently truncate existing identity or timing data.

Validate configuration, buffer capacities, header lengths and admission before opening either stream. Delay materialization until the first retained PCM frame. Create both files exclusively with the same start UTC. If creating or initializing the second fails, stop setup, close any opened stream best-effort, report the incomplete pair and preserve the files. A pre-existing member blocks the pair; never overwrite a collision.

Feed both codecs from the retained frame sequence and keep separate encoded/appended counters. Release the shared Storage lock during compression. At a segment end or deliberate early stop, feed both paths through the same final frame, flush both partial blocks, finalize and successfully close the reference, then append CRCL END with its sample count and digest and finish the sidecar. Report pair completion only after both closes succeed. Begin the next pair only after finalization.

Filesystem writes are not a transaction. If an append, rewrite or close fails, stop the paired recording, retain the error and close handles best-effort. Do not replay a failed append or silently continue in mono. The surviving file may contain a longer prefix; report its independently persisted counts. Power loss can likewise leave an unmatched or truncated member. Offline recovery uses only the common validated interval and cannot treat that as a normally completed segment.

A source gap, per-channel capture error, overrun or synchronization mismatch stops the pair before accepting data beyond the fault. Finalize the available common prefix with discontinuity status where possible, record losses in frame and channel terms, and resume only as a new pair at the next valid segment boundary. Do not bridge gaps with zeros or renumber later samples as though the gap had not occurred. After reboot, preserve unfinished recordings and start new files under the storage collision policy.

## Reuse of the FLAC implementation

Reuse mechanisms where doing so keeps the dependency small:

- Capacity-checked MSB-first bit writes, zero padding and bounded unary writes from `flacencoder/src/bitstream.rs`.
- Signed residual folding, exact Rice cost statistics and fixed order-zero/order-one prediction ideas from `flacencoder/src/predict.rs`.
- Caller-owned persistent workspaces and incremental analysis with bounded executor yields.
- Existing recorder metadata, UTC segmentation, generic storage operations and error reporting.

These helpers are currently private or `pub(crate)`; they are not an existing public codec API. Extract a small internal shared primitive module/crate if it simplifies both implementations, with explicit arithmetic bounds and independent tests. Otherwise adapt the small helpers within CRCL. Preserve FLAC's interoperability checks if its code is refactored.

Do not reuse FLAC frames, STREAMINFO, subframe headers, reserved Rice values or CRC polynomials as the CRCL format. CRCL has its own version, descriptor fields and CRC-32C. Its narrow quantized values do not need FLAC's general predictor search or 32-bit residual policy. The reference continues to use the existing encoder unchanged in behavior.

## Resource bounds and expected storage

Packed fallback bounds the sidecar's DATA payload to five `b`-bit channels. At 16 kHz for 60 seconds, before metadata and framing:

| Difference precision | Five-channel packed payload | Payload plus raw 32-bit mono reference |
| ---: | ---: | ---: |
| 6 bits | 3.60 MB | 7.44 MB |
| 8 bits | 4.80 MB | 8.64 MB |
| 10 bits | 6.00 MB | 9.84 MB |
| 12 bits | 7.20 MB | 11.04 MB |

MB here means 1,000,000 bytes. Six raw 32-bit channels would occupy 23.04 MB per minute. With 1024-frame blocks, 938 DATA records add 90,048 bytes. Header, bounded TIME records, END and filesystem overhead are additional. FLAC can reduce reference bytes, and Rice can reduce the packed sidecar payload; neither saving is guaranteed.

For a segment of `S` frames, reserve the reference codec's worst-case bound plus `sum(96 + 5*ceil(N_i*b/8))` over its CRCL blocks, a 4096-byte header allowance, up to one 548-byte TIME record per recorded second, and a 92-byte END record, then filesystem overhead and protected space. Admission for partial and configured longer segments uses the same bounds.

The workspace uses the existing recorder-owned six-channel PCM block (`24*B` bytes) and output capacity `max(4096, 96 + 5*ceil(B*b/8))` bytes. Difference formation, scaling and quantization overwrite MIC2..MIC6 in that PCM block and require **no additional sample buffers**. At `B=1024`, supporting 12 bits needs 32,352 bytes for these two buffers. Add the existing 2048-sample FLAC input/output workspace, currently 16,416 bytes, for 48,768 bytes of codec buffers. Header construction reuses output workspace; scalar codec state, time/hash state and task stacks are additional. Further buffer reuse is allowed if its lifetimes preserve the reference and alignment requirements.

Audit capture DMA and AudioSource retention separately. Aardwolf's current 128,000-scalar-sample ring retains eight seconds of mono at 16 kHz. Approximately the same scalar capacity retains only 1.33 seconds of six-channel audio, and hexaphonic capacity must be divisible by six. Keeping eight seconds would require 768,000 scalar samples, or 3,072,000 bytes, before other allocations. Choose a realistic retention budget from target RAM and measured SD stalls; do not multiply the mono ring blindly.

Bound uninterrupted work and yield between analysis chunks, as the FLAC recorder currently does. Measure six-channel capture, reference encoding, five-channel scaling/quantization/Rice encoding, hashing and paired writes together. Stable operation requires sustained drain rate above capture rate, bounded worst-case backlog and sufficient scheduling margin with radio, logging and other enabled services.

## Validation and acceptance

Implementation acceptance requires:

1. **Codec correctness:** independent host decoding verifies exact reference PCM, exact recovery of the quantized values for both Rice paths and packed fallback, deterministic scale/rounding, the stated reconstruction error bound, CRC vectors, byte order, padding and SHA-256 binding. Cover all precisions and blocks, one-frame and short final blocks, silence, constants, impulses and extremes of the supported microphone range. Verify in-place transforms preserve MIC1, require no extra sample buffers and match a host mathematical reference. Exercise subtraction/absolute-value bounds, overflow-safe scaling and explicit rejection of unsupported full-range inputs.
2. **Format robustness:** bounded failures for corrupt lengths, IDs, flags, scales, reserved quantized values, excessive unary runs, truncated headers/records, invalid CRCs, unsupported versions and records after END. Test absent/wrong references, identical filenames with different PCM, missing END and recovery of a common prefix.
3. **Alignment and lifecycle:** uniquely indexed synthetic six-channel fixtures demonstrate zero insertion, deletion or relative shift across source wraps, mismatched codec blocks, minute/hour changes, deliberate stops and restarts. Reference and sidecar totals, source indices and segment identities agree. Exercise storage/collision failure at every paired setup, append and finalization stage, plus capture gaps and power-loss tails.
4. **Hardware synchronization:** verify all six channels' startup and relative timing through the production driver, using known common stimuli and expected geometry. Distinguish acoustic delay from clock/filter/DMA misalignment and repeat after capture restart.
5. **Downstream quality:** record a full-resolution six-channel baseline in a bounded lab/test profile. Compare reconstructed data against that baseline using the actual beamformer and angle estimator over the required frequency bands, angles, distances, SNRs, diffuse noise, transients and clipping. Measure pairwise phase/coherence, beam pattern or output SNR, and angle error distributions. Include block boundaries to detect scale-change artifacts. Agree numeric degradation limits before choosing the production precision; PCM SNR alone is insufficient.
6. **Embedded operation:** build the codec without `std`, `alloc` or allocator references, and keep codec/recorder code safe; inspect static RAM and stacks. Sustain the deployment rate under concurrent services and realistic SD stalls, with bounded backlog and no codec-induced loss. Measure worst-case latency, storage rate and energy. Preserve existing mono WAV/FLAC behavior and their required interoperability checks.

Keep codec tests with the new crate, paired recorder cases with the recorder tests, host decoding/evaluation tooling under `scripts/`, and a bounded board exercise under `servicetests/crclencoder`. Host tooling and the lab baseline may allocate or temporarily store all six raw channels; production hexaphonic recording uses the selected reference-plus-differences format.

## Implementation and validation status

The implementation adds `crates/services/crclencoder` and `HexaphonicRecorder` alongside the existing mono recorder in `crates/services/audiorecorder`. Aardwolf selects the paired path with the `hexaphonic` Cargo feature, enabled by default for current testing. Mono can be selected with `--no-default-features --features firmware,core-smps`. The paired service supports WAV and FLAC references, all four precisions, direct/temporal/automatic Rice, in-place 32-bit quantization, matched rotation and failure handling, CRC-32C, time checkpoints and SHA-256 binding. `scripts/crcl_decode.py` supplies independent host verification, six-channel reconstruction and explicit prefix recovery. Optional location-age metadata is omitted when the current metadata source cannot supply an age; the existing reference metadata retains its available location fields.

Array capture now checks independent DMA completion counts for all six microphones and consumes no DMA synchronization-copy buffer. Timestamp handlers run before Embassy clears the hardware flags and count only enabled notifications; a half-transfer flag can latch while its interrupt is disabled. Completion time is also checked against the capture clock to detect collapsed interrupts. All array-driver clients bind the additional channel handlers. Software-published DMA halves and restarts have been exercised; acoustic phase and filter-delay calibration remain separate checks.

Host checks cover in-place arithmetic against a wider mathematical reference, every precision and prediction policy, packed fallback, exact reference PCM through WAV and desktop FLAC decoding, record integrity, paired rotation/tails, gaps and storage failures. The final 2026-10-14 STM32U595 live matrix passed 12 profiles and capture restart. A laboratory Time Service anchor then supported a finalized 960,000-frame minute pair at timestamp `1792046700`, with observed maximum combined codec time 38,051 microseconds, write time 20,681 microseconds and backlog 9,918 scalar samples in a 96,000-scalar-sample ring. These measurements exclude radio and persistent logging concurrency. See the [test instructions and build record](../../../../servicetests/crclencoder/README.md).

GPS did not establish valid UTC during the three-minute observation window. GPS timing, card-extracted desktop round trips, calibrated acoustic alignment, downstream quality thresholds and sustained deployment concurrency/energy remain acceptance work. The initial 10-bit profile is an evaluation setting until those quality measurements select a production precision.

## Experimental temporal-only variant (CRCL v2)

For the CPU/compressibility experiment, select `Config::cheap()` / Aardwolf's `cheap-crcl` feature. MIC2..MIC6 use their own block peaks for power-of-two shift scaling, rounded quantization in-place, and temporal prediction against their own previous quantized samples. No reference-channel subtraction is performed. Each channel/block estimates Rice k from the mean folded temporal residual magnitude using one integer mean and CLZ, without a histogram or k search. Quantization precedes prediction so reconstruction error remains bounded per sample rather than accumulating through a block. MIC1 remains signed-32-bit lossless FLAC.

The experimental v2 container omits all CRCL CRC fields and the reference PCM SHA-256; the recorder also skips hashed segment identity and PCM hashing. Metadata, sample indices, paired minute boundaries, byte-budget admission and failure handling remain. V1 retains its original integrity and spatial-coding contract. The independent decoder dispatches by version, reports absent integrity checks for v2, and preserves structural bounds and sequence checks. Standard reference FLAC frame CRCs remain required by that format. See the [v2 algorithm/layout](../../../../crates/services/crclencoder/cheap.md) and [hardware observations](../../../../servicetests/crclencoder/cheap-profiling.md).

## Alternatives and consequences

Six-channel WAV or lossless multichannel FLAC would preserve every microphone exactly, at greater storage and compute cost. Retain that option for laboratory comparison rather than the production storage design requested here. Five independently compressed files would complicate synchronization, metadata and recovery without improving the sample model.

Storing reduced-precision absolute channels would remove the reference dependency but would quantize each channel's entire signal. Difference coding concentrates the precision on each channel's departure from MIC1. Its benefit depends on the scene and cannot be assumed from microphone proximity alone.

An independent CRCL recorder with its own reader and rotation task would make alignment and failures harder to coordinate. A codec module invoked by the existing recorder keeps one timeline and one session owner. Recording only beamformed output or angle estimates would save more space but prevent later reprocessing with different algorithms.

The decision preserves a standard, full-resolution mono recording and adds bounded spatial storage. It introduces a custom decoder, approximate reconstructed amplitudes, extra capture/codec resources and a dependency on the reference file. The main review choices are the proposed block scaling rule, block length, supported precision range, prediction policy and quality thresholds required before deployment.
