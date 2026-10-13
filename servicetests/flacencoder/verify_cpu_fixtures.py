"""Generate benchmark fixtures and independently round-trip every one with Xiph FLAC."""
import argparse
import json
import os
import re
import subprocess
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]

def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--results", type=Path, required=True)
    parser.add_argument("--flac", type=Path, required=True)
    args = parser.parse_args()
    fixtures = args.results.resolve() / "fixtures"
    env = dict(os.environ, FLAC_CPU_FIXTURES=str(fixtures))
    subprocess.run(["rtk", "cargo", "test", "-p", "raylar-flac-encoder-service", "--features", "benchmark",
                    "--target", "x86_64-pc-windows-msvc", "--release", "write_benchmark_desktop_fixtures",
                    "--", "--ignored"], cwd=ROOT, env=env, check=True)
    files = sorted(fixtures.glob("*.flac"))
    assert len(files) == 864, len(files)
    version = subprocess.run([str(args.flac.resolve()), "--version"], capture_output=True, text=True, check=True).stdout.strip()
    for i, file in enumerate(files):
        depth = int(re.match(r"depth(\d+)", file.name)[1])
        decoded = subprocess.run([str(args.flac.resolve()), "--decode", "--stdout", "--silent",
                                  "--force-raw-format", "--endian=little", "--sign=signed", str(file)], capture_output=True)
        assert decoded.returncode == 0, (file.name, decoded.stderr.decode(errors="replace"))
        expected = (fixtures / f"expected-{depth}.raw").read_bytes()
        assert decoded.stdout == expected, file.name
        if (i + 1) % 108 == 0:
            print(f"Decoded and matched {i + 1}/864", flush=True)
    result = {"decoder": version, "fixtures": len(files), "bit_exact_pcm_matches": len(files)}
    (args.results / "verification.json").write_text(json.dumps(result, indent=2) + "\n")
    print(json.dumps(result))

if __name__ == "__main__":
    main()
