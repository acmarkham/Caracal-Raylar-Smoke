use defmt::{info, warn};
use embassy_futures::select::{select, select3, Either, Either3};
use embassy_stm32::exti::ExtiInput;
use embassy_stm32::gpio::Output;
use embassy_stm32::mode::{Async, Blocking};
use embassy_stm32::spi::mode::Master;
use embassy_stm32::spi::Spi;
use embassy_stm32::time::mhz;
use embassy_time::{Duration, Instant, Timer};
use heapless::Vec;
use raylar_board_v1p0::EbyteRf;
use raylar_drivers::radio::{DriverTiming, ManualCsSpiDevice, RadioDriver};
use raylar_radio_service::link::{ChannelProfile, LinkObservation, LinkOutcome, PassiveLinkState};
use raylar_radio_service::{
    BatterySoc, BootId, CapabilityFlags, ChargingState, Epoch, ErrorFlags, FrameBuffer,
    FrameHeader, FrameType, Heartbeat, JobId, NeighbourEntry, NeighbourTable, NodeId,
    PresenceAdvert, RadioEvent, RadioEventReceiver, RadioHandle, RadioPriority, RadioResources,
    RadioRxJob, RadioService, RadioServiceError, RadioTxJob, RendezvousPurpose, RxPurpose,
    ScheduleError, Sequence, SequenceState, StorageUsage,
};
use raylar_time_service::{TimeState, UtcStatus, UtcTimestamp};

use crate::diagnostics::{self, DiagnosticKind};
use crate::indication;
use crate::{common, LOCATION};
use integration004_radioheartbeat::config;
use integration004_radioheartbeat::policy::{self, Role, UtcWindow, WindowClass};

pub const JOB_DEPTH: usize = 24;
pub const EVENT_DEPTH: usize = 32;
pub const STATE_WATCHERS: usize = 4;
const NEIGHBOUR_CAPACITY: usize = 8;
const WINDOW_CAPACITY: usize = 24;
const RADIO_REARM_LEAD: Duration = Duration::from_millis(110);
const PRE_UTC_BASE_RX: Duration = Duration::from_secs(5);
const MAX_BASE_RX_RESERVATION: Duration = Duration::from_secs(5);

pub static RADIO: RadioResources<JOB_DEPTH, EVENT_DEPTH, STATE_WATCHERS> = RadioResources::new();

type RadioSpi = Spi<'static, Blocking, Master>;
type RadioSpiDevice = ManualCsSpiDevice<RadioSpi, Output<'static>>;
type BoardRadio = RadioDriver<
    RadioSpiDevice,
    ExtiInput<'static, Async>,
    Output<'static>,
    ExtiInput<'static, Async>,
>;
type BoardRadioService = RadioService<'static, BoardRadio, JOB_DEPTH, EVENT_DEPTH, STATE_WATCHERS>;

pub fn make_radio(rf: EbyteRf<'static>) -> BoardRadio {
    let EbyteRf {
        spi,
        sck,
        miso,
        mosi,
        cs,
        busy,
        nrst,
        irq,
    } = rf;
    let mut spi_config = embassy_stm32::spi::Config::default();
    spi_config.frequency = mhz(1);
    let spi = Spi::new_blocking(spi, sck, mosi, miso, spi_config);
    let spi_device = ManualCsSpiDevice::new(spi, cs);
    RadioDriver::with_timing(
        spi_device,
        busy,
        nrst,
        irq,
        DriverTiming {
            preparation_guard: Duration::from_millis(10),
            ..DriverTiming::default()
        },
    )
}

#[embassy_executor::task]
pub async fn radio_service_task(service: BoardRadioService) -> ! {
    service.run().await
}

