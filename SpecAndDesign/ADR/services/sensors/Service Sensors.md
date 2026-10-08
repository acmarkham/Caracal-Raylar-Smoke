# ADR: Sensor Service Architecture

## Status

Proposed

---

## Constraints

Read `ADR/common/AGENTS.md` before planning and implementing.

---

# Context

The platform has several low-rate physical sensors, including the LIS2HH12
accelerometer and LIS2MDL magnetometer. Other sources may be added later, such
as the STM32 internal temperature sensor or sensors on the extension bus.

Drivers expose device-specific operations and calibrated readings, but
application services should not each need to schedule polling, retain the last
reading, identify duplicate measurement sources, or repeat common calculations.

A measurement type is not necessarily unique. For example, temperature may be
available from the LIS2HH12 die, LIS2MDL die, STM32 core, radio, and future
external sensors. Consumers must be able to request a particular source rather
than treating all temperatures as interchangeable.

The platform also needs derived measurements that combine or transform raw
sensor data:

* accelerometer-derived tilt
* magnetometer-derived heading
* a tilt-compensated electronic compass
* vectorial dynamic body acceleration (VeDBA)
* overall dynamic body acceleration (ODBA)

Finally, applications need events when an absolute limit is crossed or when a
measurement changes significantly between readings.

---

# Decision

Introduce a heapless Sensor Service under:

```text
crates/services/sensors/
```

The service polls statically registered sensor sources, retains their latest
readings in RAM, calculates registered composite sensors, evaluates threshold
rules, and publishes the current sensor snapshot.

The service owns scheduling and sensor policy. Hardware drivers continue to own
I2C, ADC, register maps, and device-level conversion.

The default polling interval is 10 seconds, equivalent to 0.1 Hz. Each source
may override that interval when its measurement or composite consumers require
a different rate.

---

# Design Goals

The Sensor Service shall:

* Be heapless and use fixed-capacity storage.
* Poll registered sensors automatically.
* Retain the most recent valid reading from every registered source.
* Distinguish sensor identity from measurement type.
* Permit multiple sources of the same measurement type.
* Publish the latest complete snapshot through `embassy_sync::watch`.
* Support independently configurable polling intervals, defaulting to 10
  seconds.
* Calculate registered composite sensors from raw readings.
* Evaluate absolute and delta threshold rules.
* Emit ordered threshold events through a bounded channel.
* Preserve timestamps, validity, staleness, and error information.
* Continue operating when one sensor fails.

---

# Non-Goals

The first Sensor Service is not responsible for:

* I2C, SPI, ADC, or GPIO configuration
* Device register access
* Durable sensor-data logging or historical storage
* Sensor discovery or arbitrary runtime hot-plugging
* Automatic calibration without supplied calibration parameters
* Interrupt-driven high-rate acquisition
* Geographic magnetic-declination lookup
* Application actions in response to events
* Choosing power-management policy

The Storage or Logging Services may persist readings and events. Application or
policy services decide what to do after receiving a threshold event.

---

# Architecture

```text
 Sensor Drivers / Adapters
 ACC   MAG   STM32 temp   Extension sensors
  |     |         |               |
  +-----+---------+---------------+
                    |
                    v
              Sensor Service
       +----------------------------+
       | Fixed-capacity registry    |
       | Poll scheduler             |
       | Latest-reading store       |
       | Composite calculations     |
       | Threshold evaluator        |
       +-------------+--------------+
                     |
          +----------+----------+
          |                     |
          v                     v
 Watch<SensorSnapshot>   Channel<SensorEvent>
          |                     |
          v                     v
 State consumers       Policy/event consumer
```

One service task schedules all low-rate polling. It sleeps until the earliest
registered source is due rather than waking at a fixed high-frequency tick.
The initial implementation should not create one Embassy task per sensor.

---

# Sensor Registration

Sensors are registered before the service starts. Registration uses a
fixed-capacity `heapless::Vec`; exceeding capacity returns a clear error.

Each hardware source is wrapped by a small adapter implementing a service-level
sampling interface. Conceptually:

```rust
pub trait SensorSource {
    fn sample(&mut self) -> Result<SensorValue, SensorSourceError>;
}

pub struct SensorRegistration<'a> {
    pub descriptor: SensorDescriptor,
    pub poll_interval: Duration,
    pub source: &'a mut dyn SensorSource,
}
```

