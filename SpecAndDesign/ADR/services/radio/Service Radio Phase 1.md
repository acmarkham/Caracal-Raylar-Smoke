# Radio Messaging Service Architecture

## Status

Proposed

---

# Context

The Raylar platform has a reusable LR1121 radio driver that owns the Ebyte E80 hardware and exposes timed transmit/receive operations, channel configuration, receive metadata, power-state management, and error recovery.

The radio driver is intentionally not a networking service. Addressing, framing, reliability, retries, TDMA scheduling, UTC conversion, regulatory policy, and multi-client arbitration belong above the driver.

The driver also deliberately has a single owner. A higher-level service is expected to own the driver and arbitrate multiple networking clients rather than allowing independent users to access the hardware directly.

The platform already provides two important services required by the networking layer:

- a **Time Service**, which maintains the authoritative mapping between local monotonic system time and UTC, including uncertainty and holdover behaviour; and
- a **Location Service**, which publishes a stable application-facing geographic position and associated validity/uncertainty.

The Time Service is the sole owner of UTC estimation. Radio protocols shall consume its UTC conversion and uncertainty APIs rather than performing independent GPS or clock estimation.

The Location Service explicitly provides geographic information for future routing and communications while leaving routing policy to those downstream services.

The intended radio networking capability will be developed incrementally:

1. unreliable heartbeat and peer discovery;
2. reliable tree-based messaging using contention-free TDMA;
3. adaptive link estimation and multiband/channel selection;
4. full mesh and geographically informed routing;
5. higher-level communication policies, potentially including learned policies for in-network optimisation.

The architecture must permit this evolution without requiring the fundamental radio-service boundary to be replaced at each phase.

---

# Decision

Introduce a dedicated **Radio Messaging Service** above the LR1121 driver.

The Radio Messaging Service shall be the sole owner of the radio driver and shall provide a common scheduling, framing, addressing, neighbour, and link-selection substrate to higher-level networking protocols.

Higher-level protocols such as heartbeat/discovery, tree transport, mesh routing, and collaborative communication shall be implemented as modular subservices using this substrate.

Conceptually:

```text
Application Services
        |
        | semantic messages
        v
+--------------------------------------+
|          Messaging / Policy          |
|                                      |
| Fixed / heuristic / future learned   |
| communication policies               |
+-------------------+------------------+
                    |
                    v
+--------------------------------------+
|         Network Subservices          |
|                                      |
| Heartbeat / Presence                 |
| Reliable Tree                        |
| Mesh / Geographic Routing            |
| Collaborative Groups                 |
+-------------------+------------------+
                    |
                    v
+--------------------------------------+
|        Radio Messaging Service       |
|                                      |
| Message queues                       |
| Frame codec                          |
| Neighbour state                      |
| Link Estimator                       |
| Slot / rendezvous scheduling         |
| Radio arbitration                    |
| Reliability primitives              |
+----------+---------------------------+
           |
           | monotonic deadlines
           v
+---------------------+
|   LR1121 Driver     |
+---------------------+

       ^                         ^
       |                         |
   Time Service             Location Service
 UTC <-> system time       stable geographic state
 uncertainty
```

The fundamental rule is:

> There is one physical radio owner, but multiple independently modular protocol users.

Protocols are therefore atomic at the logical level, not at the hardware-ownership level.

---

# Design Goals

The Radio Messaging Service shall:

- preserve exclusive ownership of the LR1121 driver;
- be `no_std`, heapless, statically allocated, and Embassy-async;
- support multiple logical radio users without exposing the driver directly;
- expose semantic messages separately from physical radio packets;
- use UTC-synchronised scheduling where advantageous;
- consume time uncertainty from the Time Service when deriving receive/transmit guard intervals;
- support compact framing suitable for slow LoRa links;
- provide predictable rendezvous opportunities between nodes;
- support an always-defined common broadcast/control mechanism;
- maintain bounded soft-state knowledge of neighbouring nodes;
- support link-profile selection through a dedicated Link Estimator abstraction;
- permit static sub-GHz and 2.4 GHz LoRa/GFSK profiles initially and adaptive
  profile selection later;
- support hop-by-hop reliable communication without requiring end-to-end acknowledgement;
- allow different traffic classes to occupy different logical slots, channels, and reliability mechanisms;
- support future geographically informed routing;
- support future collaborative node groups;
- allow communication policy to evolve independently of the lower networking mechanisms;
- keep later advanced functionality separable into additional design documents.

---

# Non-Goals

The initial Radio Messaging Service will not:

- estimate UTC independently;
- communicate with GPS hardware directly;
- estimate device location independently;
- expose raw LR1121 commands to networking clients;
- implement end-to-end reliable transport;
- provide TCP-like sessions or streams;
- provide unrestricted dynamic radio configuration to applications;
- require continuous radio listening;
- implement full mesh routing in Phase I;
- implement active link probing in Phase I;
- implement geographically complete routing in Phase I;
- implement machine-learned communication policy in Phase I;
- guarantee security, authentication, or confidentiality in Phase I.

Security may be added as a separate protocol concern without changing the core ownership and scheduling architecture.

---

# External Service Dependencies

## Time Service

The Radio Messaging Service consumes the existing Time Service.

The Time Service already owns:

```text
System Time <-------> UTC Time
```

and provides UTC validity and uncertainty.

Conceptually, the Radio Messaging Service requires:

```rust
utc_to_system(UtcTimestamp)
system_to_utc(Instant)
current_utc()
time_state()
```

The LR1121 driver itself remains strictly monotonic-time based. This preserves the driver boundary already defined: UTC slot boundaries are converted by the higher networking layer to local monotonic deadlines before radio operations are submitted.

The service shall not assume a fixed clock error such as ±20 ms.

Instead, scheduling shall derive guard intervals from the uncertainty currently reported by the Time Service.

Conceptually:

```text
RX guard =
      local UTC uncertainty
    + expected remote uncertainty
    + radio scheduling uncertainty
    + propagation allowance
    + engineering margin
```

Initial implementations may use conservative fixed margins in addition to the Time Service uncertainty.

---

## Location Service

The Radio Messaging Service consumes the Location Service when geographic information is required.

Phase I uses location primarily for heartbeat/status reporting.

Later phases may use it for:

- geographically informed neighbour state;
- geographic routing;
- gateway selection;
- geographically scoped broadcasts;
- collaborative acoustic-node selection.

The Radio Messaging Service shall not modify the location estimate or implement location filtering.

---

## Identity

The service requires a stable node identity.

The exact source and representation should reuse the platform's existing identity/traceability facility rather than inventing a second hardware identity mechanism.

The on-air representation should be deliberately compact.

