# Firmware images and build records

Build and archive the latest Aardwolf image without connecting a board:

```powershell
rtk proxy powershell -NoProfile -ExecutionPolicy Bypass -File scripts/firmware.ps1 -Package aardwolf -BuildOnly
```

Each build creates a new directory below `<package>/`, named with its UTC build
time, source commit, binary, profile and ELF hash. Earlier images are never
overwritten. Commit the whole directory along with the updated `<package>.json`
latest-build record. No rebuild or source rollback is needed to redistribute
an archived image. `-NoBuild` continues to reuse the existing target artifact
without creating a new archive or inventing its provenance.

Each archive contains:

- `<binary>.elf`: the exact linked image, including load addresses and debugging information.
- `<binary>.bin`: the raw flash image, checked byte for byte against every ELF load segment.
- `build.json`: firmware/source identity, board, target MCU, flash address, both
  image hashes and sizes, Cargo arguments, toolchain versions and build environment.
- `Cargo.lock`: the dependency resolution used by this build, with its hash in the record.
- `source.patch` and `source-untracked.zip` when needed: tracked changes relative
  to the recorded commit and untracked, nonignored source files, each hashed in
  the record. Generated `firmware-builds` output is excluded from source dirtiness.
- `FLASHING.txt`: the image's flash address and STM32CubeProgrammer instructions.

The archive and latest JSON share the same record. When a flashing session
reports a runtime CRC32, both records are updated; the images remain unchanged.
The legacy `artifact_sha256` field remains the ELF hash. Historical schema-1
records have no saved images; their missing binaries cannot be recreated from
the hash alone. VS Code firmware builds and Run actions use the same archiver.

Install the conversion tools once if they are missing:

```powershell
rtk cargo install cargo-binutils
rtk proxy rustup component add llvm-tools-preview
```

For USB DFU, enter the STM32 system bootloader using the board's BOOT0/reset
procedure, connect over USB in STM32CubeProgrammer, and select **Erasing &
Programming**. Open the archived BIN, enter the `flash_address` from its
`build.json` / `FLASHING.txt`, enable verification and program it. The normal
Raylar v1.0 nonsecure Aardwolf image starts at **0x08000000**. The ELF carries
its own addresses and can also be opened by CubeProgrammer. Leave bootloader
mode and reset to start the application. This package is an application image;
it does not install a DFU bootloader or change security/boot option bytes.

To reproduce a build, check out its `source_git` commit (without `-dirty`), apply
`source.patch` and extract `source-untracked.zip` if present, restore the archived
`Cargo.lock` at the repository root, and use the recorded Rust toolchain, Cargo
arguments, build environment and `RAYLAR_*` metadata from `build.json`. Use
`--locked` to keep dependency resolution fixed. These records support rebuilding;
bit-identical results across different hosts/toolchains are not guaranteed. For
the exact distributed bytes, use the archived ELF/BIN and verify their SHA-256.

The host packaging checks need an existing Aardwolf target ELF, but no board:

```powershell
rtk proxy powershell -NoProfile -ExecutionPolicy Bypass -File scripts/test_firmware_artifacts.ps1
```
