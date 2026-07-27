# ADR: AudioSource Meta-Driver for Shared Audio Distribution

## Constraints
Read
ADR\common\AGENTS.md
before planning and implementing


## Context

The system acquires audio from a PDM microphone using DMA. The low-level microphone driver produces timestamped PCM buffers as DMA completes.



Multiple independent services require access to the same audio stream:

* **Audio Recorder** – packages PCM into a container (initially WAV) and streams it to the storage service.
* **Neural Detector** – performs DSP and neural network inference to detect audio events.
* Future services may include:

  * voice activity detection (VAD)
  * FFT/spectrogram generation
  * live streaming
  * diagnostics

These consumers have different processing characteristics:

* the recorder consumes audio sequentially with minimal latency;
* the detector may consume overlapping windows and occasionally spend significant time performing inference;
* future consumers may operate at different rates again.

The microphone driver should remain a hardware abstraction and should not become responsible for managing multiple consumers, buffering policies or application-level flow control.

## Problem

Using the microphone driver's DMA buffers directly as the system audio interface tightly couples consumers to the hardware implementation.

This creates several problems:

* DMA buffer lifetime becomes visible outside the driver.
* Multiple consumers compete for ownership of buffers.
* Consumers become coupled to DMA interrupt timing.
* Slow consumers complicate DMA buffer recycling.
* Hardware implementation details leak into higher-level services.

These concerns are unrelated to microphone hardware and should not be part of the driver.

## Decision

Introduce an **AudioSource** meta-driver between the microphone driver and higher-level services.

The AudioSource owns a long-lived circular PCM buffer representing the current audio timeline.

The microphone driver writes audio into this buffer.

Consumers independently read from the AudioSource using their own read cursors.

```
                    +-------------------+
                    | Microphone Driver |
                    +-------------------+
                              │
                    Timestamped PCM
                              │
                              ▼
                    +------------------+
                    |   AudioSource    |
                    |                  |
                    | Circular Buffer  |
                    | Read Cursors     |
                    | Notifications    |
                    +------------------+
                       │           │
                       ▼           ▼
             Audio Recorder   Neural Detector
                    │
                    ▼
              Storage Service
```

The AudioSource is considered an infrastructure component rather than an application service.

---

# Responsibilities

## Microphone Driver

Responsible for:

* microphone peripheral configuration
* DMA
* PDM → PCM conversion
* timestamp generation
* copying completed DMA buffers into the AudioSource

Not responsible for:

* multiple consumers
* recording
* inference
* storage
* buffering policy

---

## AudioSource

Responsible for:

* owning the circular PCM buffer
* exposing a continuous audio timeline
* maintaining the current write position
* managing reader registration
* tracking independent reader positions
* notifying readers when new audio arrives
* detecting reader overrun

Not responsible for:

* recording
* inference
* storage
* container formats

---

## Audio Recorder

Responsible for:

* consuming PCM from the AudioSource
* generating container data (WAV)
* metadata
* interaction with the storage service

---

## Neural Detector

Responsible for:

* consuming PCM from the AudioSource
* feature extraction
* inference
* event generation

The detector has no knowledge of storage.

---

# Buffer Ownership

The AudioSource owns the canonical copy of the audio stream.

DMA buffers are temporary implementation details of the microphone driver.

```
DMA Buffer
     │
     │ memcpy
     ▼
AudioSource Circular Buffer
```

Only one copy is performed:

> DMA buffer → AudioSource buffer

All subsequent consumers read directly from the AudioSource without further copying.

---

# Reader Model

Each consumer owns an independent read cursor.

```
write →

+------------------------------------------------------+

          ▲               ▲                 ▲

      recorder        detector          future...
      read idx        read idx
```

Consumers advance independently.

No consumer blocks another.

No consumer affects the ordering seen by another.

---

# Data Model

Conceptually, the AudioSource exposes a continuous stream of timestamped PCM samples.

Readers consume samples in order. In the event of multichannel data, samples are interleaved.

The AudioSource presents audio as a timeline rather than a sequence of DMA buffers.

This intentionally hides hardware implementation details from higher layers.

---

# Synchronisation

The AudioSource uses Embassy synchronization primitives only for task notification.

Typical flow:

```
DMA complete

↓

copy into circular buffer

↓

advance write cursor

↓

notify waiting readers
```

Readers sleep while waiting for new audio and wake only when additional samples become available.

The circular buffer itself is owned and managed by the AudioSource rather than Embassy.

---

# Overrun Policy

Because readers advance independently, the AudioSource monitors the distance between the write cursor and each reader.

If the producer would overwrite unread data, an overrun has occurred.

The exact recovery policy is implementation-defined but may include:

* advancing the slow reader to the oldest retained sample;
* reporting an overrun to the consumer;
* dropping the consumer;
* recording diagnostic statistics.

The policy should be explicit and observable.

---

# Rationale

The AudioSource separates hardware concerns from application concerns.

The microphone driver remains responsible only for acquiring audio.

The AudioSource becomes the canonical owner of audio once it enters the system.

This provides:

* stable buffer lifetime;
* zero-copy sharing between consumers;
* decoupling from DMA timing;
* independent consumer scheduling;
* a single place to implement buffering and overrun handling.

The abstraction is specialised for continuous audio acquisition rather than attempting to provide a general-purpose publish/subscribe or concurrent queue implementation.

---

# Alternatives Considered

## Multiple Consumers Read DMA Buffers

Rejected.

This couples application logic to DMA lifetime and requires the microphone driver to manage multiple consumers and buffer ownership.

---

## Embassy PubSub

Rejected.

`PubSubChannel` distributes messages rather than exposing a shared audio timeline.

Large PCM buffers would either require copying or complicated shared ownership semantics.

The abstraction does not naturally support long-lived overlapping reads.

---

## Independent Queues Per Consumer

Rejected.

This duplicates audio data for every consumer and scales poorly as additional processing stages are added.

---

# Consequences

## Advantages

* Clean separation between hardware acquisition and audio processing.
* Single canonical audio buffer.
* Only one copy of audio data.
* Zero-copy access for all consumers.
* Independent consumer execution rates.
* Hardware implementation details remain encapsulated.
* Additional consumers can be added without modifying the microphone driver.

## Disadvantages

* The AudioSource is a custom concurrency primitive rather than an off-the-shelf Embassy abstraction.
* Memory is reserved for the shared circular buffer.
* Overrun handling becomes an explicit responsibility of the AudioSource.

---

# Design Principles

1. **Hardware drivers own hardware concerns.** DMA and peripherals remain encapsulated within the microphone driver.

2. **AudioSource owns the system audio timeline.** Once samples leave the driver, they become part of a shared, long-lived buffer independent of DMA.

3. **Consumers are independent.** Recording, inference and future processing stages consume the same audio without knowledge of each other.

4. **One copy, many readers.** Audio is copied exactly once from DMA into the shared timeline, after which all access is zero-copy.

5. **Expose audio, not DMA.** Higher-level software reasons about timestamped PCM samples rather than interrupt cadence or DMA buffer boundaries.


# Implementation Specifics

The audiosource should be implemented under crates/metadrivers/audiosource. 

It should also be simple for future decoupling that an audiosource could be derived e.g. from an sd card file for testing and not just a real-time microphone array.

Implementation should be:
* Be heapless.
* Require no dynamic allocation.
* Require minimal RAM.
* Cohesive and coherent, with small blocks preferred over extensive abstractions
* Respect low-power embedded async patterns

# Testing

Make a small, relatively standalone testsuite under:
metadrivertests/audiosource

This should spin up the microphone driver and show that two consumers can read from the audiosource circular buffer.