#[derive(Clone, Copy, Debug)]
struct TxContext {
    id: JobId,
    epoch: Epoch,
    purpose: RendezvousPurpose,
    slot: u32,
    sequence: Sequence,
    start: Instant,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum ReceiveMode {
    Scan,
    Predicted,
    Promiscuous,
    Unverifiable,
}

#[derive(Clone, Copy, Debug)]
struct RxContext {
    id: JobId,
    epoch: Option<Epoch>,
    end: Instant,
    mode: ReceiveMode,
    receptions: u16,
}

#[derive(Clone, Copy, Debug)]
struct PeerMeta {
    node_id: NodeId,
    boot_id: BootId,
    capabilities: CapabilityFlags,
}

#[derive(Clone, Copy, Debug, Default)]
struct LocalStats {
    heartbeat_rx: u32,
    unknown_heartbeat_rx: u32,
    predicted_rx: u32,
    predicted_misses: u32,
    scan_rx: u32,
    outside_rx: u32,
    unverifiable_rx: u32,
    application_malformed: u32,
}

pub async fn run(role: Role, node_id: NodeId, boot_id: BootId, profile: ChannelProfile) -> ! {
    let neighbour_expiry = config::service_config().neighbour_expiry;
    let mut coordinator = Coordinator {
        role,
        node_id,
        profile,
        handle: RADIO.handle(),
        events: RADIO.event_receiver(),
        sequence: SequenceState::new(boot_id),
        neighbours: NeighbourTable::new(neighbour_expiry),
        peers: Vec::new(),
        tx: Vec::new(),
        rx: Vec::new(),
        local: LocalStats::default(),
        completed_epochs: 0,
        force_scan: true,
        base_rx_horizon: Instant::now(),
        last_scheduled: None,
        last_finished: None,
        last_time_status: UtcStatus::Invalid,
    };
    coordinator.run().await
}

struct Coordinator {
    role: Role,
    node_id: NodeId,
    profile: ChannelProfile,
    handle: RadioHandle<'static, JOB_DEPTH>,
    events: RadioEventReceiver<'static, EVENT_DEPTH>,
    sequence: SequenceState,
    neighbours: NeighbourTable<NEIGHBOUR_CAPACITY>,
    peers: Vec<PeerMeta, NEIGHBOUR_CAPACITY>,
    tx: Vec<TxContext, JOB_DEPTH>,
    rx: Vec<RxContext, JOB_DEPTH>,
    local: LocalStats,
    completed_epochs: u8,
    force_scan: bool,
    base_rx_horizon: Instant,
    last_scheduled: Option<Epoch>,
    last_finished: Option<Epoch>,
    last_time_status: UtcStatus,
}

impl Coordinator {
    async fn run(&mut self) -> ! {
        let initial = self.wait_for_utc().await;
        let now_utc = initial
            .system_to_utc(Instant::now())
            .expect("UTC was checked immediately before conversion");
        let current = config::epoch_config()
            .position(now_utc)
            .expect("checked integration configuration");
        let first = Epoch(current.epoch.0.saturating_add(1));
        self.schedule_epoch(first, initial);
        let mut wake_at = self.schedule_wake(first, &initial);
        let mut time_states = common::TIME_RESOURCES
            .state_receiver()
            .expect("time state watcher capacity");

        loop {
            match select3(
                self.events.receive(),
                time_states.changed(),
                Timer::at(wake_at),
            )
            .await
            {
                Either3::First(event) => self.handle_event(event).await,
                Either3::Second(state) => {
                    self.handle_time_transition(state);
                    if self.role.is_base_station() {
                        if state.utc_status == UtcStatus::Invalid {
                            self.base_rx_horizon = Instant::now() + PRE_UTC_BASE_RX;
                        }
                        self.ensure_base_rx();
                    }
                    if state.utc_status != UtcStatus::Invalid
                        && self.last_scheduled_epoch_is_past(&state)
                    {
                        let target = next_complete_epoch(&state).unwrap_or(Epoch(0));
                        self.schedule_epoch(target, state);
                        wake_at = self.schedule_wake(target, &state);
                    }
                }
                Either3::Third(_) => {
                    self.finish_epoch();
                    let state = common::TIME_RESOURCES.time_state();
                    if state.utc_status == UtcStatus::Invalid {
                        if self.role.is_base_station() {
                            self.base_rx_horizon = Instant::now() + PRE_UTC_BASE_RX;
                            self.ensure_base_rx();
                        }
                        wake_at = Instant::now() + PRE_UTC_BASE_RX;
                        continue;
                    }
                    let next = self
                        .last_scheduled
                        .map(|epoch| Epoch(epoch.0.saturating_add(1)))
                        .unwrap_or_else(|| next_complete_epoch(&state).unwrap_or(Epoch(0)));
                    let target = match config::epoch_config()
                        .position(state.system_to_utc(Instant::now()).unwrap_or_default())
                    {
                        Ok(position) if next.0 <= position.epoch.0 => {
                            Epoch(position.epoch.0.saturating_add(1))
                        }
                        _ => next,
                    };
                    self.schedule_epoch(target, state);
                    wake_at = self.schedule_wake(target, &state);
                }
            }
        }
    }

    async fn wait_for_utc(&mut self) -> TimeState {
        loop {
            let state = common::TIME_RESOURCES.time_state();
            if state.utc_status != UtcStatus::Invalid && state.system_to_utc(Instant::now()).is_ok()
            {
                self.handle_time_transition(state);
                return state;
            }
            if self.role.is_base_station() {
                self.base_rx_horizon = Instant::now() + PRE_UTC_BASE_RX;
                self.ensure_base_rx();
                match select(self.events.receive(), Timer::after_millis(500)).await {
                    Either::First(event) => self.handle_event(event).await,
                    Either::Second(_) => {}
                }
            } else {
                Timer::after_millis(500).await;
            }
        }
    }

