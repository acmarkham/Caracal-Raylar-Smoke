"""Compare generated native FLAC files with two independent integer decoders."""
import argparse
from array import array
from pathlib import Path
import subprocess


def run(command):
    result = subprocess.run(command, capture_output=True, check=False)
    if result.returncode or b"ERROR" in result.stderr:
        raise AssertionError((command, result.stderr.decode(errors="replace")))
    return result


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("folder", type=Path)
    parser.add_argument("--flac", required=True)
    parser.add_argument("--ffmpeg", default="ffmpeg")
    args = parser.parse_args()
    files = sorted(args.folder.glob("*.flac"))
    assert files, "No fixtures found"
    for path in files:
        stem = path.stem.replace("_stream", "")
        depth = int(stem.split("_")[-2])
        expected = path.with_name(stem + ".s32le").read_bytes()
        ffmpeg = run([args.ffmpeg, "-v", "error", "-i", str(path),
                      "-f", "s32le", "-acodec", "pcm_s32le", "-"])
        assert not ffmpeg.stderr and ffmpeg.stdout == expected, (path, "FFmpeg PCM mismatch")
        run([args.flac, "--test", "--silent", str(path)])
        decoded = run([args.flac, "--decode", "--stdout", "--silent", "--force-raw-format",
                       "--endian=little", "--sign=signed", str(path)]).stdout
        width = depth // 8
        assert len(decoded) // width == len(expected) // 4, (path, "sample count")
        values = array("i", (int.from_bytes(decoded[i:i+width], "little", signed=True) << (32-depth)
                             for i in range(0, len(decoded), width)))
        assert values.tobytes() == expected, (path, "Xiph PCM mismatch")
    print(f"FFmpeg and Xiph FLAC verified {len(files)} exact PCM round trips.")


if __name__ == "__main__":
    main()
