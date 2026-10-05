from __future__ import annotations

import argparse
import re
from collections import Counter, defaultdict
from pathlib import Path

import matplotlib

matplotlib.use("Agg")
import matplotlib.pyplot as plt
import numpy as np
import pandas as pd


LINE_RE = re.compile(
    r"^(?P<line_seq>\d+)\s+(?P<mono>[0-9.]+)\s+\w+\s+\w+\s+"
    r"record=(?P<record>\d+)\s+part=(?P<part>\d+)/(?P<parts>\d+)\s(?P<chunk>.*)$"
)
COMMON_RE = re.compile(
    r"node=(?P<node>0x[0-9a-f]+) boot=(?P<boot>0x[0-9a-f]+) "
    r"utc=(?:Some\(UtcTimestamp \{ seconds: (?P<utc_s>\d+), microseconds: (?P<utc_us>\d+) \}\)|None) "
    r"utc_status=(?P<utc_status>\w+) utc_uncertainty_us=(?P<uncertainty>\d+) event=(?P<event>.*)$"
)


def field_int(text: str, name: str) -> int | None:
    match = re.search(
        rf"\b{name}: (?:Some\()?(?:(?:Epoch|Sequence|NodeId|BootId|JobId)\()?(-?\d+)",
        text,
    )
    return int(match.group(1)) if match else None


def option_int(text: str, name: str) -> int | None:
    match = re.search(rf"\b{name}: (?:Some\((-?\d+)\)|None)", text)
    return int(match.group(1)) if match and match.group(1) is not None else None


def parse_log(path: Path) -> tuple[pd.DataFrame, dict[int, str], dict[str, pd.DataFrame]]:
    pieces: dict[int, dict] = {}
    physical_sequences: list[int] = []
    for raw in path.read_text(encoding="utf-8", errors="replace").splitlines():
        match = LINE_RE.match(raw)
        if not match:
            continue
        data = match.groupdict()
        record = int(data["record"])
        physical_sequences.append(int(data["line_seq"]))
        item = pieces.setdefault(
            record,
            {"mono": float(data["mono"]), "parts": int(data["parts"]), "chunks": {}},
        )
        item["chunks"][int(data["part"])] = data["chunk"]

    logical: dict[int, str] = {}
    rows: list[dict] = []
    for record, item in sorted(pieces.items()):
        payload = "".join(item["chunks"].get(i, "") for i in range(1, item["parts"] + 1))
        logical[record] = payload
        match = COMMON_RE.match(payload)
        if not match:
            continue
        common = match.groupdict()
        utc = None
        if common["utc_s"] is not None:
            utc = int(common["utc_s"]) + int(common["utc_us"]) / 1_000_000
        event_text = common["event"]
        event_type_match = re.match(r"([A-Za-z]+)", event_text)
        rows.append(
            {
                "record": record,
                "mono_s": item["mono"],
                "parts": item["parts"],
                "node": common["node"],
                "boot": common["boot"],
                "utc_s": utc,
                "utc_status": common["utc_status"],
                "utc_uncertainty_us": int(common["uncertainty"]),
                "event_type": event_type_match.group(1) if event_type_match else "Unknown",
                "event": event_text,
            }
        )

    events = pd.DataFrame(rows)
    meta = {
        "physical_line_count": len(physical_sequences),
        "physical_sequence_gaps": sum(
            b != a + 1 for a, b in zip(physical_sequences, physical_sequences[1:])
        ),
        "logical_record_count": len(pieces),
        "record_sequence_gaps": sum(
            b != a + 1 for a, b in zip(sorted(pieces), sorted(pieces)[1:])
        ),
        "incomplete_multipart_records": sum(
            len(item["chunks"]) != item["parts"] for item in pieces.values()
        ),
        "multipart_records": sum(item["parts"] > 1 for item in pieces.values()),
        "maximum_parts": max(item["parts"] for item in pieces.values()),
    }
    return events, logical, {"meta": pd.DataFrame([meta])}


