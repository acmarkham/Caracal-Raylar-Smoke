---

# ADR: LIS2HH12 Accelerometer Driver

## Constraints

Read `ADR/common/AGENTS.md` before planning and implementing.

---

# Context

Raylar v1.0 includes an ST LIS2HH12 three-axis accelerometer on the SensI2C bus.

The initial use case is low-rate orientation and inclination measurement. The
device will normally be stationary, so the first driver need not continuously
sample, use interrupts, or use the FIFO. Applications should not need to know
the LIS2HH12 register map, scale encodings, or temperature-data format.

The hardware address used on this board is `0x1D`. `WHO_AM_I` is register
`0x0F` and must return `0x41` for an LIS2HH12.

`unitsmoke/05_sensi2c_acc/main.rs` demonstrates a six-byte, auto-incrementing
read starting at `OUT_X_L` (`0x28`). `unitsmoke/26_tempsensors/main.rs`
demonstrates reading the two temperature-output bytes starting at `TEMP_L`
(`0x0B`) and converting the signed, 11-bit value using the nominal 25 degC
offset and 8 LSB/degC sensitivity.

The LIS2HH12 data sheet, including Table 24 for output-data-rate selection, is
the source of truth for register encodings and timing:

<https://www.st.com/resource/en/datasheet/lis2hh12.pdf>

---

# Decision

Implement a small LIS2HH12 driver under:

```text
crates/drivers/sensor_acc/
```

The driver owns LIS2HH12-specific I2C transactions, register encoding,
identity validation, sample conversion, and power-mode control. It receives an
I2C device from the board or bus-sharing layer; it does not own the SensI2C
peripheral, pins, or bus-wide configuration because that bus is shared by
other sensors.

The driver is polling-only for this revision. A caller explicitly requests an
acceleration or temperature sample. No task, interrupt pin, FIFO, DMA, heap
allocation, or shared-state publisher is required.

---

# Responsibilities

The driver shall:

* Verify the device identity during construction/initialisation by reading
  `WHO_AM_I` and requiring `0x41`.
* Provide explicit `turn_on` and `turn_off` operations.
* Configure full-scale range to 2 g, 4 g, or 8 g, with 2 g as the default.
* Configure one of the supported output data rates from Table 24: 10, 50, 100,
  200, 400, or 800 Hz, with 10 Hz as the default.
* Read a coherent X/Y/Z sample by one auto-incrementing six-byte read of the
  output registers.
* Convert acceleration to a documented physical unit as well as making the
  native signed sample counts available when useful for diagnostics.
* Read and convert the onboard temperature-sensor output.
* Return distinguishable errors for I2C failures, an absent/wrong device ID,
  and invalid configuration requests.

---

# Non-Goals

This initial driver is not responsible for:

* FIFO configuration or draining
* Data-ready or motion interrupts
* Free-fall, click, wake-up, activity, or threshold detection
* Continuous acquisition, background tasks, buffering, or publishing samples
* Orientation filtering, tilt estimation, calibration, or application policy
* Ambient-temperature measurement or temperature calibration
* Coordinating access to the shared SensI2C bus

These can be layered above the driver, or added in a later ADR when a real
application needs them.

---

# Configuration and Power State

Use a configuration type that represents only valid choices, rather than
accepting arbitrary register values or integer rates:

```rust
pub enum FullScale {
    G2,
    G4,
    G8,
}

pub enum OutputDataRate {
    Hz10,
    Hz50,
    Hz100,
    Hz200,
    Hz400,
    Hz800,
}

pub struct AccelerometerConfig {
    pub full_scale: FullScale,
    pub odr: OutputDataRate,
}

impl Default for AccelerometerConfig {
    // full_scale: G2, odr: Hz10
}
```

The implementation shall map these options to the `CTRL4.FS` and `CTRL1.ODR`
fields respectively. It shall use the data-sheet-defined encodings, not values
copied from application code. Values outside the enum must not be silently
rounded to a different rate or range.

`turn_on()` applies the stored configuration, enables all three axes, and sets
block-data update (BDU), so a multi-byte sample cannot mix old and new axis
data. `turn_off()` places the accelerometer in power-down by clearing the ODR
field. It retains the requested range and rate in driver state so a subsequent
`turn_on()` restores them.

Construction/initialisation verifies `WHO_AM_I`, records the default
configuration, and leaves the device powered down until `turn_on()` is called.
This makes power usage explicit while retaining the requested default settings.

`configure()` updates the stored configuration. If the device is on, it shall
apply the new configuration safely and preserve the on state; if it is off, the
new settings take effect on the next `turn_on()`.

---

# Public API

The exact Rust generics and error bounds may follow the selected Embassy and
`embedded-hal` I2C abstraction, but the device-level API should be equivalent
to:

