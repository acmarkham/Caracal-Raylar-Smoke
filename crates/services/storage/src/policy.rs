use crate::types::{StorageLayout, StorageServiceError, StreamKind, PATH_CAPACITY};
use core::fmt::Write;
use heapless::String;

pub(crate) fn stream_path<E>(
    kind: StreamKind,
    layout: StorageLayout,
    timestamp: Option<i64>,
    sequence: u32,
) -> Result<String<PATH_CAPACITY>, StorageServiceError<E>> {
    if !layout.is_valid() {
        return Err(StorageServiceError::InvalidConfig);
    }

    if kind == StreamKind::Log && layout == StorageLayout::Flat {
        let mut path = String::new();
        path.push_str("/syslog.txt")
            .map_err(|_| StorageServiceError::InvalidPath)?;
        return Ok(path);
    }

    let timestamp = timestamp.ok_or(StorageServiceError::InvalidTimestamp)?;
    if timestamp < 0 {
        return Err(StorageServiceError::InvalidTimestamp);
    }

    let mut path = String::new();
    match layout {
        StorageLayout::Flat => {}
        StorageLayout::DailyFolders => {
            write!(&mut path, "/{}", align_down(timestamp, 86_400))
                .map_err(|_| StorageServiceError::InvalidPath)?;
        }
        StorageLayout::HourlyFolders => {
            write!(&mut path, "/{}", align_down(timestamp, 3_600))
                .map_err(|_| StorageServiceError::InvalidPath)?;
        }
        StorageLayout::MissionFolders => {
            path.push_str("/mission")
                .map_err(|_| StorageServiceError::InvalidPath)?;
        }
        StorageLayout::IntervalFolders { interval_seconds } => {
            write!(&mut path, "/{}", align_down(timestamp, interval_seconds))
                .map_err(|_| StorageServiceError::InvalidPath)?;
        }
    }

    match kind {
        StreamKind::Log => write!(&mut path, "/log_{timestamp}_{sequence}.txt"),
        StreamKind::Audio => write!(&mut path, "/aud_{timestamp}_{sequence}.wav"),
        StreamKind::GpsTiming => write!(&mut path, "/gps_{timestamp}_{sequence}.pps"),
    }
    .map_err(|_| StorageServiceError::InvalidPath)?;
    Ok(path)
}

pub(crate) fn folder_path<E>(
    path: &str,
) -> Result<Option<String<PATH_CAPACITY>>, StorageServiceError<E>> {
    let end = path.rfind('/').ok_or(StorageServiceError::InvalidPath)?;
    if end == 0 {
        return Ok(None);
    }
    let mut folder = String::new();
    folder
        .push_str(&path[..end])
        .map_err(|_| StorageServiceError::InvalidPath)?;
    Ok(Some(folder))
}

const fn align_down(timestamp: i64, interval: i64) -> i64 {
    timestamp.div_euclid(interval) * interval
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn daily_audio_layout_keeps_naming_inside_storage() {
        let path = stream_path::<()>(
            StreamKind::Audio,
            StorageLayout::DailyFolders,
            Some(1_784_016_510),
            7,
        )
        .unwrap();
        assert_eq!(path.as_str(), "/1783987200/aud_1784016510_7.wav");
    }

    #[test]
    fn flat_log_preserves_the_restart_safe_system_log() {
        let path = stream_path::<()>(StreamKind::Log, StorageLayout::Flat, None, 1).unwrap();
        assert_eq!(path.as_str(), "/syslog.txt");
    }

    #[test]
    fn interval_layout_is_configurable_without_owning_stream_duration() {
        let path = stream_path::<()>(
            StreamKind::GpsTiming,
            StorageLayout::IntervalFolders {
                interval_seconds: 600,
            },
            Some(1_784_016_510),
            2,
        )
        .unwrap();
        assert_eq!(path.as_str(), "/1784016000/gps_1784016510_2.pps");
    }
}
