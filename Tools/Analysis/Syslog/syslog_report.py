#!/usr/bin/env python3
"""Create an offline, interactive Aardwolf syslog report. Source files are read only."""
import argparse
import collections
from datetime import datetime, timezone
import hashlib
import html
import json
import math
from pathlib import Path
import re
import shutil
import sys

from audio_audit import audit

PREFIX = re.compile(r'(\d{10})\s+([\d.]+)\s+(TRACE|DEBUG|INFO|WARN|ERROR)\s+(\w+)\s+')
FIELDS = re.compile(r'\b(\w+)=(Some\([^)]*\)|\[[^\]]*\]|[^\s]+)')
COLORS = ['#65aaff', '#4fd1a5', '#ffb454', '#da93ff', '#ff6b81', '#8bd5ea']
LOCATION_PERIOD_KM = 50.0
COORDINATE_KEYS = {'lat_e7', 'lon_e7', 'lat', 'lon', 'latitude', 'longitude', 'lat_deg', 'lon_deg'}
COORDINATE_TEXT = re.compile(
    r'\b(lat_e7|lon_e7|latitude|longitude|lat_deg|lon_deg|lat|lon)\s*[:=]\s*'
    r'(?:Some\([^)]*\)|[^\s;,}]+)', re.IGNORECASE)


def wrapped_location(latitude, longitude):
    """Web Mercator metres modulo 50 km; no absolute origin is exported."""
    if not (-90 <= latitude <= 90 and -180 <= longitude <= 180):
        return None
    radius = 6378137.0
    latitude = max(-85.05112878, min(85.05112878, latitude))
    east = radius*math.radians(longitude)
    north = radius*math.log(math.tan(math.pi/4+math.radians(latitude)/2))
    period = LOCATION_PERIOD_KM*1000
    return east % period, north % period


def obfuscate_coordinates(message, values):
    """Discard precise coordinate fields before they can reach tables or JSON."""
    coordinates = {k.lower(): v for k,v in values.items() if k.lower() in COORDINATE_KEYS}
    def coordinate(names):
        for name in names:
            value = coordinates.get(name)
            if isinstance(value, (int, float)) and not isinstance(value, bool):
                return value/1e7 if name.endswith('_e7') else value
        return None
    latitude = coordinate(('lat_e7','latitude','lat_deg','lat'))
    longitude = coordinate(('lon_e7','longitude','lon_deg','lon'))
    for key in list(values):
        if key.lower() in COORDINATE_KEYS:
            del values[key]
    if latitude is not None and longitude is not None:
        wrapped = wrapped_location(latitude, longitude)
        if wrapped is not None:
            values['location_x_m'], values['location_y_m'] = wrapped
    return COORDINATE_TEXT.sub(lambda m: m[1]+'=[obfuscated]', message)


def fields(text):
    result = {}
    for key, value in FIELDS.findall(text):
        if value.startswith('Some(') and value.endswith(')'):
            value = value[5:-1]
        if value == 'None': value = None
        elif value == 'true': value = True
        elif value == 'false': value = False
        elif re.fullmatch(r'-?\d+', value): value = int(value)
        elif re.fullmatch(r'-?\d+\.\d+', value): value = float(value)
        result[key] = value
    return result


def parse(path):
    """Handle multiple boots, concatenated records, NUL padding, and partial logs."""
    boots = []
    digest = hashlib.sha256()
    unparsed = 0
    with path.open('rb') as stream:
        for raw in stream:
            digest.update(raw)
            line = raw.decode('utf8', errors='replace')
            matches = list(PREFIX.finditer(line))
            if not matches and line.strip('\0\r\n '): unparsed += 1
            for i, match in enumerate(matches):
                msg = line[match.end():matches[i+1].start() if i+1<len(matches) else len(line)].strip('\0\r\n ')
                seq, seconds, level, source = match.groups()
                seq, seconds = int(seq), float(seconds)
                if msg.startswith('boot node=') or not boots:
                    boots.append(dict(info=fields(msg) if msg.startswith('boot node=') else {},
                        rows=[], sequence_gaps=[], last_sequence=None, anchor=None, anchor_kind=None))
                boot = boots[-1]
                if boot['last_sequence'] is not None and seq != boot['last_sequence']+1:
                    boot['sequence_gaps'].append([boot['last_sequence'], seq])
                boot['last_sequence'] = seq
                f = fields(msg)
                msg = obfuscate_coordinates(msg, f)
                row = dict(seq=seq, t=seconds, level=level, source=source, msg=msg, f=f)
                boot['rows'].append(row)
                # Prefer the actual recording anchor to an inferred scheduled slot.
                if isinstance(f.get('utc_us'), int) and isinstance(f.get('ticks'), int):
                    if boot['anchor'] is None or msg.startswith('audio started '):
                        boot['anchor'] = (f['utc_us']-f['ticks'])/1e6
                        boot['anchor_kind'] = 'audio anchor' if msg.startswith('audio started ') else 'logged UTC anchor'
                if boot['anchor'] is None and isinstance(f.get('slot_utc_us'), int) and isinstance(f.get('slot_ticks'), int):
                    boot['anchor'] = (f['slot_utc_us']-f['slot_ticks'])/1e6
                    boot['anchor_kind'] = 'radio slot anchor'
    return boots, dict(sha256=digest.hexdigest(), unparsed_lines=unparsed, input_bytes=path.stat().st_size)


