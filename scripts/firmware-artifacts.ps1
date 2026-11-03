# Shared by the bounded firmware loop and the Cargo/VS Code probe runner.
function Write-FirmwareJson {
    param([string]$Path, [System.Collections.IDictionary]$Record)
    [System.IO.File]::WriteAllText(
        $Path, ($Record | ConvertTo-Json -Depth 10) + [Environment]::NewLine,
        (New-Object System.Text.UTF8Encoding($false))
    )
}

function Write-FirmwareBuildRecord {
    param([string]$Path, [System.Collections.IDictionary]$Record, [string]$WorkspaceRoot)
    # The latest pointer and the archived record always describe the same image.
    if ($Record.archive_record) {
        Write-FirmwareJson -Path (Join-Path $WorkspaceRoot $Record.archive_record) -Record $Record
    }
    Write-FirmwareJson -Path $Path -Record $Record
}

function Get-FirmwareSourceChanges {
    param([string]$WorkspaceRoot)
    $changes = & git -C $WorkspaceRoot status --porcelain --untracked-files=all -- . ':(exclude)firmware-builds'
    if ($LASTEXITCODE -ne 0) { throw "Unable to inspect firmware source changes." }
    return $changes
}

function Get-FirmwareElfLayout {
    param([string]$Path)
    $bytes = [System.IO.File]::ReadAllBytes($Path)
    if ($bytes.Length -lt 52 -or $bytes[0] -ne 0x7f -or
        [Text.Encoding]::ASCII.GetString($bytes, 1, 3) -ne 'ELF' -or
        $bytes[4] -ne 1 -or $bytes[5] -ne 1 -or
        [BitConverter]::ToUInt16($bytes, 18) -ne 40) {
        throw "Expected a little-endian 32-bit ARM ELF: $Path"
    }
    $headerOffset = [BitConverter]::ToUInt32($bytes, 28)
    $headerSize = [BitConverter]::ToUInt16($bytes, 42)
    $headerCount = [BitConverter]::ToUInt16($bytes, 44)
    if ($headerSize -lt 32) { throw "Invalid ELF program header size." }
    $segments = @(
        for ($index = 0; $index -lt $headerCount; $index++) {
            $offset = [long]$headerOffset + [long]$index * $headerSize
            if ($offset + 32 -gt $bytes.Length) { throw "Truncated ELF program headers." }
            $type = [BitConverter]::ToUInt32($bytes, $offset)
            $size = [BitConverter]::ToUInt32($bytes, $offset + 16)
            if ($type -eq 1 -and $size -gt 0) {
                $fileOffset = [BitConverter]::ToUInt32($bytes, $offset + 4)
                $address = [BitConverter]::ToUInt32($bytes, $offset + 12)
                if ([long]$fileOffset + $size -gt $bytes.Length) { throw "Truncated ELF load segment." }
                [pscustomobject]@{ Address = [long]$address; Offset = [long]$fileOffset; Size = [long]$size }
            }
        }
    )
    if ($segments.Count -eq 0) { throw "ELF has no loadable data." }
    $base = ($segments | Measure-Object -Property Address -Minimum).Minimum
    $end = ($segments | ForEach-Object { $_.Address + $_.Size } | Measure-Object -Maximum).Maximum
    # STM32U5 nonsecure and secure flash aliases. Reject accidental RAM images.
    if ($base -lt 0x08000000 -or $end -gt 0x10000000) { throw "ELF load image is outside STM32 flash." }
    return [pscustomobject]@{ Base = [long]$base; End = [long]$end; Segments = $segments; Bytes = $bytes }
}

function Assert-FirmwareBinary {
    param($Layout, [string]$Path)
    $binary = [System.IO.File]::ReadAllBytes($Path)
    if ($binary.Length -ne $Layout.End - $Layout.Base) { throw "BIN length does not match the ELF flash image." }
    foreach ($segment in $Layout.Segments) {
        $start = $segment.Address - $Layout.Base
        for ($index = 0; $index -lt $segment.Size; $index++) {
            if ($binary[$start + $index] -ne $Layout.Bytes[$segment.Offset + $index]) {
                throw "BIN differs from ELF load data at flash offset $($start + $index)."
            }
        }
    }
}

