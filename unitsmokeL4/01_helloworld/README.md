# Evo L4 first LED smoke test

Implements [ADR-L4-006](../../SpecAndDesign/Platform/L4/ADR-L4-006-first-led-bring-up.md):
16 MHz HSE crystal, 80 MHz PLL system clock, Embassy TIM5 at 1 MHz, and the
active-low blue LED on PC1 at 1 Hz. PB11/red and PC3/green stay off. Clock and
GPIO readback checks produce `L4 HELLOWORLD PASS` after 20 edges; blinking
continues afterwards. No SD, GPS, microphone or buzzer is initialized.

Run these commands from the repository root. `rtk` is the repository command
wrapper; the underlying Cargo/probe-rs commands also work independently.

```text
rtk rustup target add thumbv7em-none-eabihf
rtk cargo build -p unit-smoke-l4-01-helloworld --target thumbv7em-none-eabihf --release --locked
rtk probe-rs list
rtk probe-rs run --chip STM32L476RGTx --probe 0483:3748 --protocol swd --speed 100 --verify --non-interactive target/thumbv7em-none-eabihf/release/unit-smoke-l4-01-helloworld
```

Select a probe serial as well if multiple identical probes are connected.
The 100 kHz SWD request favors the connected ST-Link V2/wiring. Add
`--connect-under-reset` if attaching fails. Initial programming can take longer
than the ten-second observation interval. Stop the host monitor after PASS;
reset/resume the target if the debugger leaves it halted. Never enable mass
erase or change option bytes for this test.

For a repeat after reset, use the same `probe-rs run` command with `--preverify`:
it checks the existing image and starts a new attached run. Avoid attaching to
an already-running instance for cadence acceptance: a debug halt can disturb
the running timer's overflow accounting and legitimately trip its strict timing
assertion. To leave a standalone blink running afterwards:

```text
rtk probe-rs reset --chip STM32L476RGTx --probe 0483:3748 --speed 100 --non-interactive
```

Always specify the L4 Rust target. The root default and firmware automation
remain U5-specific. Build one platform per invocation; a combined workspace
build unifies incompatible HAL chip features. The original U5 default members
are preserved explicitly. This smoke image uses the same workspace/lockfile
and shared Embassy versions; it selects the L4 HAL directly to avoid inheriting
the existing U5 chip selection.

Inspect the `L4 clocks`, `L4 CLOCK PASS`, `L4 LED` and final PASS records. Both
the output latch and physical digital pad must match each commanded level.
Compare several seconds of host wall time against the target edge stream, and
observe the blue LED if physically accessible. Register readback and RTT do not
prove optical output or calibrated crystal accuracy.

[Recorded validation](evidence/README.md) distinguishes checks performed on this
machine/board from pending physical or cross-host checks.
