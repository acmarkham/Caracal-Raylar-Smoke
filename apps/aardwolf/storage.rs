use core::sync::atomic::{AtomicU16, Ordering};
use embassy_sync::{blocking_mutex::raw::CriticalSectionRawMutex, mutex::Mutex};
use raylar_audio_recorder_service::RecordingStorage;
use raylar_drivers::storage::StorageError;
use raylar_logging_service::LogSink;
use raylar_storage_service::{
    StorageLayout, StorageService, StorageServiceError, StreamHandle, StreamKind,
};
use raylar_time_service::UtcTimestamp;

use crate::common;
#[cfg(not(feature = "hexaphonic"))]
use crate::policy;

pub static STORAGE_FLAGS: AtomicU16 =
    AtomicU16::new(raylar_radio_service::heartbeat_v4::STORAGE_UNAVAILABLE);
type BackendError =
    <common::BoardStorageBackend as raylar_storage_service::StorageBackend<512>>::Error;
fn flag_error<T>(
    result: Result<T, StorageServiceError<BackendError>>,
) -> Result<T, StorageServiceError<BackendError>> {
    match &result {
        Err(StorageServiceError::Backend(StorageError::OutOfSpace))
        | Err(StorageServiceError::OutOfSpace) => {
            STORAGE_FLAGS.fetch_or(
                raylar_radio_service::heartbeat_v4::STORAGE_FULL,
                Ordering::Relaxed,
            );
        }
        Err(StorageServiceError::Backend(StorageError::FileAlreadyExists)) => {}
        Err(StorageServiceError::Backend(_)) => {
            STORAGE_FLAGS.fetch_or(
                raylar_radio_service::heartbeat_v4::STORAGE_UNAVAILABLE,
                Ordering::Relaxed,
            );
        }
        _ => {}
    }
    result
}
type Service = StorageService<
    common::BoardStorageBackend,
    &'static raylar_time_service::TimeResources<4, 8>,
    512,
    3,
    16_384,
>;
pub struct SharedStorage {
    inner: Mutex<CriticalSectionRawMutex, Service>,
}
impl SharedStorage {
    pub fn new(service: Service) -> Self {
        Self {
            inner: Mutex::new(service),
        }
    }
    pub async fn begin_log(
        &self,
    ) -> Result<
        StreamHandle,
        StorageServiceError<
            <common::BoardStorageBackend as raylar_storage_service::StorageBackend<512>>::Error,
        >,
    > {
        flag_error(
            self.inner
                .lock()
                .await
                .begin_stream(StreamKind::Log, StorageLayout::Flat)
                .await,
        )
    }
    #[cfg(not(feature = "hexaphonic"))]
    pub async fn begin_audio(
        &self,
        layout: StorageLayout,
        utc: UtcTimestamp,
        suffix: &str,
    ) -> Result<
        StreamHandle,
        StorageServiceError<
            <common::BoardStorageBackend as raylar_storage_service::StorageBackend<512>>::Error,
        >,
    > {
        let mut service = self.inner.lock().await;
        let (_, free, cluster) =
            flag_error(service.space_info().await)?.ok_or(StorageServiceError::InvalidConfig)?;
        if !policy::can_start_audio(free, cluster, service.pending_bytes()) {
            STORAGE_FLAGS.fetch_or(
                raylar_radio_service::heartbeat_v4::STORAGE_FULL,
                Ordering::Relaxed,
            );
            return Err(StorageServiceError::OutOfSpace);
        }
        flag_error(
            service
                .begin_new_audio_with_suffix_at(layout, utc, suffix)
                .await,
        )
    }
    pub async fn write(
        &self,
        stream: StreamHandle,
        bytes: &[u8],
    ) -> Result<
        (),
        StorageServiceError<
            <common::BoardStorageBackend as raylar_storage_service::StorageBackend<512>>::Error,
        >,
    > {
        flag_error(self.inner.lock().await.write(stream, bytes).await)
    }
    pub async fn finish(
        &self,
        stream: StreamHandle,
    ) -> Result<
        (),
        StorageServiceError<
            <common::BoardStorageBackend as raylar_storage_service::StorageBackend<512>>::Error,
        >,
    > {
        flag_error(self.inner.lock().await.finish(stream).await)
    }
    pub async fn rewrite(
        &self,
        stream: StreamHandle,
        offset: u64,
        bytes: &[u8],
    ) -> Result<
        (),
        StorageServiceError<
            <common::BoardStorageBackend as raylar_storage_service::StorageBackend<512>>::Error,
        >,
    > {
        flag_error(self.inner.lock().await.rewrite(stream, offset, bytes).await)
    }
    pub async fn flush(
        &self,
        stream: StreamHandle,
    ) -> Result<
        (),
        StorageServiceError<
            <common::BoardStorageBackend as raylar_storage_service::StorageBackend<512>>::Error,
        >,
    > {
        flag_error(self.inner.lock().await.flush(stream).await)
    }
    pub async fn checkpoint(
        &self,
        stream: StreamHandle,
    ) -> Result<
        (),
        StorageServiceError<
            <common::BoardStorageBackend as raylar_storage_service::StorageBackend<512>>::Error,
        >,
    > {
        flag_error(self.inner.lock().await.checkpoint(stream).await)
    }
}

