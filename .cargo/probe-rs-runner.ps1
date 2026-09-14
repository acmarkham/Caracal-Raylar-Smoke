param(
    [Parameter(ValueFromRemainingArguments = $true)]
    [string[]]$RunnerArgs
)

$defmtLog = "info"

foreach ($arg in $RunnerArgs) {
    if ($arg -match 'unit-smoke-09_serial-gps_pps') {
        $defmtLog = "info"
        break
    }
}

$env:DEFMT_LOG = $defmtLog

$probeArgs = @(
    "run",
    "--chip", "STM32U595VJ",
    "--non-interactive",
    "--disable-progressbars",
    "--verify"
)

if ($env:PROBE_RS_PROBE) {
    $probeArgs += @("--probe", $env:PROBE_RS_PROBE)
}

& probe-rs @probeArgs @RunnerArgs
$exitCode = $LASTEXITCODE

if ($null -eq $exitCode) {
    $exitCode = 0
}

exit $exitCode
