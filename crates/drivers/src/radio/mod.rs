//! Heap-free async LR1121 driver for the Ebyte E80 module.
//!
//! IRQ timestamps mark end-of-packet detection. They are captured immediately
//! after the DIO9 edge wakes this task, before any SPI transaction.

mod config;
mod ebyte_e80;
mod error;
mod irq;
mod packet;
mod spi_device;
mod state;

pub use config::*;
pub use error::{ConfigError, Error, TransportError};
pub use packet::*;
pub use spi_device::ManualCsSpiDevice;
pub use state::{RadioState, RadioStats};

use embassy_time::{with_deadline, with_timeout, Duration, Instant, Timer};
use embedded_hal::digital::{InputPin, OutputPin};
use embedded_hal_async::{digital::Wait, spi::SpiDevice};

use ebyte_e80::{BackendError, EbyteE80};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DriverTiming {
    pub reset_low: Duration,
    pub reset_settle: Duration,
    pub command_timeout: Duration,
    /// Minimum lead time for final packet/IRQ preparation before a timed arm.
    pub preparation_guard: Duration,
    /// Hardware and host-side upper bound for one transmission.
    pub tx_timeout: Duration,
}

impl Default for DriverTiming {
    fn default() -> Self {
        Self {
            reset_low: Duration::from_millis(10),
            reset_settle: Duration::from_millis(25),
            command_timeout: Duration::from_millis(500),
            preparation_guard: Duration::from_millis(10),
            tx_timeout: Duration::from_secs(5),
        }
    }
}

/// Single-owner Ebyte E80 LR1121 driver.
///
/// `SPI` is an async `SpiDevice`, so a manual-CS adapter may be supplied by the
/// board layer. Construction takes ownership but deliberately performs no I/O.
pub struct RadioDriver<SPI, BUSY, RESET, IRQ> {
    backend: EbyteE80<SPI, BUSY>,
    reset: RESET,
    irq: IRQ,
    timing: DriverTiming,
    state: RadioState,
    initialized: bool,
    operation_in_flight: bool,
    prepared: Option<ChannelConfig>,
    prepared_band: Option<RadioBand>,
    stats: RadioStats,
}

