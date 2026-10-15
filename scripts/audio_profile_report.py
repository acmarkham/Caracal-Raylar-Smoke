"""Turn bounded CRCL PROFILE target logs into an inclusive cycle-budget report."""
import argparse
import csv
import json
from pathlib import Path
import re


def parse(text, compiled_control=False):
    config = None
    windows = []
    passed = ("CRCL PROFILE CONTROL PASS" if compiled_control else "CRCL PROFILE TARGET PASS") in text
    for line in text.splitlines():
        if "PROFILE " not in line:
            continue
        fields = dict(re.findall(r"(\w+)=([^\s]+)", line))
        if "core_hz" in fields:
            config = fields
        elif "window" in fields:
            windows.append({"summary": fields, "stages": [], "representations": {}})
        elif "stage" in fields and windows:
            windows[-1]["stages"].append(fields)
        elif "zero" in fields and windows:
            windows[-1]["representations"] = fields
    if config is None or len(windows) != (1 if compiled_control else 2) or not passed:
        raise ValueError("Incomplete run: require configuration, recording windows and target PASS")
    required = {"recorder_poll", "capture_check", "capture_publish", "source_copy", "spatial_transform",
                "crcl_serialize", "crcl_pack", "crcl_crc", "reference_feed", "reference_sha_compress",
                "flac_analysis", "flac_encode", "storage_buffer", "filesystem_append", "filesystem_finalize", "sd_read", "sd_write"}
    for window in windows:
        if int(window["summary"]["frames"]) != 960000:
            raise ValueError("Each window must contain exactly 960,000 aligned frames")
        if not required.issubset(s["stage"] for s in window["stages"]):
            raise ValueError("Missing profiling stages")
    return {"config": config, "windows": windows}


def report(run, source, compiled_control=None):
    config = run["config"]
    if compiled_control:
        control_config = compiled_control["config"]
        if config.get("codec", "spatial_v1") != control_config.get("codec", "spatial_v1") or config["core_hz"] != control_config["core_hz"]:
            raise ValueError("Compiled control must use the same codec and core clock")
    hz = int(config["core_hz"])
    empty = int(config["empty_span_cycles"])
    cheap = config.get("codec") == "temporal_shift_v2"
    prediction = "independent temporal Rice with per-channel power-of-two scaling, no CRCL CRC/SHA" if cheap else "direct spatial Rice"
    lines = ["# Hexaphonic audio stage profile", "", f"Source: `{source}`", "",
             f"Core: {hz / 1e6:g} MHz, SMPS; GPS off; laboratory time. Six channels at 16 kHz; "
             "MIC1 signed-32-bit FLAC, 2048-sample fixed-order-1 blocks; five 10-bit CRCL channels, "
             f"1024-frame {prediction} blocks; 16 KiB storage buffers.", "",
             "This bench excludes Aardwolf radio, persistent logging and power/location tasks. "
             "Cycle spans include interrupts that occur inside them. Async rows measure CPU only "
             "during polls; elapsed time includes suspension and scheduler delay. Parent/child rows "
             "are inclusive and must not be added together.", "",
             f"Empty measured span: {empty} cycles; complete probe overhead: "
             f"{config['probe_overhead_cycles']} cycles. Per-sample stages sample 1/32 values, with "
             "phase varied by microphone. Their estimates subtract the empty-span cost and scale by 32; "
             "they are approximate components of spatial_transform.", ""]
    for window in run["windows"]:
        summary = window["summary"]
        budget = int(summary["cycle_budget"])
        accounted = int(summary["recorder_cycles"]) + int(summary["capture_cycles"])
        title = "Detailed probes" if summary["window"] == "1" else "Control: detailed probes disabled"
        lines += [f"## {title}", "", f"Frames: {summary['frames']}; elapsed: {int(summary['elapsed_us']) / 1e6:.3f} s; "
                  f"accounted CPU: {100 * accounted / budget:.2f}%.", "",
                  "| Stage | CPU ms | Core budget % | Calls/polls | Maximum cycles | Async elapsed ms | Bytes |",
                  "| --- | ---: | ---: | ---: | ---: | ---: | ---: |"]
        for s in window["stages"]:
            if not int(s["calls"]):
                continue
            cycles = int(s["cycles"])
            sampled = s["sampled"] == "true"
            if sampled:
                cycles = max(0, cycles - empty * int(s["calls"])) * 32
            label = s["stage"] + (" (estimated)" if sampled else "")
            if cheap:
                label = label.replace("spatial_transform", "independent_transform").replace("spatial_delta_sample", "channel_peak_sample").replace("quantize_fast_sample", "shift_quantize_sample").replace("rice_select", "rice_estimate")
            lines.append(f"| {label} | {cycles * 1000 / hz:.3f} | {cycles * 100 / budget:.3f} | "
                         f"{s['calls']} | {s['max_cycles']} | {int(s['wall_us']) / 1000:.3f} | {s['bytes']} |")
        r = window["representations"]
        if r and sum(int(v) for v in r.values()):
            lines += ["", "CRCL channel-block representations: " + ", ".join(f"{k}={v}" for k, v in r.items()) + "."]
        lines.append("")
    if compiled_control:
        s = compiled_control["windows"][0]["summary"]
        cycles = int(s["recorder_cycles"]) + int(s["capture_cycles"])
        lines += ["## Control with detailed probes compiled out", "",
                  f"Frames: {s['frames']}; elapsed: {int(s['elapsed_us']) / 1e6:.3f} s; "
                  f"accounted CPU: {100 * cycles / int(s['cycle_budget']):.2f}%.", "",
                  "This separate build retains only the top-level recorder and capture probes. "
                  "The runtime-disabled control above retains instrumentation branches and potentially "
                  "different compiler optimization. Sound and SD allocation can differ between runs; "
                  "the comparison is an overhead check, not an exact correction for each stage.", ""]
    return "\n".join(lines)


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("log", type=Path)
    parser.add_argument("--output-dir", type=Path, default=Path("target/audio-profile"))
    parser.add_argument("--compiled-control-log", type=Path)
    args = parser.parse_args()
    run = parse(args.log.read_text(encoding="utf-8", errors="replace"))
    args.output_dir.mkdir(parents=True, exist_ok=True)
    control = parse(args.compiled_control_log.read_text(encoding="utf-8", errors="replace"), compiled_control=True) if args.compiled_control_log else None
    markdown = report(run, args.log, control)
    if control:
        run["compiled_control"] = control
    (args.output_dir / "report.md").write_text(markdown, encoding="utf-8")
    (args.output_dir / "profile.json").write_text(json.dumps(run, indent=2) + "\n", encoding="utf-8")
    with (args.output_dir / "stages.csv").open("w", newline="", encoding="utf-8") as output:
        fields = ["window", "stage", "calls", "cycles", "max_cycles", "wall_us", "bytes", "sampled"]
        writer = csv.DictWriter(output, fieldnames=fields)
        writer.writeheader()
        for window in run["windows"]:
            for stage in window["stages"]:
                writer.writerow({"window": window["summary"]["window"], **stage})
        if control:
            for stage in control["windows"][0]["stages"]:
                writer.writerow({"window": "compiled_control", **stage})
    print(markdown)


if __name__ == "__main__":
    main()
