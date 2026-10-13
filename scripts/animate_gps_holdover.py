"""Make a square, social-media-friendly GPS holdover animation.

Install: python -m pip install numpy matplotlib pillow
MP4 also requires ffmpeg on PATH; GIF needs no external encoder.

Examples (run from the repository root):
    python scripts/animate_gps_holdover.py E:/syslog.txt
    python scripts/animate_gps_holdover.py another.log --output demo.gif
    python scripts/animate_gps_holdover.py E:/syslog.txt --layout panels --output panels.mp4
    python scripts/animate_gps_holdover.py E:/syslog.txt --mode log
    python scripts/animate_gps_holdover.py slow.log --mode log --without-loop fixed.log
    python scripts/animate_gps_holdover.py slow.log --mode log --without-loop gps_clock.csv
    python scripts/animate_gps_holdover.py --demo --calibrated-ppb 800 --cycles 20

Default: a 20-second, 1080-square MP4, a PNG poster, cycle CSV and JSON metadata.
The default illustration assumes a constant oscillator and one slow-loop update
per GPS recovery: remaining bias *= (1 - gain). GPS removes phase error on
recovery, while the slow loop improves the next holdover. Initial and calibrated
bias magnitudes are estimated from the selected log, but convergence is MODELLED.
The raw oscillator / learned correction coefficient does not itself approach zero;
the remaining frequency error does. A displayed '~0' means below 1% of the starting
residual bias, not exact mathematical zero. --gain defaults to the firmware's 1/4.

--mode log retains measured residuals and cycle-to-cycle variation; only the path
between measurements is interpolated. With no --without-loop log, the comparison
freezes the initial frequency correction and reconstructs holdover drift from
the slow-loop log, with idealized phase recovery. It is not an independent test.
Two input logs compare cycles by ordinal, not wall-clock time or environment.
Extracted gps_clock.csv files are also supported: mono_s, accepted, locked,
last_pps_ticks, last_residual_us, and optional calibrated_ppb / tracking_updates.

Plots show magnitudes, not signed error. Firmware last_residual_us is GPS minus
predicted UTC. Effective signed residual bias = phase change / PPS gap * 1000 ppb.
1 ppb gives 1 ns/s; peak error in ms = bias_ppb * holdover_s / 1e6.
GPS recovery is expanded horizontally for readability. No unknown holdover phase
is presented as a measurement. Repeated stale statuses do not add measurements.
When a log contains several boots, --session selects one (1-based; default last).
--layout panels removes titles, legend text and captions, retaining axes, cycle
status and the three bias figures below the two graphs. Modelling assumptions
remain in the JSON sidecar rather than on the image.
"""

from __future__ import annotations

import argparse
import csv
import json
import math
import re
from dataclasses import dataclass
from pathlib import Path
from collections.abc import Iterator

import matplotlib

matplotlib.use("Agg")
import matplotlib.pyplot as plt
from matplotlib.animation import FFMpegWriter, FuncAnimation, PillowWriter, writers
from matplotlib.patches import FancyBboxPatch
import numpy as np


@dataclass
class Status:
    t: float
    accepted: int
    ticks: int | None
    residual: int | None
    correction: int | None
    locked: bool
    updates: int


@dataclass
class Cycle:
    gap_s: float
    bias_ppb: float
    correction_ppb: float | None
    recovery_t: list[float]
    recovery_ms: list[float]
    pre_ms: float


@dataclass
class Log:
    path: str
    session: int
    initial_correction: float | None
    updates: int
    cycles: list[Cycle]


def number(line: str, name: str) -> int | None:
    match = re.search(rf"\b{re.escape(name)}=(?:Some\()?(-?\d+)\b", line)
    return int(match[1]) if match else None


