---

# ADR-0006: Power Management Service


---

## Constraints
Read
ADR\common\AGENTS.md
before planning and implementing

# Context

The system contains multiple hardware components related to power management:

* Voltage Monitor Driver
* Battery Charger Driver

These expose observations about the hardware but deliberately avoid implementing system policy.

Many application services require higher-level information such as:

* Is external power available?
* Should GPS remain enabled?
* Is the battery critically low?
* Is the system charging?
* Should recording continue?

A dedicated Power Management Service should combine these hardware observations into a coherent model of system power.

---

# Decision

Introduce a Power Management Service.

The service subscribes to the Voltage Monitor and Charger Driver watches and publishes a unified view of platform power.

It also becomes the natural home for future power policy.

---

# Architecture

```text
           Voltage Monitor
                │
                ▼
         Watch<VoltageState>

           Charger Driver
                │
                ▼
         Watch<ChargerState>

                │
                ▼
      Power Management Service

                │
                ▼
        Watch<PowerState>
```

The service owns no hardware.

It consumes observations and produces decisions.

---

# Responsibilities

The Power Management Service shall:

* Determine the current power source
* Monitor battery voltage
* Determine charging status
* Estimate battery state-of-charge
* Publish unified power state
* Maintain battery history if required
* Become the central location for future power policy

---

# Non-Goals

The service is **not** responsible for:

* ADC configuration
* Charger register programming
* I²C communication

These remain within the underlying drivers.

---

# Public State

Suggested structure:

```rust
pub enum PowerSource {
    Battery,
    Solar,
    ExternalDc,
    Usb,
    Unknown,
}

pub enum BatteryHealth {
    Normal,
    Low,
    Critical,
}

pub struct PowerState {
    pub source: PowerSource,

    pub battery_mv: u16,
    pub solar_mv: u16,
    pub ext_dc_mv: u16,

    pub charging: bool,

    pub battery_percent: Option<u8>,

    pub health: BatteryHealth,

    pub charger: ChargerState,
}
```

Published using:

```rust
Watch<PowerState>
```

---

# Policy

The Power Management Service owns platform power policy.

Examples include:

* Low battery thresholds
* Critical battery thresholds
* Battery percentage estimation
* Charge enable/disable decisions
* Future thermal derating
* Future power budgeting
* Future load shedding

Hardware drivers remain policy-free.

---

# Battery Percentage

The battery percentage estimator should be encapsulated within the service.

Initially this may use a simple lookup table derived from battery voltage.

Future versions may incorporate:

* Load compensation
* Temperature compensation
* Coulomb counting
* Charge history

The public API should remain unchanged.

---

# Future Extensions

The architecture should naturally accommodate:

* Low-power operating modes
* GPS duty-cycle adjustment
* Sensor throttling
* Audio recording policy
* Solar-aware charging strategies
* System suspend recommendations
* Power consumption statistics

without requiring changes to the underlying drivers. These are extensions however, and should not overly dictate the design of the current driver - prefer conciseness over excessive abstraction.

---

# Consequences

Separating hardware drivers from power policy produces a clean layering:

* Hardware drivers expose calibrated physical observations.
* The Power Management Service models the platform's energy state.
* Application services consume a single, coherent view of system power.

This mirrors the architecture already established elsewhere in the firmware, where drivers own hardware and services own policy and system behaviour.

# Implementation

Implement the service under 'crates/services/powermanagement'. The 'crates/drivers/src/voltagemonitor' and 'crates/drivers/src/batterycharger' drivers are the key inputs into the power management service.

# Testing

Make a small test suite under 'servicetests/powermanagement/' that exercises the key functionality of the service and indicates the current power status of the system periodically.