The full manufacturing identity does not necessarily need to appear in every radio frame if a shorter deployment-unique `NodeId` can be provided.

---

# Core Architecture

The implementation should separate five concerns:

```text
Semantic Messaging
        |
Network Protocols
        |
Scheduling / Reliability / Neighbour State
        |
Link Estimation
        |
Radio Arbitration
        |
LR1121 Driver
```

These boundaries should correspond to sensible implementation modules rather than a single large service file.

---

# Radio Ownership and Arbitration

A single `RadioService` task shall own the LR1121 driver.

No heartbeat, tree, mesh, probing, or application task may call the driver directly.

Conceptually:

```rust
pub struct RadioService<D> {
    driver: D,
    // bounded queues
    // scheduler state
    // current radio state
    // statistics
}
```

Clients interact through a lightweight handle backed by bounded Embassy channels or equivalent statically allocated queues:

```rust
pub struct RadioHandle {
    // request channel sender
}
```

The internal scheduler accepts radio jobs such as:

```rust
pub enum RadioJob {
    Transmit(TxJob),
    Receive(RxJob),
}
```

A job may include:

```rust
pub struct TxJob {
    pub earliest: Instant,
    pub deadline: Instant,
    pub profile: ChannelProfile,
    pub priority: RadioPriority,
    pub payload: FrameBuffer,
}

pub struct RxJob {
    pub start: Instant,
    pub end: Instant,
    pub profile: ChannelProfile,
    pub purpose: RxPurpose,
}
```

The exact Rust API may evolve, but several properties are required:

- requests are bounded;
- requests have explicit timing;
- requests may be rejected if they cannot be scheduled safely;
- channel setup occurs before the slot where possible;
- radio conflicts are resolved centrally;
- higher-level clients never depend on LR1121-specific operation types.

The underlying radio driver already supports preparing expensive configuration in advance and issuing the final RX/TX operation close to a monotonic deadline.

---

# Message, Frame, and Packet Separation

The architecture shall distinguish three layers:

```text
Semantic Message
      |
      v
Network / Link Frame
      |
      v
LR1121 Radio Packet
```

A **message** expresses application intent.

Examples:

```text
Heartbeat
Presence Advertisement
Detection Summary
Configuration Update
Acoustic Collaboration Request
Acoustic Collaboration Result
```

A **frame** contains the addressing, protocol, reliability, scheduling, and message-fragment information necessary for network transport.

A **radio packet** is the PHY-level payload passed to the LR1121 driver.

Applications shall not depend directly on:

- LoRa packet sizes;
- spreading factor;
- bandwidth;
- RF frequency;
- fragmentation;
- retransmissions;
- route selection.

---

# Message Metadata

Messages should carry sufficient semantic information for future policy decisions without exposing low-level RF parameters.

Conceptually:

```rust
pub struct MessageOptions {
    pub destination: Destination,
    pub class: MessageClass,
    pub priority: MessagePriority,
    pub reliability: Reliability,
    pub deadline: Option<UtcTimestamp>,
    pub expiry: Option<UtcTimestamp>,
}
```

Possible destinations may eventually include:

```rust
pub enum Destination {
    Gateway,
    Node(NodeId),
    Broadcast,
    Group(GroupId),
    // Future:
    // GeographicRegion(...)
}
```

Message classes provide a mechanism for different traffic types to coexist using different logical network resources:

```rust
pub enum MessageClass {
    Control,
    Presence,
    Telemetry,
    ReliableData,
    Collaborative,
}
```

These classes are semantic.

The scheduler and networking protocols decide which:

- time slots;
- RF profiles;
- routes;
- reliability mechanisms;

are appropriate for each class.

---

# Compact Common Frame Envelope

A common frame envelope shall be used across heartbeat, discovery, tree, and later mesh protocols.

The envelope must remain small because some messages will use low-data-rate LoRa modes where every transmitted byte has meaningful airtime and energy cost.

The wire format should therefore be explicitly compact rather than using a general self-describing serialization format.

A conceptual base header is:

```text
+----------------------+
| version / frame type |
+----------------------+
| flags                |
+----------------------+
| source node ID       |
+----------------------+
| sequence             |
+----------------------+
| optional fields...   |
+----------------------+
| payload              |
+----------------------+
```

The exact bit allocation shall be determined during Phase I implementation after the deployment-wide `NodeId` representation is known.

The following rules apply:

- broadcast frames shall not waste bytes carrying a broadcast destination field;
- destination fields appear only when required;
- acknowledgement fields appear only on reliable frame classes;
- mesh routing fields do not appear in Phase I frames;
- values have explicitly defined byte order and wire representation;
- Rust enum memory layout must never implicitly define the protocol;
- unknown protocol versions or frame types are rejected cleanly.

The first byte should encode enough information to determine the remainder of the header.

For example, it may combine:

```text
protocol version
frame class
optional-field flags
```

The protocol should reserve enough versioning capability to evolve without reserving large unused headers.

---

# Node Boot Identity and Sequence Numbers

Persistent monotonic packet counters are not required.

Each boot shall instead establish a `BootId`.

The pair:

```text
(NodeId, BootId)
```

identifies a node execution session.

Frames requiring duplicate detection can then use:

```text
(NodeId, BootId, Sequence)
```

where `Sequence` is monotonically increasing only within that boot.

The sequence counter may therefore live entirely in RAM.

`BootId` needs a very low probability of accidental reuse; it does not need to be monotonic.

Preferred sources are:

1. a platform hardware random-number source, where available. On this repo, it is supported by the Driver TRNG module that provides bootID directly;
2. another board-provided entropy source;
3. as a fallback, a hash combining several varying startup observations such as device identity, UTC/system startup time, ADC/audio noise, or GPS-derived observations.

The exact entropy implementation belongs in the platform/identity integration rather than the framing module.

A cryptographic guarantee is not required merely for reboot discrimination.

If future security protocols require nonces with cryptographic uniqueness, those requirements must be specified separately.

---

# Rendezvous Scheduling

UTC synchronisation allows nodes to predict each other's activity without continuously listening.

The service shall define a versioned deterministic rendezvous function.

Conceptually:

```text
slot =
    H(
        network identifier,
        schedule version,
        purpose,
        node identifier,
        epoch number,
        occurrence
    )
    mod slots_per_window
```

The function shall be:

- deterministic;
- computationally cheap;
- stable across firmware implementations;
- explicitly versioned;
- sufficiently well distributed for node populations of interest.

It is not required to be cryptographically secure.

Different `purpose` values ensure that the same node does not necessarily choose the same position for:

- heartbeat;
- presence advertisement;
- peer listening;
- other future contention-based opportunities.

