import tempfile
import unittest
from pathlib import Path

from batch_report import build_data, read_log


class BatchReportTests(unittest.TestCase):
    def test_packet_metadata_boots_missing_values_and_coordinates(self):
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            node = root/'W1'
            node.mkdir()
            path = node/'syslog.txt'
            messages = [
                'boot node=11 boot=1',
                'profile=1 configuration=ChannelProfile { frequency_hz: 868100000 }',
                'minute epoch=1 profile=1 slot_utc_us=1700000001000000 slot_ticks=1000000',
                'rx id=4 node=22 epoch_min_slot_pred=Some((1, 1, 4, Some(4)))',
                'rxmeta id=4 rssi_x2=-213 snr_x4=Some(-33)',
                'rx id=5 node=22 epoch_min_slot_pred=Some((1, 1, 4, Some(4)))',
                'summary soc=Some(90) battery_mv=4000 charging=false source=Battery',
                'location valid=true lat_e7=517776566 lon_e7=-13398200',
                'acceleration x_mg=0 y_mg=0 z_mg=1000',
                'magnetic_field x_nt=0 y_nt=100 z_nt=0',
                'boot node=11 boot=2',
                'summary soc=None battery_mv=3900',
            ]
            path.write_text('\n'.join(f'{i:010} {i:.3f} INFO Aardwolf {m}' for i,m in enumerate(messages)))
            runs = read_log(path, root)
            self.assertEqual(len(runs), 2)
            self.assertEqual(runs[0]['orientation'], dict(pitch=0, roll=0, heading=90))
            self.assertIsNone(runs[1]['start'])
            data = build_data(root, root/'overview.html')
            self.assertEqual(data['runs'][0]['position'], [0, 0])
            self.assertEqual(data['runs'][0]['report'], 'W1/syslog.report.html')
            link = data['links'][0]
            self.assertEqual((link['sender'],link['receiver'],link['count']), ('22','11',2))
            self.assertEqual(link['rssi']['mean'], -106.5)
            self.assertEqual(link['snr']['mean'], -8.25)
            self.assertEqual(link['snr']['n'], 1)
            self.assertNotIn('lat_e7', str(data))

    def test_empty_folder(self):
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            with self.assertRaisesRegex(ValueError, 'No supported'):
                build_data(root, root/'batch.html')

    def test_common_map_crosses_wrapping_boundary(self):
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            for label, node, lon in [('W1', 11, -1000), ('W2', 22, 1000)]:
                folder = root/label
                folder.mkdir()
                (folder/'syslog.txt').write_text(
                    f'0000000000 0.000 INFO Aardwolf boot node={node} boot=1\n'
                    f'0000000001 1.000 INFO Location location valid=true lat_e7=0 lon_e7={lon}\n')
            data = build_data(root, root/'batch.html')
            self.assertAlmostEqual(data['runs'][1]['position'][0], 22.2639, places=3)
            self.assertEqual(data['runs'][1]['position'][1], 0)


if __name__ == '__main__':
    unittest.main()
