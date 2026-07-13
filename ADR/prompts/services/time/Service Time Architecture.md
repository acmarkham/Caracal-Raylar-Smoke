# ADR-0002: Time Service Architecture

## Status

Proposed

---

# Context

The system maintains a monotonic clock derived from the STM32U5 hardware timer. This clock:

* Starts at zero on every boot
* Never runs backwards
* Is monotonic
* Has no concept of UTC or calendar time
* Is derived from an HSE crystal with a nominal frequency error of approximately ±10 ppm
* Exhibits temperature-dependent drift and long-term ageing

Many services require absolute time rather than monotonic time:

* Storage Service (timestamp-based filenames and directory structure)
* Logging
* GPS
* Sensor timestamping
* Future RF/network protocols

These services should not perform time synchronisation themselves. Instead, a single Time Service should maintain the relationship between monotonic system time and UTC.

---

# Decision

Introduce a dedicated Time Service responsible for maintaining a continuously improving estimate of UTC time from one or more external timing anchors.

The Time Service owns the transformation between:

```text
System Time  <------->  UTC Time
```

It consumes timing information from external sources (initially GPS) and publishes a calibrated mapping for use by the rest of the application.

The Time Service does **not** own the monotonic clock itself. It only estimates its relationship to UTC.

---

# Design Goals

The Time Service shall:

* Maintain a bidirectional mapping between system time and UTC.
* Continuously refine this mapping as improved timing information becomes available.
* Compensate for oscillator frequency error and drift.
* Quantify confidence in the current time estimate.
* Continue operating while external timing sources are temporarily unavailable.
* Support future timing sources beyond GPS.

---

# Non-Goals

The Time Service is **not** responsible for:

* GPS communication
* PPS capture
* Calendar formatting
* Time zones
* Daylight Saving Time
* Leap second presentation
* Human-readable date formatting

Its responsibility is maintaining an accurate UTC timescale.

---

# Time Domains

The system contains three distinct notions of time.

## System Time

A monotonic timer represented by Embassy `Instant`.

Characteristics:

* Starts at zero
* Never decreases
* High resolution
* Local to the current boot
* No absolute meaning

---

## UTC Time

Represents the standard Unix epoch.

Representation:

```text
Seconds since 1970-01-01T00:00:00Z
+
Microseconds
```

Internally, UTC should be represented numerically rather than as calendar fields.

Conversion to calendar date/time is left to higher-level utilities.

---

## Anchor Time

An external observation relating UTC to a particular system timestamp.

Initially these anchors originate from GPS.

Future sources may include:

* RF synchronisation
* USB
* Network protocols
* Laboratory timing equipment

The Time Service should not assume GPS is the only possible source.

---

# GPS Integration

The GPS driver exposes several watches.

Relevant inputs are:

## Coarse Time

```rust
Watch<GpsFix>
```

This provides:

* UTC timestamp
* Local monotonic timestamp

Accuracy:

Approximately one second.

This establishes an initial relationship between local time and UTC.

---

## Fine Time

```rust
Watch<PpsInfo>
```

Initially the important field is:

```rust
timestamp: Instant
```

Future versions will additionally expose:

```rust
capture_ticks
```

derived from timer capture hardware operating at approximately 1 MHz.

This enables:

* Sub-microsecond alignment
* Frequency estimation
* Long-term oscillator calibration

The architecture should anticipate this extension without requiring redesign.

---

# Architecture

```text
                 GPS Driver
              ┌──────────────┐
              │ GpsFix Watch │
              │ PPS Watch    │
              └──────┬───────┘
                     │
                     ▼
             Time Service
     ┌──────────────────────────┐
     │ Anchor Manager           │
     │ Frequency Estimator      │
     │ UTC Mapper               │
     │ Uncertainty Estimator    │
     └──────────┬───────────────┘
                │
                ▼
        Watch<TimeState>

Application Services
    │
    ├── Storage
    ├── Logging
    ├── Sensors
    └── Communications
```

---

# Time Model

The Time Service maintains an estimate of:

```text
UTC = Offset + Scale × SystemTime
```

