#!/usr/bin/env python3
"""Independent CRCL v1 decoder and reference verifier (Python standard library).

FLAC references need ffmpeg on PATH. WAV references use the standard wave reader.
Use --recover to expose an explicitly incomplete, validated prefix after a bad tail.
"""
import argparse
import array
import hashlib
import json
from pathlib import Path
import re
import struct
import subprocess
import sys
import wave


class FormatError(ValueError):
    pass


def require(condition, message):
    if not condition:
        raise FormatError(message)


def _crc_table():
    result = []
    for i in range(256):
        for _ in range(8):
            i = (i >> 1) ^ (0x82F63B78 if i & 1 else 0)
        result.append(i)
    return result


CRC_TABLE = _crc_table()


def crc32c(data):
    crc = 0xFFFFFFFF
    for byte in data:
        crc = (crc >> 8) ^ CRC_TABLE[(crc ^ byte) & 255]
    return crc ^ 0xFFFFFFFF


def number(data, offset, fmt):
    size = struct.calcsize(fmt)
    require(offset + size <= len(data), "truncated numeric field")
    return struct.unpack_from("<" + fmt, data, offset)[0]


def tlvs(data, checkpoint=False):
    values = {}
    offset = 0
    while offset < len(data):
        require(offset + 8 <= len(data), "truncated TLV")
        kind, flags, size = struct.unpack_from("<HHI", data, offset)
        offset += 8
        require(flags in (0, 1) and offset + size <= len(data), "invalid TLV flags/length")
        require(kind not in values, "duplicate TLV")
        if kind not in range(1, 12):
            require(flags == 0, "unknown required TLV")
        else:
            values[kind] = data[offset:offset + size]
        offset += size
    required = {1, 2} if checkpoint else {1, 2, 3, 4, 5, 6, 7, 11}
    require(required <= values.keys(), "missing mandatory metadata")
    if checkpoint:
        require(values.keys() <= {1, 2, 8, 9}, "unexpected checkpoint metadata")
    lengths = {1: 16, 2: 28, 4: 4, 6: 6, 8: 16, 9: 24, 11: 1}
    for kind, size in lengths.items():
        if kind in values:
            require(len(values[kind]) == size, "invalid metadata field length")
    require(values[2][16] in (1, 2) and values[2][17] in (0, 1)
            and values[2][18:20] == b"\0\0", "invalid time quality")
    if 9 in values:
        require(values[9][16] in (0, 1, 2) and values[9][17:] == bytes(7), "invalid location")
    for kind, limit in ((3, 32), (5, 128), (7, 512), (10, 128)):
        if kind in values:
            require(0 < len(values[kind]) <= limit, "invalid text metadata length")
            values[kind].decode("utf-8", errors="strict")
    if not checkpoint:
        require(values[6] == bytes(range(1, 7)) and values[11][0] in (0, 1, 2), "invalid channel/prediction metadata")
        keys = ("pdm_clock_hz", "sinc_order", "decimation", "cic_scale", "hpf_enabled", "pcm_shift", "gain_tenths_db")
        pattern = ";".join(key + r"=-?\d+" for key in keys)
        require(re.fullmatch(pattern, values[7].decode("utf-8")) is not None, "invalid capture configuration")
    return values


