"""Export a completed 05 snapshot over SWD; no reset, flash or SD writes."""
import argparse
import datetime
import hashlib
import json
import math
from pathlib import Path
import struct
import subprocess
import wave


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--audio-address', type=lambda s: int(s, 0), required=True)
    parser.add_argument('--meta-address', type=lambda s: int(s, 0), required=True)
    parser.add_argument('--output', type=Path, default=Path(__file__).parent / 'evidence')
    args = parser.parse_args()
    args.output.mkdir(parents=True, exist_ok=True)
    commands = []

    def read(address, words, filename):
        if not 0x20000000 <= address < address + words * 4 <= 0x20018000:
            raise ValueError('Snapshot must be entirely within SRAM1')
        destination = args.output / filename
        command = ['probe-rs', 'read', 'b32', hex(address), str(words),
                   '--output', str(destination), '--format', 'binary',
                   '--chip', 'STM32L476RGTx', '--probe', '0483:3748',
                   '--speed', '100', '--non-interactive']
        commands.append(command)
        result = subprocess.run(command, stdout=subprocess.PIPE,
                                stderr=subprocess.STDOUT, text=True, timeout=120)
        (args.output / (filename + '.log')).write_text(result.stdout, encoding='utf-8')
        result.check_returncode()
        data = destination.read_bytes()
        if len(data) != words * 4:
            raise ValueError('Incomplete SWD read')
        return data

    metadata = struct.unpack('<12I', read(args.meta_address, 12, 'snapshot-meta.bin'))
    if metadata[:8] != (int.from_bytes(b'L4AU', 'little'), 1, 16000, 8192, 1, 24, 32, 1):
        raise ValueError('Snapshot is not ready or has an unexpected format; wait for PASS')
    raw = read(args.audio_address, metadata[3], 'snapshot-raw.bin')
    fnv = 0x811c9dc5
    for byte in raw:
        fnv = ((fnv ^ byte) * 0x01000193) & 0xffffffff
    if fnv != metadata[10]:
        raise ValueError('RAM data hash differs from the stopped firmware snapshot')
    words = struct.unpack('<8192i', raw)
    samples = [word >> 8 for word in words]
    if any(word & 0xf8 for word in words):
        raise ValueError('Unexpected pending/reserved flags in raw DFSDM words')
    pcm = b''.join((sample & 0xffffff).to_bytes(3, 'little') for sample in samples)
    with wave.open(str(args.output / 'snapshot-pcm24.wav'), 'wb') as wav:
        wav.setnchannels(1)
        wav.setsampwidth(3)
        wav.setframerate(metadata[2])
        wav.writeframes(pcm)
    mean = sum(samples) / len(samples)
    rms_ac = math.sqrt(sum((x - mean) ** 2 for x in samples) / len(samples))
    result = {
        'utc': datetime.datetime.now(datetime.timezone.utc).isoformat(),
        'commands': commands, 'capture_stopped_before_read': True,
        'sample_rate_hz_nominal': metadata[2], 'channels': 1, 'valid_bits': 24,
        'raw_word_bits': 32, 'words': len(samples), 'duration_seconds': 0.512,
        'start_sample': metadata[8] | metadata[9] << 32,
        'raw_fnv32': f'{fnv:08x}', 'matches_firmware_hash': True,
        'raw_sha256': hashlib.sha256(raw).hexdigest(),
        'wav_sha256': hashlib.sha256((args.output / 'snapshot-pcm24.wav').read_bytes()).hexdigest(),
        'min': min(samples), 'max': max(samples), 'mean_dc': mean,
        'rms_ac': rms_ac, 'rms_ac_dbfs': 20 * math.log10(max(rms_ac, 1e-30) / 8388608),
        'unique_samples': len(set(samples)),
        'distinct_low_pcm_bytes': len(set(x & 255 for x in samples)),
        'clipped_samples': sum(x in (-8388608, 8388607) for x in samples),
        'normalization_or_dc_removal': False,
    }
    (args.output / 'snapshot.json').write_text(json.dumps(result, indent=2) + '\n')
    print(json.dumps(result, indent=2))


if __name__ == '__main__':
    main()