    fn handle_time_transition(&mut self, state: TimeState) {
        if state.utc_status != self.last_time_status {
            diagnostics::emit(DiagnosticKind::TimeTransition {
                previous: self.last_time_status,
                current: state.utc_status,
            });
            if state.utc_status == UtcStatus::Invalid
                || state.uncertainty_us > config::NARROW_RENDEZVOUS_THRESHOLD.as_micros()
            {
                self.force_scan = true;
            }
            if state.utc_status == UtcStatus::Invalid {
                self.cancel_timed_jobs();
            }
            self.last_time_status = state.utc_status;
        }
    }

    fn schedule_epoch(&mut self, epoch: Epoch, time: TimeState) {
        if self.last_scheduled == Some(epoch) {
            return;
        }
        if time.utc_status == UtcStatus::Invalid {
            diagnostics::emit(DiagnosticKind::WindowSkipped {
                epoch,
                reason: ScheduleError::UtcUnavailable,
            });
            return;
        }
        self.prune_neighbours(time);

        let presence_slot =
            match config::absolute_slot(self.node_id, epoch, RendezvousPurpose::Presence) {
                Ok(slot) => slot,
                Err(error) => {
                    diagnostics::emit(DiagnosticKind::WindowSkipped {
                        epoch,
                        reason: error,
                    });
                    return;
                }
            };
        let heartbeat_slot =
            match config::absolute_slot(self.node_id, epoch, RendezvousPurpose::Heartbeat) {
                Ok(slot) => slot,
                Err(error) => {
                    diagnostics::emit(DiagnosticKind::WindowSkipped {
                        epoch,
                        reason: error,
                    });
                    return;
                }
            };

        let presence_time = config::epoch_config()
            .slot_time(epoch, presence_slot)
            .expect("validated local presence slot");
        let heartbeat_time = config::epoch_config()
            .slot_time(epoch, heartbeat_slot)
            .expect("validated local heartbeat slot");
        self.submit_presence(epoch, presence_slot, presence_time, &time);
        self.submit_heartbeat(epoch, heartbeat_slot, heartbeat_time, &time);

        let scan = self.completed_epochs < config::STARTUP_SCAN_EPOCHS
            || !self.base_station_known()
            || self.force_scan
            || time.uncertainty_us > config::NARROW_RENDEZVOUS_THRESHOLD.as_micros()
            || epoch.0.is_multiple_of(config::PERIODIC_SCAN_EPOCHS);
        let receive_windows = if self.role.is_base_station() {
            let epoch_end = config::epoch_config()
                .epoch_start(Epoch(epoch.0.saturating_add(1)))
                .ok()
                .and_then(|utc| time.utc_to_system(utc).ok())
                .unwrap_or_else(|| Instant::now() + config::EPOCH_DURATION);
            self.base_rx_horizon = epoch_end;
            self.ensure_base_rx();
            1
        } else {
            self.schedule_node_rx(epoch, &time, scan, presence_time, heartbeat_time)
        };

        diagnostics::emit(DiagnosticKind::EpochScheduled {
            epoch,
            scan,
            presence_slot,
            heartbeat_slot,
            receive_windows,
        });
        info!(
            "Integration004 epoch={} role_base={} scan={} presence_slot={} heartbeat_slot={} rx_windows={}",
            epoch.0,
            self.role.is_base_station(),
            scan,
            presence_slot,
            heartbeat_slot,
            receive_windows,
        );
        self.force_scan = false;
        self.last_scheduled = Some(epoch);
    }

    fn submit_presence(
        &mut self,
        epoch: Epoch,
        slot: u32,
        slot_utc: UtcTimestamp,
        time: &TimeState,
    ) {
        let advert = PresenceAdvert {
            schedule_version: config::SCHEDULE_VERSION,
            capabilities: CapabilityFlags(if self.role.is_base_station() {
                config::BASE_STATION_CAPABILITY
            } else {
                0
            }),
        };
        let mut payload = [0u8; PresenceAdvert::ENCODED_LEN];
        if advert.encode(&mut payload).is_err() {
            return;
        }
        self.submit_frame(
            epoch,
            slot,
            slot_utc,
            RendezvousPurpose::Presence,
            FrameType::Presence,
            &payload,
            time,
        );
    }

    fn submit_heartbeat(
        &mut self,
        epoch: Epoch,
        slot: u32,
        slot_utc: UtcTimestamp,
        time: &TimeState,
    ) {
        let heartbeat = Heartbeat::from_service_states(
            *time,
            LOCATION.state(),
            Instant::now(),
            BatterySoc::new(None),
            ChargingState::Unknown,
            ErrorFlags(0),
            StorageUsage::new(None),
        );
        let mut payload = [0u8; 32];
        let Ok(length) = heartbeat.encode(&mut payload) else {
            return;
        };
        self.submit_frame(
            epoch,
            slot,
            slot_utc,
            RendezvousPurpose::Heartbeat,
            FrameType::Heartbeat,
            &payload[..length],
            time,
        );
    }

