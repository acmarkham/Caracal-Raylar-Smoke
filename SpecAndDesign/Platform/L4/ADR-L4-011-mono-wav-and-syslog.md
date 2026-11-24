# ADR-L4-011: Minute WAV recording and concurrent syslog on Evo

## Status

Accepted for implementation, 2026-11-24. Target:
[`integrationtestsL4/integration001_monoaudiolog`](../../../integrationtestsL4/integration001_monoaudiolog/README.md).
Actual results belong in its [evidence record](../../../integrationtestsL4/integration001_monoaudiolog/evidence/README.md).

## Context

[L4-007](ADR-L4-007-sdmmc-dma-exfat.md) verified SDMMC/DMA and exclusive exFAT
file I/O. [L4-010](ADR-L4-010-dfsdm-mono-audio.md) verified 16 kHz mono DFSDM
acquisition and the portable AudioSource representation. This integration needs
to demonstrate that microphone acquisition continues while two persistent
streams share one filesystem, including minute boundaries and log flushes.
Real GPS synchronization is deliberately omitted at the user's request.

## Decision

Add an explicitly selected L4 integration package outside the root U5
`default-members` globs. Reuse the unchanged upstream AudioSource, Audio
Recorder, Storage, Logging and Time services. Reference the verified L4 smoke
adapters locally rather than copying the U5 hardware backend or modifying the
earlier smokes. No new service behavior is selected by the U5 defaults.

The normal image records continuously, rotating at exactly 960000 mono samples
per file. A `bench-three-minutes` feature records three complete files, closes
both clients, stops audio DMA and verifies the files with a fresh filesystem
instance. The bench adapter returns a distinct planned completion result when
the recorder requests a fourth stream, after the third file has closed and
before allocating any fourth file. No in-flight filesystem operation is
cancelled to stop the test. The default image has no three-file stop.

### Resources and data flow

| Resource | Composition |
| --- | --- |
| CPU clock | Verified HSE16 / PLL M2 N20 R2: 80 MHz system, AHB and APB clocks. |
| SD kernel | PLLSAI1 M2 N12 Q2: 48 MHz; SDMMC 4-bit bus at 24 MHz. |
| SD pins / DMA | PC12 clock, PD2 command, PC8–PC11 data; PC6 detect; DMA2 channel 5/request 7. |
| Microphone | PC2 AF6 clock, PB12 AF6 data; the same one-phase channel 1/filter 0 profile as 05. |
| Audio DMA | DMA1 channel 4/request 0; 1024 words split into 512-sample / 32 ms halves. |
| AudioSource | 4096 signed samples / 16 KiB, 256 ms of retention; one recorder reader. |
| Recorder staging | 4096 bytes; unchanged upstream WAV serialization. |
| Storage Service | Exactly two slots, each with 512 bytes of partial-sector buffering. Whole recorder buffers still pass through as multi-sector writes. |
| Logging | Eight bounded 192-byte messages, 256-byte formatted lines; one heartbeat per second and checkpoint every ten seconds. |
| Filesystem | Existing patched exFAT engine, one cache block, portable StorageDriver and one filesystem owner. |
| CPU stack | Dedicated 32 KiB physical SRAM2; painted watermark and 1 KiB bottom guard reused from 02. Statics and all DMA buffers remain in 96 KiB SRAM1. |
| Heap | Fixed 4096 bytes for filesystem naming/allocation conveniences. |

An asynchronous mutex serializes all Storage Service operations. Recorder and
logger each own a distinct stream handle; neither owns a second filesystem or
SD driver. Audio DMA has its own channel and task, and continues while storage
operations await SD DMA. This is concurrent stream lifetimes and interleaved
I/O through one device, not simultaneous SD bus transactions.

The capture adapter rejects transfer errors, missed DMA halves, overwrite,
selected-channel clock absence, filter overrun and invalid sample flags. It
discards the verified 2.048-second warm-up. Before every AudioSource publication
it checks a conservative producer-minus-consumer backlog against retention.
Consumption is credited only after the recorder returns from its storage append,
so this bound also covers the final bench packet and cannot hide its loss behind
the planned completion result. Any exhausted retention or reported drop fails
the test rather than presenting gapped audio as continuous.

### Filesystem staging and stack constraints

The existing exFAT write future reserves a 64 KiB multi-sector staging array;
the upstream profiling wrapper and two pending stream clients amplify its
reserved async state. Smaller application buffers alone do not remove that
array. Add an opt-in `exfat-slim/small-write-batches` feature that stages four
512-byte sectors, or 2 KiB. The default remains 128 sectors / 64 KiB, and its
existing assertion remains unchanged in default tests. The small feature has
its own batch-size assertion and runs the same file I/O regression cases.
Only this integration package selects it. Build chips separately so Cargo
cannot merge this resource choice into a U5 build.