def measured_relocks(time_rows, minimum_gap=10):
    """Only actual residuals at a NEW accepted PPS after observed holdover.

    The denominator is time since the last *logged* accepted PPS. Sparse log
    publication can miss the first returning edge, so this is an observed
    average divergence, not an oscillator calibration measurement.
    """
    events = []
    previous = None
    for row in time_rows:
        f = row['f']
        if previous:
            old = previous['f']
            tick, old_tick = f.get('last_pps_ticks'), old.get('last_pps_ticks')
            residual = f.get('last_residual_us')
            if (isinstance(tick, int) and isinstance(old_tick, int) and tick > old_tick
                    and isinstance(residual, (int, float))
                    and f.get('accepted', 0) > old.get('accepted', 0)
                    and old.get('holdover_s', 0) >= minimum_gap and f.get('holdover_s', 0) <= 2):
                gap = (tick-old_tick)/1e6
                if gap >= minimum_gap:
                    events.append(dict(t=row['t'], residual_us=residual, gap_s=gap,
                        ppm=residual/gap, accepted=f.get('accepted'), pps_ticks=tick))
        previous = row
    return events


def decimate(rows, limit=3000):
    """Keep first/last and numeric extrema per bucket, rather than hiding spikes."""
    if len(rows) <= limit: return rows
    size = max(1, math.ceil(len(rows)/(limit/6)))
    keep = set()
    for start in range(0, len(rows), size):
        group = rows[start:start+size]
        keep.update((start, start+len(group)-1))
        keys = {key for r in group for key, value in r['f'].items()
                if isinstance(value, (int, float)) and not isinstance(value, bool)}
        for key in keys:
            indices = [i for i,r in enumerate(group) if isinstance(r['f'].get(key), (int,float))]
            if indices:
                keep.add(start+min(indices, key=lambda i: group[i]['f'][key]))
                keep.add(start+max(indices, key=lambda i: group[i]['f'][key]))
    return [rows[i] for i in sorted(keep)]


def plot(title, traces, ytitle='', note='', **layout):
    base = dict(yaxis=dict(title=ytitle))
    base.update(layout)
    return dict(title=title, traces=traces, layout=base, note=note)


def trace(name, x, y, **kwargs):
    base = dict(type='scatter', mode='lines', name=name, x=x, y=y)
    base.update(kwargs)
    return base


def summary_table(headers, rows):
    esc = lambda x: html.escape(str(x))
    return '<div class="table-scroll"><table><thead><tr>'+''.join('<th>'+esc(h)+'</th>' for h in headers)+'</tr></thead><tbody>'+''.join(
        '<tr>'+''.join('<td>'+esc(v)+'</td>' for v in row)+'</tr>' for row in rows)+'</tbody></table></div>'


def bytes_text(value):
    return f'{value/1e9:,.3f} GB' if value >= 1e9 else f'{value/1e6:,.2f} MB'