Because every node knows UTC and the shared scheduling parameters, a peer that learns another node's identity can predict future rendezvous opportunities.

---

# Epochs and Broadcast Window

Phase I shall define a network epoch and a common broadcast window.

Example only:

```text
epoch duration:           5 minutes
broadcast active window:  1 minute
```

The actual durations shall be configurable.

The gateway may listen continuously throughout the broadcast window.

Battery-powered nodes may wake for only selected portions of the same window.

Heartbeat and presence traffic use the common broadcast mechanism and shall continue to exist even after scheduled tree or mesh communication is introduced.

This gives the network an independent bootstrap/control plane even when:

- a tree schedule is lost;
- a node reboots;
- a parent disappears;
- adaptive link state becomes stale.

---

# Heartbeat Service

The heartbeat service provides relatively rich node status primarily for gateway telemetry.

Heartbeats are:

- broadcast;
- unacknowledged;
- periodic;
- scheduled pseudo-randomly within an agreed broadcast window;
- permitted to be repeated for probabilistic reliability.

A node should derive one or more transmit opportunities from the deterministic rendezvous function.

For example:

```text
heartbeat_tx_0 = H(... occurrence = 0)
heartbeat_tx_1 = H(... occurrence = 1)
```

A minimum separation may be enforced between repetitions.

The number of repetitions shall be configurable.

---

# Heartbeat Payload

The Phase I heartbeat should support fields equivalent to:

```rust
pub struct Heartbeat {
    pub boot_id: BootId,

    pub location: Option<CompactLocation>,
    pub location_age: Option<Duration>,

    pub battery_soc: BatterySoc,
    pub charging_state: ChargingState,

    pub error_flags: ErrorFlags,

    pub storage_usage: StorageUsage,

    pub gps_status: GpsStatus,
}
```

The exact wire representation should be compact.

Potential charging states include:

```text
None
Solar
Usb
External
Unknown
```

GPS/time-related status should avoid duplicating the full Time Service state.

A compact representation may indicate:

```text
UTC valid
GPS/fix quality class
time uncertainty class
holdover state
```

where useful.

Raw high-resolution diagnostic information should not automatically be placed in every heartbeat.

Heartbeat fields may evolve under protocol versioning.

---

# Presence Advertisement Service

Presence advertisements are distinct from heartbeats.

A heartbeat answers approximately:

> What is the status of this node?

A presence advertisement answers approximately:

> I exist, and here is enough information to interact with me.

Presence advertisements should therefore normally be smaller and may occur at a different cadence.

A conceptual advertisement contains:

```rust
pub struct PresenceAdvert {
    pub boot_id: BootId,
    pub schedule_version: ScheduleVersion,
    pub capabilities: CapabilityFlags,
}
```

Optional fields may include:

```text
location summary
gateway/tree status
link protocol capabilities
schedule generation
```

The exact initial payload should remain minimal.

Heartbeat and presence may initially share some fields or even be emitted at the same cadence, but they shall remain different protocol concepts.

---

# Neighbour Table

Peer discovery is soft-state.

No permanent "join" event is required.

A node maintains a bounded neighbour table containing recently observed peers.

Conceptually:

```rust
pub struct NeighbourEntry {
    pub node_id: NodeId,
    pub boot_id: BootId,

    pub last_seen_utc: UtcTimestamp,

    pub location: Option<CompactLocation>,
    pub location_uncertainty: Option<LocationUncertainty>,

    pub schedule_version: ScheduleVersion,

    pub last_rssi: Option<Rssi>,
    pub last_snr: Option<Snr>,

    pub link_state: LinkState,
}
```

The implementation may keep additional rolling statistics.

Entries expire or degrade in confidence with age.

The table shall be fixed-capacity.

Replacement policy should favour useful/recent neighbours rather than panicking or allocating when full.

Possible policies include:

```text
oldest expired entry
then least recently heard
```

The exact policy should be documented and tested.

---

# Predicting Peer Activity

Neighbour discovery is more useful than simple presence tracking.

Once a node knows:

```text
peer NodeId
schedule version
network epoch configuration
```

it can calculate the peer's future deterministic rendezvous opportunities.

The neighbour subsystem should therefore expose functionality equivalent to:

```rust
next_presence_time(peer)
next_broadcast_tx_time(peer)
next_expected_listen_time(peer)
```

where the relevant protocol defines such a window.

These predictions should be derived rather than stored where practical.

---

# Link Estimator

Channel/profile selection belongs to a dedicated `LinkEstimator` subservice.

Higher protocols shall not directly select arbitrary:

```text
frequency
spreading factor
bandwidth
coding rate
TX power
```

Instead they request a link suitable for an intent.

Conceptually:

```rust
pub struct LinkRequest {
    pub target: LinkTarget,
    pub purpose: LinkPurpose,
    pub constraints: LinkConstraints,
    pub utc_slot: Option<UtcTimestamp>,
}

pub trait LinkEstimator {
    fn select_profile(
        &self,
        request: &LinkRequest,
    ) -> Result<ChannelProfile, LinkError>;
}
```

Examples of intent include:

```text
common robust broadcast
fast link to neighbour X
high-reliability link to neighbour X
low-energy link to neighbour X
collaborative-group channel
```

Constraints may later include:

```text
maximum airtime
minimum expected reliability
allowed bands
energy preference
latency requirement
```

---

# Channel Profiles

`ChannelProfile` is owned conceptually by the Link Estimator layer.

It represents a complete PHY configuration that can be applied to the radio driver.

Conceptually it resolves to:

```text
frequency
modulation
LoRa bandwidth, spreading factor, and coding rate, or GFSK bitrate, deviation,
  RX filter bandwidth, and pulse shape
TX power policy
other PHY compatibility parameters
```

The radio scheduler receives a fully resolved profile and does not need to understand why it was selected.

### 2.4 GHz LoRa and GFSK profiles

The service shall accept policy-approved, complete `ChannelProfile`s for both
LoRa and GFSK at 2400–2500 MHz and resolve them to the driver's matching
`ChannelConfig` and `TxConfig`. A profile's stable identifier shall distinguish
frequency, modulation, and all waveform/packet compatibility fields. Nodes
must agree on the exact profile and schedule before a transmit or receive
window; a node cannot infer a peer's modulation merely from the band. The
common bootstrap profile may remain sub-GHz while a configured 2.4 GHz
profile is used for a scheduled window. Profile changes include measured
retune, RF-path/calibration, and preparation guards, and the scheduler must
restore the agreed bootstrap profile for its next window.

Use the 2.4 GHz `RFIO_HF` rows of LR1121 datasheet Table 3-9 for the
reference receive profiles. The 125/250/500 kHz LoRa rows in that table
describe S-band operation; they are not the 2.4 GHz reference rows.

