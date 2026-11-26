# L4 Aardwolf: fault UI, telemetry and audio validation

Date: 2026-11-26. [Decision](../../SpecAndDesign/Platform/L4/ADR-L4-013-fault-ui-telemetry-audio.md).

Raw local evidence is retained under `apps/aardwolf_l4/evidence/20261126/`,
which is already ignored by the repository. Frozen ELF/BIN copies are under
`target/l4-hardening/`. This reviewable summary records the conclusions;
the earlier port/smoke evidence is unchanged.

## Build and U5 regression

The clean baseline was HEAD `a7f50cb3920dd27cbbf6d2b2553e60f896cd8673`.
All 61 original U5 firmware packages build. All 66 ELF files and loadable flash
images match their pre-change hashes. Eleven effective feature trees, root
default members and 212 earlier L4 source/evidence files are unchanged.
U5 hardware was not connected or tested.

Portable baseline and post-change outcomes are identical: 203 pass, five existing
failures (`gps::framer::tests::frames_sentence` and four Time Service estimator
tests: `estimates_frequency_error_with_long_baseline_regression`,
`frequency_regression_rejects_one_in_window_outlier`,
`hardware_scale_sample_handles_small_capture_quantization`,
`holdover_warning_is_one_way_until_an_anchor_is_accepted`). No expectation or
test was changed to obtain this result. Details: local `regression.json`,
`host-baseline.log`, `host-portable.log`.

L4 portable tests: 18 pass across `caracal-evo` and `aardwolf-l4`; seven application
tests also pass with `bench-audio-ui`. Commands and full output: local
`checks.json`, `host-l4.log`, `host-l4-audio-ui.log`. The existing unused
`resolve_periodic_capture_delta` warning remains.

| Image | Flash bytes | SRAM1 statics | SRAM1 spare | SRAM2 stack |
| --- | ---: | ---: | ---: | ---: |
| Normal | 265520 | 92284 | 6020 | 32768 |
| Two-minute audio/UI bench | 267000 | 92284 | 6020 | 32768 |

The normal ELF SHA-256 is
`9e405871291858879b13c75846e7cb081027e5632f02717b0f9a8e937f12cf21`;
the bench ELF SHA-256 is
`5ae76efc48c2d177fb2b9186bd599a31e4ddadfa077afafda6ec262d02d65b7d`.
Local `continuous-build.json` and `bench-build.json` include exact source hashes,
binary hashes and paths. The finite bench's `verify.rs` differs from the final
source only by rustfmt splitting one argument line; token equality and both
source hashes are in `source-format-equivalence.json`. The frozen tested ELF
and original verifier remain under `target/l4-hardening/`. The normal image
was rebuilt from the formatted sources. Release builds use explicit `thumbv7em-none-eabihf`,
`--locked --offline`; each chip is built separately.

The linked initial stack pointer is `0x10008000`; the HardFault vector resolves
to the dedicated SRAM2 emergency-stack handler (`vector-check.json`). The
exception injection described below checks that path on hardware; it does
not simulate every form of memory corruption or stack overflow.

## Hardware checks

ST-Link V2, chip STM32L476RGTx, 100 kHz SWD, MCU UID
`3F001F001050525857323120`. The first reconnect enumerated USB but could not
identify the MCU; checking board power/SWD restored access. `audio-ui-01` records
that connection failure.

The next successful flash detected PC6 high and latched no-card fault code 1 at
50 ms, before storage/audio startup (`audio-ui-02`). Readback confirmed audio
stop set, recording disarmed, SD writes prohibited, both DMA channels disabled
and red GPIO toggling. TIM8 retained the expected note configuration. The user
confirmed **red flashing and the repeated descending error beep audible**, then
reseated the card. These observations are in `no-card-fault.json`; the failed
recording run is retained as such, not counted as a recording pass.

## Recording and telemetry

`audio-ui-04` passed at firmware time 147.277 s, after both writers closed and
a fresh filesystem reopened every byte. It used real GPS UTC, the normal
GPS/estimator policy, Sinc4/FOSR200/IOSR1, 16 kHz mono PCM32 and two simultaneous
streams. The full ten-minute GPS lock/sleep cycle was not repeated here.

| Check | Result |
| --- | --- |
| First WAV | `/1795680000/aud_1795681860.wav`, 3840512 bytes, 960000 samples, FNV32 `15b69bfb` |
| Second WAV | `/1795680000/aud_1795681920.wav`, 3840512 bytes, 960000 samples, FNV32 `9888194a` |
| Payload | Every low byte zero; all low significant PCM byte values represented; varying PCM, no clipping; full write/read hashes match |
| Raw PCM24 ranges | First 109074..122637, mean 116074; second 108400..122294, mean 115901 |
| AC mean power | 2887584 and 2892534 counts squared (about 1700 counts RMS); DC retained |
| Syslog | 22732 appended bytes after 904287 existing bytes; hash `af71f931`, 195 consecutive records, 138 heartbeats, two segment-closed markers, at least two CPU records |
| CPU | 12.5% first window including startup/wait, 15.4% full recording window; executor-idle measurement, not IRQ-inclusive |
| Errors | Zero DMA errors, dropped audio, syslog drops, truncations or write failures |
| Storage | Two simultaneous streams; thirteen syslog checkpoints while audio open; maximum append/mutex wait 52488 microseconds |
| Retention | Maximum conservative backlog 2048/4096 samples; copy/PCM processing at most 731 microseconds |
| Stack | High-water bound 28884/32768 bytes, 3884 unused including the intact 1 KiB guard |