Keep setup, recording and verification polling phases separate using an
out-of-line `Future::poll` wrapper. Keep large service construction out of the
common polling frame as well. Link-time static fit alone is insufficient:
an initial composition fit SRAM1 but generated a CPU stack frame over 40 KiB.
Inspect the generated frame allocations and require intact stack guards during
real two-stream writes and after full readback. Ordinary debugger memory attach
or halting during capture can interrupt CPU servicing while DMA continues; only
inspect RAM after acquisition has stopped.

### WAV files and simulated time

Use the upstream 512-byte WAV header and recorder exactly as composed on U5.
Each minute is mono, nominal 16000 Hz, 960000 samples, a 3840000-byte payload
and **3840512 bytes including the header**. The WAV declares 32-bit integer PCM.
The microphone supplies signed 24-bit values, right-justified/sign-extended in
`i32`, matching the current upstream producer/recorder convention. This smoke
does not normalize amplitude or change the shared WAV sample scaling.

Inject one clearly labelled `TimeSource::Laboratory` anchor into the unchanged
Time Service. It maps local monotonic time to a simulated epoch; its uncertainty
starts at one second, calibration is unlocked and holdover uncertainty grows.
No GPS fix, PPS, coordinates or radio are fabricated. Green stays off. The
firmware hash field carries an explicit `FAKE-GPS:` prefix in WAV metadata,
and syslog records the simulated source and epoch before acquisition begins.
This tests composition and file boundaries, not UTC accuracy or GPS alignment.

Choose an unused simulated hour beginning at 2026-01-01, bounded to 4096
candidates, rather than overwriting a previous run's WAVs. Keep Storage Service
hourly folders and `aud_<start>.wav` names. Audio creation is exclusive; a later
collision or insufficient free space fails without replacing earlier files.
Existing `/syslog.txt` is reopened for append, retaining its previous content.
Do not format, delete files or reconfigure the attached GPS. The default image
runs until stopped, a collision, insufficient space or an I/O failure; it is not
a retention/automatic-deletion implementation.

### Acceptance and evidence

For the bench build, require all three WAVs to close at exact minute sample
counts with zero DMA errors, ring overwrite, lost audio samples, log drops,
truncation or write failures. Observe two open streams and at least seventeen
successful syslog checkpoints while audio is open. Record worst observed
retention backlog, capture processing, storage append time and stack watermark.

After both clients close, prohibit further SD writes and reconstruct the
filesystem from the same device. Read every byte of all three WAV files;
validate headers, lengths, 24-bit sample bounds, changing low bits and rolling
hashes accumulated before the writes. Verify the complete appended syslog
suffix against its original length, byte count and hash, including consecutive
log sequence numbers, heartbeat records, three closure records and a terminal
bench marker. Completion depends on this readback, not just successful append
calls. The finite image then blinks blue; the continuous image keeps recording.

Retain the exact bench and continuous ELF/BIN, source/build identity, features,
memory layout, target logs and measurements. Run the default and small-batch
filesystem tests plus the existing U5 firmware/host regression matrix. Compare
effective features and loadable U5 image bytes; distinguish source/debug-info
changes from executable behavior. Do not change baseline U5 expectations or
claim U5 hardware coverage from this L4 board.

## Policy overrides

Storage naming, exclusive creation, append-only syslog and ten-second logging
checkpoints follow the shared policies. The target chooses mono/16 kHz as
permitted by the audio policy. The deliberate lab time and minute segmentation
are explicit test settings; the default Time Service estimator is unchanged.
Full production version discovery and UI fault annunciation are not composed.

```yaml
policy_overrides:
  - policy: audio
    key: recording_seconds
    value: 60
    reason: This integration exercises one-minute sample-count boundaries and repeated file rotation.
  - policy: audio
    key: alignment
    value: Start from an explicit simulated whole-minute epoch; rotate by sample count without waiting for physical GPS.
    reason: The user requested fake GPS for this initial SD/audio integration.
  - policy: versioning
    key: authority
    value: Persist compile-time package, Git and build identity; report unavailable device/module fields explicitly.
    reason: This small L4 composition omits the U5 Versioning hardware adapter.
  - policy: ui
    key: startup.enabled
    value: false
    reason: No buzzer is initialized.
  - policy: ui
    key: errors.enabled
    value: false
    reason: Diagnostic failure is reported through RTT/panic-probe; application fault annunciation is absent.
```
