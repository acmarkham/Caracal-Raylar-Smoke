# Caracal-Raylar-Smoke
Smoke tests for Caracal Raylar boards using Rust Embassy

## Automated firmware loop

Use `scripts/firmware.ps1` to build one firmware package, flash it with
probe-rs, and capture its defmt/RTT output without leaving an unbounded process
running:

```powershell
powershell -ExecutionPolicy Bypass -File scripts/firmware.ps1 `
    -Package unit-smoke-01-helloworld `
    -MonitorSeconds 10
```

The command verifies the flash, prints target output, and saves every session
under `.probe-rs-logs/`. The most recent target output is also copied to
`.probe-rs-logs/latest.log` so it can be inspected after the run.

To turn firmware output into an automated pass condition, provide a regular
expression with `-Until`. The command stops successfully as soon as it sees the
expression and exits with code 4 if the timeout expires first:

```powershell
powershell -ExecutionPolicy Bypass -File scripts/firmware.ps1 `
    -Package integration-test-002-monoaudiolog `
    -MonitorSeconds 60 `
    -Until "recording started|TEST PASS"
```

Useful options:

- `-DefmtLog debug` changes the target log filter.
- `-Probe <VID:PID:SERIAL>` selects a probe. `PROBE_RS_PROBE` provides the same
  selection without changing the command.
- `-ConnectUnderReset` helps recover firmware that reconfigures the SWD pins or
  immediately enters a low-power mode.
- `-NoBuild` reflashes the existing artifact, and `-NoVerify` skips read-back
  verification when iteration speed matters more than certainty.
- `-MonitorSeconds 0` streams until probe-rs exits or Ctrl+C is pressed.
- `-CargoArgs <args>` and `-ProbeArgs <args>` pass advanced options through.

The normal `cargo run -p <package> --release` runner is also non-interactive,
verifies flashed firmware, and honors `PROBE_RS_PROBE`. Use the script above for
automation because it adds bounded monitoring and persistent output capture.