The exact trait may be synchronous or asynchronous to match the underlying bus
abstraction. Implementations must be statically allocated; trait objects, if
used, are borrowed static objects and do not imply heap allocation.

The service does not configure an accelerometer or magnetometer register
directly. Adapters call the corresponding driver APIs and translate their
outputs into service-level values.

Runtime hot-plug discovery is out of scope. A future extension may add bounded
registration commands for extension-bus devices.

---

# Identity and Measurement Type

Every registered source has a stable `SensorId`. `SensorId` identifies a
specific provider, while `SensorKind` describes what it measures.

Conceptually:

```rust
pub struct SensorId(pub u16);

pub enum SensorKind {
    Acceleration,
    MagneticField,
    Temperature,
    Tilt,
    Heading,
    TiltCompensatedHeading,
    Vedba,
    Odba,
}

pub enum SensorOrigin {
    Lis2hh12,
    Lis2mdl,
    Stm32Core,
    Radio,
    ExtensionBus(u8),
    Composite,
}

pub struct SensorDescriptor {
    pub id: SensorId,
    pub kind: SensorKind,
    pub origin: SensorOrigin,
}
```

`SensorId` values must be unique within the registry. Duplicate IDs are rejected
at registration. Multiple descriptors may use the same `SensorKind`.

For example, the following are distinct readings:

```text
SensorId(10): Temperature / LIS2HH12
SensorId(11): Temperature / LIS2MDL
SensorId(12): Temperature / STM32 core
SensorId(20): Temperature / Extension bus slot 0
```

Consumers query by `SensorId` when they require a particular source. Queries by
`SensorKind` return all matching sources rather than silently choosing one.

IDs should be assigned by board/application configuration and remain stable
across firmware builds when they are included in telemetry or stored data.

---

# Values and Units

Use typed, fixed-point values rather than an unlabelled scalar. Suggested
representation:

```rust
pub enum SensorValue {
    AccelerationMg { x: i32, y: i32, z: i32 },
    MagneticFieldNt { x: i32, y: i32, z: i32 },
    TemperatureMilliCelsius(i32),
    TiltCentiDegrees { roll: i32, pitch: i32 },
    HeadingCentiDegrees(u16),
    DynamicAccelerationMg(u32),
}
```

Heading values use the half-open range `[0, 36000)`, representing 0.00 through
359.99 degrees. Units are part of the variant name and must not depend on
external metadata.

Floating point may be used internally only if justified by the target and the
chosen trigonometric implementation. Public values remain fixed-point for
compact, deterministic storage and comparison.

---

# Latest Reading and Published State

The service stores one latest-reading record for every registered raw or
composite sensor:

```rust
pub struct SensorReading {
    pub id: SensorId,
    pub value: Option<SensorValue>,
    pub status: ReadingStatus,
    pub sequence: u64,
    pub last_attempt: Instant,
    pub last_success: Option<Instant>,
    pub consecutive_errors: u32,
}

pub enum ReadingStatus {
    Unavailable,
    Current,
    Stale,
    Fault,
}

pub struct SensorSnapshot<const SENSORS: usize> {
    pub readings: heapless::Vec<SensorReading, SENSORS>,
    pub generation: u64,
    pub published_at: Instant,
}
```

The latest snapshot is published using:

```rust
Watch<SensorSnapshot<SENSORS>>
```

The watch is appropriate because state consumers need the newest readings, not
every intermediate polling update.

The service publishes a new generation after a raw reading and all composites
affected by that reading have been evaluated. Consumers therefore do not see a
raw update without its corresponding derived-state update.

“Stored” means retained in service RAM. The Sensor Service does not write files.

---

# Polling and Scheduling

The default poll interval is:

```text
10 seconds (0.1 Hz)
```

Each registration may specify a different non-zero interval. The scheduler
tracks a separate next-due time for each source and sleeps until the earliest
deadline.

Polling behavior shall follow these rules:

* Polling starts after source registration is complete.
* A slow or failed source does not invalidate unrelated readings.
* Missed periods do not cause a burst of catch-up reads.
* The next deadline is advanced from the current time after a late poll.
* A zero interval is rejected.
* Composite sensors are evaluated when an input changes, not on an independent
  hardware-poll schedule.
* Poll interval changes, if supported at runtime, use a bounded command channel.

The service should expose staleness independently of polling interval. A
reasonable default is to mark a reading stale after a configurable multiple of
its interval, for example three missed or failed periods.

