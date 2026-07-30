# ADR: Audio Recorder Service

## Constraints
Read
ADR\common\AGENTS.md
before planning and implementing

---

# Context

The firmware acquires timestamped PCM audio through the `AudioSource` meta-driver.

`AudioSource` owns the shared audio timeline and allows multiple independent consumers to access the same audio stream without additional copying.

One such consumer is the Audio Recorder Service, whose responsibility is to persist recordings to storage.

The firmware also contains:

* **Storage Service** – manages files, directories, buffering and stream materialisation.
* **Time Service** – provides UTC time.
* **Location Service** – provides the latest location estimate.
* **Logging Service** – system diagnostics.
* **Neural Detector Service** – performs inference on the same audio stream.

Storage is intentionally content-agnostic and operates only on opaque byte streams. It should not understand audio formats such as WAV.

The initial recording format will be WAV, however future container formats (e.g. FLAC or raw PCM) should be possible without modifications to the Storage Service.

---

# Decision

Introduce an **Audio Recorder Service** responsible for converting PCM audio from `AudioSource` into an audio container and streaming the resulting bytes to Storage.

The recorder owns all recording-specific state and all knowledge of audio container formats.

```
                 AudioSource
                      │
               Timestamped PCM
                      │
                      ▼
          +----------------------+
          | Audio Recorder       |
          |----------------------|
          | Recording State      |
          | WavContainer         |
          | Metadata             |
          +----------------------+
                      │
                 Byte Stream
                      │
                      ▼
             Storage Service
```

Storage remains unaware of the recording format.

---

# Responsibilities

The Audio Recorder Service is responsible for:

* subscribing to `AudioSource`
* managing recording sessions
* selecting the audio container implementation
* collecting recording metadata
* constructing recording headers
* streaming encoded bytes to Storage
* finalising recordings
* determining recording boundaries
* implementing recording policies
* starting new streams
* finalising completed streams

The recorder owns all knowledge relating to audio recording and the logical recording lifecycle.

---

# Non-Responsibilities

The recorder is **not** responsible for:

* microphone hardware
* DMA
* audio buffering
* audio distribution
* filesystem management
* file naming
* directory management
* storage buffering
* neural network inference

Those responsibilities belong to other system components.

---

# Container Ownership

The recorder owns the selected audio container.

Initially:

```
AudioRecorder

├── Recording State
├── WavContainer
└── Storage Stream
```

The container implementation is considered an internal implementation detail of the recorder.

Storage never exposes APIs such as:

```
make_wav_header(...)
make_flac_header(...)
```

Adding a new container format should only require changes within the recorder.

---

# Recording Metadata

Recording metadata originates from other services.

Examples include:

* recording start time
* GPS location
* firmware version
* device identifier

When a recording begins, the recorder snapshots the required metadata.

```
Time Service
      │
Location Service
      │
Device Information
      │
      ▼
RecordingMetadata
      │
      ▼
Audio Recorder
```

The metadata is passed to the selected container implementation when constructing the recording header.

Storage never interprets this metadata.

---

# Storage Interface

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

# Storage Layout

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

# Recording Lifecycle

A recording session follows the lifecycle:

```
Begin Recording

↓

Snapshot metadata

↓

Create container

↓

Write initial header

↓

Consume PCM from AudioSource

↓

Encode into byte stream

↓

Write to Storage

↓

Finalise container

↓

Close stream
```

The recorder maintains all recording state associated with this lifecycle.

---

# Stream Lifecycle

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

# WAV Container

The initial implementation uses a WAV container.

The recorder:

1. writes a placeholder header;
2. streams PCM data;
3. finalises the WAV header when recording ends;
4. requests header patching through Storage if required.

Storage remains unaware that the stream contains a WAV file.

---

# Future Containers

The recorder is designed to support additional container implementations.

Possible future formats include:

* FLAC
* raw PCM
* Opus
* RF64

Supporting a new container should require changes only within the recorder.

The Storage Service should remain unchanged.

---

# Alternatives Considered

## Storage Generates WAV Files

Rejected.

This couples Storage to audio-specific knowledge and violates its role as a generic persistence service.

Adding new recording formats would require modifications to Storage.

---

## Recorder Writes Files Directly

Rejected.

