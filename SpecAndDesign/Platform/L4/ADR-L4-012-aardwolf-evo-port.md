# ADR-L4-012: Aardwolf on Caracal Evo L4

## Status

Accepted for implementation, 2026-11-25. Application:
[`apps/aardwolf_l4`](../../../apps/aardwolf_l4/README.md).
Hardware results are recorded separately in its
[evidence](../../../apps/aardwolf_l4/evidence/README.md); a successful build is
not hardware acceptance.

The initial Evo profile passed its connected-board bench on 2026-11-25:
thirteen minute WAVs plus concurrent syslog passed complete readback, and the
real receiver completed calibration, standby, UART wake, PPS reacquisition and
phase-qualified shutdown. The normal image then passed two minute rotations
and was left recording. The evidence distinguishes this functional acceptance
from deferred electrical power and extended unattended qualification.

## Context

The U5 Aardwolf application remains the development source of truth. The L4
smokes and [two-stream integration](ADR-L4-011-mono-wav-and-syslog.md) establish
the Evo clock, SDMMC/DMA, DFSDM and chained PPS capture paths. This step composes
those paths with real GPS time and Aardwolf's calibration/duty-cycle model.

The requested first Evo profile is continuously active, externally powered,
mono 16 kHz WAV. It has no radio, inertial sensors, charger/voltage monitoring
or user button. FLAC and current measurements are subsequent work.

## Decision

### Shared source and build boundary

Add the explicit `aardwolf-l4` leaf and `caracal-evo` board crate. Keep the U5
workspace defaults and all U5 manifests/target selection unchanged. Build U5
and L4 in separate Cargo invocations; never combine their HAL chip features in
one workspace build.

Use the existing AudioSource, AudioRecorder, StorageService, LoggingService,
GpsDriver, TimeService and LocationService implementations. Compile the
portable policy directly from `apps/aardwolf/policy.rs` with the L4 file-size
budget. The L4 app composes these services; it does not copy their algorithms.
Future common fixes belong in those upstream services/policy first. Battery
activity gating is not instantiated for a board without voltage readings.

Promote the tested L4 SD, DFSDM, chained capture, PCM interpretation and stack
guard adapters to the new board crate. Preserve the original smoke/integration
targets and their evidence unchanged. Do not import diagnostic executables as
production dependencies. The board crate reads the L4 UID through Embassy and
uses the existing portable UID-to-serial derivation. Never enable the U5-only
identity adapter, which contains a U5 UID address.

Two new driver features are opt-in: `ublox-m10` supplies bounded protocol
framing/commands, and `pps-chained-capture` names the hardware timing source.
U5 builds select neither. Keep disabled additions from moving existing
executable source locations, since panic source locations can change flash
bytes even when behavior is unchanged.

### Evo composition

| Area | Initial application behavior |
| --- | --- |
| Clock/memory | Verified 80 MHz HSE/PLL profile; 96 KiB SRAM1 for statics/DMA, 32 KiB SRAM2 for CPU stack, 1 MiB flash. No inherited U5 measured-HSE correction. |
| SD | Four-bit SDMMC at 24 MHz, DMA2_CH5; existing exFAT partition discovery and one filesystem owner. No formatting or deletion. |
| Audio | PC2 clock, PB12 data, DFSDM channel 1/filter 0, DMA1_CH4; 512-sample halves at nominal 16 kHz, 4096-sample retention. |
| WAV | Upstream 512-byte header, signed 24-bit PCM right-justified in 32-bit words, mono, 960000 samples per minute; 3840512 bytes per completed file. Preserve the upstream scaling convention. |
| Files | Exclusive `/hour/aud_<utc>.wav` creation; existing `/syslog.txt` append. Preserve earlier files and log content. |
| Storage budget | Two stream slots with 512-byte pending buffers, 4096-byte recorder staging, L4-only four-sector filesystem staging. Check Aardwolf's 100 MB reserve plus cluster-rounded next file and pending bytes. Syslog is capped at 1 GB. |
| Logging | Bounded eight-entry/192-byte queue, startup flush, one-second health heartbeat, ten-second checkpoint; raw UART traffic is not logged per byte. |
| GPS | USART2 PA2/PA3, 38400 8N1, buffered interrupt UART. PB4 stays input/no-pull; no rail switching or hardware reset. |
| PPS | PB5 TIM3_CH2 at 1 MHz; TIM3 update TRGO clocks TIM2 over ITR2. Preserve the tested carry reconstruction and capture latency guard. TIM5 remains Embassy's monotonic timer. |
| UI | Blue while an audio stream is open. Green uses upstream search/accepted-anchor policy, sampled every 50 ms. Buzzer and full fault annunciation remain unverified. |

