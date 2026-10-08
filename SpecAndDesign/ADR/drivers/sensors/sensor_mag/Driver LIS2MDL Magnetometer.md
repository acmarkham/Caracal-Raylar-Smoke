---

# ADR: LIS2MDL Magnetometer Driver

## Constraints

Read `ADR/common/AGENTS.md` before planning and implementing.

---

# Context

Raylar v1.0 includes an ST LIS2MDL three-axis magnetometer on the SensI2C bus.

The initial use case is low-rate compass-heading measurement. The device will
normally be stationary, so the first driver need not continuously publish data,
use interrupts, or perform high-rate acquisition. Applications should not need
to know the LIS2MDL register map, operating-mode encodings, offset-cancellation
configuration, or temperature-data format.

The hardware address used on this board is `0x1E`. `WHO_AM_I` is register
`0x4F` and must return `0x40` for an LIS2MDL.

`unitsmoke/06_sensi2c_mag/main.rs` demonstrates a six-byte,
auto-incrementing read starting at `OUTX_L` (`0x68`).
`unitsmoke/26_tempsensors/main.rs` demonstrates reading the two temperature
bytes starting at `TEMP_OUT_L` (`0x6E`) and converting the signed 12-bit
result using the nominal 25 degC offset and 8 LSB/degC sensitivity.

The LIS2MDL data sheet, including Table 24 for output-data-rate selection, is
the source of truth for register encodings and timing:

<https://www.st.com/resource/en/datasheet/lis2mdl.pdf>

The default offset-cancellation behaviour follows ST application note AN5069:

<https://www.st.com/resource/en/application_note/an5069-lis2mdl-ultralowpower-highperformance-3axis-magnetometer-stmicroelectronics.pdf>

---

# Decision

Implement a small LIS2MDL driver under:

```text
crates/drivers/sensor_mag/
```

The driver owns LIS2MDL-specific I2C transactions, register encoding,
identity validation, sample conversion, offset-cancellation configuration, and
power-mode control. It receives an I2C device from the board or bus-sharing
layer; it does not own the SensI2C peripheral, pins, or bus-wide configuration
because that bus is shared by other sensors.

The driver is polling-only for this revision. A caller explicitly requests a
magnetic-field or temperature sample. No task, interrupt pin, FIFO, DMA, heap
allocation, or shared-state publisher is required.

---

# Responsibilities

The driver shall:

* Verify device identity during construction/initialisation by reading
  `WHO_AM_I` and requiring `0x40`.
* Provide explicit `turn_on` and `turn_off` operations.
* Use high-resolution mode by default.
* Configure one of the supported output data rates from Table 24: 10, 20, 50,
  or 100 Hz, with 10 Hz as the default.
* Use continuous-conversion mode whenever the sensor is on.
* Enable temperature compensation and sensor offset cancellation by default.
* Read a coherent X/Y/Z sample by one auto-incrementing six-byte read of the
  output registers.
* Convert magnetic field to a documented physical unit as well as making the
  native signed sample counts available for diagnostics and calibration.
* Read and convert the onboard temperature-sensor output.
* Return distinguishable errors for I2C failures, an absent/wrong device ID,
  and attempts to read while the sensor is off.

---

# Non-Goals

This initial driver is not responsible for:

* Data-ready, threshold, or other interrupts
* Single-shot operation
* Low-power resolution mode
* Background acquisition, buffering, or publishing samples
* Heading calculation, tilt compensation, hard/soft-iron calibration, or
  declination correction
* Ambient-temperature measurement or temperature calibration
* Coordinating access to the shared SensI2C bus

Heading and calibration policy belong to a higher-level orientation service.
These features can be added in a later ADR if a real application requires
them.

---

# Configuration and Power State

Use a configuration type that represents only valid rates, rather than
accepting arbitrary register values or integer rates:

```rust
pub enum OutputDataRate {
    Hz10,
    Hz20,
    Hz50,
    Hz100,
}

pub struct MagnetometerConfig {
    pub odr: OutputDataRate,
}

impl Default for MagnetometerConfig {
    // odr: Hz10
}
```

The implementation shall map these options to `CFG_REG_A.ODR` using the
data-sheet-defined encodings. Values outside the enum must not be silently
rounded to a different rate.

The following settings are fixed for this first revision and applied whenever
the sensor is enabled:

| Setting | Register field | Required value |
| --- | --- | --- |
| Operating mode | `CFG_REG_A.MD` | Continuous mode |
| Resolution | `CFG_REG_A.LP` | High resolution (`0`) |
| Temperature compensation | `CFG_REG_A.COMP_TEMP_EN` | Enabled |
| Offset cancellation | `CFG_REG_B.SET_RST` | Sensor offset cancellation every ODR cycle |
| Coherent multi-byte data | `CFG_REG_C.BDU` | Enabled |
| Data byte order | `CFG_REG_C.BLE` | Little-endian |
| I2C interface | `CFG_REG_C.I2C_DIS` | Enabled |

The application-note-recommended `SET_RST` mode performs sensor offset
cancellation at every ODR cycle. This is intentional for stable compass use:
it reduces residual sensor offset and drift, at the modest power and timing
cost appropriate for the low-rate stationary use case.

`turn_on()` applies the stored ODR and the fixed settings above, then selects
continuous mode. `turn_off()` selects power-down mode through `CFG_REG_A.MD`.
It retains the requested ODR in driver state so a subsequent `turn_on()`
restores it.

