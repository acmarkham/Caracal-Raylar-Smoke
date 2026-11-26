# ADR-L4-013: Evo fault indication, health telemetry and WAV scaling

Date: 2026-11-26. Status: implemented; hardware qualification recorded separately
in the [validation record](../../../apps/aardwolf_l4/VALIDATION-20261126.md).

This decision updates the L4 Aardwolf composition from
[L4-012](ADR-L4-012-aardwolf-evo-port.md). U5 remains the upstream policy/service
implementation. No shared service or U5 source change is needed.

## Context

The first L4 port initialized the red LED but never drove it on errors. Storage
and acquisition assertions could leave the board frozen without a visible
fault, and card presence was checked only at boot. CPU load was absent from
syslog. A listening test also reported scratchy audio.

Two audio issues require separate treatment. The WAV header described signed
32-bit PCM, but its payload held right-justified, sign-extended 24-bit values.
That scales playback down by 256 (48.16 dB). Also, the initial filter decimated
Sinc5 by 50 to 64 kHz, then used only an integrator/boxcar of four to reach
16 kHz. This provides weak rejection of frequencies that alias in the final
decimation. It is a plausible contributor to the reported sound, not a proven
diagnosis of every audible artifact.

## Decision

### Indication and error containment

Use the repository [UI policy](../../Policy/ui.yaml), with a single async owner
for the three LEDs and buzzer. The Evo adapter uses **PB14 AF3 / TIM8_CH2N**;
the U5 PA5/TIM8_CH1N driver remains unchanged. Hardware PWM requires no buzzer
interrupt or audio-sized buffer. TIM8 is independent of PPS TIM3/TIM2 and the
Embassy TIM5 timebase.

Play startup pitches 1047/1319/1568 Hz for 70 ms each, separated by 35 ms,
volume 96. A fatal recorder fault flashes active-low PB11 red at 1 Hz/50% and
repeats 440/330/220 Hz, 140 ms notes with 70 ms gaps, every ten seconds. Faults
override normal buzzer activity and suppress blue recording indication. Green
continues its GPS role: solid while searching, 50 ms pulses for newly accepted
GPS anchors while tracking, off in standby. Startup does not imply storage or
GPS readiness.

Sample PC6 card detect every nominal 10 ms; six consecutive absent samples
latch an error (about 50–60 ms plus executor scheduling latency). Actual SD I/O
errors can be detected sooner. First fault wins: removed card=1, storage=2,
audio=3, panic/HardFault=4. Stop publishing audio, disable further SD writes,
and park the affected sessions while the UI and healthy independent tasks run.
Reinsertion cannot clear the latch: reset is required. Routine syslog queue
pressure is counted and warned about rather than becoming a fatal panic.

An already active transfer can finish or fail after the latch; subsequent
filesystem calls are refused. There is no attempted flush/remount on an absent
card. A physically removed card cannot store its own error message. RTT and
the retained first-fault code provide diagnostics. Sudden removal can lose
buffered samples, the active file and filesystem metadata; this indication
does not promise hot-removal durability or filesystem repair.

Replace this leaf image's panic-probe handler with a terminal indication
backstop. It disables interrupts and SD/audio DMA, takes terminal GPIO
ownership, and drives the same red/error-note timing using the DWT cycle
counter, independent of Embassy interrupts. Rust and defmt panics both enter
this backstop. The HardFault vector switches to
the existing bottom 1 KiB SRAM2 guard before entering this path, avoiding a
failed thread stack. The guard is used this way only after normal operation
has ended. Terminal GPIO tones use full-duty-range switching; normal PWM uses
policy volume. The terminal path switches SYSCLK to the internal HSI16 and
uses the composition's AHB DIV1 setting, so it also covers clock-init panics.
Disable debugger reset/HardFault catch when checking physical fault indication.

### CPU and recorder health

Enable executor trace hooks only in the L4 leaf and use U5 Aardwolf's existing
thread-executor idle measurement. Every 60 seconds syslog records CPU usage,
active/idle/window milliseconds and an explicit `basis=executor`. Interrupt
work before the executor wakes is included in idle; this is not an
IRQ-inclusive cycle profiler. Wrapping low-word tick arithmetic is safe for
these one-minute windows at the configured 1 MHz Embassy tick rate.

The same cadence reports logging drops, truncations, write failures, maximum
queue depth and maximum audio-copy/storage-append times. Preserve existing
one-second acquisition heartbeats and ten-second GPS/time/location reports
and log checkpoints. The bounded eight-message queue and both concurrent
storage streams remain unchanged. Do not add battery or absent-sensor readings.

### Audio