    #[allow(clippy::too_many_arguments)]
    fn submit_frame(
        &mut self,
        epoch: Epoch,
        slot: u32,
        slot_utc: UtcTimestamp,
        purpose: RendezvousPurpose,
        frame_type: FrameType,
        payload: &[u8],
        time: &TimeState,
    ) {
        if time.utc_status == UtcStatus::Invalid
            || time.uncertainty_us > config::MAXIMUM_TX_UNCERTAINTY.as_micros()
        {
            diagnostics::emit(DiagnosticKind::TxSubmitFailed {
                epoch,
                purpose,
                slot,
                error: if time.utc_status == UtcStatus::Invalid {
                    ScheduleError::UtcUnavailable
                } else {
                    ScheduleError::UtcUncertaintyTooHigh
                },
            });
            return;
        }
        let Ok(start) = time.utc_to_system(slot_utc) else {
            diagnostics::emit(DiagnosticKind::TxSubmitFailed {
                epoch,
                purpose,
                slot,
                error: ScheduleError::UtcUnavailable,
            });
            return;
        };
        let sequence = self.sequence.take();
        let frame = match FrameBuffer::encode(
            FrameHeader {
                frame_type,
                source: self.node_id,
                boot_id: self.sequence.boot_id(),
                sequence,
                destination: None,
            },
            payload,
        ) {
            Ok(frame) => frame,
            Err(_) => {
                diagnostics::emit(DiagnosticKind::TxSubmitFailed {
                    epoch,
                    purpose,
                    slot,
                    error: ScheduleError::InvalidSchedule,
                });
                return;
            }
        };
        let end = start + config::TX_RESERVATION;
        match self.handle.try_submit_tx(RadioTxJob {
            earliest: start,
            deadline: end,
            profile: self.profile.clone(),
            priority: RadioPriority::Control,
            payload: frame,
        }) {
            Ok(id) => {
                let context = TxContext {
                    id,
                    epoch,
                    purpose,
                    slot,
                    sequence,
                    start,
                };
                if self.tx.push(context).is_err() {
                    warn!("Integration004 TX context capacity exhausted");
                }
                diagnostics::emit(DiagnosticKind::TxSubmitted {
                    id,
                    epoch,
                    purpose,
                    slot,
                    sequence,
                });
            }
            Err(error) => {
                diagnostics::emit(DiagnosticKind::TxSubmitFailed {
                    epoch,
                    purpose,
                    slot,
                    error,
                });
            }
        }
    }

    fn schedule_node_rx(
        &mut self,
        epoch: Epoch,
        time: &TimeState,
        scan: bool,
        presence_time: UtcTimestamp,
        heartbeat_time: UtcTimestamp,
    ) -> u8 {
        let epoch_start = match config::epoch_config().epoch_start(epoch) {
            Ok(value) => value,
            Err(_) => return 0,
        };
        let active_end = UtcTimestamp::from_micros(
            epoch_start
                .as_micros()
                .saturating_add(config::ACTIVE_WINDOW.as_micros() as i64),
        );
        let mut windows = Vec::<UtcWindow, WINDOW_CAPACITY>::new();
        let mode = if time.uncertainty_us > config::NARROW_RENDEZVOUS_THRESHOLD.as_micros() {
            ReceiveMode::Unverifiable
        } else if scan {
            ReceiveMode::Scan
        } else {
            ReceiveMode::Predicted
        };

        if scan {
            let _ = windows.push(UtcWindow {
                start_us: epoch_start.as_micros(),
                end_us: active_end.as_micros(),
            });
        } else {
            let guard = policy::rendezvous_guard(Duration::from_micros(time.uncertainty_us));
            for neighbour in self.neighbours.iter() {
                for purpose in [RendezvousPurpose::Presence, RendezvousPurpose::Heartbeat] {
                    if let Ok(mut window) =
                        policy::predicted_window(neighbour.node_id, epoch, purpose, guard)
                    {
                        window.start_us = window.start_us.max(epoch_start.as_micros());
                        window.end_us = window.end_us.min(active_end.as_micros());
                        if window.end_us > window.start_us {
                            let _ = windows.push(window);
                        }
                    }
                }
            }
        }

        let block_guard = config::PREPARATION_GUARD.as_micros() as i64;
        let tx_duration = config::TX_RESERVATION.as_micros() as i64;
        let mut blocks = Vec::<UtcWindow, 2>::new();
        for slot in [presence_time, heartbeat_time] {
            let _ = blocks.push(UtcWindow {
                start_us: slot
                    .as_micros()
                    .saturating_sub(block_guard)
                    .max(epoch_start.as_micros()),
                end_us: slot
                    .as_micros()
                    .saturating_add(tx_duration)
                    .saturating_add(block_guard)
                    .min(active_end.as_micros()),
            });
        }
        let Ok(windows) =
            policy::subtract_windows::<WINDOW_CAPACITY, 2, WINDOW_CAPACITY>(&windows, &blocks)
        else {
            return 0;
        };
        let mut submitted = 0u8;
        for window in windows {
            let (Ok(start), Ok(end)) = (
                time.utc_to_system(window.start()),
                time.utc_to_system(window.end()),
            ) else {
                continue;
            };
            if self.submit_rx(start, end, Some(epoch), mode, RxPurpose::Broadcast, 0) {
                submitted = submitted.saturating_add(1);
            }
        }
        submitted
    }

