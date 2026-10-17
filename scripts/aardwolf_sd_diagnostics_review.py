"""Read-only review of Aardwolf's per-operation SD diagnostics and paired files."""

import argparse
import collections
import csv
import datetime as dt
import hashlib
import json
from pathlib import Path
import re
import statistics
from zoneinfo import ZoneInfo

from overnight_files import flac, sidecar


def fields(message):
    return dict(re.findall(r"\b(\w+)=([^\s]+)", message))


def numeric_fields(message):
    return {k: int(v) if re.fullmatch(r"-?\d+", v) else v for k, v in fields(message).items()}


def save_json(path, value):
    path.write_text(json.dumps(value, indent=2) + "\n", encoding="utf-8")


def save_csv(path, rows):
    if not rows:
        return
    columns = list(dict.fromkeys(key for row in rows for key in row))
    with path.open("w", newline="", encoding="utf-8") as stream:
        writer = csv.DictWriter(stream, fieldnames=columns)
        writer.writeheader()
        writer.writerows(rows)


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--card", type=Path, default=Path("E:/"))
    parser.add_argument("--output", type=Path, required=True)
    args = parser.parse_args()
    out = args.output
    out.mkdir(parents=True, exist_ok=True)
    raw = (args.card / "syslog.txt").read_bytes()
    (out / "syslog.txt").write_bytes(raw)
    text = raw.decode("utf-8", errors="replace")
    # Abrupt removal/reboots can concatenate the last partial record and a new
    # boot record. Parse record prefixes rather than assuming one per line.
    matches = list(re.finditer(r"(\d{10}) ([\d.]+) (TRACE|DEBUG|INFO|WARN|ERROR)\s+(\w+)\s+", text))
    boots = []
    for index, match in enumerate(matches):
        seq, seconds, level, source = match.groups()
        message = text[match.end():matches[index + 1].start() if index + 1 < len(matches) else len(text)].strip()
        if message.startswith("boot node="):
            boots.append({"boot": int(fields(message)["boot"]), "rows": []})
        if boots:
            boots[-1]["rows"].append(dict(sequence=int(seq), seconds=float(seconds), level=level, source=source, message=message))
    run = next(b for b in reversed(boots) if any(r["message"].startswith("audio started utc_us=") for r in b["rows"]))
    rows = run["rows"]
    anchor = next(r for r in rows if r["message"].startswith("audio started utc_us="))
    av = numeric_fields(anchor["message"])
    offset = (av["utc_us"] - av["ticks"]) / 1e6

    def local_time(ticks):
        return dt.datetime.fromtimestamp(offset + ticks / 1e6, dt.timezone.utc).astimezone(ZoneInfo("Europe/London")).isoformat(timespec="milliseconds")

    for row in rows:
        row["uk_time"] = local_time(row["seconds"] * 1e6)
    save_json(out / "boots.json", boots)
    operations = {}
    io_rows = []
    for row in rows:
        message = row["message"]
        if message.startswith("storage sd id="):
            op = numeric_fields(message)
            op.update(log_seconds=row["seconds"], log_sequence=row["sequence"], io=[])
            op["uk_time"] = local_time(op["start_ticks"])
            if op["id"] in operations:
                raise ValueError(f"Duplicate SD operation ID {op['id']}")
            operations[op["id"]] = op
        elif message.startswith("storage sd_io id="):
            io = numeric_fields(message)
            operations[io["id"]]["io"].append(io)
            io_rows.append(io)
    ops = sorted(operations.values(), key=lambda op: op["start_ticks"])
    accounting_errors = []
    for op in ops:
        seen = set()
        for io in op["io"]:
            missing = set("id phase rw calls sectors single err us poll_us max_us lba blocks max_poll_us seq".split()) - io.keys()
            if missing:
                accounting_errors.append(f"Operation {op['id']}: missing I/O fields {sorted(missing)}")
            key = (io["phase"], io["rw"])
            if key in seen:
                accounting_errors.append(f"Operation {op['id']}: duplicated I/O phase/direction {key}")
            seen.add(key)
            if io["poll_us"] > io["us"]:
                accounting_errors.append(f"Operation {op['id']}: poll time exceeds command wall time")
        op["sd_us"] = sum(io["us"] for io in op["io"])
        op["poll_us"] = sum(io["poll_us"] for io in op["io"])
        op["outside_sd_us"] = op["execution_us"] - op["sd_us"]
        if op["outside_sd_us"] < 0 or op["end_ticks"] < op["start_ticks"]:
            accounting_errors.append(f"Operation {op['id']}: inconsistent operation/command timing")
        op["deferred_s"] = op["log_seconds"] - op["end_ticks"] / 1e6
        for rw in ("read", "write"):
            for key in ("calls", "sectors", "single", "err"):
                op[f"{rw}_{key}"] = sum(io[key] for io in op["io"] if io["rw"] == rw)
    save_json(out / "operations.json", ops)
    save_csv(out / "operations.csv", [{k: v for k, v in op.items() if k != "io"} for op in ops])
    save_csv(out / "sd-io.csv", io_rows)

    transitions = []
    expected = ["rewrite", "close_ref", "close_crcl", "open_ref", "open_crcl"]
    for index, op in enumerate(ops):
        if op["op"] != "rewrite":
            continue
        group = ops[index:index + 5]
        if [o["op"] for o in group] != expected:
            continue
        transition = dict(uk_time=op["uk_time"], start_ticks=op["start_ticks"],
                          end_ticks=group[-1]["end_ticks"], ids=[o["id"] for o in group])
        transition["elapsed_us"] = transition["end_ticks"] - transition["start_ticks"]
        transition["sd_us"] = sum(o["sd_us"] for o in group)
        transition["poll_us"] = sum(o["poll_us"] for o in group)
        transition.update({o["op"] + "_us": o["execution_us"] for o in group})
        transitions.append(transition)
    save_csv(out / "transitions.csv", transitions)

    files, unfinished = [], []
    first_utc, last_utc = offset + rows[0]["seconds"], offset + rows[-1]["seconds"]
    for folder in sorted(args.card.iterdir()):
        if not folder.is_dir() or not folder.name.isdigit() or not first_utc - 3600 <= int(folder.name) <= last_utc:
            continue
        for path in sorted(folder.glob("*.flac")):
            ref = flac(path)
            companion = path.with_suffix(".CRCL")
            if ref["bytes"] == 0:
                unfinished.append(dict(file=str(path), ref_bytes=0, crcl_bytes=companion.stat().st_size if companion.exists() else None))
                continue
            if ref.get("boot_id") != str(run["boot"]):
                continue
            packed = sidecar(companion, ref)
            files.append(dict(file=path.name, reference=ref, crcl=packed))
    save_json(out / "files.json", files)

    stats = {}
    for name in dict.fromkeys(op["op"] for op in ops):
        selected = [op for op in ops if op["op"] == name]
        values = [op["execution_us"] for op in selected]
        stats[name] = dict(count=len(values), min_us=min(values), median_us=statistics.median(values), max_us=max(values),
                           max_id=max(selected, key=lambda op: op["execution_us"])["id"])
    metrics = {}
    prefixes = ("audio health", "audio capture", "audio timing", "summary2", "summary active=")
    for row in rows:
        if row["message"].startswith(prefixes):
            for key, value in numeric_fields(row["message"]).items():
                if isinstance(value, int):
                    metrics[key] = max(metrics.get(key, 0), abs(value))
    timing = collections.defaultdict(lambda: dict(calls=0, wait_max_us=0, execution_max_us=0, total_max_us=0))
    for row in rows:
        if row["message"].startswith("storage timing op="):
            values = numeric_fields(row["message"])
            if not all(k in values for k in ("calls", "wait_max_us", "execution_max_us", "total_max_us")):
                continue  # An incomplete final log record is not a diagnostic loss.
            target = timing[values["op"]]
            target["calls"] += values["calls"]
            for key in ("wait_max_us", "execution_max_us", "total_max_us"):
                target[key] = max(target[key], values[key])
    for name, stat in stats.items():
        if stat["count"] != timing[name]["calls"]:
            accounting_errors.append(f"{name}: detailed count {stat['count']} differs from aggregate count {timing[name]['calls']}")
    source_gaps = [int(b["reference"]["first_source_frame"]) - int(a["reference"]["first_source_frame"]) - a["crcl"]["frames"] for a, b in zip(files, files[1:])]
    summary = dict(log_sha256=hashlib.sha256(raw).hexdigest(), boots=[dict(boot=b["boot"], records=len(b["rows"]), last_uptime_s=b["rows"][-1]["seconds"]) for b in boots],
                   boot=run["boot"], start_uk=rows[0]["uk_time"], last_uk=rows[-1]["uk_time"], uptime_s=rows[-1]["seconds"],
                   audio_start_uk=anchor["uk_time"], operations=len(ops), io_records=len(io_rows),
                   ids_contiguous=[op["id"] for op in ops] == list(range(1, len(ops) + 1)),
                   accounting_errors=accounting_errors,
                   failed_operations=[op["id"] for op in ops if op["ok"] != "true"],
                   sequence_gaps=[(a["sequence"], b["sequence"]) for a, b in zip(rows, rows[1:]) if b["sequence"] != a["sequence"] + 1],
                   diagnostic_loss_records=[r for r in rows if "diagnostics_lost=" in r["message"]],
                   warnings_errors=[r for r in rows if r["level"] in ("WARN", "ERROR")], operation_stats=stats,
                   transitions=transitions, storage_window_stats=timing, health_maxima=metrics,
                   radio_modes=dict(collections.Counter(fields(r["message"])["radio"] for r in rows if r["message"].startswith("summary active="))),
                   finalized_pairs=len(files), recorded_s=sum(f["crcl"]["frames"] for f in files) / 16000,
                   unfinished=unfinished, source_gaps_frames=source_gaps,
                   file_errors=[dict(file=f["file"], errors=f["crcl"]["errors"]) for f in files if f["crcl"]["errors"]],
                   completion_statuses=dict(collections.Counter(f["crcl"].get("end_status") for f in files)))
    save_json(out / "summary.json", summary)

    import matplotlib
    matplotlib.use("Agg")
    import matplotlib.pyplot as plt
    fig, axes = plt.subplots(2, 1, figsize=(11, 7), layout="constrained")
    stages = expected
    colors = ["#0072B2", "#56B4E9", "#009E73", "#E69F00", "#CC79A7"]
    xs = list(range(len(transitions)))
    bottom = [0.] * len(xs)
    for stage, color in zip(stages, colors):
        values = [t[stage + "_us"] / 1000 for t in transitions]
        axes[0].bar(xs, values, bottom=bottom, label=stage, color=color)
        bottom = [a + b for a, b in zip(bottom, values)]
    axes[0].plot(xs, [t["elapsed_us"] / 1000 for t in transitions], "k.", label="Measured backend span")
    axes[0].axhline(127998 / 96000 * 1000, color="#D55E00", ls="--", label="Nominal PCM retention (1.333 s)")
    axes[0].set_xticks(xs, [t["uk_time"][11:16] for t in transitions], rotation=45)
    axes[0].set_ylabel("Milliseconds")
    axes[0].set_title("Reference rewrite through next CRCL open; initial folder creation excluded")
    axes[0].legend(ncol=3, fontsize=8)
    for stage, color in zip(stages, colors):
        selected = [o for o in ops if o["op"] == stage]
        axes[1].plot([o["start_ticks"] / 1e6 for o in selected], [o["execution_us"] / 1000 for o in selected], ".-", label=stage, color=color)
    axes[1].set_xlabel("System uptime (seconds)")
    axes[1].set_ylabel("Milliseconds")
    axes[1].set_title("Individual operations, including initial folder creation")
    axes[1].legend(ncol=5, fontsize=8)
    for ax in axes:
        ax.grid(axis="y", alpha=.2)
        ax.set_axisbelow(True)
    fig.savefig(out / "transitions.png", dpi=160)
    plt.close(fig)
    print(json.dumps({k: v for k, v in summary.items() if k not in ("transitions",)}, indent=2))
    print("Slowest transitions:", json.dumps(sorted(transitions, key=lambda t: t["elapsed_us"], reverse=True)[:3], indent=2))


if __name__ == "__main__":
    main()
