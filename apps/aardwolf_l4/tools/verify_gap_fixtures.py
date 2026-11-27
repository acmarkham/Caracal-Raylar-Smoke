"""Independently decode silence-recovery fixtures from the recorder host tests."""
import argparse
import array
import importlib.util
import hashlib
import json
import os
from pathlib import Path
import subprocess

ROOT = Path(__file__).resolve().parents[3]
parser = argparse.ArgumentParser(description=__doc__)
parser.add_argument('--fixtures', type=Path, default=ROOT / 'target/l4-gaps/fixtures')
parser.add_argument('--flac', type=Path, default=ROOT / 'target/flac-tools/flac-1.5.0-win/Win64/flac.exe')
parser.add_argument('--generate', action='store_true')
parser.add_argument('--golden', type=Path, help='Optional existing six-channel SHA-256 manifest')
args = parser.parse_args()
if args.generate:
    env = dict(os.environ, RAYLAR_CRCL_TEST_OUTPUT=str(args.fixtures.resolve()))
    subprocess.run(['rtk', 'cargo', 'test', '--lib', '-p', 'raylar-audio-recorder-service',
                    '--target', 'x86_64-pc-windows-msvc', '--locked', '--offline'],
                   cwd=ROOT, env=env, check=True)
spec = importlib.util.spec_from_file_location('crcl_decode', ROOT / 'scripts/crcl_decode.py')
decoder = importlib.util.module_from_spec(spec)
spec.loader.exec_module(decoder)
count = 0
for initial in (0, 128):
    for fmt in ('wav', 'flac'):
        for segment in range(3):
            stem = args.fixtures / f'gap-{initial}-{fmt}-{segment}'
            reference = stem.with_suffix('.' + fmt)
            expected = stem.with_suffix('.s32le').read_bytes()
            ref = decoder.reference(reference)
            assert ref['raw'] == expected, f'{stem}: reference PCM differs'
            if fmt == 'flac':
                # A second, independent FLAC decoder, preserving native 24-bit samples.
                raw = subprocess.run([str(args.flac), '-d', '-c', '-s', '--force-raw-format',
                                      '--endian=little', '--sign=signed', str(reference)],
                                     check=True, capture_output=True).stdout
                packed = b''.join(int.from_bytes(raw[n:n+3], 'little', signed=True).to_bytes(4, 'little', signed=True)
                                  for n in range(0, len(raw), 3))
                assert packed == expected, f'{stem}: Xiph FLAC differs'
            pcm, report = decoder.decode(stem.with_suffix('.CRCL').read_bytes(), ref)
            original = array.array('i')
            original.frombytes(stem.with_suffix('.interleaved.s32le').read_bytes())
            assert len(pcm) == len(original) and report['complete']
            assert report['completion_status'] == (1 if segment < 2 else 2)
            for n in range(0, len(pcm), 4):
                assert pcm[n] == original[n], f'{stem}: reference alignment'
                if original[n:n+4] == array.array('i', [0, 0, 0, 0]):
                    assert pcm[n:n+4] == original[n:n+4], f'{stem}: nonzero gap'
                else:
                    # The side channels are deliberately quantized; verify their
                    # retained waveform and source position within that error bound.
                    assert max(abs(pcm[n+c]-original[n+c]) for c in range(1, 4)) <= 512
            assert report['frames'] == len(expected) // 4
            count += 1
print(f'PASS: {count} paired gap fixtures; exact reference PCM, aligned zero gaps, CRCL completion and retained side channels')
if args.golden:
    before = json.loads(args.golden.read_text())
    assert before and all(hashlib.sha256((args.fixtures / name).read_bytes()).hexdigest() == sha
                          for name, sha in before.items()), 'Existing six-channel output changed'
    print(f'PASS: {len(before)} existing six-channel fixture files are byte-identical')
