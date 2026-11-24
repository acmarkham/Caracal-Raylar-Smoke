#!/usr/bin/env python3
"""Build an offline overview of a folder of Aardwolf syslogs."""
import argparse
from collections import defaultdict
from datetime import datetime, timezone
import json
import math
import os
from pathlib import Path
import re
from statistics import median
from urllib.parse import quote

from syslog_report import PREFIX, fields, wrapped_location, decimate


def read_log(path, root):
    """Stream large logs, retaining only overview telemetry and packet records."""
    runs = []
    run = None
    with path.open('rb') as stream:
        for raw in stream:
            line = raw.decode('utf8', errors='replace')
            matches = list(PREFIX.finditer(line))
            for i, match in enumerate(matches):
                msg = line[match.end():matches[i+1].start() if i+1 < len(matches) else len(line)].strip('\0\r\n ')
                t = float(match[2])
                if run is None or msg.startswith('boot node='):
                    run = dict(info=fields(msg) if msg.startswith('boot node=') else {}, anchor=None,
                               start=t, end=t, power=[], tilt=[], heading=[], positions=[], profiles={}, rx=[], pending={}, current=None)
                    runs.append(run)
                run['end'] = max(run['end'], t)
                if not msg.startswith(('boot node=', 'audio started ', 'minute epoch=', 'profile=', 'rx id=', 'rxmeta ', 'summary ', 'summary2 ', 'location ', 'acceleration ', 'magnetic_field ')):
                    continue
                f = fields(msg)
                if isinstance(f.get('utc_us'), int) and isinstance(f.get('ticks'), int):
                    if run['anchor'] is None or msg.startswith('audio started '):
                        run['anchor'] = (f['utc_us']-f['ticks'])/1e6
                if run['anchor'] is None and isinstance(f.get('slot_utc_us'), int) and isinstance(f.get('slot_ticks'), int):
                    run['anchor'] = (f['slot_utc_us']-f['slot_ticks'])/1e6
                if msg.startswith('profile='):
                    # Keep all channel parameters so different configurations never merge by ID alone.
                    run['profiles'][f.get('profile')] = msg.split('configuration=', 1)[-1]
                elif msg.startswith('minute epoch='):
                    run['current'] = f.get('profile')
                elif msg.startswith('rx id='):
                    prediction = re.search(r'epoch_min_slot_pred=Some\(\(\d+,\s*(\d+),', msg)
                    packet = dict(t=t, sender=str(f.get('node', 'unknown')), profile=int(prediction[1]) if prediction else run['current'], rssi=None, snr=None)
                    run['rx'].append(packet)
                    run['pending'][f.get('id')] = packet
                elif msg.startswith('rxmeta '):
                    packet = run['pending'].pop(f.get('id'), None)
                    if packet is not None:
                        packet.update(rssi=f['rssi_x2']/2 if isinstance(f.get('rssi_x2'), (int, float)) else None,
                                      snr=f['snr_x4']/4 if isinstance(f.get('snr_x4'), (int, float)) else None)
                elif msg.startswith('summary '):
                    run['power'].append(dict(t=t, f={k:f.get(k) for k in ('soc', 'battery_mv', 'charging', 'source')}))
                elif msg.startswith(('location ', 'summary2 ')) and (f.get('valid') or f.get('location_valid')):
                    if all(isinstance(f.get(k), (int, float)) for k in ('lat_e7', 'lon_e7')):
                        pos = wrapped_location(f['lat_e7']/1e7, f['lon_e7']/1e7)
                        if pos is not None:
                            run['positions'].append(pos)
                elif msg.startswith('acceleration ') and all(isinstance(f.get(k+'_mg'), (int, float)) for k in 'xyz'):
                    x,y,z = (f[k+'_mg'] for k in 'xyz')
                    if x*x+y*y+z*z:
                        run['tilt'].append(dict(t=t, f=dict(pitch=math.degrees(math.atan2(-x, math.hypot(y,z))), roll=math.degrees(math.atan2(y,z)))))
                elif msg.startswith('magnetic_field ') and all(isinstance(f.get(k+'_nt'), (int, float)) for k in 'xy'):
                    x,y = f['x_nt'],f['y_nt']
                    if x or y:
                        run['heading'].append(dict(t=t, f=dict(heading=math.degrees(math.atan2(y,x))%360)))
    result = []
    for index, run in enumerate(runs):
        node = str(run['info'].get('node', 'unknown'))
        label = str(path.parent.relative_to(root)) or '.'
        def stamp(t):
            return datetime.fromtimestamp(run['anchor']+t, timezone.utc).isoformat() if run['anchor'] is not None else None
        series = {}
        for key in ('power', 'tilt', 'heading'):
            series[key] = [dict(time=stamp(r['t']), hours=r['t']/3600, **r['f']) for r in decimate(run[key], 2000)]
        positions = run['positions']
        # Unwrap around this run's first fix before taking a robust deployment centre.
        position = [median(p[a] + round((positions[0][a]-p[a])/50000)*50000 for p in positions) % 50000 for a in (0,1)] if positions else None
        packets = []
        for p in run['rx']:
            profile = run['profiles'].get(p['profile'], 'Unknown configuration')
            packets.append(dict(**p, config=profile))
        result.append(dict(node=node, label=label, boot=run['info'].get('boot', index), start=stamp(run['start']), end=stamp(run['end']),
                           duration_h=(run['end']-run['start'])/3600, position=position, fixes=len(positions), series=series, packets=packets,
                           power_first=run['power'][0]['f'] if run['power'] else {}, power_last=run['power'][-1]['f'] if run['power'] else {},
                           orientation={**(run['tilt'][-1]['f'] if run['tilt'] else {}), **(run['heading'][-1]['f'] if run['heading'] else {})},
                           source=str(path.relative_to(root)), report=str(path.with_suffix('.report.html').relative_to(root)), report_exists=path.with_suffix('.report.html').is_file()))
    return result


