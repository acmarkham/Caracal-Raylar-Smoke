[CmdletBinding()]
param(
    [long]$StartUtc = 1900000800,
    [ValidateRange(1, 10000)][int]$Pairs = 256,
    [ValidateRange(0, 10000)][int]$SmallPairs = 32,
    [ValidateRange(1, 10000)][int]$FullBlocks = 937,
    [ValidateRange(0, 10000)][int]$RewriteUntil = 224,
    [switch]$AlternateRewrite,
    [ValidateRange(0, 1000)][int]$PaceMs = 0,
    [switch]$BuildOnly,
    [switch]$DeepDiagnostics,
    [ValidateRange(100, 24000)][int]$ProbeSpeedKhz = 1000,
    [switch]$ConnectUnderReset,
    [string]$Output = 'analysis/sd_lifecycle/20261016/accelerated'
)
$ErrorActionPreference = 'Stop'
$settings = [ordered]@{
    SD_STRESS_START_UTC = "$StartUtc"
    SD_STRESS_PAIRS = "$Pairs"
    SD_STRESS_SMALL_PAIRS = "$SmallPairs"
    SD_STRESS_FULL_BLOCKS = "$FullBlocks"
    SD_STRESS_REWRITE_UNTIL = "$RewriteUntil"
    SD_STRESS_ALTERNATE_REWRITE = "$([int][bool]$AlternateRewrite)"
    SD_STRESS_PACE_MS = "$PaceMs"
}
$saved = @{}
foreach ($key in $settings.Keys) {
    $saved[$key] = [Environment]::GetEnvironmentVariable($key, 'Process')
    [Environment]::SetEnvironmentVariable($key, $settings[$key], 'Process')
}
try {
    $workspace = Split-Path -Parent $PSScriptRoot
    $destination = Join-Path $workspace $Output
    New-Item -ItemType Directory -Path $destination -Force | Out-Null
    $settings | ConvertTo-Json | Set-Content -LiteralPath (Join-Path $destination 'settings.json')
    $seconds = [Math]::Min(86400, [Math]::Max(2400, $Pairs * ($FullBlocks * $PaceMs / 1000 + 30)))
    $firmwareOptions = @{}
    if ($BuildOnly) { $firmwareOptions.BuildOnly = $true }
    if ($DeepDiagnostics) { $firmwareOptions.CargoArgs = @('--features', 'deep-sd-diagnostics') }
    if ($ConnectUnderReset) { $firmwareOptions.ConnectUnderReset = $true }
    $firmwareOptions.ProbeArgs = @('--speed', "$ProbeSpeedKhz")
    Copy-Item -LiteralPath (Join-Path $workspace 'servicetests/crclencoder/sd_lifecycle_stress.rs') -Destination (Join-Path $destination 'source.rs')
    & "$PSScriptRoot/firmware.ps1" service-test-crclencoder -Bin sd-lifecycle-stress `
        -MonitorSeconds $seconds -Until 'SD LIFECYCLE STRESS PASS' -QuietTargetOutput @firmwareOptions
    $runnerExit = $LASTEXITCODE
    Copy-Item -LiteralPath (Join-Path $workspace 'firmware-builds/service-test-crclencoder.json') -Destination (Join-Path $destination 'build.json')
    if ($BuildOnly) {
        if ($runnerExit -ne 0) { throw "Firmware build failed: $runnerExit" }
        return
    }
    Copy-Item -LiteralPath (Join-Path $workspace '.probe-rs-logs/latest.log') -Destination (Join-Path $destination 'target.log')
    & rtk proxy python "$PSScriptRoot/sd_lifecycle_report.py" (Join-Path $destination 'target.log') $destination
    if ($LASTEXITCODE -ne 0) { throw "Report failed: $LASTEXITCODE" }
    if ($runnerExit -ne 0) { throw "Firmware runner failed: $runnerExit (partial results preserved)" }
}
finally {
    foreach ($key in $settings.Keys) { [Environment]::SetEnvironmentVariable($key, $saved[$key], 'Process') }
}