This duplicates filesystem functionality already provided by Storage and weakens separation of concerns.

---

## Recorder Operates on DMA Buffers

Rejected.

The recorder should consume audio from `AudioSource`, not directly from hardware.

This decouples recording from DMA implementation details and allows multiple independent consumers to process the same audio stream.

---

# Consequences

## Advantages

* Clear separation between recording and persistence.
* Storage remains content-agnostic.
* Audio format knowledge is isolated within the recorder.
* Future container formats can be added without modifying Storage.
* Recording metadata has a single owner.
* Recording lifecycle is encapsulated within one service.
* The recorder can coexist with other `AudioSource` consumers such as neural detectors or streaming services.
* Sample-accurate recording boundaries.
* Producers determine stream boundaries using domain-specific knowledge.
* Storage no longer requires lifecycle callbacks.
* Filesystem organization remains centralized.
* Stream lifecycle and filesystem representation are cleanly separated.
* Storage API is simpler and more generic.

## Disadvantages

* The recorder owns container lifecycle management.
* Header finalisation requires coordination with Storage.
* Recording state is more complex than a simple byte forwarding task.
* Stream segmentation logic moves into each producer.
* Producers requiring automatic segmentation must implement their own lifecycle policy.

---

# Design Principles

1. **The recorder owns recording semantics.** Audio formats, metadata and container generation belong exclusively to the Audio Recorder Service.

2. **Storage owns persistence.** Filesystem operations, filesystem representation and persistence remain the responsibility of the Storage Service.

3. **Producers own logical streams.** Producers determine when streams begin and end according to their own domain-specific semantics. Storage materializes those streams into filesystem objects but does not determine their boundaries.

4. **Separate logical streams from filesystem representation.** Producers own the semantics and lifecycle of streams. Storage owns how those streams are represented on persistent media, including filenames, directory layout and filesystem conventions.

5. **AudioSource owns audio distribution.** The recorder consumes timestamped PCM from `AudioSource` rather than interacting with microphone hardware.

6. **Containers are replaceable.** Recording formats are internal implementation details of the recorder and may evolve independently of Storage.

7. **Services own domain knowledge.** Each service is responsible only for concepts within its own domain, minimising coupling between the recording and storage subsystems.


# Implementation Specifics

The audiorecorder should be implemented under crates/services/audiorecorder

Implementation should be:
* Be heapless.
* Require no dynamic allocation.
* Require minimal RAM.
* Cohesive and coherent, with small blocks preferred over extensive abstractions
* Respect low-power embedded async patterns

The audio recorder determines file boundaries from its recording policy and exact sample count. It finalises the current stream and explicitly begins the next one.

The storage service currently also has a stub which should be removed in lib.rs:
```
/// Placeholder for the recording service's future WAV metadata implementation.
///
/// Audio remains opaque to storage; the recording service can populate and
/// append a header through this stable hook later.
pub const fn make_wavfile_header() -> &'static [u8] {
    &[]
}
```

# Testing

Make a small testsuite under:
servicetests/audiorecorder

This by necessity will require other services to correctly operate e.g. the storage service and the time service. 

It needs to wait until the GPS time is valid before starting.

This should save minute long, mono, 16kHz audio wav files to sd card with 10 minute long directory.


# Prototype wav header:

