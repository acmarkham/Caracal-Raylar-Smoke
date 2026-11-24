# ADR-L4-006: First Evo L4 bring-up with HSE, Embassy time and LED GPIO

## Status

Accepted for implementation, 2026-11-24. Hardware results are recorded separately
in [the smoke target evidence](../../../unitsmokeL4/01_helloworld/evidence/README.md).
This qualifies the small LED image only, not the later recorder milestones.

## Context

[L4-001](ADR-L4-001-hardware-baseline.md) and
[L4-005](ADR-L4-005-schematic-and-legacy-reference.md) identify the L476RGT6,
16 MHz HSE crystal and three active-low LEDs. The first executable should prove
compilation/linking, SWD programming, external oscillator startup, clock selection,
Embassy scheduling and one GPIO output before introducing storage or acquisition.

The current root HAL dependency still selects the U5. Inheriting it in an L4
package would enable both chips. A workspace-wide U5 build already fails before
this change because Aardwolf's executor `trace` feature is unified into firmware
that does not supply the trace hooks. Package-specific builds are the baseline;
this change does not repair or redefine those existing U5 configurations.

## Decision

### Package and build boundary

Implement `unit-smoke-l4-01-helloworld` in `unitsmokeL4/01_helloworld`, in the
existing workspace and using its lockfile, release profile and common dependency
versions. Preserve all existing U5 source, HAL features, runners and test inputs.

For this isolated image, select `embassy-stm32` 0.6.0 directly in the leaf
manifest with `stm32l476rg`, `memory-x`, `time-driver-tim5`, `defmt` and
`unstable-pac`. Do not inherit the U5-specific workspace HAL declaration or depend
on `raylar-board-v1p0`/`raylar-drivers`. Other dependencies inherit the existing
workspace definitions. Build each platform in a separate Cargo invocation.

This is a scoped first-bring-up adjustment to the ordering in L4-002/L4-004:
defer the broad removal of U5 chip features until shared drivers are needed.
It avoids editing every U5 manifest merely to test three GPIOs. It is not a
second workspace or service implementation. Later extraction must consolidate
the neutral HAL version/settings and retain the same U5 feature graphs.

Add the L4 package as a workspace member, and explicitly retain the original
U5 packages as the workspace default members. Pattern expansion must select
exactly the original member set. Thus an ordinary existing root build does not
start combining chips. `--workspace` remains unsuitable for dual-chip builds.
Track the shared root lockfile so the demonstrated dependency resolution is
available in a clean checkout.

Use the explicit `thumbv7em-none-eabihf` target. The package build script rejects
other target triples, including the repository's default U5 target, with an
actionable message. No host tests or other firmware graph activates the L4 HAL.
Use the pinned HAL-generated flash/SRAM1 linker layout: 1 MiB flash at
`0x08000000` and 96 KiB SRAM1 at `0x20000000`. SRAM2 remains unused. Link with
`--nmagic`, `link.x` and `defmt.x`; inspect the resulting ELF load segments.

### Clock plan

| Item | Selection |
| --- | --- |
| HSE | Fitted 16 MHz crystal, oscillator mode; external-clock bypass disabled |
| Main PLL | HSE / 2 x 20 / 2 = 80 MHz PLL R output; VCO = 160 MHz |
| SYSCLK / AHB / APB1 / APB2 | PLL R / DIV1 / DIV1 / DIV1, all 80 MHz |
| Core supply | L4 reset Range 1; verify VOS readback. No U5 SMPS or fractional-PLL writes. |
| Flash | Embassy selects four wait states for 80 MHz; verify readback. |
| Embassy monotonic timer | TIM5, 1 MHz ticks; prescaler 79 from 80 MHz |
| Low-speed clock domain | Disabled for this smoke image; no fitted LSE assumption or RTC requirement |
| Audio / USB PLL outputs | Unused and disabled |

Keep MSI available as Embassy's startup/switching clock, but require the final
SYSCLK status to report PLL and the PLL source to report HSE. A successful MSI
fallback would not pass this test. The pinned Embassy oscillator wait is not
bounded; an absent/faulty HSE can stall initialization. Bound the host observation
window and use SWD/under-reset recovery instead of claiming a target PASS.

After HAL initialization, check the actual RCC ready/source/divider bits,
reported HAL clock frequencies, PWR Range 1, flash latency and TIM5 prescaler.
Print raw RCC registers for independent inspection. These establish the selected
clock path and nominal rates; they do not measure crystal ppm accuracy. A host
wall-clock comparison of RTT edge intervals is a coarse independent cadence
check. A calibrated scope/frequency counter is needed for oscillator accuracy.

