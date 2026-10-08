---

# Battery Charger Driver (BQ25186)

## Constraints
Read
ADR\common\AGENTS.md
before planning and implementing


---

# Context

The platform uses a TI BQ25186 battery charger and power-path management IC.

The charger exposes a register-based I²C interface.

Higher-level services should not require knowledge of registers, bit fields or device-specific implementation details.

Instead they should interact with charger concepts.

---

# Decision

Implement a BQ25186 Driver that translates register-level communication into charger-specific state and operations.

The driver owns the I²C interface to the charger.

---

# Responsibilities

The driver shall:

* Configure the charger
* Read charger status
* Read charger faults
* Enable and disable charging
* Configure charge parameters
* Publish charger state

---

# Non-Goals

The driver is **not** responsible for:

* Deciding whether charging should occur
* Battery state-of-charge estimation
* Power budgeting
* Load shedding
* Sleep decisions

These belong to the Power Service.

---

# Public State

```rust
pub enum ChargeState {
    NotCharging,
    TrickleCharge,
    PreCharge,
    FastCharge,
    TopOff,
    ChargeComplete,
}

pub enum ChargeFault {
    Thermal,
    InputFault,
    SafetyTimer,
    BatteryFault,
    ChargerFault,
}

pub struct ChargerState {
    pub charging: bool,
    pub state: ChargeState,
    pub fault: Option<ChargeFault>,
}
```

Published using:

```rust
Watch<ChargerState>
```

---

# Public API

Conceptually:

```rust
enable()

disable()

set_charge_current()

set_input_current_limit()

watch()
```

The API should describe charger behaviour rather than register operations.

---

# Hardware Knowledge

The driver owns:

* Register map
* Bit fields
* Device reset
* Device initialisation
* Register encoding

The remainder of the application should remain independent of the BQ25186 implementation.

---

# Consequences

Future charger ICs can be adopted with minimal impact to higher-level services.

---
# Prototype

unitsmoke/24_batt_charger has implemented some of the routines for charging/monitoring batteries.

In particular, it is important on initialization to set the default charging rate e.g. to 200mA (or as the user requires) or it defaults to 10mA which is too slow and is also very risky for long term field deployments if it doesn't really charge.

# Implementation

Implement the driver under 'crates/drivers/batterycharger/'

# Tests

Make a thin test under 'drivertests/batterycharger/'

