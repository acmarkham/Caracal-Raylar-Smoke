$ErrorActionPreference = 'Stop'
& "$PSScriptRoot/firmware.ps1" service-test-crclencoder -Bin service-test-crclencoder -CargoArgs @('--features', 'recovery-test') -MonitorSeconds 90 -Until 'CRCL RECOVERY TARGET PASS'
if ($LASTEXITCODE -ne 0) { throw "Audio recovery target test failed: $LASTEXITCODE" }