    fn submit_rx(
        &mut self,
        start: Instant,
        end: Instant,
        epoch: Option<Epoch>,
        mode: ReceiveMode,
        purpose: RxPurpose,
        receptions: u16,
    ) -> bool {
        if end <= start || start < Instant::now() + config::PREPARATION_GUARD {
            return false;
        }
        match self.handle.try_reserve_rx(RadioRxJob {
            start,
            end,
            profile: self.profile.clone(),
            priority: RadioPriority::BestEffort,
            purpose,
        }) {
            Ok(id) => {
                if self
                    .rx
                    .push(RxContext {
                        id,
                        epoch,
                        end,
                        mode,
                        receptions,
                    })
                    .is_err()
                {
                    warn!("Integration004 RX context capacity exhausted");
                }
                diagnostics::emit(DiagnosticKind::RxWindow {
                    id,
                    opened: true,
                    scan: matches!(mode, ReceiveMode::Scan | ReceiveMode::Unverifiable),
                    predicted: mode == ReceiveMode::Predicted,
                    receptions,
                });
                true
            }
            Err(error) => {
                diagnostics::emit(DiagnosticKind::WindowSkipped {
                    epoch: epoch.unwrap_or_default(),
                    reason: error,
                });
                false
            }
        }
    }

    fn ensure_base_rx(&mut self) {
        if !self.role.is_base_station()
            || self
                .rx
                .iter()
                .any(|context| context.mode == ReceiveMode::Promiscuous)
        {
            return;
        }
        let now = Instant::now();
        let start = now + RADIO_REARM_LEAD;
        let next_tx = self
            .tx
            .iter()
            .filter(|context| context.start > start)
            .min_by_key(|context| context.start)
            .copied();
        let mut end = self.base_rx_horizon;
        if let Some(tx) = next_tx {
            end = end.min(Instant::from_ticks(
                tx.start
                    .as_ticks()
                    .saturating_sub(config::PREPARATION_GUARD.as_ticks()),
            ));
        }
        end = end.min(start + MAX_BASE_RX_RESERVATION);
        if end <= start + config::PREPARATION_GUARD {
            return;
        }
        let epoch = common::TIME_RESOURCES
            .time_state()
            .system_to_utc(start)
            .ok()
            .and_then(|utc| config::epoch_config().position(utc).ok())
            .map(|position| position.epoch);
        self.submit_rx(
            start,
            end,
            epoch,
            ReceiveMode::Promiscuous,
            RxPurpose::Broadcast,
            0,
        );
    }

    async fn handle_event(&mut self, event: RadioEvent) {
        match event {
            RadioEvent::Admitted { .. } => {}
            RadioEvent::Completed { id } => {
                if let Some(tx) = self.take_tx(id) {
                    diagnostics::emit(DiagnosticKind::TxCompleted {
                        id,
                        epoch: tx.epoch,
                        purpose: tx.purpose,
                        slot: tx.slot,
                        sequence: tx.sequence,
                    });
                    indication::transmit();
                }
                self.ensure_base_rx();
            }
            RadioEvent::Received {
                id,
                frame,
                metadata,
            } => {
                let mut context = self.take_rx(id);
                let valid = self.process_received(frame, metadata, context);
                if valid {
                    if let Some(context) = context.as_mut() {
                        context.receptions = context.receptions.saturating_add(1);
                    }
                }
                if let Some(context) = context {
                    self.rearm(context);
                }
                self.ensure_base_rx();
            }
            RadioEvent::RxWindowClosed { id } => {
                let context = self.take_rx(id);
                if context
                    .map(|value| value.mode == ReceiveMode::Predicted && value.receptions == 0)
                    .unwrap_or(false)
                {
                    self.local.predicted_misses = self.local.predicted_misses.saturating_add(1);
                }
                diagnostics::emit(DiagnosticKind::RxWindow {
                    id,
                    opened: false,
                    scan: context
                        .map(|value| {
                            matches!(value.mode, ReceiveMode::Scan | ReceiveMode::Unverifiable)
                        })
                        .unwrap_or(false),
                    predicted: context
                        .map(|value| value.mode == ReceiveMode::Predicted)
                        .unwrap_or(false),
                    receptions: context.map(|value| value.receptions).unwrap_or(0),
                });
                self.ensure_base_rx();
            }
            RadioEvent::PacketRejected { id } => {
                let context = self.take_rx(id);
                diagnostics::emit(DiagnosticKind::PacketRejected { id });
                if let Some(context) = context {
                    self.rearm(context);
                }
                self.ensure_base_rx();
            }
            RadioEvent::Cancelled { id } => {
                self.take_tx(id);
                self.take_rx(id);
                self.ensure_base_rx();
            }
            RadioEvent::Rejected { id, error, .. } | RadioEvent::Failed { id, error } => {
                if let Some(tx) = self.take_tx(id) {
                    diagnostics::emit(DiagnosticKind::TxRejected {
                        id,
                        epoch: tx.epoch,
                        purpose: tx.purpose,
                        slot: tx.slot,
                        error,
                    });
                } else {
                    self.take_rx(id);
                    diagnostics::emit(DiagnosticKind::RadioFailure { id, error });
                }
                self.ensure_base_rx();
            }
        }
    }

