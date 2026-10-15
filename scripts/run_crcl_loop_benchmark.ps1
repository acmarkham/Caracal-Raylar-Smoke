[CmdletBinding()]
param([int]$MonitorSeconds = 60)
$ErrorActionPreference = 'Stop'
# A dedicated binary means normal capture/SD test entrypoints cannot run.
rtk proxy powershell -ExecutionPolicy Bypass -File scripts/firmware.ps1 `
    -Package service-test-crclencoder -Bin crcl-loop-benchmark `
    -MonitorSeconds $MonitorSeconds -Until 'CRCL LOOP BENCH PASS' `
    -CargoArgs '--features=loop-benchmark'
if ($LASTEXITCODE -ne 0) { exit $LASTEXITCODE }