def analyze(boot, audio_root=None, check_audio=True):
    rows = boot['rows']
    info = boot['info']
    offset = boot['anchor']
    end_t = max(r['t'] for r in rows)
    def x(t):
        return datetime.fromtimestamp(offset+t, timezone.utc).isoformat() if offset is not None else t/3600
    def selected(prefix, source=None):
        return [r for r in rows if r['msg'].startswith(prefix) and (source is None or r['source']==source)]
    def series(data, key, scale=1):
        data = decimate(data)
        return [x(r['t']) for r in data], [r['f'].get(key)/scale if isinstance(r['f'].get(key), (int,float)) else None for r in data]
    def ts(name, data, key, scale=1, **kwargs): return trace(name, *series(data, key, scale), **kwargs)
    sections = []
    alerts = []
    device = selected('versioning device=')
    firmware = selected('versioning firmware=')
    sd = selected('versioning sd_card=')
    card_capacity = sd[0]['f'].get('capacity_bytes') if sd else None
    storage = selected('storage ready ')
    record_config = selected('recording mode=')
    audio_config = selected('audio config ')
    config = record_config[0]['f'] if record_config else {}
    ac = audio_config[0]['f'] if audio_config else {}
    channels = int(config.get('channels', ac.get('channels', 1)))
    rate, bits = int(ac.get('rate_hz', 16000)), int(ac.get('depth_bits', 32))
    health = selected('audio health ')
    audio_start = selected('audio started ')
    last_health = health[-1]['f'] if health else {}
    encoded = last_health.get('encoded_frames', 0)
    backend_bytes = last_health.get('backend_bytes', 0)
    start_utc = audio_start[0]['f'].get('utc_us', 0)/1e6 if audio_start else None
    intervals = []
    active_start = None
    for r in rows:
        if r['msg'].startswith('audio started ') and isinstance(r['f'].get('utc_us'),int):
            active_start = r['f']['utc_us']/1e6
        elif r['msg'].startswith('audio stopped ') and active_start is not None and offset is not None:
            intervals.append((active_start,offset+r['t']))
            active_start = None
    if active_start is not None and offset is not None:
        intervals.append((active_start,offset+end_t))
    file_check = audit(audio_root, info.get('boot'), start_utc,
                       offset+end_t if offset is not None else 0, channels,
                       expected=bool(audio_start),intervals=intervals) if check_audio else dict(available=False, files=[], missing=[])
    files = file_check['files']
    good = [f for f in files if f['complete']]
    bad = [f for f in files if not f['complete']]
    first_ref = next((f['reference'] for f in files if f['reference'].get('device_id')), {})
    identities = [
        ('Device ID', first_ref.get('device_id') or
            (str(device[0]['f']['device']) if device and re.fullmatch(r'[0-9a-fA-F]{8}',str(device[0]['f'].get('device'))) else
             device[0]['msg'] if device else 'Not logged / unavailable')),
        ('Node / boot ID', f"{info.get('node', 'Unknown')} / {info.get('boot', 'Unknown')}"),
        ('Firmware', first_ref.get('firmware_hash') or (firmware[0]['msg'] if firmware else info.get('firmware', 'Unknown'))),
        ('SD identity', sd[0]['msg'] if sd else 'Not logged / unavailable'),
        ('SD capacity', bytes_text(card_capacity)+' (logged SD card)' if isinstance(card_capacity,int) else
            bytes_text(storage[0]['f']['total_bytes'])+' (logged filesystem)' if storage and 'total_bytes' in storage[0]['f'] else
            bytes_text(shutil.disk_usage(audio_root).total)+' (current mounted filesystem; not logged card identity)' if audio_root else 'Not logged / unavailable'),
        ('Period (UTC)', f"{x(0)} to {x(end_t)}" if offset is not None else f'0–{end_t/3600:.2f} hours uptime; no UTC anchor'),
        ('Reset flags', selected('reset ')[0]['msg'] if selected('reset ') else 'Not logged'),
        ('Record / sequence gaps', f"{len(rows):,} / {len(boot['sequence_gaps'])}"),
    ]
    if boot['sequence_gaps']: alerts.append(f"{len(boot['sequence_gaps'])} log sequence gap(s): telemetry may be missing.")
    if not sd: alerts.append('SD identity is absent from this log. Mounted filesystem capacity does not identify the original card.')
    sections.append(dict(title='Device and run', table=summary_table(['Field','Value'], identities)))
    errors = {k: max((abs(r['f'][k]) for r in health if isinstance(r['f'].get(k),(int,float))), default=None)
              for k in ('dropped_scalars','errors','capture_restarts','missing_capture_frames','timestamp_error_ticks')}
    complete_frames = sum(f['frames'] for f in good)
    stored = sum(f['bytes'] for f in good)
    raw = complete_frames*channels*bits/8
    correctness = 'red' if bad or file_check.get('missing') or any(errors.values()) else 'green' if files else 'amber'
    if not audio_start and not health:
        correctness = 'amber'
    audio_summary = [('Recording configuration', record_config[0]['msg'] if record_config else 'Not logged'),
        ('Reference format', audio_config[0]['msg'] if audio_config else 'Not logged; log-counter estimates assume 16 kHz / 32 bits'),
        ('Validation', file_check.get('method', 'Log counters only; recording files not checked')),
        ('Finalized recordings', f'{len(good):,} pairs / {complete_frames/rate/3600:.3f} h' if files else 'Unavailable'),
        ('Incomplete / missing minutes', f"{len(bad)} / {len(file_check.get('missing', []))}" if files else 'Not verified'),
        ('Finalized stored / raw PCM', f'{bytes_text(stored)} / {bytes_text(raw)}' if raw else 'Unavailable'),
        ('Compression vs full-resolution raw', f'{raw/stored:.2f}:1; {100*(1-stored/raw):.1f}% smaller' if raw and stored else 'Unavailable'),
        ('Last encoded audio / bytes', f'{encoded/rate/3600:.3f} h / {bytes_text(backend_bytes)} (log counters, including open files)'),
        ('Log-only compression estimate', f'{encoded*channels*bits/8/backend_bytes:.2f}:1' if encoded and backend_bytes else 'Unavailable'),
        ('Audio error counters', ', '.join(f'{k}={v if v is not None else "unavailable"}' for k,v in errors.items()))]
    if files and encoded > complete_frames:
        audio_summary.append(('Encoded beyond finalized files', f'{(encoded-complete_frames)/rate:.3f} s; includes the open final segment'))
    audio_plots = []
    if files:
        audio_plots.append(plot('Recording completeness', [dict(type='bar',name='Duration', x=[datetime.fromtimestamp(f['stamp'],timezone.utc).isoformat() for f in files],
            y=[f['frames']/rate for f in files], marker=dict(color=['#4fd1a5' if f['complete'] else '#ff6b81' for f in files]),
            customdata=[f['name']+': '+('; '.join(f['errors']) or 'Finalized / aligned') for f in files],
            hovertemplate='%{x}<br>%{customdata}<br>%{y:.4f} s<extra></extra>')], 'Audio seconds',
            'Green: metadata and completion checks passed. Red: incomplete or inconsistent. Payload bits are not decoded. Zero-length files are marked below.'))
        bad_x = [datetime.fromtimestamp(f['stamp'],timezone.utc).isoformat() for f in bad]
        audio_plots[-1]['traces'].append(trace('Incomplete',bad_x,[0]*len(bad_x), mode='markers',marker=dict(color='#ff6b81',size=12,symbol='x')))
    if health:
        audio_plots.append(plot('Audio produced and stored', [ts('Encoded audio (h)',health,'encoded_frames',rate*3600),
            ts('Written bytes (GB)',health,'backend_bytes',1e9,yaxis='y2')], 'Encoded hours',yaxis2=dict(title='GB',overlaying='y',side='right')))
    if bad: alerts.append(f'{len(bad)} incomplete/inconsistent audio file(s), including any open final segment.')
    if file_check.get('missing'): alerts.append(f"{len(file_check['missing'])} minute(s) have no recording pair.")
    issue_rows = [[f['name'], '; '.join(f['errors'])] for f in bad]+[[datetime.fromtimestamp(s,timezone.utc).isoformat(),'Missing minute'] for s in file_check.get('missing',[])]
    sections.append(dict(title='Audio', status=correctness, table=summary_table(['Check','Result'],audio_summary)+
        (summary_table(['File / time','Issue'],issue_rows) if issue_rows else ''), plots=audio_plots))

    times = selected('time status=')
    gps = selected('gps state=')
    locks = [r for r in times if r['f'].get('locked')]
    relocks = measured_relocks(times)
    pps_plots = []
    if gps:
        states = list(dict.fromkeys(r['f'].get('state','Unknown') for r in gps))
        pps_plots.append(plot('GPS acquisition states', [trace('GPS',[x(r['t']) for r in gps],
            [states.index(r['f'].get('state','Unknown')) for r in gps],line=dict(shape='hv'),mode='lines+markers',
            marker=dict(size=5,color=['#4fd1a5' if r['f'].get('state')=='Acquired' else '#a7b7cc' if r['f'].get('state')=='Standby' else '#ffb454' for r in gps]),
            text=[r['msg'] for r in gps],hovertemplate='%{x}<br>%{text}<extra></extra>')],
            yaxis=dict(title='State',tickvals=list(range(len(states))),ticktext=states)))
    if times:
        pps_plots.append(plot('PPS acceptance and frequency lock', [ts('Accepted anchors',times,'accepted'),ts('Rejected anchors',times,'rejected'),
            ts('Frequency calibration locked',times,'locked',yaxis='y2',line=dict(shape='hv'))], 'Anchors',
            'Frequency lock is retained calibration, not proof of fresh PPS or continuous phase accuracy.',
            yaxis2=dict(title='Locked',overlaying='y',side='right',range=[-.05,1.1])))
        pps_plots.append(plot('Time since last accepted PPS', [ts('Holdover (minutes)',times,'holdover_s',60)], 'Minutes'))
    pps_info = [('First frequency lock',str(x(locks[0]['t'])) if locks else 'No lock logged'),
        ('Final GPS counters',gps[-1]['msg'] if gps else 'No GPS acquisition telemetry'),
        ('Actual relock observations',len(relocks))]
    if times: pps_info.append(('Final time state',times[-1]['msg']))
    sections.append(dict(title='PPS acquisition',table=summary_table(['Check','Result'],pps_info), plots=pps_plots))
    drift_plots = []
    if times:
        residual_rows = []
        previous_tick = None
        for r in times:
            tick = r['f'].get('last_pps_ticks')
            if tick is not None and tick != previous_tick and isinstance(r['f'].get('last_residual_us'),(int,float)):
                residual_rows.append(r)
            previous_tick = tick
        drift_plots.append(plot('Logged PPS phase residuals', [ts('PPS residual (ms)',residual_rows,'last_residual_us',1000,mode='markers')], 'Residual (ms)',
            'Only observations with a new accepted PPS timestamp are shown. Stale residuals during holdover and estimated uncertainty growth are excluded.'))
    if relocks:
        xx=[x(r['t']) for r in relocks]
        hover=[f"Gap since last logged PPS: {r['gap_s']/60:.2f} min<br>Residual: {r['residual_us']:.0f} µs" for r in relocks]
        drift_plots.append(plot('Measured divergence when PPS returns', [trace('Observed residual (ms)',xx,[r['residual_us']/1000 for r in relocks],mode='markers',text=hover,marker=dict(size=10)),
            trace('Equivalent divergence (ppm)',xx,[r['ppm'] for r in relocks],mode='markers',text=hover,yaxis='y2',marker=dict(size=10,symbol='diamond'))], 'Relock residual (ms)',
            'Equivalent ppm = signed observed residual (µs) / interval (s) since the last logged accepted PPS. Sparse logging may miss the first returned anchor, and prior phase correction can affect the residual. This is an average divergence proxy, not a hardware frequency measurement. No estimated holdover drift is used.',
            yaxis2=dict(title='Equivalent ppm',overlaying='y',side='right')))
    sections.append(dict(title='Measured PPS drift',plots=drift_plots,
        table=summary_table(['Relock time','Gap (min)','Observed residual (µs)','Equivalent ppm'],
            [[x(e['t']),f"{e['gap_s']/60:.2f}",e['residual_us'],f"{e['ppm']:+.4f}"] for e in relocks]) if relocks else '',
        note='No relock residual observations available.' if not relocks else ''))

    power = selected('summary active=', 'Radio') or selected('energy ')
    battery_plots=[]
    if power:
        battery_plots.append(plot('Battery level and voltage',[ts('SOC (%)',power,'soc'),ts('Battery (V)',power,'battery_mv',1000,yaxis='y2')], 'SOC (%)',
            yaxis2=dict(title='Battery V',overlaying='y',side='right')))
        sources=list(dict.fromkeys(r['f'].get('source','Unknown') for r in power))
        battery_plots.append(plot('Charging and power source',[ts('Charging',power,'charging',line=dict(shape='hv')),
            trace('Source',[x(r['t']) for r in power],[sources.index(r['f'].get('source','Unknown')) for r in power],yaxis='y2',line=dict(shape='hv'))], 'Charging',
            yaxis2=dict(title='Source',overlaying='y',side='right',tickvals=list(range(len(sources))),ticktext=sources)))
        battery_plots.append(plot('External supply voltages',[ts('Solar (V)',power,'solar_mv',1000),ts('External DC (V)',power,'ext_dc_mv',1000)], 'Volts'))
    sections.append(dict(title='Battery and power',plots=battery_plots))

    sensors=[]
    temperatures=selected('die_temperature ')
    if temperatures:
        origins=list(dict.fromkeys(r['f'].get('origin','Unknown') for r in temperatures))
        sensors.append(plot('Sensor die temperatures',[ts(origin,[r for r in temperatures if r['f'].get('origin')==origin],'milli_celsius',1000) for origin in origins], '°C',
            'These are sensor die temperatures, not calibrated ambient temperature.'))
    accel=selected('acceleration ')
    if accel:
        sensors.append(plot('Accelerometer',[ts(a.upper()+' (mg)',accel,a+'_mg') for a in 'xyz'],'mg'))
        tilt=[]
        for r in accel:
            f=r['f']
            if not all(isinstance(f.get(a+'_mg'),(int,float)) for a in 'xyz'): continue
            ax,ay,az=(f[a+'_mg'] for a in 'xyz')
            tilt.append(dict(t=r['t'],f=dict(pitch=math.degrees(math.atan2(-ax,math.hypot(ay,az))),roll=math.degrees(math.atan2(ay,az)))))
        sensors.append(plot('Gravity-based board tilt',[ts('Pitch',tilt,'pitch'),ts('Roll',tilt,'roll')],'Degrees',
            'Board-axis convention: pitch=atan2(−X,√(Y²+Z²)); roll=atan2(Y,Z). Motion contaminates gravity-based tilt.'))
    magnet=selected('magnetic_field ')
    if magnet:
        sensors.append(plot('Magnetometer',[ts(a.upper(),magnet,a+'_nt',1000) for a in 'xyz'],'µT'))
        headings=[dict(t=r['t'],f=dict(heading=math.degrees(math.atan2(r['f']['y_nt'],r['f']['x_nt']))%360)) for r in magnet
                  if isinstance(r['f'].get('x_nt'),(int,float)) and isinstance(r['f'].get('y_nt'),(int,float))]
        sensors.append(plot('Magnetic XY bearing',[ts('Uncalibrated bearing',headings,'heading')],'Degrees',
            'atan2(Y,X), 0–360°. No hard/soft-iron calibration, tilt compensation or magnetic declination correction; this is not a true-north heading.'))
    sections.append(dict(title='Sensors and orientation',plots=sensors))

    locations=selected('location ')
    valid=[r for r in locations if r['f'].get('valid') and 'location_x_m' in r['f']]
    if not valid:
        valid=[r for r in selected('summary2 ') if r['f'].get('location_valid') and 'location_x_m' in r['f']]
    location_plots=[]
    location_table=''
    if valid:
        last=valid[-1]['f']
        location_table=summary_table(['Wrapped X (m)','Wrapped Y (m)','HDOP','Satellites'],[[f"{last['location_x_m']:.1f}",f"{last['location_y_m']:.1f}",
            last.get('hdop')/100 if isinstance(last.get('hdop'),(int,float)) else 'Unknown', last.get('satellites','Unknown')]])
        location_plots.append(plot('Device position (offline coordinate view)',[trace('Obfuscated GPS fixes',[r['f']['location_x_m'] for r in valid],[r['f']['location_y_m'] for r in valid],mode='markers',
            text=[str(x(r['t'])) for r in valid],marker=dict(size=7,color=list(range(len(valid))),colorscale='Viridis',showscale=True,colorbar=dict(title='Fix order')),
            hovertemplate='%{text}<br>Wrapped X %{x:,.1f} m, Y %{y:,.1f} m<extra></extra>')],'Wrapped Y (m)',
            'Web Mercator X and Y in metres, modulo 50,000 m; precise latitude/longitude and the original grid cell are omitted. The period is in projected metres; ground scale varies with latitude. Points can wrap at grid boundaries.',
            xaxis=dict(title='Wrapped X (m)',tickformat=',.0f'),
            yaxis=dict(title='Wrapped Y (m)',tickformat=',.0f',scaleanchor='x',scaleratio=1)))
    sections.append(dict(title='Device location',table=location_table,plots=location_plots,
        note='Location is obfuscated using a 50 km periodic grid. This is simple obfuscation, not anonymization.'))

    radio_plots=[]
    radio_states=selected('radio timing_state ')
    if radio_states:
        modes=list(dict.fromkeys(r['f'].get('mode','Unknown') for r in radio_states))
        plotted_states=decimate(radio_states,limit=8000)
        radio_plots.append(plot('Radio states',[trace('State',[x(r['f'].get('ticks',r['t']*1e6)/1e6) for r in plotted_states],
            [modes.index(r['f'].get('mode','Unknown')) for r in plotted_states],line=dict(shape='hv'),text=[r['f'].get('mode') for r in plotted_states])],
            note=f'{len(radio_states):,} transitions; chart reduced to {len(plotted_states):,} points for responsiveness.' if len(plotted_states)<len(radio_states) else '',
            yaxis=dict(title='Mode',tickvals=list(range(len(modes))),ticktext=modes)))
    else:
        state_summary=[r for r in power if 'radio' in r['f']]
        if state_summary:
            radio_plots.append(plot('Radio states (summary snapshots)',[trace('State',[x(r['t']) for r in state_summary],[r['f']['radio'] for r in state_summary],line=dict(shape='hv'))]))
    profiles={}
    for r in selected('profile='):
        f=r['f']; msg=r['msg']
        profile=f.get('profile')
        freq=re.search(r'frequency_hz:\s*(\d+)',msg)
        sf=re.search(r'spreading_factor:\s*(\w+)',msg)
        bw=re.search(r'bandwidth:\s*(\w+)',msg)
        profiles[profile]=f"P{profile}"+(f" · {int(freq[1])/1e6:g} MHz" if freq else '')+(f" · {sf[1]}/{bw[1]}" if sf and bw else ' · GFSK' if 'Gfsk' in msg else '')
    packets=[]
    profile_by_tx={}
    current_profile=None
    rx_info={}
    for r in rows:
        msg,f=r['msg'],r['f']
        if msg.startswith('minute epoch='):
            current_profile=f.get('profile')
            if f.get('tx_id') is not None: profile_by_tx[f['tx_id']]=current_profile
        elif msg.startswith('tx complete '):
            packets.append(dict(t=r['t'],kind='TX',profile=f.get('profile',profile_by_tx.get(f.get('id'))),**{k:f[k] for k in ('id',) if k in f}))
        elif msg.startswith('rx id='):
            match=re.search(r'epoch_min_slot_pred=Some\(\(\d+,\s*(\d+),',msg)
            rx_info[f.get('id')]=dict(profile=int(match[1]) if match else current_profile, node=f.get('node'))
        elif msg.startswith('rxmeta '):
            meta=rx_info.get(f.get('id'),{})
            packets.append(dict(t=f.get('ticks',r['t']*1e6)/1e6,kind='RX',profile=meta.get('profile'),id=f.get('id'),
                node=meta.get('node'),rssi=f.get('rssi_x2')/2 if isinstance(f.get('rssi_x2'),(int,float)) else None,
                snr=f.get('snr_x4')/4 if isinstance(f.get('snr_x4'),(int,float)) else None))
    # A received packet with no rxmeta still counts once; avoid double-counting
    # normal two-record RX events. Lost metadata does not become a zero RSSI.
    rx_ids={p.get('id') for p in packets if p['kind']=='RX'}
    for r in selected('rx id='):
        f=r['f']
        if f.get('id') not in rx_ids:
            packets.append(dict(t=r['t'],kind='RX',profile=rx_info.get(f.get('id'),{}).get('profile'),id=f.get('id'),rssi=None,snr=None))
    buckets=collections.Counter()
    configs=collections.Counter()
    for p in packets:
        hour=math.floor(((offset or 0)+p['t'])/3600)*3600
        buckets[(hour,p['kind'])]+=1
        configs[(p['profile'],p['kind'])]+=1
    hours=sorted({key[0] for key in buckets})
    if hours:
        radio_plots.append(plot('Packets per hour',[dict(type='bar',name=kind,x=[datetime.fromtimestamp(h,timezone.utc).isoformat() if offset is not None else h/3600 for h in hours],
            y=[buckets[(h,kind)] for h in hours]) for kind in ('TX','RX')],'Packets',barmode='group'))
    profile_keys=sorted({key[0] for key in configs},key=lambda n:(n is None,str(n).zfill(6)))
    if profile_keys:
        radio_plots.append(plot('Packets per radio configuration',[dict(type='bar',name=kind,x=[profiles.get(p,f'P{p}' if p is not None else 'Unknown') for p in profile_keys],
            y=[configs[(p,kind)] for p in profile_keys]) for kind in ('TX','RX')],'Packets',barmode='group'))
        joint=collections.Counter((math.floor(((offset or 0)+p['t'])/3600)*3600,p['profile'],p['kind']) for p in packets)
        for kind in ('TX','RX'):
            if not any(p['kind']==kind for p in packets): continue
            radio_plots.append(plot(kind+' by hour and configuration',[dict(type='heatmap',
                x=[datetime.fromtimestamp(h,timezone.utc).isoformat() if offset is not None else h/3600 for h in hours],
                y=[profiles.get(p,str(p)) for p in profile_keys],z=[[joint[(h,p,kind)] for h in hours] for p in profile_keys],
                colorscale='Blues',hovertemplate='%{x}<br>%{y}: %{z} packets<extra></extra>')],'Configuration'))
    received=[p for p in packets if p['kind']=='RX']
    if any(p.get('rssi') is not None or p.get('snr') is not None for p in received):
        radio_plots.append(plot('Received packet signal quality',[trace('RSSI (dBm)',[x(p['t']) for p in received],[p.get('rssi') for p in received],mode='markers'),
            trace('SNR (dB)',[x(p['t']) for p in received],[p.get('snr') for p in received],mode='markers',yaxis='y2')],'RSSI (dBm)',
            yaxis2=dict(title='SNR (dB)',overlaying='y',side='right')))
    packet_rows=[]
    for p in profile_keys:
        q=[v for v in received if v['profile']==p]
        rssis=[v['rssi'] for v in q if v.get('rssi') is not None]
        snrs=[v['snr'] for v in q if v.get('snr') is not None]
        stats=lambda v: f'{min(v):.1f} / {sum(v)/len(v):.1f} / {max(v):.1f}' if v else 'Unavailable'
        packet_rows.append([profiles.get(p,str(p)),configs[(p,'TX')],configs[(p,'RX')],stats(rssis),stats(snrs)])
    radio_errors=[r for r in selected('radio timing_error ') if 'RxTimeout' not in r['msg']]
    if radio_errors: alerts.append(f'{len(radio_errors)} non-timeout radio driver fault(s); successful recovery can be absent from coarse error counters.')
    radio_info=summary_table(['Configuration','TX','RX','RSSI min/mean/max (dBm)','SNR min/mean/max (dB)'],packet_rows)
    if not received: radio_info+='<p>No received packets in this boot; RSSI/SNR are unavailable.</p>'
    if radio_errors:
        radio_info+=summary_table(['Time','Driver fault'],[[x(r['t']),r['msg']] for r in radio_errors[:500]])
        if len(radio_errors)>500: radio_info+=f'<p>Showing the first 500 of {len(radio_errors):,} driver faults.</p>'
    sections.append(dict(title='Radio',plots=radio_plots,table=radio_info))

    # Operational cost and explicit faults remain visible alongside the requested curves.
    cpu=selected('cpu usage=')
    for r in cpu:
        value=r['f'].get('usage')
        if isinstance(value,str) and value.endswith('%'):
            try: r['f']['usage']=float(value[:-1])
            except ValueError: pass
    timing=selected('audio timing ')
    diagnostics=[]
    if cpu: diagnostics.append(plot('CPU load',[ts('CPU (%)',cpu,'usage')],'Percent'))
    if timing: diagnostics.append(plot('Audio buffering',[ts('Backlog (s)',timing,'backlog_max_samples',rate*channels),
        ts('Source capacity (s)',timing,'source_capacity_samples',rate*channels)],'Seconds'))
    faults=[r for r in rows if r['level'] in ('WARN','ERROR') or r['msg'].startswith(('audio write failed','audio start failed'))]
    sections.append(dict(title='Diagnostics',plots=diagnostics,table=summary_table(['Time','Level','Message'],[[x(r['t']),r['level'],r['msg']] for r in faults[:500]]),
        note=f'{len(faults)} WARN/ERROR or explicit audio fault records. Showing at most 500.' if faults else 'No WARN/ERROR-level records. Recovered radio faults and missing files are reported separately.'))
    return dict(label=f"Boot {info.get('boot','unknown')} · {end_t/3600:.2f} h", boot_id=info.get('boot'),
        sections=sections, alerts=alerts, status=correctness, time_axis='UTC' if offset is not None else 'Uptime (hours)',
        summary=dict(audio_complete=len(good), audio_incomplete=len(bad), missing_minutes=len(file_check.get('missing',[])),
            audio_seconds=complete_frames/rate, audio_encoded_seconds=encoded/rate, audio_stored_bytes=stored,
            audio_errors=errors, first_lock_seconds=locks[0]['t'] if locks else None,
            relocks=relocks, radio_tx=sum(p['kind']=='TX' for p in packets),radio_rx=len(received),
            radio_faults=len(radio_errors),log_sequence_gaps=boot['sequence_gaps']),
        audio_audit=file_check)