    fn rearm(&mut self, context: RxContext) {
        let start = Instant::now() + RADIO_REARM_LEAD;
        if context.end <= start + config::PREPARATION_GUARD {
            return;
        }
        self.submit_rx(
            start,
            context.end,
            context.epoch,
            context.mode,
            RxPurpose::Broadcast,
            context.receptions,
        );
    }

    fn process_received(
        &mut self,
        frame: FrameBuffer,
        metadata: raylar_radio_service::DriverPacketMetadata,
        context: Option<RxContext>,
    ) -> bool {
        let Ok(decoded) = FrameHeader::decode(frame.as_slice()) else {
            self.local.application_malformed = self.local.application_malformed.saturating_add(1);
            return false;
        };
        indication::packet_received(decoded.header.frame_type);
        let time = common::TIME_RESOURCES.time_state();
        let received_utc = time.system_to_utc(metadata.packet_complete_at).ok();
        let epoch = received_utc
            .and_then(|utc| config::epoch_config().position(utc).ok())
            .map(|position| position.epoch);
        let purpose = match decoded.header.frame_type {
            FrameType::Presence => Some(RendezvousPurpose::Presence),
            FrameType::Heartbeat => Some(RendezvousPurpose::Heartbeat),
            FrameType::Data => None,
        };
        let expected_slot = epoch.and_then(|epoch| {
            purpose.and_then(|purpose| {
                config::absolute_slot(decoded.header.source, epoch, purpose).ok()
            })
        });
        let was_scan = context
            .map(|value| matches!(value.mode, ReceiveMode::Scan))
            .unwrap_or(false);
        let class = match (epoch, purpose) {
            (Some(epoch), Some(purpose)) => policy::classify_observation(
                decoded.header.source,
                epoch,
                purpose,
                received_utc,
                time.utc_status,
                Duration::from_micros(time.uncertainty_us),
                was_scan,
                false,
            ),
            _ => WindowClass::Unverifiable,
        };
        let mut valid = false;
        match decoded.header.frame_type {
            FrameType::Presence => {
                valid =
                    self.process_presence(frame.as_slice(), decoded.header, received_utc, metadata);
            }
            FrameType::Heartbeat => {
                valid = self.process_heartbeat(
                    frame.as_slice(),
                    decoded.header,
                    received_utc,
                    metadata,
                );
            }
            FrameType::Data => {
                self.local.application_malformed =
                    self.local.application_malformed.saturating_add(1);
            }
        }
        if valid {
            match class {
                WindowClass::Predicted => {
                    self.local.predicted_rx = self.local.predicted_rx.saturating_add(1)
                }
                WindowClass::Scan => self.local.scan_rx = self.local.scan_rx.saturating_add(1),
                WindowClass::Outside => {
                    self.local.outside_rx = self.local.outside_rx.saturating_add(1)
                }
                WindowClass::Unverifiable | WindowClass::SchedulerConflict => {
                    self.local.unverifiable_rx = self.local.unverifiable_rx.saturating_add(1)
                }
            }
        }
        diagnostics::emit(DiagnosticKind::Rx {
            frame_type: decoded.header.frame_type,
            source: decoded.header.source,
            source_boot: decoded.header.boot_id,
            sequence: decoded.header.sequence,
            epoch,
            expected_slot,
            class,
            rssi_dbm_x2: metadata.rssi_dbm_x2,
            snr_db_x4: metadata.snr_db_x4,
            valid,
        });
        valid
    }