class Bits:
    def __init__(self, data):
        self.data, self.position = data, 0

    def read(self, width):
        require(self.position + width <= len(self.data) * 8, "truncated bit payload")
        value = 0
        for _ in range(width):
            value = (value << 1) | ((self.data[self.position // 8] >> (7 - self.position % 8)) & 1)
            self.position += 1
        return value

    def signed(self, width):
        value = self.read(width)
        return value - (1 << width) if value & (1 << (width - 1)) else value

    def rice(self, k, maximum):
        quotient = 0
        while self.read(1) == 0:
            quotient += 1
            require(quotient <= maximum >> k, "excessive unary quotient")
        unsigned = (quotient << k) | self.read(k)
        require(unsigned <= maximum, "Rice residual outside bounds")
        return unsigned // 2 if unsigned % 2 == 0 else -(unsigned // 2) - 1

    def padding(self):
        remaining = len(self.data) * 8 - self.position
        require(remaining < 8 and self.read(remaining) == 0, "invalid padding or extra payload")


def reference(path):
    data = path.read_bytes()
    if data[:4] == b"fLaC":
        require(len(data) >= 42, "truncated FLAC")
        info = int.from_bytes(data[18:26], "big")
        rate, channels, depth = info >> 44, ((info >> 41) & 7) + 1, ((info >> 36) & 31) + 1
        offset, comments, last = 4, bytearray(), False
        while not last:
            require(offset + 4 <= len(data), "truncated FLAC metadata")
            last = bool(data[offset] & 128)
            kind = data[offset] & 127
            size = int.from_bytes(data[offset + 1:offset + 4], "big")
            require(offset + 4 + size <= len(data), "truncated FLAC metadata payload")
            if kind == 4:
                comments.extend(data[offset + 4:offset + 4 + size])
            offset += 4 + size
        raw = subprocess.run(["ffmpeg", "-v", "error", "-i", str(path), "-f", "s32le",
                              "-acodec", "pcm_s32le", "-"], check=True, capture_output=True).stdout
        fmt = 2
    else:
        with wave.open(str(path), "rb") as audio:
            rate, channels, depth = audio.getframerate(), audio.getnchannels(), audio.getsampwidth() * 8
            raw = audio.readframes(audio.getnframes())
        require(data[:4] == b"RIFF" and data[8:12] == b"WAVE", "unsupported reference")
        offset, comments = 12, bytearray()
        while offset + 8 <= len(data):
            size = number(data, offset + 4, "I")
            require(offset + 8 + size <= len(data), "truncated WAV chunk")
            if data[offset:offset + 4] == b"LIST":
                comments.extend(data[offset + 8:offset + 8 + size])
            offset += 8 + size + (size & 1)
        fmt = 1
    require(channels == 1 and depth == 32 and len(raw) % 4 == 0, "reference must be mono signed 32-bit PCM")
    pcm = array.array("i")
    pcm.frombytes(raw)
    if sys.byteorder != "little":
        pcm.byteswap()
    return dict(rate=rate, format=fmt, pcm=pcm, raw=raw, metadata=bytes(comments), file_bytes=len(data))


def decode(data, ref, recover=False):
    require(len(data) >= 96 and data[:8] == b"CRCL\r\n\x1a\n", "invalid CRCL header")
    length = number(data, 10, "H")
    require(number(data, 8, "H") == 1 and 96 <= length <= 4096 and length <= len(data), "unsupported version/header length")
    require(number(data, 12, "I") == 0 and data[76:92] == bytes(16), "unsupported flags/reserved fields")
    require(number(data, 68, "I") == length - 96 and number(data, 72, "I") > 0, "invalid metadata length/timebase")
    require(number(data, length - 4, "I") == crc32c(data[:length - 4]), "header CRC mismatch")
    require(data[36:40] == bytes((32, 1, 6, 5)), "unsupported channel profile")
    block, precision = number(data, 40, "H"), data[42]
    require(block in (256, 512, 1024, 2048, 4096) and precision in (6, 8, 10, 12), "invalid block/precision")
    require(number(data, 32, "I") == ref["rate"] and data[43] == ref["format"], "reference profile mismatch")
    metadata = tlvs(data[92:length - 4])
    identity = hashlib.sha256(b"CRCL-segment-v1" + struct.pack("<H", len(metadata[3])) + metadata[3]
                              + metadata[4] + data[52:60] + data[44:52]).digest()[:16]
    require(identity == data[16:32], "segment identity mismatch")
    require(b"segment_id=" + identity.hex().encode() in ref["metadata"], "reference segment identity missing/mismatched")
    require(b"first_source_frame=" + str(number(data, 44, "Q")).encode() + b";companion_format=CRCL"
            in ref["metadata"], "reference source offset/companion mismatch")
    output = array.array("i")
    offset, frames, sequence, records = length, 0, 0, 0
    short, ended, digest_verified, recovery_error = False, False, False, None
    status = None
    limit = (1 << (precision - 1)) - 1
    try:
        while offset < len(data):
            require(offset + 32 <= len(data) and data[offset:offset + 4] == b"CRBK", "truncated/invalid record header")
            header, size = number(data, offset + 6, "H"), number(data, offset + 8, "I")
            kind = data[offset + 4]
            maximum = 96 + 5 * ((block * precision + 7) // 8) if kind == 1 else 548 if kind == 2 else 92
            require(kind in (1, 2, 3) and size <= maximum and 32 <= header <= size - 4
                    and offset + size <= len(data), "invalid/truncated record size")
            record = data[offset:offset + size]
            require(record[5] == 0 and record[27] == 0, "invalid record flags")
            require(number(record, header - 4, "I") == crc32c(record[:header - 4]), "record header CRC mismatch")
            require(number(record, size - 4, "I") == crc32c(record[:size - 4]), "record CRC mismatch")
            require(number(record, 12, "I") == sequence and number(record, 16, "Q") == frames, "record sequence/sample alignment mismatch")
            count = number(record, 24, "H")
            if kind == 1:
                require(header == 92 and record[26] == 5 and 1 <= count <= block and not short, "invalid DATA dimensions/order")
                require(frames + count <= len(ref["pcm"]), "reference is shorter than sidecar")
                channels, payload = [], 92
                for c in range(5):
                    descriptor = 28 + c * 12
                    channel_id, mode, reserved, k, scale, payload_size = struct.unpack_from("<BBBBII", record, descriptor)
                    require(channel_id == c + 2 and mode <= 3 and reserved == 0, "invalid channel descriptor")
                    require(k <= precision if mode in (1, 2) else k == 0, "invalid Rice parameter")
                    require(payload + payload_size <= size - 4 and payload_size <= (count * precision + 7) // 8, "invalid channel payload size")
                    require((scale == 0 and payload_size == 0) if mode == 0 else 0 < scale <= 0x7FFFFFFF, "invalid scale/zero representation")
                    bits = Bits(record[payload:payload + payload_size])
                    previous, values = 0, []
                    for n in range(count):
                        if mode == 0:
                            q = 0
                        elif mode == 3 or (mode == 2 and n == 0):
                            q = bits.signed(precision)
                        elif mode == 1:
                            q = bits.rice(k, 2 * limit)
                        else:
                            q = previous + bits.rice(k, 4 * limit)
                        require(-limit <= q <= limit, "invalid quantized value")
                        previous = q
                        delta = (2 * abs(q) * scale + limit) // (2 * limit)
                        values.append(delta if q >= 0 else -delta)
                    bits.padding()
                    channels.append(values)
                    payload += payload_size
                require(payload == size - 4, "extra DATA payload")
                for n in range(count):
                    value = ref["pcm"][frames + n]
                    decoded = [value] + [value + channel[n] for channel in channels]
                    require(all(-0x80000000 <= v < 0x80000000 for v in decoded), "reconstruction exceeds i32 export range")
                    output.extend(decoded)
                frames += count
                records += 1
                short = count < block
            else:
                require(header == 32 and count == 0 and record[26] == 0, "invalid TIME/END dimensions")
                if kind == 2:
                    tlvs(record[32:size - 4], checkpoint=True)
                else:
                    require(size == 92 and offset + size == len(data), "invalid END or bytes after END")
                    total, reference_bytes, data_records, status = struct.unpack_from("<QQII", record, 32)
                    require(total == frames == len(ref["pcm"]) and reference_bytes == ref["file_bytes"]
                            and data_records == records and status in (1, 2, 4), "END/reference totals mismatch")
                    require(hashlib.sha256(ref["raw"]).digest() == record[56:88], "reference PCM digest mismatch")
                    digest_verified, ended = True, True
            sequence += 1
            offset += size
        require(ended, "missing END record")
    except FormatError as error:
        if not recover:
            raise
        recovery_error = str(error)
    return output, dict(frames=frames, sample_rate_hz=ref["rate"], channels=6, precision=precision,
                        data_records=records, complete=ended, digest_verified=digest_verified,
                        completion_status=status, recovery_error=recovery_error, segment_id=identity.hex())


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("sidecar", type=Path)
    parser.add_argument("--reference", required=True, type=Path)
    parser.add_argument("--output", type=Path, help="reconstructed six-channel 32-bit WAV")
    parser.add_argument("--recover", action="store_true", help="explicitly expose a validated incomplete prefix")
    args = parser.parse_args()
    try:
        pcm, report = decode(args.sidecar.read_bytes(), reference(args.reference), args.recover)
        if args.output:
            if sys.byteorder != "little":
                pcm.byteswap()
            with wave.open(str(args.output), "wb") as audio:
                audio.setparams((6, 4, report["sample_rate_hz"], 0, "NONE", "not compressed"))
                audio.writeframes(pcm.tobytes())
        print(json.dumps(report, indent=2))
    except (FormatError, UnicodeError, OSError, subprocess.CalledProcessError, wave.Error) as error:
        parser.exit(1, f"CRCL decode failed: {error}\n")


if __name__ == "__main__":
    main()
