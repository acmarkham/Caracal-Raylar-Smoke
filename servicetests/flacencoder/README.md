# FLAC encoder validation

The firmware test writes twelve deterministic minute files at 16 kHz: all four
bit depths and all three block sizes. It leaves the global allocator empty and
reports the maximum synchronous encoding time alongside each frame's real-time
budget. It exercises SD materialization and STREAMINFO rewriting. Aardwolf is
the capture/radio/logging concurrency test; this synthetic test does not operate
the microphone.

Build with `rtk cargo build -p service-test-flacencoder --release`. Flash and run
through the normal probe runner only when hardware testing is intended. The
first sample of every file uses the same xorshift32 seed `0x12345678`. Update the
state with shifts 13, 17 and 5, interpret it as signed 32-bit, then arithmetic
shift right by `32 - depth`. Compare 960,000 decoded samples to that sequence.

Host validation uses the allocation-free service with host-only fixture tools:

```powershell
rtk cargo test -p raylar-flac-encoder-service --target x86_64-pc-windows-msvc
rtk cargo run -p raylar-flac-encoder-service --example fixtures --release --target x86_64-pc-windows-msvc -- target/flac-fixtures
rtk proxy python servicetests/flacencoder/verify.py target/flac-fixtures --flac path/to/flac.exe
```

The verifier requires FFmpeg and the Xiph FLAC command-line tool. It compares
both decoders' integer output to the expected PCM and uses Xiph test mode to
validate frame CRCs. A warning about the deliberately absent MD5 is acceptable.
Host fixture files and downloaded verification tools belong under `target/`.

## On-board codec CPU matrix

`flac-cpu-benchmark` is an opt-in binary. It uses aardwolf's 144 MHz PLL tree,
-9 ppm HSE correction and SMPS supply. It leaves the allocator empty, uses
static PCM/frame buffers, and times the codec with DWT. Only the timer interrupt
is active; microphone capture, storage, GPS and radio are not started. This is
codec CPU duty at 16 kHz, rather than whole-application utilization.

The matrix has 720 rows: WAV and production FLAC for recorded audio, silence and
full-range deterministic noise; plus 648 recorded-audio variants covering all
four depths, all three block sizes, auto/order 0–4 prediction, auto/Rice4/Rice5,
and exact bit counts/exhaustive shifts/mean-neighbor Rice search. Each row
processes four distributed 4096-sample excerpts twice. Forced choices retain
constant handling and bounded verbatim fallback, reported in the row counters.
All residuals use one partition. The mean search checks k at the estimated
floor(log2(mean folded residual)) and its immediate neighbors.

Prepare the corpus from captured mono signed little-endian 32-bit PCM. The
conversion uses the full recording's signed width: narrower depths right-shift
only as far as needed to fit; depths that fit preserve the original values.
Preparation and conversion are outside the timed region. Corpus hashes,
excerpt offsets and shifts are recorded in `corpus.json`.

Save the exact existing aardwolf ELF before testing and release any probe debug
session. The runner restores that saved ELF with flash verification and resets
the board in its `finally` block. The benchmark signals a semihosting exit so
probe-rs can release USB normally. Avoid interrupting a probe session by killing
its process; ST-Link/WinUSB can require a physical reconnect afterward.

```powershell
rtk proxy python servicetests/flacencoder/prepare_cpu_corpus.py path/to/audio.s32le --results analysis/flac-cpu
rtk cargo build -p service-test-flacencoder --bin flac-cpu-benchmark --features cpu-benchmark --release
rtk proxy powershell -NoProfile -ExecutionPolicy Bypass -File servicetests/flacencoder/run_cpu_benchmark.ps1 -OriginalFirmware path/to/saved-aardwolf.elf -Results analysis/flac-cpu
rtk proxy python servicetests/flacencoder/verify_cpu_fixtures.py --results analysis/flac-cpu --flac path/to/flac.exe
rtk proxy python servicetests/flacencoder/analyse_cpu.py --results analysis/flac-cpu --firmware target/thumbv8m.main-none-eabihf/release/flac-cpu-benchmark
```

The host verifier independently decodes all 864 experimental combinations with
Xiph FLAC and compares PCM byte for byte. It also checks default exact choices
against production frame bytes. The analyzer requires a complete, unique
720-row matrix, checks both exact searches choose identical frames, and emits
`matrix.csv`, provenance, charts and a filterable `report.html`. CPU deltas are
percentage points relative to WAV packing at the same depth/block size; size
ratios include FLAC frame overhead and use WAV PCM payload as the reference.
Selection and emission are timed separately. Input preparation, frame commit,
executor waits and logging are excluded.

Experimental selector APIs exist only behind the codec's `benchmark` feature.
Normal aardwolf configuration and repository defaults are unchanged.

An additional `cpu-benchmark-fused-mean` feature runs 216 new cases plus the same
72 baselines. It evaluates the same estimated Rice neighbors in one residual
pass, using native 32-bit shifts and sharing candidates between Rice methods.
This distinguishes parameter estimation from the cost of recomputing residuals
for each candidate. The host verifier covers all 864 combinations and confirms
that both mean implementations produce identical frame bytes.

Keep the initial log and ELF before building this supplemental binary. Combine
the two runs into 936 unique matrix rows with:

```powershell
rtk cargo build -p service-test-flacencoder --bin flac-cpu-benchmark --features cpu-benchmark-fused-mean --release
rtk proxy powershell -NoProfile -ExecutionPolicy Bypass -File servicetests/flacencoder/run_cpu_benchmark.ps1 -OriginalFirmware path/to/saved-aardwolf.elf -Results analysis/flac-cpu
rtk proxy python servicetests/flacencoder/analyse_cpu.py --results analysis/flac-cpu --log path/to/base-board.log --firmware path/to/base-benchmark.elf --additional-log analysis/flac-cpu/board.log --additional-firmware target/thumbv8m.main-none-eabihf/release/flac-cpu-benchmark
```

Supplemental deltas use that run's WAV measurements. The additional baseline
observations are retained separately in `repeated-baselines.csv`; both binaries
and logs receive their own hashes in the provenance file.
