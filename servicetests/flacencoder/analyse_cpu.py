"""Turn the on-board DWT matrix into CSV, charts, provenance and a standalone report."""
import argparse
import csv
from datetime import datetime
import hashlib
import html
import itertools
import json
import re
import shutil
import subprocess
from pathlib import Path

DATA = {0: "recorded", 1: "silence", 2: "full-range noise"}
CODEC = {0: "WAV PCM", 1: "FLAC production", 2: "FLAC experimental"}
METHOD = {0: "auto", 1: "Rice4", 2: "Rice5"}
SEARCH = {0: "exact bit counts", 1: "exhaustive shifts", 2: "mean ±1 repeated passes", 3: "mean ±1 fused pass"}
ROOT = Path(__file__).resolve().parents[2]

def table(rows, columns):
    headers = "".join(f"<th>{html.escape(label)}</th>" for _, label in columns)
    body = ""
    for row in rows:
        cells = ""
        for key, _ in columns:
            value = row[key]
            if isinstance(value, float):
                value = f"{value:.3f}"
            cells += f"<td>{html.escape(str(value))}</td>"
        body += f"<tr>{cells}</tr>"
    return f"<table><thead><tr>{headers}</tr></thead><tbody>{body}</tbody></table>"

def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--results", type=Path, required=True)
    parser.add_argument("--log", type=Path)
    parser.add_argument("--firmware", type=Path, required=True)
    parser.add_argument("--additional-log", type=Path)
    parser.add_argument("--additional-firmware", type=Path)
    args = parser.parse_args()
    destination = args.results
    log = args.log or destination / "board.log"
    text = log.read_text(encoding="utf-8-sig")
    assert "CPU_BENCH_DONE rows=720" in text, "Incomplete hardware matrix"
    metadata = re.search(r"CPU_META (.*?) \(", text)[1]
    meta = dict(re.findall(r"(\w+)=(\w+)", metadata))
    assert int(meta["signed_width"]) == json.loads((destination / "corpus.json").read_text())["signed_width"]
    core_hz = int(meta["nominal_hz"])
    base_rows = [dict((key, int(value)) for key, value in re.findall(r"(\w+)=(\d+)", line))
            for line in text.splitlines() if "CPU_ROW " in line]
    supplemental = []
    extra_meta = None
    if args.additional_log:
        extra = args.additional_log.read_text(encoding="utf-8-sig")
        assert "CPU_BENCH_DONE rows=288" in extra
        supplemental = [dict((key, int(value)) for key, value in re.findall(r"(\w+)=(\d+)", line))
                        for line in extra.splitlines() if "CPU_ROW " in line]
        assert len(supplemental) == 288
        extra_meta = dict(re.findall(r"(\w+)=(\w+)", re.search(r"CPU_META (.*?) \(", extra)[1]))
        assert extra_meta["nominal_hz"] == meta["nominal_hz"]
        assert extra_meta["signed_width"] == meta["signed_width"]
    for row in base_rows:
        row["run"] = "base"
    for row in supplemental:
        row["run"] = "supplement"
    rows = base_rows + [r for r in supplemental if r["codec"] == 2]
    keys = ["data", "depth", "block", "codec", "order", "method", "search"]
    expected = {(data, depth, block, codec, 255, 0, 0)
                for data, depth, block, codec in itertools.product(range(3), [8, 16, 24, 32], [1024, 2048, 4096], range(2))}
    expected.update((0, depth, block, 2, order, method, search)
                    for depth, block, order, method, search in itertools.product([8, 16, 24, 32], [1024, 2048, 4096], [255, 0, 1, 2, 3, 4], range(3), range(3)))
    if supplemental:
        expected.update((0, depth, block, 2, order, method, 3)
                        for depth, block, order, method in itertools.product([8, 16, 24, 32], [1024, 2048, 4096], [255, 0, 1, 2, 3, 4], range(3)))
    actual = {tuple(row[key] for key in keys) for row in rows}
    assert len(rows) == len(actual) == len(expected) and actual == expected
    all_rows = base_rows + supplemental
    baselines = {(r["run"], r["data"], r["depth"], r["block"]): r for r in all_rows if r["codec"] == 0}
    for row in all_rows:
        total = row["select"] + row["emit"]
        row["cycles_per_sample"] = total / row["samples"]
        row["cpu_percent"] = 100 * total * 16000 / (core_hz * row["samples"])
        baseline = baselines[row["run"], row["data"], row["depth"], row["block"]]
        baseline_cpu = 100 * baseline["emit"] * 16000 / (core_hz * baseline["samples"])
        row["delta_pp"] = row["cpu_percent"] - baseline_cpu
        row["wav_cpu_percent"] = baseline_cpu
        row["select_cpu_percent"] = 100 * row["select"] * 16000 / (core_hz * row["samples"])
        row["emit_cpu_percent"] = 100 * row["emit"] * 16000 / (core_hz * row["samples"])
        row["max_frame_ms"] = row["max"] * 1000 / core_hz
        row["max_frame_duty_percent"] = 100 * row["max"] * 16000 / (core_hz * row["block"])
        row["ratio_percent"] = 100 * row["bytes"] / (row["samples"] * row["depth"] / 8)
        row["bits_per_sample"] = 8 * row["bytes"] / row["samples"]
        row["data_name"] = DATA[row["data"]]
        row["codec_name"] = CODEC[row["codec"]]
        row["predictor"] = "auto" if row["order"] == 255 else str(row["order"])
        row["rice_method"] = METHOD[row["method"]]
        row["search_name"] = SEARCH[row["search"]]
    with (destination / "matrix.csv").open("w", newline="", encoding="utf-8") as file:
        writer = csv.DictWriter(file, fieldnames=list(rows[0]))
        writer.writeheader()
        writer.writerows(rows)
    if supplemental:
        with (destination / "repeated-baselines.csv").open("w", newline="", encoding="utf-8") as file:
            writer = csv.DictWriter(file, fieldnames=list(rows[0]))
            writer.writeheader()
            writer.writerows(r for r in supplemental if r["codec"] != 2)
    lookup = {tuple(row[key] for key in keys): row for row in rows}
    # Both exact implementations must select identical bounded encodings for
    # every constrained order/method. Production must match the default choice.
    for row in rows:
        if row["data"] == 0 and row["codec"] == 2 and row["search"] == 0:
            exhaustive = lookup[(0, row["depth"], row["block"], 2, row["order"], row["method"], 1)]
            for field in ["bytes", "o0", "o1", "o2", "o3", "o4", "verbatim", "constant", "rice4", "rice5"]:
                assert row[field] == exhaustive[field], (row, field)
            if row["order"] == 255 and row["method"] == 0:
                production = lookup[(0, row["depth"], row["block"], 1, 255, 0, 0)]
                assert row["bytes"] == production["bytes"]
        if row["search"] == 3:
            original_mean = lookup[(0, row["depth"], row["block"], 2, row["order"], row["method"], 2)]
            for field in ["bytes", "o0", "o1", "o2", "o3", "o4", "verbatim", "rice4", "rice5"]:
                assert row[field] == original_mean[field]
        if row["codec"] == 2 and row["method"] == 0:
            for method in [1, 2]:
                constrained = lookup[(0, row["depth"], row["block"], 2, row["order"], method, row["search"])]
                assert row["bytes"] <= constrained["bytes"]
        if row["codec"] == 2 and row["order"] == 255:
            for order in range(5):
                constrained = lookup[(0, row["depth"], row["block"], 2, order, row["method"], row["search"])]
                assert row["bytes"] <= constrained["bytes"]
    verification = json.loads((destination / "verification.json").read_text())
    provenance = {
        "probe": "0483:374b:0667FF505277504867014435", "chip": "STM32U595VJ",
        "board": "Raylar v1.0", "metadata": meta,
        "frequency_calibration_hz": int(meta["measured_cycles"]) * 1e6 / int(meta["measured_us"]),
        "firmware_sha256": hashlib.sha256(args.firmware.read_bytes()).hexdigest(),
        "log_sha256": hashlib.sha256(log.read_bytes()).hexdigest(),
        "source_git": subprocess.run(["rtk", "proxy", "git", "rev-parse", "HEAD"], cwd=ROOT, capture_output=True, text=True, check=True).stdout.strip(),
        "compiler": subprocess.run(["rtk", "proxy", "rustc", "--version"], capture_output=True, text=True, check=True).stdout.strip(),
        "build": "release, opt-level=s; encoder opt-level=3; lto=false; codegen-units=1; thumbv8m.main-none-eabihf",
        "rows": len(rows), "verification": verification,
        "measurement": "DWT cycles, interrupts enabled, only timer ISR active; input preparation, commit, logging, SD/capture/radio excluded",
    }
    if args.additional_log:
        provenance["supplemental"] = {
            "metadata": extra_meta,
            "firmware_sha256": hashlib.sha256(args.additional_firmware.read_bytes()).hexdigest(),
            "log_sha256": hashlib.sha256(args.additional_log.read_bytes()).hexdigest(),
            "rows": len(supplemental), "new_variants": 216,
            "baseline_repeats": "repeated-baselines.csv",
            "source_snapshot": "fused-source",
        }
    original = destination / "aardwolf-original.elf"
    if original.exists():
        provenance["restored_firmware_sha256"] = hashlib.sha256(original.read_bytes()).hexdigest()
    restored = destination / "restore.log"
    provenance["restoration_verified"] = restored.exists() and "Saved firmware verified; reset command succeeded." in restored.read_text(encoding="utf-8-sig")
    (destination / "provenance.json").write_text(json.dumps(provenance, indent=2) + "\n")
    # Preserve the measured source changes alongside the binary and data.
    patch = subprocess.run(["rtk", "proxy", "git", "diff", "--binary"], cwd=ROOT, capture_output=True, check=True).stdout
    (destination / ("fused-tracked-source.patch" if supplemental else "tracked-source.patch")).write_bytes(patch)
    for folder in ["crates/services/flacencoder", "servicetests/flacencoder"]:
        for file in (ROOT / folder).rglob("*"):
            if file.is_file() and file.suffix in [".rs", ".toml", ".py", ".ps1", ".md"]:
                target = destination / ("fused-source" if supplemental else "source") / file.relative_to(ROOT)
                target.parent.mkdir(parents=True, exist_ok=True)
                shutil.copy2(file, target)
    import matplotlib
    matplotlib.use("Agg")
    import matplotlib.pyplot as plt
    fig, axes = plt.subplots(1, 2, figsize=(12, 4.6), constrained_layout=True)
    for block in [1024, 2048, 4096]:
        group = [lookup[(0, depth, block, 1, 255, 0, 0)] for depth in [8, 16, 24, 32]]
        axes[0].plot([r["depth"] for r in group], [r["cpu_percent"] for r in group], marker="o", label=f"FLAC {block}")
        axes[1].plot([r["depth"] for r in group], [r["ratio_percent"] for r in group], marker="o", label=f"FLAC {block}")
    wav = [lookup[(0, depth, 2048, 0, 255, 0, 0)] for depth in [8, 16, 24, 32]]
    axes[0].plot([r["depth"] for r in wav], [r["cpu_percent"] for r in wav], "k--", label="WAV 2048")
    axes[0].set_ylabel("Codec CPU duty at 16 kHz (%)")
    axes[1].set_ylabel("Encoded frame bytes / WAV PCM bytes (%)")
    for ax in axes:
        ax.set_xlabel("Bit depth (8/16-bit input is shifted)")
        ax.set_xticks([8, 16, 24, 32])
        ax.grid(alpha=.25)
        ax.legend()
    fig.suptitle("Measured on STM32U595VJ, 144 MHz — recorded audio")
    fig.savefig(destination / "depths.png", dpi=160)
    plt.close(fig)
    search_count = 4 if supplemental else 3
    if search_count == 4:
        fig, grid = plt.subplots(2, 2, figsize=(12, 9), constrained_layout=True, sharex=True, sharey=True)
        axes = grid.flatten()
    else:
        fig, axes = plt.subplots(1, search_count, figsize=(5 * search_count, 4.6), constrained_layout=True, sharex=True, sharey=True)
    markers = {0: "o", 1: "s", 2: "^"}
    for search, ax in enumerate(axes):
        for method in range(3):
            group = [r for r in rows if r["data"] == 0 and r["depth"] == 32 and r["block"] == 2048 and r["codec"] == 2 and r["search"] == search and r["method"] == method]
            ax.scatter([r["cpu_percent"] for r in group], [r["ratio_percent"] for r in group], marker=markers[method], label=METHOD[method], alpha=.75)
            if method == 0:
                for r in group:
                    ax.annotate(r["predictor"], (r["cpu_percent"], r["ratio_percent"]), xytext=(3, 4), textcoords="offset points", fontsize=8)
        ax.set_title(SEARCH[search])
        ax.set_xlabel("Codec CPU duty (%)")
        ax.grid(alpha=.25)
        ax.legend()
    axes[0].set_ylabel("Frame bytes / WAV PCM bytes (%)")
    fig.suptitle("32-bit / 2048 samples: CPU vs compressed size; labels are predictor orders")
    fig.savefig(destination / "tradeoffs.png", dpi=160)
    plt.close(fig)
    prod = lookup[(0, 32, 2048, 1, 255, 0, 0)]
    best = min((r for r in rows if r["data"] == 0 and r["depth"] == 32 and r["block"] == 2048 and r["codec"] == 2), key=lambda r: r["cpu_percent"])
    baseline_rows = [r for r in rows if r["data"] == 0 and r["codec"] in [0, 1]]
    predictor_rows = [lookup[(0, 32, 2048, 2, order, 0, 0)] for order in [255, 0, 1, 2, 3, 4]]
    strategy_rows = [lookup[(0, 32, 2048, 2, 255, method, search)] for search, method in itertools.product(range(search_count), range(3))]
    order_one = lookup[(0, 32, 2048, 2, 1, 0, 0)]
    exact = lookup[(0, 32, 2048, 2, 255, 0, 0)]
    exhaustive = lookup[(0, 32, 2048, 2, 255, 0, 1)]
    mean = lookup[(0, 32, 2048, 2, 255, 0, 2)]
    fused = lookup.get((0, 32, 2048, 2, 255, 0, 3))
    block4096 = lookup[(0, 32, 4096, 1, 255, 0, 0)]
    findings = (f"<p>For this recorded input, forcing order 1 with auto Rice reduces codec duty from {prod['cpu_percent']:.2f}% to {order_one['cpu_percent']:.2f}% "
                f"and increases FLAC frame bytes by {100 * (order_one['bytes'] / prod['bytes'] - 1):.2f}%. Production selected order 0 in {prod['o0']} frames, order 1 in {prod['o1']}, and order 2 in {prod['o2']}. "
                "That makes order 1 a useful optional performance setting to validate on broader audio, while auto keeps the best compressed size across all supported predictors.</p>"
                f"<p>With automatic prediction and Rice method, experimental exact bit counts use {exact['cpu_percent']:.2f}% CPU, exhaustive shifts {exhaustive['cpu_percent']:.2f}%, "
                f"and repeated-pass mean search {mean['cpu_percent']:.2f}%.")
    if fused:
        findings += f" The fused mean version reduces that to {fused['cpu_percent']:.2f}%, but exact bit counts remain faster."
    findings += (f" All {search_count} searches produced the same size at these default dimensions; mean search is heuristic and does not guarantee the best parameter on other residual distributions. "
                 "The experimental exact loop avoids incremental state/budget handling, explaining its smaller duty than production despite identical frames.</p>"
                 f"<p>Increasing the production block from 2048 to 4096 saves only {prod['cpu_percent'] - block4096['cpu_percent']:.2f} CPU percentage points here, "
                 f"increases frame bytes by {100 * (block4096['bytes'] / prod['bytes'] - 1):.2f}%, and increases the longest measured frame from {prod['max_frame_ms']:.2f} to {block4096['max_frame_ms']:.2f} ms. "
                 "The existing 2048 block is a sensible choice for these recordings. 24-bit and 32-bit FLAC take almost identical CPU and bytes because they retain the same source values. "
                 "Retain auto Rice: limiting it to Rice4 saves very little with bit counts and removes efficient parameters for larger residuals.</p>")
    supplemental_note = ("The supplemental fused-mean run repeats all 72 baselines; its deltas use WAV timings from that same run, retained in repeated-baselines.csv." if supplemental else "")
    data_rows = [lookup[(data, 32, 2048, codec, 255, 0, 0)] for data, codec in itertools.product(range(3), range(2))]
    base_columns = [("depth", "Depth"), ("block", "Block"), ("codec_name", "Codec"), ("cpu_percent", "CPU %"), ("delta_pp", "Δ vs WAV (pp)"), ("ratio_percent", "Size % of PCM"), ("max_frame_ms", "Slowest frame ms")]
    predictor_columns = [("predictor", "Order"), ("cpu_percent", "CPU %"), ("delta_pp", "Δ vs WAV (pp)"), ("select_cpu_percent", "Select %"), ("emit_cpu_percent", "Emit %"), ("ratio_percent", "Size % of PCM"), ("verbatim", "Verbatim frames"), ("max_frame_ms", "Slowest frame ms")]
    strategy_columns = [("search_name", "Search"), ("rice_method", "Method"), ("cpu_percent", "CPU %"), ("delta_pp", "Δ vs WAV (pp)"), ("ratio_percent", "Size % of PCM"), ("verbatim", "Verbatim frames")]
    all_columns = [("data_name", "Input")] + base_columns + [("predictor", "Order"), ("rice_method", "Rice"), ("search_name", "Search"), ("verbatim", "Verbatim"), ("o0", "Order 0"), ("o1", "Order 1"), ("o2", "Order 2"), ("o3", "Order 3"), ("o4", "Order 4"), ("rice4", "Rice4 frames"), ("rice5", "Rice5 frames"), ("max_frame_duty_percent", "Max frame duty %")]
    content = f"""<!doctype html><html lang="en"><meta charset="utf-8"><title>Aardwolf FLAC CPU measurements</title>
<style>body{{font:16px system-ui;max-width:1250px;margin:40px auto;padding:0 24px;color:#202b37;line-height:1.55}}h1,h2{{line-height:1.2}}table{{border-collapse:collapse;font-size:13px;width:100%;margin:16px 0}}td,th{{padding:7px 10px;border-bottom:1px solid #dde3ea;text-align:right}}th{{background:#edf2f7;position:sticky;top:0}}td:first-child,th:first-child{{text-align:left}}img{{max-width:100%}}.scroll{{overflow:auto;max-height:650px}}input{{padding:10px;width:400px;max-width:90%}}code{{background:#edf2f7;padding:2px 4px}}a{{color:#1264aa}}</style>
<h1>Aardwolf FLAC CPU measurements</h1><p>{datetime.fromtimestamp(log.stat().st_mtime).strftime('%d %B %Y')} · actual Raylar v1.0 / STM32U595VJ · 144 MHz · SMPS · mono 16 kHz</p>
<p><strong>Current 32-bit / 2048-sample FLAC: {prod['cpu_percent']:.2f}% codec CPU, {prod['delta_pp']:+.2f} percentage points above WAV.</strong>
Selection accounts for {prod['select_cpu_percent']:.2f}% and frame emission for {prod['emit_cpu_percent']:.2f}%. Encoded frames occupy {prod['ratio_percent']:.2f}% of equivalent WAV PCM bytes.</p>
<p>Lowest measured codec cost at 32-bit / 2048: order {best['predictor']}, {best['rice_method']}, {best['search_name']} = {best['cpu_percent']:.2f}% CPU and {best['ratio_percent']:.2f}% of WAV PCM size.
This is a result for these excerpts, not a recommendation to force that setting for all audio.</p>
<p><a href="matrix.csv">All {len(rows)} unique measurements (CSV)</a> · <a href="{log.name}">Raw base board log</a> · <a href="board.log">Latest board log</a> · <a href="corpus.json">Input provenance</a> · <a href="provenance.json">Build and measurement provenance</a> · <a href="verification.json">Decoder verification</a> · <a href="restore.log">Firmware restoration</a></p>
<h2>Scope and method</h2><p>Every case processed 32,768 samples: four distributed 4096-sample excerpts, repeated twice.
The full source spans 370,688 samples and requires 21 signed bits. The 24- and 32-bit cases preserve identical original PCM values;
16-bit uses arithmetic right shift 5 and 8-bit right shift 13, prepared outside timing. Narrower cases therefore quantize the input.
All matrix rows use the same excerpts. Silence and deterministic full-range xorshift noise are additional production/WAV baselines.</p>
<p>DWT counts CPU cycles during selection and emission, including FLAC framing, CRCs and the encoder's normal checks.
CPU duty = cycles × 16,000 / (144,000,000 × samples) × 100. Deltas are percentage points above WAV serialization at the same depth and block size.
Clock cross-check against the hardware timer: {provenance['frequency_calibration_hz'] / 1e6:.6f} MHz. Timer interrupts remain enabled; only that ISR is active.
Logging, input preparation, commit, executor waits, filesystem headers/finalization, microphone capture, SD, radio and GPS are excluded.
This measures incremental codec cost, not whole-board CPU utilization or SD backpressure.
WAV baselines use minimal PCM packers at each depth; the production recorder currently writes 32-bit WAV.
{supplemental_note}
The FLAC emission measurement includes its PCM validation; input conversion before both packers is excluded.</p>
<p>Production runs the actual incremental analyzer with a 256-residual budget. Experimental exact bit counts use the same quotient recurrence with selectable order/method;
exhaustive shifts accumulate exact costs for every legal parameter; mean ±1 estimates k from floor(log₂(mean folded residual)) and checks neighboring parameters exactly.
The repeated-pass version recomputes residuals per candidate (up to six passes for auto Rice); the fused version checks the union of candidates in one additional residual pass using native 32-bit shifts. Both select identical frames.
Rice4 permits k=0–14; Rice5 permits k=0–30. All use one residual partition.
Forced order/method retains constant handling and bounded verbatim fallback. A row with verbatim frames measures that fallback as well.
Both exact approaches matched encoded byte counts and choices throughout the matrix. All {verification['fixtures']} experimental files decoded with {html.escape(verification['decoder'])} and matched expected PCM byte for byte.</p>
<h2>Bit depth and block size</h2><img src="depths.png" alt="CPU duty and compressed size by depth and block size">{table(baseline_rows, base_columns)}
<h2>Predictor orders at 32-bit / 2048</h2><p>Exact bit-count search, automatic Rice method. Size includes frame overhead; WAV reference is PCM payload.</p>{table(predictor_rows, predictor_columns)}
<h2>Rice methods and search approaches at 32-bit / 2048</h2><p>Automatic predictor selection for this table. The plots include all orders.</p>{table(strategy_rows, strategy_columns)}<img src="tradeoffs.png" alt="CPU/compression tradeoffs across all predictor orders and Rice methods">
<h2>Dependence on source data</h2>{table(data_rows, [('data_name', 'Input')] + base_columns[2:])}
<h2>Interpretation</h2>{findings}<p>Use auto prediction for varying microphone content unless a constrained strategy has been validated on broader recordings.
Lower CPU settings can increase frame size; a reduction in SD traffic may offset codec cost, but this isolated matrix does not measure that effect.
4096-sample blocks have a 256 ms frame budget at 16 kHz, 2048 have 128 ms, and 1024 have 64 ms; compare maximum frame timing as well as average duty.
Changing between 24 and 32 stored bits preserves these source samples, while 8/16-bit results include quantization. The recorded signal occupies fewer than 24 bits, so stored bit depth alone need not predict runtime or residual size.
Silence and full-range noise show data dependence; they do not establish worst-case CPU bounds.</p>
<h2>Full matrix</h2><p>Filter table text, for example <code>Rice5</code>, <code>noise</code> or <code>mean</code>. CSV retains exact cycle counts and statistics.</p>
<input id="filter" placeholder="Filter rows" aria-label="Filter full matrix"><div class="scroll" id="matrix">{table(rows, all_columns)}</div>
<script>document.getElementById('filter').addEventListener('input',e=>{{const term=e.target.value.toLowerCase();document.querySelectorAll('#matrix tbody tr').forEach(r=>r.hidden=!r.textContent.toLowerCase().includes(term))}})</script></html>"""
    (destination / "report.html").write_text(content, encoding="utf-8")
    summary = {"production_32_2048": prod, "lowest_cpu_32_2048": best, "rice_strategies_32_2048": strategy_rows}
    (destination / "summary.json").write_text(json.dumps(summary, indent=2) + "\n")
    print(json.dumps({"rows": len(rows), "production_cpu_percent": prod["cpu_percent"], "production_delta_pp": prod["delta_pp"], "lowest_cpu_percent": best["cpu_percent"]}, indent=2))

if __name__ == "__main__":
    main()