| Table 3-9 2.4 GHz condition | Typical RX sensitivity |
| --- | --- |
| LoRa BW406 kHz, SF5 / SF7 | -111 / -114 dBm |
| LoRa BW812 kHz, SF5 / SF7 | -108 / -112 dBm |
| 2-FSK 1.2 / 4.8 kb/s, 5 kHz deviation, 20 kHz nominal RX BW | -117 / -112 dBm |
| 2-FSK 38.4 kb/s, 40 kHz deviation, 160 kHz nominal RX BW | -103 dBm |
| 2-FSK 250 kb/s, 125 kHz deviation, 500 kHz nominal RX BW | -97.5 dBm |

Table 3-9 also characterizes LoRa BW406/BW812 at SF7/SF12 for rejection,
but does not specify CR or raw data rate. Table 3-7 supplies the wider
2.4 GHz LoRa BW203–BW812 capability and CR4/5 raw-rate endpoints:
SF12/BW203 at 0.476 kb/s and SF5/BW812 at 101.5 kb/s. The user manual
specifies BW203/BW406/BW812, SF5–SF12, short-interleaver CR4/5, 4/6,
4/7, 4/8, and long-interleaver CR4/5, 4/6, 4/8 with payload limits.
Preserve BW, SF, CR, and interleaver mode exactly; calculate packet airtime
separately from raw rate.

Table 3-7 gives general (G)FSK programmable limits of 0.6–300 kb/s and
0.6–200 kHz deviation, while Table 3-9's 2.4 GHz test points reach
250 kb/s. Preserve bitrate, deviation, RX filter, Gaussian pulse shape,
and packet format exactly. The Table 3-9 sensitivity results are for
unfiltered 2-FSK, so they are not GFSK sensitivity guarantees. GFSK has no
LoRa SF or CR.

The LR1121 filter command offers 19.5, 156.2, and up to 467 kHz DSB,
respectively, for the datasheet's nominal 20/160/500 kHz FSK conditions.
The driver permits the exact 250 kb/s / 125 kHz deviation / 467 kHz
filter combination as a documented exception to its conservative
`bitrate + 2 * deviation <= RX bandwidth` check. The service can construct
this profile explicitly for hardware characterization, but static deployment
policy shall not select it by default until packet error rate and frequency
error tolerance are measured on the fitted Ebyte board. Do not round a
requested bandwidth or substitute another bitrate, deviation, SF, or CR
without changing the declared profile and making that change explicit to
peers. Report unsupported profiles as errors rather than silently skipping
their scheduled windows.

GFSK profiles also require agreed preamble, sync word, address filtering,
packet length mode, CRC, whitening, and pulse shaping. TX and RX must use the
same packet contract, and received GFSK RSSI/status must feed the service's
normal receive, framing, and link-observation paths without requiring LoRa
SNR. Keep profile sets bounded and approved by the regional radio policy.

