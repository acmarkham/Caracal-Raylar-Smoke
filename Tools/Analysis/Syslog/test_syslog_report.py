"""Small synthetic regressions; no SD card or firmware is required."""
import json
import math
from pathlib import Path
import re
import struct
import tempfile
import unittest
from html.parser import HTMLParser

from audio_audit import audit, reference_header
from syslog_report import analyze, fields, measured_relocks, parse, render, wrapped_location


def recording(root, stamp, first=0, frames=960000, status=1):
    folder=root/str(stamp//3600*3600)
    folder.mkdir(exist_ok=True)
    path=folder/f'aud_{stamp}.flac'
    streaminfo=bytearray(34)
    struct.pack_into('>Q',streaminfo,10,(16000 << 44) | (31 << 36) | frames)
    comment=f'boot_id=7;first_source_frame={first};segment_id={bytes(16).hex()};device_id=test;'
    data=b'fLaC'+bytes([0])+len(streaminfo).to_bytes(3,'big')+streaminfo
    data+=bytes([0x84])+len(comment).to_bytes(3,'big')+comment.encode()
    path.write_bytes(data)
    header=bytearray(92)
    header[:8]=b'CRCL\r\n\x1a\n'
    struct.pack_into('<HH',header,8,2,92)
    struct.pack_into('<I',header,32,16000)
    struct.pack_into('<H',header,40,1024)
    struct.pack_into('<Q',header,44,first)
    struct.pack_into('<Q',header,60,960000)
    end=bytearray(52)
    end[:5]=b'CRBK\x03'
    struct.pack_into('<HI',end,6,28,52)
    struct.pack_into('<Q',end,16,frames)
    struct.pack_into('<QQII',end,28,frames,len(data),(frames+1023)//1024,status)
    path.with_suffix('.CRCL').write_bytes(header+end)
    return path


class ReportTests(unittest.TestCase):
    def test_location_wraps_with_50_km_projected_period(self):
        latitude, longitude = 51.2345678, -1.3456789
        x, y = wrapped_location(latitude, longitude)
        self.assertTrue(0 <= x < 50000 and 0 <= y < 50000)
        self.assertAlmostEqual(x, (6378137*math.radians(longitude)) % 50000, places=6)
        north = 6378137*math.log(math.tan(math.pi/4+math.radians(latitude)/2))
        shifted_lat = math.degrees(2*math.atan(math.exp((north+50000)/6378137))-math.pi/2)
        shifted_lon = longitude+math.degrees(50000/6378137)
        shifted = wrapped_location(shifted_lat, shifted_lon)
        self.assertAlmostEqual(x, shifted[0], places=7)
        self.assertAlmostEqual(y, shifted[1], places=7)

    def test_precise_coordinates_do_not_reach_report_payload_or_fault_text(self):
        with tempfile.TemporaryDirectory() as tmp:
            path = Path(tmp)/'log.txt'
            path.write_text('0000000000 0.000 INFO Aardwolf boot node=4 boot=7\n'
                '0000000001 1.000 INFO Location location valid=true lat_e7=512345678 lon_e7=-13456789\n'
                '0000000002 2.000 WARN Location location invalid latitude=51.2345678 longitude=-1.3456789\n')
            boots,meta = parse(path)
            result = analyze(boots[0],check_audio=False)
            serialized = json.dumps(result)
            for precise in ('512345678','-13456789','51.2345678','-1.3456789'):
                self.assertNotIn(precise, serialized)
            self.assertNotIn('lat_e7', boots[0]['rows'][1]['f'])
            self.assertIn('latitude=[obfuscated]', boots[0]['rows'][2]['msg'])
            section=next(s for s in result['sections'] if s['title']=='Device location')
            self.assertIn('Wrapped X',section['table'])
            output=Path(tmp)/'report.html'
            render([result],meta,path,output)
            payload=re.search(r'<script id="report-data" type="application/json">(.*?)</script>',output.read_text(encoding='utf8'),re.S)[1]
            self.assertFalse(any(value in payload for value in ('512345678','-13456789','51.2345678','-1.3456789')))

    def test_fields_optional_values_and_signed_measurements(self):
        self.assertEqual(fields('soc=Some(26) hdop=None locked=true residual=Some(-245) v=1.25'),
                         dict(soc=26,hdop=None,locked=True,residual=-245,v=1.25))

    def test_parser_multiple_boots_concatenated_lines_and_padding(self):
        with tempfile.TemporaryDirectory() as tmp:
            p=Path(tmp)/'log.txt'
            p.write_bytes(b'0000000000 0.000 INFO Aardwolf boot node=4 boot=7 firmware=v1\n'
                b'0000000001 1.000 INFO Time time status=Synchronized utc_us=10000000 ticks=1000000'
                b'0000000003 3.000 INFO Gps gps state=Standby\0\n'
                b'0000000000 0.000 INFO Aardwolf boot node=4 boot=8\n')
            boots,meta=parse(p)
            self.assertEqual(len(boots),2)
            self.assertEqual(boots[0]['anchor'],9)
            self.assertEqual(boots[0]['sequence_gaps'],[[1,3]])
            self.assertEqual(meta['input_bytes'],p.stat().st_size)

    def test_observed_relock_drift_excludes_uncertainty_and_stale_residuals(self):
        rows=[dict(t=0,f=dict(last_pps_ticks=1000000,accepted=1,last_residual_us=100,holdover_s=0)),
              dict(t=1800,f=dict(last_pps_ticks=1000000,accepted=1,last_residual_us=100,holdover_s=1800,uncertainty_us=999999999)),
              dict(t=1801,f=dict(last_pps_ticks=1801000000,accepted=2,last_residual_us=-900,holdover_s=0)),
              dict(t=1802,f=dict(last_pps_ticks=1802000000,accepted=3,last_residual_us=-880,holdover_s=0))]
        events=measured_relocks(rows)
        self.assertEqual(len(events),1)
        self.assertEqual(events[0]['residual_us'],-900)
        self.assertAlmostEqual(events[0]['ppm'],-.5)

    def test_pps_no_data_is_not_zero_error(self):
        self.assertEqual(measured_relocks([dict(t=100,f=dict(holdover_s=100)),dict(t=200,f=dict(holdover_s=0))]),[])

    def test_audio_missing_companion_and_open_tail_are_red(self):
        with tempfile.TemporaryDirectory() as tmp:
            root=Path(tmp)
            first=recording(root,1800000000)
            last=recording(root,1800000060,first=960000)
            last.write_bytes(b'')
            last.with_suffix('.CRCL').write_bytes(b'')
            result=audit(root,7,1800000000,1800000110,6)
            self.assertTrue(result['files'][0]['complete'])
            self.assertFalse(result['files'][1]['complete'])
            first.with_suffix('.CRCL').unlink()
            self.assertIn('Missing CRCL companion',audit(root,7,1800000000,1800000050,6)['files'][0]['errors'])

    def test_audio_missing_minute_and_frame_gap(self):
        with tempfile.TemporaryDirectory() as tmp:
            root=Path(tmp)
            recording(root,1800000000)
            recording(root,1800000120,first=1920000)
            result=audit(root,7,1800000000,1800000170,6)
            self.assertEqual(result['missing'],[1800000060])
            self.assertIn('discontinuity',' '.join(result['files'][1]['errors']))

    def test_short_finalized_file_is_reported(self):
        with tempfile.TemporaryDirectory() as tmp:
            root=Path(tmp)
            recording(root,1800000000,frames=16000)
            result=audit(root,7,1800000000,1800000050,6)
            self.assertIn('Short/long segment',' '.join(result['files'][0]['errors']))

    def test_audio_wrong_boot_not_attributed_to_current_run(self):
        with tempfile.TemporaryDirectory() as tmp:
            root=Path(tmp)
            recording(root,1800000000)
            self.assertFalse(audit(root,8,1800000000,1800000050,6)['available'])

    def test_intentional_recording_pause_does_not_invent_missing_audio(self):
        with tempfile.TemporaryDirectory() as tmp:
            root=Path(tmp)
            recording(root,1800000000)
            recording(root,1800000600,first=9000000)
            result=audit(root,7,1800000000,1800000650,6,
                         intervals=[(1800000000,1800000059),(1800000600,1800000650)])
            self.assertEqual(result['missing'],[])
            self.assertTrue(all(r['complete'] for r in result['files']))

    def test_rx_tx_matching_signal_units_and_no_double_counting(self):
        with tempfile.TemporaryDirectory() as tmp:
            p=Path(tmp)/'log.txt'
            messages=['boot node=4 boot=7 firmware=v1','minute epoch=1 profile=2 tx_id=Some(3)',
                'tx complete id=3 profile=2','rx id=4 node=8 epoch_min_slot_pred=Some((1, 2, 0, Some(0)))',
                'rxmeta id=4 ticks=4000000 rssi_x2=-140 snr_x4=Some(28)']
            p.write_text('\n'.join(f'{i:010d} {i}.000 INFO Aardwolf {msg}' for i,msg in enumerate(messages)))
            boots,_=parse(p)
            result=analyze(boots[0],check_audio=False)
            self.assertEqual(result['summary']['radio_tx'],1)
            self.assertEqual(result['summary']['radio_rx'],1)
            radio=next(s for s in result['sections'] if s['title']=='Radio')
            self.assertIn('-70.0',radio['table'])
            self.assertIn('7.0',radio['table'])

    def test_render_embeds_plotly_and_escapes_script_termination(self):
        with tempfile.TemporaryDirectory() as tmp:
            output=Path(tmp)/'report.html'
            render([dict(label='</script><script>bad()</script>')],{},Path('sample.log'),output)
            text=output.read_text(encoding='utf8')
            payload=re.search(r'<script id="report-data" type="application/json">(.*?)</script>',text,re.S)[1]
            self.assertEqual(json.loads(payload)['runs'][0]['label'],'</script><script>bad()</script>')
            self.assertNotIn('</script>',payload)
            class Tags(HTMLParser):
                scripts=[]
                def handle_starttag(self, tag, attrs):
                    if tag=='script': self.scripts.append(dict(attrs))
            tags=Tags()
            tags.feed(text)
            self.assertFalse(any('src' in attrs for attrs in tags.scripts))


if __name__=='__main__': unittest.main()
