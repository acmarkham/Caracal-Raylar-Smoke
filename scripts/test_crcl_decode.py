"""Run after generating fixtures with RAYLAR_CRCL_TEST_OUTPUT set to an absolute path."""
import copy
from pathlib import Path
import unittest

from crcl_decode import FormatError, crc32c, decode, reference

FIXTURES = Path(__file__).resolve().parent.parent / "target" / "crcl-fixtures"


class DecoderTests(unittest.TestCase):
    def setUp(self):
        self.data = (FIXTURES / "paired-wav-0.CRCL").read_bytes()
        self.ref = reference(FIXTURES / "paired-wav-0.wav")

    def test_wav_and_flac_pairs_and_reference_pcm(self):
        for kind in ("wav", "flac"):
            for segment, count in ((0, 16000), (1, 13)):
                stem = FIXTURES / f"paired-{kind}-{segment}"
                ref = reference(stem.with_suffix("." + kind))
                self.assertEqual(ref["raw"], stem.with_suffix(".s32le").read_bytes())
                decoded, report = decode(stem.with_suffix(".CRCL").read_bytes(), ref)
                self.assertTrue(report["digest_verified"])
                self.assertEqual(report["frames"], count)
                self.assertEqual(decoded[::6], ref["pcm"])
                for n in range(count):
                    for c in range(1, 6):
                        self.assertEqual(decoded[n * 6 + c], ref["pcm"][n] + c * 157)

    def test_known_crc(self):
        self.assertEqual(crc32c(b"123456789"), 0xE3069283)

    def test_truncation_and_corruption_fail_with_explicit_recovery(self):
        for cut in (1, 4, 40, 91, 92):
            data = self.data[:-cut]
            with self.assertRaises(FormatError):
                decode(data, self.ref)
            _, report = decode(data, self.ref, recover=True)
            self.assertFalse(report["complete"])
            self.assertFalse(report["digest_verified"])
            self.assertIsNotNone(report["recovery_error"])
        header = int.from_bytes(self.data[10:12], "little")
        for position in (header + 8, header + 12, header + 32, header + 92):
            corrupt = bytearray(self.data)
            corrupt[position] ^= 1
            with self.assertRaises(FormatError):
                decode(corrupt, self.ref)
        with self.assertRaises(FormatError):
            decode(self.data + b"extra", self.ref)

    def test_wrong_reference_samples_are_rejected(self):
        wrong = copy.deepcopy(self.ref)
        wrong["raw"] = b"\1" + wrong["raw"][1:]
        with self.assertRaisesRegex(FormatError, "digest"):
            decode(self.data, wrong)
        wrong = copy.deepcopy(self.ref)
        wrong["metadata"] = wrong["metadata"].replace(b"segment_id=", b"changed_id=")
        with self.assertRaisesRegex(FormatError, "identity"):
            decode(self.data, wrong)


if __name__ == "__main__":
    unittest.main()