impl<SPI, BUSY, RESET, IRQ> RadioDriver<SPI, BUSY, RESET, IRQ>
where
    SPI: SpiDevice<u8>,
    BUSY: InputPin + Wait,
    RESET: OutputPin,
    IRQ: InputPin + Wait,
{
    pub fn new(spi: SPI, busy: BUSY, reset: RESET, irq: IRQ) -> Self {
        Self::with_timing(spi, busy, reset, irq, DriverTiming::default())
    }

    pub fn with_timing(spi: SPI, busy: BUSY, reset: RESET, irq: IRQ, timing: DriverTiming) -> Self {
        Self {
            backend: EbyteE80::new(spi, busy),
            reset,
            irq,
            timing,
            state: RadioState::Standby,
            initialized: false,
            operation_in_flight: false,
            prepared: None,
            prepared_band: None,
            stats: RadioStats::default(),
        }
    }

    pub fn state(&self) -> RadioState {
        self.state
    }

    pub fn stats(&self) -> RadioStats {
        self.stats
    }

    pub fn prepared_channel(&self) -> Option<&ChannelConfig> {
        self.prepared.as_ref()
    }

    /// Conservative lower bound for choosing retained sleep over XOSC standby.
    /// Applications should replace this bound with measurements from their SPI
    /// rate and board revision before using it in a tight schedule.
    pub fn minimum_useful_sleep_interval(&self) -> Duration {
        self.timing.command_timeout * 2 + self.timing.preparation_guard
    }

    pub async fn initialize(&mut self) -> Result<(), Error> {
        self.initialized = false;
        self.prepared = None;
        self.prepared_band = None;
        self.operation_in_flight = false;

        self.reset
            .set_high()
            .map_err(|_| self.transport(TransportError::Reset))?;
        self.reset
            .set_low()
            .map_err(|_| self.transport(TransportError::Reset))?;
        Timer::after(self.timing.reset_low).await;
        self.reset
            .set_high()
            .map_err(|_| self.transport(TransportError::Reset))?;
        Timer::after(self.timing.reset_settle).await;

        with_timeout(self.timing.command_timeout, self.backend.wait_ready())
            .await
            .map_err(|_| Error::BusyTimeout)?
            .map_err(|error| self.backend_error(error))?;
        with_timeout(self.timing.command_timeout, self.backend.initialize())
            .await
            .map_err(|_| Error::BusyTimeout)?
            .map_err(|error| self.backend_error(error))?;

        self.initialized = true;
        self.set_state(RadioState::Standby);
        RadioStats::increment(&mut self.stats.resets);
        Ok(())
    }

    /// Hardware-reset recovery followed by complete initialization.
    pub async fn recover(&mut self) -> Result<(), Error> {
        self.initialize().await
    }

    pub async fn prepare_channel(&mut self, channel: &ChannelConfig) -> Result<(), Error> {
        self.require_initialized()?;
        let validated = channel.validate()?;
        self.reconcile_cancelled_operation().await?;
        self.require_state(RadioState::Standby)?;
        if self.prepared.as_ref() == Some(channel) {
            return Ok(());
        }

        with_timeout(
            self.timing.command_timeout,
            self.backend.apply_channel(channel, validated.band),
        )
        .await
        .map_err(|_| Error::BusyTimeout)?
        .map_err(|error| self.backend_error(error))?;
        self.prepared = Some(channel.clone());
        self.prepared_band = Some(validated.band);
        Ok(())
    }

    pub async fn receive_at<'a>(
        &'a mut self,
        start: Instant,
        end: Instant,
        buffer: &'a mut [u8],
    ) -> Result<ReceivedPacket<'a>, Error> {
        self.require_initialized()?;
        if end <= start {
            return Err(Error::InvalidTimeWindow);
        }
        self.reconcile_cancelled_operation().await?;
        self.require_state(RadioState::Standby)?;
        let channel = self.prepared.clone().ok_or(Error::ChannelNotPrepared)?;
        let required = channel.maximum_payload_len();
        if buffer.len() < required {
            return Err(Error::BufferTooSmall {
                required,
                available: buffer.len(),
            });
        }
        self.require_lead_time(start)?;

        let payload_limit = required as u8;
        with_timeout(
            self.timing.command_timeout,
            self.backend.configure_payload(&channel, payload_limit),
        )
        .await
        .map_err(|_| Error::BusyTimeout)?
        .map_err(|error| self.backend_error(error))?;
        with_timeout(
            self.timing.command_timeout,
            self.backend.clear_irq(irq::ALL),
        )
        .await
        .map_err(|_| Error::BusyTimeout)?
        .map_err(|error| self.backend_error(error))?;
        with_timeout(
            self.timing.command_timeout,
            self.backend.set_irq_mask(irq::RX_MASK),
        )
        .await
        .map_err(|_| Error::BusyTimeout)?
        .map_err(|error| self.backend_error(error))?;

        self.require_start_not_passed(start)?;

        self.operation_in_flight = true;
        Timer::at(start).await;
        self.set_state(RadioState::Rx);
        let timeout_ticks = rtc_ticks(end - start);
        match with_timeout(
            self.timing.command_timeout,
            self.backend.start_rx(timeout_ticks),
        )
        .await
        {
            Ok(Ok(())) => {}
            Ok(Err(error)) => {
                let error = self.backend_error(error);
                let _ = self.finish_standby().await;
                return Err(error);
            }
            Err(_) => {
                let _ = self.finish_standby().await;
                return Err(Error::BusyTimeout);
            }
        }

        let irq_at = match self.wait_for_irq_until(end).await {
            Ok(at) => at,
            Err(error) => {
                RadioStats::increment(&mut self.stats.rx_timeouts);
                self.finish_standby().await?;
                return Err(error);
            }
        };
        let flags = self.read_irq_flags().await?;
        self.clear_handled_irq(flags & irq::RX_MASK).await?;

        if flags & irq::CRC_ERROR != 0 {
            RadioStats::increment(&mut self.stats.crc_errors);
            self.finish_standby().await?;
            return Err(Error::CrcRejected { at: irq_at });
        }
        if flags & irq::HEADER_ERROR != 0 {
            RadioStats::increment(&mut self.stats.header_errors);
            self.finish_standby().await?;
            return Err(Error::HeaderRejected { at: irq_at });
        }
        if flags & irq::GFSK_LENGTH_ERROR != 0 {
            RadioStats::increment(&mut self.stats.gfsk_length_errors);
            self.finish_standby().await?;
            return Err(Error::GfskLengthRejected { at: irq_at });
        }
        if flags & irq::GFSK_ADDRESS_ERROR != 0 {
            RadioStats::increment(&mut self.stats.gfsk_address_errors);
            self.finish_standby().await?;
            return Err(Error::GfskAddressRejected { at: irq_at });
        }
        if flags & irq::TIMEOUT != 0 {
            RadioStats::increment(&mut self.stats.rx_timeouts);
            self.finish_standby().await?;
            return Err(Error::RxTimeout { at: irq_at });
        }
        if let Err(error) = self.check_device_irq(flags, irq_at) {
            let _ = self.finish_standby().await;
            return Err(error);
        }
        if flags & irq::RX_DONE == 0 {
            self.finish_standby().await?;
            return Err(Error::Device);
        }

        let is_lora = matches!(channel.modulation, ModulationConfig::LoRa(_));
        let frequency_hz = channel.frequency_hz;
        let (payload, metrics) = with_timeout(
            self.timing.command_timeout,
            self.backend.read_packet(buffer, is_lora),
        )
        .await
        .map_err(|_| Error::BusyTimeout)?
        .map_err(|error| self.backend_error(error))?;
        self.finish_standby().await?;
        RadioStats::increment(&mut self.stats.rx_packets);
        Ok(ReceivedPacket {
            payload,
            metadata: RxMetadata {
                packet_complete_at: irq_at,
                frequency_hz,
                metrics,
            },
        })
    }

    pub async fn transmit_at(
        &mut self,
        start: Instant,
        payload: &[u8],
        tx: &TxConfig,
    ) -> Result<TxReport, Error> {
        self.require_initialized()?;
        self.reconcile_cancelled_operation().await?;
        self.require_state(RadioState::Standby)?;
        let channel = self.prepared.clone().ok_or(Error::ChannelNotPrepared)?;
        let band = self.prepared_band.ok_or(Error::ChannelNotPrepared)?;
        tx.validate(band)?;
        validate_tx_payload(&channel, payload.len())?;
        self.require_lead_time(start)?;

        with_timeout(
            self.timing.command_timeout,
            self.backend
                .configure_payload(&channel, payload.len() as u8),
        )
        .await
        .map_err(|_| Error::BusyTimeout)?
        .map_err(|error| self.backend_error(error))?;
        with_timeout(
            self.timing.command_timeout,
            self.backend.configure_tx(*tx, band),
        )
        .await
        .map_err(|_| Error::BusyTimeout)?
        .map_err(|error| self.backend_error(error))?;
        with_timeout(
            self.timing.command_timeout,
            self.backend.write_payload(payload),
        )
        .await
        .map_err(|_| Error::BusyTimeout)?
        .map_err(|error| self.backend_error(error))?;
        with_timeout(
            self.timing.command_timeout,
            self.backend.clear_irq(irq::ALL),
        )
        .await
        .map_err(|_| Error::BusyTimeout)?
        .map_err(|error| self.backend_error(error))?;
        with_timeout(
            self.timing.command_timeout,
            self.backend.set_irq_mask(irq::TX_MASK),
        )
        .await
        .map_err(|_| Error::BusyTimeout)?
        .map_err(|error| self.backend_error(error))?;

        self.require_start_not_passed(start)?;

        self.operation_in_flight = true;
        Timer::at(start).await;
        self.set_state(RadioState::Tx);
        let command_started_at = Instant::now();
        let start_result = with_timeout(
            self.timing.command_timeout,
            self.backend.start_tx(rtc_ticks(self.timing.tx_timeout)),
        )
        .await;
        match start_result {
            Ok(Ok(())) => {}
            Ok(Err(error)) => {
                let error = self.backend_error(error);
                let _ = self.finish_standby().await;
                return Err(error);
            }
            Err(_) => {
                let _ = self.finish_standby().await;
                return Err(Error::BusyTimeout);
            }
        }
        let command_completed_at = Instant::now();
        let host_deadline = start + self.timing.tx_timeout + self.timing.command_timeout;
        let irq_at = match self.wait_for_irq_until(host_deadline).await {
            Ok(at) => at,
            Err(Error::RxTimeout { .. }) => {
                RadioStats::increment(&mut self.stats.tx_timeouts);
                self.finish_standby().await?;
                return Err(Error::TxTimeout { at: Instant::now() });
            }
            Err(error) => {
                let _ = self.finish_standby().await;
                return Err(error);
            }
        };
        let flags = self.read_irq_flags().await?;
        self.clear_handled_irq(flags & irq::TX_MASK).await?;
        if flags & irq::TIMEOUT != 0 {
            RadioStats::increment(&mut self.stats.tx_timeouts);
            self.finish_standby().await?;
            return Err(Error::TxTimeout { at: irq_at });
        }
        if let Err(error) = self.check_device_irq(flags, irq_at) {
            let _ = self.finish_standby().await;
            return Err(error);
        }
        if flags & irq::TX_DONE == 0 {
            self.finish_standby().await?;
            return Err(Error::Device);
        }
        self.finish_standby().await?;
        RadioStats::increment(&mut self.stats.tx_packets);
        Ok(TxReport {
            requested_start: start,
            command_started_at,
            command_completed_at,
            tx_done_at: irq_at,
        })
    }

    pub async fn standby(&mut self) -> Result<(), Error> {
        self.require_initialized()?;
        if self.state == RadioState::Sleep {
            return self.wake().await;
        }
        self.operation_in_flight = false;
        self.finish_standby().await
    }

    pub async fn sleep(&mut self) -> Result<(), Error> {
        self.require_initialized()?;
        self.reconcile_cancelled_operation().await?;
        self.require_state(RadioState::Standby)?;
        with_timeout(self.timing.command_timeout, self.backend.sleep_retained())
            .await
            .map_err(|_| Error::BusyTimeout)?
            .map_err(|error| self.backend_error(error))?;
        self.set_state(RadioState::Sleep);
        Ok(())
    }

    pub async fn wake(&mut self) -> Result<(), Error> {
        self.require_initialized()?;
        self.require_state(RadioState::Sleep)?;
        with_timeout(self.timing.command_timeout, self.backend.standby(true))
            .await
            .map_err(|_| Error::WakeUp)?
            .map_err(|_| Error::WakeUp)?;
        with_timeout(self.timing.command_timeout, self.backend.wait_ready())
            .await
            .map_err(|_| Error::WakeUp)?
            .map_err(|_| Error::WakeUp)?;
        self.set_state(RadioState::Standby);
        Ok(())
    }

    async fn reconcile_cancelled_operation(&mut self) -> Result<(), Error> {
        if self.operation_in_flight {
            self.finish_standby().await?;
            self.clear_handled_irq(irq::ALL).await?;
        }
        Ok(())
    }

    async fn finish_standby(&mut self) -> Result<(), Error> {
        self.operation_in_flight = false;
        with_timeout(self.timing.command_timeout, self.backend.standby(true))
            .await
            .map_err(|_| Error::BusyTimeout)?
            .map_err(|error| self.backend_error(error))?;
        self.set_state(RadioState::Standby);
        Ok(())
    }

    async fn wait_for_irq_until(&mut self, deadline: Instant) -> Result<Instant, Error> {
        let already_high = self
            .irq
            .is_high()
            .map_err(|_| self.transport(TransportError::Irq))?;
        if !already_high {
            with_deadline(deadline, self.irq.wait_for_rising_edge())
                .await
                .map_err(|_| Error::RxTimeout { at: Instant::now() })?
                .map_err(|_| self.transport(TransportError::Irq))?;
        }
        Ok(Instant::now())
    }

    async fn read_irq_flags(&mut self) -> Result<u32, Error> {
        with_timeout(self.timing.command_timeout, self.backend.irq_status())
            .await
            .map_err(|_| Error::BusyTimeout)?
            .map_err(|error| self.backend_error(error))
    }

    async fn clear_handled_irq(&mut self, flags: u32) -> Result<(), Error> {
        if flags == 0 {
            return Ok(());
        }
        with_timeout(self.timing.command_timeout, self.backend.clear_irq(flags))
            .await
            .map_err(|_| Error::BusyTimeout)?
            .map_err(|error| self.backend_error(error))
    }

    fn check_device_irq(&mut self, flags: u32, at: Instant) -> Result<(), Error> {
        if flags & irq::COMMAND_ERROR != 0 {
            RadioStats::increment(&mut self.stats.command_errors);
            return Err(Error::CommandIrq { at });
        }
        if flags & irq::DEVICE_ERROR != 0 {
            return Err(Error::DeviceIrq { at });
        }
        Ok(())
    }

    fn require_initialized(&self) -> Result<(), Error> {
        if self.initialized {
            Ok(())
        } else {
            Err(Error::NotInitialized)
        }
    }

    fn require_state(&self, expected: RadioState) -> Result<(), Error> {
        if self.state == expected {
            Ok(())
        } else {
            Err(Error::InvalidState {
                expected,
                actual: self.state,
            })
        }
    }

    fn require_lead_time(&mut self, start: Instant) -> Result<(), Error> {
        let observed = Instant::now();
        if start < observed + self.timing.preparation_guard {
            RadioStats::increment(&mut self.stats.deadline_misses);
            Err(Error::DeadlineMissed {
                requested: start,
                observed,
            })
        } else {
            Ok(())
        }
    }

    fn require_start_not_passed(&mut self, start: Instant) -> Result<(), Error> {
        let observed = Instant::now();
        if observed >= start {
            RadioStats::increment(&mut self.stats.deadline_misses);
            Err(Error::DeadlineMissed {
                requested: start,
                observed,
            })
        } else {
            Ok(())
        }
    }

    fn set_state(&mut self, state: RadioState) {
        self.state = state;
        self.stats.set_state(state);
    }

    fn transport(&mut self, error: TransportError) -> Error {
        RadioStats::increment(&mut self.stats.transport_errors);
        Error::Transport(error)
    }

    fn backend_error(&mut self, error: BackendError) -> Error {
        match error {
            BackendError::Transport(error) => self.transport(error),
            BackendError::Command => {
                RadioStats::increment(&mut self.stats.command_errors);
                Error::Command
            }
            BackendError::Device => Error::Device,
            BackendError::Calibration => Error::Calibration,
        }
    }
}

