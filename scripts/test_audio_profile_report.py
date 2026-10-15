"""Checks that interrupted or short target runs cannot become successful reports."""
import unittest
from audio_profile_report import parse, report

STAGES = ("recorder_poll capture_check capture_publish source_copy spatial_transform "
          "spatial_delta_sample quantize_fast_sample quantize_wide_sample rice_stats_sample rice_select "
          "crcl_serialize crcl_pack crcl_crc reference_feed reference_sha_compress flac_analysis "
          "flac_encode storage_buffer filesystem_append filesystem_finalize sd_read sd_write").split()


def fixture():
    lines = ["PROFILE config core_hz=144000000 empty_span_cycles=2 probe_overhead_cycles=120"]
    for window in [1, 2]:
        lines.append(f"PROFILE window={window} frames=960000 elapsed_us=60000000 cycle_budget=8640000000 recorder_cycles=5000000000 capture_cycles=1000000")
        for stage in STAGES:
            sampled = str(stage.endswith("_sample")).lower()
            lines.append(f"PROFILE stage={stage} calls=100 cycles=1200 max_cycles=20 wall_us=0 bytes=0 sampled={sampled}")
    lines.append("CRCL PROFILE TARGET PASS")
    return "\n".join(lines)


class ReportChecks(unittest.TestCase):
    def test_completed_run_preserves_raw_inclusive_counters(self):
        run = parse(fixture())
        self.assertEqual(len(run["windows"]), 2)
        self.assertEqual(run["windows"][0]["stages"][4]["cycles"], "1200")
        self.assertIn("(estimated)", report(run, "fixture.log"))

    def test_interrupted_or_short_capture_is_rejected(self):
        for text in [fixture().replace("CRCL PROFILE TARGET PASS", ""),
                     fixture().replace("frames=960000", "frames=959999", 1),
                     fixture().replace("stage=sd_write", "stage=unexpected")]:
            with self.subTest(text=text[-60:]):
                with self.assertRaises(ValueError):
                    parse(text)

    def test_compiled_control_requires_its_own_pass_and_one_window(self):
        text = fixture().split("PROFILE window=2")[0] + "CRCL PROFILE CONTROL PASS"
        self.assertEqual(len(parse(text, compiled_control=True)["windows"]), 1)
        with self.assertRaises(ValueError):
            parse(text)

    def test_control_from_another_codec_is_not_an_overhead_comparison(self):
        run = parse(fixture())
        text = fixture().split("PROFILE window=2")[0] + "CRCL PROFILE CONTROL PASS"
        control = parse(text, compiled_control=True)
        control["config"]["codec"] = "temporal_shift_v2"
        with self.assertRaisesRegex(ValueError, "same codec"):
            report(run, "fixture.log", control)


if __name__ == "__main__":
    unittest.main()