def render(runs, metadata, source, output):
    try:
        from plotly.offline import get_plotlyjs
    except ImportError:
        raise SystemExit('Plotly is required: python -m pip install -r Tools/Analysis/Syslog/requirements.txt')
    template=(Path(__file__).parent/'report_template.html').read_text(encoding='utf8')
    # Escape '<' in JSON so untrusted log content cannot terminate a script tag.
    payload=json.dumps(dict(runs=runs, metadata=metadata, source=str(source)),ensure_ascii=True,allow_nan=False).replace('<',chr(92)+'u003c')
    output.write_text(template.replace('/*__PLOTLY__*/',get_plotlyjs()).replace('/*__REPORT_DATA__*/',payload),encoding='utf8')


def main():
    parser=argparse.ArgumentParser(description=__doc__)
    parser.add_argument('syslog',type=Path,help='Aardwolf syslog.txt (may contain multiple boots)')
    parser.add_argument('-o','--output',type=Path,help='HTML output; defaults beside the input log')
    parser.add_argument('--audio-root',type=Path,help='Card root containing numeric hourly directories; auto-detected beside syslog')
    parser.add_argument('--no-audio-check',action='store_true',help='Use log counters only; audio correctness remains unverified')
    parser.add_argument('--json',type=Path,help='Optional machine-readable summary and audit output')
    args=parser.parse_args()
    output=args.output or args.syslog.with_suffix('.report.html')
    if output.resolve()==args.syslog.resolve(): parser.error('Output must not overwrite the input log')
    if args.json and args.json.resolve() in (args.syslog.resolve(),output.resolve()): parser.error('JSON output must be distinct from input and HTML')
    boots,metadata=parse(args.syslog)
    if not boots: parser.error('No supported syslog records found')
    audio_root=args.audio_root
    if audio_root is None and not args.no_audio_check:
        candidate=args.syslog.resolve().parent
        if any(p.is_dir() and re.fullmatch(r'\d{10}',p.name) for p in candidate.iterdir()): audio_root=candidate
    runs=[]
    for boot in boots:
        print(f"Analyzing boot {boot['info'].get('boot','unknown')}: {len(boot['rows']):,} records",flush=True)
        runs.append(analyze(boot,audio_root,not args.no_audio_check))
    output.parent.mkdir(parents=True,exist_ok=True)
    render(runs,metadata,args.syslog,output)
    if args.json:
        args.json.parent.mkdir(parents=True,exist_ok=True)
        args.json.write_text(json.dumps(dict(metadata=metadata,runs=[dict(label=r['label'],summary=r['summary'],audio_audit=r['audio_audit']) for r in runs]),indent=2)+'\n',encoding='utf8')
    print(f'HTML: {output.resolve()} ({output.stat().st_size/1e6:.2f} MB; Plotly embedded, no CDN required)')
    for r in runs:
        s=r['summary']
        print(f"{r['label']}: {s['audio_complete']} finalized, {s['audio_incomplete']} incomplete, "
              f"{s['missing_minutes']} missing minutes; TX/RX {s['radio_tx']}/{s['radio_rx']}; "
              f"{len(s['relocks'])} observed PPS returns; {s['radio_faults']} radio faults")


if __name__=='__main__': main()