def status_records(path: Path) -> Iterator[Status | None]:
    """Yield statuses; None marks an explicit boot boundary."""
    with path.open(encoding="utf-8", errors="replace") as source:
        if path.suffix.lower() == ".csv":
            reader = csv.DictReader(source)
            required = {"mono_s", "accepted", "locked", "last_pps_ticks", "last_residual_us"}
            if not required.issubset(reader.fieldnames or []):
                raise ValueError(f"{path}: expected gps_clock.csv columns {', '.join(sorted(required))}")
            for row in reader:
                def optional(key: str) -> int | None:
                    return int(row[key]) if row.get(key) else None
                yield Status(float(row["mono_s"]), int(row["accepted"]),
                             optional("last_pps_ticks"), optional("last_residual_us"),
                             optional("calibrated_ppb"), row["locked"].lower() == "true",
                             optional("tracking_updates") or 0)
            return
        for line in source:
            if re.search(r"\bboot node=", line):
                yield None
            if not re.search(r"\btime status=", line):
                continue
            timestamp = re.match(r"\s*\d+\s+(\d+(?:\.\d+)?)\s+", line)
            accepted = number(line, "accepted")
            if timestamp is None or accepted is None:
                continue
            yield Status(
                float(timestamp[1]), accepted, number(line, "last_pps_ticks"),
                number(line, "last_residual_us"), number(line, "calibrated_ppb"),
                "locked=true" in line, number(line, "tracking_updates") or 0,
            )


def read_log(path: Path, session: int, min_gap: float) -> Log:
    """Stream Aardwolf time-status records, separating boots and timer resets."""
    sessions: list[list[Status]] = [[]]
    for status in status_records(path):
        if status is None:
            if sessions[-1]:
                sessions.append([])
            continue
        if sessions[-1] and (status.t < sessions[-1][-1].t or status.accepted < sessions[-1][-1].accepted):
            sessions.append([])
        sessions[-1].append(status)
    sessions = [s for s in sessions if s]
    chosen = session if session > 0 else len(sessions)
    if not 1 <= chosen <= len(sessions):
        raise ValueError(f"{path}: session {session} unavailable; found {len(sessions)} sessions")
    statuses = sessions[chosen - 1]
    lock = next((s for s in statuses if s.locked), None)
    if lock is None:
        raise ValueError(f"{path}: selected session has no frequency calibration lock")
    anchors: list[Status] = []
    last_accepted = -1
    for s in statuses:
        if s.t < lock.t:
            continue
        if s.accepted > last_accepted and s.ticks is not None and s.residual is not None:
            if not anchors or s.ticks > anchors[-1].ticks:
                anchors.append(s)
        last_accepted = s.accepted
    gaps = [i for i in range(1, len(anchors))
            if (anchors[i].ticks - anchors[i - 1].ticks) / 1e6 >= min_gap]
    cycles = []
    for n, i in enumerate(gaps):
        before, first = anchors[i - 1], anchors[i]
        end = gaps[n + 1] if n + 1 < len(gaps) else len(anchors)
        recovery = anchors[i:end]
        # Ignore partial cycles: need a later accepted PPS showing recovery.
        if len(recovery) < 2:
            continue
        gap = (first.ticks - before.ticks) / 1e6
        cycles.append(Cycle(
            gap, (first.residual - before.residual) / gap * 1000,
            before.correction,
            [(s.ticks - first.ticks) / 1e6 for s in recovery],
            [s.residual / 1000 for s in recovery], before.residual / 1000,
        ))
    if not cycles:
        raise ValueError(f"{path}: no completed post-lock PPS gaps >= {min_gap:g} seconds")
    return Log(str(path.resolve()), chosen, lock.correction,
               max(s.updates for s in statuses), cycles)


def validate_args(parser: argparse.ArgumentParser, args: argparse.Namespace) -> None:
    for key in ("duration", "fps", "size", "holdover_s", "min_gap_s", "summary_window"):
        value = getattr(args, key)
        if value is not None and (not math.isfinite(value) or value <= 0):
            parser.error(f"--{key.replace('_', '-')} must be positive and finite")
    if args.duration < 5:
        parser.error("--duration must be at least 5 seconds")
    if args.size < 400 or args.size % 2:
        parser.error("--size must be an even integer >= 400")
    if not 0 < args.gain < 1:
        parser.error("--gain must be between 0 and 1")
    if args.cycles is not None and args.cycles < 2:
        parser.error("--cycles must be at least 2")
    if args.session == 0 or args.session < -1:
        parser.error("--session must be -1 (last) or a positive session number")
    for key in ("initial_ppb", "calibrated_ppb"):
        value = getattr(args, key)
        if value is not None and (not math.isfinite(value) or value < 0):
            parser.error(f"--{key.replace('_', '-')} must be a finite magnitude >= 0")
    if args.output.suffix.lower() not in (".mp4", ".gif"):
        parser.error("--output must end in .mp4 or .gif")
    if args.mode == "log" and args.demo:
        parser.error("--demo is available only in illustration mode")
    if args.without_loop and args.mode != "log":
        parser.error("--without-loop requires --mode log")
    if args.mode == "log" and any(getattr(args, key) is not None
                                  for key in ("initial_ppb", "calibrated_ppb", "holdover_s")):
        parser.error("bias and holdover overrides are available only in illustration mode")


