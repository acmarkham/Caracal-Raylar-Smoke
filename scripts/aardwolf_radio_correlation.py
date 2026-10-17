"""Read preserved daytime-run evidence; correlate storage incidents with radio."""
import collections
import datetime as dt
import json
import re
from pathlib import Path
from zoneinfo import ZoneInfo

import matplotlib
matplotlib.use("Agg")
import matplotlib.dates as mdates
import matplotlib.pyplot as plt

ROOT = Path("analysis/aardwolf_overnight/20261016_day")
OUT = ROOT / "radio-correlation"
OUT.mkdir(parents=True, exist_ok=True)
boots = json.loads((ROOT / "log.json").read_text(encoding="utf-8"))
boot = next(b for b in boots if b["boot"] == 10466)
rows = boot["rows"]
files = json.loads((ROOT / "files.json").read_text(encoding="utf-8"))


def kv(message):
    return dict(re.findall(r"(\w+)=([^\s]+)", message))


summaries = [r for r in rows if r["source"] == "Radio" and r["message"].startswith("summary active=")]
plans = [r for r in rows if r["source"] == "Radio" and r["message"].startswith("minute epoch=")]
events = [r for r in rows if r["source"] == "Radio" and not r["message"].startswith("summary")]
planned_minutes = {
    int(kv(r["message"])["epoch"]) * 1800 + (int(kv(r["message"])["profile"]) - 1) * 60: r
    for r in plans
}
print("Main boot", boot["boot"], "radio summaries", len(summaries), "plans", len(plans))
print("Radio modes", dict(collections.Counter(kv(r["message"])["radio"] for r in summaries)))
print("Radio event classes", dict(collections.Counter(r["message"].split(" id=")[0].split(" epoch=")[0] for r in events)))

