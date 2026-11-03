param([string]$ArtifactPath = "$PSScriptRoot/../target/thumbv8m.main-none-eabihf/release/aardwolf")
$ErrorActionPreference = 'Stop'
. "$PSScriptRoot/firmware-artifacts.ps1"
$ArtifactPath = (Resolve-Path -LiteralPath $ArtifactPath).Path
$fixture = Join-Path ([IO.Path]::GetTempPath()) ('raylar-firmware-test-' + [Guid]::NewGuid().ToString('N'))

function Assert-True($Condition, [string]$Message) {
    if (-not $Condition) { throw $Message }
}
function Expect-Failure([scriptblock]$Action, [string]$Pattern) {
    try { & $Action }
    catch {
        Assert-True ($_.Exception.Message -match $Pattern) "Unexpected failure: $_"
        return
    }
    throw "Expected failure matching $Pattern"
}
try {
    New-Item -ItemType Directory -Path $fixture | Out-Null
    & git -C $fixture init --quiet
    [IO.File]::WriteAllText((Join-Path $fixture 'source.txt'), "original`n")
    & git -C $fixture add source.txt
    & git -C $fixture -c user.name=Test -c user.email=test@example.invalid commit --quiet -m fixture
    if ($LASTEXITCODE -ne 0) { throw 'Cannot initialize test repository.' }
    $gitHash = (& git -C $fixture rev-parse HEAD).Trim()
    [IO.File]::WriteAllText((Join-Path $fixture 'Cargo.lock'), '# test lock')
    & git -C $fixture add Cargo.lock
    & git -C $fixture -c user.name=Test -c user.email=test@example.invalid commit --quiet -m lock
    $gitHash = (& git -C $fixture rev-parse HEAD).Trim()

    $record = [ordered]@{
        package = 'fixture'; binary = 'fixture'; source_git = $gitHash
        source_tree_dirty = $false; firmware_version = 'test'; board_revision = 'Raylar v1.0'
        build_timestamp_utc = '2026-11-03T12:00:00Z'; build_profile = 'release'; runtime_crc32 = $null
    }
    $saved = Save-FirmwareBuild -WorkspaceRoot $fixture -ArtifactPath $ArtifactPath -Record $record `
        -BuildArguments @('build', '--release')
    $latest = Get-Content -LiteralPath $saved.RecordPath -Raw | ConvertFrom-Json
    Assert-True ($latest.flash_address -eq '0x08000000') 'Incorrect flash address.'
    Assert-True ($latest.artifacts.elf.sha256 -eq (Get-FileHash $ArtifactPath -Algorithm SHA256).Hash.ToLowerInvariant()) 'ELF hash differs.'
    Assert-True ($latest.artifacts.bin.bytes -eq (Get-Item $saved.BinPath).Length) 'BIN length differs.'
    Assert-True ($latest.cargo_arguments[1] -eq '--release') 'Build arguments lost.'
    Assert-True ($null -ne $latest.provenance.cargo_lock_sha256) 'Lockfile not captured.'
    Assert-True (-not (Get-FirmwareSourceChanges -WorkspaceRoot $fixture)) 'Archive output marks clean source dirty.'
    Expect-Failure { Save-FirmwareBuild -WorkspaceRoot $fixture -ArtifactPath $ArtifactPath -Record $record } 'already exists'

    # Verify validation rejects damaged BIN data, including an omitted segment.
    $badBin = Join-Path $fixture 'damaged.bin'
    $bytes = [IO.File]::ReadAllBytes($saved.BinPath)
    $bytes[0] = $bytes[0] -bxor 1
    [IO.File]::WriteAllBytes($badBin, $bytes)
    $layout = Get-FirmwareElfLayout -Path $saved.ElfPath
    Expect-Failure { Assert-FirmwareBinary -Layout $layout -Path $badBin } 'differs from ELF'
    [IO.File]::WriteAllBytes($badBin, [byte[]]@(0))
    Expect-Failure { Assert-FirmwareBinary -Layout $layout -Path $badBin } 'length'
    Remove-Item -LiteralPath $badBin

    # A new archive preserves the previous build and captures tracked + new source.
    [IO.File]::WriteAllText((Join-Path $fixture 'source.txt'), "changed`n")
    [IO.File]::WriteAllText((Join-Path $fixture 'new-source.txt'), "new source`n")
    Assert-True ([bool](Get-FirmwareSourceChanges -WorkspaceRoot $fixture)) 'Dirty source was missed.'
    $previousRecord = [IO.File]::ReadAllText((Join-Path $fixture $record.archive_record))
    $record2 = [ordered]@{}
    foreach ($entry in $record.GetEnumerator()) { $record2[$entry.Key] = $entry.Value }
    $record2.build_timestamp_utc = '2026-11-03T12:00:01Z'
    $record2.source_tree_dirty = $true
    $record2.source_git = "$gitHash-dirty"
    $second = Save-FirmwareBuild -WorkspaceRoot $fixture -ArtifactPath $ArtifactPath -Record $record2
    Assert-True ([IO.File]::ReadAllText((Join-Path $fixture $record.archive_record)) -ceq $previousRecord) 'Previous archive was changed.'
    Assert-True (Test-Path (Join-Path (Split-Path $second.ElfPath) 'source.patch')) 'Source patch missing.'
    $zip = [IO.Compression.ZipFile]::OpenRead((Join-Path (Split-Path $second.ElfPath) 'source-untracked.zip'))
    try { Assert-True ($zip.Entries.FullName -contains 'new-source.txt') 'Untracked source missing.' }
    finally { $zip.Dispose() }
    $record2.runtime_crc32 = '0x12345678'
    Write-FirmwareBuildRecord -Path $second.RecordPath -Record $record2 -WorkspaceRoot $fixture
    $archived = Get-Content (Join-Path $fixture $record2.archive_record) -Raw | ConvertFrom-Json
    $latest = Get-Content $second.RecordPath -Raw | ConvertFrom-Json
    Assert-True ($archived.runtime_crc32 -eq $latest.runtime_crc32 -and $latest.runtime_crc32 -eq '0x12345678') 'Runtime CRC record mismatch.'
    Write-Host 'PASS: archival, hashes, binary validation, no overwrite, provenance, latest pointer and runtime CRC.'
}
finally {
    if (Test-Path -LiteralPath $fixture) {
        $resolved = (Resolve-Path -LiteralPath $fixture).Path
        $expected = [IO.Path]::GetFullPath($fixture)
        if ($resolved -ne $expected -or -not $resolved.StartsWith([IO.Path]::GetTempPath(), [StringComparison]::OrdinalIgnoreCase)) {
            throw "Refusing to delete unexpected fixture path: $resolved"
        }
        Remove-Item -LiteralPath $resolved -Recurse -Force
    }
}
