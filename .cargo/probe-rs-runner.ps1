param(
    [Parameter(ValueFromRemainingArguments = $true)]
    [string[]]$RunnerArgs
)

$defmtLog = "info"
$workspaceRoot = (Resolve-Path (Join-Path $PSScriptRoot "..")).Path
$logRoot = Join-Path $workspaceRoot ".probe-rs-logs"
$buildRecordRoot = Join-Path $workspaceRoot "firmware-builds"

foreach ($arg in $RunnerArgs) {
    if ($arg -match 'unit-smoke-09_serial-gps_pps') {
        $defmtLog = "info"
        break
    }
}

$env:DEFMT_LOG = $defmtLog

$artifactPath = if ($RunnerArgs.Count -gt 0) { $RunnerArgs[0] } else { $null }
$buildRecord = $null
$buildRecordPath = $null
$targetLog = $null

if ($artifactPath -and (Test-Path -LiteralPath $artifactPath -PathType Leaf) -and $env:RAYLAR_BUILD_PACKAGE) {
    New-Item -ItemType Directory -Path $logRoot -Force | Out-Null
    New-Item -ItemType Directory -Path $buildRecordRoot -Force | Out-Null
    $timestamp = Get-Date -Format "yyyyMMdd-HHmmss"
    $safePackage = $env:RAYLAR_BUILD_PACKAGE -replace '[^A-Za-z0-9_.-]', '_'
    $targetLog = Join-Path $logRoot "$timestamp-$safePackage-vscode.log"
    $buildRecordPath = Join-Path $buildRecordRoot "$safePackage.json"
    $buildRecord = [ordered]@{
        schema_version = 1
        package = $env:RAYLAR_BUILD_PACKAGE
        binary = [System.IO.Path]::GetFileName($artifactPath)
        firmware_version = $env:RAYLAR_FIRMWARE_VERSION
        source_git = $env:RAYLAR_GIT_HASH
        source_tree_dirty = $env:RAYLAR_GIT_DIRTY -eq "true"
        build_timestamp_utc = $env:RAYLAR_BUILD_TIMESTAMP
        build_profile = $env:RAYLAR_BUILD_PROFILE
        board_revision = $env:RAYLAR_BOARD_REVISION
        target = "thumbv8m.main-none-eabihf"
        artifact_sha256 = (Get-FileHash -LiteralPath $artifactPath -Algorithm SHA256).Hash.ToLowerInvariant()
        runtime_crc32 = $null
    }
    [System.IO.File]::WriteAllText(
        $buildRecordPath,
        ($buildRecord | ConvertTo-Json -Depth 4) + [Environment]::NewLine
    )
    Write-Host "Build record: $buildRecordPath (commit this file with the tested source)"
}

$probeArgs = @(
    "run",
    "--chip", "STM32U595VJ",
    "--non-interactive",
    "--disable-progressbars",
    "--verify"
)

if ($targetLog) {
    $probeArgs += @("--target-output-file", $targetLog)
}

if ($env:PROBE_RS_PROBE) {
    $probeArgs += @("--probe", $env:PROBE_RS_PROBE)
}

& probe-rs @probeArgs @RunnerArgs
$exitCode = $LASTEXITCODE

if ($buildRecord -and $targetLog -and (Test-Path -LiteralPath $targetLog -PathType Leaf)) {
    $targetText = Get-Content -LiteralPath $targetLog -Raw
    $runtimeCrcMatch = [regex]::Match(
        $targetText,
        'runtime_crc32=Known\((?<crc>(?:0x)?[0-9A-Fa-f]+)\)'
    )
    if ($runtimeCrcMatch.Success) {
        $buildRecord.runtime_crc32 = $runtimeCrcMatch.Groups['crc'].Value
        [System.IO.File]::WriteAllText(
            $buildRecordPath,
            ($buildRecord | ConvertTo-Json -Depth 4) + [Environment]::NewLine
        )
    }
}

if ($null -eq $exitCode) {
    $exitCode = 0
}

exit $exitCode
