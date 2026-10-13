[CmdletBinding()]
param(
    [Parameter(Mandatory = $true)][string]$OriginalFirmware,
    [Parameter(Mandatory = $true)][string]$Results,
    [int]$MonitorSeconds = 1800
)
$ErrorActionPreference = 'Stop'
$workspace = (Resolve-Path (Join-Path $PSScriptRoot '../..')).Path
$original = (Resolve-Path -LiteralPath $OriginalFirmware).Path
$resultsDirectory = (Resolve-Path -LiteralPath $Results).Path
$probe = '0483:374b:0667FF505277504867014435'
$probeExe = (Get-Command probe-rs).Source
function Invoke-ProbeCommand {
    param([string]$Arguments, [string]$Log)
    $startInfo = New-Object System.Diagnostics.ProcessStartInfo
    $startInfo.FileName = $probeExe
    $startInfo.Arguments = $Arguments
    $startInfo.UseShellExecute = $false
    $startInfo.CreateNoWindow = $true
    $startInfo.RedirectStandardOutput = $true
    $startInfo.RedirectStandardError = $true
    $process = New-Object System.Diagnostics.Process
    $process.StartInfo = $startInfo
    try {
        if (-not $process.Start()) { throw 'probe-rs did not start.' }
        $stdout = $process.StandardOutput.ReadToEndAsync()
        $stderr = $process.StandardError.ReadToEndAsync()
        $process.WaitForExit()
        $output = $stdout.GetAwaiter().GetResult() + $stderr.GetAwaiter().GetResult()
        [System.IO.File]::AppendAllText($Log, $output)
        Write-Host $output
        if ($process.ExitCode -ne 0) { throw "probe-rs failed: $($process.ExitCode)" }
    }
    finally { $process.Dispose() }
}
try {
    & (Join-Path $workspace 'scripts/firmware.ps1') -Package service-test-flacencoder `
        -Bin flac-cpu-benchmark -NoBuild -Probe $probe -MonitorSeconds $MonitorSeconds `
        -QuietTargetOutput
    if ($LASTEXITCODE -and $LASTEXITCODE -ne 0) { throw "Benchmark failed: $LASTEXITCODE" }
    Copy-Item -LiteralPath (Join-Path $workspace '.probe-rs-logs/latest.log') `
        -Destination (Join-Path $resultsDirectory 'board.log') -Force
    if (-not (Select-String -LiteralPath (Join-Path $resultsDirectory 'board.log') -Pattern CPU_BENCH_DONE -Quiet)) {
        throw 'Benchmark did not complete.'
    }
}
finally {
    Write-Host 'Restoring and verifying saved aardwolf firmware, then resetting the board.'
    $restoreLog = Join-Path $resultsDirectory 'restore.log'
    [System.IO.File]::WriteAllText($restoreLog, '')
    Invoke-ProbeCommand -Arguments ('download --chip STM32U595VJ --probe ' + $probe +
        ' --non-interactive --disable-progressbars --verify "' + $original + '"') -Log $restoreLog
    Invoke-ProbeCommand -Arguments ('reset --chip STM32U595VJ --probe ' + $probe + ' --non-interactive') -Log $restoreLog
    Add-Content -LiteralPath $restoreLog -Value 'Saved firmware verified; reset command succeeded.'
}