Keep PC2/PB12, the selected rising-edge channel, 3.2 MHz PDM clock, DMA route,
512-sample halves, 4096-sample retention and 16 kHz output. Use **Sinc4,
FOSR=200, IOSR=1**, channel right shift=8, offset=0. The FCR readback is
`0x80c70000`. Gain is `200^4 = 1,600,000,000`, within signed 32-bit internal
range; shifting by eight fits the signed 24-bit result. This configuration
decimates directly to 16 kHz.

Mask the raw result's low metadata byte and publish `(word & 0xffffff00) as i32`
to this WAV-only producer. The existing shared recorder then writes signed
little-endian PCM32 without any header/service change: the 24 captured bits
occupy bits 31..8 and bits 7..0 are zero. Thus the file remains ordinary PCM32,
with 24 significant captured bits; no 32-bit acoustic resolution is claimed.
The raw `pcm24` helper remains available for analysis and future 24-bit codecs.
Future FLAC must use right-justified signed samples appropriate to its declared
precision, not feed this WAV-scaled representation into a 24-bit encoder.

The [analytic magnitude model](../../../apps/aardwolf_l4/evidence/20261126/filter-model.json)
predicts rejection at 12 kHz improving from 12.49 to 41.82 dB and at 20 kHz
from 20.66 to 59.56 dB. Passband droop increases: 4 kHz is about -3.65 dB and
6 kHz about -8.45 dB. These are normalized digital filter calculations, not
measured acoustic sensitivity or an acoustic anti-alias certification. No
compensation FIR, calibrated microphone gain, DC removal or hardware sample
clock synchronization is added. Filter group delay is not compensated in
the existing DMA timestamp scheme. A fresh listening test is still required.

## Validation and acceptance

Keep U5 defaults, dependencies, features, tests and flash images unchanged.
Capture a clean baseline before editing and compare every original U5 release
target and portable-test outcome. Preserve older L4 smoke/integration sources
and their historical evidence.

Portable tests cover signed/full-scale WAV packing, removal of metadata,
nominal clock/filter gain limits, UI boundaries/debounce, and full-precision
spectral measurement. The `bench-audio-ui` feature records two complete real-UTC
minutes with concurrent syslog and a two-second 1 kHz buzzer tone in the first
file. After closing both streams, mount a fresh filesystem and hash every
byte; verify header/size, zero low byte, varying unclipped PCM, CPU records and
zero drops/DMA/logging failures. Report DC/AC power and 1/2/3 kHz spectral power
against neighboring bins during the stimulus. The buzzer-to-microphone path
has no calibrated acoustic coupling or minimum level; a fixed acoustic SNR
threshold cannot replace a listening test. Retain weak/failed spectral results
explicitly, without describing them as an acoustic pass. This bench
does not replace the earlier full GPS sleep/wake calibration test.

Load the normal image for a live SD-removal test, observe the latch, red LED,
descending/repeating sound and stopped audio/DMA, then reinsert and reset.
Leave the normal image recording for the user's next listening test. Do not
attach a second debugger or halt the CPU during active audio.

The [validation record](../../../apps/aardwolf_l4/VALIDATION-20261126.md)
distinguishes completed checks from any remaining physical confirmation.

## Sources

- [ST RM0351](https://www.st.com/resource/en/reference_manual/rm0351-stm32l47xxx-stm32l48xxx-stm32l49xxx-and-stm32l4axxx-advanced-armbased-32bit-mcus-stmicroelectronics.pdf): DFSDM fast conversion, decimation, right shift and timer/GPIO registers.
- [ST AN4990](https://www.st.com/resource/en/application_note/an4990-getting-started-with-sigmadelta-digital-interface-on-applicable-stm32-microcontrollers-stmicroelectronics.pdf): filter gain/overflow and integrator behavior.
- [ST AN5027](https://www.st.com/resource/en/application_note/an5027-interfacing-pdm-digital-microphones-using-stm32-mcus-and-mpus-stmicroelectronics.pdf): PDM microphone interface/filter guidance.
- [TDK ICS-41350 specification](https://invensense.tdk.com/wp-content/uploads/2016/02/DS-000047-ICS-41350-v1.1.pdf): fitted microphone clock/electrical limits.
- [Microsoft WAVEFORMATEXTENSIBLE](https://learn.microsoft.com/en-us/windows-hardware/drivers/ddi/ksmedia/ns-ksmedia-waveformatextensible): valid-data alignment within PCM containers. The retained legacy header here declares PCM32; the source precision is separately documented as 24 bits.

## Policy overrides

This supersedes the L4-012 Aardwolf exceptions deferring buzzer/fatal indication.
There is no new UI override. Earlier standalone smoke images and their policy
exceptions remain historical and unchanged. All other L4-012 feature/GPS
exceptions remain in effect.