    fn process_presence(
        &mut self,
        frame: &[u8],
        header: FrameHeader,
        received_utc: Option<UtcTimestamp>,
        metadata: raylar_radio_service::DriverPacketMetadata,
    ) -> bool {
        let Ok((_, advert)) = PresenceAdvert::decode_frame(frame) else {
            self.local.application_malformed = self.local.application_malformed.saturating_add(1);
            return false;
        };
        if advert.schedule_version != config::SCHEDULE_VERSION {
            self.local.application_malformed = self.local.application_malformed.saturating_add(1);
            return false;
        }
        if header.source == self.node_id {
            return true;
        }
        let Some(received_utc) = received_utc else {
            return true;
        };
        let previous = self.neighbours.get(header.source).copied();
        if self
            .neighbours
            .observe_presence_frame_with_gfsk_status(
                frame,
                received_utc,
                metadata.packet_complete_at,
                self.profile.id(),
                metadata.rssi_dbm_x2,
                metadata.snr_db_x4,
                metadata.gfsk_status,
            )
            .is_err()
        {
            self.local.application_malformed = self.local.application_malformed.saturating_add(1);
            return false;
        }
        self.update_peer(header.source, header.boot_id, advert.capabilities);
        RADIO.set_neighbour_count(self.neighbours.len());
        let base_station = advert.capabilities.0 & config::BASE_STATION_CAPABILITY != 0;
        let boot_changed = previous
            .map(|entry| entry.boot_id != header.boot_id)
            .unwrap_or(false);
        if base_station && boot_changed {
            self.force_scan = true;
        }
        diagnostics::emit(DiagnosticKind::NeighbourChanged {
            node: header.source,
            boot: header.boot_id,
            discovered: previous.is_none(),
            boot_changed,
            base_station,
            count: self.neighbours.len() as u16,
        });
        let base_count = self.base_station_count();
        if base_count > 1 {
            diagnostics::emit(DiagnosticKind::TopologyWarning {
                base_station_count: base_count,
            });
        }
        true
    }

    fn process_heartbeat(
        &mut self,
        frame: &[u8],
        header: FrameHeader,
        received_utc: Option<UtcTimestamp>,
        metadata: raylar_radio_service::DriverPacketMetadata,
    ) -> bool {
        let Ok((_, heartbeat)) = Heartbeat::decode_frame(frame) else {
            self.local.application_malformed = self.local.application_malformed.saturating_add(1);
            return false;
        };
        self.local.heartbeat_rx = self.local.heartbeat_rx.saturating_add(1);
        if header.source == self.node_id {
            return true;
        }
        let Some(existing) = self.neighbours.get(header.source).copied() else {
            self.local.unknown_heartbeat_rx = self.local.unknown_heartbeat_rx.saturating_add(1);
            return true;
        };
        if existing.boot_id != header.boot_id {
            self.local.unknown_heartbeat_rx = self.local.unknown_heartbeat_rx.saturating_add(1);
            return true;
        }
        let Some(received_utc) = received_utc else {
            return true;
        };
        let observation = LinkObservation {
            peer: header.source,
            profile: self.profile.id(),
            rssi_dbm_x2: Some(metadata.rssi_dbm_x2),
            snr_db_x4: metadata.snr_db_x4,
            gfsk_status: metadata.gfsk_status,
            outcome: LinkOutcome::Received,
            observed_at: metadata.packet_complete_at,
        };
        let mut link_state = PassiveLinkState::default();
        link_state.observe(observation);
        self.neighbours.observe(
            NeighbourEntry {
                node_id: existing.node_id,
                boot_id: existing.boot_id,
                last_seen_utc: received_utc,
                location: heartbeat.location.or(existing.location),
                location_uncertainty_meters: existing.location_uncertainty_meters,
                schedule_version: existing.schedule_version,
                last_rssi_dbm_x2: Some(metadata.rssi_dbm_x2),
                last_snr_db_x4: metadata.snr_db_x4,
                link_state,
            },
            received_utc,
        );
        true
    }

    fn update_peer(&mut self, node_id: NodeId, boot_id: BootId, capabilities: CapabilityFlags) {
        if let Some(peer) = self.peers.iter_mut().find(|peer| peer.node_id == node_id) {
            peer.boot_id = boot_id;
            peer.capabilities = capabilities;
            return;
        }
        if self
            .peers
            .push(PeerMeta {
                node_id,
                boot_id,
                capabilities,
            })
            .is_err()
        {
            if let Some((index, _)) = self
                .peers
                .iter()
                .enumerate()
                .find(|(_, peer)| self.neighbours.get(peer.node_id).is_none())
            {
                self.peers[index] = PeerMeta {
                    node_id,
                    boot_id,
                    capabilities,
                };
            }
        }
    }

    fn base_station_count(&self) -> u8 {
        self.peers
            .iter()
            .filter(|peer| {
                self.neighbours.get(peer.node_id).is_some()
                    && peer.capabilities.0 & config::BASE_STATION_CAPABILITY != 0
            })
            .count()
            .min(u8::MAX as usize) as u8
    }

    fn base_station_known(&self) -> bool {
        self.base_station_count() != 0
    }