def build_data(root, output):
    runs = []
    for path in sorted(root.rglob('syslog.txt')):
        print(f'Reading {path.relative_to(root)}', flush=True)
        runs.extend(read_log(path, root))
    if not runs:
        raise ValueError('No supported syslog records found')
    origin = next((r['position'] for r in runs if r['position']), None)
    links = defaultdict(lambda: dict(count=0, rssi=[], snr=[]))
    for run in runs:
        if run['position'] and origin:
            run['position'] = [(v-o+25000)%50000-25000 for v,o in zip(run['position'],origin)]
        for key in ('report', 'source'):
            run[key] = quote(Path(os.path.relpath(root/run[key], output.parent)).as_posix(), safe='/')
        for packet in run.pop('packets'):
            link = links[(packet['sender'], run['node'], packet['config'])]
            link['count'] += 1
            for key in ('rssi', 'snr'):
                if packet[key] is not None:
                    link[key].append(packet[key])
    aggregated = []
    for (sender, receiver, config), values in sorted(links.items()):
        row = dict(sender=sender, receiver=receiver, config=config, count=values['count'])
        for key in ('rssi', 'snr'):
            samples = values[key]
            row[key] = dict(n=len(samples), mean=sum(samples)/len(samples), min=min(samples), max=max(samples)) if samples else None
        aggregated.append(row)
    return dict(runs=runs, links=aggregated, folder=root.name)


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('folder', type=Path)
    parser.add_argument('-o', '--output', type=Path)
    args = parser.parse_args()
    root = args.folder.resolve()
    output = (args.output or root/'batch.report.html').resolve()
    if output.suffix.lower() != '.html':
        parser.error('Output must be an HTML file')
    data = build_data(root, output)
    from plotly.offline import get_plotlyjs
    template = Path(__file__).with_name('batch_template.html').read_text(encoding='utf8')
    payload = json.dumps(data, ensure_ascii=True, allow_nan=False).replace('<', '\\u003c')
    output.parent.mkdir(parents=True, exist_ok=True)
    output.write_text(template.replace('/*__PLOTLY__*/', get_plotlyjs()).replace('/*__DATA__*/', payload), encoding='utf8')
    print(f'HTML: {output} ({len(data["runs"])} runs, {len(data["links"])} directional link/configuration groups)', flush=True)


if __name__ == '__main__':
    main()
