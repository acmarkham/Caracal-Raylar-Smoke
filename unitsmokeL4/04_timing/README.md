# Evo L4 chained capture and UTC discipline

Implements [ADR-L4-009](../../SpecAndDesign/Platform/L4/ADR-L4-009-chained-capture-and-time-discipline.md).
TIM3_CH2 captures PB5 at 1 MHz. TIM3 update TRGO clocks TIM2 through ITR2,
extending the free-running counter to 48 bits without overflow interrupts.
TIM5 remains the Embassy monotonic clock.

```text
rtk cargo build -p unit-smoke-l4-04-timing --target thumbv7em-none-eabihf --release --locked --config unitsmokeL4/04_timing/defmt.toml
rtk probe-rs run --chip STM32L476RGTx --probe 0483:3748 --speed 100 --verify --preverify --non-interactive target/thumbv7em-none-eabihf/release/unit-smoke-l4-04-timing
```

Keep the previously verified Quescan M10 connected and powered, at 38400 baud.
PB4 remains input/no pull; the image does not reset or reconfigure the receiver.
The SD card is unused. Use attached reset runs, not a live mid-test attach.

Look for carry self-tests and `L4 TIMING CHAIN PASS`, then MON-VER and hardware
PPS/UTC anchor diagnostics. The unchanged upstream Time Service needs eleven
samples at least sixty local seconds apart: allow roughly eleven minutes for
calibration, then one minute of holdover and about twenty seconds for anchor
resumption. The total deadline is thirty minutes to allow initial acquisition.
Only final `L4 TIMING PASS` means all three real-GPS phases passed. Logs continue
afterward. Green pulses for accepted Time Service GPS anchors; red/blue stay off.

PPS timestamps come from the latched low word plus a resolved high word and a
bounded cross-clock epoch. The **capture IRQ must be serviced within 65.536 ms**;
this is not an atomic 48-bit latch under arbitrary interrupt blackouts. The
highest-priority ISR does no logging; normal rollover needs no ISR at all.
The deliberate holdover keeps GPS running and withholds anchors from the
service, allowing an independent comparison against its corrected local time.
PPS sequence/cadence remain continuous, so the upstream service can resume UTC
labels without invoking its physical PPS-loss discard gate. The diagnostic
phase `reacquiring` names this anchor resumption, not a receiver restart test.

```text
rtk cargo test -p unit-smoke-l4-04-timing --no-default-features --lib --target x86_64-pc-windows-msvc --locked
```

[Evidence](evidence/README.md) records actual results, memory use and retained
firmware. Keep U5/L4 builds in separate Cargo invocations; root defaults stay U5.
