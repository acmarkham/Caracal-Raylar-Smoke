# CRCL target and desktop validation

## Isolated 1024-frame CRCL loop benchmark

The opt-in `crcl-loop-benchmark` binary uses a deterministic 500 Hz sinusoid plus
seeded integer noise. It starts no capture, SD, GPS, radio or executor tasks and
masks interrupts while timing with DWT CYCCNT. It measures production transforms
and serialization, individual arithmetic kernels, and byte-identical test-only
Rice packing candidates. No production encoding behavior changes.

```powershell
rtk proxy powershell -ExecutionPolicy Bypass -File scripts/run_crcl_loop_benchmark.ps1
```

Each stage has eight warm-ups and 129 measurements; the board remains idle after
`CRCL LOOP BENCH PASS`. See [Analysis 003](../../docs/audio/multichannel/analysis003.md)
for measured cycles, pipeline pseudocode, caveats, artifacts and reproduction.
The production single-codeword/u32-reservoir integration and repeat measurements
are recorded in [Analysis 004](../../docs/audio/multichannel/analysis004.md).
The bounded transform-loop optimization and same-image frozen-transform controls
are recorded in [Analysis 005](../../docs/audio/multichannel/analysis005.md).
The package now has two binaries, so select `-Bin service-test-crclencoder` for
the existing microphone/SD diagnostics below; the benchmark runner selects its
own binary explicitly.

## Raw microphone noise diagnostic

The `noise-floor` feature bypasses the codec matrix and SD recording. It uses the
production six-channel Sinc5/R50/reshape/HPF preset, discards two seconds of startup,
then copies four continuous seconds of unquantized PCM into a diagnostic-only
1,536,000-byte RAM buffer. GPS, SD and radio tasks are not started. The normal
`>> 8` hardware unpacking is applied; no CRCL scaling or quantization is applied.
All six DMA sequences must advance without gaps; target sums, squared sums and
extrema allow independent verification of the debugger readback.

```powershell
rtk proxy powershell -ExecutionPolicy Bypass -Command "& ./scripts/firmware.ps1 -Package service-test-crclencoder -Bin service-test-crclencoder -MonitorSeconds 30 -Until 'NOISE CAPTURE READY' -CargoArgs @('--features','noise-floor')"
# Use ADDRESS from NOISE CAPTURE READY; it can change between builds.
rtk proxy probe-rs read --chip STM32U595VJ --non-interactive b32 ADDRESS 384000 --format binary --output target/noise-floor/raw.i32le
rtk proxy python scripts/mic_noise_report.py target/noise-floor/raw.i32le --target-log .probe-rs-logs/latest.log --output target/noise-floor/report
```

The Python report requires NumPy, SciPy and Matplotlib. It exports per-channel RMS,
A-weighted and band-limited noise, error from candidate integer shifts, interchannel
correlation, JSON, CSV and a spectrum plot. Its dBFS24 reference is the RMS of a
full-scale signed-24-bit sine, not full-scale i32 or microphone-input dBFS.
Measurements include the acoustic environment and board noise; this is not a
calibrated microphone self-noise measurement. Restore the desired application image
after reading the buffer; the diagnostic leaves acquisition stopped.

The default firmware captures six real microphones using the production MDF driver, exercises 6/8/10/12 bits with direct, temporal and automatic Rice selection, checks the unchanged MIC1 digest and record CRCs, and tests stopping/restarting capture. The allocator remains uninitialized; workspaces are statically allocated.

```powershell
rtk proxy powershell -ExecutionPolicy Bypass -File scripts/firmware.ps1 -Package service-test-crclencoder -Bin service-test-crclencoder -MonitorSeconds 20 -Until 'CRCL TARGET PASS'
```

Add `sd-recording` to record and finalize one matching minute of mono 32-bit FLAC plus CRCL on the inserted card. This profile uses GPS UTC and stops its acquisition wait after 180 seconds:

```powershell
rtk proxy powershell -ExecutionPolicy Bypass -Command "& ./scripts/firmware.ps1 -Package service-test-crclencoder -Bin service-test-crclencoder -MonitorSeconds 300 -Until 'CRCL SD TARGET PASS' -CargoArgs @('--features','sd-recording')"
```

For a bench without GPS reception, use `laboratory-time`, which includes `sd-recording`. It injects an explicitly logged Laboratory anchor through the Time Service. The anchor and file names vary with firmware build time; this profile does not validate GPS time and is unavailable in Aardwolf:

```powershell
rtk proxy powershell -ExecutionPolicy Bypass -Command "& ./scripts/firmware.ps1 -Package service-test-crclencoder -Bin service-test-crclencoder -MonitorSeconds 240 -Until 'CRCL SD TARGET PASS' -CargoArgs @('--features','laboratory-time')"
```

Both profiles preserve existing card files, create the pair exclusively, check space for both members, and stop after successful finalization. A repeated filename fails instead of overwriting prior data. The test uses no radio or persistent logging task; Aardwolf concurrency and deployment energy require a separate soak.

