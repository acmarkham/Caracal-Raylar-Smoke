"""Summarize RTT output from sd-lifecycle-stress; also usable during a run."""
import argparse
import csv
import hashlib
import json
import re
from pathlib import Path

import numpy as np
import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt


def distribution(values):
    a = np.asarray(list(values), dtype=float)
    if not len(a):
        return {}
    return dict(n=len(a), median=float(np.median(a)), p95=float(np.percentile(a, 95)),
                p99=float(np.percentile(a, 99)), max=float(np.max(a)))


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("log", type=Path)
    parser.add_argument("output", type=Path)
    args = parser.parse_args()
    raw = args.log.read_bytes()
    text = raw.decode("utf-8", errors="replace")
    start_match = re.search(r"start_utc=(\d+)", text)
    start_utc = int(start_match[1]) if start_match else 0
    pairs, operations = [], []
    for line in text.splitlines():
        if "SDPAIR " not in line and "SDOP " not in line:
            continue
        row = dict(re.findall(r"(\w+)=([^\s]+)", line.split(" (sd_lifecycle_stress")[0]))
        for key, value in row.items():
            if value in ("true", "false"):
                row[key] = value == "true"
            elif value.isdecimal():
                row[key] = int(value)
        row["target_seconds"] = float(line.split()[0])
        (pairs if "SDPAIR " in line else operations).append(row)
    args.output.mkdir(parents=True, exist_ok=True)
    for name, rows in (("pairs", pairs), ("operations", operations)):
        if rows:
            with (args.output / f"{name}.csv").open("w", newline="", encoding="utf-8") as f:
                writer = csv.DictWriter(f, fieldnames=list(rows[0]))
                writer.writeheader()
                writer.writerows(rows)
    transitions = [p for p in pairs if p["has_next"]]
    folder_transitions = [p for p in transitions if (start_utc + (p["pair"] + 1) * 60) % 3600 == 0]
    normal_transitions = [p for p in transitions if p not in folder_transitions]
    stages = sorted({o["phase"] for o in operations})
    operation_map = {(o["pair"], o["phase"]): o for o in operations}
    lifecycle_phases = {"rewrite", "close_ref", "close_extra", "next_open_ref", "next_open_extra", "next_headers"}
    gaps = []
    for pair in transitions:
        rows = [o for o in operations if o["pair"] == pair["pair"] and o["phase"] in lifecycle_phases]
        # A live capture may end partway through the deferred SDOP messages.
        if {o["phase"] for o in rows} == lifecycle_phases:
            gap = pair["transition_us"] - sum(o["us"] for o in rows)
            assert gap >= 0, "Stage timings exceed their containing interval"
            gaps.append(gap)
    summary = {
        "log_sha256": hashlib.sha256(raw).hexdigest(),
        "complete": "SD LIFECYCLE STRESS PASS" in text,
        "pairs": len(pairs), "overruns": [p for p in transitions if p["overrun"]],
        "payload_bytes": sum(p["ref_bytes"] + p["extra_bytes"] for p in pairs),
        "lifecycle_unattributed_us": distribution(gaps),
        "transition_us": distribution(p["transition_us"] for p in transitions),
        "normal_transition_us": distribution(p["transition_us"] for p in normal_transitions),
        "folder_transition_us": distribution(p["transition_us"] for p in folder_transitions),
        "folder_transitions": folder_transitions,
        "groups": {},
        "operations": {phase: distribution(o["us"] for o in operations if o["phase"] == phase) for phase in stages},
        "longest_operations": sorted(operations, key=lambda o: o["us"], reverse=True)[:20],
        "longest_lifecycle_operations": sorted([o for o in operations if o["phase"] != "payload"], key=lambda o: o["us"], reverse=True)[:20],
    }
    for name, select in (("small", lambda p: p["blocks"] == 8),
                         ("full_rewrite", lambda p: p["blocks"] == 937 and p["rewrite"]),
                         ("full_no_rewrite", lambda p: p["blocks"] == 937 and not p["rewrite"])):
        group = [p for p in transitions if select(p)]
        summary["groups"][name] = {"transition_us": distribution(p["transition_us"] for p in group),
                                   "append_max_us": distribution(p["append_max"] for p in group),
                                   "reference_finalize_us": distribution(
                                       operation_map[p["pair"], "rewrite"]["us"] + operation_map[p["pair"], "close_ref"]["us"]
                                       for p in group if (p["pair"], "close_ref") in operation_map),
                                   "paired_open_us": distribution(
                                       operation_map[p["pair"], "next_open_ref"]["us"] + operation_map[p["pair"], "next_open_extra"]["us"]
                                       for p in group if (p["pair"], "next_open_ref") in operation_map and (p["pair"], "next_open_extra") in operation_map)}
        for label, phases, field in (
                ("paired_open_read_us", ("next_open_ref", "next_open_extra"), "read_us"),
                ("paired_open_write_us", ("next_open_ref", "next_open_extra"), "write_us"),
                ("paired_open_reads", ("next_open_ref", "next_open_extra"), "reads"),
                ("reference_finalize_writes", ("rewrite", "close_ref"), "writes")):
            summary["groups"][name][label] = distribution(
                sum(operation_map[p["pair"], phase][field] for phase in phases)
                for p in group if all((p["pair"], phase) in operation_map for phase in phases))
    (args.output / "summary.json").write_text(json.dumps(summary, indent=2) + "\n", encoding="utf-8")
    if not pairs:
        print("No completed pairs yet")
        return
    fig, axes = plt.subplots(2, 2, figsize=(13, 8), constrained_layout=True)
    ax = axes[0, 0]
    ax.plot([p["pair"] for p in transitions], [p["transition_us"] / 1e3 for p in transitions], ".-", ms=3)
    ax.axhline(1333.312, color="r", linestyle="--", label="PCM retention (1.333 s)")
    ax.set(xlabel="Pair", ylabel="ms", title="Uninterrupted finish → new pair ready")
    ax.legend(fontsize=8)
    ax = axes[0, 1]
    for phase in ("rewrite", "close_ref", "close_extra", "next_open_ref", "next_open_extra"):
        rows = [o for o in operations if o["phase"] == phase]
        ax.plot([o["pair"] for o in rows], [o["us"] / 1e3 for o in rows], ".-", ms=2, label=phase)
    ax.set(xlabel="Pair", ylabel="ms", title="Individual lifecycle stages")
    ax.legend(fontsize=8)
    ax = axes[1, 0]
    rows = [o for o in operations if o["phase"] == "next_open_ref"]
    ax.plot([o["pair"] for o in rows], [o["us"] / 1e3 for o in rows], label="Total")
    ax.plot([o["pair"] for o in rows], [o["read_us"] / 1e3 for o in rows], label="SD reads")
    ax.plot([o["pair"] for o in rows], [o["write_us"] / 1e3 for o in rows], label="SD writes")
    ax.set(xlabel="Pair", ylabel="ms", title="Reference creation: where time is spent")
    ax.legend(fontsize=8)
    ax = axes[1, 1]
    ax.plot([p["pair"] for p in pairs], [p["append_max"] / 1e3 for p in pairs], label="Worst append")
    ax.plot([p["pair"] for p in pairs], [p["log_max"] / 1e3 for p in pairs], label="Worst dummy-log checkpoint")
    ax.set(xlabel="Pair", ylabel="ms", title="Payload operations")
    ax.legend(fontsize=8)
    changes = [pairs[i]["pair"] for i in range(1, len(pairs)) if pairs[i]["blocks"] != pairs[i-1]["blocks"]]
    rewrite_changes = [pairs[i]["pair"] for i in range(1, len(pairs)) if pairs[i]["rewrite"] != pairs[i-1]["rewrite"]]
    if len(rewrite_changes) < 5:
        changes += rewrite_changes
    for ax in axes.flat:
        ax.grid(alpha=.25)
        for pair in changes:
            ax.axvline(pair, color="grey", linestyle=":", linewidth=.7)
    fig.savefig(args.output / "timings.png", dpi=160)
    plt.close(fig)
    print(json.dumps({k: summary[k] for k in ("complete", "pairs", "payload_bytes", "transition_us")}, indent=2))
    print("Overruns:", len(summary["overruns"]))
    for phase in ("rewrite", "close_ref", "close_extra", "next_open_ref", "next_open_extra"):
        print(phase, summary["operations"].get(phase))


if __name__ == "__main__":
    main()