The test tone started at published block 220: TIM8 PSC=1, ARR=39999 and
CCER=`0x50`, giving nominal 1 kHz PWM at 80 MHz. The first audio timestamp was
only 39 microseconds after the planned local start. Spectral analysis of the
0.2-second stimulus window retained all 24 source bits and centered a constant
DC offset for analysis only. Power at 1/2/3 kHz was 12871/232534/1995784 in the
detector's common scaled units; neighboring 0.8/1.2/2.2/3.2 kHz powers were
625/785/840/161. The largest harmonic exceeded the largest neighbor by 2375x;
the fundamental also exceeded its local neighbors by over 16x. This confirms
the acoustic stimulus was captured. It is not an acoustic sensitivity,
passband, SNR or subjective sound-quality calibration.

The earlier `audio-ui-03` recorded both minutes and verified the first WAV
hash but stopped on an experimental acoustic threshold. That initial detector
discarded eight more bits and retained DC; it reported a weak fundamental
against a high neighboring-bin value. It is retained as a failed run. The
revised diagnostic retains source precision, centers DC only for analysis,
reports harmonics/neighbors and does not impose an uncalibrated coupling-level
acceptance threshold. No DMA/packing expectation was relaxed. The final
measurement above also meets the original fundamental-versus-local-neighbor
ratio with the corrected analysis.

## Independent fatal backstop

After the successful finite bench, readback confirmed zero open streams, audio
stopped, SD writes prohibited and both DMA channels disabled. Only then, an
unused SRAM1 tail region held a copied interrupt-vector table. An invalid
TIM7 vector was software-pended to generate a real exception without changing
flash or SD files. The target escalated invalid Thumb state to HardFault:
CFSR=`0x00020000`, HFSR=`0x40000000`.

`hardfault-check.json` records code 4, the changed emergency-stack paint,
HSI16 selected (CFGR=`0x5`), PB14 switched to GPIO, both DMA channels disabled
and red GPIO toggling. The first immediate CODE read was zero; later register
reads showed the backstop active and a follow-up CODE read confirmed 4. The
initial observation is retained; debugger attachments can pause/resume target
execution. Reflashing/reset restores the normal vector table and stack paint.

The monitor now uses a hidden console and a console-interrupt stop before any
forced-termination fallback. A local signal test and successful ST-Link reuse
after the finite bench verified the new shutdown path. Older abrupt shutdowns
had wedged the Windows USB endpoint and required reconnects. Reset/HardFault
debugger catching is disabled during target indication tests.

## Live removal and normal image

The final normal image was flashed and verified (`removal-01`). It recorded
`/1795680000/aud_1795682340.wav` to completion, 3840512 bytes, hash `ed4fc867`,
then opened the next minute. Physical removal at firmware time 128.372 s latched
code 1; the recorder refused further I/O at 128.385 s. GPS/time processing
continued during the observed error-repeat interval. The user confirmed both
**red flashing and the repeated descending error beep audible**.

`live-removal.json` confirms audio stop=1, armed=0, SD write permission=0,
DMA1_CH4 CR=`0xa`, DMA2_CH5 CR=`0x3a8a` (both EN bits clear) and red GPIO
toggling. After physical reinsertion, PC6 read low but code 1 remained latched
(`reinsertion-latch.json`); reinsertion alone did not restart writes.

Reset then mounted the same card and appended syslog after its interrupted
tail, with real GPS UTC available at 1.869 s (`continuous-01`). No formatting,
deletion, filesystem repair, receiver reset or EN pulse was used. The active
file interrupted by removal is not claimed complete or durable.

The final normal-image check passed two rotations:

- `/1795680000/aud_1795682580.wav`: 3840512 bytes / 960000 samples,
  hash `fda9f835`, closed at firmware time 83.563 s.
- `/1795680000/aud_1795682640.wav`: 3840512 bytes / 960000 samples,
  hash `caea3d73`, closed at 143.568 s.
- `/1795680000/aud_1795682700.wav` opened at 143.580 s and was recording when
  the monitor disconnected. No further target attachment or reset was made.

The latest full CPU window was 15.4% executor load, with zero log drops,
truncations or write failures. Conservative backlog stayed at 1536/4096 samples,
maximum append time was 17108 microseconds, and stack high-water was
28524/32768 bytes with 4244 unused and the guard intact. The normal files were
monitored through closure; the complete stopped-writer readback belongs to the
finite bench above. **The normal image is left recording for the user's
listening test, with no generated acoustic test tone.**

`regression-final.json` rechecks all 66 unchanged U5 images, eleven feature trees
and 212 preserved earlier L4 files after the target work. `final-checks.json`
checks image/source identities, the declared whitespace-only bench difference,
rustfmt, document links and whitespace errors. Local `firmware.zip` and its
SHA-256 inventory preserve the exact accepted ELF/BIN images, source overlay,
the bench's original verifier and local evidence. The archive is an overlay
on the recorded base commit, not a complete repository snapshot.

The analytic filter response and the 48.16 dB PCM scaling correction are recorded
in `filter-model.json`. They are calculations, not measured frequency response.
The final assessment of sound quality remains the user's next listening test.
