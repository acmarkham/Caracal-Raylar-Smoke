# Logging Service Architecture


## Constraints
Read
ADR\common\AGENTS.md
before planning and implementing

---

# Context

The firmware consists of a number of independent drivers and services including:

* GPS Driver
* Time Service
* Storage Service
* Audio
* Sensor Drivers
* Future communication services

Every component requires a simple mechanism for recording diagnostic information.

Logging serves several purposes:

* Debugging during development
* Diagnosing failures in deployed devices
* Providing useful information for non-technical users
* Recording important system events

The logging system should be simple enough that developers naturally add logging to new components without needing to understand the underlying storage implementation.

---

# Decision

Introduce a dedicated Logging Service.

The Logging Service owns a log stream provided by the Storage Service and exposes lightweight logging handles to application components.

Application components never interact directly with files or the Storage Service.

The Logging Service is responsible for:

* Formatting log records
* Assigning sequence numbers
* Timestamping messages
* Appending formatted text to the log stream
* Maintaining logging statistics

---

# Design Goals

The Logging Service shall:

* Be heapless.
* Require no dynamic allocation.
* Produce human-readable log files.
* Be lightweight enough to use throughout the codebase.
* Require minimal RAM.
* Be easy to integrate into new drivers and services.
* Preserve message ordering.
* Detect dropped log messages.

---

# Non-Goals

The Logging Service is **not** responsible for:

* Log rotation
* File naming
* Filesystem interaction
* Time synchronisation
* Binary trace collection
* Remote logging

These responsibilities belong elsewhere.

---

# Architecture

```text
Application Components
        │
        ▼
  Logging Handles
        │
        ▼
  Logging Service
        │
        ▼
Storage Stream (Log)
        │
        ▼
 Storage Service
        │
        ▼
 Storage Driver
```

The Logging Service owns exactly one log stream.

The Storage Service determines where this stream is stored.

---

# Log Handles

Each component owns a lightweight logging handle.

Conceptually:

```rust
let log = logging.register("GPS");
```

The handle stores only a static component name.

```rust
struct LoggerHandle {
    component: &'static str,
}
```

This avoids:

* allocation
* registration tables
* global component enumerations

Adding a new component requires no changes outside that component.

Typical examples:

```text
GPS
Storage
Time
Audio
Accelerometer
Battery
```

---

# Log Levels

Support the standard embedded log levels.

```text
Trace
Debug
Info
Warn
Error
```

These closely match the existing `defmt` model and should feel familiar to developers.

---

# Log Format

Logs are stored as plain UTF-8 text.

This is a deliberate design decision.

The primary consumers of log files are expected to be:

* developers
* field engineers
* non-technical users emailing SD card contents

Human-readable logs remove the need for offline decoding tools.

A typical log line might resemble:

```text
0000001234 3812.491 INFO  GPS        First fix acquired (8 satellites)
```

Each line contains:

* sequence number
* system timestamp
* log level
* component name
* formatted message

Additional fields may be added later if required.

---

# Timestamp

Every log entry records the local monotonic system timestamp.

The Logging Service does **not** attempt to convert timestamps into UTC.

Reasons:

* system time always exists
* logging functions before GPS acquisition
* conversion to UTC can be performed later using the Time Service if required

This keeps logging independent of absolute time.

---

# Sequence Numbers

Each message receives an incrementing sequence number.

Purpose:

* detect dropped messages
* identify missing log entries
* simplify debugging

Example:

```text
1051
1052
1054
```

Immediately indicates that one message was lost.

Sequence numbers are maintained solely by the Logging Service.

---

# Message Flow

Typical flow:

```text
GPS Driver

↓

LoggerHandle

↓

Logging Service

↓

Storage Stream

↓

Storage Service
```

Application components never interact directly with files.

---

# Buffering

Logging is expected to be relatively low bandwidth.

Typical events include:

* startup
* shutdown
* GPS acquisition
* warnings
* errors
* configuration changes