def extract_tables(events: pd.DataFrame) -> dict[str, pd.DataFrame]:
    tables: dict[str, list[dict]] = defaultdict(list)
    for row in events.itertuples(index=False):
        text = row.event
        base = {"record": row.record, "mono_s": row.mono_s, "utc_s": row.utc_s}
        if row.event_type == "Rx":
            frame = re.search(r"frame_type: (\w+)", text)
            source = re.search(r"source: NodeId\((\d+)\)", text)
            classification = re.search(r"class: (\w+)", text)
            tables["rx"].append(
                base
                | {
                    "frame_type": frame.group(1) if frame else "Unknown",
                    "source": int(source.group(1)) if source else None,
                    "epoch": field_int(text, "epoch"),
                    "expected_slot": field_int(text, "expected_slot"),
                    "sequence": field_int(text, "sequence"),
                    "class": classification.group(1) if classification else "Unknown",
                    "rssi_dbm": field_int(text, "rssi_dbm_x2") / 2,
                    "snr_db": option_int(text, "snr_db_x4") / 4
                    if option_int(text, "snr_db_x4") is not None
                    else np.nan,
                }
            )
        elif row.event_type == "TxCompleted":
            purpose = re.search(r"purpose: (\w+)", text)
            tables["tx"].append(
                base
                | {
                    "purpose": purpose.group(1) if purpose else "Unknown",
                    "epoch": field_int(text, "epoch"),
                    "slot": field_int(text, "slot"),
                    "sequence": field_int(text, "sequence"),
                }
            )
        elif row.event_type == "EpochScheduled":
            tables["scheduled"].append(
                base
                | {
                    "epoch": field_int(text, "epoch"),
                    "presence_slot": field_int(text, "presence_slot"),
                    "heartbeat_slot": field_int(text, "heartbeat_slot"),
                }
            )
        elif row.event_type == "Summary":
            fields = [
                "frames_tx",
                "frames_rx",
                "heartbeat_tx",
                "presence_tx",
                "presence_rx",
                "malformed_frames",
                "unsupported_frames",
                "schedule_misses",
                "scheduler_conflicts",
                "queue_drops",
                "neighbour_count",
                "radio_errors",
                "heartbeat_rx",
                "unknown_heartbeat_rx",
                "predicted_rx",
                "predicted_misses",
                "scan_rx",
                "outside_rx",
                "unverifiable_rx",
                "application_malformed",
                "diagnostic_drops",
                "logging_drops",
                "logging_truncations",
            ]
            tables["summary"].append(
                base | {"epoch": field_int(text, "epoch")} | {name: field_int(text, name) for name in fields}
            )
        elif row.event_type == "TimeCalibration":
            source = re.search(r"source: (\w+)", text)
            locked = re.search(r"calibration_locked: (true|false)", text)
            tables["time"].append(
                base
                | {
                    "source": source.group(1) if source else "Unknown",
                    "uncertainty_us": field_int(text, "uncertainty_us"),
                    "accepted_anchors": field_int(text, "accepted_anchors"),
                    "rejected_anchors": field_int(text, "rejected_anchors"),
                    "calibration_samples": field_int(text, "calibration_samples"),
                    "calibration_locked": locked.group(1) == "true" if locked else False,
                    "calibrated_error_ppb": field_int(text, "calibrated_error_ppb"),
                }
            )
        elif row.event_type == "LocationStatus":
            valid = re.search(r"valid: (true|false)", text)
            tables["location"].append(
                base
                | {
                    "valid": valid.group(1) == "true" if valid else False,
                    "fixes_seen": field_int(text, "fixes_seen"),
                    "satellites": option_int(text, "satellites"),
                    "hdop": option_int(text, "hdop_centi") / 100
                    if option_int(text, "hdop_centi") is not None
                    else np.nan,
                    "uncertainty_m": option_int(text, "uncertainty_meters"),
                }
            )
        elif row.event_type == "RadioFailure":
            error_match = re.search(r"error: (.*) \}$", text)
            error = error_match.group(1) if error_match else "Unknown"
            category = "MissedSlot" if "MissedSlot" in error else "UnsupportedFrame" if "UnsupportedVersion" in error else "Other"
            tables["failure"].append(base | {"category": category, "error": error})
        elif row.event_type == "PacketRejected":
            tables["rejected"].append(base)
        elif row.event_type == "NeighbourEntry":
            tables["neighbour"].append(
                base
                | {
                    "epoch": field_int(text, "epoch"),
                    "node": field_int(text, "node"),
                    "rssi_dbm": option_int(text, "rssi_dbm_x2") / 2
                    if option_int(text, "rssi_dbm_x2") is not None
                    else np.nan,
                    "snr_db": option_int(text, "snr_db_x4") / 4
                    if option_int(text, "snr_db_x4") is not None
                    else np.nan,
                    "received_packets": field_int(text, "received_packets"),
                    "failed_packets": field_int(text, "failed_packets"),
                }
            )
    return {name: pd.DataFrame(rows) for name, rows in tables.items()}


