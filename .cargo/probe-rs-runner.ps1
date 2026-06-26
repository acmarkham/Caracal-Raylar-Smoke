param(
    [Parameter(ValueFromRemainingArguments = $true)]
    [string[]]$RunnerArgs
)

$defmtLog = "trace"

foreach ($arg in $RunnerArgs) {
    if ($arg -match 'unit-smoke-09_serial-gps_pps') {
        $defmtLog = "info"
        break
    }
}

$env:DEFMT_LOG = $defmtLog

& probe-rs run --chip STM32U585CI @RunnerArgs
$exitCode = $LASTEXITCODE

if ($null -eq $exitCode) {
    $exitCode = 0
}

exit $exitCode