fn rtc_ticks(duration: Duration) -> u32 {
    let ticks = duration
        .as_micros()
        .saturating_mul(32_768)
        .div_ceil(1_000_000);
    ticks.clamp(1, 0xFF_FFFE) as u32
}

fn validate_tx_payload(channel: &ChannelConfig, length: usize) -> Result<(), Error> {
    if length == 0 || length > MAX_PACKET_LEN {
        return Err(Error::PayloadTooLarge);
    }
    let valid = match &channel.modulation {
        ModulationConfig::LoRa(config) => match (config.header, config.payload_length) {
            (LoRaHeaderMode::Implicit, Some(exact)) => length == usize::from(exact),
            (LoRaHeaderMode::Explicit, Some(maximum)) => length <= usize::from(maximum),
            (LoRaHeaderMode::Explicit, None) => true,
            (LoRaHeaderMode::Implicit, None) => false,
        },
        ModulationConfig::Gfsk(config) => match config.packet_length {
            GfskPacketLength::Fixed(exact) => length == usize::from(exact),
            GfskPacketLength::Variable { maximum }
            | GfskPacketLength::VariableSx128x { maximum } => length <= usize::from(maximum),
        },
    };
    if valid {
        Ok(())
    } else {
        Err(Error::PayloadTooLarge)
    }
}

#[cfg(test)]
mod tests;