comparison = []
for f in files:
    t = f["crcl"]["sample_ticks"] / 1e6
    phase = (f["stamp"] // 60) % 30
    failed = f["crcl"]["status"] != 1
    before = max((r for r in summaries if r["seconds"] <= t), key=lambda r: r["seconds"], default=None)
    after = min((r for r in summaries if r["seconds"] > t), key=lambda r: r["seconds"], default=None)
    item = dict(file=f["file"], minute=dt.datetime.fromtimestamp(f["stamp"], dt.timezone.utc).strftime("%H:%M UTC"),
                phase=phase, scheduled_active=phase < 12, radio_planned=f["stamp"] in planned_minutes,
                profile=phase + 1 if phase < 12 else None,
                failed=failed, boundary_ticks=f["crcl"]["sample_ticks"], before=before, after=after)
    comparison.append(item)
    if not failed:
        continue
    print("\nFAIL", item["minute"], "phase", phase, "profile", item["profile"], "ticks", item["boundary_ticks"])
    for r in (before, after):
        if r:
            v = kv(r["message"])
            print("SUMMARY", f'{r["seconds"] - t:+.3f}s', r["bst"], {k: v[k] for k in ("radio", "tx", "rx", "misses", "errors", "time", "holdover_s")})
    nearby = [r for r in rows if abs(r["seconds"] - t) < 12 and r["source"] in ("Radio", "Gps", "Cpu")]
    item["nearby"] = nearby
    for r in nearby:
        if r["source"] == "Radio" and r["message"].startswith("summary"):
            continue
        print(f'{r["seconds"] - t:+.3f}s', r["source"], r["message"][:650])
    # Include planned transmissions whose timestamp, rather than log emission,
    # falls close to the recording boundary.
    item["tx_plans_near_boundary"] = [r for r in plans if abs(int(kv(r["message"])["slot_ticks"]) / 1e6 - t) < 5]
    for r in item["tx_plans_near_boundary"]:
        print("NEAR TX", r["message"])

print("\nFAILURES / FINALIZED FILES BY HALF-HOUR PHASE")
phase_counts = []
for phase in range(30):
    selected = [x for x in comparison if x["phase"] == phase]
    result = dict(phase=phase, total=len(selected), failed=sum(x["failed"] for x in selected))
    phase_counts.append(result)
    print(phase, result["failed"], "/", result["total"])
for active in (True, False):
    selected = [x for x in comparison if x["scheduled_active"] == active]
    print("ACTIVE" if active else "QUIET", sum(x["failed"] for x in selected), "/", len(selected))
for active in (True, False):
    selected = [x for x in comparison if x["radio_planned"] == active]
    print("ACTUAL RADIO PLAN" if active else "NO RADIO PLAN", sum(x["failed"] for x in selected), "/", len(selected))

# Retain every sampled radio state change, including reinitialization in quiet
# periods. Summaries sample at approximately 10 s; these are observation times.
changes = []
last_mode = None
for r in summaries:
    mode = kv(r["message"])["radio"]
    if mode != last_mode:
        changes.append(r)
        last_mode = mode
print("\nSTATE CHANGES", len(changes))
for r in changes[:24]:
    print(r["bst"], kv(r["message"])["radio"])
(OUT / "correlation.json").write_text(json.dumps(dict(boot=boot["boot"], files=comparison,
    phase_counts=phase_counts, radio_state_changes=changes), indent=2) + "\n", encoding="utf-8")

fig, axes = plt.subplots(2, 1, figsize=(13, 6), sharex=True, constrained_layout=True)
mode_values = {"Initializing": 0, "Idle": 1, "Receiving": 2, "Transmitting": 3, "Sleeping": 4}
for mode, y in mode_values.items():
    selected = [r for r in summaries if kv(r["message"])["radio"] == mode]
    if selected:
        axes[0].scatter([dt.datetime.fromisoformat(r["bst"]) for r in selected], [y] * len(selected), s=7, label=mode)
axes[0].set_yticks(list(mode_values.values()), list(mode_values.keys()))
axes[0].set_title("Reported radio mode (approximately 10-second snapshots)")
for field, label in (("open_max_us", "Paired open window maximum"), ("finish_max_us", "Finish window maximum")):
    selected = [(r, int(kv(r["message"])[field]) / 1e6) for r in rows
                if r["source"] == "Audio" and r["message"].startswith("audio lifecycle ") and field in kv(r["message"])]
    axes[1].scatter([dt.datetime.fromisoformat(r["bst"]) for r, value in selected if value > 0],
                    [value for r, value in selected if value > 0], s=8, label=label)
axes[1].axhline(1.3333125, color="grey", linestyle="--", label="PCM retention; finish + open must fit")
axes[1].set(ylabel="Seconds", title="Storage lifecycle diagnostics (separate window maxima, not summed)")
for ax in axes:
    for i, stamp in enumerate(sorted(planned_minutes)):
        ax.axvspan(dt.datetime.fromtimestamp(stamp, dt.timezone.utc), dt.datetime.fromtimestamp(stamp + 60, dt.timezone.utc),
                   color="green", alpha=.06, linewidth=0, label="Scheduled active minute" if i == 0 else None)
    for i, f in enumerate([f for f in files if f["crcl"]["status"] != 1]):
        ax.axvline(dt.datetime.fromtimestamp(f["stamp"], dt.timezone.utc), color="red", alpha=.65, linewidth=.8,
                   label="Failed recording boundary" if i == 0 else None)
    ax.grid(alpha=.2)
    ax.legend(loc="upper left", fontsize=7, ncol=3)
axes[1].xaxis.set_major_locator(mdates.MinuteLocator(byminute=(0, 15, 30, 45)))
axes[1].xaxis.set_major_formatter(mdates.DateFormatter("%H:%M", tz=ZoneInfo("Europe/London")))
axes[1].set_xlabel("16 October 2026, BST")
fig.savefig(OUT / "timeline.png", dpi=160)
plt.close(fig)
