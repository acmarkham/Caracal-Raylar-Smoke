## ADR Patch: Producer-Owned Stream Lifecycle and Storage-Owned Stream Materialization

## Constraints
Read
ADR\common\AGENTS.md
before planning and implementing

### Summary

Transfer ownership of stream lifecycle from the Storage Service to the stream producer.

The producer becomes responsible for determining when a stream begins and ends.

The Storage Service remains responsible for materializing logical streams into filesystem objects according to a configurable storage layout.

This removes the callback relationship from Storage to producers while preserving centralized control over filesystem organization.

---

## Replace "Storage Interface"

Replace the existing section with:

### Storage Interface

Storage exposes a generic stream interface.

Conceptually:

```text
stream = storage.begin_stream(
    StreamKind::Audio,
    StorageLayout::HourlyFolders,
)

stream.write(bytes)

stream.finish()
```

The Storage Service does not own stream lifetime.

Instead, it materializes logical streams into persistent filesystem objects.

Storage is responsible for:

* creating files
* generating filenames
* selecting directory hierarchy
* filesystem interaction
* buffering
* flushing
* retries
* closing completed streams

Storage does not determine when a stream begins or ends.

---

## Replace "Storage Rotation"

Replace the entire section with:

### Stream Lifecycle

Logical stream boundaries are owned by the producer.

For audio recordings, the producer is the Audio Recorder Service.

The recorder determines when a recording should begin or end based on its own recording policy.

Typical policies include:

* rotate every hour
* rotate every ten minutes
* continuous recording
* event-driven recording

Because the recorder owns the relationship between sample count, sample rate and absolute time, it can terminate recordings on an exact sample boundary.

When a recording boundary is reached, the recorder:

1. finalises the current container;
2. finishes the current Storage stream;
3. requests a new Storage stream;
4. begins a new recording.

Storage does not request stream rotation.

This removes the need for lifecycle callbacks from Storage to producers.

---

## Add New Section: Storage Layout

Insert immediately after **Storage Interface**.

### Storage Layout

Although producers own stream lifetime, Storage continues to own how streams are represented within the filesystem.

When a producer creates a stream, it specifies a storage layout describing how that stream should be materialized.

For example:

```rust
storage.begin_stream(
    StreamKind::Audio,
    StorageLayout::HourlyFolders,
)
```

Possible layouts might include:

* Flat
* DailyFolders
* HourlyFolders
* MissionFolders

The selected layout determines:

* directory hierarchy
* filename generation
* filename uniqueness
* filesystem-specific conventions

The producer remains unaware of the resulting filenames and directory structure.

This allows filesystem organization to evolve independently of producer logic.

---

## Replace "Responsibilities – Storage Service"

Replace with:

### Storage Service

Responsible for:

* creating streams
* materializing streams as filesystem objects
* filename generation
* directory hierarchy
* filesystem interaction
* buffering
* flushing
* retries
* closing streams

Not responsible for:

* recording duration
* stream segmentation
* recording policies
* media formats
* metadata

Storage persists complete logical streams but does not determine their boundaries.

---

## Replace "Responsibilities – Audio Recorder"

Append the following responsibilities:

The Audio Recorder Service is additionally responsible for:

* determining recording boundaries
* implementing recording policies
* starting new streams
* finalising completed streams

The recorder owns the logical recording lifecycle.

---

## Replace "Consequences"

Add the following advantages:

### Advantages

* Sample-accurate recording boundaries.
* Producers determine stream boundaries using domain-specific knowledge.
* Storage no longer requires lifecycle callbacks.
* Filesystem organization remains centralized.
* Stream lifecycle and filesystem representation are cleanly separated.
* Storage API is simpler and more generic.

Add the following disadvantage:

### Disadvantages

* Stream segmentation logic moves into each producer.
* Producers requiring automatic segmentation must implement their own lifecycle policy.

---

## Replace Design Principle 3

Replace:

> **Control flows downward; lifecycle events flow upward.** Producers stream data into Storage, while Storage communicates lifecycle events such as rotation back to producers.

with:

> **Producers own logical streams.** Producers determine when streams begin and end according to their own domain-specific semantics. Storage materializes those streams into filesystem objects but does not determine their boundaries.

---

## New Design Principle

Add:

> **Separate logical streams from filesystem representation.** Producers own the semantics and lifecycle of streams. Storage owns how those streams are represented on persistent media, including filenames, directory layout and filesystem conventions.

---

## Notes

This patch changes the conceptual relationship from:

```text
Storage
    │
RotateRequested
    │
    ▼
Producer
```

to:

```text
Producer
    │
begin_stream(layout)
    ▼
Storage

Producer
    │
finish_stream()
    ▼
Storage
```

The resulting responsibilities are:

| Producer                      | Storage                 |
| ----------------------------- | ----------------------- |
| Decide when a stream starts   | Create file             |
| Decide when a stream ends     | Generate filename       |
| Generate stream contents      | Choose directory layout |
| Implement segmentation policy | Persist bytes           |
| Finalise media container      | Close file              |

This separation keeps **stream semantics** within the producing domain while keeping **filesystem semantics** centralized within the Storage Service.

# Implementation:

This is a large change that will touch a number of existing services (e.g. the logging service, the audiorecorder service) and their corresponding service/integration tests. Rewrite these all with the proposed change in filestream rotation ownership.