def build_story(args: argparse.Namespace, slow: Log | None, fixed: Log | None) -> dict:
    window = args.summary_window
    initial = args.initial_ppb
    calibrated = args.calibrated_ppb
    if initial is None:
        if slow and slow.initial_correction is None:
            raise ValueError("Log has no calibrated_ppb field; supply --initial-ppb")
        initial = abs(slow.initial_correction) if slow else 2000.0
    if calibrated is None:
        calibrated = float(np.median([abs(c.bias_ppb) for c in slow.cycles[:window]])) if slow else 800.0
    if args.mode == "illustration":
        count = args.cycles or 20
        holdover = args.holdover_s or (float(np.median([c.gap_s for c in slow.cycles])) if slow else 1800)
        slow_bias = calibrated * (1 - args.gain) ** np.arange(count)
        fixed_bias = np.full(count, calibrated)
        gaps = np.full(count, holdover)
        source_note = "Illustrative model | log-derived scale" if slow else "Illustrative model | example values"
        footer = "Constant oscillator assumed. GPS recovery expanded for clarity."
    else:
        if slow.updates == 0:
            raise ValueError("Selected slow-loop log has no tracking updates; choose a slow-loop recording")
        count = min(args.cycles or len(slow.cycles), len(slow.cycles),
                    len(fixed.cycles) if fixed else len(slow.cycles))
        if count < 2:
            raise ValueError("Need at least two completed holdover cycles for a comparison")
        slow_bias = np.array([abs(c.bias_ppb) for c in slow.cycles[:count]])
        gaps = np.array([c.gap_s for c in slow.cycles[:count]])
        if fixed:
            fixed_bias = np.array([abs(c.bias_ppb) for c in fixed.cycles[:count]])
            source_note = "Log replay | two separate recordings"
            footer = "Cycles aligned by order. Paths interpolated; recovery expanded."
        else:
            if slow.initial_correction is None or any(c.correction_ppb is None for c in slow.cycles[:count]):
                raise ValueError("Fixed-calibration replay needs calibrated_ppb fields or --without-loop")
            # GPS-minus-clock residual rate = -raw oscillator rate - correction.
            # Holding correction at C0 gives residual rate b + C - C0.
            fixed_bias = np.array([abs(c.bias_ppb + c.correction_ppb - slow.initial_correction)
                                   for c in slow.cycles[:count]])
            source_note = "Log replay | fixed-calibration comparison modelled"
            footer = "Paths interpolated. Fixed-loop recovery idealized; recovery expanded."

    # Each cycle occupies equal screen time. GPS-off is 76% of its width;
    # the much shorter recovery gets 24% so viewers can see it happen.
    points = 80
    x_parts, slow_parts, fixed_parts, bias_parts, fixed_bias_parts = [], [], [], [], []
    rows = []
    for i in range(count):
        u = np.unique(np.append(np.linspace(0, 1, points), .76))
        off = u <= .76
        progress = np.clip((u - .76) / .24, 0, 1)
        shape = np.where(off, u / .76, (1 - progress) ** 2)
        peak_slow = slow_bias[i] * gaps[i] / 1e6
        fixed_gap = fixed.cycles[i].gap_s if fixed and args.mode == "log" else gaps[i]
        peak_fixed = fixed_bias[i] * fixed_gap / 1e6
        if args.mode == "illustration":
            y_slow, y_fixed = peak_slow * shape, peak_fixed * shape
        else:
            def replay(cycle: Cycle) -> np.ndarray:
                end = cycle.recovery_t[-1]
                recover = np.interp(progress * end, cycle.recovery_t, cycle.recovery_ms)
                grow = cycle.pre_ms + (cycle.recovery_ms[0] - cycle.pre_ms) * u / .76
                return np.abs(np.where(off, grow, recover))
            y_slow = replay(slow.cycles[i])
            if fixed:
                y_fixed = replay(fixed.cycles[i])
            else:
                c = slow.cycles[i]
                signed_fixed = c.bias_ppb + c.correction_ppb - slow.initial_correction
                fixed_first = c.pre_ms + signed_fixed * gaps[i] / 1e6
                y_fixed = np.abs(np.where(off, c.pre_ms + signed_fixed * gaps[i] / 1e6 * u / .76,
                                         fixed_first * (1 - progress) ** 2))
        x_parts.append(i + u)
        slow_parts.append(y_slow)
        fixed_parts.append(y_fixed)
        bias_parts.append(np.full(len(u), slow_bias[i]))
        fixed_bias_parts.append(np.full(len(u), fixed_bias[i]))
        rows.append(dict(cycle=i + 1, slow_gap_s=float(gaps[i]), fixed_gap_s=float(fixed_gap),
                         slow_bias_magnitude_ppb=float(slow_bias[i]),
                         fixed_bias_magnitude_ppb=float(fixed_bias[i]),
                         slow_peak_error_ms=float(np.max(y_slow)),
                         fixed_peak_error_ms=float(np.max(y_fixed))))
    settled = float(slow_bias[-1]) if args.mode == "illustration" else float(np.median(slow_bias[-window:]))
    return dict(x=np.concatenate(x_parts), slow=np.concatenate(slow_parts), fixed=np.concatenate(fixed_parts),
                bias=np.concatenate(bias_parts), fixed_bias=np.concatenate(fixed_bias_parts), count=count,
                initial=initial, calibrated=calibrated, settled=settled, rows=rows,
                source_note=source_note, footer=footer)


