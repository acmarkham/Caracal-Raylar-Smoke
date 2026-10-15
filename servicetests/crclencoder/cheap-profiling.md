# Cheap CRCL v2 hardware observations, 2026-10-14

The connected STM32U595 ran six real microphones at 16 kHz with GPS off and an explicitly logged Laboratory UTC anchor. The bench used a 144 MHz core, SMPS, 1024-frame/10-bit independent temporal CRCL, 2048-sample/fixed-order-1 signed-32-bit mono FLAC, a 127,998-scalar source ring and 16 KiB storage buffers. V2 uses per-channel shift scaling and estimated Rice k, with no CRCL CRC or PCM SHA-256. All three 960,000-frame minute pairs finalized without dropped source samples; the live matrix passed 6/8/10/12-bit profiles and capture restart.

## Comparison with the previous v1 observations

| Observation | V1 spatial/ratio/Rice search | V2 independent shift/temporal/Rice estimate |
| --- | ---: | ---: |
| Accounted CPU, detailed probes compiled out | 69.86% | 52.79% |
| Accounted CPU, detailed build | 80.35% | 56.00% |
| Accounted CPU, probes disabled at runtime | 78.79% | 54.70% |
| Five-channel encoded payload/minute | 5,647,920 bytes | 3,956,136 bytes |
| Physical SD writes/minute, including reference and filesystem | 7,334,400 bytes | 5,470,208 bytes |
| Measured CRCL CRC calls in detailed minute | 1,997 | 0 |
| Measured reference SHA compression calls | 60,003 | 0 |

The compiled-out control difference is 17.07 percentage points (about 24% fewer accounted CPU cycles per unit time). The observed payload reduction is about 30%; physical SD bytes decreased about 25%. These are observations from separate recordings, not identical PCM A/B tests. Power-of-two scaling changes quantization error, and temporal prediction replaces spatial subtraction, so byte improvements do not establish equivalent beamforming fidelity. CPU-cycle accounting includes interrupts inside probes and excludes unscoped executor work; these percentages are not the application's original executor-idle percentage or measured current.

## Detailed stage costs

| Stage | V2 core budget | Scope |
| --- | ---: | --- |
| Independent transform | 15.13% | Inclusive of peak scan, quantization, residual statistics and loop/probe overhead. |
| Per-channel peak scan | ~4.21% | Sampled component of transform. |
| Shift quantization | ~2.33% | Sampled component of transform. |
| Folded residual accumulation | ~2.49% | Sampled component of transform; no histogram. |
| Rice k estimation | 0.004% | Exact block/channel probe. |
| CRCL bit packing | 16.13% | Largest remaining CRCL stage. |
| Reference FLAC analysis | 2.46% | Component of reference feed. |
| Reference FLAC encode | 7.09% | Component of reference feed. |
| Reference SHA compression | 0% | Hardware assertion confirms absence. |
| CRCL CRC | 0% | Hardware assertion confirms absence. |
| Capture publication/interleaving | 2.57% | Separate capture task. |
| Source copy/timeline validation | 2.71% | Recorder stage. |
| Storage buffering | 0.31% | Common file buffering. |
| Filesystem append polls | 0.63% | Inclusive; SD calls may be nested. |
| SDMMC write polls | 0.95% | CPU work only; elapsed writes took 1.427 seconds. |

Do not sum parents and children. Arithmetic probes sample 1/32 values with per-microphone phase offsets, subtract the empty-span reading cost, and scale by 32. The detailed build changes optimization and retains observer overhead; use its ranking rather than interpreting its stage percentages as exact production costs. The generated raw cycle/max/call/elapsed/byte report is in `target/cheap-audio-profile/` as Markdown, JSON and CSV.

Maximum combined codec time was 36,038 microseconds in the detailed run and 32,199 microseconds in the compiled-out control, against a 64,000-microsecond CRCL block interval. The separate control encountered an 88,815-microsecond write stall near the end of its minute, with peak backlog 29,172 scalar samples; the source ring absorbed it without sample loss. This confirms variable SD latency, not a bound on all future card stalls. Initial file-creation/pre-start backlog is included in the detailed run's larger backlog maximum.

## Traceability and reproduction

- Detailed/runtime-disabled log: `.probe-rs-logs/20261014-174506-service-test-crclencoder-2432/target.log`; Laboratory stems `1792060320`, `1792060380`.
- Compiled-out control log: `.probe-rs-logs/20261014-174932-service-test-crclencoder-11528/target.log`; Laboratory stem `1792060620`.
- Detailed build: `2026-10-14T16:44:55Z`, ELF SHA-256 `37b765fc777ef4205f7001e47e335da8d133ea958e407f48787acd06bd962f65`.
- Control build: `2026-10-14T16:49:22Z`, ELF SHA-256 `313d26e9fba7d554ab943ac11c3271128237a7cd3b4fbc8e3fd022618ea1d376`.
- Both source identities: `8a02ade9e2db7bdef8f1efcd65ecc950e0cbb528-dirty`. Build records are saved beside the report as `profile-build.json` and `control-build.json`. These host artifact hashes are traceability metadata, not codec PCM hashes.

Use `stage-profiling,cheap-crcl` for the two-minute detailed/runtime-disabled run and `profile-control,cheap-crcl` for the separate one-minute control. Both select Laboratory time without enabling GPS and preserve existing card files. The benchmark omits radio, persistent logging and power/location/sensor concurrency. Energy/current and downstream acoustic quality remain separate experiments.

The board was subsequently flashed and verified with Aardwolf `--features cheap-crcl`. V1 remains the normal hexaphonic default for builds that omit this feature. Aardwolf uses its normal UTC/power policy; Laboratory time is confined to the bench test.
