"""Audit overnight continuity using FLAC/CRCL headers and END records.

Streams large radio logs; audio payloads are deliberately not exhaustively decoded.
The card is read only and an immutable local log snapshot is preserved first.
"""
import argparse
import collections
import datetime as dt
import hashlib
import json
from pathlib import Path
import re
import struct
from zoneinfo import ZoneInfo

from aardwolf_sd_diagnostics_review import numeric_fields, save_csv, save_json
from crcl_decode import number, tlvs
from overnight_files import flac

PREFIX = re.compile(r'(\d{10}) ([\d.]+) (TRACE|DEBUG|INFO|WARN|ERROR)\s+(\w+)\s+')


def new_run(boot):
    return dict(boot=boot, rows=[], first_seconds=None, last_seconds=0, records=0,
                previous_sequence=None, sequence_gaps=[], sd={}, incomplete=[],
                radio_counts=collections.Counter(), radio_reasons=collections.Counter(),
                radio_errors=collections.Counter(), radio_states=collections.Counter(),
                radio_max={}, radio_io={}, radio_samples={}, radio_recent=collections.OrderedDict(),
                radio_totals={}, radio_last_id=0, radio_id_gaps=[])


def log_run(path):
    runs = []
    with path.open(encoding='utf8', errors='replace') as stream:
        for line in stream:
            matches = list(PREFIX.finditer(line))
            for i, m in enumerate(matches):
                seq, seconds, level, source = m.groups()
                msg = line[m.end():matches[i+1].start() if i+1 < len(matches) else len(line)].strip('\0\r\n ')
                if msg.startswith('boot node='):
                    runs.append(new_run(numeric_fields(msg)['boot']))
                if not runs:
                    continue
                run = runs[-1]
                seq, seconds = int(seq), float(seconds)
                if run['first_seconds'] is None:
                    run['first_seconds'] = seconds
                if run['previous_sequence'] is not None and seq != run['previous_sequence'] + 1:
                    run['sequence_gaps'].append([run['previous_sequence'], seq])
                run.update(previous_sequence=seq, last_seconds=seconds, records=run['records']+1)
                if msg.startswith('radio timing id='):
                    r = numeric_fields(msg)
                    r['io'] = []
                    run['radio_counts'][r['op']] += 1
                    run['radio_reasons'][f"{r['op']}:{r['reason']}:{r['result']}"] += 1
                    if r['id'] != run['radio_last_id'] + 1:
                        run['radio_id_gaps'].append([run['radio_last_id'], r['id']])
                    run['radio_last_id'] = r['id']
                    run['radio_recent'][r['id']] = r
                    if len(run['radio_recent']) > 64:
                        run['radio_recent'].popitem(last=False)
                    for metric in ('us', 'max_poll_us'):
                        key = r['op'] + ':' + metric
                        if key not in run['radio_max'] or r[metric] > run['radio_max'][key][metric]:
                            run['radio_max'][key] = r
                    continue
                if msg.startswith('radio timing_error '):
                    r = numeric_fields(msg)
                    error = msg.split('error=', 1)[1]
                    family = error.split('{', 1)[0].strip()
                    run['radio_errors'][family] += 1
                    parent = run['radio_recent'].get(r['id'])
                    if parent is not None:
                        parent['error'] = error
                        run['radio_samples'].setdefault(parent['op'] + ':' + family, parent)
                    continue
                if msg.startswith('radio timing_io '):
                    r = numeric_fields(msg)
                    required = {'id', 'kind', 'calls', 'err', 'cancelled', 'us', 'poll_us', 'max_us', 'max_poll_us', 'tag', 'start_ticks', 'end_ticks'}
                    if required - r.keys():
                        run['incomplete'].append(dict(sequence=seq, message=msg))
                        continue
                    parent = run['radio_recent'].get(r['id'])
                    if parent is not None:
                        parent['io'].append(r)
                    stats = run['radio_io'].setdefault(r['kind'], dict(calls=0, errors=0, cancelled=0, us=0, poll_us=0, max_us=0, max_poll_us=0))
                    for key in ('calls', 'cancelled', 'us', 'poll_us'):
                        stats[key] += r[key]
                    stats['errors'] += r['err']
                    for key in ('max_us', 'max_poll_us'):
                        if r[key] > stats[key]:
                            stats[key] = r[key]
                            stats[key + '_record'] = r
                    continue
                if msg.startswith('radio timing_state '):
                    run['radio_states'][numeric_fields(msg)['mode']] += 1
                    continue
                if msg.startswith('radio timing_totals '):
                    r = numeric_fields(msg)
                    run['radio_totals'] = r
                    if 'observer_late_us' in r:
                        prior = run.get('observer_late_max', {})
                        if r['observer_late_us'] > prior.get('observer_late_us', -1):
                            run['observer_late_max'] = r
                if msg.startswith('storage sd id='):
                    r = numeric_fields(msg)
                    r.update(io=[], log_seconds=seconds)
                    if r['id'] in run['sd']:
                        raise ValueError('duplicate SD operation ID')
                    run['sd'][r['id']] = r
                elif msg.startswith('storage sd_io '):
                    r = numeric_fields(msg)
                    run['sd'][r['id']]['io'].append(r)
                else:
                    run['rows'].append(dict(sequence=seq, seconds=seconds, level=level, source=source, message=msg))
    return runs


