# Storage Service and Stream-Based Storage Model

# Context

The storage driver provides low-level filesystem operations over ExFAT and SDMMC. It is intentionally unaware of application concepts such as log files, audio recordings, or GPS timing data.

Several independent application services require persistent storage:

* System logging
* Audio capture
* GPS timing
* Accelerometer
* Environmental sensors
* Future sensor types

These services should not manage filenames, directory structures, filesystem interaction, or recovery after reboot. Producers do own the domain-specific policy that determines when each logical stream begins and ends.

The application exhibits only a small number of storage patterns, and these patterns are stable. The system does not require a highly configurable storage framework.

---

# Decision

Introduce a **Storage Service** that owns the Storage Driver and exposes lightweight append-oriented **streams**.

A stream represents one complete logical destination for sequential data. Its producer owns its lifetime, while the Storage Service materializes it as a filesystem object.

The Storage Service is responsible for:

* Creating streams
* Materializing streams as filesystem objects
* Filename generation
* Directory hierarchy
* Filesystem interaction
* Buffering
* Flushing
* Retries
* Closing streams

The Storage Service is not responsible for:

* Recording duration
* Stream segmentation
* Recording policies
* Media formats
* Metadata

Storage persists complete logical streams but does not determine their boundaries.

Clients never manipulate filenames directly.

UTC timestamps are in unix time (i.e. integer seconds), not in human calendar time. UTC timestamps are obviously in Zulu time (timezone: zero) so there is no confusion about local timezones or daylight savings.

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
let stream = storage.begin_stream(
    StreamKind::Audio,
    StorageLayout::HourlyFolders,
);
```

The client subsequently performs:

```rust
stream.write(...)
stream.flush()
stream.finish()
```

The client does **not** know:

* current filename
* directory structure
* whether a file has been reopened after restart

The producer does know when its logical stream begins and ends.

---

# Storage Layout

Although producers own stream lifetime, Storage owns how streams are represented within the filesystem.

Each `begin_stream` request includes a layout such as `Flat`, `DailyFolders`, `HourlyFolders`, or `MissionFolders`. The selected layout controls directory hierarchy, filename generation, filename uniqueness, and filesystem-specific conventions.

The producer never receives or constructs the resulting path. Filesystem organization can therefore evolve independently of producer lifecycle policy.

The Storage Service also exposes the read-only physical-media identity captured
by the Storage Driver. This accessor performs no new SD transaction and returns
a small copied snapshot, allowing the Identity and Versioning Service to obtain
card CID/CSD metadata without bypassing Storage's ownership of the driver.

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
* Continue appending to the restart-safe log for the lifetime selected by the Logging Service

Typical usage:

```text
System Logger
      │
      ▼
syslog.txt
```

---

## Audio Stream

Purpose:

Continuous audio recording.

Characteristics:

* One logical WAV stream per producer-selected recording interval
* Producer-owned segmentation
* Files organised into daily folders
* File names derived from UTC timestamps
* Folder names derived from UTC midnight epoch
* If the system starts, it will commence a file from the top of the minute (e.g. aud_1784016480.wav->Tue Jul 14 2026 08:08:00 GMT+0000) and fill it until it reaches the normal top of the hour mark (e.g. aud_1784019600.wav -> Tue Jul 14 2026 09:00:00 GMT+0000).
* The producer can change its recording interval independently of the selected folder layout

Example:

```text
1783987200/
            aud_1783987200.wav
            aud_1783990800.wav
            aud_1783994400.wav
            aud_1784016480.wav
            aud_1784019600.wav
```

The recording service appends audio blocks and finishes the stream at an exact sample boundary. It then begins a new stream using the desired storage layout.

---

## GPS Timing Stream

Purpose:

Persistent PPS timing information and optional NMEA sentence storage

Characteristics:

* One file per UTC day
* Stored within a daily directory
* Append-only
* It should be easy to change the folder interval (e.g. from daily to hourly)

Example:

```text
1784019600/

    gps_1784019600.pps
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

Producer reaches its logical boundary

↓

Finish stream

↓

Begin the next stream when required
```

Storage never sends rollover events. Producers explicitly finish completed streams.

---

# File Naming Policy

File naming is owned entirely by the Storage Service.

Clients never generate filenames.

This allows naming conventions to evolve independently of application code.

Current policy:

| Stream     | Naming                              |
| ---------- | ----------------------------------- |
| Log        | Single append-only logfile          |
| Audio      | UTC timestamped hourly WAV with UTC day directory |
| GPS Timing | Daily file within UTC day directory |

---

# Directory Structure

The Storage Service creates directories automatically.

Current convention:

```text
/
syslog.txt
/1783987200/
            aud_1783987200.wav
            gps_1783987200.pps
            aud_1783990800.wav
            aud_1783994400.wav
            aud_1784016480.wav
            aud_1784019600.wav
            
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

Audio files on startup will commence on the top of the minute, so there is no need to open/append new files on startup.

---

## GPS Timing

Locate today's PPS file.

Append if it exists.

Create if necessary.

---

# Stream Lifecycle

Logical stream boundaries are owned by the producer.

For audio, the Audio Recorder Service determines boundaries using sample count, sample rate and recording policy. At a boundary it finalises the current container, finishes the Storage stream, requests a new stream, and starts the next recording.

Storage does not request or perform stream rotation. It creates, buffers, flushes and closes the filesystem object corresponding to each producer-owned logical stream.

---

# Time Dependency

The Storage Service depends on UTC time.

UTC is required for:

* folder naming
* filename generation
* selecting timestamp-derived directory and filename components when a stream begins

Time is expected to come from the system time service, which is disciplined by GPS.

Flat, restart-safe streams such as the system log do not require UTC and may start immediately. A timestamped stream cannot begin until UTC is available; its producer decides whether to wait, retry, or omit that logical stream.

---

# Driver Independence

The Storage Driver remains unaware of:

* UTC
* stream lifecycle
* folders
* file naming
* log semantics
* audio
* GPS timing

It only provides filesystem primitives.

This separation keeps the driver reusable and simplifies testing. 

UTC timing is provided by the time service which has already been implemented and provides bidirectional mappings between system time and UTC time using GPS informed scaling.


--
# Implementation

Implement as a service in 'crates/services/storage' to sit alongside the existing 'time' service. 


# Testing

Provide two small test suites in servicetests/storage that will:
- Start the GPS driver ('crates/drivers/gps') and time service ('crates/services/time') to provide UTC time
- Write fake data to logfile every second
- Write GPS data (which can be listened to from the gps driver) to the PPS file 
- Write fake audio data at nominal 16kHz 16 bit depth while the test producer explicitly finishes minute-long streams. Use hourly folder names and wait until UTC time is valid before beginning audio streams.

---

# Consequences

## Advantages

* Sample-accurate recording boundaries are possible.
* Producers determine stream boundaries using domain-specific knowledge.
* Storage no longer requires lifecycle callbacks.
* Filesystem organization remains centralized.
* Naming conventions remain consistent.
* Recovery logic exists in one location.
* Filesystem code remains independent of application semantics.
* Stream lifecycle and filesystem representation are cleanly separated.
* The Storage API is simpler and more generic.

## Disadvantages

* Stream segmentation logic moves into each producer.
* Producers requiring automatic segmentation must implement their own lifecycle policy.
* Changes to naming conventions require modifications to the Storage Service.

These trade-offs preserve centralized filesystem policy while keeping stream semantics in the producing domain.