where:

* Offset relates local boot time to UTC.
* Scale compensates for oscillator frequency error.

Initially:

```text
Scale = 1.0
```

As additional anchors arrive, the estimate of Scale improves.

The service should be capable of modelling oscillator error at significantly better than parts-per-million resolution.

Long-term stability should target parts-per-billion estimation where measurement quality permits.

---

# Anchors

An anchor represents a known correspondence between:

```text
System Timestamp
↓

UTC Timestamp
```

Each anchor also carries an estimate of measurement quality.

The architecture should support multiple anchor sources.

Conceptually:

```text
GPS PPS
GPS NMEA
RF
USB
Future Sources
```

The estimator should not depend on GPS-specific types internally.

---

# Frequency Estimation

The local oscillator will exhibit:

* manufacturing tolerance
* temperature drift
* ageing

The Time Service should estimate frequency error continuously.

Initially, this may use a simple exponentially weighted moving average (EWMA).

Future estimators may use more sophisticated techniques without changing the public API.

The implementation should separate:

* measurement acquisition
* estimator
* published mapping

---

# Holdover

GPS will periodically be powered down.

During these periods:

* no new anchors arrive
* no PPS is available

The Time Service should continue estimating UTC using the most recent calibrated oscillator model.

As holdover duration increases, confidence decreases.

---

# Uncertainty

Every published UTC estimate should include an uncertainty estimate.

Uncertainty should increase while operating without external discipline.

Factors include:

* elapsed holdover duration
* estimated oscillator stability
* recent anchor quality

This allows downstream services to determine whether the current timestamp is suitable for their application.

The exact mathematical model is intentionally left open.

---

# Validity

Before the first anchor is acquired:

```text
UTC Valid = false
```

During this period:

* UTC conversions are unavailable.
* Services should not fabricate timestamps.
* Consumers must be able to determine that absolute time is unavailable.

Once sufficient anchors have been acquired:

```text
UTC Valid = true
```

Loss of GPS should **not** immediately invalidate UTC.

Instead, validity should depend on uncertainty exceeding a configurable threshold.

---

# Public API

Conceptually the service should support:

```rust
system_to_utc(Instant)

utc_to_system(UtcTimestamp)

current_utc()

time_state()
```

Conversions should fail cleanly if UTC is not currently valid.

---

# Published State

The Time Service should publish a watch containing its current state.

Suggested fields:

```rust
pub struct TimeState {
    utc_valid: bool,

    current_offset,

    estimated_frequency_error,

    uncertainty,

    last_anchor_system_time,

    last_anchor_utc,

    holdover_duration,

    active_time_source,
}
```

The precise representation may evolve.

---

# Multiple Time Sources

Although GPS is the initial source of timing information, the architecture should permit additional providers.

Each provider contributes anchors.

The Time Service determines:

* whether an anchor should be accepted
* how much weight it receives
* how it influences the current estimate

This allows future integration of:

* RF timing
* USB synchronisation
* Network time distribution
* Laboratory reference clocks

without changing consumer interfaces.

---

# Separation of Responsibilities

GPS Driver:

* acquires measurements
* timestamps PPS
* parses NMEA
* publishes observations

Time Service:

* estimates UTC
* estimates oscillator frequency
* computes uncertainty
* performs time conversions

Application Services:

* consume UTC
* never estimate time independently

---

# Consequences

## Advantages

* Single authoritative UTC mapping.
* Consistent timestamps throughout the system.
* Oscillator calibration is implemented once.
* GPS implementation remains independent of time estimation.
* Additional timing sources can be integrated without changing application code.
* Storage, logging and communications all consume the same notion of absolute time.

## Disadvantages

* The Time Service becomes a critical system dependency.
* Frequency estimation introduces algorithmic complexity.
* Correct uncertainty modelling requires careful validation.

These trade-offs are acceptable because accurate, system-wide timekeeping is fundamental to the application and centralising the estimation logic simplifies all downstream services.

# Tests

Write a small test suite in servicetests/time that will start the GPS and printout the system time, utc time, validity, drift etc every second