def draw_animation(story: dict, args: argparse.Namespace) -> tuple[plt.Figure, FuncAnimation, object]:
    bg, ink, muted = "#0c1424", "#f3f6fc", "#aab8cf"
    orange, teal = "#ffb45e", "#43e0bc"
    plt.rcParams.update({"font.family": "DejaVu Sans", "font.size": 13,
                         "text.color": ink, "axes.labelcolor": muted,
                         "xtick.color": muted, "ytick.color": muted})
    fig = plt.figure(figsize=(10.8, 10.8), dpi=args.size / 10.8, facecolor=bg)
    panels_only = args.layout == "panels"
    if not panels_only:
        fig.text(.085, .949, "GPS HOLDOVER & RECOVERY", fontsize=13, color=teal, weight="bold")
        fig.text(.085, .900, "GPS fixes time.\nThe slow loop fixes drift.", fontsize=29,
                 weight="bold", va="top", linespacing=1.2)
        fig.text(.085, .777, "Without slow loop", color=orange, fontsize=14, weight="bold")
        fig.text(.515, .777, "With slow loop", color=teal, fontsize=14, weight="bold")

    near_zero = story["settled"] <= story["calibrated"] * .01 and story["calibrated"] > 0
    final_value = "~0 ppb" if near_zero else f"{story['settled']:,.0f} ppb"
    cards = [("INITIAL / UNCORRECTED", f"{story['initial']:,.0f} ppb", muted),
             ("AFTER CALIBRATION", f"{story['calibrated']:,.0f} ppb", orange),
             ("AFTER SLOW LOOP", final_value, teal)]
    card_y = .060 if panels_only else .661
    for i, (label, value, color) in enumerate(cards):
        left = .085 + i * .285
        fig.add_artist(FancyBboxPatch((left, card_y), .26, .082, transform=fig.transFigure,
                                     boxstyle="round,pad=0.012", linewidth=0, facecolor="#19253a"))
        fig.text(left + .012, card_y + .063, label, fontsize=9, color=muted)
        fig.text(left + .012, card_y + .017, value, fontsize=23, weight="bold", color=color)

    ax = fig.add_axes([.115, .49, .79, .435] if panels_only else [.115, .355, .79, .255], facecolor=bg)
    bx = fig.add_axes([.115, .235, .79, .20] if panels_only else [.115, .160, .79, .125],
                     facecolor=bg, sharex=ax)
    if not panels_only:
        ax.set_title("Time error grows while GPS is off", loc="left", fontsize=15, pad=17, color=ink)
        bx.set_title("Remaining frequency bias", loc="left", fontsize=15, pad=17, color=ink)
    ax.set_ylabel("Time error magnitude (ms)", fontsize=12)
    bx.set_ylabel("Bias magnitude (ppb)", fontsize=12)
    bx.set_xlabel("GPS wake cycle", fontsize=12, labelpad=8)
    count = story["count"]
    for panel in (ax, bx):
        panel.set_xlim(0, count)
        for spine in panel.spines.values():
            spine.set_visible(False)
        panel.grid(axis="y", color="#34435c", alpha=.5, lw=.7)
        panel.tick_params(length=0, labelsize=11, pad=8)
        panel.axhline(0, color=muted, alpha=.45, lw=1)
    ax.tick_params(labelbottom=False)
    bx.set_xticks(np.unique(np.linspace(0, count, min(6, count + 1), dtype=int)))
    ymax = max(float(np.max(story["slow"])), float(np.max(story["fixed"])), .05)
    ax.set_ylim(-.045 * ymax, ymax * 1.22)
    biasmax = max(float(np.max(story["bias"])), float(np.max(story["fixed_bias"])), 10)
    bx.set_ylim(-.065 * biasmax, biasmax * 1.15)
    # Subtle bands mark the expanded GPS-on portions of each cycle.
    for i in range(count):
        ax.axvspan(i + .76, i + 1, color=teal, alpha=.05)
    fixed_line, = ax.plot([], [], color=orange, lw=2.7)
    slow_line, = ax.plot([], [], color=teal, lw=2.7)
    fixed_dot, = ax.plot([], [], "o", color=orange, ms=6)
    slow_dot, = ax.plot([], [], "o", color=teal, ms=6)
    fixed_bias_line, = bx.plot([], [], color=orange, lw=2, ls=(0, (5, 3)))
    slow_bias_line, = bx.plot([], [], color=teal, lw=2.7)
    cursors = [panel.axvline(0, color=muted, alpha=.35, lw=.8) for panel in (ax, bx)]
    message = None
    if not panels_only:
        message = fig.text(.085, .064, "", fontsize=16, weight="bold")
        fig.text(.085, .033, story["source_note"], fontsize=10, color=muted)
        fig.text(.085, .013, story["footer"], fontsize=9, color=muted)
    status = fig.text(.905, .952 if panels_only else .626, "", ha="right", fontsize=11, color=muted)
    nframes = round(args.duration * args.fps)
    intro, outro = 1.5, 2.5
    x = story["x"]

    def update(frame: int) -> list:
        seconds = frame / args.fps
        progress = np.clip((seconds - intro) / (args.duration - intro - outro), 0, 1)
        position = progress * count
        end = max(1, int(np.searchsorted(x, position, side="right")))
        for line, values in ((fixed_line, story["fixed"]), (slow_line, story["slow"]),
                             (fixed_bias_line, story["fixed_bias"]), (slow_bias_line, story["bias"])):
            line.set_data(x[:end], values[:end])
        fixed_dot.set_data([x[end - 1]], [story["fixed"][end - 1]])
        slow_dot.set_data([x[end - 1]], [story["slow"][end - 1]])
        for cursor in cursors:
            cursor.set_xdata([position, position])
        recovering = position % 1 > .76
        if panels_only:
            recovering = progress >= 1 or recovering
            status.set_text(f"Cycle {min(int(position) + 1, count)}  |  "
                            + ("GPS ON / recovering" if recovering else "GPS OFF / holdover"))
        elif progress >= 1:
            status.set_text(f"{count} cycles complete")
            message.set_text("The slow loop drives bias toward zero." if near_zero
                             else "GPS recovers time; the slow loop tracks drift.")
        elif seconds < intro:
            status.set_text("Comparing GPS duty cycles" if args.without_loop else "Same starting calibration")
            message.set_text("Two recordings. Different long-term drift." if args.without_loop
                             else "Same calibration. Different long-term drift.")
        else:
            status.set_text(f"Cycle {min(int(position) + 1, count)}  |  "
                            + ("GPS ON / recovering" if recovering else "GPS OFF / holdover"))
            message.set_text("GPS recovers time. The slow loop learns the rate." if recovering
                             else "Without GPS, remaining frequency bias becomes time error.")
        return [fixed_line, slow_line, fixed_dot, slow_dot, fixed_bias_line, slow_bias_line,
                *cursors, status] + ([message] if message is not None else [])

    animation = FuncAnimation(fig, update, frames=nframes, interval=1000 / args.fps,
                              blit=False, repeat=True, cache_frame_data=False)
    return fig, animation, update


