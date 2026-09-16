# ADR-0012: Location Service Architecture

## Status

Proposed

---

# Context

The existing GPS Driver owns the GPS hardware and is responsible for:

* UART reception
* NMEA parsing
* PPS capture
* GPS power cycling
* fix acquisition
* publishing GPS fixes
* publishing timing information
* publishing GPS operating statistics

The GPS Driver already exposes valid GPS fixes containing latitude, longitude, UTC time, satellite count, HDOP and the local system timestamp at which the fix was observed.

Most devices are expected to be statically placed after deployment. Therefore, higher-level application services usually require a stable estimate of device location rather than a continuous high-rate navigation stream.

A thin Location Service should sit above the GPS Driver and provide a filtered, stable, application-facing view of location.

---

# Decision

Introduce a dedicated Location Service.

The Location Service subscribes to the GPS Driver's fix output and publishes a stable estimate of device location.

The first implementation should be intentionally small:

* consume valid GPS fixes
* maintain a bounded history of recent positions
* compute a median-filtered latitude/longitude estimate
* publish the current location state
* expose validity and uncertainty information

The Location Service does not control the GPS hardware directly.

---

# Design Goals

The Location Service shall:

* Be thin.
* Be heapless.
* Avoid dynamic allocation.
* Depend on the GPS Driver for raw fixes.
* Publish a stable application-facing location estimate.
* Be suitable for statically deployed devices.
* Reject or de-weight obviously poor fixes.
* Preserve enough metadata to make the location estimate auditable.
* Support future non-GPS location sources if required.

---

# Non-Goals

The Location Service is **not** responsible for:

* GPS UART handling
* NMEA parsing
* PPS handling
* GPS power control
* GPS duty cycling
* time synchronisation
* map projection
* route planning
* geocoding
* human-readable address lookup

Those responsibilities belong to other components.

---

# Architecture

```text
             GPS Driver
        Watch<GpsFix>
               │
               ▼
        Location Service
 ┌──────────────────────────┐
 │ Fix Intake               │
 │ Quality Gate             │
 │ Median Filter            │
 │ Location State Publisher │
 └─────────────┬────────────┘
               │
               ▼
      Watch<LocationState>

Application Services
   │
   ├── Storage Service
   ├── Logging Service
   ├── Identity / Traceability
   ├── Routing / Communication
   └── Diagnostics
```

---

# GPS Input

The Location Service consumes GPS fixes from the existing GPS Driver.

Conceptually:

```rust
pub struct GpsFix {
    pub latitude: Coordinate,
    pub longitude: Coordinate,
    pub utc_time: UtcDateTime,
    pub satellites: u8,
    pub hdop_centi: Option<u16>,
    pub system_timestamp: Instant,
}
```

Only valid fixes should be processed.

The GPS Driver remains the authority on whether a fix is valid.

---

# Location Output

The Location Service should publish a `LocationState` using an Embassy watch.

Suggested structure:

```rust
pub struct LocationState {
    pub valid: bool,

    pub latitude: Coordinate,
    pub longitude: Coordinate,

    pub source: LocationSource,

    pub fix_count_used: u8,
    pub total_fix_count_seen: u64,

    pub last_fix_system_time: Instant,
    pub last_fix_utc_time: Option<UtcTimestamp>,

    pub hdop_centi: Option<u16>,
    pub satellites: Option<u8>,

    pub uncertainty_meters: Option<u32>,
}
```

Suggested source enum:

```rust
pub enum LocationSource {
    None,
    Gps,
    Manual,
    FutureExternal,
}
```

The first implementation may only support `None` and `Gps`.

---

# Filtering Model

Because devices are expected to be statically deployed, the service should prioritise stability over responsiveness.

The initial algorithm should use a bounded median filter over recent valid GPS fixes.

Conceptually:

```text
recent valid GPS fixes
        │
        ▼
quality filtering
        │
        ▼
median latitude
median longitude
        │
        ▼
published stable location
```

The filter should use fixed-capacity storage.

Suggested initial capacity:

```text
5 to 15 fixes
```

The exact value should be configurable at compile time or construction time.

---

# Quality Gate

The Location Service should discard obviously poor fixes before they enter the median filter.

Possible criteria:

* minimum satellite count
* maximum HDOP
* valid latitude/longitude range
* stale fix timestamp
* optional maximum jump distance from current estimate

Initial defaults may be conservative.

Example conceptual thresholds:

```text
minimum satellites: 4
maximum HDOP: configurable
```

