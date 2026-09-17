#![no_std]

//! Heapless, source-agnostic distribution of a continuous PCM timeline.
//!
//! A producer copies interleaved PCM into [`AudioSource`]. Each [`Reader`] has
//! an independent absolute cursor and reads the shared ring without another
//! copy. The source can be fed by DMA, a file, or a test fixture.

#[cfg(test)]
extern crate std;

use core::cell::RefCell;

use embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex;
#[cfg(target_arch = "arm")]
use embassy_sync::blocking_mutex::raw::ThreadModeRawMutex;
use embassy_sync::blocking_mutex::Mutex;
use embassy_sync::watch::{Receiver as WatchReceiver, Watch};

pub const DEFAULT_READERS: usize = 4;

// PCM callbacks can be nontrivial. On Cortex-M the buffer is task-context-only
// so those callbacks do not mask DMA interrupts. The host fallback is for tests.
#[cfg(target_arch = "arm")]
type SourceMutex = ThreadModeRawMutex;
#[cfg(not(target_arch = "arm"))]
type SourceMutex = CriticalSectionRawMutex;
type NotificationMutex = CriticalSectionRawMutex;
type NotificationReceiver<'a, const READERS: usize> =
    WatchReceiver<'a, NotificationMutex, u64, READERS>;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct AudioFormat {
    pub sample_rate_hz: u32,
    pub channels: u8,
    /// Ticks per second used by timestamps supplied to [`AudioSource::write`].
    pub timestamp_hz: u32,
}