```rust
pub struct Lis2hh12<I2C> { /* private */ }

pub struct RawAcceleration {
    pub x: i16,
    pub y: i16,
    pub z: i16,
}

pub struct Acceleration {
    pub x_mg: i32,
    pub y_mg: i32,
    pub z_mg: i32,
}

pub struct DieTemperature {
    pub raw: i16,
    pub milli_celsius: i32,
}

impl<I2C> Lis2hh12<I2C> {
    pub fn new(i2c: I2C) -> Result<Self, Error>;
    pub fn configure(&mut self, config: AccelerometerConfig) -> Result<(), Error>;
    pub fn turn_on(&mut self) -> Result<(), Error>;
    pub fn turn_off(&mut self) -> Result<(), Error>;
    pub fn is_on(&self) -> bool;
    pub fn read_raw_acceleration(&mut self) -> Result<RawAcceleration, Error>;
    pub fn read_acceleration(&mut self) -> Result<Acceleration, Error>;
    pub fn read_die_temperature(&mut self) -> Result<DieTemperature, Error>;
}
```

The final methods may be `async` if the project supplies an async I2C device;
that is an integration detail, not a change to the API semantics.

Read operations made while the device is off shall return a clear
`NotEnabled`-style error rather than implicitly powering the sensor or
returning stale output registers.

---

# Sampling and Conversion

Each acceleration request shall perform one polling transaction:

1. Optionally inspect the data-available status bit so callers can distinguish
   a fresh sample from a request made before the configured ODR period elapsed.
2. Read six bytes starting at `OUT_X_L` with the I2C register auto-increment
   bit set, as proved by `unitsmoke/05_sensi2c_acc`.
3. Decode X, Y, and Z as little-endian signed values.
4. Convert counts using the sensitivity for the currently configured full-scale
   setting from the data sheet.

`read_raw_acceleration()` exists for diagnostics and calibration. The normal
application-facing measurement is `Acceleration` in milli-g (`mg`), avoiding
floating point and making inclination use straightforward. The driver must
document that acceleration includes gravity.

The driver shall not wait internally for the next sample period. A polling
caller may use the selected ODR to choose its own cadence. If freshness status
is exposed, it must not change the fact that reading is caller-driven.

---

# Temperature Sensor

`read_die_temperature()` shall read the two temperature registers in one
auto-incrementing I2C transaction. It shall sign-extend the data-sheet-defined
11-bit temperature result and return both that raw value and:

```text
temperature_degC = 25 + raw / 8
```

The implementation should use fixed-point milli-degrees Celsius, preserving
the 0.125 degC nominal resolution without floating point.

This is the accelerometer die temperature, useful for health monitoring and
trend observation. It is not specified as an accurate ambient-temperature
sensor; callers needing ambient temperature must use an appropriate external
sensor and calibration.

---

# Error Handling

Expose an error type with at least these categories:

```rust
pub enum Error<E> {
    Bus(E),
    DeviceIdMismatch { observed: u8 },
    NotEnabled,
}
```

An I2C acknowledgement or read error is not equivalent to a missing device ID.
Neither error should be hidden or converted into a zero acceleration or
temperature reading. A failed identity check means construction/initialisation
fails and the driver is not usable.

---

# Hardware Knowledge

The driver owns:

* I2C address `0x1D`
* `WHO_AM_I` register and expected value `0x41`
* Register addresses, auto-increment handling, and bit fields
* ODR mapping from Table 24
* Full-scale mapping and sensitivity conversion
* Little-endian output decoding
* Temperature sign extension and nominal conversion
* BDU and three-axis enable configuration

No service outside this driver should set LIS2HH12 registers directly.

---

# Prototype References

Use these smoke tests as working hardware references during implementation:

* `unitsmoke/05_sensi2c_acc` for I2C initialisation, identity access, control
  register setup, and X/Y/Z output reads.
* `unitsmoke/26_tempsensors` for the LIS2HH12 temperature-register read,
  11-bit sign handling, and nominal temperature conversion.

The smoke-test register literals are evidence of working board wiring, but the
driver must replace them with named fields and validated configuration types.

---

# Testing

Add a thin hardware test under:

```text
drivertests/sensor_acc/
```

It shall:

* Confirm successful initialisation and log the observed `WHO_AM_I` value.
* Confirm a mismatched identity is reported by a unit/mock-I2C test.
* Exercise on/off transitions and verify reads while off return `NotEnabled`.
* Exercise every supported range and ODR encoding with mock-I2C assertions.
* At the default 2 g / 10 Hz configuration, print periodic raw and `mg` X/Y/Z
  samples from hardware.
* Read and print raw and converted die temperature from hardware.

No FIFO, interrupt, high-rate, or long-duration test is needed for this first
driver revision.

---

# Consequences

Applications receive a simple, low-power inclinometer-oriented interface and
do not depend on LIS2HH12 registers. The deliberately narrow scope keeps the
driver small and testable, while preserving a clean place to add FIFO or
interrupt support later without changing ordinary polling clients.

