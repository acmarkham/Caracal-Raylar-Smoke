"""Prepare four distributed 4096-sample excerpts of captured mono s32le PCM."""
import argparse
import hashlib
import json
import struct
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]

def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("input", type=Path)
    parser.add_argument("--results", type=Path, required=True)
    args = parser.parse_args()
    raw = args.input.read_bytes()
    samples = [x[0] for x in struct.iter_unpack("<i", raw)]
    assert len(samples) >= 4 * 4096
    width = max(max(samples).bit_length() + 1, (~min(samples)).bit_length() + 1)
    starts = [(len(samples) - 4096) * i // 3 for i in range(4)]
    corpus = b"".join(raw[start * 4:(start + 4096) * 4] for start in starts)
    output = ROOT / "target/flac-cpu/corpus.s32le"
    output.parent.mkdir(parents=True, exist_ok=True)
    output.write_bytes(corpus)
    (output.parent / "signed_width.txt").write_text(str(width))
    args.results.mkdir(parents=True, exist_ok=True)
    (args.results / "corpus.s32le").write_bytes(corpus)
    manifest = {
        "input": str(args.input.resolve()), "input_sha256": hashlib.sha256(raw).hexdigest(),
        "input_samples": len(samples), "minimum": min(samples), "maximum": max(samples),
        "signed_width": width, "excerpt_starts": starts, "excerpt_samples": 4096,
        "corpus_samples": len(corpus) // 4, "corpus_sha256": hashlib.sha256(corpus).hexdigest(),
        "right_shifts": {str(depth): max(0, width - depth) for depth in [8, 16, 24, 32]},
    }
    (args.results / "corpus.json").write_text(json.dumps(manifest, indent=2) + "\n")
    print(json.dumps(manifest, indent=2))

if __name__ == "__main__":
    main()