impl AudioFormat {
    pub const fn new(sample_rate_hz: u32, channels: u8, timestamp_hz: u32) -> Self {
        Self {
            sample_rate_hz,
            channels,
            timestamp_hz,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Error {
    InvalidFormat,
    InvalidCapacity,
    UnalignedSamples,
    NoReaderSlots,
    NotificationSlotsExhausted,
    InvalidMinimum,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ReaderStart {
    /// Start at the next sample written after registration.
    Latest,
    /// Start at the oldest sample which is still retained.
    Oldest,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct ReaderStats {
    pub position: u64,
    pub available_samples: usize,
    pub overrun_count: u64,
    pub dropped_samples: u64,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct ReadStatus {
    pub consumed_samples: usize,
    pub available_samples: usize,
    pub overruns_since_last_read: u64,
    pub dropped_since_last_read: u64,
}

/// A zero-copy view of the next part of a reader's timeline.
///
/// The data is split only when it crosses the physical end of the ring.
/// Keep callback work short: the producer cannot append while the callback is
/// running.
pub struct AudioChunk<'a> {
    pub first: &'a [i32],
    pub second: &'a [i32],
    pub position: u64,
    pub timestamp_ticks: Option<u64>,
    pub format: AudioFormat,
}

impl AudioChunk<'_> {
    pub fn len(&self) -> usize {
        self.first.len() + self.second.len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

#[derive(Clone, Copy)]
struct ReaderState {
    active: bool,
    cursor: u64,
    overrun_count: u64,
    dropped_samples: u64,
    pending_overruns: u64,
    pending_dropped: u64,
}

impl ReaderState {
    const EMPTY: Self = Self {
        active: false,
        cursor: 0,
        overrun_count: 0,
        dropped_samples: 0,
        pending_overruns: 0,
        pending_dropped: 0,
    };
}

struct State<const CAPACITY: usize, const READERS: usize> {
    samples: [i32; CAPACITY],
    write_position: u64,
    first_position: Option<u64>,
    first_timestamp_ticks: Option<u64>,
    readers: [ReaderState; READERS],
}

/// Static PCM ring and reader registry.
pub struct AudioSource<const CAPACITY: usize, const READERS: usize = DEFAULT_READERS> {
    format: AudioFormat,
    state: Mutex<SourceMutex, RefCell<State<CAPACITY, READERS>>>,
    changed: Watch<NotificationMutex, u64, READERS>,
}

impl<const CAPACITY: usize, const READERS: usize> AudioSource<CAPACITY, READERS> {
    pub const fn new(format: AudioFormat) -> Self {
        Self {
            format,
            state: Mutex::new(RefCell::new(State {
                samples: [0; CAPACITY],
                write_position: 0,
                first_position: None,
                first_timestamp_ticks: None,
                readers: [ReaderState::EMPTY; READERS],
            })),
            changed: Watch::new_with(0),
        }
    }

    pub const fn format(&self) -> AudioFormat {
        self.format
    }

    pub const fn capacity(&self) -> usize {
        CAPACITY
    }

    fn validate(&self) -> Result<usize, Error> {
        let channels = usize::from(self.format.channels);
        if self.format.sample_rate_hz == 0 || channels == 0 || self.format.timestamp_hz == 0 {
            return Err(Error::InvalidFormat);
        }
        if CAPACITY == 0 || !CAPACITY.is_multiple_of(channels) {
            return Err(Error::InvalidCapacity);
        }
        Ok(channels)
    }

    /// Append complete interleaved PCM frames.
    ///
    /// Call this from task/thread mode, not an interrupt handler.
    ///
    /// The first non-empty write establishes the timeline epoch from its
    /// `timestamp_ticks`; later timestamps are accepted so producers can use
    /// one uniform call, while sample position defines the continuous timeline.
    /// If a write is larger than the ring, only its newest `CAPACITY` samples
    /// are retained.
    pub fn write(&self, samples: &[i32], timestamp_ticks: u64) -> Result<(), Error> {
        self.write_from_fn(samples.len(), timestamp_ticks, |index| samples[index])
    }

    /// Append samples generated directly into the ring.
    ///
    /// This lets a hardware adapter interleave planar DMA channels without an
    /// intermediate staging buffer. Call it from task/thread mode.
    pub fn write_from_fn<F>(
        &self,
        sample_count: usize,
        timestamp_ticks: u64,
        mut sample_at: F,
    ) -> Result<(), Error>
    where
        F: FnMut(usize) -> i32,
    {
        let channels = self.validate()?;
        if !sample_count.is_multiple_of(channels) {
            return Err(Error::UnalignedSamples);
        }
        if sample_count == 0 {
            return Ok(());
        }

        let published = self.state.lock(|cell| {
            let mut state = cell.borrow_mut();
            let original_len = sample_count;
            let incoming_position = state.write_position;
            if state.first_position.is_none() {
                state.first_position = Some(incoming_position);
                state.first_timestamp_ticks = Some(timestamp_ticks);
            }

            state.write_position = state.write_position.saturating_add(original_len as u64);
            let retained = original_len.min(CAPACITY);
            let skipped = original_len - retained;
            let retained_start = state.write_position - retained as u64;

            // Split once at the physical ring boundary. The previous form did
            // a modulo operation for every sample in this hot 16 kHz path.
            let start = retained_start as usize % CAPACITY;
            let first_len = retained.min(CAPACITY - start);
            for (offset, destination) in state.samples[start..start + first_len]
                .iter_mut()
                .enumerate()
            {
                *destination = sample_at(skipped + offset);
            }
            for (offset, destination) in
                state.samples[..retained - first_len].iter_mut().enumerate()
            {
                *destination = sample_at(skipped + first_len + offset);
            }

            let oldest = state.write_position.saturating_sub(CAPACITY as u64);
            for reader in &mut state.readers {
                if reader.active && reader.cursor < oldest {
                    let dropped = oldest - reader.cursor;
                    reader.cursor = oldest;
                    reader.overrun_count = reader.overrun_count.saturating_add(1);
                    reader.dropped_samples = reader.dropped_samples.saturating_add(dropped);
                    reader.pending_overruns = reader.pending_overruns.saturating_add(1);
                    reader.pending_dropped = reader.pending_dropped.saturating_add(dropped);
                }
            }
            state.write_position
        });
        self.changed.sender().send(published);
        Ok(())
    }

    pub fn register(&self, start: ReaderStart) -> Result<Reader<'_, CAPACITY, READERS>, Error> {
        self.validate()?;
        let receiver = self
            .changed
            .receiver()
            .ok_or(Error::NotificationSlotsExhausted)?;
        let slot = self.state.lock(|cell| {
            let mut state = cell.borrow_mut();
            let cursor = match start {
                ReaderStart::Latest => state.write_position,
                ReaderStart::Oldest => state.write_position.saturating_sub(CAPACITY as u64),
            };
            let slot = state
                .readers
                .iter()
                .position(|reader| !reader.active)
                .ok_or(Error::NoReaderSlots)?;
            state.readers[slot] = ReaderState {
                active: true,
                cursor,
                ..ReaderState::EMPTY
            };
            Ok(slot)
        })?;
        Ok(Reader {
            source: self,
            slot,
            notification: receiver,
        })
    }

    fn timestamp_at(&self, state: &State<CAPACITY, READERS>, position: u64) -> Option<u64> {
        let origin_position = state.first_position?;
        let origin_ticks = state.first_timestamp_ticks?;
        let channels = u64::from(self.format.channels);
        let frames = position.saturating_sub(origin_position) / channels;
        Some(origin_ticks.saturating_add(
            frames.saturating_mul(u64::from(self.format.timestamp_hz))
                / u64::from(self.format.sample_rate_hz),
        ))
    }
}

pub struct Reader<'a, const CAPACITY: usize, const READERS: usize = DEFAULT_READERS> {
    source: &'a AudioSource<CAPACITY, READERS>,
    slot: usize,
    notification: NotificationReceiver<'a, READERS>,
}

impl<const CAPACITY: usize, const READERS: usize> Reader<'_, CAPACITY, READERS> {
    pub fn stats(&self) -> ReaderStats {
        self.source.state.lock(|cell| {
            let state = cell.borrow();
            let reader = state.readers[self.slot];
            ReaderStats {
                position: reader.cursor,
                available_samples: (state.write_position - reader.cursor) as usize,
                overrun_count: reader.overrun_count,
                dropped_samples: reader.dropped_samples,
            }
        })
    }

    /// Move this reader to the producer's current position.
    ///
    /// Samples produced while a consumer performs asynchronous session setup
    /// are intentionally skipped and are not reported as an overrun.
    pub fn seek_to_latest(&mut self) {
        self.source.state.lock(|cell| {
            let mut state = cell.borrow_mut();
            let write_position = state.write_position;
            let reader = &mut state.readers[self.slot];
            reader.cursor = write_position;
            reader.overrun_count = 0;
            reader.dropped_samples = 0;
            reader.pending_overruns = 0;
            reader.pending_dropped = 0;
        });
    }

    /// Inspect and consume samples without copying.
    ///
    /// The callback returns how many samples it consumed. The value is clamped
    /// to the supplied chunk and must preserve whole interleaved frames.
    /// This API is task/thread-mode-only.
    pub fn read<F>(&mut self, max_samples: usize, consume: F) -> Result<ReadStatus, Error>
    where
        F: FnOnce(AudioChunk<'_>) -> usize,
    {
        let channels = self.source.validate()?;
        if !max_samples.is_multiple_of(channels) {
            return Err(Error::UnalignedSamples);
        }
        self.source.state.lock(|cell| {
            let mut state = cell.borrow_mut();
            let reader = state.readers[self.slot];
            let available = (state.write_position - reader.cursor) as usize;
            let length = available.min(max_samples);
            let start = reader.cursor as usize % CAPACITY;
            let first_len = length.min(CAPACITY - start);
            let timestamp_ticks = self.source.timestamp_at(&state, reader.cursor);
            let consumed = consume(AudioChunk {
                first: &state.samples[start..start + first_len],
                second: &state.samples[..length - first_len],
                position: reader.cursor,
                timestamp_ticks,
                format: self.source.format,
            })
            .min(length);

            if !consumed.is_multiple_of(channels) {
                return Err(Error::UnalignedSamples);
            }
            let reader = &mut state.readers[self.slot];
            reader.cursor += consumed as u64;
            let status = ReadStatus {
                consumed_samples: consumed,
                available_samples: available - consumed,
                overruns_since_last_read: reader.pending_overruns,
                dropped_since_last_read: reader.pending_dropped,
            };
            reader.pending_overruns = 0;
            reader.pending_dropped = 0;
            Ok(status)
        })
    }

    /// Sleep until at least `minimum_samples` are available.
    pub async fn wait_for_samples(&mut self, minimum_samples: usize) -> Result<(), Error> {
        let channels = self.source.validate()?;
        if minimum_samples == 0
            || minimum_samples > CAPACITY
            || !minimum_samples.is_multiple_of(channels)
        {
            return Err(Error::InvalidMinimum);
        }
        loop {
            if self.stats().available_samples >= minimum_samples {
                return Ok(());
            }
            self.notification.changed().await;
        }
    }
}

impl<const CAPACITY: usize, const READERS: usize> Drop for Reader<'_, CAPACITY, READERS> {
    fn drop(&mut self) {
        self.source.state.lock(|cell| {
            cell.borrow_mut().readers[self.slot] = ReaderState::EMPTY;
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const FORMAT: AudioFormat = AudioFormat::new(16_000, 2, 1_000_000);

    #[test]
    fn readers_advance_independently_across_wrap() {
        let source = AudioSource::<8, 2>::new(FORMAT);
        let mut recorder = source.register(ReaderStart::Latest).unwrap();
        let mut detector = source.register(ReaderStart::Latest).unwrap();

        source.write(&[0, 1, 2, 3, 4, 5], 100).unwrap();
        recorder
            .read(4, |chunk| {
                assert_eq!(chunk.first, &[0, 1, 2, 3]);
                4
            })
            .unwrap();
        detector.read(2, |_| 2).unwrap();
        source.write(&[6, 7, 8, 9], 287).unwrap();

        recorder
            .read(6, |chunk| {
                assert_eq!(chunk.first, &[4, 5, 6, 7]);
                assert_eq!(chunk.second, &[8, 9]);
                6
            })
            .unwrap();
        assert_eq!(recorder.stats().available_samples, 0);
        assert_eq!(detector.stats().available_samples, 8);
    }

    #[test]
    fn slow_reader_is_advanced_and_observes_overrun() {
        let source = AudioSource::<4, 1>::new(FORMAT);
        let mut reader = source.register(ReaderStart::Latest).unwrap();
        source.write(&[0, 1, 2, 3, 4, 5], 0).unwrap();

        let status = reader
            .read(4, |chunk| {
                assert_eq!(chunk.first, &[2, 3]);
                assert_eq!(chunk.second, &[4, 5]);
                4
            })
            .unwrap();
        assert_eq!(status.overruns_since_last_read, 1);
        assert_eq!(status.dropped_since_last_read, 2);
        assert_eq!(reader.stats().overrun_count, 1);
    }

    #[test]
    fn timestamps_follow_interleaved_frames() {
        let source = AudioSource::<8, 1>::new(FORMAT);
        source.write(&[10, 11, 12, 13], 1_000).unwrap();
        let mut reader = source.register(ReaderStart::Oldest).unwrap();
        reader
            .read(2, |chunk| {
                assert_eq!(chunk.timestamp_ticks, Some(1_000));
                2
            })
            .unwrap();
        reader
            .read(2, |chunk| {
                assert_eq!(chunk.timestamp_ticks, Some(1_062));
                2
            })
            .unwrap();
    }

    #[test]
    fn invalid_alignment_is_rejected() {
        let source = AudioSource::<8, 1>::new(FORMAT);
        assert_eq!(source.write(&[1], 0), Err(Error::UnalignedSamples));
    }

    #[test]
    fn seeking_to_latest_starts_a_clean_session() {
        let source = AudioSource::<8, 1>::new(FORMAT);
        let mut reader = source.register(ReaderStart::Latest).unwrap();
        source.write(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 10], 0).unwrap();

        reader.seek_to_latest();
        assert_eq!(reader.stats().available_samples, 0);
        assert_eq!(reader.stats().overrun_count, 0);
        assert_eq!(reader.stats().dropped_samples, 0);
        source.write(&[11, 12], 625).unwrap();
        let status = reader.read(2, |_| 2).unwrap();
        assert_eq!(status.dropped_since_last_read, 0);
        assert_eq!(status.overruns_since_last_read, 0);
    }
}
