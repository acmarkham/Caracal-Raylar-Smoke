# Evo L4 GPS smoke

Implements [ADR-L4-008](../../SpecAndDesign/Platform/L4/ADR-L4-008-gps-uart-fix-pps.md)
for the fitted Quescan G10A-F30 / u-blox M10. USART2 TX=PA2, RX=PA3; PPS=PB5.
PB4 enable stays input/no pull, so the module must already be powered/enabled.
The SD card is unused.

```text
rtk cargo build -p unit-smoke-l4-03-gps --target thumbv7em-none-eabihf --release --locked --config unitsmokeL4/03_gps/defmt.toml
rtk probe-rs run --chip STM32L476RGTx --probe 0483:3748 --speed 100 --verify --preverify --non-interactive target/thumbv7em-none-eabihf/release/unit-smoke-l4-03-gps
```

The image listens at common baud rates, then polls UBX-MON-VER, UBX-NAV-PVT
and ASCII PUBX position/time. It sends no reset, baud-rate or saved-configuration
commands. Keep the receiver powered with sky view and allow up to 15 minutes.
A silent UART is reported separately from slow satellite acquisition.
The explicit logging configuration suppresses per-byte HAL warnings during
wrong-baud listening; the leaf IRQ counts UART errors instead. It leaves the
root U5 logging configuration unchanged. Require zero errors after baud lock.

Look for `L4 GPS UART PASS`, `MON-VER` strings, qualified `NAV-PVT`/GGA/RMC,
and `PPS` edges. Final `L4 GPS PASS` requires identity, both PUBX responses,
three qualified distinct fix epochs and twenty consecutive one-second PPS
intervals. Receiver UTC seconds must agree across the current fix messages.
Progress continues every ten seconds after PASS. Blue toggles each PPS edge
(one complete LED cycle per two seconds); red and green stay off.

PPS measurements use EXTI task timestamps. They verify the signal and its rate,
not precise UTC alignment or clock calibration. First-fix elapsed time starts
with the firmware, not with receiver power-on. See
[evidence](evidence/README.md) for actual outcomes and remaining limitations.

Run the allocation-free protocol tests on a native host:

```text
rtk cargo test -p unit-smoke-l4-03-gps --no-default-features --lib --target x86_64-pc-windows-msvc --locked
```

Keep U5 and L4 builds in separate Cargo invocations. The root default stays U5.
Use `--connect-under-reset` if target attach needs it. A WinUSB open failure
requires fixing the probe connection before it can produce receiver evidence.
