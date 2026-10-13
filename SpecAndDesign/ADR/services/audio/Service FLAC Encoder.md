# ADR: Minimal FLAC Encoder Service

- **Status:** Accepted design; implementation pending
- **Date:** 2026-10-13
- **Decision owners:** Firmware team

## Context

Aardwolf currently records mono 16 kHz audio in a 32-bit integer WAV container. Lossless compression can reduce SD traffic and recording capacity without changing the PCM values supplied by `AudioSource`. Compression depends on the signal; storage planning must also accommodate incompressible audio.

The [Audio Recorder Service](Service%20Audio.md) already permits additional formats. Add a small FLAC encoder service that the recorder can select alongside WAV. The recorder continues to own sessions, metadata, UTC alignment and file boundaries. The encoder owns FLAC encoding, and Storage continues to persist opaque bytes.

Follow [common constraints](../../../common/AGENTS.md), the [Audio policy](../../../Policy/audio.yaml), the [Storage policy](../../../Policy/storage.yaml), and the [policy hierarchy](../../../Policy/policy_hierarchy.md). Aardwolf's conditional FLAC exceptions are recorded in its [application ADR](../../apps/app/_aardwolf/ADR%20Aardwolf%20GPS-Calibrated%20Audio%20Logger.md). This decision adds a format; it does not change the repository's WAV default.

## Decision

Implement an independent Rust service crate at `crates/services/flacencoder`, provisionally named `raylar_flac_encoder_service`. It is a synchronous codec component invoked by the Audio Recorder Service, with no separate Embassy task, microphone subscription or storage handle. Use `#![no_std]`, fixed arrays and, where useful, bounded `heapless` collections. Neither the crate nor its firmware dependency graph may require `alloc`, an allocator or `std`.