    fn take_tx(&mut self, id: JobId) -> Option<TxContext> {
        let index = self.tx.iter().position(|context| context.id == id)?;
        Some(self.tx.swap_remove(index))
    }

    fn take_rx(&mut self, id: JobId) -> Option<RxContext> {
        let index = self.rx.iter().position(|context| context.id == id)?;
        Some(self.rx.swap_remove(index))
    }

    fn cancel_timed_jobs(&mut self) {
        let mut ids = Vec::<JobId, JOB_DEPTH>::new();
        for context in &self.tx {
            let _ = ids.push(context.id);
        }
        for context in &self.rx {
            let _ = ids.push(context.id);
        }
        for id in ids {
            if let Err(error) = self.handle.try_cancel(id) {
                diagnostics::emit(DiagnosticKind::RadioFailure {
                    id,
                    error: RadioServiceError::Schedule(error),
                });
            }
        }
    }

    fn prune_neighbours(&mut self, time: TimeState) {
        let Ok(now) = time.system_to_utc(Instant::now()) else {
            return;
        };
        let expired = self.neighbours.expire(now);
        if expired != 0 {
            self.force_scan = true;
            self.peers
                .retain(|peer| self.neighbours.get(peer.node_id).is_some());
            RADIO.set_neighbour_count(self.neighbours.len());
            diagnostics::emit(DiagnosticKind::NeighboursExpired {
                count: expired.min(u16::MAX as usize) as u16,
            });
        }
    }

    fn finish_epoch(&mut self) {
        let epoch = self.last_scheduled.unwrap_or_default();
        if self.last_finished == Some(epoch) {
            return;
        }
        self.last_finished = Some(epoch);
        self.completed_epochs = self.completed_epochs.saturating_add(1);
        let state = RADIO.state();
        diagnostics::emit(DiagnosticKind::NeighbourTable {
            epoch,
            count: self.neighbours.len().min(u16::MAX as usize) as u16,
        });
        for (index, entry) in self.neighbours.iter().enumerate() {
            let base_station = self
                .peers
                .iter()
                .find(|peer| peer.node_id == entry.node_id)
                .map(|peer| peer.capabilities.0 & config::BASE_STATION_CAPABILITY != 0)
                .unwrap_or(false);
            diagnostics::emit(DiagnosticKind::NeighbourEntry {
                epoch,
                index: index.min(u16::MAX as usize) as u16,
                node: entry.node_id,
                boot: entry.boot_id,
                base_station,
                schedule_version: entry.schedule_version.0,
                last_seen_utc: entry.last_seen_utc,
                location: entry.location,
                location_uncertainty_meters: entry.location_uncertainty_meters,
                rssi_dbm_x2: entry.last_rssi_dbm_x2,
                snr_db_x4: entry.last_snr_db_x4,
                received_packets: entry.link_state.received_packets,
                failed_packets: entry.link_state.failed_packets,
            });
        }
        diagnostics::emit(DiagnosticKind::Summary {
            epoch,
            mode: state.mode,
            radio: state.stats,
            heartbeat_rx: self.local.heartbeat_rx,
            unknown_heartbeat_rx: self.local.unknown_heartbeat_rx,
            predicted_rx: self.local.predicted_rx,
            predicted_misses: self.local.predicted_misses,
            scan_rx: self.local.scan_rx,
            outside_rx: self.local.outside_rx,
            unverifiable_rx: self.local.unverifiable_rx,
            application_malformed: self.local.application_malformed,
            diagnostic_drops: diagnostics::enqueue_drops(),
            logging_drops: diagnostics::logging_drops(),
            logging_truncations: diagnostics::logging_truncations(),
        });
    }

    fn schedule_wake(&self, epoch: Epoch, time: &TimeState) -> Instant {
        config::epoch_config()
            .epoch_start(epoch)
            .ok()
            .map(|start| {
                UtcTimestamp::from_micros(
                    start
                        .as_micros()
                        .saturating_add(config::ACTIVE_WINDOW.as_micros() as i64),
                )
            })
            .and_then(|utc| time.utc_to_system(utc).ok())
            .unwrap_or_else(|| Instant::now() + PRE_UTC_BASE_RX)
    }

    fn last_scheduled_epoch_is_past(&self, time: &TimeState) -> bool {
        let (Some(last), Ok(now)) = (self.last_scheduled, time.system_to_utc(Instant::now()))
        else {
            return self.last_scheduled.is_none();
        };
        config::epoch_config()
            .position(now)
            .map(|position| position.epoch.0 > last.0)
            .unwrap_or(false)
    }
}

fn next_complete_epoch(time: &TimeState) -> Option<Epoch> {
    let now = time.system_to_utc(Instant::now()).ok()?;
    let position = config::epoch_config().position(now).ok()?;
    Some(Epoch(position.epoch.0.saturating_add(1)))
}