To help guide your implementation, a previous wave header has been written here (https://raw.githubusercontent.com/acmarkham/CARACAL_EVO_COMBRETUM/refs/heads/main/EvoC_007_Mpala/wavheader.h). This makes a 512 byte chunk for the wav header, sets it to an arbitrary length (which means that we don't need to come back to the header once the file is complete to update it again) and leaves most of the chunk for a COMMENT field. The index of the COMMENT field is then passed up to be filled with whatever human readable string meta data is necessary. You should use this as a rough model for making your own wavheader.

```
#ifndef __WavHeader_h
#define __WavHeader_h

#include "mbed.h"
// NB: Length of comment must be even or padding fails in the wavheader
// We make a big field here (448) so that the header is exactly one block size (512 bytes)
#define LENGTH_OF_COMMENT 448

//
// Header for wav files with 32-bit integer samples
// borrowed with modification from
// https://gist.github.com/stoneface86/747da307f723d89fbf118d0bc8db2da3
// and this has details of info (metadata)
// https://www.recordingblogs.com/wiki/list-chunk-of-a-wave-file
// __attribute__((packed))
#pragma push
#pragma pack(1)

typedef struct chunkID{
    char ID[4];
}chunkID_t;

typedef struct chunkHeader{
    chunkID_t chunkID;
    uint32_t chunkSize;
}chunkHeader_t;

typedef struct wavFormat{
    uint16_t fmtTag;            // = 0x1 for PCM_INT
    uint16_t fmtChannels;       // [B]
    uint32_t fmtSampleRate;     // [B]
    uint32_t fmtAvgBytesPerSec; // [B] = 4 * fmtSampleRate * fmtChannels
    uint16_t fmtBlockAlign;     // [B] = 4 * fmtChannels
    uint16_t fmtBitsPerSample;  // = 32
}wavFormat_t;

typedef struct comment{
    chunkHeader_t commentHeader;
    char comment[LENGTH_OF_COMMENT];
}comment_t;

struct wavHeader{
    chunkHeader_t riff; //'RIFF'
    chunkID_t format;   // 'WAVE'
    chunkHeader_t fmtHeader; // 'fmt '
    wavFormat_t wavFormat;
    chunkHeader_t list; // 'LIST'
    chunkID_t info;     // 'INFO' (subchunk of LIST)
    comment_t comment;
    chunkHeader_t data; // 'DATA'

    // default constructor
    wavHeader()
    {
        // ----------------------------
        //  RIFF header 'RIFF'
        riff.chunkID.ID[0] = 'R';
        riff.chunkID.ID[1] = 'I';
        riff.chunkID.ID[2] = 'F';
        riff.chunkID.ID[3] = 'F';
        // size
        riff.chunkSize = sizeof(wavHeader);
        // WAVE header
        format.ID[0] = 'W';
        format.ID[1] = 'A';
        format.ID[2] = 'V';
        format.ID[3] = 'E';
        // ----------------------------
        // Format header
        fmtHeader.chunkID.ID[0] = 'f';
        fmtHeader.chunkID.ID[1] = 'm';
        fmtHeader.chunkID.ID[2] = 't';
        fmtHeader.chunkID.ID[3] = ' ';
        fmtHeader.chunkSize = sizeof(wavFormat_t);
        wavFormat.fmtTag = 0x01;            // PCM
        wavFormat.fmtChannels = 0x01;       // 1 channel
        wavFormat.fmtSampleRate = 44100;    // 44.1 kHz
        wavFormat.fmtAvgBytesPerSec = 44100*4*1; // 44.1kHz * 4 bytes * 1 channel
        wavFormat.fmtBlockAlign = 4;        // Align to the word (4 bytes)
        wavFormat.fmtBitsPerSample =32;     // 32 bits per sample
        // -----------------------------
        // LIST
        list.chunkID.ID[0] = 'L';
        list.chunkID.ID[1] = 'I';
        list.chunkID.ID[2] = 'S';
        list.chunkID.ID[3] = 'T';
        list.chunkSize = sizeof(info) + sizeof(comment);
        // INFO
        info.ID[0] = 'I';
        info.ID[1] = 'N';
        info.ID[2] = 'F';
        info.ID[3] = 'O';
        // Comment (ICMT)
        comment.commentHeader.chunkID.ID[0] = 'I';
        comment.commentHeader.chunkID.ID[1] = 'C';
        comment.commentHeader.chunkID.ID[2] = 'M';
        comment.commentHeader.chunkID.ID[3] = 'T';
        comment.commentHeader.chunkSize = LENGTH_OF_COMMENT;
        comment.comment[0] = 'A';
        comment.comment[1] = 'b';
        comment.comment[2] = '#'; 
        comment.comment[3] = 0; 
        // ---------------------------
        // data field
        data.chunkID.ID[0] = 'd';
        data.chunkID.ID[1] = 'a';
        data.chunkID.ID[2] = 't';
        data.chunkID.ID[3] = 'a';
        // Todo - this should be set *properly* by the main function
        // also, perhaps setting it slightly larger than anticipated would be sensible
        data.chunkSize = 44100*4*105;
    }
};
#pragma pop

#endif
```
