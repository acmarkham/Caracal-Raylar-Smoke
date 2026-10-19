"""Read-only checks of recording metadata and CRCL completion records."""
from pathlib import Path
import re
import struct
import wave


def reference_header(path):
    info = {'path': str(path), 'bytes': path.stat().st_size}
    if not info['bytes']:
        return dict(info, error='Empty reference file')
    if path.suffix.lower() == '.wav':
        with wave.open(str(path), 'rb') as stream:
            return dict(info, rate=stream.getframerate(), channels=stream.getnchannels(),
                        bits=stream.getsampwidth()*8, frames=stream.getnframes())
    with path.open('rb') as stream:
        if stream.read(4) != b'fLaC':
            raise ValueError('Invalid FLAC signature')
        while True:
            head = stream.read(4)
            if len(head) != 4:
                raise ValueError('Truncated FLAC metadata')
            length = int.from_bytes(head[1:], 'big')
            data = stream.read(length)
            if len(data) != length:
                raise ValueError('Truncated FLAC metadata payload')
            if head[0] & 127 == 0:
                if length != 34:
                    raise ValueError('Invalid STREAMINFO size')
                packed = int.from_bytes(data[10:18], 'big')
                info.update(rate=packed >> 44, channels=((packed >> 41) & 7)+1,
                            bits=((packed >> 36) & 31)+1, frames=packed & ((1 << 36)-1))
            if head[0] & 127 == 4:
                text = data.decode('utf8', errors='replace')
                for key in ('device_id', 'firmware_hash', 'boot_id', 'segment_id',
                            'first_source_frame', 'system_timestamp', 'start_utc'):
                    match = re.search(r'\b'+key+r'=([^;\x00]*)', text)
                    if match:
                        info[key] = match[1]
            if head[0] & 128:
                break
    if 'frames' not in info or not info['rate']:
        raise ValueError('Missing/invalid audio dimensions')
    if not info['frames']:
        info['error'] = 'Unfinalized STREAMINFO (zero samples)'
    return info


def crcl_end(path, reference):
    size = path.stat().st_size
    if size < 144:
        raise ValueError('Empty/truncated CRCL')
    with path.open('rb') as stream:
        header = stream.read(92)
        if not header.startswith(b'CRCL') or struct.unpack_from('<H', header, 8)[0] != 2:
            raise ValueError('Unsupported CRCL signature/version')
        stream.seek(-52, 2)
        end = stream.read(52)
    if end[:5] != b'CRBK\x03' or struct.unpack_from('<HI', end, 6) != (28, 52):
        raise ValueError('No final CRCL END record')
    frames, ref_bytes, blocks, status = struct.unpack_from('<QQII', end, 28)
    first = struct.unpack_from('<Q', header, 44)[0]
    block_frames = struct.unpack_from('<H', header, 40)[0]
    errors = []
    planned_frames = struct.unpack_from('<Q', header, 60)[0]
    rate = struct.unpack_from('<I', header, 32)[0]
    if planned_frames and abs(frames-planned_frames) > max(rate//20, block_frames):
        errors.append(f'Short/long segment: {frames} samples; planned {planned_frames}')
    if frames != reference.get('frames'): errors.append('Reference/CRCL sample mismatch')
    if ref_bytes != reference['bytes']: errors.append('Reference/CRCL byte-length mismatch')
    if status != 1: errors.append(f'CRCL completion status {status}')
    if not block_frames or blocks != (frames+block_frames-1)//block_frames:
        errors.append('CRCL block-count mismatch')
    if struct.unpack_from('<Q', end, 16)[0] != frames:
        errors.append('CRCL END offset mismatch')
    if reference.get('first_source_frame') is not None and str(first) != reference['first_source_frame']:
        errors.append('First-source-frame mismatch')
    if reference.get('segment_id') and header[16:32].hex() != reference['segment_id']:
        errors.append('Segment identity mismatch')
    return dict(bytes=size, frames=frames, first=first, blocks=blocks, status=status, errors=errors)


def audit(root, boot_id, start_utc, end_utc, channels, expected=True, intervals=None):
    """Only inspect files in this boot's UTC range; never modify the source."""
    result = dict(available=bool(root and start_utc is not None), files=[], missing=[],
                  root=str(root) if root else None, method='FLAC/WAV headers + CRCL END; payloads not decoded')
    if not result['available'] or not expected:
        return result
    first_minute = int(start_utc)//60*60
    last_minute = int(end_utc)//60*60
    windows = intervals or [(start_utc, end_utc)]
    wanted = set()
    session_for_minute = {}
    for index, (start, end) in enumerate(windows):
        minutes = range(int(start)//60*60, int(end)//60*60+1, 60)
        wanted.update(minutes)
        session_for_minute.update((minute,index) for minute in minutes)
    found = {}
    folders = [p for p in Path(root).iterdir() if p.is_dir() and p.name.isdigit()
               and first_minute//3600*3600 <= int(p.name) <= last_minute]
    for folder in sorted(folders):
        paths = {p.name.lower(): p for p in folder.iterdir() if p.is_file()}
        refs = {}
        for name, path in paths.items():
            match = re.fullmatch(r'aud_(\d+)\.(flac|wav|crcl)', name)
            if match and int(match[1]) in wanted:
                refs.setdefault(int(match[1]), {})[match[2]] = path
        for stamp, pair in sorted(refs.items()):
            refpath = pair.get('flac', pair.get('wav'))
            errors = []
            ref = {}
            extra = {}
            try:
                if refpath is None:
                    raise ValueError('CRCL without reference')
                ref = reference_header(refpath)
                if boot_id is not None and ref.get('boot_id') and str(boot_id) != ref['boot_id']:
                    continue
                if ref.get('error'): errors.append(ref['error'])
                if 'crcl' in pair or channels > ref.get('channels', 1):
                    if 'crcl' not in pair: raise ValueError('Missing CRCL companion')
                    extra = crcl_end(pair['crcl'], ref)
                    errors.extend(extra['errors'])
            except (ValueError, OSError, EOFError, wave.Error, struct.error) as error:
                errors.append(str(error))
            row = dict(stamp=stamp, name=refpath.name if refpath else pair['crcl'].name,
                reference=ref, crcl=extra, errors=errors, complete=not errors,
                frames=ref.get('frames', 0), bytes=ref.get('bytes', 0)+extra.get('bytes', 0))
            found[stamp] = row
    result['files'] = [found[k] for k in sorted(found)]
    # Do not invent missing minutes before the logged audio start. A final open
    # minute is retained as red; a terminal file need not be finalized yet.
    if not found:
        result['available'] = False
        return result
    result['missing'] = sorted(wanted-found.keys())
    previous = None
    for row in result['files']:
        if row['complete'] and row['crcl']:
            if previous and session_for_minute[row['stamp']] == session_for_minute[previous['stamp']]:
                gap = row['crcl']['first']-previous['crcl']['first']-previous['frames']
                if gap:
                    row['errors'].append(f'Source-frame discontinuity: {gap:+d} samples')
                    row['complete'] = False
            previous = row
        else:
            previous = None
    return result