---

# Sensor Failure Behavior

When a source read fails:

* preserve the last valid value and its `last_success` timestamp
* record the failed `last_attempt`
* increment the source's consecutive and total error counters
* change status to `Stale` or `Fault` according to configured policy
* do not substitute zero
* do not prevent other sensors from being polled
* do not evaluate delta thresholds using the failed attempt

After the next successful sample, reset the consecutive error count and return
the reading to `Current`.

---

# Composite Sensors

Composite sensors are registered with their own `SensorId`, descriptor,
configuration, and explicit dependency IDs. Their outputs appear in the same
snapshot and may be used by threshold rules exactly like hardware readings.

A composite is valid only when every required input:

* exists
* has a compatible value type
* has a valid latest value
* is not stale
* satisfies any configured maximum age or cross-input timestamp skew

If these conditions are not met, the composite retains its last valid value but
is marked `Stale` or `Unavailable`.

---

# Tilt

Tilt is derived from the gravity vector reported by an accelerometer. The first
implementation should publish roll and pitch in centi-degrees.

The calculation must:

* use a configured board-to-sensor axis mapping
* reject or flag samples whose acceleration magnitude is implausibly far from
  1 g for a stationary tilt estimate
* document its sign and axis conventions
* use `atan2`-based formulas to avoid quadrant ambiguity

Tilt is an orientation estimate under the assumption that acceleration is
dominated by gravity. It is unreliable during significant linear acceleration.

---

# Magnetic Heading

Basic heading is derived from the horizontal magnetometer X/Y axes using
`atan2` and normalized into `[0, 360)` degrees.

The calculation must apply configured board-to-sensor axis mapping and any
supplied hard-iron/soft-iron calibration before computing heading.

An uncalibrated result may be exposed for development, but its descriptor or
status must make that limitation clear. Magnetic declination is a configurable
correction; automatic geographic lookup is out of scope.

Basic heading assumes the board is level. Tilted installations should consume
the tilt-compensated heading instead.

---

# Tilt-Compensated Electronic Compass

The electronic-compass composite combines an accelerometer and magnetometer.
It shall follow the rotation and compensation method described in ST design tip
DT0058:

<https://www.st.com/resource/en/design_tip/dt0058-computing-tilt-measurement-and-tiltcompensated-ecompass-stmicroelectronics.pdf>

The configured accelerometer and magnetometer IDs are explicit; the service
must never select an arbitrary source merely because its kind matches.

Before calculating a compensated heading, the service shall verify:

* both readings are current
* their timestamps differ by no more than a configurable maximum skew
* both use the configured axis mapping
* accelerometer magnitude is suitable for a gravity-based tilt estimate
* magnetometer calibration, if required by configuration, is available

The output uses centi-degrees normalized to `[0, 36000)`. The implementation and
tests must define north, rotation direction, board axes, and expected results
for level and tilted reference vectors.

---

# Dynamic Acceleration: VeDBA and ODBA

Dynamic acceleration is calculated by separating each acceleration axis into a
static component and a dynamic component:

```text
dynamic_axis = measured_axis - estimated_static_axis
```

The static component should initially use a configurable fixed-point low-pass
filter or bounded moving mean.

Derived metrics are:

```text
VeDBA = sqrt(dynamic_x^2 + dynamic_y^2 + dynamic_z^2)
ODBA  = abs(dynamic_x) + abs(dynamic_y) + abs(dynamic_z)
```

Outputs are expressed in milli-g.

The 0.1 Hz default is not sufficient for meaningful dynamic body acceleration.
Registering VeDBA or ODBA therefore requires the dependency accelerometer to use
a suitable configured rate and filter window. The service shall reject a
composite configuration whose input rate is below a documented minimum rather
than publishing a misleading value. A starting minimum such as 10 Hz may be
chosen during implementation and validated with tests.

The low-rate default remains appropriate for stationary tilt, heading,
temperature, and health telemetry.

---

# Threshold Rules

Threshold rules are stored in a fixed-capacity table. Each rule has a stable
`ThresholdId`, targets a specific `SensorId`, and selects a scalar metric from
the reading.

Conceptually:

