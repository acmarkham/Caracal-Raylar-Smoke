"""Independently decode an actual hardware FLAC block exported after DMA stops."""
import argparse
import hashlib
import json
from pathlib import Path
import re
import subprocess

p = argparse.ArgumentParser()
p.add_argument('log', type=Path)
p.add_argument('--flac', type=Path, required=True)
p.add_argument('--ffmpeg', default='ffmpeg')
a = p.parse_args()
text = a.log.read_text(encoding='utf-8')
size, samples, expected = re.search(r'FIXTURE bytes=(\d+) pcm_samples=(\d+) pcm_hash=([0-9a-f]+)', text).groups()
data = bytearray()
for offset, values in re.findall(r'FIXTURE offset=(\d+) data=\[([^\]]*)\]', text):
    assert int(offset) == len(data), ('missing RTT fixture chunk', offset, len(data))
    data.extend(int(value.strip(), 16) for value in values.split(',') if value.strip())
assert len(data) == int(size)
fixture = a.log.with_suffix('.flac')
fixture.write_bytes(data)
def run(args):
    result = subprocess.run(['rtk', 'proxy'] + [str(x) for x in args], capture_output=True)
    assert result.returncode == 0, (args, result.stderr.decode(errors='replace'))
    return result.stdout, result.stderr.decode(errors='replace')
_, crc_diagnostics = run([a.flac, '--test', '--silent', fixture])
raw, diagnostics = run([a.flac, '--decode', '--stdout', '--silent', '--force-raw-format', '--endian=little', '--sign=signed', fixture])
decoded, _ = run([a.ffmpeg, '-v', 'error', '-i', fixture, '-f', 's32le', '-acodec', 'pcm_s32le', '-'])
assert len(raw) == int(samples) * 3 and len(decoded) == int(samples) * 4
pcm = b''.join(int.from_bytes(raw[n:n+3], 'little', signed=True).to_bytes(4, 'little', signed=True) for n in range(0, len(raw), 3))
ffmpeg_pcm = b''.join((int.from_bytes(decoded[n:n+4], 'little', signed=True) >> 8).to_bytes(4, 'little', signed=True) for n in range(0, len(decoded), 4))
assert pcm == ffmpeg_pcm
value = 0x811c9dc5
for byte in pcm:
    value = ((value ^ byte) * 0x01000193) & 0xffffffff
assert value == int(expected, 16), (hex(value), expected)
record = {'samples': int(samples), 'bits': 24, 'sample_rate_hz': 16000,
          'fixture_bytes': len(data), 'fixture_sha256': hashlib.sha256(data).hexdigest(),
          'pcm_fnv32': f'{value:08x}', 'both_decoders_match_hardware_pcm': True,
          'xiph_crc_test': True, 'diagnostics': crc_diagnostics + diagnostics,
          'xiph_version': run([a.flac, '--version'])[0].decode().strip(),
          'ffmpeg_version': run([a.ffmpeg, '-version'])[0].decode().splitlines()[0]}
a.log.with_suffix('.decode.json').write_text(json.dumps(record, indent=2) + '\n')
print(json.dumps(record, indent=2))