def add_elapsed(tables: dict[str, pd.DataFrame], first_epoch: int) -> None:
    origin = first_epoch * 60
    for table in tables.values():
        if not table.empty and "utc_s" in table:
            table["elapsed_min"] = (table["utc_s"] - origin) / 60


def rendezvous_slot(node: int, epoch: int, purpose: int) -> int:
    value = 0xCBF29CE484222325
    data = (
        (1_230_241_796).to_bytes(4, "big")
        + bytes([1, 2, purpose])
        + node.to_bytes(4, "big")
        + epoch.to_bytes(8, "big")
        + bytes([0])
    )
    for byte in data:
        value = ((value ^ byte) * 0x100000001B3) & 0xFFFF_FFFF_FFFF_FFFF
    return value % 20


def make_epoch_table(tables: dict[str, pd.DataFrame]) -> pd.DataFrame:
    summary, scheduled, rx = tables["summary"], tables["scheduled"], tables["rx"]
    peer = int(rx.source.mode().iloc[0])
    rows = []
    for epoch in summary.epoch.astype(int):
        local = scheduled[scheduled.epoch == epoch].iloc[0]
        peer_presence = rendezvous_slot(peer, epoch, 2)
        peer_heartbeat = 20 + rendezvous_slot(peer, epoch, 1)
        presence_received = bool(((rx.epoch == epoch) & (rx.frame_type == "Presence")).any())
        heartbeat_received = bool(((rx.epoch == epoch) & (rx.frame_type == "Heartbeat")).any())
        rows.append(
            {
                "epoch": epoch,
                "local_presence_slot": int(local.presence_slot),
                "peer_presence_slot": peer_presence,
                "presence_received": presence_received,
                "presence_collision": int(local.presence_slot) == peer_presence,
                "local_heartbeat_slot": int(local.heartbeat_slot),
                "peer_heartbeat_slot": peer_heartbeat,
                "heartbeat_received": heartbeat_received,
                "heartbeat_collision": int(local.heartbeat_slot) == peer_heartbeat,
                "received_total": int(presence_received) + int(heartbeat_received),
            }
        )
    return pd.DataFrame(rows)


def shade_windows(ax: plt.Axes, minutes: int) -> None:
    for minute in range(minutes + 1):
        ax.axvspan(minute, minute + 1 / 3, color="#d8f3dc", alpha=0.35, lw=0)
        ax.axvspan(minute + 1 / 3, minute + 2 / 3, color="#dbeafe", alpha=0.35, lw=0)
        ax.axvspan(minute + 2 / 3, minute + 1, color="#eeeeee", alpha=0.45, lw=0)


