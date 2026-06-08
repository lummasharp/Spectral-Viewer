$ErrorActionPreference = "Stop"

$projectRoot = Split-Path -Parent $PSScriptRoot
$installer = Join-Path $projectRoot "dist\SpectralViewer-Setup-0.3.0.exe"
$installDir = Join-Path $env:TEMP "SpectralViewerInstallerTest-$PID"
$uninstallKey = "HKCU:\Software\Microsoft\Windows\CurrentVersion\Uninstall\{E8403079-5B8A-47CE-AF70-315DF3322B98}_is1"
$pngKey = "HKCU:\Software\Classes\SystemFileAssociations\.png\shell\SpectralViewer"
$txtKey = "HKCU:\Software\Classes\SystemFileAssociations\.txt\shell\SpectralViewer"
$applicationKey = "HKCU:\Software\Classes\Applications\spectral-viewer.exe"
$progIdKey = "HKCU:\Software\Classes\SpectralViewer.Image"
$pngOpenWithKey = "HKCU:\Software\Classes\.png\OpenWithProgids"
$registeredApplicationsKey = "HKCU:\Software\RegisteredApplications"

if (-not (Test-Path -LiteralPath $installer)) {
    throw "Installer not found. Run .\scripts\build-installer.ps1 first."
}

function Invoke-Installer([string[]] $Arguments) {
    $process = Start-Process -FilePath $installer -ArgumentList $Arguments -Wait -PassThru
    if ($process.ExitCode) {
        throw "Installer exited with code $($process.ExitCode)."
    }
}

try {
    Invoke-Installer @(
        "/VERYSILENT",
        "/SUPPRESSMSGBOXES",
        "/NORESTART",
        "/TASKS=contextmenu",
        "/DIR=`"$installDir`""
    )

    if (-not (Test-Path -LiteralPath $pngKey)) {
        throw "The PNG context-menu entry was not installed."
    }
    if (Test-Path -LiteralPath $txtKey) {
        throw "An unsupported TXT context-menu entry was installed."
    }
    if (-not (Test-Path -LiteralPath $applicationKey)) {
        throw "Spectral Viewer was not registered as an Open With application."
    }
    if (-not (Test-Path -LiteralPath $progIdKey)) {
        throw "The Spectral Viewer image ProgID was not installed."
    }
    if ((Get-ItemProperty -LiteralPath $pngOpenWithKey)."SpectralViewer.Image" -ne "") {
        throw "The PNG OpenWithProgids handler was not installed."
    }
    if ((Get-ItemProperty -LiteralPath $registeredApplicationsKey)."Spectral Viewer" -ne "Software\Spectral Viewer\Capabilities") {
        throw "Spectral Viewer was not registered with Default Apps capabilities."
    }
    if (-not (Test-Path -LiteralPath (Join-Path $installDir "spectral-viewer.ico"))) {
        throw "The application icon was not installed."
    }

    $command = (Get-ItemProperty -LiteralPath "$pngKey\command")."(default)"
    if ($command -notlike "*spectral-viewer.exe*`"%1`"*") {
        throw "Unexpected context-menu command: $command"
    }
    $contextMenuIcon = (Get-ItemProperty -LiteralPath $pngKey).Icon
    if ($contextMenuIcon -notlike "*spectral-viewer.exe*") {
        throw "Unexpected context-menu icon: $contextMenuIcon"
    }

    Invoke-Installer @(
        "/VERYSILENT",
        "/SUPPRESSMSGBOXES",
        "/NORESTART",
        "/TASKS=!contextmenu",
        "/DIR=`"$installDir`""
    )

    if (Test-Path -LiteralPath $pngKey) {
        throw "The context-menu entry remained after the task was unchecked."
    }
    if (-not (Test-Path -LiteralPath $applicationKey)) {
        throw "Unchecking the separate context-menu task removed Open With registration."
    }
}
finally {
    if (Test-Path -LiteralPath $uninstallKey) {
        $uninstaller = (Get-ItemProperty -LiteralPath $uninstallKey).UninstallString.Trim('"')
        $process = Start-Process -FilePath $uninstaller -ArgumentList @(
            "/VERYSILENT",
            "/SUPPRESSMSGBOXES",
            "/NORESTART"
        ) -Wait -PassThru
        if ($process.ExitCode) {
            throw "Uninstaller exited with code $($process.ExitCode)."
        }
    }
}

if (Test-Path -LiteralPath $pngKey) {
    throw "The context-menu entry remained after uninstall."
}
if (Test-Path -LiteralPath $applicationKey) {
    throw "The Open With application registration remained after uninstall."
}
if (Test-Path -LiteralPath $progIdKey) {
    throw "The image ProgID remained after uninstall."
}
if ((Get-ItemProperty -LiteralPath $pngOpenWithKey -ErrorAction SilentlyContinue)."SpectralViewer.Image" -ne $null) {
    throw "The PNG OpenWithProgids value remained after uninstall."
}

Write-Host "Installer behavior verified."
