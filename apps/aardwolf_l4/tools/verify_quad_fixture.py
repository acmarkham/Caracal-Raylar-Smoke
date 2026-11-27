"""Check the real four-lane PCM, FLAC frame and CRCL prefix exported after DMA stops."""
import argparse
import hashlib
import json
from pathlib import Path
import re
import struct
import subprocess
import sys

sys.path.insert(0, str(Path(__file__).resolve().parents[3] / "scripts"))
from crcl_decode import decode, reference

p = argparse.ArgumentParser(description=__doc__)
p.add_argument("log", type=Path)
p.add_argument("--flac", type=Path, required=True, help="Xiph FLAC executable")
a = p.parse_args()
data = {kind: bytearray() for kind in ("pcm", "flac", "CRCL")}
for kind, offset, values in re.findall(r"L4 QUAD FIXTURE kind=(\w+) offset=(\d+) data=\[([^\]]*)\]", a.log.read_text()):
    assert int(offset) == len(data[kind]), ("missing RTT chunk", kind, offset)
    data[kind].extend(int(v.strip(), 16) for v in values.split(",") if v.strip())
assert len(data["pcm"]) == 4096
for kind, content in data.items():
    a.log.with_suffix("." + kind).write_bytes(content)
path = a.log.with_suffix(".flac")
test = subprocess.run([str(a.flac), "--test", "--silent", str(path)], capture_output=True)
assert test.returncode == 0, test.stderr
xiph = subprocess.run([str(a.flac), "--decode", "--stdout", "--silent", "--force-raw-format", "--endian=little", "--sign=signed", str(path)], capture_output=True, check=True).stdout
xiph_pcm = [int.from_bytes(xiph[n:n+3], "little", signed=True) for n in range(0, len(xiph), 3)]
ref = reference(path)
assert list(ref["pcm"]) == xiph_pcm and len(xiph_pcm) == 256
original = struct.unpack("<1024i", data["pcm"])
decoded, report = decode(data["CRCL"], ref, recover=True)
assert report["frames"] == 256 and report["channels"] == 4 and not report["complete"]
assert report["recovery_error"] == "missing END record", report
assert list(decoded[::4]) == list(original[::4]) == xiph_pcm[:256]
offset = int.from_bytes(data["CRCL"][10:12], "little")
errors = []
for c in range(1, 4):
    d = offset + 28 + (c-1)*12
    shift = int.from_bytes(data["CRCL"][d+4:d+8], "little")
    half = (1 << (shift-1)) if shift else 0
    expected = [((original[n*4+c]+half) >> shift) << shift for n in range(256)]
    assert list(decoded[c::4]) == expected
    errors.append({"lane": c, "shift": shift, "max_abs_error": max(abs(decoded[n*4+c]-original[n*4+c]) for n in range(256)), "bound": half})
assert all(len(set(original[c::4])) > 1 for c in range(4))
assert len({tuple(original[c::4]) for c in range(4)}) == 4, "duplicated microphone lane"
record = {"frames_checked": 256, "channels": 4, "sample_rate_hz": 16000, "source_bits": 24,
          "flac_frames": 256, "xiph_crc_test": True, "xiph_and_ffmpeg_match": True,
          "reference_exact": True, "all_three_crcl_lanes_match_rounded_source": True,
          "distinct_varying_microphones": True, "quantization": errors,
          "sha256": {kind: hashlib.sha256(content).hexdigest() for kind,content in data.items()},
          "sidecar_fixture": "one complete DATA record; deliberate prefix, not a finalized recording",
          "xiph_diagnostics": test.stderr.decode(errors="replace")}
a.log.with_suffix(".decode.json").write_text(json.dumps(record, indent=2) + "\n")
print(json.dumps(record, indent=2))