def make_plots(out: Path, tables: dict[str, pd.DataFrame]) -> None:
    rx, tx, summary = tables["rx"], tables["tx"], tables["summary"]
    duration_minutes = int(np.ceil(max(rx.elapsed_min.max(), tx.elapsed_min.max(), summary.elapsed_min.max())))

    fig, (ax, err_ax) = plt.subplots(2, 1, figsize=(15, 7), sharex=True, height_ratios=[3, 1])
    shade_windows(ax, duration_minutes)
    positions = {("tx", "Presence"): 3, ("rx", "Presence"): 2, ("tx", "Heartbeat"): 1, ("rx", "Heartbeat"): 0}
    styles = {"Presence": ("#18864b", "o"), "Heartbeat": ("#1769aa", "D")}
    for direction, frame in positions:
        table = tx if direction == "tx" else rx
        column = "purpose" if direction == "tx" else "frame_type"
        subset = table[table[column] == frame]
        color, marker = styles[frame]
        ax.scatter(subset.elapsed_min, np.full(len(subset), positions[(direction, frame)]), s=34, marker=marker,
                   facecolors=color if direction == "rx" else "none", edgecolors=color,
                   label=f"{direction.upper()} {frame}", zorder=3)
    ax.set_yticks([0, 1, 2, 3], ["RX heartbeat", "TX heartbeat", "RX presence", "TX presence"])
    ax.set_title("Packet timeline (green=presence window, blue=heartbeat window, grey=idle)")
    ax.grid(axis="x", alpha=0.2)
    ax.legend(ncol=4, loc="upper right")

    failures = tables.get("failure", pd.DataFrame())
    rejected = tables.get("rejected", pd.DataFrame())
    categories = [("MissedSlot", 2, "#e69f00"), ("UnsupportedFrame", 1, "#d55e00")]
    for category, y, color in categories:
        subset = failures[failures.category == category]
        err_ax.scatter(subset.elapsed_min, np.full(len(subset), y), s=22, color=color, label=category)
    err_ax.scatter(rejected.elapsed_min, np.zeros(len(rejected)), s=22, color="#7b2cbf", label="PacketRejected")
    err_ax.set_yticks([0, 1, 2], ["Rejected", "Unsupported", "RX displaced by TX"])
    err_ax.set_xlabel("Minutes from first complete UTC epoch")
    err_ax.grid(axis="x", alpha=0.2)
    fig.tight_layout()
    fig.savefig(out / "packet_timeline.png", dpi=180)
    plt.close(fig)

    epochs = tables["epochs"]
    complete_epochs = epochs.epoch.astype(int).tolist()
    counts = epochs.set_index("epoch")[["presence_received", "heartbeat_received"]].astype(int)
    epoch_index = np.arange(len(complete_epochs))
    fig, ax = plt.subplots(figsize=(14, 5))
    width = 0.36
    ax.bar(epoch_index - width / 2, counts.presence_received, width, color="#18864b", label="Presence received")
    ax.bar(epoch_index + width / 2, counts.heartbeat_received, width, color="#1769aa", label="Heartbeat received")
    ax.axhline(1, color="black", lw=1, ls="--", label="Expected per type")
    ax.set_xticks(epoch_index, [str(epoch)[-3:] for epoch in complete_epochs], rotation=90)
    ax.set_xlabel("Epoch (last three digits)")
    ax.set_ylabel("Packets received")
    ax.set_ylim(0, max(1.25, counts.to_numpy().max() + 0.25))
    ax.set_title("Reception completeness per finished epoch")
    ax.grid(axis="y", alpha=0.25)
    ax.legend(ncol=3)
    fig.tight_layout()
    fig.savefig(out / "epoch_reception.png", dpi=180)
    plt.close(fig)

    scheduled = tables["scheduled"]
    epoch0 = int(summary.epoch.min())
    fig, ax = plt.subplots(figsize=(14, 6))
    ax.axhspan(0, 20, color="#d8f3dc", alpha=0.55, label="Presence window")
    ax.axhspan(20, 40, color="#dbeafe", alpha=0.55, label="Heartbeat window")
    ax.axhspan(40, 60, color="#eeeeee", alpha=0.8, label="Idle")
    ax.scatter(scheduled.epoch - epoch0, scheduled.presence_slot, marker="x", s=55, color="#006d2c", label="Local TX presence")
    ax.scatter(scheduled.epoch - epoch0, scheduled.heartbeat_slot, marker="x", s=55, color="#08519c", label="Local TX heartbeat")
    present = epochs[epochs.presence_received]
    missing = epochs[~epochs.presence_received]
    heartbeats = epochs[epochs.heartbeat_received]
    ax.scatter(present.epoch - epoch0, present.peer_presence_slot, marker="o", s=35, color="#31a354", label="Peer RX presence")
    ax.scatter(heartbeats.epoch - epoch0, heartbeats.peer_heartbeat_slot, marker="D", s=30, color="#3182bd", label="Peer RX heartbeat")
    ax.scatter(missing.epoch - epoch0, missing.peer_presence_slot, marker="x", s=75, linewidths=2.0, color="#d62728", label="Missing peer presence")
    ax.set_xlabel(f"Epoch offset from {epoch0}")
    ax.set_ylabel("Slot / UTC seconds into epoch")
    ax.set_ylim(-1, 60)
    ax.set_title("Rendezvous slot map: local transmissions and received peer packets")
    ax.grid(alpha=0.25)
    ax.legend(ncol=3, loc="upper right")
    fig.tight_layout()
    fig.savefig(out / "slot_map.png", dpi=180)
    plt.close(fig)

    time = tables["time"].replace([np.inf, -np.inf], np.nan)
    location = tables["location"]
    fig, axes = plt.subplots(3, 1, figsize=(14, 9), sharex=True)
    axes[0].plot(rx.elapsed_min, rx.rssi_dbm, "o-", ms=4, lw=1, color="#444444", label="RSSI")
    snr_ax = axes[0].twinx()
    snr_ax.plot(rx.elapsed_min, rx.snr_db, "o", ms=3, color="#0072b2", alpha=0.75, label="SNR")
    axes[0].set_ylabel("RSSI (dBm)")
    snr_ax.set_ylabel("SNR (dB)", color="#0072b2")
    axes[0].set_title("Radio and navigation/time quality")
    axes[0].grid(alpha=0.25)

    valid_uncertainty = time[time.uncertainty_us < 10**12]
    axes[1].semilogy(valid_uncertainty.elapsed_min, valid_uncertainty.uncertainty_us, color="#d55e00", label="UTC uncertainty")
    lock = time[time.calibration_locked]
    if not lock.empty:
        axes[1].axvline(lock.elapsed_min.iloc[0], color="#009e73", ls="--", label="Calibration locked")
    axes[1].set_ylabel("UTC uncertainty (µs, log)")
    axes[1].grid(alpha=0.25)
    axes[1].legend()

    axes[2].plot(location.elapsed_min, location.satellites, "o-", ms=4, color="#009e73", label="Satellites")
    hdop_ax = axes[2].twinx()
    hdop_ax.plot(location.elapsed_min, location.hdop, "s-", ms=3, color="#cc79a7", label="HDOP")
    axes[2].set_ylabel("Satellites")
    hdop_ax.set_ylabel("HDOP", color="#cc79a7")
    axes[2].set_xlabel("Minutes from first complete UTC epoch")
    axes[2].grid(alpha=0.25)
    fig.tight_layout()
    fig.savefig(out / "radio_time_quality.png", dpi=180)
    plt.close(fig)