### GPIO and behavior

| Pin | Function | Initial output | Test behavior |
| --- | --- | --- | --- |
| PB11 | Red LED | High/off | Remains off |
| PC3 | Green LED | High/off | Remains off; no GPS indication is implied |
| PC1 | Blue LED | High/off | Low/on and high/off alternate every 500 ms |

Create push-pull GPIO outputs with the off level supplied to the constructor,
at low GPIO speed. Leave the buzzer, expansion, SD, GNSS and microphone pins
unclaimed. Preserve SWD pins. The blue LED has a one-second period and 50% duty
cycle; one Embassy task owns all three outputs.

Use an async ticker, not busy-wait delays. At each edge read back the blue output
latch and digital input pad, and verify the unused LEDs stay high. Count edges
and report their monotonic spacing. Require 490,000..510,000 microseconds per
edge and at least 20 verified edges before emitting `L4 HELLOWORLD PASS`.
Continue blinking and checking after PASS so the board remains useful without
a debugger. RTT defaults to nonblocking when no host is attached.

Read DBGMCU device ID and the flash-size signature to reject a mismatched part:
the expected L47x/L48x device ID is `0x415` and flash size is 1024 KiB. Log the
raw revision/device value rather than inferring exact package marking from it.

Assertions fail through `panic-probe`/RTT; a failed clock or GPIO check must not
produce PASS. GPIO latch/pad readback verifies electrical logic level at the MCU,
not LED current, color, optical output or the full physical board revision.
Human observation or an instrument remains the optical check.

### Flash and observe

From the repository root, build using the command in the target
[README](../../../unitsmokeL4/01_helloworld/README.md), then use `probe-rs run`
with the exact L476RG target and `--verify`. Do not use the U5 firmware script
with only its `-Chip` argument changed. Prefer a conservative 100 kHz SWD request
for the connected ST-Link V2; the probe may round to its supported speed.
If attaching fails, retry with `--connect-under-reset` and record that choice.

Capture bounded RTT output, executable SHA-256, tool versions, source identity,
lockfile hash, selected HAL features, ELF memory sizes and probe/device identity.
Confirm at least 20 edges over approximately ten host seconds, with no assertion
failure or evidence loss. Verify reset/restart, then leave the board running the
LED image. Do not change option bytes or mass-erase unrelated flash.

For the reset/restart check, attach before execution using another
`probe-rs run --preverify --verify` session. Attaching to an already-running
timing test can halt the CPU while TIM5 continues counting, disturbing overflow
accounting. Such a run is debugger-perturbed evidence, not a valid cadence pass;
do not relax the interval assertion to accommodate it. Finish with a standalone
reset and leave the firmware running without another live attach.

## Acceptance and U5 regression

1. The L4 leaf graph contains one chip (`stm32l476rg`) and one time driver
   (`time-driver-tim5`); it has no U5 board/drivers or TrustZone selection.
2. The linked image fits the documented flash/SRAM1 layout. The wrong Rust
   target fails before linking. Build-only operation requires no probe or
   platform-specific shell.
3. Programming verifies; runtime identifies the expected device/flash and
   passes RCC/PWR/flash/TIM5 checks, both blue GPIO levels and the edge window.
4. The host cadence check and a reset/restart run pass. Optical observation is
   reported independently and never inferred from RTT.
5. Existing U5 default member selection, package feature graphs, release profile,
   source/test inputs and package-specific build results remain the same.
   Run the existing portable regression suite with its host settings; no golden
   data or assertions change. Record existing failures separately. No U5 board
   is assumed connected, so U5 hardware regression remains pending where needed.

This small image does not complete the full L4-004 migration baseline or prove
Linux/macOS builds, SRAM2, low-power operation, USB, ADC, GNSS, SD or audio.

## Consequences and alternatives

The only platform-specific Rust here is a short smoke executable with explicit
clock/pin ownership and readback. Promote verified board setup into the future
Evo board crate when another L4 test needs it. A separate workspace/lockfile,
copying Mbed startup and globally changing U5 HAL defaults are unnecessary for
this first result.

## Policy overrides

The [UI policy](../../Policy/ui.yaml) applies to included LEDs. This diagnostic
has no buzzer initialization and reports failures over RTT:

```yaml
policy_overrides:
  - policy: ui
    key: startup.enabled
    value: false
    reason: This GPIO/clock smoke image intentionally does not initialize the buzzer.
  - policy: ui
    key: errors.enabled
    value: false
    reason: Assertions halt through panic-probe with RTT evidence; audible and latched application fault indications are outside this first smoke image.
```