Sources: [Semtech LR1121 Datasheet, Rev 2.1, Tables 3-7 and 3-9](https://static6.arrow.com/aropdfconversion/558d7379c488375138d6317a5a5c06f1a144bd3/61252685.lr1121_v2_1_data_sheet.pdf)
and [Semtech LR1121 User Manual, Rev 1.1, Sections 8.3.1 and 8.5.1](https://www.mouser.com/pdfdocs/usermanual_lr1121_v1_1.pdf).

Phase I may implement the extreme static case:

```text
select_profile(any request) -> DEFAULT_PROFILE
```

A slightly richer initial implementation may define:

```text
BOOTSTRAP_BROADCAST_PROFILE
DEFAULT_DATA_PROFILE
```

without performing any adaptive estimation.

Static selection does not restrict the set of representable profiles to
sub-GHz LoRa. Add a GFSK profile construction path and the 2.4 GHz LoRa
bandwidths to the service/driver adapter before scheduling either modulation
on 2.4 GHz. The existing LoRa-only profile constructor and 62.5–500 kHz
bandwidth mapping do not cover these cases.

This allows the rest of the architecture to stabilise before link optimisation is introduced.

---

# Common Broadcast Profile

The network must always have a predictable bootstrap mechanism.

Therefore one profile or deterministic profile-selection rule shall be defined for common broadcast/control operation.

Every participating node must be able to determine this profile without first exchanging adaptive link state.

Later implementations may frequency-hop the common channel according to a globally deterministic rule, but such hopping must remain derivable from shared network state alone.

A node must not require a successful unicast negotiation merely to discover the broadcast channel.

---

# Passive Link Estimation

The Link Estimator shall eventually consume observations from ordinary traffic.

Every successfully received radio packet provides modulation-specific PHY
information through the driver: RSSI for LoRa and GFSK, LoRa SNR for LoRa,
and GFSK packet status for GFSK.

Passive observations may include:

```text
profile used
RSSI
LoRa SNR or GFSK packet status, as applicable
successful packet count
CRC/header failures where attributable
ACK success/failure
retry count
time since observation
```

A future `LinkState` may maintain rolling or exponentially weighted statistics for each useful peer/profile pair.

Phase I need only define the interface and optionally record basic observations.

---

# Active Link Probing

Active probing is a future extension.

A node may initiate a probe exchange with a peer and sweep candidate:

```text
bands
frequencies
spreading factors
bandwidths
TX powers
other modulation parameters
```

The result updates the Link Estimator.

Active probing must be scheduled traffic.

It must not bypass:

- radio arbitration;
- regional RF policy;
- power constraints;
- higher-priority communication.

The active probe protocol should be specified in its own later design document.

---

# Link Cost

The eventual notion of a "best" channel should not simply minimise transmitter power.

The useful quantity is closer to:

```text
expected energy per successfully delivered useful bit
```

which may include:

```text
transmit energy
receiver listening energy
wake-up/setup energy
expected retries
relay energy
airtime opportunity cost
```

Different intents may legitimately prefer different profiles.

For example:

- a very short urgent message may prefer low airtime;
- routine telemetry may prefer minimum expected energy;
- broadcast may prefer robust network-wide reception;
- collaborative acoustic traffic may prefer a dedicated high-rate local channel.

The Link Estimator exposes estimates.

The higher communication policy decides which trade-off is appropriate.

---

# Scheduling Model

The scheduler operates on logical network slots rather than assuming one slot equals one packet airtime.

A slot may contain:

```text
radio wake / preparation
clock guard
RX/TX transition guard
packet airtime
ACK turnaround
ACK airtime
cleanup
final guard
```

The initial system should deliberately use generous slot sizes.

The network has substantial time and frequency resources, so correctness and predictable power behaviour are more important initially than highly packed airtime.

---

# Traffic Classes and Logical Slot Spaces

Different message classes may use different logical schedules.

For example:

```text
common broadcast / control slots
heartbeat slots
tree child-uplink slots
tree parent-forwarding slots
bulk-data slots
collaborative acoustic slots
probe slots
```

These are logical network resources.

The central radio scheduler resolves them onto the single physical LR1121.

This permits multiple networking mechanisms to coexist without requiring them to share one uniform MAC behaviour.

---

# Phase II Tree Scheduling

The reliable tree protocol is outside the detailed scope of Phase I, but the Phase I architecture shall support it.

A parent may assign child activity using a compact schedule.

One useful representation is a slot bitmap:

```text
slots:  0 1 2 3 4 5 6 7 8 ...
child:  0 0 1 0 0 1 0 0 0 ...
```

A schedule grant may conceptually include:

```rust
pub struct ScheduleGrant {
    pub generation: ScheduleGeneration,
    pub valid_from: UtcTimestamp,
    pub valid_for_epochs: u16,
    pub slot_mask: SlotMask,
    pub channel_plan: ChannelPlanId,
}
```

The binary mask can remain very compact while allowing a parent to vary how much capacity a child receives.

For example:

```text
quiet node       -> one occasional slot
normal node      -> several slots
backlogged node  -> burst allocation
```

The common broadcast mechanism continues independently of these grants.

A node which loses its schedule therefore still has a bootstrap path back into the network.

---

# Wave Scheduling

A tree should exploit UTC synchronisation to form a forwarding wave.

Conceptually:

```text
leaf level TX
      |
      v
intermediate level RX

intermediate level TX
      |
      v
upper level RX

upper level TX
      |
      v
gateway RX
```

This allows intermediate nodes to:

- wake for their child windows;
- receive messages;
- aggregate or queue them;
- transmit toward their parent in a later scheduled window;
- return to sleep.

The goal is timely delivery rather than minimum instantaneous latency.

A message may therefore traverse multiple hops over seconds or minutes while nodes remain asleep for most of the interval.

---

# Channel Hopping

Time and frequency may both participate in scheduling.

A logical slot does not need to imply one permanent RF channel.

Later network schedules may define a deterministic channel selection such as:

```text
profile =
    LinkEstimator.select_profile(
        peer,
        slot,
        constraints
    )
```

or may reference a shared `ChannelPlanId`.

A static implementation may simply return one profile for every slot.

A later implementation may use:

```text
hash(node pair, epoch, slot, channel plan)
```

to choose from an approved profile set.

Both ends must be able to derive the same result deterministically or receive an explicit control-plane update describing the new plan.

Dynamic link estimation must never cause two peers to independently guess incompatible channels.

---

# Burst Data Channels

Longer transfers do not need to occupy the common broadcast channel.

A scheduled control exchange may allocate:

```text
a future UTC window
+
a dedicated ChannelProfile / ChannelPlan
+
a maximum duration or slot count
```

for a longer data burst.

This enables the broadcast channel to remain available for discovery/control while larger transfers move elsewhere.

---

# Hop-by-Hop Reliability

Reliable transport shall initially be hop-by-hop only.

Conceptually:

```text
child -> parent : DATA(sequence)
parent -> child : ACK(sequence)
```

If an acknowledgement is not received, the child may retry according to the link protocol.

Intermediate nodes take ownership of forwarding once they have acknowledged successful receipt.

End-to-end retransmission from the original source to the gateway is not required.

This avoids keeping an entire path active during retries.

End-to-end identifiers may still be retained for deduplication and application identity.

For example:

```text
origin NodeId
origin BootId
message sequence
```

A gateway can therefore identify duplicates even if packets have been independently retransmitted by several hops.

---

# Bounded Reliability

Retries shall always be bounded.

A reliability policy may specify:

```text
maximum attempts
retry opportunities
message expiry
deadline
```

No networking operation may retry forever.

If delivery cannot be completed within the allowed resource/deadline budget, the failure is reported to the appropriate higher layer.

---

# Geographic Routing

Full geographic routing is deferred to a later phase.

The architecture shall nevertheless preserve the required inputs:

```text
own stable location
neighbour locations
neighbour location uncertainty
link quality
message destination
```

Simple greedy geographic forwarding alone is not considered sufficient because it may become trapped at geographic voids.

A later geographic-routing design should therefore include an explicit recovery mechanism such as the family of approaches used by GPSR or another equivalent algorithm.

The Phase I design does not select that algorithm.

---

# Collaborative Node Groups

Later application protocols may create temporary geographic or semantic sub-clusters.

For example, an acoustic detector may identify neighbouring nodes useful for correlated time-of-arrival observations.

Conceptually:

```text
Node A detects candidate event
        |
        v
identify useful neighbours
        |
        v
form temporary collaboration group
        |
        v
exchange compact observations
        |
        v
perform local fusion
        |
        v
send higher-value aggregate result
```

Such traffic may use:

- a dedicated message class;
- a dedicated slot allocation;
- a separate ChannelProfile;
- a temporary `GroupId`.

The core Radio Messaging Service should support such scheduling without understanding the acoustic algorithm itself.

Acoustic timestamping remains entirely outside the radio layer.

Radio timestamps are used for radio scheduling and packet observations, not as substitutes for sensor-event timestamps.

---

# Communication Policy

The system should separate:

```text
what communication is desirable
```

from:

```text
how a permitted transmission is executed
```

A policy interface should therefore be introduced before learned policies are required.

Conceptually:

```rust
pub trait CommunicationPolicy {
    fn decide(
        &mut self,
        context: &CommunicationContext,
        message: &PendingMessage,
    ) -> CommunicationDecision;
}
```

Possible implementations may evolve through:

```text
FixedPolicy
HeuristicPolicy
CostPolicy
Future learned policy
```

A decision may choose:

```text
send now
delay
drop because expired
send toward gateway
send to selected neighbours
send to collaboration group
request particular link characteristics
```

The policy shall not directly command the LR1121.

---

# Policy Constraints

All policy decisions pass through deterministic constraints.

A policy may request:

```text
send message M to neighbour B
with a fast reliable profile
before time T
```

but the lower layers remain authoritative over:

```text
RF legality
TX power limits
approved frequencies
radio availability
allocated TDMA slots
queue bounds
retry bounds
battery protection limits
protocol invariants
```

A future neural policy therefore scores or proposes communication actions rather than replacing the MAC, scheduler, or regulatory checks.

---

# Protocol Evolution Phases

The implementation shall proceed in numbered phases.

The phase number forms part of the design policy and roadmap.

Later phases should receive separate design documents before substantial implementation.

---

## Phase I — Radio Service, Heartbeat, and Discovery

Phase I is the normative implementation scope of this document.

Implement:

- single-owner `RadioService`;
- bounded radio request arbitration;
- compact common frame envelope;
- node addressing integration;
- per-boot `BootId`;
- volatile frame/message sequence numbers;
- UTC epoch/rendezvous utilities;
- deterministic heartbeat scheduling;
- common broadcast profile;
- heartbeat protocol;
- presence advertisement protocol;
- bounded neighbour table;
- peer activity prediction;
- static `LinkEstimator`;
- bounded, complete sub-GHz and 2.4 GHz LoRa/GFSK profile representation and
  explicit rejection of any profile the driver cannot apply;
- passive RSSI/SNR observation hooks;
- service statistics and diagnostics.

Phase I does **not** require:

- ACKs;
- parent/child relationships;
- dynamic slot grants;
- active channel probing;
- adaptive PHY selection;
- mesh routing.

---

## Phase II — Reliable Scheduled Tree

Add:

- parent/child topology;
- explicit tree-control protocol;
- TDMA schedule grants;
- binary slot allocation;
- forwarding-wave scheduling;
- hop-by-hop DATA/ACK;
- bounded retries;
- store-and-forward message queues;
- additional burst/data channels;
- schedule generation and expiry;
- topology repair.

Tree formation and reliable transport should remain separable modules.

The first implementation may use a manually configured tree before automatic tree formation is added.

---

## Phase III — Adaptive Links and Multiband Operation

Add:

- richer passive link statistics;
- per-peer/per-profile link state;
- active probe exchanges;
- adaptive exploration of the already representable sub-GHz and 2.4 GHz
  LoRa/GFSK profiles;
- deterministic channel hopping;
- profile ranking;
- expected-delivery and energy-cost estimates;
- negotiated/shared link plans.

The service API should remain unchanged for ordinary message users.

---

## Phase IV — Mesh, Geographic Routing, and Collaboration

Add:

- mesh route discovery/maintenance;
- geographically informed forwarding;
- geographic void recovery;
- scoped multicast/broadcast;
- temporary collaboration groups;
- geographically selected acoustic "friends";
- group-specific communication windows;
- group-specific profile/channel selection.

The exact routing algorithms require a dedicated design document.

---

## Phase V — Adaptive Communication Policy

Add or extend:

- application utility/value metadata;
- richer energy models;
- congestion/backlog information;
- contextual communication decisions;
- swappable heuristic/cost-based policies;
- optional learned policy implementation.

A learned policy remains subordinate to deterministic transport and safety constraints.

---

# Phase I Module Structure

The implementation should live under a service directory such as:

```text
crates/services/radio/
```

or the repository's equivalent naming convention.

A suggested layout is:

```text
radio/
    mod.rs

    service.rs
    config.rs
    error.rs
    stats.rs

    frame.rs
    address.rs
    message.rs

    epoch.rs
    rendezvous.rs
    scheduler.rs

    heartbeat.rs
    presence.rs
    neighbour.rs

    link/
        mod.rs
        profile.rs
        estimator.rs
        observation.rs
```

The intent of this structure is functional coherence, not arbitrary file splitting.

---

## `mod.rs`

Public service surface.

Should:

- export stable public types;
- hide internal scheduler and codec implementation details;
- avoid exposing LR1121-specific types unnecessarily.

---

## `service.rs`

Owns the LR1121 driver.

Responsibilities:

- main Embassy service task;
- request intake;
- central radio arbitration;
- invoking timed driver operations;
- returning request results;
- publishing service state.

It should not contain heartbeat encoding or neighbour-selection logic.

---

## `config.rs`

Network/service configuration.

Examples:

```text
network identifier
epoch duration
broadcast-window duration
heartbeat cadence
presence cadence
repetition count
guard margins
queue capacities
neighbour-table capacity
```

Configuration should distinguish compile-time structural capacities from runtime policy values.

---

## `frame.rs`

Owns:

- wire frame header;
- frame types;
- encoding;
- decoding;
- protocol-version validation;
- bounds checking.

It must be independently host-testable.

---

## `address.rs`

Owns network-facing node and group address representations.

It should adapt the system identity service into compact network identity without making the radio layer responsible for manufacturing traceability.

---

## `message.rs`

Defines semantic messaging primitives:

```text
Message
MessageOptions
MessageClass
Destination
Reliability
Priority
```

Later protocol modules may extend message types without changing frame parsing.

---

## `epoch.rs`

Owns conversion between UTC and:

```text
network epoch
epoch offset
logical slot index
```

It consumes the Time Service.

It does not estimate UTC.

---

## `rendezvous.rs`

Owns the deterministic hash/scheduling function.

It should contain no radio-driver code.

Host tests should verify bit-for-bit stable slot selection across test vectors.

Changing this algorithm requires changing its protocol/schedule version.

---

## `scheduler.rs`

Owns:

- physical radio reservations;
- request ordering;
- conflict detection;
- preparation time;
- guard intervals;
- priority handling.

It resolves logical protocol activity onto the single radio.

This is distinct from `rendezvous.rs`, which only computes desired network times.

---

## `heartbeat.rs`

Owns:

- heartbeat payload representation;
- heartbeat cadence;
- deterministic transmit opportunity selection;
- status collection;
- submission to the radio scheduler.

It should not decode arbitrary networking frames.

---

## `presence.rs`

Owns:

- compact presence advertisements;
- advertisement cadence;
- bootstrap capability/schedule information;
- presence reception handling.

---

## `neighbour.rs`

Owns:

- bounded neighbour table;
- ageing;
- replacement;
- neighbour metadata;
- peer schedule/rendezvous prediction;
- passive receipt observations.

It should not select RF profiles itself.

---

## `link/profile.rs`

Defines complete link/PHY profiles used by the scheduler and driver adapter.

The type should be sufficient to resolve to an LR1121 `ChannelConfig` and transmission policy without exposing LR1121 command details to upper protocols.

---

## `link/estimator.rs`

Defines the profile-selection API.

Phase I implementation may be static.

For example:

```rust
pub struct StaticLinkEstimator {
    pub broadcast: ChannelProfile,
    pub default_data: ChannelProfile,
}
```

The API should nevertheless already use semantic constraints so the static implementation can later be replaced.

---

## `link/observation.rs`

Defines passive link measurements such as:

```text
peer
profile
RSSI
SNR
success/failure
timestamp
```

Phase I may only retain a small amount of information.

The type should be designed so later estimators can consume richer statistics without requiring the radio service to reinterpret old frames.

---

# Future Module Expansion

Modules should be added when their corresponding phase is implemented.

For example:

```text
tree/
    control.rs
    schedule.rs
    reliability.rs
    forwarding.rs

link/
    probe.rs
    cost.rs

mesh/
    routing.rs
    geographic.rs
    recovery.rs

policy/
    mod.rs
    fixed.rs
    heuristic.rs
```

Empty speculative module trees should not be created merely to mirror this document.

The architecture should guide modularity, but source structure should remain proportional to implemented functionality.

---

# Public API Direction

The final API should favour application-facing message submission and protocol-specific handles rather than direct physical radio requests.

Conceptually:

```rust
pub struct MessagingHandle {
    // bounded service-channel handles
}

impl MessagingHandle {
    pub async fn send(
        &self,
        message: Message,
        options: MessageOptions,
    ) -> Result<MessageId, SendError>;

    pub fn neighbour_state(&self) -> NeighbourSnapshot;

    pub fn stats(&self) -> RadioServiceStats;
}
```

Internal networking subservices require a lower-level scheduler handle:

```rust
pub struct RadioSchedulerHandle {
    // crate-private or tightly scoped
}
```

which may expose:

```rust
submit_tx(...)
reserve_rx(...)
```

This lower-level handle should not normally be exposed to application code.

---

# State Publication

State that represents the latest known condition should use watches.

Examples:

```text
RadioServiceState
NeighbourSummary
LinkEstimatorState
```

Events that must be processed individually should use bounded channels.

Examples:

```text
received application message
radio scheduling request
delivery completion
probe result
```

A watch must not be used where losing intermediate events would violate protocol semantics.

---

# Queueing

All queues shall be bounded.

The service must define capacities for:

```text
pending semantic messages
radio jobs
received frames
neighbours
link observations
future reliable messages
```

Queue exhaustion shall return explicit errors or invoke documented traffic-class-specific drop policies.

There shall be no hidden dynamic allocation.

High-priority control traffic should not be permanently starved by bulk application data.

---

# Priorities

A small priority model is preferable to an unrestricted numeric priority space.

For example:

```text
CriticalControl
Control
ReliableData
BestEffort
```

The exact names may change.

Priority affects arbitration when requests conflict.

It does not allow a caller to violate an allocated TDMA schedule or radio-policy restriction.

---

# Error Handling

The service shall distinguish at least:

```text
UTC unavailable
UTC uncertainty too high
invalid schedule
missed slot
radio scheduler conflict
queue full
frame too large
malformed frame
unsupported protocol version
unknown frame type
invalid destination
no acceptable link profile
radio driver error
message expired
retry limit exceeded
neighbour unknown
```

Errors should be recoverable wherever practical.

Malformed or unknown frames shall never panic the receiver.

A radio-driver error should normally return the hardware to the driver's defined recovery path rather than permanently terminating the Radio Service.

---

# Observability

The Radio Messaging Service should publish a bounded statistics snapshot.

Suggested fields include:

```rust
pub struct RadioServiceStats {
    pub frames_tx: u32,
    pub frames_rx: u32,

    pub heartbeat_tx: u32,
    pub presence_tx: u32,
    pub presence_rx: u32,

    pub malformed_frames: u32,
    pub unsupported_frames: u32,

    pub schedule_misses: u32,
    pub scheduler_conflicts: u32,
    pub queue_drops: u32,

    pub neighbour_count: u16,

    pub radio_errors: u32,
}
```

Later phases may extend this with:

```text
ACK success/failure
retry counts
forwarded messages
route changes
probe statistics
profile selections
energy estimates
```

Counters shall have documented saturation or wrapping behaviour.

---

# Energy Accounting

Precise energy optimisation is not required in Phase I.

However, the architecture should make future energy accounting possible.

Useful quantities include:

```text
TX airtime by profile
RX airtime
radio wake duration
number of retries
number of active slots
number of probing operations
```

The scheduler is a natural place to measure radio-active durations because all access passes through it.

These measurements can later feed the Link Estimator and communication policy.

---

# Time-Uncertainty Behaviour

Network protocols should explicitly define acceptable time uncertainty.

For example:

```text
heartbeat transmission:
    may tolerate relatively large uncertainty

tight TDMA child slot:
    requires uncertainty below threshold
```

If uncertainty becomes too high:

- the scheduler should widen receive guards where allowed;
- tightly scheduled operations may be refused;
- the node may fall back to the common broadcast mechanism;
- higher layers can request reacquisition of a good time source through existing system policy.

The Radio Messaging Service itself does not control GPS.

---

# Location-Uncertainty Behaviour

Geographic features shall similarly consider location validity and uncertainty.

If the Location Service reports invalid or excessively uncertain location:

- heartbeat may omit location;
- neighbour tables may retain older location with age information;
- geographic routing must not pretend precise coordinates are available;
- non-geographic tree/broadcast operation should continue.

---

# Protocol Compatibility

Each node shall expose a protocol/schedule version sufficient to identify incompatible rendezvous behaviour.

The protocol should distinguish between:

```text
wire-frame version
rendezvous/schedule version
optional capability flags
```

These concepts should not automatically be one large version number.

A framing revision does not necessarily imply that the hash scheduling function changed.

---

# Regulatory Policy

Regional frequency, power, and duty-cycle rules belong above the hardware driver.

The LR1121 driver validates hardware-safe configurations but deliberately does not embed regional regulatory policy.

The Link Estimator must therefore select only from profiles approved by the product's regional radio policy.

Active probing must also be restricted to approved profiles.

The exact fitted Ebyte E80 module's permitted/certified operating bands must be confirmed independently of the LR1121 silicon's theoretical capabilities.

---

# Testing

## Host Tests

Phase I host tests should cover:

### Frame Codec

- round-trip encoding/decoding;
- minimum and maximum payload sizes;
- optional fields;
- malformed frames;
- unknown versions;
- unknown frame types;
- truncated frames;
- exact expected encoded sizes.

### Boot and Sequence Identity

- new `BootId` resets sequence state;
- duplicate keys include boot identity;
- sequence wrap has documented behaviour.

### Epoch Logic

- UTC-to-epoch conversion;
- epoch boundaries;
- slot boundaries;
- invalid UTC;
- wrap-safe calculations.

### Rendezvous Function

Use fixed test vectors:

```text
NodeId
epoch
purpose
occurrence
expected slot
```

The results become part of the protocol compatibility contract.

### Neighbour Table

- insertion;
- update;
- boot-ID change;
- expiry;
- capacity replacement;
- metadata refresh;
- predicted rendezvous calculation.

### Static Link Estimator

- broadcast requests select the bootstrap profile;
- supported data requests select the expected profile;
- impossible constraints fail cleanly.
- complete 2.4 GHz LoRa BW203/BW406/BW812 and GFSK profiles resolve without
  losing their modulation parameters; unsupported or unresolved profiles fail
  explicitly.

### Scheduler

- non-overlapping reservations;
- conflicts;
- priority behaviour;
- missed preparation deadline;
- uncertainty guard calculation;
- bounded queue behaviour.

---

## Hardware-in-the-Loop Tests

Create a service test such as:

```text
servicetests/radio
```

Initial tests should use at least two nodes and preferably a gateway listener.

Test:

1. initialise the Time Service;
2. wait for valid UTC;
3. start the Radio Messaging Service;
4. transmit deterministic heartbeat frames;
5. verify that independent nodes select the expected UTC windows;
6. verify gateway reception;
7. verify presence discovery;
8. verify neighbour-table updates;
9. verify RSSI/SNR observations;
10. verify nodes can predict a neighbour's future presence opportunity;
11. verify sleep/wake behaviour outside required windows;
12. deliberately degrade or remove GPS and observe behaviour as UTC uncertainty increases.

Also run scheduled TX/RX windows for an agreed 2.4 GHz LoRa profile and an
agreed 2.4 GHz GFSK profile, switching back to the bootstrap profile between
them. Record the actual RX filter, packet format, retune guard, TX/RX outcome,
and modulation-specific receive metrics. Characterize the 250 kb/s FSK case
at the actual 467 kHz filter before selecting it in deployment policy.

A useful diagnostic mode should print:

```text
current UTC
epoch
current slot
next heartbeat slot
next presence slot
selected profile
radio state
last RX RSSI/SNR
neighbour count
```

This is primarily a test facility and should not require verbose logging in production firmware.

---

# Phase I Acceptance Criteria

Phase I is complete when:

- one Radio Service exclusively owns the LR1121 driver;
- no higher-level service accesses the driver directly;
- two or more UTC-synchronised nodes independently calculate compatible broadcast windows;
- heartbeat messages are transmitted using deterministic pseudo-random slot selection;
- a gateway can receive and decode heartbeat status;
- nodes can receive presence advertisements from peers;
- peers appear in a bounded soft-state neighbour table;
- future peer rendezvous opportunities can be predicted;
- the common frame envelope is compact and measured;
- the Link Estimator selects the Phase I static profile;
- the service can resolve and schedule agreed 2.4 GHz LoRa and GFSK profiles,
  and rejects unresolved profiles explicitly;
- received packets update passive link observations;
- time uncertainty is consumed from the Time Service;
- location is consumed from the Location Service rather than GPS directly;
- all service queues and tables are statically bounded;
- malformed traffic and radio errors do not panic the service.

---

# Design Principles for Later Phases

The following principles are expected to remain stable beyond Phase I.

## One Radio Owner

There shall continue to be one arbiter of the physical radio.

## Common Bootstrap Plane

Presence/control broadcast remains available independently of more advanced schedules.

## UTC-Based Rendezvous

Where nodes need to meet, deterministic UTC-derived rendezvous is preferred over long idle listening.

## Hop-by-Hop Reliability

Link failures are handled locally where practical.

## Soft State

Neighbour and route knowledge should naturally age rather than require perfect global membership state.

## Separate Link Estimation from Routing

The Link Estimator answers:

> How can I communicate with this peer or group?

Routing answers:

> Which peer should receive this message next?

These remain separate concerns.

## Separate Policy from Mechanism

Communication policy decides:

> Is communication worthwhile, and with whom?

The networking mechanisms decide:

> How can that requested communication be performed safely and efficiently?

## Message Semantics Above Packet Mechanics

Application decisions operate on messages rather than radio packet structure.

## Deterministic Safety Boundaries

Adaptive or learned components may rank or request actions but may not bypass:

```text
radio scheduling
regulatory rules
TDMA ownership
queue limits
retry limits
energy safeguards
protocol correctness
```

---

# Consequences

## Advantages

- Preserves the clean LR1121 hardware abstraction.
- Provides one authoritative radio scheduler.
- Allows heartbeat, tree, mesh, and probing protocols to remain modular.
- Exploits accurate UTC to minimise idle listening.
- Supports extremely simple Phase I operation without preventing later sophistication.
- Makes deterministic peer rendezvous possible.
- Separates semantic messages from packets and PHY behaviour.
- Allows static link selection to evolve into adaptive multiband estimation.
- Provides a clean path from fixed communication rules to learned policies.
- Makes timing, location, neighbour, and link uncertainty explicit.
- Retains heapless and bounded-memory operation.
- Supports separate logical communication planes over one physical radio.

## Disadvantages

- The Radio Messaging Service becomes a central coordination point and must remain carefully modularised.
- Scheduling multiple logical protocols over one transceiver adds state-machine complexity.
- Protocol versioning and deterministic rendezvous functions become compatibility contracts.
- Bounded queues require explicit overload behaviour.
- Adaptive link selection will eventually require peer state synchronisation.
- Geographic mesh routing and learned policy introduce substantial later-phase complexity.

These costs are preferable to allowing networking protocols to independently manipulate one physical radio.

---

# Open Questions for Phase I Implementation

The following should be resolved during implementation rather than expanded into later-phase protocol design:

1. Exact compact on-air `NodeId` width.
2. Exact `BootId` width and entropy source.
3. Sequence-number width for Phase I frame classes.
4. Exact bit allocation of the compact frame header.
5. Epoch and broadcast-window default durations.
6. Heartbeat and presence default cadence.
7. Number of repeated heartbeat transmissions.
8. Initial neighbour-table capacity.
9. Exact deterministic rendezvous hash.
10. Schedule/rendezvous version representation.
11. Initial common broadcast `ChannelProfile`.
12. Whether heartbeat and presence initially use the same profile.
13. Time-uncertainty threshold above which normal scheduled operation is suspended.
14. Appropriate additional radio-start guard based on measured LR1121 scheduling performance.
15. Exact regional profile set permitted for the fitted Ebyte E80 variant.
16. Whether a short boot/session tag belongs in every frame or only in protocol state that requires reboot discrimination.

These choices should be measured and documented, but they should not materially alter the service architecture described above.

---

# Implementation Policy

Implementation should follow the numbered phases.

Phase I should be completed, tested, and characterised before Phase II networking complexity is introduced.

Each subsequent phase should receive its own focused design document describing the new protocol algorithms, states, packet types, timing rules, and failure recovery.

The common Radio Messaging Service, frame primitives, rendezvous utilities, neighbour representation, Link Estimator interface, and message/policy boundaries should remain stable unless implementation experience demonstrates a concrete deficiency.

The objective is not to construct a generic networking framework in advance.

The objective is to establish a small set of durable primitives around which increasingly capable radio protocols can be built coherently.
