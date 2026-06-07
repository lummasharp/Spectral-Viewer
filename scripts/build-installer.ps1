$ErrorActionPreference = "Stop"

$projectRoot = Split-Path -Parent $PSScriptRoot
$isccCandidates = @(
    (Join-Path ${env:ProgramFiles(x86)} "Inno Setup 6\ISCC.exe"),
    (Join-Path $env:ProgramFiles "Inno Setup 6\ISCC.exe"),
    (Join-Path $env:LOCALAPPDATA "Programs\Inno Setup 6\ISCC.exe")
)
$iscc = $isccCandidates | Where-Object { $_ -and (Test-Path -LiteralPath $_) } | Select-Object -First 1

if (-not $iscc) {
    throw "Inno Setup 6 was not found. Install it with: winget install --id JRSoftware.InnoSetup -e"
}

Push-Location $projectRoot
try {
    & "$HOME\.cargo\bin\cargo.exe" build --release
    if ($LASTEXITCODE) {
        exit $LASTEXITCODE
    }

    & $iscc "installer\spectral-viewer.iss"
    if ($LASTEXITCODE) {
        exit $LASTEXITCODE
    }
}
finally {
    Pop-Location
}

Write-Host "Installer created in $projectRoot\dist"