## CPU stage profiling without GPS

`stage-profiling` includes laboratory time and records two complete minute pairs: detailed profiling, then a runtime-disabled control. It uses Aardwolf's 144 MHz core, SMPS supply, 127,998-scalar source ring, 1024-frame/10-bit/direct CRCL, 2048-sample/fixed-order-1 FLAC, and 16 KiB write buffers. GPS remains off. Existing files are preserved.

```powershell
rtk proxy powershell -ExecutionPolicy Bypass -Command "& ./scripts/firmware.ps1 -Package service-test-crclencoder -Bin service-test-crclencoder -MonitorSeconds 600 -Until 'CRCL PROFILE TARGET PASS' -CargoArgs @('--features','stage-profiling')"
```

The `raylar-stage-profiling` crate uses DWT CYCCNT with 64-bit accumulated counters, so totals survive multiple counter wraps. Synchronous probes never cross yields; async wrappers measure each poll separately and report end-to-end elapsed time separately. Stages include capture validation/publication, source copy, spatial transform, sampled subtraction/quantization/Rice statistics, Rice selection, CRCL packing/CRC, reference SHA-256 compression, FLAC analysis/encoding, storage buffering, filesystem append/finalization, and SDMMC reads/writes. Per-sample arithmetic is sampled at 1/32 with a phase offset per microphone. Counters are static; no heap or sample workspace is added. Normal builds compile the probes away.

Stage totals are inclusive. Do not add parents to their children, or mistake SD elapsed time for CPU work or measured energy. Interrupt cycles that occur within a probe are included; unscoped interrupt/executor work is not fully attributed. The bench excludes Aardwolf radio, logging, sensors, power and location concurrency. Initial file creation and finalization are included in each recording window; pre-recording mount and the deliberate capture restart are excluded.

Use `profile-control` for a separate one-minute build with detailed probes compiled out, retaining only top-level recorder/capture counters. Disabling probes at runtime still retains instrumentation branches and can affect compiler optimization.

```powershell
rtk proxy powershell -ExecutionPolicy Bypass -Command "& ./scripts/firmware.ps1 -Package service-test-crclencoder -Bin service-test-crclencoder -MonitorSeconds 360 -Until 'CRCL PROFILE CONTROL PASS' -CargoArgs @('--features','profile-control')"
rtk proxy python scripts/audio_profile_report.py <detailed-session>/target.log --compiled-control-log <control-session>/target.log --output-dir target/audio-profile
```

The report script requires target PASS, all stages, and exactly 960,000 frames per window. It exports Markdown, raw JSON and CSV. Preserve the detailed image's build record before building the control image, since `firmware-builds/service-test-crclencoder.json` describes the latest build. See [measured stage costs](profiling.md).

Add `cheap-crcl` to either profiling build for the independent-channel shift/temporal/Rice-estimate variant, for example `@('--features','stage-profiling,cheap-crcl')` or `@('--features','profile-control,cheap-crcl')`. Its live matrix checks four precisions and verifies MIC1 directly without SHA or CRC; paired profiling asserts zero CRCL CRC and SHA-compression calls. Host fixtures cover both versions, WAV/FLAC, complete segments and partial tails. See the [cheap variant](../../crates/services/crclencoder/cheap.md) and [measured results](cheap-profiling.md).

Allow enough wall time for the next minute boundary, a full captured minute, flashing and debugger overhead. Target timestamps may advance more slowly than the host monitoring clock while the debugger is attached.

Generate paired host fixtures and run the independent decoder checks:

```powershell
$env:RAYLAR_CRCL_TEST_OUTPUT = (Join-Path (Get-Location) 'target/crcl-fixtures')
rtk cargo test -p raylar-audio-recorder-service --target x86_64-pc-windows-msvc multichannel
rtk proxy python scripts/test_crcl_decode.py
```

Use an absolute fixture path because Cargo runs tests from the package directory. The tests cover WAV and FLAC reference equality, partial tails, rotation, source loss, setup/write/rewrite/close failures, wrong references, truncated tails and corruption.

The final 2026-10-14 STM32U595 board run passed all 12 live codec profiles and capture restart. The paired SD test then finalized 960,000 aligned frames at laboratory UTC `1792046700`, using 1024-frame/10-bit/direct CRCL blocks and 2048-sample/fixed-order-1 FLAC. Maximum combined codec time was 38,051 microseconds, write time 20,681 microseconds, and source backlog 9,918 interleaved scalar samples. The test used a 96,000-scalar-sample ring. These are workload observations, not a worst-case proof or energy result.

The GPS profile did not acquire valid UTC in the three-minute observation window. Acoustic phase/delay calibration, real beamforming/angle-estimation degradation limits, independent desktop decoding of card-extracted recordings, and sustained radio/logging concurrency remain deployment checks. The [build record](../../firmware-builds/service-test-crclencoder.json) identifies the target test image; bounded RTT captures are in `.probe-rs-logs/`.
