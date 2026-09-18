# ADR-0014: STM32 Core Power-Supply Driver

## Status

Accepted

---

# Context

The STM32U595 can generate VCORE with its internal linear regulator (LDO) or
its internal switched-mode power supply (SMPS). SMPS reduces conversion loss,
which is desirable for the battery-powered Raylar recorder.

SMPS is not universally safe to select. It requires an MCU package that exposes
the SMPS pins and the matching external power components. Raylar v1.0 uses the
STM32U595VJT6Q and fits the inductor between `VLXSMPS` and `VDDSMPS`, so this
board supports SMPS. LDO remains the safe portable default for the driver
library and for boards without that hardware.

The current `embassy-stm32` 0.6 STM32U5 RCC configuration has no core-regulator
field. It also resets the PWR peripheral during `embassy_stm32::init`, so a
selection made before that call would be lost.

---

# Decision

Add a small `stm32_core` driver to `raylar-drivers`. Its public API represents
the regulator as:

```rust
pub enum CoreSupply {
    Ldo,
    Smps,
}
```

`CoreConfig` selects one of those modes and `CoreSupplyControl` provides the
portable contract. The STM32 backend maps the modes to the STM32 core PWR
driver values:

| Driver mode | STM32 PWR `REGSEL` value |
| --- | --- |
| `CoreSupply::Ldo` | `Regsel::LDO` |
| `CoreSupply::Smps` | `Regsel::SMPS` |

After writing `PWR_CR3.REGSEL`, the backend reads `PWR_SVMSR.REGS` until the
requested regulator is active. The wait is bounded and reports a transition
timeout instead of hanging indefinitely on incompatible or faulty hardware.

The driver is synchronous, heapless, and configured once during startup. It is
not part of the Power Management Service: that service decides operating policy
while core-regulator selection is a hardware/platform concern.

---

# Startup Ordering

Applications shall initialize in this order:

1. call `embassy_stm32::init` to configure clocks and reset/init PWR;
2. initialize `Stm32CoreDriver` with the board-selected core supply;
3. construct the board and start peripheral drivers and services.

This ordering prevents Embassy's PWR reset from reverting the selection and
confirms the regulator transition before peripherals add load.

---

# Integration Test 002

Integration test 002 enables its `core-smps` Cargo feature by default. This
matches the STM32U595 Q-package and fitted inductor on Raylar v1.0. Building
the test with `--no-default-features` selects LDO, allowing comparative power
measurements and recovery diagnostics without changing source code.

The selected supply is emitted over RTT at startup for traceability.

---

# Safety Constraints

* SMPS may be selected only on an SMPS-capable STM32 package and board.
* Package capability is a board-level fact; it cannot be inferred reliably
  from the common STM32U595 peripheral register layout.
* The driver library defaults to LDO. Each board/application must opt in to
  SMPS deliberately.
* Regulator switching is startup configuration. Services must not switch it
  dynamically during normal operation.
* Backup-domain options that can only be written while LDO is active must be
  configured before selecting SMPS if they are added later.

---

# Testing

Host unit tests verify the portable configuration default. Cross-compilation
verifies the STM32 typed-register mapping. Integration test 002 validates the
SMPS path on Raylar v1.0 hardware; its startup trace must report `Smps` before
normal service initialization.

---

# Consequences

## Advantages

* Raylar v1.0 uses the more efficient regulator by default.
* LDO remains available through the same API and as a diagnostic build option.
* STM32 register details remain isolated from application and service code.
* Read-back confirmation catches unsupported hardware or failed transitions.
* The design can map to a future higher-level STM32 core driver without
  changing callers.

## Disadvantages

* The application must respect a specific initialization order.
* Hardware capability remains a board declaration rather than runtime
  discovery.
* Selecting SMPS on a board without the required external circuit is unsafe;
  the type system cannot prevent a wrong board configuration.
