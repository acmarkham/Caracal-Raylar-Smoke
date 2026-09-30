[CmdletBinding()]
param(
    [Parameter(Mandatory = $true, Position = 0)]
    [ValidateNotNullOrEmpty()]
    [string]$Package,

    [string]$Bin,

    [ValidateSet("debug", "release")]
    [string]$Profile = "release",

    [ValidateScript({
        if ($_ -eq 0 -or ($_ -ge 5 -and $_ -le 86400)) {
            return $true
        }
        throw "MonitorSeconds must be 0 (unbounded) or between 5 and 86400."
    })]
    [int]$MonitorSeconds = 20,

    [string]$Until,

    [ValidateNotNullOrEmpty()]
    [string]$Chip = "STM32U595VJ",

    [string]$Probe = $env:PROBE_RS_PROBE,

    [ValidateNotNullOrEmpty()]
    [string]$DefmtLog = "info",

    [switch]$ConnectUnderReset,
    [switch]$NoBuild,
    [switch]$BuildOnly,
    [switch]$NoVerify,
    [switch]$DryRun,
    [switch]$QuietTargetOutput,

    [string[]]$CargoArgs = @(),
    [string[]]$ProbeArgs = @()
)

$ErrorActionPreference = "Stop"
$targetTriple = "thumbv8m.main-none-eabihf"
$workspaceRoot = (Resolve-Path (Join-Path $PSScriptRoot "..")).Path
$logRoot = Join-Path $workspaceRoot ".probe-rs-logs"
$metadataConfigPath = Join-Path $workspaceRoot "firmware-metadata.json"
$buildRecordRoot = Join-Path $workspaceRoot "firmware-builds"

function Require-Command {
    param([Parameter(Mandatory = $true)][string]$Name)

    $command = Get-Command $Name -ErrorAction SilentlyContinue
    if ($null -eq $command) {
        throw "Required command '$Name' was not found on PATH."
    }
    return $command.Source
}

function Read-NewText {
    param(
        [Parameter(Mandatory = $true)][string]$Path,
        [Parameter(Mandatory = $true)][ref]$Offset
    )

    if (-not (Test-Path -LiteralPath $Path)) {
        return ""
    }

    $stream = [System.IO.File]::Open(
        $Path,
        [System.IO.FileMode]::Open,
        [System.IO.FileAccess]::Read,
        [System.IO.FileShare]::ReadWrite
    )

    try {
        if ($stream.Length -lt $Offset.Value) {
            $Offset.Value = 0L
        }

        [void]$stream.Seek($Offset.Value, [System.IO.SeekOrigin]::Begin)
        $reader = New-Object System.IO.StreamReader($stream, [System.Text.Encoding]::UTF8, $true, 4096, $true)
        try {
            $text = $reader.ReadToEnd()
            $Offset.Value = $stream.Position
            return $text
        }
        finally {
            $reader.Dispose()
        }
    }
    finally {
        $stream.Dispose()
    }
}

function Quote-NativeArgument {
    param([AllowEmptyString()][string]$Value)

    if ($Value.Length -eq 0) {
        return '""'
    }
    if ($Value -notmatch '[\s"]') {
        return $Value
    }

    # Follow the Windows CommandLineToArgvW escaping rules. In particular,
    # backslashes immediately before a quote or the closing quote are doubled.
    $quoted = '"'
    $backslashes = 0
    foreach ($character in $Value.ToCharArray()) {
        if ($character -eq '\') {
            $backslashes++
            continue
        }

        if ($character -eq '"') {
            $quoted += (('\\' * (($backslashes * 2) + 1)) -join '')
            $quoted += '"'
            $backslashes = 0
            continue
        }

        if ($backslashes -gt 0) {
            $quoted += (('\\' * $backslashes) -join '')
            $backslashes = 0
        }
        $quoted += $character
    }

    if ($backslashes -gt 0) {
        $quoted += (('\\' * ($backslashes * 2)) -join '')
    }
    return $quoted + '"'
}

function Stop-ProbeProcess {
    param([Parameter(Mandatory = $true)][System.Diagnostics.Process]$Process)

    if (-not $Process.HasExited) {
        Stop-Process -Id $Process.Id -Force -ErrorAction SilentlyContinue
        $Process.WaitForExit()
    }
}

function Write-BuildRecord {
    param(
        [Parameter(Mandatory = $true)][string]$Path,
        [Parameter(Mandatory = $true)][System.Collections.IDictionary]$Record
    )

    New-Item -ItemType Directory -Path (Split-Path -Parent $Path) -Force | Out-Null
    $json = $Record | ConvertTo-Json -Depth 4
    [System.IO.File]::WriteAllText($Path, $json + [Environment]::NewLine)
}

