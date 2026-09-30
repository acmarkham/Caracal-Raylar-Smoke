[CmdletBinding()]
param(
    [Parameter(ValueFromRemainingArguments = $true)]
    [string[]]$CargoArgs
)

$ErrorActionPreference = "Stop"
$workspaceRoot = (Resolve-Path (Join-Path $PSScriptRoot "..")).Path
$metadataConfigPath = Join-Path $workspaceRoot "firmware-metadata.json"

function Require-Command {
    param([Parameter(Mandatory = $true)][string]$Name)

    $command = Get-Command $Name -ErrorAction SilentlyContinue
    if ($null -eq $command) {
        throw "Required command '$Name' was not found on PATH."
    }
    return $command.Source
}

function Find-CargoArgument {
    param(
        [Parameter(Mandatory = $true)][string[]]$Arguments,
        [Parameter(Mandatory = $true)][string[]]$Names
    )

    for ($index = 0; $index -lt $Arguments.Count; $index++) {
        foreach ($name in $Names) {
            if ($Arguments[$index] -eq $name -and $index + 1 -lt $Arguments.Count) {
                return $Arguments[$index + 1]
            }
            if ($Arguments[$index].StartsWith("$name=")) {
                return $Arguments[$index].Substring($name.Length + 1)
            }
        }
    }
    return $null
}

Push-Location $workspaceRoot
try {
    $cargoPath = Require-Command "cargo"
    $gitPath = Require-Command "git"

    $package = Find-CargoArgument -Arguments $CargoArgs -Names @("--package", "-p")
    if (-not $package) {
        throw "The VS Code firmware runnable must include --package/-p."
    }

    $metadataText = & $cargoPath metadata --no-deps --format-version 1
    if ($LASTEXITCODE -ne 0) {
        throw "cargo metadata failed with exit code $LASTEXITCODE."
    }
    $metadata = $metadataText | ConvertFrom-Json
    $packageInfo = @($metadata.packages | Where-Object { $_.name -eq $package })
    if ($packageInfo.Count -ne 1) {
        throw "Expected exactly one workspace package named '$package'."
    }

    if (-not (Test-Path -LiteralPath $metadataConfigPath -PathType Leaf)) {
        throw "Firmware metadata config was not found at '$metadataConfigPath'."
    }
    $metadataConfig = Get-Content -LiteralPath $metadataConfigPath -Raw | ConvertFrom-Json
    if (-not $metadataConfig.boardRevision) {
        throw "firmware-metadata.json must define boardRevision."
    }

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
    $profile = if ($CargoArgs -contains "--release") {
        "release"
    }
    else {
        $selectedProfile = Find-CargoArgument -Arguments $CargoArgs -Names @("--profile")
        if ($selectedProfile) { $selectedProfile } else { "debug" }
    }
    $buildTimestamp = [DateTime]::UtcNow.ToString(
        "yyyy-MM-dd'T'HH:mm:ss'Z'",
        [Globalization.CultureInfo]::InvariantCulture
    )

    $env:RAYLAR_FIRMWARE_VERSION = [string]$packageInfo[0].version
    $env:RAYLAR_GIT_HASH = $gitIdentity
    $env:RAYLAR_GIT_DIRTY = $sourceDirty.ToString().ToLowerInvariant()
    $env:RAYLAR_BUILD_TIMESTAMP = $buildTimestamp
    $env:RAYLAR_BUILD_PROFILE = $profile
    $env:RAYLAR_BOARD_REVISION = [string]$metadataConfig.boardRevision
    $env:RAYLAR_BUILD_PACKAGE = $package

    & $cargoPath @CargoArgs
    if ($null -eq $LASTEXITCODE) {
        exit 0
    }
    exit $LASTEXITCODE
}
finally {
    Pop-Location
}
