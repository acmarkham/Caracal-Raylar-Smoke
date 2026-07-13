# Storage Driver Design

## Overview

The storage driver provides a reusable abstraction over the SDMMC peripheral and the ExFAT filesystem (via `exfat-slim`). It is intended to be the only component that directly interacts with the filesystem implementation. Higher-level services communicate with the storage driver through a separate storage service, which will provide stream management, ownership, and multiplexing of requests.

The storage driver is designed for an embedded `no_std` environment running on STM32U5 using Embassy. The primary workload is **high-throughput append-only writes** to a small number of files, with relatively infrequent file reads. The driver should not dynamically allocate memory after initialisation.

The driver should prioritise:

* High write throughput
* Low RAM usage
* Zero-copy operation where practical
* Predictable behaviour
* Simple ownership model
* Recoverability after unexpected power loss

---

# Architectural Constraints

- The driver should be implemented as a thin abstraction over exfat-slim.
- Do not modify or duplicate filesystem logic already implemented by exfat-slim.
- Avoid generic filesystem abstractions that support arbitrary file operations; optimise for append-only workloads.
- Keep the implementation modular with separate modules for mounting, directory management, read, write, handles, and errors.
- Do not introduce dynamic allocation after initialisation.
- Do not retain ownership of caller buffers beyond the duration of a single API call.
- Favour explicit state and simple ownership over abstraction.

# Scope

This driver is responsible for:

* SD card access (through existing SDMMC layer)
* ExFAT filesystem interaction
* Directory creation
* File creation
* File open/close
* Append operations
* Read operations
* File flushing
* Managing a small number of simultaneously open files

This driver is **not** responsible for:

* Scheduling writes from multiple producers
* Stream ownership
* Log formatting
* CSV generation
* WAV formatting
* File naming policy
* Rotation of log files
* Background buffering

Those responsibilities belong to the higher-level Storage Service.

---

# Existing Work

The following functionality already exists and should be reused where possible:

* SDMMC read/write prototype (unitsmoke/11_sdmmc)
* ExFAT VBR discovery (unitsmoke/29_exfat_read)
* Basic file read/write prototype (unitsmoke/30_exfat_write)

The storage driver should use these components as stubs when writing the driver.

The exfat-slim crate has been locally patched to handle large (>128kib) clusters. Use this patched crate.

---
Create the driver under:
crates\drivers\src\storage

There is another existing driver under 
crates\drivers\src\gps
if you need to make consistent structure and naming


---

# Design Goals

The dominant workload is append-only writes.

Typical examples include:

* System logs
* Audio recordings (.wav)
* Accelerometer logs (.csv)
* GPS logs
* Sensor output

Reads are required but are considered a secondary use case.

The implementation should optimise for sequential writes.

---

# Assumptions

The higher-level Storage Service owns the Storage Driver.

The Storage Service guarantees:

* Buffers remain valid for the duration of the call.
* Buffers are aligned appropriately.
* Buffers are multiples of the filesystem block size (512 bytes).

The driver therefore should avoid copying caller buffers whenever possible.

Zero-copy operation is preferred.

---

# Concurrency Model

The Storage Driver is intended to have a single owner.

Multiple application services will never call the driver directly.

Instead:

```
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

This significantly simplifies locking and ownership.

The driver itself does not need to implement complicated multi-client synchronisation.

---

# File Model

The driver supports:

* Up to four simultaneously open files for writing.
* One open file for reading.

This matches expected application behaviour while keeping internal bookkeeping straightforward.

Reading and writing are independent operations.

---

# Typical Workloads

## Log Files

Characteristics:

* Small append operations
* Often less than one filesystem block
* Long idle periods
* Flushes requested periodically

Typical sequence:

```
Open

Append

Append

Append

Flush

Append

Close
```

---

## Sensor Files

Characteristics:

* Large sequential writes
* Continuous streaming
* Few flushes
* Closed when recording ends

Example:

```
Open audio.wav

Append 64 KiB

Append 64 KiB

Append 64 KiB

...

Close
```

---

# Driver API

The driver should expose functionality roughly equivalent to:

```rust
create_directory(path)

create_file(path)

open_for_append(path)

append(handle, &[u8])

flush(handle)

close(handle, valid_bytes_last_block)

open_for_read(path)

read(handle, &mut [u8])

