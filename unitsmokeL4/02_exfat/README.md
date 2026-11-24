# Evo L4 SDMMC/DMA and exFAT smoke

Implements [ADR-L4-007](../../SpecAndDesign/Platform/L4/ADR-L4-007-sdmmc-dma-exfat.md).
Uses SDMMC1 four-bit mode, DMA2 channel 5/request 7 and a 48 MHz PLLSAI1 kernel
clock with a requested 24 MHz card bus. Core clocks remain as verified by 01.
The existing patched exFAT engine and portable partition code are shared with U5.

From the repository root, build the **default read-only image first**:

```text
rtk cargo build -p unit-smoke-l4-02-exfat --target thumbv7em-none-eabihf --release --locked
rtk probe-rs run --chip STM32L476RGTx --probe 0483:3748 --speed 100 --verify --non-interactive target/thumbv7em-none-eabihf/release/unit-smoke-l4-02-exfat
```

It requires PC6 card-detect low and valid raw/MBR exFAT. It does not format or
repair the card. Look for `L4 EXFAT READ PASS`, observed DMA interrupts, zero
writes and `L4 EXFAT PASS write_test=false`. The blue LED then blinks at 1 Hz.

To exercise file writing after the inspection passes:

```text
rtk cargo build -p unit-smoke-l4-02-exfat --target thumbv7em-none-eabihf --release --locked --features write-test
rtk probe-rs run --chip STM32L476RGTx --probe 0483:3748 --speed 100 --verify --non-interactive target/thumbv7em-none-eabihf/release/unit-smoke-l4-02-exfat
```

This creates one new `/L4Ennnnn.BIN` file using exclusive creation. Existing
names are skipped; no file is overwritten, truncated, deleted or resumed. It
writes at least 256 KiB + 173 bytes, flushes/closes, checks collision rejection,
mounts with fresh caches and verifies every byte and EOF. A bounded root-entry
fingerprint checks that pre-existing names/types/lengths remain consistent.
The test file remains on the card.

Rebuild and flash the default image afterwards. It verifies the most recent
test-named file without permitting writes, providing a reset-persistence check.
Leave this default profile running so later resets do not create new files.
Retain the RTT logs and `L4 SD stats` counters with the exact flashed image.

Use separate Cargo invocations for U5 and L4; the root default remains U5.
If attaching fails, add `--connect-under-reset`. Avoid live debug attach during
SD operations, which may disturb timing. An operation that fails or exceeds its
software timeout halts with an RTT failure instead of proceeding to writes/PASS.
Do not remove the card during this initial smoke test.

Statics and DMA buffers use SRAM1; the CPU stack has a separate, explicitly
linked 32 KiB SRAM2 bank. Startup paints/read-verifies its unused span, and the
write adapter checks its bottom guard. DMA buffers are checked to stay in SRAM1.
A one-block cache parameter, shared aligned 4 KiB chunk and 2 KiB diagnostic heap
bound the diagnostic allocation. It is neither a recording throughput
benchmark nor a power-loss/removal recovery qualification. See the evidence
directory for actual board measurements and outstanding checks.
