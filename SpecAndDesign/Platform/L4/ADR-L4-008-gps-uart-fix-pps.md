# ADR-L4-008: Evo u-blox M10 UART, fixes and PPS bring-up

## Status

Accepted for implementation, 2026-11-24. Target:
[`unitsmokeL4/03_gps`](../../../unitsmokeL4/03_gps/README.md).
Actual receiver identity and hardware outcomes belong in its
[`evidence`](../../../unitsmokeL4/03_gps/evidence/README.md) record. A successful
build alone does not establish UART, navigation or PPS support.

## Context

The connected receiver is identified by the operator as Quescan G10A-F30,
using u-blox M10. This differs from the U5 receiver and its PMTK control profile.
The module may start without retained satellite data. Preserve its ongoing
acquisition while checking the digital interfaces; do not issue a cold reset to
manufacture a time-to-first-fix measurement.

[L4-005](ADR-L4-005-schematic-and-legacy-reference.md) supplies the connector
mapping. The schematic remains pin evidence, not proof of an unchanged harness
or the enable circuit's polarity. [L4-006](ADR-L4-006-first-led-bring-up.md)
establishes the clock and flash chain. SD access is unnecessary here; leave the
inserted card untouched.

## Decision

Add one explicit L4 leaf, with native tests for its bounded protocol decoder.
Keep the root U5 defaults, chip features, source, fixtures and acceptance tests
unchanged. Build the two architectures in separate Cargo invocations. Reuse the
upstream portable `NmeaParser` through `raylar-drivers` without its `stm32`
feature. Keep the diagnostic's u-blox mixed-stream framing local until a later
ADR integrates receiver profiles into the shared GPS manager. Do not import the
U5 backend's pins, timer or PMTK commands.

### Resources and control

| Resource | Selection |
| --- | --- |
| Core / monotonic | HSE 16 MHz, main PLL M=2/N=20/R=2; core/APB buses 80 MHz; Embassy TIM5, 1 MHz. No LSE assumption. |
| MCU transmit | PA2, USART2_TX AF7, to receiver RX; schematic net `GPS_RX`, SV1 pin 3. |
| MCU receive | PA3, USART2_RX AF7, from receiver TX; schematic net `GPS_TX`, SV1 pin 4. |
| UART | 8N1, no flow control; USART2 interrupt buffering, 4096-byte RX and 256-byte TX rings. No DMA allocation. |
| PPS | PB5 / SV1 pin 6, rising-edge EXTI5, EXTI9_5 handler, weak pulldown. Separate Embassy task. |
| Enable | PB4 / SV1 pin 1 stays input/no pull. Observe its level, do not drive or cycle an unverified module/harness control. |
| Supply | Existing board/harness supply, SV1 pin 5 VDD and pin 2 GND. No invented reset GPIO. |
| LEDs | Active-low PB11 red and PC3 green off. PC1 blue toggles on each observed PPS edge; its full on/off cycle takes two PPS periods. |
| Memory | Normal SRAM1 statics and CPU stack; link-time minimum 16 KiB gap after statics. SRAM2 unused. No heap allocation. |

The portable driver crate contains allocating storage code, even when this
image only calls its allocation-free NMEA parser. A rejecting global allocator
turns any unexpected heap call into an explicit diagnostic failure. No SD,
filesystem, radio, microphone or Time Service tasks run in this image.

### Identify the receiver without reconfiguring it

Listen at 9600, 38400, 115200, 19200, 57600, 4800 and 230400 baud. Require at
least two checksum-valid frames before accepting a rate; activity or readable
fragments alone are insufficient. The MCU changes only its own UART rate.
Report a clear interface failure if no valid stream is found, separately from
the longer navigation acquisition deadline.

Build with the leaf's explicit `--config unitsmokeL4/03_gps/defmt.toml`.
It suppresses HAL USART warnings for intentional wrong-baud traffic: printing
each error inside the IRQ can saturate RTT and interfere with timing. The leaf
IRQ wrapper counts error interrupts before delegating to Embassy; report scan
errors separately, reset the count at baud lock, and require zero subsequent
error IRQs at PASS. This configuration does not change U5 logging defaults.

Use these read-only polls after detecting the rate:

- Binary UBX-MON-VER for software/hardware strings and all available extension
  fields, including firmware/protocol and module identifiers when supplied.
- Binary UBX-NAV-PVT every two seconds for navigation and receiver UTC validity.
- ASCII `$PUBX,00` and `$PUBX,04` polls every ten seconds to exercise proprietary
  position and time responses alongside standard NMEA.

`$PUBX` is u-blox's proprietary ASCII NMEA framing; **UBX is the separate binary
protocol**. Log the receiver's actual returned identity. The Quescan product
label is operator evidence and need not appear in MON-VER. Missing module
extensions are not grounds for inventing a model string.

