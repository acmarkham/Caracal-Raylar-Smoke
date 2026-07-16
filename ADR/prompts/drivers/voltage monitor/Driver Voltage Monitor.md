---
# ADR Voltage Monitor Driver


## Constraints
Read
ADR\common\AGENTS.md
before planning and implementing

---

# Context

The platform measures several analog voltages using the STM32 ADC peripherals.

These include:

* Battery voltage
* Solar panel voltage
* External DC input voltage
* Internal voltage reference (VREFINT)

The ADC configuration, factory calibration, voltage divider ratios and conversion from ADC counts to millivolts are properties of the hardware design and should not leak into higher-level services.

The Embassy STM32 ADC driver already provides low-level ADC functionality. This driver provides a higher-level abstraction representing calibrated platform voltages.

---

# Decision

Introduce a dedicated Voltage Monitor Driver.

The driver owns the ADC peripherals and periodically samples all configured voltage channels.

It publishes calibrated voltage measurements expressed in millivolts.

The driver exposes physical measurements rather than ADC counts.

---

# Responsibilities

The Voltage Monitor Driver shall:

* Configure ADC peripherals
* Read VREFINT
* Apply STM32 factory calibration
* Apply PCB-specific voltage divider ratios
* Convert measurements to millivolts
* Publish the latest voltage measurements
* Hide all ADC implementation details

---

# Non-Goals

The driver is **not** responsible for:

* Battery percentage estimation
* Charger control
* Low battery decisions
* Power source selection
* Sleep policy

These belong to the Power Service.

---

# Public State

```rust
pub struct VoltageState {
    pub battery_mv: u32,
    pub solar_mv: u32,
    pub ext_dc_mv: u32,
    pub usb_mv: u32,
    pub vref_mv: u32,
}
```

Published using:

```rust
Watch<VoltageState>
```

---

# Sampling

Voltage measurements change slowly.

The driver should sample periodically (nominally 1 Hz).

Future revisions may support configurable sampling rates.

---

# Hardware Knowledge

The driver owns:

* ADC channel assignments
* Voltage divider constants
* ADC calibration
* Conversion algorithms

Changes to PCB resistor values should require changes only within this driver.

---

# Consequences

Applications and services consume calibrated voltages without needing knowledge of ADC peripherals or PCB implementation.


---
# Prototype

unitsmoke/22_read_adc has fully working and tested ADC voltage routines including setting VREF, sampling, and handling the voltage dividers. Use this code to build a small and sensible driver for voltage monitoring.

The USB voltage divider has not been implemented yet. It is 5.1k/10k connected to USB_VBUS. It should be optional to use it as an ADC pin, as other services (like USB) may need it as digital input to flag an interrupt if the USB is connected at a random point in time. USB_VBUS is PA9 for raylar.

# Implementation

Implement the voltage monitor driver under 'crates/drivers/voltagemonitor/'

# Tests

Make a thin test under 'drivertests/voltagemonitor/'