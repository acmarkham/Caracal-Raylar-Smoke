use crate::format::format_line;
use crate::resources::LogRecord;
use crate::{LogSink, LoggerHandle, LoggingResources, LoggingStats, ProcessOutcome};

pub const DEFAULT_MESSAGE_LENGTH: usize = 256;
pub const DEFAULT_QUEUE_DEPTH: usize = 8;
pub const DEFAULT_LINE_LENGTH: usize = 384;

pub struct LoggingService<
    'a,
    S,
    const MESSAGE_LENGTH: usize = DEFAULT_MESSAGE_LENGTH,
    const QUEUE_DEPTH: usize = DEFAULT_QUEUE_DEPTH,
    const LINE_LENGTH: usize = DEFAULT_LINE_LENGTH,
> {
    resources: &'a LoggingResources<MESSAGE_LENGTH, QUEUE_DEPTH>,
    sink: S,
}

impl<'a, S, const MESSAGE_LENGTH: usize, const QUEUE_DEPTH: usize, const LINE_LENGTH: usize>
    LoggingService<'a, S, MESSAGE_LENGTH, QUEUE_DEPTH, LINE_LENGTH>
where
    S: LogSink,
{
    pub const fn new(
        resources: &'a LoggingResources<MESSAGE_LENGTH, QUEUE_DEPTH>,
        sink: S,
    ) -> Self {
        Self { resources, sink }
    }

    pub const fn register(
        &self,
        component: &'static str,
    ) -> LoggerHandle<'a, MESSAGE_LENGTH, QUEUE_DEPTH> {
        self.resources.register(component)
    }

    pub fn stats(&self) -> LoggingStats {
        self.resources.stats()
    }

    pub fn sink_mut(&mut self) -> &mut S {
        &mut self.sink
    }

    pub fn into_sink(self) -> S {
        self.sink
    }

    pub async fn process_one(&mut self) -> Result<ProcessOutcome, S::Error> {
        let Some(record) = self.resources.try_record() else {
            return Ok(ProcessOutcome::Empty);
        };
        self.write_record(record).await?;
        Ok(ProcessOutcome::Written)
    }

    /// Drain pending records before adding critical startup metadata. This
    /// makes room in the existing bounded queue without increasing its size.
    pub async fn drain(&mut self) -> Result<(), S::Error> {
        while self.process_one().await? == ProcessOutcome::Written {}
        Ok(())
    }

    pub async fn flush(&mut self) -> Result<(), S::Error> {
        if let Err(error) = self.sink.flush().await {
            self.resources.record_write_failure();
            return Err(error);
        }
        Ok(())
    }

    pub async fn checkpoint(&mut self) -> Result<(), S::Error> {
        if let Err(error) = self.sink.checkpoint().await {
            self.resources.record_write_failure();
            return Err(error);
        }
        Ok(())
    }

    pub async fn run(mut self) -> ! {
        loop {
            let record = self.resources.next_record().await;
            let _ = self.write_record(record).await;
        }
    }

    async fn write_record(&mut self, record: LogRecord<MESSAGE_LENGTH>) -> Result<(), S::Error> {
        let (line, line_truncated) = format_line::<MESSAGE_LENGTH, LINE_LENGTH>(&record);
        if let Err(error) = self.sink.append(line.as_bytes()).await {
            self.resources.record_write_failure();
            return Err(error);
        }
        self.resources
            .record_write(line.len(), line_truncated, record.message_truncated);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use embassy_time::Instant;

    use super::*;
    use crate::{debug, error, info, LogLevel, LogOutcome};

    #[derive(Default)]
    struct StartupSink {
        text: heapless::String<4096>,
        fail_write: bool,
        checkpoints: u8,
    }

    impl LogSink for StartupSink {
        type Error = ();

        async fn append(&mut self, bytes: &[u8]) -> Result<(), ()> {
            if self.fail_write {
                return Err(());
            }
            self.text
                .push_str(core::str::from_utf8(bytes).unwrap())
                .unwrap();
            Ok(())
        }

        async fn flush(&mut self) -> Result<(), ()> {
            self.checkpoints += 1;
            Ok(())
        }
    }

    fn ready<F: core::future::Future>(future: F) -> F::Output {
        let mut future = core::pin::pin!(future);
        let mut cx = core::task::Context::from_waker(core::task::Waker::noop());
        match future.as_mut().poll(&mut cx) {
            core::task::Poll::Ready(result) => result,
            core::task::Poll::Pending => panic!("test sink must complete without waiting"),
        }
    }

    #[test]
    fn startup_identity_survives_a_full_production_sized_queue() {
        let resources = LoggingResources::<384, 32>::new();
        let log = resources.register("Aardwolf");
        for _ in 0..32 {
            assert_eq!(
                info!(log, "radio startup diagnostics"),
                LogOutcome::Enqueued
            );
        }
        let mut service =
            LoggingService::<_, 384, 32, 512>::new(&resources, StartupSink::default());
        ready(service.drain()).unwrap();
        assert_eq!(resources.stats().queue_depth, 0);
        assert_eq!(
            info!(
                log,
                "versioning sd_card=known serial={} capacity_bytes={}",
                u32::MAX,
                u64::MAX
            ),
            LogOutcome::Enqueued
        );
        ready(service.drain()).unwrap();
        ready(service.checkpoint()).unwrap();
        assert!(service
            .sink_mut()
            .text
            .contains("serial=4294967295 capacity_bytes=18446744073709551615"));
        assert_eq!(service.sink_mut().checkpoints, 1);
        assert_eq!(resources.stats().dropped_messages, 0);
        assert_eq!(resources.stats().truncated_messages, 0);
        assert_eq!(resources.stats().queue_depth, 0);
    }

    #[test]
    fn startup_drain_propagates_storage_failure() {
        let resources = LoggingResources::<64, 2>::new();
        let log = resources.register("Aardwolf");
        let _ = info!(log, "versioning sd_card=known");
        let mut service = LoggingService::<_, 64, 2, 128>::new(
            &resources,
            StartupSink {
                fail_write: true,
                ..StartupSink::default()
            },
        );
        assert_eq!(ready(service.drain()), Err(()));
        assert_eq!(resources.stats().write_failures, 1);
    }

    #[test]
    fn three_components_log_ordered_formatted_records() {
        let resources = LoggingResources::<64, 4>::new();
        let gps = resources.register("GPS");
        let storage = resources.register("Storage");
        let battery = resources.register("Battery");

        assert_eq!(
            gps.log_at(
                Instant::from_micros(3_812_491_000),
                LogLevel::Info,
                format_args!("First fix acquired ({} satellites)", 8),
            ),
            LogOutcome::Enqueued
        );
        assert_eq!(
            storage.log_at(
                Instant::from_micros(3_812_492_000),
                LogLevel::Debug,
                format_args!("wrote {} bytes", 512),
            ),
            LogOutcome::Enqueued
        );
        assert_eq!(
            battery.log_at(
                Instant::from_micros(3_812_493_000),
                LogLevel::Error,
                format_args!("voltage = {:.2} V", 3.25),
            ),
            LogOutcome::Enqueued
        );

        let (first, _) = format_line::<64, 128>(&resources.try_record().unwrap());
        let (second, _) = format_line::<64, 128>(&resources.try_record().unwrap());
        let (third, _) = format_line::<64, 128>(&resources.try_record().unwrap());
        assert_eq!(
            first.as_str(),
            "0000000000 3812.491 INFO  GPS        First fix acquired (8 satellites)\n"
        );
        assert_eq!(
            second.as_str(),
            "0000000001 3812.492 DEBUG Storage    wrote 512 bytes\n"
        );
        assert_eq!(
            third.as_str(),
            "0000000002 3812.493 ERROR Battery    voltage = 3.25 V\n"
        );
    }

    #[test]
    fn public_macros_accept_core_format_arguments() {
        let resources = LoggingResources::<32, 3>::new();
        let gps = resources.register("GPS");
        let storage = resources.register("Storage");
        let battery = resources.register("Battery");

        let _ = info!(gps, "satellites={}", 9u8);
        let _ = debug!(storage, "block={:#x}", 0x20u8);
        let _ = error!(battery, "low={:?}", true);

        assert_eq!(resources.stats().total_messages, 3);
        assert_eq!(resources.stats().queue_depth, 3);
    }

    #[test]
    fn full_queue_drops_newest_and_records_statistics() {
        let resources = LoggingResources::<16, 2>::new();
        let log = resources.register("Test");
        let at = Instant::from_ticks(0);

        assert_eq!(
            log.log_at(at, LogLevel::Info, format_args!("one")),
            LogOutcome::Enqueued
        );
        assert_eq!(
            log.log_at(at, LogLevel::Warn, format_args!("two")),
            LogOutcome::Enqueued
        );
        assert_eq!(
            log.log_at(at, LogLevel::Error, format_args!("three")),
            LogOutcome::DroppedQueueFull
        );

        let stats = resources.stats();
        assert_eq!(stats.total_messages, 3);
        assert_eq!(stats.dropped_messages, 1);
        assert_eq!(stats.queue_depth, 2);
        assert_eq!(stats.maximum_queue_depth, 2);
    }

    #[test]
    fn long_utf8_message_is_safely_truncated_and_enqueued() {
        let resources = LoggingResources::<5, 1>::new();
        let log = resources.register("Test");

        assert_eq!(
            log.log_at(Instant::from_ticks(0), LogLevel::Info, format_args!("éééé"),),
            LogOutcome::EnqueuedTruncated
        );
        let record = resources.try_record().unwrap();
        assert_eq!(record.message.as_str(), "éé");
        assert_eq!(resources.stats().dropped_messages, 0);
        assert_eq!(resources.stats().truncated_messages, 1);
    }
}