The exact thresholds should be documented during implementation.

---

# Static Deployment Assumption

The first implementation assumes that the device is normally stationary.

This affects the design:

* median filtering is preferred over fast tracking
* sudden jumps may be rejected
* the service does not need a velocity model
* the service does not need route or movement estimation

If future products require moving devices, this service may need a different filtering mode.

---

# Validity

Before enough acceptable fixes have been collected:

```text
Location valid = false
```

After the minimum number of acceptable fixes has been collected:

```text
Location valid = true
```

Validity should not necessarily be revoked immediately when GPS is powered down.

Instead, the service may retain the last known valid location and mark it as stale if required.

Suggested state distinction:

```text
valid = true

stale = true / false
```

A future revision may add explicit staleness tracking.

---

# Uncertainty

The Location Service should expose an approximate uncertainty estimate.

For the first implementation, this may be derived from:

* median filter spread
* HDOP
* satellite count
* age of last accepted fix

The uncertainty estimate does not need to be survey-grade.

It is intended to help downstream services decide whether the location is good enough for their use case.

---

# Relationship to Time Service

The Location Service does not maintain UTC time.

It may preserve timestamps associated with accepted GPS fixes, but UTC conversion and system-to-UTC mapping remain the responsibility of the Time Service.

---

# Relationship to Storage Service

The Storage Service may use location metadata in future file headers, metadata files or directory-level manifests.

The Location Service should not depend on the Storage Service.

---

# Relationship to Logging Service

The Logging Service may log location acquisition, rejection of poor fixes and changes in location validity.

The Location Service should not depend on the Logging Service directly unless a standard logging-handle pattern is already established for services.

---

# Relationship to Traceability / Identity

The Traceability Driver provides device identity.

The Location Service may provide geographic context for device identity.

Together they can support geographically informed identity or deployment records, for example:

```text
device serial + approximate deployment location
```

The Location Service should not modify or own device identity.

---

# Relationship to Routing / Communication

Future routing or communication services may use location to make geographically informed decisions.

Examples:

* choosing a regional gateway
* selecting RF routing parameters
* annotating telemetry
* applying deployment-region behaviour

The Location Service should provide the stable location estimate but should not implement routing policy.

---

# Public API

Conceptual API:

```rust
pub fn init(gps_fixes: WatchReceiver<GpsFix>) -> LocationService

impl LocationService {
    pub async fn run(&mut self) -> !;

    pub fn state(&self) -> LocationState;

    pub fn watch(&self) -> WatchReceiver<LocationState>;
}
```

Exact API details may change to match the rest of the service architecture.

---

# Error Handling

The service should handle the following without panicking:

* GPS fixes unavailable
* GPS not yet acquired
* invalid coordinates
* insufficient accepted fixes
* stale fixes
* poor HDOP
* insufficient satellites

The service should continue operating and publish an invalid or stale state as appropriate.

---

# Test

Build a small test in:

```text
servicetests/location
```

The test should:

* start the GPS Driver
* start the Location Service
* subscribe to the published `LocationState`
* print or log the raw GPS fix and filtered location estimate periodically
* verify that `valid` becomes true after enough acceptable fixes are collected
* verify that the median-filtered position remains stable across successive fixes

For a hardware-in-the-loop test, the board should be placed where GPS reception is available.

For a host-side or simulated test, feed deterministic fake `GpsFix` values into the Location Service and verify that:

* invalid fixes are ignored
* outliers are rejected or suppressed by the median filter
* the published median latitude/longitude is correct
* `fix_count_used` behaves as expected

---

# Future Extensions

Possible future additions include:

* manual location override
* persistent last-known location
* multiple location sources
* movement-aware filtering
* geofence support
* deployment metadata file generation
* location confidence scoring
* regional configuration selection
* geographic routing support

These features should not complicate the initial thin service.

---

# Consequences

## Advantages

* Provides a stable application-facing location estimate.
* Keeps GPS hardware and parsing concerns inside the GPS Driver.
* Matches the static deployment assumption.
* Reduces duplicated location filtering logic in downstream services.
* Provides a natural point for future geographic metadata and routing logic.
* Remains small and testable.

## Disadvantages

* Median filtering is not suitable for fast-moving devices.
* Location validity depends on receiving enough acceptable GPS fixes.
* Uncertainty estimation is approximate.
* A thin service may need extension if location becomes a more central application concept.

These trade-offs are acceptable because the current deployment model assumes static devices and only requires a stable location estimate rather than continuous navigation.
