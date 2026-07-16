use crate::types::{RollingPolicy, StorageServiceError, StreamType, PATH_CAPACITY};
use core::fmt::Write;
use heapless::String;

pub(crate) struct FilePolicy {
    pub path: String<PATH_CAPACITY>,
    pub file_start: i64,
    pub rollover_at: i64,
}

pub(crate) fn file_policy<E>(
    kind: StreamType,
    timestamp: i64,
    policy: RollingPolicy,
    initial_open: bool,
) -> Result<FilePolicy, StorageServiceError<E>> {
    if timestamp < 0 || !policy.is_valid() {
        return Err(StorageServiceError::InvalidTimestamp);
    }
    let natural_start = align_down(timestamp, policy.file_interval_seconds);
    let file_start = if initial_open && timestamp != natural_start {
        align_down(timestamp, policy.startup_alignment_seconds)
    } else {
        natural_start
    };
    let folder_start = align_down(file_start, policy.folder_interval_seconds);
    let rollover_at = next_boundary(timestamp, policy.file_interval_seconds);
    let mut path = String::new();
    match kind {
        StreamType::Audio => write!(&mut path, "/{folder_start}/aud_{file_start}.wav"),
        StreamType::GpsTiming => write!(&mut path, "/{folder_start}/gps_{file_start}.pps"),
        StreamType::Log => return Err(StorageServiceError::InvalidStream),
    }
    .map_err(|_| StorageServiceError::InvalidPath)?;
    Ok(FilePolicy {
        path,
        file_start,
        rollover_at,
    })
}

pub(crate) fn folder_path<E>(path: &str) -> Result<String<PATH_CAPACITY>, StorageServiceError<E>> {
    let end = path.rfind('/').ok_or(StorageServiceError::InvalidPath)?;
    if end == 0 {
        return Err(StorageServiceError::InvalidPath);
    }
    let mut folder = String::new();
    folder
        .push_str(&path[..end])
        .map_err(|_| StorageServiceError::InvalidPath)?;
    Ok(folder)
}

const fn align_down(timestamp: i64, interval: i64) -> i64 {
    timestamp.div_euclid(interval) * interval
}
const fn next_boundary(timestamp: i64, interval: i64) -> i64 {
    align_down(timestamp, interval).saturating_add(interval)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn audio_startup_is_minute_aligned_then_rolls_on_the_hour() {
        let result = file_policy::<()>(
            StreamType::Audio,
            1_784_016_510,
            RollingPolicy::audio_default(),
            true,
        )
        .unwrap();
        assert_eq!(result.path.as_str(), "/1783987200/aud_1784016480.wav");
        assert_eq!(result.file_start, 1_784_016_480);
        assert_eq!(result.rollover_at, 1_784_019_600);
    }

    #[test]
    fn rollover_open_uses_the_natural_file_boundary() {
        let result = file_policy::<()>(
            StreamType::Audio,
            1_784_019_600,
            RollingPolicy::audio_default(),
            false,
        )
        .unwrap();
        assert_eq!(result.path.as_str(), "/1783987200/aud_1784019600.wav");
        assert_eq!(result.rollover_at, 1_784_023_200);
    }
}
