# GPS Driver Architecture (STM32U5 + Embassy + Rust)

## Overview

Implement a reusable GPS driver for an STM32U5 using Embassy. The driver should be designed as a long-lived async service that owns all GPS-related hardware and exposes data to the rest of the application through Embassy synchronization primitives.

The design should be modular, testable, `no_std` friendly, and avoid dynamic allocation. Use `heapless` where buffering is required.

Use unitsmoke/08_serial_gps and unitsmoke/09_serial_gps_pps as prototypes to build out your solution. Place the driver in crates\drivers\src\gps

The driver should own:

* GPS UART RX
* GPS UART TX (if required for module configuration)
* GPS PPS capture peripheral
* GPS enable/reset/power control pins

No other part of the application should access these peripherals directly.

The implementation should separate business logic from protocol parsing. In particular, NMEA parsing should be implemented in a separate module or library so that the driver itself is largely independent of the NMEA protocol. The driver should consume parsed events rather than contain parsing logic. This makes the driver reusable with different GPS modules or protocols in the future.

---

# Driver Structure

The driver should consist of three asynchronous tasks.

## 1. GPS Manager Task

This task implements the overall GPS state machine.

Responsibilities include:

* Powering the GPS on and off
* Managing search and standby timing
* Managing hot/warm/cold starts
* Tracking operating statistics
* Receiving commands
* Responding to successful fix acquisition
* Managing retries and search failures

This task should **not** perform UART parsing or PPS timestamp capture.

---

## 2. Serial RX Task

This task owns the UART receive peripheral.

Responsibilities:

* Continuously receive UART bytes
* Assemble complete NMEA sentences
* Validate sentence checksums
* Publish raw validated NMEA sentences for logging
* Pass validated sentences to the separate NMEA parser
* Receive parsed navigation events from the parser
* Update published GPS fix information
* Notify the GPS Manager when a valid fix is acquired

The UART task should not contain business logic relating to GPS operating modes, timers, or power management.

---

## 3. GPS PPS Task

This task owns the timer input capture peripheral connected to the GPS PPS output.

Responsibilities:

* Timestamp every PPS pulse using hardware input capture
* Maintain PPS count
* Compute interval between successive PPS events
* Publish PPS timing information
* Perform minimal work inside the interrupt path

The PPS task should not perform GPS state management or UART processing.

---

# NMEA Parsing

The NMEA parser should be implemented separately from the driver.

The parser should:

* Accept validated NMEA sentences
* Parse supported sentence types
* Produce typed navigation events
* Be independent of Embassy and STM32 peripherals
* Be testable on a desktop host

Initially only support:

* GGA
* RMC

Additional sentence types should be easy to add later.

The GPS driver should not contain protocol-specific parsing logic beyond framing complete NMEA sentences and validating checksums.

---

# Public API

The driver should expose:

```rust
GpsDriver::new(...)
GpsDriver::run()

GpsCommand::Start
GpsCommand::Stop
GpsCommand::ForceSearch
GpsCommand::ColdStart
GpsCommand::WarmStart
GpsCommand::HotStart
```

Configuration should be immutable after construction.

---

# Configuration

Suggested configuration:

```rust
pub struct GpsConfig {
    gps_on_time: Duration,                  // default 30 seconds
    gps_off_time: Duration,                 // default 5 minutes
    first_search_time: Duration,            // default 15 minutes
    search_time: Duration,                  // default 30 seconds
    search_failure_threshold: u32,          // default 10
    initial_start_mode: StartMode,          // Hot/Warm/Cold
}
```

Defaults:

* GPS On Time: 30 seconds
* GPS Off Time: 5 minutes
* First Search Time: 15 minutes
* Search Time: 30 seconds
* Search Failure Threshold: 10
* Initial Start Mode: Hot Start

---

# Operating State Machine

The driver should explicitly model its operating state.

Suggested states:

```rust
enum OperatingState {
    Off,
    PoweringOn,
    Searching,
    Acquired,
    Standby,
    PoweringOff,
    Error,
}
```

The GPS Manager task owns all state transitions.

---

# Published Data

## GPS Fix

Publish using `embassy_sync::watch`.

Only update when a new valid fix has been obtained.

Suggested structure:

```rust
pub struct GpsFix {
    latitude,
    longitude,
    utc_time,                  // GPS UTC time (1-second resolution)
    satellites,
    hdop,
    system_timestamp,          // Local monotonic timestamp when fix became valid
}
```

---

## PPS Information

Publish using `embassy_sync::watch`.

Suggested structure:

```rust
pub struct PpsInfo {
    pps_count,
    timestamp,                 // Local monotonic timestamp
    delta_time,
}
```

Target timing accuracy is approximately **1 microsecond** using STM32 timer input capture hardware.

---

## Driver Statistics

Publish using `embassy_sync::watch`.

Suggested fields:

```rust
pub struct GpsStats {
    powered,
    got_first_fix,
    operating_state,

    last_fix_attempt_time,
    last_successful_fix_time,

    num_fixes,

    total_on_time,
    total_off_time,

    num_search_attempts,
    num_search_failures,

    num_checksum_errors,
    num_uart_errors,

    num_pps_events,
}
```

---

# Raw NMEA Logging

Every validated NMEA sentence should be published to a logging interface together with the local monotonic timestamp at which the final byte of the sentence was received.

This should use an appropriate publish/subscribe mechanism (for example `PubSubChannel`) rather than a watch, since every message is significant.

---

# Time Synchronisation

Three independent time domains exist:

* GPS UTC time
* Local monotonic system time
* PPS hardware capture timestamps

The driver should associate the UTC second obtained from valid RMC/GGA messages with the corresponding PPS pulse so that GPS UTC time can be accurately mapped onto the local monotonic clock.

This mapping should support disciplining the local oscillator and compensating for drift.

---

# Error Handling

Maintain counters for:

* UART framing errors
* UART overrun errors
* Invalid NMEA checksum
* Buffer overflow
* PPS timeout
* GPS search timeout

Errors should not terminate the driver. The driver should continue operating whenever recovery is possible.

---

# Design Goals

The implementation should prioritise:

* Clear separation of responsibilities
* Modular architecture
* Minimal interrupt latency
* Zero-copy where practical
* No heap allocation
* Testable parser and business logic
* Extensibility to support additional GPS protocols in the future
* Comment clearly

When making architectural decisions, favour maintainability and clear ownership boundaries over minimising the number of modules or tasks.

----
# Test

Build a small test in drivertests/gpstest that would power up and check that the three tasks are working correctly in a minimal PoC using the default settings.