Mount storage and persist startup diagnostics before waiting for navigation.
No artificial UTC or fix is injected. Only a fresh, qualified UBX NAV-PVT fix
allows NMEA through to the shared GPS parser. Only NMEA correlated with captured
PPS reaches the Time Service. Recording waits for real UTC and the next whole
minute, allowing microphone warm-up before that boundary. Subsequent segments
close at exact sample counts. Metadata uses the shared UTC/holdover mapping,
calibration quality, last accepted PPS and filtered location.

Open the first WAV/header ahead of its planned minute while microphone DMA
warms up. Gate PCM publication until that local start instant; trim the first
DMA packet to its first eligible sample. Timestamp the first sample rather
than the DMA completion. This keeps the initial large-card allocation-bitmap
scan and directory creation outside the 256 ms retention window. The existing
filesystem caches its free-cluster count for subsequent boundary checks. A
partial initial packet counts as a whole half in the conservative backlog
bound. Fail if storage setup misses the planned start; never silently discard
recording samples to cover initialization latency.

This is nominal sample-rate recording with UTC metadata, as in the shared
recorder; it does not hardware-lock the microphone sample clock to PPS.
Do not claim phase accuracy beyond the shared conservative 100 microsecond
anchor bound and measured capture alignment. A gap is a failed recording,
never a successful continuous-audio result.

### M10 software standby and wake

Use the M10 protocol, not the U5 receiver's PMTK commands or a CFG-RST restart.
Poll MON-VER, then save the six UART transport keys with CFG-VALSET to RAM and
BBR only; require ACK and read back both layers. Keep 38400 baud and UBX/NMEA
input/output enabled. No receiver flash writes or aiding-database clearing.

Request indefinite standby with RXM-PMREQ version 0: backup/force flags `6`,
UART-RX wake source `8`. Keep supplies connected. Suppress periodic UART polls
throughout standby, since host TX transitions would wake the receiver. Wake
with `0xff`, allow an initial 500 ms settling interval, then re-query identity
and navigation; identity polling retries while awake. That delay is a starting
implementation choice to validate on this receiver.