Construction/initialisation verifies `WHO_AM_I`, records the default
configuration, and leaves the device in power-down until `turn_on()` is
called. Thus the default operating mode is continuous when enabled, while
sensor power use remains explicit.

`configure()` updates the stored ODR. If the device is on, it shall apply the
new ODR while preserving continuous, high-resolution, temperature-compensated,
and offset-cancelled operation; if it is off, the new rate takes effect on the
next `turn_on()`.

---

# Public API

The exact Rust generics and error bounds may follow the selected Embassy and
`embedded-hal` I2C abstraction, but the device-level API should be equivalent
to:

```rust
pub struct Lis2mdl<I2C> { /* private */ }

pub struct RawMagneticField {
    pub x: i16,
    pub y: i16,
    pub z: i16,
}

pub struct MagneticField {
    pub x_nanotesla: i32,
    pub y_nanotesla: i32,
    pub z_nanotesla: i32,
}

pub struct DieTemperature {
    pub raw: i16,
    pub milli_celsius: i32,
}

impl<I2C> Lis2mdl<I2C> {
    pub fn new(i2c: I2C) -> Result<Self, Error>;
    pub fn configure(&mut self, config: MagnetometerConfig) -> Result<(), Error>;
    pub fn turn_on(&mut self) -> Result<(), Error>;
    pub fn turn_off(&mut self) -> Result<(), Error>;
    pub fn is_on(&self) -> bool;
    pub fn is_data_ready(&mut self) -> Result<bool, Error>;
    pub fn read_raw_magnetic_field(&mut self) -> Result<RawMagneticField, Error>;
    pub fn read_magnetic_field(&mut self) -> Result<MagneticField, Error>;
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

Each magnetic-field request shall perform one polling transaction:

1. Optionally inspect `STATUS_REG.ZYXDA` so callers can distinguish a fresh
   sample from a request made before the configured ODR period elapsed.
2. Read six bytes starting at `OUTX_L` with the I2C register auto-increment
   bit set, as proved by `unitsmoke/06_sensi2c_mag`.
3. Decode X, Y, and Z as little-endian signed values.
4. Convert counts using the data-sheet sensitivity of 1.5 milli-gauss per LSB.

`read_raw_magnetic_field()` exists for diagnostics and for later
hard/soft-iron calibration. The normal application-facing measurement is
`MagneticField` in nanotesla (`nT`): one LIS2MDL LSB is exactly 150 nT, so this
retains the sensor's nominal resolution without floating point.

The driver shall not wait internally for the next sample period. A polling
caller may use the selected ODR to choose its own cadence. Reading magnetic
field does not produce a compass heading; a heading requires board orientation,
calibration, and often tilt compensation from the accelerometer.

---

# Temperature Sensor

`read_die_temperature()` shall read the two temperature registers in one
auto-incrementing I2C transaction. It shall sign-extend the data-sheet-defined
12-bit temperature result and return both that raw value and:

```text
temperature_degC = 25 + raw / 8
```

The implementation should use fixed-point milli-degrees Celsius, preserving
the 0.125 degC nominal resolution without floating point.

This is the magnetometer die temperature, useful for health monitoring and
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
Neither error should be hidden or converted into a zero magnetic-field or
temperature reading. A failed identity check means construction/initialisation
fails and the driver is not usable.

---

# Hardware Knowledge

The driver owns:

* I2C address `0x1E`
* `WHO_AM_I` register and expected value `0x40`
* `CFG_REG_A`, `CFG_REG_B`, and `CFG_REG_C` bit fields
* ODR mapping from Table 24
* Continuous and power-down mode encoding
* High-resolution, temperature-compensation, BDU, and offset-cancellation
  configuration
* Register addresses, auto-increment handling, and little-endian output
  decoding
* Magnetic-field and temperature conversion

No service outside this driver should set LIS2MDL registers directly.

---

# Prototype References

Use these smoke tests as working hardware references during implementation:

* `unitsmoke/06_sensi2c_mag` for I2C initialisation, identity access,
  continuous-mode setup, and X/Y/Z output reads.
* `unitsmoke/26_tempsensors` for the LIS2MDL temperature-register read,
  12-bit sign handling, temperature compensation enable, and nominal
  temperature conversion.

The smoke-test register literals are evidence of working board wiring, but the
driver must replace them with named fields and validated configuration types.

---

# Testing

Add a thin hardware test under:

```text
drivertests/sensor_mag/
```

It shall:

* Confirm successful initialisation and log the observed `WHO_AM_I` value.
* Confirm a mismatched identity is reported by a unit/mock-I2C test.
* Exercise on/off transitions and verify reads while off return `NotEnabled`.
* Exercise every supported ODR encoding with mock-I2C assertions.
* Assert the default high-resolution, continuous-mode, temperature-compensation,
  BDU, and every-ODR offset-cancellation register settings with mock I2C.
* At the default 10 Hz configuration, print periodic raw and `nT` X/Y/Z samples
  from hardware.
* Read and print raw and converted die temperature from hardware.

No interrupt, high-rate, heading, calibration, or long-duration test is needed
for this first driver revision.

---

# Consequences

Applications receive a simple, low-power compass-oriented interface and do not
depend on LIS2MDL registers. Fixed high-resolution continuous operation with
per-ODR offset cancellation favors stable field measurements over the lowest
possible sensor power. The deliberately narrow scope keeps the driver small
and testable while preserving a clean place to add heading or calibration
services later without changing ordinary polling clients.