#[derive(Clone, Copy)]
pub struct RecorderStorage {
    pub shared: &'static SharedStorage,
}
impl RecordingStorage for RecorderStorage {
    type Error = StorageServiceError<
        <common::BoardStorageBackend as raylar_storage_service::StorageBackend<512>>::Error,
    >;
    type Handle = StreamHandle;
    async fn admit_audio_bytes(&mut self, bytes: u64) -> Result<(), Self::Error> {
        let mut service = self.shared.inner.lock().await;
        let (_, free, cluster) =
            flag_error(service.space_info().await)?.ok_or(StorageServiceError::InvalidConfig)?;
        let required = crate::config::LOG_RESERVE_BYTES
            .saturating_add(bytes)
            .saturating_add(u64::from(cluster.max(1)) * 2)
            .saturating_add(service.pending_bytes());
        if free < required {
            STORAGE_FLAGS.fetch_or(
                raylar_radio_service::heartbeat_v4::STORAGE_FULL,
                Ordering::Relaxed,
            );
            return Err(StorageServiceError::OutOfSpace);
        }
        Ok(())
    }
    async fn begin_audio_stream(
        &mut self,
        layout: StorageLayout,
        utc: UtcTimestamp,
        suffix: &str,
    ) -> Result<StreamHandle, Self::Error> {
        // The paired recorder already admits the whole pair before either creation.
        // Keep mono admission in begin_audio; hexaphonic calls generic creation here.
        #[cfg(feature = "hexaphonic")]
        {
            flag_error(
                self.shared
                    .inner
                    .lock()
                    .await
                    .begin_new_audio_with_suffix_at(layout, utc, suffix)
                    .await,
            )
        }
        #[cfg(not(feature = "hexaphonic"))]
        {
            self.shared.begin_audio(layout, utc, suffix).await
        }
    }
    async fn append_audio(
        &mut self,
        stream: StreamHandle,
        bytes: &[u8],
    ) -> Result<(), Self::Error> {
        self.shared.write(stream, bytes).await
    }
    async fn finish_audio(&mut self, stream: StreamHandle) -> Result<(), Self::Error> {
        self.shared.finish(stream).await
    }
    async fn rewrite_audio(
        &mut self,
        stream: StreamHandle,
        offset: u64,
        bytes: &[u8],
    ) -> Result<(), Self::Error> {
        self.shared.rewrite(stream, offset, bytes).await
    }
}

pub struct SystemLogSink {
    shared: &'static SharedStorage,
    stream: StreamHandle,
}
impl SystemLogSink {
    pub async fn open(
        shared: &'static SharedStorage,
    ) -> Result<
        Self,
        StorageServiceError<
            <common::BoardStorageBackend as raylar_storage_service::StorageBackend<512>>::Error,
        >,
    > {
        Ok(Self {
            shared,
            stream: shared.begin_log().await?,
        })
    }
}
impl LogSink for SystemLogSink {
    type Error = StorageServiceError<
        <common::BoardStorageBackend as raylar_storage_service::StorageBackend<512>>::Error,
    >;
    async fn append(&mut self, bytes: &[u8]) -> Result<(), Self::Error> {
        self.shared.write(self.stream, bytes).await
    }
    async fn flush(&mut self) -> Result<(), Self::Error> {
        self.shared.flush(self.stream).await
    }
    async fn checkpoint(&mut self) -> Result<(), Self::Error> {
        self.shared.checkpoint(self.stream).await
    }
}
