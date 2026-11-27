"""Reassemble the read-only inspect-syslog RTT export into a text file."""
import argparse
from pathlib import Path
import re

p = argparse.ArgumentParser(description=__doc__)
p.add_argument("log", type=Path)
a = p.parse_args()
text = a.log.read_text(encoding="utf-8")
length = int(re.search(r"SD SYSLOG END length=(\d+)", text)[1])
raw = b"".join(bytes(int(v.strip(), 16) for v in row.split(",") if v.strip())
               for row in re.findall(r"SD SYSLOG data=\[([^]]*)\]", text))
assert len(raw) == min(16384, length), "Incomplete RTT export"
output = a.log.with_suffix(".syslog.txt")
output.write_bytes(raw)
print(f"Read {len(raw)} tail bytes from /syslog.txt ({length} bytes total): {output}")
