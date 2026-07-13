# ADR-0001: Storage Service and Stream-Based Storage Model

## Status

Proposed

---

# Context

The storage driver provides low-level filesystem operations over ExFAT and SDMMC. It is intentionally unaware of application concepts such as log files, audio recordings, or GPS timing data.

Several independent application services require persistent storage:

* System logging
* Audio capture
* GPS timing
* Accelerometer
* Environmental sensors
* Future sensor types

These services should not manage filenames, directory structures, file rollover, or recovery after reboot. Those concerns are common across multiple services and belong in a higher-level abstraction.

The application exhibits only a small number of storage patterns, and these patterns are stable. The system does not require a highly configurable storage framework.

---

# Decision

Introduce a **Storage Service** that owns the Storage Driver and exposes lightweight append-oriented **streams**.

A stream represents a logical destination for sequential data. The Storage Service manages the lifecycle of the underlying files while clients simply append data.

The Storage Service is responsible for:

* Opening files
* Closing files
* File rollover
* Directory creation
* Naming files
* Recovery after reboot
* Mapping logical streams onto physical files

Clients never manipulate filenames directly.

---

# Architecture

```text
Application Services
        │
        ▼
Storage Service
        │
        ▼
Storage Driver
        │
        ▼
ExFAT
        │
        ▼
SDMMC
```

The Storage Service is the sole owner of the Storage Driver.

This ensures:

* Single ownership of filesystem state
* Simple concurrency
* Consistent file management
* Centralised storage policy

---

# Stream Model

Clients create streams rather than files.

Conceptually:

```rust
let stream = storage.create_stream(StreamType::Audio);
```

The client subsequently performs:

```rust
stream.append(...)
stream.flush()
stream.close()
```

The client does **not** know:

* current filename
* directory structure
* rollover policy
* whether a file has been reopened after restart

---

# Stream Archetypes

Rather than allowing arbitrary policies, the service provides a small number of predefined stream types.

These correspond directly to the application's storage patterns.

## Log File

Purpose:

Continuous append-only system log.

Characteristics:

* Single logical log
* Append-only
* Flush periodically
* On startup, locate the existing log and continue appending
* Create a new log only if none exists or rollover policy requires it

Typical usage:

```text
System Logger
      │
      ▼
system.log
```

---

## Audio Stream

Purpose:

Continuous audio recording.

Characteristics:

* One WAV file per hour
* Automatic rollover
* Files organised into daily folders
* File names derived from UTC timestamps
* Folder names derived from UTC midnight epoch

Example:

```text
20260713/

    20260713T140000.wav

    20260713T150000.wav

    20260713T160000.wav
```

The recording service simply appends audio blocks.

The Storage Service performs rollover automatically.

---

## GPS Timing Stream

Purpose:

Persistent PPS timing information.

Characteristics:

* One file per UTC day
* Stored within a daily directory
* Append-only

Example:

```text
20260713/

    pps.bin
```

or

```text
20260713/

    pps.csv
```

The precise filename is determined by the storage policy.

---

## Future Sensor Streams

Additional stream archetypes may be introduced as required.

Examples:

* Accelerometer
* Magnetometer
* Environmental sensors

These are expected to follow the same append-oriented design.

---

# Stream Ownership

Each stream has exactly one owner.

The owner:

* creates the stream
* appends data
* flushes
* closes

Streams are not shared between tasks.

Shared logging should instead be implemented by the owning service (for example, a central logger that aggregates messages before writing).

---

# Stream Lifetime

Typical lifecycle:

```text
Create

↓

Open underlying file

↓

Append

↓

Append

↓

Flush (optional)

↓

Automatic rollover (if required)

↓

Continue appending

↓

Close
```

Clients are unaware of rollover events.

---

# File Naming Policy

File naming is owned entirely by the Storage Service.

Clients never generate filenames.

This allows naming conventions to evolve independently of application code.

Current policy:

| Stream     | Naming                              |
| ---------- | ----------------------------------- |
| Log        | Single append-only logfile          |
| Audio      | UTC timestamped hourly WAV          |
| GPS Timing | Daily file within UTC day directory |

---

# Directory Structure

The Storage Service creates directories automatically.

Current convention:

```text
/

logs/

audio/

gps/

20260713/
```

The exact hierarchy remains an implementation detail.

Clients should not depend on it.

---

# Recovery

The Storage Service is responsible for startup recovery.

Examples:

## Log Stream

Locate the existing logfile.

Open for append.

Continue writing.

---

## Audio Stream

Determine whether the previous recording should continue or whether a new hourly file should be created.

---

## GPS Timing

Locate today's PPS file.

Append if it exists.

Create if necessary.

---

# Rollover

The Storage Service owns rollover policy.

Examples:

Audio:

```text
14:59:59

↓

15:00:00

↓

Close previous WAV

↓

Open new WAV

↓

Continue recording
```

The recording service is unaware this occurred.

---

# Time Dependency

The Storage Service depends on UTC time.

UTC is required for:

* folder naming
* filename generation
* rollover decisions

Time is expected to come from the system time service, which is disciplined by GPS.

If UTC is unavailable during startup, the Storage Service should define a deterministic fallback behaviour (to be decided during implementation).

---

# Driver Independence

The Storage Driver remains unaware of:

* UTC
* rollover
* folders
* file naming
* log semantics
* audio
* GPS timing

It only provides filesystem primitives.

This separation keeps the driver reusable and simplifies testing.

---

# Consequences

## Advantages

* Application services become extremely simple.
* Storage policy is centralised.
* Naming conventions remain consistent.
* Recovery logic exists in one location.
* Filesystem code remains independent of application semantics.
* New sensor types require minimal code.

## Disadvantages

* The Storage Service becomes responsible for all storage policy.
* Changes to naming conventions require modifications to the Storage Service.
* Additional stream archetypes require explicit implementation rather than configuration.

These trade-offs are acceptable because the application has a small number of well-defined storage patterns, and simplicity is preferred over a highly generic storage framework.