def packed_header_end(path, ref):
    with path.open('rb') as stream:
        data = stream.read(4096)
        if len(data) < 92 or number(data, 8, 'H') != 2:
            raise ValueError('short or unsupported CRCL header')
        length = number(data, 10, 'H')
        if not 92 <= length <= len(data):
            raise ValueError('invalid CRCL header size')
        metadata = tlvs(data[92:length])
        stream.seek(-52, 2)
        end = stream.read(52)
    if end[:5] != b'CRBK\x03' or number(end, 6, 'H') != 28 or number(end, 8, 'I') != 52:
        raise ValueError('no valid final CRCL END record')
    frames, refbytes, blocks, status = struct.unpack_from('<QQII', end, 28)
    r = dict(bytes=path.stat().st_size, first_frame=number(data, 44, 'Q'), frames=frames,
             blocks=blocks, status=status, reference_bytes=refbytes,
             boot=number(metadata[4], 0, 'I'), segment_id=data[16:32].hex(),
             sample_ticks=number(metadata[1], 8, 'Q'), block_frames=number(data, 40, 'H'))
    errors = []
    for matches, message in [
        (frames == ref['frames'], 'sample total mismatch'),
        (refbytes == ref['bytes'], 'reference byte length mismatch'),
        (str(r['first_frame']) == ref.get('first_source_frame'), 'first source frame mismatch'),
        (r['segment_id'] == ref.get('segment_id'), 'segment identity mismatch'),
        (str(r['sample_ticks']) == ref.get('system_timestamp'), 'sample timestamp mismatch'),
        (str(r['boot']) == ref.get('boot_id'), 'boot mismatch'),
        (blocks == (frames + r['block_frames'] - 1) // r['block_frames'], 'DATA block total mismatch'),
        (number(end, 16, 'Q') == frames, 'END source offset mismatch'),
    ]:
        if not matches:
            errors.append(message)
    r['errors'] = errors
    return r


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--card', type=Path, default=Path('E:/'))
    parser.add_argument('--output', type=Path, required=True)
    args = parser.parse_args()
    out = args.output
    out.mkdir(parents=True, exist_ok=True)
    digest = hashlib.sha256()
    with (args.card / 'syslog.txt').open('rb') as source, (out / 'syslog.txt').open('wb') as target:
        while chunk := source.read(1024*1024):
            digest.update(chunk)
            target.write(chunk)
    runs = log_run(out / 'syslog.txt')
    run = next(r for r in reversed(runs) if any(row['message'].startswith('audio started utc_us=') for row in r['rows']))
    rows = run['rows']
    start = next(r for r in rows if r['message'].startswith('audio started utc_us='))
    anchor = numeric_fields(start['message'])
    offset = (anchor['utc_us'] - anchor['ticks']) / 1e6

    def uk(ticks):
        return dt.datetime.fromtimestamp(offset + ticks/1e6, dt.timezone.utc).astimezone(ZoneInfo('Europe/London')).isoformat(timespec='milliseconds')

    ops = sorted(run['sd'].values(), key=lambda o: o['start_ticks'])
    for op in ops:
        op['uk_time'] = uk(op['start_ticks'])
        op['sd_us'] = sum(io['us'] for io in op['io'])
    save_json(out / 'operations.json', ops)
    save_csv(out / 'operations.csv', [{k:v for k,v in o.items() if k != 'io'} for o in ops])
    save_csv(out / 'sd-io.csv', [io for o in ops for io in o['io']])
    transitions = []
    for i, op in enumerate(ops):
        group = ops[i:i+5]
        if [o['op'] for o in group] == ['rewrite', 'close_ref', 'close_crcl', 'open_ref', 'open_crcl']:
            transitions.append(dict(uk_time=op['uk_time'], ids=[o['id'] for o in group],
                                    us=group[-1]['end_ticks']-op['start_ticks'],
                                    sd_us=sum(o['sd_us'] for o in group),
                                    new_folder=any(o['dir_calls'] for o in group)))
    save_json(out / 'transitions.json', transitions)

    first_stamp = anchor['utc_us'] // 60_000_000 * 60
    last_stamp = int(offset + run['last_seconds']) // 60 * 60
    files, unfinished, orphans = [], [], []
    for folder in sorted(args.card.iterdir()):
        if not folder.is_dir() or not folder.name.isdigit() or not first_stamp//3600*3600 <= int(folder.name) <= last_stamp:
            continue
        for path in sorted(folder.iterdir()):
            match = re.fullmatch(r'aud_(\d+)\.(flac|CRCL)', path.name)
            if not match or not first_stamp <= int(match[1]) <= last_stamp:
                continue
            stamp = int(match[1])
            if match[2] == 'CRCL':
                if not path.with_suffix('.flac').exists():
                    orphans.append(str(path))
                continue
            ref = flac(path)
            companion = path.with_suffix('.CRCL')
            if not companion.exists():
                orphans.append(str(path))
                continue
            if ref['bytes'] == 0 or companion.stat().st_size == 0:
                unfinished.append(dict(stamp=stamp, file=path.name, ref_bytes=ref['bytes'], crcl_bytes=companion.stat().st_size))
                continue
            if ref.get('boot_id') != str(run['boot']):
                continue
            try:
                packed = packed_header_end(companion, ref)
            except (ValueError, KeyError, struct.error) as error:
                packed = dict(errors=[str(error)])
            files.append(dict(stamp=stamp, uk_time=uk((stamp-offset)*1e6), file=path.name, reference=ref, crcl=packed))
    files.sort(key=lambda f:f['stamp'])
    save_json(out / 'files.json', files)
    source_gaps = [dict(file=b['file'], frames=b['crcl']['first_frame'] - a['crcl']['first_frame'] - a['crcl']['frames'])
                   for a,b in zip(files, files[1:]) if 'frames' in a['crcl'] and 'first_frame' in b['crcl']]
    present = {f['stamp'] for f in files} | {f['stamp'] for f in unfinished}
    missing = sorted(set(range(first_stamp, last_stamp+1, 60)) - present)
    maxima, audio_maxima, radio_maxima = {}, {}, {}
    for row in rows:
        if row['message'].startswith(('audio health', 'audio capture', 'audio timing', 'summary2', 'summary active=')):
            for key, value in numeric_fields(row['message']).items():
                if isinstance(value, int):
                    maxima[key] = max(maxima.get(key, 0), abs(value))
                    scoped = audio_maxima if row['source'] == 'Audio' else radio_maxima
                    scoped[key] = max(scoped.get(key, 0), abs(value))
    save_json(out / 'health-and-events.json', rows)
    radio = {k:run[k] for k in ('radio_counts','radio_reasons','radio_errors','radio_states','radio_max','radio_io','radio_samples','radio_totals','radio_id_gaps')}
    radio['observer_late_max'] = run.get('observer_late_max')
    for group in ('radio_max','radio_samples'):
        for r in radio[group].values():
            r['uk_time'] = uk(r['start_ticks'])
    save_json(out / 'radio.json', radio)
    summary = dict(sha256=digest.hexdigest(), boot=run['boot'], records=run['records'],
        boots=[dict(boot=r['boot'], records=r['records'], last_uptime_s=r['last_seconds']) for r in runs],
        start_uk=uk(run['first_seconds']*1e6), audio_start_uk=uk(start['seconds']*1e6), last_uk=uk(run['last_seconds']*1e6),
        complete_pairs=len(files), recorded_s=sum(f['crcl'].get('frames',0) for f in files)/16000,
        first_file=files[0]['file'] if files else None, last_file=files[-1]['file'] if files else None,
        file_errors=[f for f in files if f['crcl']['errors']],
        short_pairs=[f for f in files if f['crcl'].get('frames',0)<950000 or f['crcl'].get('status')!=1],
        statuses=dict(collections.Counter(f['crcl'].get('status') for f in files)),
        joins_checked=len(source_gaps), source_gaps=[g for g in source_gaps if g['frames']!=0],
        missing_minutes=missing, unfinished=unfinished, orphans=orphans,
        sequence_gaps=run['sequence_gaps'], incomplete_log_records=run['incomplete'],
        warnings_errors=[r for r in rows if r['level'] in ('WARN','ERROR')],
        sd_operations=len(ops), sd_ids_contiguous=[o['id'] for o in ops]==list(range(1,len(ops)+1)),
        sd_failed=[o for o in ops if o['ok']!='true'],
        sd_command_errors=sum(io['err'] for o in ops for io in o['io']),
        sd_lost=[r for r in rows if 'diagnostics_lost=' in r['message']], health_maxima=maxima,
        audio_health_maxima=audio_maxima, radio_health_maxima=radio_maxima,
        worst_sd=sorted([{k:v for k,v in o.items() if k!='io'} for o in ops],key=lambda o:o['execution_us'],reverse=True)[:10],
        worst_transitions=sorted(transitions,key=lambda t:t['us'],reverse=True)[:10],
        radio_counts=radio['radio_counts'],radio_errors=radio['radio_errors'],radio_totals=radio['radio_totals'])
    save_json(out / 'summary.json', summary)
    print(json.dumps(summary,indent=2))


if __name__ == '__main__':
    main()