The M10 manual explains that software standby retains BBR/RTC while clearing
RAM configuration, which motivates the RAM+BBR transport profile. UART activity
and PPS stopping/resuming are functional evidence only. They do not measure
current or prove an electrical power figure. References:
[M10 SPG 5.10 interface, sections 3.10 and 3.16.6](https://content.u-blox.com/sites/default/files/u-blox-M10-SPG-5.10_InterfaceDescription_UBX-21035062.pdf),
[MAX-M10S integration manual, section 3.6.3.2](https://content.u-blox.com/sites/default/files/MAX-M10S_IntegrationManual_UBX-20053088.pdf).

### Timing and duty cycle

Keep the default shared estimator: eleven long-baseline observations spaced
sixty seconds apart, approximately ten minutes before initial lock. Release
the initial continuous tracking interval only when calibration actually locks.
Do not substitute a short elapsed-time test or copy a previous oscillator
measurement into a newly booted estimator.

Use the existing Aardwolf settings: 60-second minimum on time, 1800-second
standby, and post-reacquisition phase qualification with a 180-second maximum
on time, 120 seconds of clean admitted PPS, five consecutive accepted anchors,
250 microsecond residual and 500 microsecond uncertainty limits. Forward
rejections as well as acceptances; coalesced ambiguous observations cannot
extend a good streak. The shared PPS reacquisition gate and holdover model
remain in charge after missing edges. Never escalate automatically to cold
start on an acquisition timeout.

### Bounded resources and acceptance

Use the previous integration's separate polling phases and out-of-line service
construction to bound temporary stack use. Keep a painted stack watermark and
1 KiB guard; assert static SRAM1 fit at link time and check guards before SD
writes. Select nonblocking RTT only on the L4 leaf. Debugger memory reads or
halts during active audio are excluded from acceptance runs.

The first target run exposed a second composition hazard: adding a syslog tail
seek/read to setup increased its polling frame by about 4 KiB, even during the
unrelated later stream-open phase. Put that inspection behind its own polling
boundary as well. A frame-size listing and static fit cannot replace runtime
stack evidence for the complete call chain.

The finite `bench-gps-cycle` image changes only standby from 1800 to 30 seconds.
It must obtain real identity/fixes/PPS, record WAV and syslog concurrently,
complete the full frequency calibration, show at least seven seconds of UART
and PPS silence, wake and admit twenty new anchors, then complete a
phase-qualified subsequent shutdown. Recording continues through holdover and
reacquisition. At the next completed file boundary, stop before allocating a
new audio file, stop DMA, close syslog, and fresh-mount/read every byte written.
Check headers, lengths, sample bounds/variation, recorded hashes, consecutive
syslog sequence numbers and completion markers. Require zero DMA errors,
dropped audio, queue drops, truncations and write errors. Keep a bounded twenty
file verification ledger; a bench that cannot complete the GPS cycle fails.
The normal image has no finite stop or shortened standby.

Build all existing U5 firmware packages, compare effective features and loaded
flash bytes to a clean pre-change baseline, and compare portable host failures
without weakening expectations. Preserve all earlier L4 files. U5 hardware
tests still require a U5 board; this Evo cannot provide that coverage.

## Policy overrides

```yaml
policy_overrides:
  - policy: audio
    key: recording_seconds
    value: 60
    reason: Preserve Aardwolf minute segmentation with the requested mono WAV profile.
  - policy: gps
    key: duty_cycle
    value: {minimum_on_s: 60, standby_s: 1800}
    reason: Preserve U5 Aardwolf application timing rather than bare-driver defaults.
  - policy: gps
    key: phase_qualified_shutdown
    value: {enabled: true, maximum_on_s: 180, minimum_clean_pps_s: 120, residual_limit_us: 250, uncertainty_limit_us: 500, consecutive_accepted_anchors: 5}
    reason: Preserve Aardwolf calibration and phase convergence before standby.
  - policy: gps
    key: duty_cycle.standby_s
    value: 30
    when: bench-gps-cycle only
    reason: Exercise physical standby, wake and convergence in a finite hardware run.
  - policy: power
    key: activity
    value: Continuously active external-power profile; charger, voltage and SOC services absent.
    reason: Requested Evo capability set has no battery readings or charge management.
  - policy: versioning
    key: authority
    value: L4 HAL UID plus portable serial derivation; cached owning-driver GPS identity and application build identity.
    reason: Existing STM32 Versioning adapter reads a U5-only UID address; do not enable it on L4.
  - policy: ui
    key: startup.enabled
    value: false
    reason: PB14 buzzer PWM has not been independently validated on this board.
  - policy: ui
    key: errors.enabled
    value: false
    reason: Initial port uses RTT fail-stop diagnostics; full acoustic/LED fault indication remains deferred.
```

Radio, sensor and button services are not instantiated. Missing capabilities
and unverified identity/CRC fields are reported explicitly. SD exhaustion,
collisions and I/O faults preserve earlier files; this port has no automatic
retention/deletion scheme. Extended unattended operation, reset/power-loss
recovery, physical GPS supply measurements, verified buzzer UI and FLAC require
their own acceptance evidence.