function Save-FirmwareBuild {
    param(
        [Parameter(Mandatory = $true)][string]$WorkspaceRoot,
        [Parameter(Mandatory = $true)][string]$ArtifactPath,
        [Parameter(Mandatory = $true)][System.Collections.IDictionary]$Record,
        [string[]]$BuildArguments = @(),
        [string]$Chip = 'STM32U595VJ'
    )
    $objcopy = @('rust-objcopy', 'llvm-objcopy', 'arm-none-eabi-objcopy') |
        ForEach-Object { Get-Command $_ -ErrorAction SilentlyContinue } | Select-Object -First 1
    if (-not $objcopy) {
        throw "Install cargo-binutils and rustup component llvm-tools-preview to archive firmware ELF/BIN files."
    }
    $safePackage = $Record.package -replace '[^A-Za-z0-9_.-]', '_'
    $safeBinary = $Record.binary -replace '[^A-Za-z0-9_.-]', '_'
    $safeProfile = $Record.build_profile -replace '[^A-Za-z0-9_.-]', '_'
    $sha = (Get-FileHash -LiteralPath $ArtifactPath -Algorithm SHA256).Hash.ToLowerInvariant()
    $stamp = ([DateTimeOffset]::Parse($Record.build_timestamp_utc)).UtcDateTime.ToString('yyyyMMddTHHmmssZ')
    $gitShort = ($Record.source_git -split '-')[0].Substring(0, 12)
    $dirtySuffix = if ($Record.source_tree_dirty) { '-dirty' } else { '' }
    $archiveRelative = "firmware-builds/$safePackage/$stamp-$gitShort$dirtySuffix-$safeBinary-$safeProfile-$($sha.Substring(0, 12))"
    $archive = Join-Path $WorkspaceRoot $archiveRelative
    if (Test-Path -LiteralPath $archive) { throw "Firmware archive already exists; refusing to overwrite '$archive'." }
    New-Item -ItemType Directory -Path $archive -Force | Out-Null
    $elfPath = Join-Path $archive "$safeBinary.elf"
    $binPath = Join-Path $archive "$safeBinary.bin"
    Copy-Item -LiteralPath $ArtifactPath -Destination $elfPath
    if ((Get-FileHash -LiteralPath $elfPath -Algorithm SHA256).Hash.ToLowerInvariant() -ne $sha) {
        throw 'Firmware changed while it was being archived; build again.'
    }
    $layout = Get-FirmwareElfLayout -Path $elfPath
    & $objcopy.Source -O binary $elfPath $binPath
    if ($LASTEXITCODE -ne 0) { throw "objcopy failed; firmware archive is incomplete: $archive" }
    Assert-FirmwareBinary -Layout $layout -Path $binPath

    $Record.schema_version = 2
    $Record.artifact_sha256 = $sha # Preserve the original ELF hash field for existing analysis tools.
    $Record.chip = $Chip
    $Record.flash_address = '0x{0:X8}' -f $layout.Base
    $Record.flash_image_bytes = $layout.End - $layout.Base
    $Record.archive_record = "$archiveRelative/build.json"
    $Record.cargo_arguments = @($BuildArguments)
    $Record.rustc_version = (& rustc --version --verbose) -join "`n"
    if ($LASTEXITCODE -ne 0) { throw "Unable to record rustc version." }
    $Record.cargo_version = [string](& cargo --version)
    if ($LASTEXITCODE -ne 0) { throw "Unable to record Cargo version." }
    $Record.objcopy_version = [string](@(& $objcopy.Source --version)[0])
    if ($LASTEXITCODE -ne 0) { throw "Unable to record objcopy version." }
    $Record.build_environment = [ordered]@{}
    foreach ($name in @('RUSTFLAGS', 'CARGO_ENCODED_RUSTFLAGS', 'CARGO_INCREMENTAL', 'DEFMT_LOG')) {
        $Record.build_environment[$name] = [Environment]::GetEnvironmentVariable($name, 'Process')
    }
    $Record.artifacts = [ordered]@{}
    foreach ($kind in @('elf', 'bin')) {
        $file = Join-Path $archive "$safeBinary.$kind"
        $Record.artifacts[$kind] = [ordered]@{
            path = "$archiveRelative/$safeBinary.$kind"
            sha256 = (Get-FileHash -LiteralPath $file -Algorithm SHA256).Hash.ToLowerInvariant()
            bytes = (Get-Item -LiteralPath $file).Length
        }
    }
    $Record.provenance = [ordered]@{}
    $lockPath = Join-Path $WorkspaceRoot 'Cargo.lock'
    if (Test-Path -LiteralPath $lockPath) {
        Copy-Item -LiteralPath $lockPath -Destination (Join-Path $archive 'Cargo.lock')
        $Record.provenance.cargo_lock_sha256 = (Get-FileHash -LiteralPath $lockPath -Algorithm SHA256).Hash.ToLowerInvariant()
    }
    $patch = @(& git -C $WorkspaceRoot diff HEAD --binary -- . ':(exclude)firmware-builds')
    if ($LASTEXITCODE -ne 0) { throw "Unable to capture source changes." }
    if ($patch.Count -gt 0) {
        $patchPath = Join-Path $archive 'source.patch'
        [IO.File]::WriteAllText($patchPath, ($patch -join "`n") + "`n", (New-Object Text.UTF8Encoding($false)))
        $Record.provenance.source_patch_sha256 = (Get-FileHash -LiteralPath $patchPath -Algorithm SHA256).Hash.ToLowerInvariant()
    }
    $untracked = @(& git -C $WorkspaceRoot -c core.quotepath=false ls-files --others --exclude-standard -- . ':(exclude)firmware-builds')
    if ($LASTEXITCODE -ne 0) { throw "Unable to enumerate untracked source files." }
    if ($untracked.Count -gt 0) {
        Add-Type -AssemblyName System.IO.Compression, System.IO.Compression.FileSystem
        $zipPath = Join-Path $archive 'source-untracked.zip'
        $zip = [IO.Compression.ZipFile]::Open($zipPath, [IO.Compression.ZipArchiveMode]::Create)
        try {
            foreach ($file in $untracked) {
                [void][IO.Compression.ZipFileExtensions]::CreateEntryFromFile($zip, (Join-Path $WorkspaceRoot $file), $file)
            }
        }
        finally { $zip.Dispose() }
        $Record.provenance.untracked_source_sha256 = (Get-FileHash -LiteralPath $zipPath -Algorithm SHA256).Hash.ToLowerInvariant()
    }
    $instructions = @"
$($Record.package) $($Record.firmware_version) - $($Record.board_revision)
Built $($Record.build_timestamp_utc) from $($Record.source_git)

STM32CubeProgrammer: select the board's USB DFU device, connect, and open
$safeBinary.bin in Erasing & Programming. Set start address to
$($Record.flash_address), enable verification, and program. Start the application
after leaving the board's bootloader mode. Alternatively open $safeBinary.elf;
the ELF carries its own load addresses. Use the board's documented BOOT0/reset
procedure to enter USB DFU; this image does not add a USB DFU bootloader.

BIN SHA256: $($Record.artifacts.bin.sha256)
ELF SHA256: $($Record.artifacts.elf.sha256)
Intended MCU: $Chip. This is the application image for the recorded board and
security configuration; option bytes are not changed by this package.
"@
    [IO.File]::WriteAllText((Join-Path $archive 'FLASHING.txt'), $instructions + "`n")
    $recordPath = Join-Path $WorkspaceRoot "firmware-builds/$safePackage.json"
    Write-FirmwareBuildRecord -Path $recordPath -Record $Record -WorkspaceRoot $WorkspaceRoot
    Write-Host "    Firmware archive: $archive"
    return [pscustomobject]@{ RecordPath = $recordPath; ElfPath = $elfPath; BinPath = $binPath }
}