Use [flacenc](https://github.com/yotarok/flacenc-rs) as an algorithm and serialization reference, especially its [subframe selection](https://github.com/yotarok/flacenc-rs/blob/main/src/coding.rs) and [Rice parameter search](https://github.com/yotarok/flacenc-rs/blob/main/src/rice.rs). Do not add it as a Cargo dependency or vendor its allocation-based pipeline. Its reusable sample/error buffers and owned frame representations do not meet our memory constraint. Reimplement the required operations over caller-owned buffers. Record the reference revision during implementation; retain the applicable Apache-2.0 notices if code is adapted.

The bitstream authority is [RFC 9639](https://www.rfc-editor.org/rfc/rfc9639.html). Reference code does not replace format validation, particularly for 32-bit input.

### Initial encoder profile

| Property | Decision |
| --- | --- |
| File format | Native `.flac`, beginning with `fLaC`; no Ogg wrapper |
| Channels | Mono only; reject other channel counts |
| PCM bit depths | Signed 8, 16, 24 and 32 bits; default 32 |
| Sample rate | Fixed for a file; initial aardwolf rate 16,000 Hz |
| Nominal block size | 1024, 2048 or 4096 samples, fixed for a file |
| Initial block size | 2048 samples, subject to hardware timing and RAM validation |
| Subframes | Constant, verbatim, fixed predictors of orders 0 through 4 |
| Residuals | Rice coding with either 4-bit or 5-bit parameters |
| Rice partitions | Partition order zero: one partition per subframe |
| Order selection | Smallest exact encoded frame size among legal candidates |
| Arithmetic | Integer sample, prediction and residual arithmetic; floating point permitted only for future selection estimates |
| Metadata | STREAMINFO and a bounded Vorbis comment block |
| STREAMINFO MD5 | All zero in the initial implementation, explicitly indicating unavailable |

General LPC, stereo decorrelation, arbitrary block sizes, adaptive block lengths, Rice escape partitions, seek tables, artwork and FLAC decoding are outside the initial scope. Wasted-bit removal is also deferred; do not assume that unused microphone resolution means any specific low bits are zero.

## Service ownership and integration

```text
Timestamped AudioSource PCM
            |
            v
Audio Recorder Service
  session, metadata, segment boundary
  format selection: WAV or FLAC
            |
            v
FLAC Encoder Service
  block analysis, frame encoding, container metadata
            |
       encoded bytes
            v
Storage Service
  paths, new-file creation, buffering, byte rewrites, close
```

Factor FLAC-specific work into the additional crate while keeping the selected container's lifetime inside the recorder. Aardwolf selects the format before opening a recording. WAV remains available through its existing container. Format, sample rate, bit depth and nominal block size may change only when starting a new file.

The codec consumes bounded slices of signed `i32` PCM, encodes into a supplied byte slice, and returns the initialized byte count and small frame statistics. It has explicit configuration and errors such as `InvalidConfig`, `SampleOutOfRange`, `OutputTooSmall` and `CounterOverflow`. An invalid prediction is an internal candidate rejection, not a recording error, because verbatim encoding remains available.

The recorder accumulates a block from one or more AudioSource reads; microphone DMA packet sizes do not set FLAC block sizes. Copy samples into a recorder-owned block before analysis, then release the AudioSource borrow/lock. Do not hold the source lock or the shared Storage mutex during compression. Perform asynchronous storage writes only after a complete frame has been encoded.

### Storage adaptation

The current implementation has three WAV assumptions: `StreamKind::Audio` produces a `.wav` name in `crates/services/storage/src/policy.rs`; the `RecordingStorage` trait has no format/suffix argument; and aardwolf's `finalize_audio` adapter rewrites WAV fields at offsets 4 and 508.

Extend stream creation with a bounded, validated file suffix or equivalent generic descriptor, supplied by the recorder (`wav` or `flac`). Storage still generates the complete path, retains exclusive new-file creation, and treats the suffix as naming data. Do not implement FLAC parsing or encoding in Storage.

Expose generic `rewrite(stream, byte_offset, bytes)` through the recording adapter, using the existing Storage rewrite capability. Containers construct their own final metadata bytes. The recorder requests these rewrites and then calls generic finish. Move knowledge of WAV and FLAC offsets out of the aardwolf storage adapter. FLAC must never pass through WAV finalization.

This is a bounded generalization of the byte-stream interface, refining the original Audio ADR's expectation of unchanged Storage. Storage's content-agnostic responsibility is preserved.

## PCM representation and losslessness

The configured bit depth describes the signed sample values, not the width of the Rust storage type. The codec accepts sign-extended, right-aligned `i32` values. For depth `b`, each value must fit `[-2^(b-1), 2^(b-1)-1]`; reject values that do not fit before writing their frame. In particular, 8-bit FLAC samples are signed, and 24-bit values use three encoded bytes per verbatim sample.

The default 32-bit path reproduces every `AudioSource` sample exactly. Aardwolf already shifts microphone words when feeding AudioSource; that existing acquisition conversion is upstream of this encoder. FLAC adds no scaling, clipping, extra shift, quantization or dithering.

Selecting 8, 16 or 24 bits is valid when the supplied numerical samples already fit that depth. Do not silently narrow the current 32-bit source. Any intentional reduction of resolution needs an explicit upstream conversion and separate policy; FLAC is lossless relative to the resulting PCM.

## Blocking and recording boundaries

Let `B` be the nominal block size in samples. All ordinary frames contain `B` samples and use FLAC's fixed-block strategy and a zero-based sequential frame number. The final frame may contain `1..B` samples; it carries its actual length and is the only short frame. Each new file starts at frame number zero.

At 16 kHz, block durations are 64, 128 and 256 ms for 1024, 2048 and 4096 samples respectively. A minute contains 960,000 samples: with `B=2048`, write 468 complete frames and a final frame of 1536 samples. Do not pad the last frame, drop its remainder, or move samples across a UTC file boundary to fill a block.

On rotation or a clean early stop, encode the buffered remainder, finalize the container and finish the Storage stream. New segments start with fresh encoder state. Each frame is independently predicted; there is no prediction history across frames or files. If no samples were captured, do not publish the segment as a successful recording or manufacture an empty audio frame.

For an AudioSource overrun or timestamp discontinuity, account for the loss and close the continuous prefix, including its buffered tail. Resume in a new file with the actual new start time and the established UTC boundary policy. Do not concatenate samples from opposite sides of an unreported gap or insert synthetic silence. This applies the Audio policy's discontinuity requirement to FLAC.

## Fixed prediction and numerical bounds

For each frame, evaluate orders `p=0..4` where `N > p`, with `N` the actual sample count. Store the first `p` samples as warm-ups, then calculate `r[n] = x[n] - prediction[n]`.

| Order | Prediction |
| ---: | --- |
| 0 | `0` |
| 1 | `x[n-1]` |
| 2 | `2*x[n-1] - x[n-2]` |
| 3 | `3*x[n-1] - 3*x[n-2] + x[n-3]` |
| 4 | `4*x[n-1] - 6*x[n-2] + 4*x[n-3] - x[n-4]` |

Promote samples to `i64` before every prediction multiplication, subtraction or finite difference. Full-range 32-bit input with order four can require 36-bit residual arithmetic. Never calculate in `i32` and widen afterwards, wrap or saturate a prediction.

FLAC-coded residuals have the stricter range `[-2147483647, 2147483647]`. Reject an order if any residual falls outside it, including `i32::MIN`. This can happen even at order zero for a frame containing that minimum sample. Other legal predictors can still be tried; if none compresses legally, write verbatim. Constant and verbatim subframes can represent the full configured sample range.

Use `u64` for bit costs and folded-residual calculations. Floating-point estimates may later reduce search work, but exact integer arithmetic determines every emitted sample and residual. In particular, converting full-resolution samples through `f32` is not lossless.

## Rice coding and candidate selection

Start with partition order zero, so each fixed subframe has one Rice parameter and `N-p` residuals. This keeps search and state small. Adding more partitions later requires a measured benefit and separate decisions about partition counts, warm-up subtraction and divisibility; it is not necessary for valid FLAC.

Fold each validated signed residual into an unsigned value:

```text
u = 2*r          when r >= 0
u = -2*r - 1     when r < 0
q = u >> k
m = u & ((1 << k) - 1)
```

Write `q` zero bits, a terminating one bit, and then `k` remainder bits, most significant first. For `k=0`, there are no remainder bits. Compute folding in the wider type before narrowing to a validated unsigned value.

Evaluate Rice method `00` with parameter width `w=4` and `k=0..14`, and method `01` with `w=5` and `k=0..30`. Values 15 and 31 respectively are escape markers and are never emitted as Rice parameters. Support the 5-bit method from the outset so large residuals from 24/32-bit sources do not force excessively long unary codes.

With no wasted bits, compare these subframe costs in bits:

```text
constant:  8 + b                         (only when all samples equal)
verbatim:  8 + N*b
fixed:     8 + p*b + 2 + 4 + w + sum((u >> k) + 1 + k)
```

The fixed cost includes the subframe header, warm-ups, residual method, partition-order field and Rice parameter. Add byte-alignment padding before comparing complete frame lengths; the frame header and CRC sizes are identical across these candidates for the same input block.

Pick the smallest legal frame. Prefer constant when tied, then verbatim over prediction, then the lower predictor order, the 4-bit Rice method, and the smaller `k`. Thus prediction is used only when it saves at least one encoded byte over verbatim. The actual order varies with the source; there is no globally fixed order waiting to be chosen.

Cost evaluation counts unary lengths arithmetically and never emits trial bitstreams. Cap losing candidate costs at the verbatim bound to avoid unnecessary work, while fully validating the selected candidate. Only then emit its codes. This prevents unsuitable Rice parameters from producing unbounded unary output. Recompute the selected residuals during emission instead of retaining residual arrays for every order.

## Memory and execution bounds

Use a single `[i32; B]` input block and a caller-owned output buffer sized for a worst-case mono 32-bit verbatim frame. `4*B + 32` bytes is a conservative frame capacity for this profile, including the maximum supported frame header, subframe header, alignment and CRC footer. A frame selected by the cost rule cannot exceed the equivalent verbatim frame.

| Block samples | PCM buffer bytes | Frame capacity bytes | Combined buffer bytes |
| ---: | ---: | ---: | ---: |
| 1024 | 4096 | 4128 | 8224 |
| 2048 | 8192 | 8224 | 16416 |
| 4096 | 16384 | 16416 | 32800 |

These are buffer budgets, not the complete application RAM footprint. Add small encoder state, at most 31 `u64` Rice-cost accumulators reused per order, and a metadata buffer capped at 1024 bytes. AudioSource retention, DMA, Storage buffering and task stacks are separate existing allocations. Reuse the recorder's format-specific workspace where possible instead of retaining its current 16 KiB WAV byte buffer in addition to all FLAC buffers.

Allocate these buffers once in static resources or caller-owned persistent storage. Do not return large frame arrays by value or create block-sized temporaries on task stacks. If stable Rust requires a separate output-capacity const parameter, validate its relationship to `B` during construction. Every bit-writer operation checks capacity and returns an error rather than panicking or growing storage. Do not use recursion, unsafe code or dynamically dispatched allocated codec objects.

An exhaustive initial search has a fixed upper bound of five orders and 31 Rice parameters per sample. Accumulate costs over input samples using the reusable small accumulator array, then perform a second pass for the selected representation. Its CPU cost is bounded, but must be measured on the STM32 target.

For deployment, frame analysis plus encoding must fit inside `B/sample_rate` with scheduling margin under concurrent radio and logging activity. Bound uninterrupted CPU work; yield between bounded analysis chunks or run on an appropriate lower-priority executor if measurements require it. Storage stalls must fit the existing AudioSource retention budget. Use measured worst-case latency and RAM, alongside compression ratio and energy, to confirm 2048 or select another supported block size. An FPU does not remove the need for these measurements.

## File structure and metadata

The file consists of the native marker, STREAMINFO, one Vorbis comment block, and complete audio frames. STREAMINFO is first, has its standard 34-byte payload, and records the configured rate, mono channel count, bit depth and nominal block size. The comment block is marked as the last metadata block. No frame or file trailer is invented.

Initially set STREAMINFO's minimum and maximum block sizes to `B`, including files that ultimately contain only a short final frame. Set frame-size bounds and total sample count to zero while unknown. At a successful close, rewrite the observed frame-size bounds and actual total samples using the generic byte rewrite interface. Keep the nominal block-size fields at `B`; the final short frame is handled separately. Reject sample counts that exceed the 36-bit field or frame numbers that exceed their 31-bit range before an overflowing frame is appended.

Keep the 16-byte MD5 field zero in this minimal version. This is the standard indication that the PCM digest is unavailable; frame CRCs still provide frame integrity checks. Some desktop verification tools may report the missing digest. Do not claim whole-stream MD5 verification. A future addition can compute MD5 incrementally with fixed state and patch the digest at close without changing the audio encoding API.

Serialize existing `RecordingMetadata` fields into bounded UTF-8 `KEY=VALUE` Vorbis comments: start UTC and monotonic time, UTC status/uncertainty, device/card/boot/firmware identity, calibration and PPS correlation, and location hash with its algorithm and input representation. Include the selected bit depth and nominal block size for interpretation. Rate and channels are also authoritative in STREAMINFO. Retain final sample count and loss/finalization outcomes in the recording diagnostics.

FLAC integer fields use the prescribed bit ordering; Vorbis comment length/count fields have their own little-endian encoding. Construct and validate metadata length before opening the stream. Overflow is an explicit error, not silent truncation of time or identity fields. Do not put WAV RIFF chunks or an invented application metadata ID into the FLAC file.

### Frame serialization

Emit fixed-block frame headers, explicit sample rate and bit depth, mono channel assignment, the sequential FLAC-coded frame number, and header CRC-8. Use the common block-size codes for full blocks and an explicit actual length for the last partial block. At 16 kHz the frame rate code is explicit; configured rates must be representable both in STREAMINFO and an explicit standard frame-header rate representation or be rejected at construction.

Encode the selected subframe, add only the required zero bits for byte alignment, and append frame CRC-16. CRC-8 uses polynomial `0x07`; CRC-16 uses `0x8005`, both initialized to zero. Header CRC covers the header before its CRC byte. Frame CRC covers the full frame before its two CRC bytes, including header CRC and alignment bits. Keep reserved bits zero and encode frame numbers with FLAC's numeric UTF-8-like scheme rather than a general Unicode text encoder.

The supported block sizes, explicit frame properties and partition order zero fit FLAC's streamable subset. Desktop compatibility still needs validation at all four bit depths, particularly 32-bit audio in current tools; older FLAC implementations may have narrower support.

## Finalization and failures

1. Snapshot metadata and validate configuration, buffer capacities and metadata bounds before opening a new stream.
2. Write initial container metadata and append only fully encoded frames. Advance committed sample/frame statistics after successful append.
3. At the exact recording boundary, append any non-empty final partial frame.
4. Rewrite final STREAMINFO statistics, then finish the Storage stream. A failed rewrite or close is a finalization failure and must be reported.
5. Reset codec and block state before the next file. Preserve existing files and the application's storage/log reserve.

Initial unknown sample/frame-size fields allow decoding without waiting for finalization. A sudden power loss can still leave buffered bytes missing or a truncated final frame; recoverable complete frames are not a guarantee of a valid finalized file. Do not resume an unfinished FLAC file after reboot. Storage errors stop the affected recording and produce diagnostics where available, with no automatic switch to WAV inside that file.

Space reservation uses the conservative verbatim-frame bound and metadata allowance, not a promised compression ratio. For a segment of `S` mono samples at depth `b`, reserve at least `ceil(S*b/8) + 32*ceil(S/B)` audio bytes plus bounded file metadata and filesystem overhead. Existing power, UTC alignment, retention and protected-log-space requirements still apply.

## Alternatives and consequences

Using flacenc or the C libFLAC encoder directly would bring an allocation-oriented runtime into firmware. A bespoke codec limited to the required FLAC primitives provides explicit memory and execution bounds, at the cost of owning bitstream correctness and interoperability tests.

A fixed predictor order or selection by mean residual magnitude would be simpler, but does not measure warm-up, Rice and byte-alignment costs. Exact comparison over five orders resolves the source-dependent choice and gives a useful size bound. More Rice partitions, general LPC and adaptive block sizes could improve compression; defer them until measurements justify their complexity.

Unknown total samples throughout a file would permit an append-only implementation. Aardwolf already has generic rewrite support, so finalizing the actual count improves desktop duration reporting and permits accurate partial stops. Omitting the MD5 digest keeps the first encoder smaller, while requiring independent PCM equality checks during validation.

Expected benefits are reduced storage traffic and capacity for correlated audio, with exact sample preservation. Costs include an additional input block, bounded frame workspace, CPU work, format-neutral adapter changes, and new codec verification responsibilities. High-entropy input may use more space than raw PCM because frame overhead remains even with verbatim fallback.

## Validation and acceptance

The implementation is complete only when the following checks pass:

- Build the service and aardwolf FLAC profile for the embedded target without `std`, `alloc` or allocator symbols. Inspect transitive dependencies and the linker map, including workspace and task-stack memory.
- For each bit depth and block size, decode generated files with current Xiph `flac`/libFLAC and FFmpeg. Compare sample rate, channel count, exact sample count and every signed PCM value with the input. Normalize desktop raw-output alignment/signedness before comparison; do not compare floating-point playback output.
- Validate silence and non-zero constants, ramps and polynomial sequences, tones, realistic microphone recordings, clipping, alternating extremes and deterministic random samples. Verify orders 0 through 4 can be exercised, along with verbatim fallback, both Rice methods, parameter boundary values and `k=0`.
- Include full-range 32-bit inputs and residuals at and outside the legal limits, especially `i32::MIN`. Confirm candidate rejection preserves samples through another legal representation.
- Exercise lengths 1 through 16, `B-1`, `B`, `B+1`, multiple blocks and the 960,000-sample minute case. Check the only partial frame is last and UTC segmentation neither duplicates nor discards samples.
- Check metadata lengths and final flags, CRC coverage and known vectors, numeric frame-number boundaries, output-capacity failure, invalid configuration/sample ranges, and finalized STREAMINFO counts. Desktop test mode must report no frame errors; any missing-MD5 warning must agree with the deliberately zero digest.
- Inject source gaps, early stops, append failures, rewrite failures, storage exhaustion and reboot with an unfinished tail. Verify loss reporting, clean-prefix handling, new-file creation and correct `.flac` names without overwriting earlier files. Preserve WAV operation.
- Measure worst-case analysis/encoding time, uninterrupted execution time, RAM and stack use, SD stalls, compression ratio and energy on aardwolf during radio/logging activity. Confirm sustained 16 kHz mono operation without codec-induced source overrun, and record the selected production block size.

Keep host codec tests with the new crate and hardware service tests under `servicetests/flacencoder`, with recorder lifecycle cases in `servicetests/audiorecorder`. Host tooling may allocate; the firmware codec and its runtime dependencies may not. Use flacenc as a separate host-side reference where helpful, with matching capabilities and bit depths, but independent desktop decoding is the acceptance authority. Byte-for-byte equality with another encoder's compressed output is not required.

## Remaining implementation measurements

The required format, supported orders, exact initial selection rule, residual bounds and memory strategy are decided. Confirm the deployed block size and executor scheduling from board measurements. Any future search heuristic must retain legal-residual validation and the verbatim size bound; any additional encoding capability requires an explicit design update.