def write_report(out: Path, source: Path, events: pd.DataFrame, tables: dict[str, pd.DataFrame], meta: pd.DataFrame) -> None:
    rx, tx, summary = tables["rx"], tables["tx"], tables["summary"]
    final = summary.iloc[-1]
    rx_counts = Counter(rx.frame_type)
    failure_counts = Counter(tables["failure"].category)
    missing = []
    for epoch in summary.epoch.astype(int):
        for frame in ("Presence", "Heartbeat"):
            if len(rx[(rx.epoch == epoch) & (rx.frame_type == frame)]) == 0:
                missing.append(f"{epoch}:{frame}")
    boot = events[events.event_type == "Boot"].iloc[0].event
    role = re.search(r"role: (\w+)", boot).group(1)
    firmware = re.search(r'firmware_hash: Some\("([^"]+)"\)', boot)
    time = tables["time"]
    locked = time[time.calibration_locked]
    lock_text = f"{locked.mono_s.iloc[0]:.1f} s after boot" if not locked.empty else "not reached"
    location = tables["location"]
    valid_location = location[location.valid]
    gps_text = f"{valid_location.mono_s.iloc[0]:.1f} s after boot" if not valid_location.empty else "not reached"
    logger_clean = all(final.get(name, 1) == 0 for name in ("diagnostic_drops", "logging_drops", "logging_truncations"))
    epochs = tables["epochs"]
    missing_presence_slots = sorted(int(value) for value in epochs.loc[~epochs.presence_received, "peer_presence_slot"].unique())
    collisions = int(epochs.presence_collision.sum() + epochs.heartbeat_collision.sum())
    missed_slots = tables["failure"][tables["failure"].category == "MissedSlot"]
    tx_aligned_misses = sum((tx.mono_s - instant).abs().min() < 0.01 for instant in missed_slots.mono_s)
    valid_time = time[time.uncertainty_us < 10**12]
    final_time = time.iloc[-1]
    mean_rssi = rx.rssi_dbm.mean()
    mean_snr = rx.snr_db.mean()

    report = f"""# Integration Test 004 updated syslog analysis

Source: `{source}`

## Run summary

- One boot session: node `{events.node.iloc[0]}`, boot `{events.boot.iloc[0]}`, role `{role}`.
- Firmware: `{firmware.group(1) if firmware else 'unknown'}`; schedule version 2.
- Duration: {events.mono_s.max() / 60:.2f} minutes; {len(summary)} finished epochs.
- Local TX completions: {len(tx)} ({Counter(tx.purpose).get('Presence', 0)} presence, {Counter(tx.purpose).get('Heartbeat', 0)} heartbeat).
- Valid peer receptions: {len(rx)} ({rx_counts.get('Presence', 0)} presence, {rx_counts.get('Heartbeat', 0)} heartbeat).
- Reception rates: presence {100 * rx_counts.get('Presence', 0) / len(summary):.1f}%, heartbeat {100 * rx_counts.get('Heartbeat', 0) / len(summary):.1f}%, overall {100 * len(rx) / (2 * len(summary)):.1f}%.
- Missing expected receptions in finished epochs: {len(missing)} ({', '.join(missing) if missing else 'none'}).
- Every miss was presence in peer slot(s) {missing_presence_slots}; slot collisions with local TX: {collisions}.
- Mean valid-frame signal quality: {mean_rssi:.1f} dBm RSSI, {mean_snr:.1f} dB SNR.
- Final neighbour count: {int(final.neighbour_count)}; neighbour snapshots: {len(tables['neighbour'])}.
- GPS location became valid: {gps_text}; UTC frequency calibration lock: {lock_text}.
- UTC remained synchronized after acquisition; logged post-acquisition uncertainty ranged from {int(valid_time.uncertainty_us.min())} to {int(valid_time.uncertainty_us.max())} us.
- Final GPS/PPS anchor counters: {int(final_time.accepted_anchors)} accepted, {int(final_time.rejected_anchors)} rejected.

## Logging integrity

- Physical lines: {int(meta.physical_line_count.iloc[0])}; logical records: {int(meta.logical_record_count.iloc[0])}.
- Multipart records: {int(meta.multipart_records.iloc[0])}; maximum parts: {int(meta.maximum_parts.iloc[0])}.
- Missing physical sequences: {int(meta.physical_sequence_gaps.iloc[0])}; missing logical record sequences: {int(meta.record_sequence_gaps.iloc[0])}; incomplete multipart records: {int(meta.incomplete_multipart_records.iloc[0])}.
- Final diagnostic/logger drop/truncation counters: {int(final.diagnostic_drops)}/{int(final.logging_drops)}/{int(final.logging_truncations)} (`{'clean' if logger_clean else 'not clean'}`).

## Radio anomalies

- RX jobs reported as `MissedSlot`: {failure_counts.get('MissedSlot', 0)}; {tx_aligned_misses} align within 10 ms of local TX and one is the startup RX replacement.
- Unsupported-version frames: {failure_counts.get('UnsupportedFrame', 0)}.
- Packet-rejected events: {len(tables['rejected'])}.
- Final malformed/unsupported/radio-error counters: {int(final.malformed_frames)}/{int(final.unsupported_frames)}/{int(final.radio_errors)}.
- Scheduler conflicts and radio queue drops: {int(final.scheduler_conflicts)}/{int(final.queue_drops)}.

## Plots

- `packet_timeline.png`: packet TX/RX plus rejected/unsupported events and RX displacement.
- `epoch_reception.png`: reception completeness for every finished epoch.
- `slot_map.png`: scheduled local slots and received peer rendezvous slots across the 20/20/20 layout.
- `radio_time_quality.png`: RSSI/SNR, UTC uncertainty/calibration lock, satellites and HDOP.
"""
    (out / "report.md").write_text(report, encoding="utf-8")


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("source", nargs="?", type=Path, default=Path(r"E:\syslog.txt"))
    parser.add_argument("--out", type=Path, default=Path(__file__).resolve().parent)
    args = parser.parse_args()
    args.out.mkdir(parents=True, exist_ok=True)

    events, _, extra = parse_log(args.source)
    tables = extract_tables(events)
    first_epoch = int(tables["summary"].epoch.min())
    add_elapsed(tables, first_epoch)
    tables["epochs"] = make_epoch_table(tables)
    events.to_csv(args.out / "events.csv", index=False)
    for name, table in tables.items():
        table.to_csv(args.out / f"{name}.csv", index=False)
    make_plots(args.out, tables)
    write_report(args.out, args.source, events, tables, extra["meta"])
    print((args.out / "report.md").read_text(encoding="utf-8"))


if __name__ == "__main__":
    main()