def main() -> None:
    parser = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    parser.add_argument("input", nargs="?", type=Path, default=Path("E:/syslog.txt"))
    parser.add_argument("--output", type=Path, default=Path("analysis/gps_holdover_animation/holdover.mp4"))
    parser.add_argument("--mode", choices=("illustration", "log"), default="illustration")
    parser.add_argument("--layout", choices=("full", "panels"), default="full",
                        help="Full story, or only graph panels, cycle status and bottom bias figures")
    parser.add_argument("--without-loop", type=Path, help="Independent no-slow-loop syslog (log mode)")
    parser.add_argument("--session", type=int, default=-1, help="1-based boot session; default last")
    parser.add_argument("--cycles", type=int, help="Illustration: default 20; log: all complete cycles")
    parser.add_argument("--gain", type=float, default=.25, help="Fraction of bias removed per recovery")
    parser.add_argument("--holdover-s", type=float, help="Illustration: override median measured PPS gap")
    parser.add_argument("--initial-ppb", type=float, help="Illustration: raw oscillator bias magnitude")
    parser.add_argument("--calibrated-ppb", type=float, help="Illustration: remaining bias after calibration")
    parser.add_argument("--summary-window", type=int, default=5, help="Cycles for early / late bias medians")
    parser.add_argument("--min-gap-s", type=float, default=120, help="Minimum PPS gap counted as holdover")
    parser.add_argument("--duration", type=float, default=20, help="Total video duration in seconds")
    parser.add_argument("--fps", type=int, default=20)
    parser.add_argument("--size", type=int, default=1080, help="Square pixel dimensions")
    parser.add_argument("--demo", action="store_true", help="Use example values without reading a log")
    args = parser.parse_args()
    validate_args(parser, args)
    if args.output.suffix.lower() == ".mp4" and not writers.is_available("ffmpeg"):
        parser.error("MP4 requires ffmpeg on PATH; install ffmpeg or use --output holdover.gif")
    try:
        slow = None if args.demo else read_log(args.input, args.session, args.min_gap_s)
        fixed = read_log(args.without_loop, args.session, args.min_gap_s) if args.without_loop else None
        story = build_story(args, slow, fixed)
    except (OSError, ValueError) as error:
        parser.error(str(error))
    if slow:
        print(f"Source: {slow.path}; session {slow.session}; {len(slow.cycles)} completed cycles; "
              f"{slow.updates} slow-loop updates")
    print(f"{story['source_note']}: {story['calibrated']:.1f} -> {story['settled']:.1f} ppb magnitude")
    args.output.parent.mkdir(parents=True, exist_ok=True)
    fig, animation, update = draw_animation(story, args)
    writer = (FFMpegWriter(fps=args.fps, codec="libx264", extra_args=["-pix_fmt", "yuv420p", "-crf", "22",
               "-movflags", "+faststart"], metadata={"title": "GPS holdover and slow frequency correction",
               "comment": story["source_note"]})
              if args.output.suffix.lower() == ".mp4" else PillowWriter(fps=args.fps))
    print(f"Rendering {args.size} x {args.size}, {args.duration:g} s at {args.fps} fps...")
    animation.save(str(args.output), writer=writer)
    update(round(args.duration * args.fps) - 1)
    poster = args.output.with_suffix(".png")
    fig.savefig(poster, facecolor=fig.get_facecolor(), dpi=args.size / 10.8)
    plt.close(fig)
    cycle_csv = args.output.with_suffix(".csv")
    with cycle_csv.open("w", newline="", encoding="utf-8") as target:
        writer_csv = csv.DictWriter(target, fieldnames=list(story["rows"][0]))
        writer_csv.writeheader()
        writer_csv.writerows(story["rows"])
    metadata = dict(mode=args.mode, layout=args.layout, source_note=story["source_note"],
                    slow_log=slow.path if slow else None, slow_session=slow.session if slow else None,
                    without_loop_log=fixed.path if fixed else None,
                    without_loop_session=fixed.session if fixed else None,
                    initial_uncorrected_magnitude_ppb=story["initial"],
                    early_residual_magnitude_ppb=story["calibrated"],
                    late_residual_magnitude_ppb=story["settled"], cycles=story["count"],
                    gain=args.gain if args.mode == "illustration" else None,
                    fps=args.fps, duration_s=args.duration, size_px=args.size,
                    notes=__doc__)
    args.output.with_suffix(".json").write_text(json.dumps(metadata, indent=2), encoding="utf-8")
    print(f"Saved {args.output}\nPoster: {poster}\nCycle values: {cycle_csv}")


if __name__ == "__main__":
    main()