The Logging Service should therefore avoid maintaining a large in-memory queue.

Instead:

* maintain a very small fixed-capacity queue (approximately 4–8 records)
* rely on the Storage Service for the majority of buffering
* minimise RAM usage

If the queue becomes full:

* increment a dropped message counter
* discard the newest message

The exact queue depth remains configurable.

---

# Formatting

The Logging Service should provide a lightweight logging interface that closely resembles the ergonomics of `defmt`.

The goal is to minimise friction when adding logging to drivers and services while remaining completely heapless.

Typical usage should resemble:

```rust
log.trace!("Received {} bytes", len);

log.debug!("Current HDOP = {}", hdop);

log.info!("GPS fix acquired ({} satellites)", sats);

log.warn!("GPS signal lost");

log.error!("SD card write failed: {:?}", error);
```

The formatting implementation should:

* require no heap allocation
* avoid constructing temporary `String` objects
* support formatting of common primitive types
* support `core::fmt`-style formatting where practical
* produce directly formatted UTF-8 log lines

The preferred implementation is to expose logging macros rather than methods, allowing source location and compile-time log filtering to be added later without changing the calling syntax.

For example, the ideal macros would be virtually seamless for defmtt, allowing users to seamlessly redirect or duplicate logging destination without congnitive load.

```rust
info!(log, "GPS fix {}", sats);
warn!(log, "...");
error!(log, "...");
```

or an equivalent API that is similarly concise.

The exact implementation is intentionally left open, provided it offers an experience similar to `defmt` while remaining heapless.

Formatting should occur only once, inside the Logging Service, immediately before appending the completed log line to the Storage Service.

The formatted output should remain plain UTF-8 text suitable for direct inspection by users.

---

# Statistics

The Logging Service should publish statistics.

Suggested fields:

```rust
pub struct LoggingStats {
    total_messages,
    dropped_messages,
    queue_depth,
    maximum_queue_depth,
    bytes_written,
}
```

These statistics simplify performance tuning and diagnosing overload conditions.

---

# Error Handling

Logging failures should never terminate application code.

Possible failures include:

* storage unavailable
* stream closed
* queue full

Failures should be recorded internally whenever possible.

Applications should continue operating even if logging becomes unavailable.

---

# Relationship to the Time Service

The Logging Service depends only on monotonic system time.

It does **not** require UTC.

This allows logging to operate:

* before GPS acquisition
* during GPS outages
* during oscillator holdover

Absolute timestamps may be reconstructed later using the Time Service if necessary.

---

# Relationship to the Storage Service

The Logging Service owns a single log stream created by the Storage Service.

Conceptually:

```rust
let stream = storage.create_stream(StreamType::Log);
```

The Logging Service performs only append operations.

It has no knowledge of:

* filenames
* directory layout
* rollover policy
* filesystem implementation

These remain entirely within the Storage Service.

---

# Future Extensions

Possible future additions include:

* configurable log filtering
* compile-time log level removal
* remote log streaming
* crash log preservation
* panic logging
* coloured terminal output for host tools

These features should not complicate the initial implementation.

---

--
# Implementation

Implement as a service in 'crates/services/logging' to sit alongside the existing 'time'  and 'storage' services. 

Maximum message length should be a user defined parameter e.g. 256 bytes. Messages longer than this should safely truncate and not be dropped.


# Testing

Provide a small test suite that will have three different "components" logging messages with different levels of severity and showing formatting works. 

---


# Consequences

## Advantages

* Extremely simple API for application developers.
* Human-readable log files suitable for field diagnostics.
* No heap allocation.
* Very small RAM footprint.
* No central component registry.
* Independent of filesystem implementation.
* Independent of UTC availability.

## Disadvantages

* Text logs consume more storage than binary logs.
* Formatting is more CPU intensive than binary encoding.
* A small queue means excessive logging may result in dropped messages.

These trade-offs are acceptable because logging is expected to be relatively low bandwidth, while ease of debugging and field support are considered higher priorities than maximum logging throughput.