close_read(handle)
```

This is conceptual rather than a fixed API.

The final interface should follow idiomatic Rust ownership. The storage driver owns all filesystem objects. Clients only hold opaque file handles (e.g. FileHandle(u8)). Handles are invalid after Close().

---

# Append Semantics

Append is the primary operation.

Requirements:

* Writes occur sequentially.
* Existing file contents are never modified except where required by filesystem metadata.
* The caller provides buffers in multiples of 512 bytes.
* The driver should avoid copying buffers.
* The driver never supports arbitrary seek or overwrite operations

Append operations complete synchronously from the caller's perspective. The driver does not retain references to caller buffers after append() returns.
All public operations are async because SD card access may block.

The state machine for append is roughly:

Created

↓

Opened

↓

Append*

↓

Flush (optional)

↓

Append*

↓

Close

↓

Closed

Illegal:

* Append after Close
* Flush after Close
* Close twice


---

# Partial Final Block

Files frequently end on a non-block boundary.

Examples:

* WAV file final block
* Log file
* CSV output

When closing a file, the caller specifies how many bytes of the final block contain valid data.

For example:

```
512-byte block

Only first 100 bytes valid
```

The driver must ensure that:

* File size reflects only valid data.
* ExFAT metadata records the correct file length.
* Unused bytes are ignored.

---

# Flush

Flush should be supported independently of close.

Motivation:

Log files may receive infrequent updates.

Waiting for close before committing data risks excessive data loss after unexpected power removal.

Flush should:

* Commit pending file data
* Update filesystem metadata as required
* Leave the file open

---

# Read Operations

Reading is a secondary use case.

Requirements:

* Only one file open for reading at a time.
* Caller supplies destination buffer.
* Driver fills caller buffer directly.
* No internal buffering beyond filesystem requirements.

Typical sequence:

```
Open

Read

Read

Read

Close
```

---

# Directory Operations

Support:

* Create directory
* Create nested directories (future extension if supported)
* Create files within directories
* Create files in root

---

# Internal State

Each open write file should maintain state similar to:

* File handle
* Current file size
* Current write offset
* Dirty flag
* Last flushed position

The exact implementation is left open.

---

# Error Handling

Operations should return structured errors.

Typical errors include:

* SD card not present
* Filesystem corruption
* Directory not found
* File already exists
* File not found
* No free file handles
* Read/write failure
* Invalid path
* Invalid operation for current file state

Here are some suggested error constants:
* StorageError
* FilesystemError
* CardRemoved
* CardNotPresent
* NoFilesystem
* DirectoryNotFound
* FileNotFound
* TooManyOpenFiles
* InvalidHandle
* InvalidState
* IoError
* OutOfSpace

The driver should avoid panicking.

---

# Power Loss Considerations

Unexpected power loss is expected.

The design should minimise filesystem corruption.

Questions to resolve:

* When is directory metadata committed?
* When are FAT/exFAT allocation tables updated?
* Does `flush()` guarantee recoverability?
* Should metadata updates be batched for performance?

These decisions should be documented during implementation.

---

# Performance Goals

Optimise for:

* Sequential writes
* Minimal memory copies
* Large contiguous writes
* Efficient append operations
* Predictable latency

Reads are important but secondary.

---

# Future Extensions

The following are intentionally out of scope for the initial implementation but should influence the architecture:

* Storage Service providing virtual file streams
* Multiple producers writing to shared log files
* Automatic file rotation
* Timestamped file naming - this will be provided by the storage service
* Directory enumeration
* File deletion
* File truncation
* Rename/move operations
* Filesystem formatting
* Free-space reporting
* Wear and performance statistics
* Background metadata flushing
* Asynchronous write scheduling

The storage driver should be designed so these higher-level features can be added without requiring significant changes to its core append/read functionality.

# Driver invariants
* maximum 4 write handles
* maximum 1 read handle
* no duplicate handles
* append only increases file size
* close always updates file size
* flush never invalidates handle
* read handle never modifies filesystem
* no operation leaks filesystem resources

# Tests:

Under:
drivertests\storage
make a test suite that exercises:
- filesystem mounting
- error handling e.g. sd card absent, no filesystem
- writing four file handles simultaneously with one reflecting typical log file usage and the others reflecting larger bulk storage
- Demonstrate that files can be terminated at non-block boundaries
- Write in simple .txt format so that it can easily be parsed and read/validated with a text editor.
