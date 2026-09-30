# Firmware build records

`scripts/firmware.ps1` writes one JSON record per built firmware package here.
Commit the record for firmware used in an overnight or field test. It contains
the same version, Git identity, build timestamp, profile, and board revision
embedded in the firmware, plus the SHA-256 of the flashed ELF. After the target
reports its runtime CRC32, the script adds that value to the record as well.

A `-dirty` suffix on `source_git` and `source_tree_dirty: true` mean the image
was built with tracked changes that are not represented by that Git commit.
For fully reproducible test firmware, build from a clean tree.
