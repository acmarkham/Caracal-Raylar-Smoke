[CmdletBinding()]
param(
    [ValidateRange(15, 86400)]
    [int]$MonitorSeconds = 120,

    [string]$Probe = $env:PROBE_RS_PROBE,

    [ValidateNotNullOrEmpty()]
    [string]$DefmtLog = "info",

    [string]$OutputPath,

    [switch]$ConnectUnderReset,
    [switch]$NoBuild
)

$ErrorActionPreference = "Stop"
$workspaceRoot = (Resolve-Path (Join-Path $PSScriptRoot "..")).Path
$firmwareScript = Join-Path $PSScriptRoot "firmware.ps1"
$logRoot = Join-Path $workspaceRoot ".probe-rs-logs"
if (-not $OutputPath) {
    $OutputPath = Join-Path $logRoot "sd-card-report.md"
}
elseif (-not [System.IO.Path]::IsPathRooted($OutputPath)) {
    $OutputPath = Join-Path $workspaceRoot $OutputPath
}

function Escape-MarkdownCell {
    param([AllowEmptyString()][string]$Value)
    return $Value.Replace("\", "\\").Replace("|", "\|").Replace(
        ([char]96).ToString(),
        "\x60"
    ).Replace("`r", "").Replace("`n", "\n")
}

function Format-EscapedBytes {
    param([Parameter(Mandatory = $true)][System.Collections.Generic.List[byte]]$Bytes)

    $text = New-Object System.Text.StringBuilder
    foreach ($value in $Bytes) {
        switch ($value) {
            9 { [void]$text.Append("\t"); continue }
            10 { [void]$text.AppendLine(); continue }
            13 { [void]$text.Append("\r"); continue }
            92 { [void]$text.Append("\\"); continue }
            96 { [void]$text.Append("\x60"); continue }
        }

        if ($value -ge 32 -and $value -le 126) {
            [void]$text.Append([char]$value)
        }
        else {
            [void]$text.AppendFormat("\x{0:X2}", $value)
        }
    }
    return $text.ToString()
}

function Format-HexBytes {
    param([Parameter(Mandatory = $true)][System.Collections.Generic.List[byte]]$Bytes)

    $text = New-Object System.Text.StringBuilder
    for ($index = 0; $index -lt $Bytes.Count; $index += 16) {
        $end = [Math]::Min($index + 16, $Bytes.Count)
        $line = for ($cursor = $index; $cursor -lt $end; $cursor++) {
            $Bytes[$cursor].ToString("X2")
        }
        [void]$text.AppendLine(($line -join " "))
    }
    return $text.ToString().TrimEnd()
}

$powerShellPath = (Get-Command powershell.exe -ErrorAction Stop).Source
$arguments = @(
    "-NoProfile",
    "-ExecutionPolicy", "Bypass",
    "-File", $firmwareScript,
    "-Package", "unit-smoke-31-sd-inspect",
    "-MonitorSeconds", $MonitorSeconds.ToString(),
    "-Until", "SDGPT\|END\|status=",
    "-DefmtLog", $DefmtLog,
    "-QuietTargetOutput"
)
if ($Probe) {
    $arguments += @("-Probe", $Probe)
}
if ($ConnectUnderReset) {
    $arguments += "-ConnectUnderReset"
}
if ($NoBuild) {
    $arguments += "-NoBuild"
}
$arguments += @("-ProbeArgs", "--no-location")

Write-Host "==> Collecting bounded, read-only SD-card diagnostics"
& $powerShellPath @arguments
$firmwareExitCode = $LASTEXITCODE
if ($firmwareExitCode -ne 0) {
    throw "SD inspection firmware failed with exit code $firmwareExitCode."
}

$targetLog = Join-Path $logRoot "latest.log"
if (-not (Test-Path -LiteralPath $targetLog -PathType Leaf)) {
    throw "Firmware completed without producing '$targetLog'."
}

$payloads = New-Object System.Collections.Generic.List[string]
foreach ($line in Get-Content -LiteralPath $targetLog) {
    $start = $line.IndexOf("SDGPT|")
    if ($start -lt 0) {
        continue
    }
    $end = $line.IndexOf("|#", $start)
    if ($end -lt 0) {
        continue
    }
    $payloads.Add($line.Substring($start, $end - $start))
}

if ($payloads.Count -eq 0) {
    throw "No SDGPT records were found in '$targetLog'."
}

$entries = New-Object System.Collections.Generic.List[object]
$snippets = [ordered]@{}
$recentSyslogRecords = New-Object System.Collections.Generic.List[string]
$diagnostics = New-Object System.Collections.Generic.List[string]
$cardSummary = "Unavailable"
$volumeSummary = "Unavailable"
$endRecord = $null
$fatal = $false

