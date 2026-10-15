# Hexaphonic stage profiling, 2026-10-14

The STM32U595 test used six real microphones, GPS off, explicitly marked laboratory UTC, a 144 MHz core with SMPS, 1024-frame/10-bit/direct CRCL, 2048-sample/fixed-order-1 FLAC, 16 KiB write buffers and Aardwolf's source-ring capacity. It finalized two 960,000-frame minute pairs without dropped source samples. The deliberate restart before recording discarded one stale/partial DMA half; no discontinuity occurred during either recorded minute.

Detailed log: `.probe-rs-logs/20261014-152803-service-test-crclencoder-28028/target.log`. Laboratory filename stems: `1792052100` and `1792052160`. Build identity is saved in `target/audio-profile/profile-build.json`; the exact ELF SHA-256 in that record binds this observation to the tested image.

Tested ELF identities, both release builds from `8a02ade9e2db7bdef8f1efcd65ecc950e0cbb528-dirty`:

- Detailed, built `2026-10-14T14:27:45Z`: `875f29dda8f8c6e8d51aabf251d53a14efd4fe231497d02cfc6a9b91b9be83ae`.
- Compiled-out control, built `2026-10-14T16:02:34Z`: `cdc63a9b0fda63b25dec432ba5813cba052316ccf7b67484f4338e1b0dd29e66`.

The detailed window took 60.189 seconds. Top-level recorder/capture polls accounted for 80.35% of its core cycle budget. Runtime-disabled detailed probes accounted for 78.79% in the following minute. These figures describe this instrumented bench, not the application's reported 64% executor load or a current measurement. Runtime-disabled probes retain branches and can alter optimization; use the separate compiled-out control below to assess this effect.

The separate `profile-control` build, with detailed probes compiled out, finalized another 960,000-frame minute at laboratory stem `1792057800`. It measured **69.86% accounted CPU** over 60.167 seconds, with maximum combined codec time 39,751 microseconds and maximum write time 26,517 microseconds. Log: `.probe-rs-logs/20261014-170333-service-test-crclencoder-35772/target.log`; build record: `target/audio-profile/control-build.json`. No discontinuity occurred during its recording interval. The difference from the detailed build is about 10.5 percentage points; instrumentation/compiler effects are material, and acoustics and SD allocation also differ between runs. Do not transfer the table's absolute stage percentages to production or uniformly rescale them to the control. Use the ranking to select targeted experiments.

The generated inclusive cycle report, raw JSON and CSV are in `target/audio-profile/`. The control's cycle accounting includes interrupts within probes, unlike the existing executor-idle calculation, and the bench lacks other Aardwolf services; it does not establish why the application's recorded load was exactly 64%.

| Stage | Core budget | Relationship |
| --- | ---: | --- |
| Spatial transform | 28.70% | Includes the sampled components and transform loop/profiling overhead below. |
| Spatial subtraction/peak | ~4.10% | Sampled component of transform. |
| Quantization, fast path | ~5.99% | Sampled component of transform. |
| Quantization, wide fallback | ~0.0001% | One of 150,000 sampled quantizations. |
| Rice statistics | ~9.06% | Sampled component of transform. |
| Rice parameter selection | 0.04% | Component of transform; exact search across candidate parameters is cheap here. |
| CRCL serialization | 17.94% | Includes packing and data-record CRC. |
| CRCL bit packing | 16.55% | Component of serialization. |
| CRCL CRC | 1.38% | All record CRC calls, including small metadata records. |
| Reference SHA-256 compression | 6.04% | Compression rounds only; reference feed also includes update/copy overhead. |
| FLAC analysis | 2.45% | Component of reference feed. |
| FLAC encoding | 7.30% | Component of reference feed. |
| Capture publication/interleaving | 2.53% | Separate capture task. |
| Recorder source copy | 2.63% | Includes timeline validation. |
| Storage-buffer copy | 0.44% | Common to both files. |
| Filesystem append | 0.90% | Inclusive CPU during polls; SD operations can be nested. |
| SDMMC write polls | 1.07% | Includes writes outside append, such as file creation/finalization. |

Do not sum parent and child rows. Per-sample components sample 1/32 values with a phase offset per microphone, subtract the measured empty-span cost, and scale by 32. They are estimates; interrupts during a sample can affect them. Read the raw report for cycle counts, maxima, calls/polls and asynchronous elapsed time. The overall accounted cycle budget excludes unscoped executor/interrupt work.

Of 4,690 channel blocks, 4,595 selected direct Rice and 95 selected packed fallback. The five-channel payload was 5,647,920 bytes versus 6,000,000 bytes for fixed 10-bit packing: a 5.87% saving before record overhead. Rice is being selected frequently, but offers a modest byte reduction for this recording. This is one acoustic input, not a universal compression ratio.

SDMMC wrote 7,334,400 physical bytes including filesystem overhead. Write operations took 2.270 seconds end-to-end, while measured CPU during their polls was 0.644 seconds. End-to-end elapsed time includes suspension and scheduler delay; it does not isolate card busy time or establish SD energy consumption. Maximum combined codec time was 46,363 microseconds against a 64,000-microsecond block budget; maximum write time was 28,697 microseconds. Peak backlog includes the initial segment admission/file-creation delay and pre-start samples.

The first optimization candidates supported by this run are cheaper Rice statistics, faster bit packing, and reference hashing/feed overhead. Quantization's wide fallback and searching for the final Rice parameter are not major costs on this input. Test fixed-width packing against the existing Rice path using the same captured PCM before trading extra SD bytes for less CPU. SD batching still deserves an energy experiment, but this profile does not identify SD polling as the dominant CPU consumer.