```rust
pub enum ValueSelector {
    Scalar,
    X,
    Y,
    Z,
    Magnitude,
    Roll,
    Pitch,
}

pub enum AbsoluteComparison {
    Above,
    Below,
}

pub enum ThresholdRule {
    Absolute {
        id: ThresholdId,
        sensor: SensorId,
        selector: ValueSelector,
        comparison: AbsoluteComparison,
        threshold: i32,
        hysteresis: u32,
    },
    Delta {
        id: ThresholdId,
        sensor: SensorId,
        selector: ValueSelector,
        minimum_absolute_change: u32,
    },
}
```

Threshold values use the fixed-point unit of the selected value. Registration
validates that the selector is compatible with the target sensor kind.

Rules may be installed at construction and may optionally be added, replaced,
enabled, or removed at runtime through a bounded command channel. Capacity
errors and invalid sensor/selector combinations are returned to the caller.

---

# Absolute Threshold Semantics

An absolute rule triggers when a valid reading crosses from the normal region
into the configured violation region. For example:

```text
Temperature above 40.000 degC
```

The rule emits one event when the temperature first reaches or exceeds 40,000
millidegrees Celsius. It does not emit an event on every subsequent poll.

The rule rearms only after the value returns past the threshold by the
configured hysteresis. Hysteresis prevents repeated events caused by noise near
the boundary. Initial startup behavior is configurable; the recommended default
is to emit an event if the first valid sample is already in violation.

---

# Delta Threshold Semantics

A delta rule compares the current valid sample with the immediately preceding
valid sample for the same sensor and selector:

```text
absolute_delta = abs(current - previous)
```

An event is emitted when the absolute delta reaches or exceeds the configured
limit. Failed samples do not replace the previous valid baseline.

For angular values, delta uses the shortest circular distance. For example, a
heading change from 359 degrees to 1 degree is 2 degrees, not 358 degrees.

A rule may configure a maximum comparison interval. If the previous sample is
too old, the current sample establishes a new baseline without emitting an
event. Each qualifying pair may emit an event; delta rules do not remain latched
like absolute rules.

---

# Sensor Events

Threshold events are discrete and must not be represented by the latest-state
watch. They are delivered in order through a bounded Embassy channel:

```rust
pub struct SensorEvent {
    pub sequence: u64,
    pub threshold_id: ThresholdId,
    pub sensor_id: SensorId,
    pub kind: SensorEventKind,
    pub current: i32,
    pub previous: Option<i32>,
    pub threshold: i32,
    pub observed_at: Instant,
}

pub enum SensorEventKind {
    AbsoluteEntered,
    AbsoluteCleared,
    DeltaExceeded,
}
```

Use `embassy_sync::channel::Channel` because each event must be delivered once
to the policy consumer. The initial architecture supports one logical event
consumer. If multiple independent consumers later require every event, introduce
a bounded pub/sub layer in a separate decision.

Channel capacity is configurable at compile time. Sending may apply backpressure
when the channel is full, preserving event order and delivery. The service shall
publish queue high-water and threshold-event counters so capacity can be tuned.

---

# Public Resources and API

Conceptually:

```rust
pub struct SensorResources<
    const SENSORS: usize,
    const WATCHERS: usize,
    const EVENTS: usize,
> {
    latest: Watch<SensorMutex, SensorSnapshot<SENSORS>, WATCHERS>,
    events: Channel<SensorMutex, SensorEvent, EVENTS>,
}

pub struct SensorService<...> { /* fixed-capacity registry and rules */ }

impl SensorService<...> {
    pub fn register_source(&mut self, registration: SensorRegistration)
        -> Result<(), RegistrationError>;

    pub fn register_composite(&mut self, composite: CompositeRegistration)
        -> Result<(), RegistrationError>;

    pub fn set_threshold(&mut self, rule: ThresholdRule)
        -> Result<(), ThresholdError>;

    pub fn state(&self) -> SensorSnapshot;
    pub fn watch(&self) -> SensorStateReceiver;
    pub fn event_receiver(&self) -> SensorEventReceiver;
    pub async fn run(self) -> !;
}
```

The exact generics may evolve, but all capacities are compile-time bounded and
all resources are statically allocated.

Registration and threshold setup should normally finish before `run()` consumes
the service. If runtime configuration is required, expose a lightweight handle
backed by a bounded command channel rather than shared mutable access.

---

# Timing and Synchronisation

Every successful raw sample is timestamped with local monotonic `Instant` as
close as practical to acquisition. Composite timestamps represent the newest
input used and retain enough metadata to inspect input age/skew.