foreach ($payload in $payloads) {
    if ($payload -match '^SDGPT\|CARD\|blocks=(\d+)\|bytes=(\d+)$') {
        $cardSummary = "Blocks: $($Matches[1]); bytes: $($Matches[2])"
        continue
    }
    if ($payload -match '^SDGPT\|VOLUME\|start_lba=(\d+)\|blocks=(\d+)$') {
        $volumeSummary = "Start LBA: $($Matches[1]); blocks: $($Matches[2])"
        continue
    }
    if ($payload -match '^SDGPT\|ENTRY\|kind=([^|]+)\|size=(\d+)\|depth=(\d+)\|path=(.*)$') {
        $entries.Add([pscustomobject]@{
            Kind = $Matches[1]
            Size = [uint64]$Matches[2]
            Depth = [int]$Matches[3]
            Path = $Matches[4]
        })
        continue
    }
    if ($payload -match '^SDGPT\|DATA\|offset=(\d+)\|path=(.*?)\|bytes=\[(.*)\]$') {
        $path = $Matches[2]
        if (-not $snippets.Contains($path)) {
            $snippets[$path] = New-Object System.Collections.Generic.List[byte]
        }
        $byteText = $Matches[3].Trim()
        if ($byteText) {
            foreach ($token in $byteText.Split(',')) {
                $snippets[$path].Add([byte]::Parse($token.Trim()))
            }
        }
        continue
    }
    if ($payload -match '^SDGPT\|SYSLOG\|line=(.*)$') {
        $recentSyslogRecords.Add($Matches[1])
        continue
    }
    if ($payload -match '^SDGPT\|(ERROR|FATAL|TRUNCATED)\|') {
        $diagnostics.Add($payload)
        if ($Matches[1] -eq "FATAL") {
            $fatal = $true
        }
        continue
    }
    if ($payload -match '^SDGPT\|END\|') {
        $endRecord = $payload
    }
}

if (-not $endRecord) {
    throw "The firmware report has no SDGPT END record."
}

$report = New-Object System.Text.StringBuilder
[void]$report.AppendLine("# SD card diagnostic report")
[void]$report.AppendLine()
[void]$report.AppendLine("Generated: $([DateTime]::UtcNow.ToString('u')) UTC")
[void]$report.AppendLine()
[void]$report.AppendLine("- Card: $cardSummary")
[void]$report.AppendLine("- Volume: $volumeSummary")
[void]$report.AppendLine("- Result: ``$endRecord``")
[void]$report.AppendLine()
[void]$report.AppendLine("## Directory listing")
[void]$report.AppendLine()
[void]$report.AppendLine("| Kind | Bytes | Depth | Path |")
[void]$report.AppendLine("| --- | ---: | ---: | --- |")
foreach ($entry in $entries | Sort-Object Path) {
    $safePath = Escape-MarkdownCell $entry.Path
    [void]$report.AppendLine("| $($entry.Kind) | $($entry.Size) | $($entry.Depth) | ``$safePath`` |")
}

[void]$report.AppendLine()
[void]$report.AppendLine("## File snippets")
foreach ($path in $snippets.Keys) {
    $bytes = $snippets[$path]
    $safePath = Escape-MarkdownCell $path
    [void]$report.AppendLine()
    [void]$report.AppendLine("### ``$safePath``")
    [void]$report.AppendLine()
    [void]$report.AppendLine("First $($bytes.Count) byte(s), escaped ASCII:")
    [void]$report.AppendLine()
    [void]$report.AppendLine('```text')
    [void]$report.AppendLine((Format-EscapedBytes $bytes))
    [void]$report.AppendLine('```')
    [void]$report.AppendLine()
    [void]$report.AppendLine("Hex:")
    [void]$report.AppendLine()
    [void]$report.AppendLine('```text')
    [void]$report.AppendLine((Format-HexBytes $bytes))
    [void]$report.AppendLine('```')
}

if ($recentSyslogRecords.Count -gt 0) {
    [void]$report.AppendLine()
    [void]$report.AppendLine("## Recent Time and GPS records from syslog tail")
    [void]$report.AppendLine()
    [void]$report.AppendLine('```text')
    foreach ($line in $recentSyslogRecords) {
        [void]$report.AppendLine($line)
    }
    [void]$report.AppendLine('```')
}

if ($diagnostics.Count -gt 0) {
    [void]$report.AppendLine()
    [void]$report.AppendLine("## Firmware diagnostics")
    [void]$report.AppendLine()
    foreach ($diagnostic in $diagnostics) {
        [void]$report.AppendLine("- ``$diagnostic``")
    }
}

$reportText = $report.ToString()
$outputDirectory = Split-Path -Parent $OutputPath
if ($outputDirectory) {
    New-Item -ItemType Directory -Path $outputDirectory -Force | Out-Null
}
[System.IO.File]::WriteAllText($OutputPath, $reportText, [System.Text.Encoding]::UTF8)

Write-Host "==> GPT-ready SD report: $OutputPath"
Write-Output $reportText

if ($fatal -or $endRecord -notmatch '^SDGPT\|END\|status=ok\|') {
    exit 5
}