Use the matching u-blox interface description once MON-VER reveals the protocol
version. The observed SPG 5.10 / protocol 34.10 matches
[u-blox UBX-21035062 R03](https://content.u-blox.com/sites/default/files/u-blox-M10-SPG-5.10_InterfaceDescription_UBX-21035062.pdf),
including its MON-VER, NAV-PVT and PUBX definitions.
Validate complete frames before decoding their fields. Bound the mixed-stream
buffer to 768 bytes; reject oversize/bad-checksum packets and expire truncated
binary frames after a UART gap. A `$` inside a binary payload is ordinary data.

Do not send PMTK, CFG-RST, power-management commands, baud/protocol changes or
persistent configuration writes. A later receiver profile must explicitly
define supported commands and power semantics for each fitted receiver.

### Fix and PPS qualification

Keep the receiver active continuously during this diagnostic. Allow up to 900
seconds from UART initialization for the combined acceptance result. Emit
progress every ten seconds and continue monitoring after PASS. An expired
deadline fails visibly; it does not reset or power-cycle the receiver. Record
first qualified NAV-PVT elapsed time since firmware start as an observation,
not a measured cold-start TTFF or time since receiver power-on.

The combined `L4 GPS PASS` requires all of:

1. A decoded, checksum-valid MON-VER response and checksum-valid responses to
   both PUBX polls, proving both UART directions.
2. At least three distinct qualified NAV-PVT epochs: 3D fix, `gnssFixOK`, valid
   date/time and fully resolved UTC flags, at least four satellites, valid
   latitude/longitude and no invalid-LLH flag.
3. At least three accepted GGA epochs under the shared location policy's
   satellite/HDOP limits, plus active RMC. The current GGA, RMC and NAV-PVT must
   agree on receiver UTC second and each be younger than three seconds.
   Invalid reports clear the corresponding admission state; gaps break streaks.
4. Twenty consecutive PB5 rising-edge intervals within 990,000..1,010,000
   microseconds, with the last edge younger than 1.5 seconds. Report counts,
   discontinuities and minimum/maximum software intervals.

EXTI task-wakeup timestamps establish signal continuity and an approximately
one-second period against the HSE-derived monotonic clock. They do not establish
pulse width, voltage, absolute UTC alignment, oscillator accuracy, sawtooth
correction or precise serial-to-PPS association. PPS may exist before a valid
fix. Never promote a UART arrival timestamp or a raw PPS to a Time Service UTC
anchor. Hardware TIM3_CH2 capture on this 16-bit timer, rollover handling and
shared Time Service calibration remain later integration work.

### Evidence and regression

Preserve the exact flashed ELF/BIN, source/build identity, lockfile, feature
graph, generated memory use, tool versions and complete RTT logs. Keep failed
attempts with their reason, and separate USB/debug transport failures from
receiver failures. Capture a stable interval after PASS and leave a runnable
diagnostic image on the board. An MCU debug reset does not establish a cold
receiver start.

Native tests cover known wire checksums, interleaved ASCII/binary traffic,
corruption/oversize/truncation recovery, version-field bounds, navigation
validity admission and PPS streak loss/staleness. Existing U5 firmware packages
build individually; GPS/time/application feature graphs must match their
pre-change values. Existing host tests retain their exact expectations and
known failures. No U5 hardware result is claimed from the connected L4 board.

## Consequences

This qualifies one fitted receiver/harness at its observed UART configuration.
It does not establish compatibility with every M6/M8/M10 module or implement
L4 GPS duty cycling, standby/wake, reset recovery, Location Service filtering,
Time Service discipline or GPS-calibrated recording. Keep the initial enable
and electrical qualification gaps explicit in the evidence record.

## Policy overrides

The shared GPS, location, time and UI policies apply to included behavior.
There is no Location Service estimate or UTC authority in this image; all
reported fix/UTC data is explicitly receiver diagnostic data. Logging and
Versioning services are absent; the RTT/build record supplies lab provenance.

```yaml
policy_overrides:
  - policy: gps
    key: duty_cycle
    value: {minimum_on_s: 900, standby_s: 0}
    reason: Continuous bench acquisition and observation; no unverified receiver power commands.
  - policy: gps
    key: initial_calibration.driver_only_duration_s
    value: 900
    reason: Allow the full first-search window for this cold-start diagnostic.
  - policy: gps
    key: search_timeout
    value: Report failure through RTT and halt the diagnostic without driving receiver power or reset.
    reason: Power and standby semantics of this harness are not yet qualified.
  - policy: ui
    key: startup.enabled
    value: false
    reason: The GPS smoke does not initialize the buzzer.
  - policy: ui
    key: errors.enabled
    value: false
    reason: Diagnostic failures use panic-probe/RTT rather than application fault sounds or LEDs.
  - policy: ui
    key: gps_green
    value:
      led: SysGpsGreen
      inactive: Off.
      searching: Off.
      tracking: Off.
      pulse_ms: 50
      trigger: None; Time Service is absent.
      rejected_pps: No green pulse.
      refresh: Keep green off; blue toggles on raw PB5 edges as an explicitly diagnostic indication.
    reason: Raw PPS testing must not imply Time Service acceptance or calibrated UTC.
```
