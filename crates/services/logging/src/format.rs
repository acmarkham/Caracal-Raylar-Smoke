use core::fmt::{self, Write};

use heapless::String;

use crate::resources::LogRecord;

pub(crate) struct TruncatingWriter<'a, const N: usize> {
    output: &'a mut String<N>,
    limit: usize,
    truncated: bool,
}

impl<'a, const N: usize> TruncatingWriter<'a, N> {
    pub(crate) fn new(output: &'a mut String<N>, limit: usize) -> Self {
        Self {
            output,
            limit: limit.min(N),
            truncated: false,
        }
    }

    pub(crate) const fn truncated(&self) -> bool {
        self.truncated
    }
}

impl<const N: usize> Write for TruncatingWriter<'_, N> {
    fn write_str(&mut self, value: &str) -> fmt::Result {
        let remaining = self.limit.saturating_sub(self.output.len());
        if value.len() <= remaining {
            let _ = self.output.push_str(value);
            return Ok(());
        }

        let mut end = remaining.min(value.len());
        while end > 0 && !value.is_char_boundary(end) {
            end -= 1;
        }
        let _ = self.output.push_str(&value[..end]);
        self.truncated = true;
        Ok(())
    }
}

pub(crate) fn format_line<const MESSAGE_LENGTH: usize, const LINE_LENGTH: usize>(
    record: &LogRecord<MESSAGE_LENGTH>,
) -> (String<LINE_LENGTH>, bool) {
    let mut line = String::new();
    let timestamp_us = record.timestamp.as_micros();
    let seconds = timestamp_us / 1_000_000;
    let milliseconds = timestamp_us % 1_000_000 / 1_000;
    let truncated = {
        let mut writer = TruncatingWriter::new(&mut line, LINE_LENGTH.saturating_sub(1));
        let _ = write!(
            writer,
            "{:010} {}.{:03} {:<5} {:<10} {}",
            record.sequence, seconds, milliseconds, record.level, record.component, record.message,
        );
        writer.truncated()
    };
    if LINE_LENGTH > 0 {
        let _ = line.push('\n');
    }
    (line, truncated)
}