The service does not require UTC and does not depend on the Time Service. A
consumer may correlate monotonic timestamps with UTC elsewhere.

Sensor reads sharing a physical bus must use the platform's bus-sharing layer.
The Sensor Service schedules logical sources but does not implement I2C locking.

---

# Error Handling

The service shall return explicit configuration errors for:

* registry or threshold capacity exceeded
* duplicate `SensorId` or `ThresholdId`
* missing composite dependency
* dependency type mismatch
* incompatible threshold selector
* zero polling interval
* invalid dynamic-acceleration sampling rate or filter window

Runtime source errors are reflected in per-reading status and service
statistics. They do not panic the service or erase the last valid reading.

---

# Statistics

Publish or expose at least:

```rust
pub struct SensorServiceStats {
    pub polls_attempted: u64,
    pub polls_succeeded: u64,
    pub polls_failed: u64,
    pub composites_evaluated: u64,
    pub threshold_events_emitted: u64,
    pub event_queue_high_water: u16,
}
```

Per-source error counts remain available in each reading record or an associated
source-status record.

---

# Relationship to Other Components

## Sensor Drivers

Drivers own hardware and device-level conversion. The Sensor Service calls them
through adapters and owns polling policy, latest state, composites, and events.

## Storage and Logging Services

These services may consume snapshots or events. The Sensor Service does not
write files or logs directly.

## Power Management Service

Power policy may change polling intervals or disable selected sources through a
future command API. The initial service does not make those power decisions.

## Time Service

The Sensor Service uses monotonic timestamps only. UTC association remains the
Time Service's responsibility.

---

# Implementation Structure

Keep the service split into small modules, for example:

```text
crates/services/sensors/src/
    lib.rs
    types.rs
    registry.rs
    scheduler.rs
    composites.rs
    thresholds.rs
    service.rs
```

Device-specific adapters may live with the service or driver integration layer,
but calculations and policy must not be placed in hardware drivers.

---

# Testing

Add a service test under:

```text
servicetests/sensors/
```

Host-side tests with deterministic fake sources shall verify:

* default 10-second polling and per-source interval overrides
* independent scheduling of several sources
* multiple temperature sources remain distinct and queryable
* registry capacity and duplicate-ID errors
* latest valid values survive read failures and become stale/faulted correctly
* snapshot generation and watch publication
* tilt calculations for known gravity vectors
* level and tilted compass vectors using DT0058 reference cases
* heading wraparound near 0/360 degrees
* VeDBA and ODBA for known static and dynamic sequences
* rejection of an inadequate VeDBA/ODBA input rate
* absolute threshold crossing, hysteresis, clearing, and rearming
* delta thresholds using consecutive valid samples
* circular angular delta and stale-baseline handling
* ordered event delivery

The hardware test should register at least:

* LIS2HH12 acceleration and die temperature
* LIS2MDL magnetic field and die temperature
* one additional board temperature source if available
* tilt, basic heading, and tilt-compensated heading composites

It should print the latest snapshot at a human-readable rate and demonstrate a
safe threshold that can be triggered manually, such as a tilt-change event.

---

# Future Extensions

Possible later additions include:

* runtime extension-bus discovery
* interrupt-driven sources
* persistent calibration storage
* automatic magnetometer calibration
* multiple event subscribers
* aggregate temperature policies such as maximum board temperature
* sensor voting and source-quality selection
* adaptive polling controlled by power or activity state
* historical windows for additional derived metrics

These extensions should preserve stable sensor IDs and the distinction between
measurement kind and source identity.

---

# Consequences

## Advantages

* Applications receive one coherent latest-sensor snapshot.
* Multiple providers of the same measurement remain unambiguous.
* Polling, staleness, error handling, and thresholds are implemented once.
* Raw and composite sensors share a common interface.
* Heapless fixed capacities make RAM use explicit.
* `Watch` and `Channel` match latest-state and event-delivery semantics.

## Disadvantages

* A heterogeneous fixed-capacity registry requires explicit value and identity
  enums.
* Composite calculations require careful axis, calibration, timestamp, and
  validity handling.
* Exact event delivery can apply backpressure if its consumer stops draining the
  channel.
* VeDBA/ODBA require substantially faster polling than the service default.
* Compile-time capacities must be selected and tested for each application.

These trade-offs are acceptable because the service provides a single,
testable boundary between hardware-specific sensor drivers and application
policy while retaining deterministic embedded memory use.