$previousDefmtLog = $env:DEFMT_LOG
$hadDefmtLog = Test-Path Env:DEFMT_LOG
$process = $null
$processStarted = $false
$stoppedByScript = $false
$matchedUntil = $false
$buildRecord = $null
$buildRecordPath = $null
$buildEnvironment = @{}

Push-Location $workspaceRoot
try {
    $cargoPath = Require-Command "cargo"
    if ($BuildOnly -and $NoBuild) {
        throw "-BuildOnly and -NoBuild cannot be used together."
    }
    $untilRegex = $null
    if ($Until) {
        try {
            $untilRegex = New-Object System.Text.RegularExpressions.Regex($Until)
        }
        catch {
            throw "Invalid -Until regular expression '$Until': $($_.Exception.InnerException.Message)"
        }
    }

    $metadataText = & $cargoPath metadata --no-deps --format-version 1
    if ($LASTEXITCODE -ne 0) {
        throw "cargo metadata failed with exit code $LASTEXITCODE."
    }

    $metadata = $metadataText | ConvertFrom-Json
    $packageInfo = @($metadata.packages | Where-Object { $_.name -eq $Package })
    if ($packageInfo.Count -eq 0) {
        throw "Workspace package '$Package' was not found."
    }
    if ($packageInfo.Count -gt 1) {
        throw "More than one workspace package is named '$Package'."
    }

    $binaryTargets = @($packageInfo[0].targets | Where-Object { $_.kind -contains "bin" })
    if ($Bin) {
        $binaryTargets = @($binaryTargets | Where-Object { $_.name -eq $Bin })
    }

    if ($binaryTargets.Count -eq 0) {
        $suffix = if ($Bin) { " named '$Bin'" } else { "" }
        throw "Package '$Package' has no binary target$suffix."
    }
    if ($binaryTargets.Count -gt 1) {
        $names = ($binaryTargets.name | Sort-Object) -join ", "
        throw "Package '$Package' has multiple binaries ($names); select one with -Bin."
    }

    $binaryName = $binaryTargets[0].name
    $safePackage = $Package -replace '[^A-Za-z0-9_.-]', '_'
    $profileDirectory = if ($Profile -eq "release") { "release" } else { "debug" }
    $artifactPath = Join-Path $metadata.target_directory "$targetTriple\$profileDirectory\$binaryName"

    if (-not $NoBuild) {
        if (-not (Test-Path -LiteralPath $metadataConfigPath -PathType Leaf)) {
            throw "Firmware metadata config was not found at '$metadataConfigPath'."
        }
        $metadataConfig = Get-Content -LiteralPath $metadataConfigPath -Raw | ConvertFrom-Json
        if (-not $metadataConfig.boardRevision) {
            throw "firmware-metadata.json must define boardRevision."
        }

        $gitPath = Require-Command "git"
        $gitHash = (& $gitPath rev-parse HEAD).Trim()
        if ($LASTEXITCODE -ne 0 -or -not $gitHash) {
            throw "Unable to determine the source Git hash."
        }
        $trackedChanges = & $gitPath status --porcelain --untracked-files=no
        if ($LASTEXITCODE -ne 0) {
            throw "Unable to determine whether the source tree is dirty."
        }
        $sourceDirty = [bool]$trackedChanges
        $gitIdentity = if ($sourceDirty) { "$gitHash-dirty" } else { $gitHash }
        $buildTimestamp = [DateTime]::UtcNow.ToString(
            "yyyy-MM-dd'T'HH:mm:ss'Z'",
            [Globalization.CultureInfo]::InvariantCulture
        )

        $metadataEnvironment = [ordered]@{
            RAYLAR_FIRMWARE_VERSION = [string]$packageInfo[0].version
            RAYLAR_GIT_HASH = $gitIdentity
            RAYLAR_BUILD_TIMESTAMP = $buildTimestamp
            RAYLAR_BUILD_PROFILE = $Profile
            RAYLAR_BOARD_REVISION = [string]$metadataConfig.boardRevision
        }
        foreach ($entry in $metadataEnvironment.GetEnumerator()) {
            $existing = [Environment]::GetEnvironmentVariable($entry.Key, "Process")
            $buildEnvironment[$entry.Key] = @{
                WasSet = $null -ne $existing
                Value = $existing
            }
            [Environment]::SetEnvironmentVariable($entry.Key, $entry.Value, "Process")
        }

        $buildArguments = @("build", "--package", $Package, "--bin", $binaryName, "--target", $targetTriple)
        if ($Profile -eq "release") {
            $buildArguments += "--release"
        }
        $buildArguments += $CargoArgs

        Write-Host "==> Building $Package ($Profile)"
        & $cargoPath @buildArguments
        if ($LASTEXITCODE -ne 0) {
            throw "cargo build failed with exit code $LASTEXITCODE."
        }
    }

    if (-not (Test-Path -LiteralPath $artifactPath -PathType Leaf)) {
        throw "Firmware artifact was not found at '$artifactPath'. Build without -NoBuild first."
    }

    if (-not $NoBuild) {
        $artifactSha256 = (Get-FileHash -LiteralPath $artifactPath -Algorithm SHA256).Hash.ToLowerInvariant()
        $buildRecordPath = Join-Path $buildRecordRoot "$safePackage.json"
        $buildRecord = [ordered]@{
            schema_version = 1
            package = $Package
            binary = $binaryName
            firmware_version = [string]$packageInfo[0].version
            source_git = $gitIdentity
            source_tree_dirty = $sourceDirty
            build_timestamp_utc = $buildTimestamp
            build_profile = $Profile
            board_revision = [string]$metadataConfig.boardRevision
            target = $targetTriple
            artifact_sha256 = $artifactSha256
            runtime_crc32 = $null
        }
        Write-BuildRecord -Path $buildRecordPath -Record $buildRecord
        Write-Host "    Build record: $buildRecordPath (commit this file with the tested source)"
    }

    if ($BuildOnly) {
        Write-Host "==> Build complete; firmware was not flashed."
        return
    }

    $probeRsPath = Require-Command "probe-rs"
    New-Item -ItemType Directory -Path $logRoot -Force | Out-Null
    $timestamp = Get-Date -Format "yyyyMMdd-HHmmss"
    $sessionDirectory = Join-Path $logRoot "$timestamp-$safePackage-$PID"
    New-Item -ItemType Directory -Path $sessionDirectory -Force | Out-Null

    $targetLog = Join-Path $sessionDirectory "target.log"
    $stdoutLog = Join-Path $sessionDirectory "probe-rs.stdout.log"
    $stderrLog = Join-Path $sessionDirectory "probe-rs.stderr.log"

    $runArguments = @(
        "run",
        "--chip", $Chip,
        "--non-interactive",
        "--disable-progressbars",
        "--target-output-file", $targetLog
    )
    if (-not $NoVerify) {
        $runArguments += "--verify"
    }
    if ($Probe) {
        $runArguments += @("--probe", $Probe)
    }
    if ($ConnectUnderReset) {
        $runArguments += "--connect-under-reset"
    }
    if ($DryRun) {
        $runArguments += "--dry-run"
    }
    $runArguments += $ProbeArgs
    $runArguments += $artifactPath

    $env:DEFMT_LOG = $DefmtLog

    $processInfo = New-Object System.Diagnostics.ProcessStartInfo
    $processInfo.FileName = $probeRsPath
    $processInfo.Arguments = ($runArguments | ForEach-Object { Quote-NativeArgument ([string]$_) }) -join " "
    $processInfo.UseShellExecute = $false
    $processInfo.CreateNoWindow = $true
    $processInfo.RedirectStandardOutput = $true
    $processInfo.RedirectStandardError = $true

    $process = New-Object System.Diagnostics.Process
    $process.StartInfo = $processInfo

    Write-Host "==> Flashing $binaryName and reading defmt/RTT output"
    Write-Host "    Logs: $sessionDirectory"
    if (-not $process.Start()) {
        throw "probe-rs did not start."
    }
    $processStarted = $true

    # Asynchronously drain both pipes so neither can fill and deadlock probe-rs.
    # They are persisted once the bounded monitoring session finishes.
    $stdoutTask = $process.StandardOutput.ReadToEndAsync()
    $stderrTask = $process.StandardError.ReadToEndAsync()
    $deadline = if ($MonitorSeconds -gt 0) {
        [DateTime]::UtcNow.AddSeconds($MonitorSeconds)
    }
    else {
        [DateTime]::MaxValue
    }
    $targetOffset = 0L
    $targetHadOutput = $false
    $matchWindow = ""

    while (-not $process.HasExited) {
        $targetChunk = Read-NewText -Path $targetLog -Offset ([ref]$targetOffset)
        if ($targetChunk) {
            $targetHadOutput = $true
            if (-not $QuietTargetOutput) {
                Write-Host -NoNewline $targetChunk
            }
            if ($Until) {
                $matchWindow = ($matchWindow + $targetChunk)
                if ($matchWindow.Length -gt 65536) {
                    $matchWindow = $matchWindow.Substring($matchWindow.Length - 65536)
                }
                if ($untilRegex.IsMatch($matchWindow)) {
                    $matchedUntil = $true
                    $stoppedByScript = $true
                    break
                }
            }
        }

        if ([DateTime]::UtcNow -ge $deadline) {
            $stoppedByScript = $true
            break
        }

        Start-Sleep -Milliseconds 100
    }

    if ($stoppedByScript) {
        Stop-ProbeProcess -Process $process
    }
    else {
        $process.WaitForExit()
    }

    $stdoutText = $stdoutTask.GetAwaiter().GetResult()
    $stderrText = $stderrTask.GetAwaiter().GetResult()
    [System.IO.File]::WriteAllText($stdoutLog, $stdoutText)
    [System.IO.File]::WriteAllText($stderrLog, $stderrText)

    $targetChunk = Read-NewText -Path $targetLog -Offset ([ref]$targetOffset)
    if ($targetChunk) {
        $targetHadOutput = $true
        if (-not $QuietTargetOutput) {
            Write-Host -NoNewline $targetChunk
        }
        if ($Until -and $untilRegex.IsMatch($matchWindow + $targetChunk)) {
            $matchedUntil = $true
        }
    }

    # probe-rs currently mirrors target output to stdout even when it also
    # writes --target-output-file. Avoid echoing that stream twice.
    if ($stdoutText -and -not $targetHadOutput -and -not $QuietTargetOutput) {
        Write-Host -NoNewline $stdoutText
    }
    if ($stderrText) {
        [Console]::Error.Write($stderrText)
    }

    $latestTargetLog = Join-Path $logRoot "latest.log"
    if (Test-Path -LiteralPath $targetLog) {
        Copy-Item -LiteralPath $targetLog -Destination $latestTargetLog -Force
        if ($null -ne $buildRecord) {
            $targetText = Get-Content -LiteralPath $targetLog -Raw
            $runtimeCrcMatch = [regex]::Match(
                $targetText,
                'runtime_crc32=Known\((?<crc>(?:0x)?[0-9A-Fa-f]+)\)'
            )
            if ($runtimeCrcMatch.Success) {
                $buildRecord.runtime_crc32 = $runtimeCrcMatch.Groups['crc'].Value
                Write-BuildRecord -Path $buildRecordPath -Record $buildRecord
            }
        }
    }
    else {
        New-Item -ItemType File -Path $latestTargetLog -Force | Out-Null
    }

    if (-not $stoppedByScript -and $process.ExitCode -ne 0) {
        if ($stderrText -match 'reset not supported by WinUSB') {
            [Console]::Error.WriteLine(
                "Hint: probe-rs can see the ST-Link, but its Windows USB driver cannot reset it. " +
                "Reconnect the probe; if this persists, update probe-rs or bind the ST-Link debug interface to a supported USB driver."
            )
        }
        elseif ($stderrText -match 'Access is denied') {
            [Console]::Error.WriteLine(
                "Hint: another debugger may own the ST-Link. Close other probe-rs/debug sessions or reconnect the probe."
            )
        }
        [Console]::Error.WriteLine("probe-rs exited with code $($process.ExitCode). See $sessionDirectory")
        exit $process.ExitCode
    }

    if ($Until -and -not $matchedUntil) {
        [Console]::Error.WriteLine("Firmware output did not match -Until '$Until'. See $sessionDirectory")
        exit 4
    }

    if ($matchedUntil) {
        Write-Host "`n==> Matched '$Until'; probe-rs stopped cleanly."
    }
    elseif ($MonitorSeconds -gt 0) {
        Write-Host "`n==> Captured $MonitorSeconds seconds; probe-rs stopped cleanly."
    }
    else {
        Write-Host "`n==> probe-rs exited cleanly."
    }
    Write-Host "    Latest target output: $latestTargetLog"
}
finally {
    if ($processStarted -and -not $process.HasExited) {
        Stop-ProbeProcess -Process $process
    }
    if ($hadDefmtLog) {
        $env:DEFMT_LOG = $previousDefmtLog
    }
    else {
        Remove-Item Env:DEFMT_LOG -ErrorAction SilentlyContinue
    }
    foreach ($entry in $buildEnvironment.GetEnumerator()) {
        if ($entry.Value.WasSet) {
            [Environment]::SetEnvironmentVariable($entry.Key, $entry.Value.Value, "Process")
        }
        else {
            [Environment]::SetEnvironmentVariable($entry.Key, $null, "Process")
        }
    }
    Pop-Location
}
