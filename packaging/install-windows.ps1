$ErrorActionPreference = 'Stop'

$packageDirectory = $PSScriptRoot
$msiPath = Join-Path $packageDirectory 'dark-horse-windows-x64.msi'

if (-not (Test-Path $msiPath)) {
    throw "Missing installer: $msiPath"
}

$identity = [Security.Principal.WindowsIdentity]::GetCurrent()
$principal = New-Object Security.Principal.WindowsPrincipal($identity)
$adminRole = [Security.Principal.WindowsBuiltInRole]::Administrator
if (-not $principal.IsInRole($adminRole)) {
    Start-Process powershell.exe -Verb RunAs -ArgumentList "-NoProfile -ExecutionPolicy Bypass -File `"$PSCommandPath`"" -Wait
    exit
}

$install = Start-Process msiexec.exe -ArgumentList "/i `"$msiPath`" /passive /norestart" -Wait -PassThru
if ($install.ExitCode -notin @(0, 3010)) {
    throw "MSI installation failed with exit code $($install.ExitCode)."
}

$programFiles = [Environment]::GetFolderPath('ProgramFiles')
$programFilesX86 = [Environment]::GetFolderPath('ProgramFilesX86')
$candidates = @(
    (Join-Path $programFiles 'Dark Horse Browser'),
    (Join-Path $programFilesX86 'Dark Horse Browser')
)
$installDirectory = $candidates | Where-Object {
    Test-Path (Join-Path $_ 'dark-horse.exe')
} | Select-Object -First 1
if (-not $installDirectory) {
    throw 'The installer completed, but dark-horse.exe was not found in Program Files.'
}

$icon = Get-ChildItem -Path $installDirectory -Filter 'dark-horse-icon.ico' -Recurse -File | Select-Object -First 1
if (-not $icon) {
    throw 'The MSI installed the app but its bundled icon resource was not found.'
}
$iconPath = $icon.FullName
$executablePath = Join-Path $installDirectory 'dark-horse.exe'
$shell = New-Object -ComObject WScript.Shell
$shortcutLocations = @(
    (Join-Path ([Environment]::GetFolderPath('Programs')) 'Dark Horse Browser.lnk'),
    (Join-Path ([Environment]::GetFolderPath('DesktopDirectory')) 'Dark Horse Browser.lnk')
)
foreach ($shortcutPath in $shortcutLocations) {
    $shortcut = $shell.CreateShortcut($shortcutPath)
    $shortcut.TargetPath = $executablePath
    $shortcut.WorkingDirectory = $installDirectory
    $shortcut.IconLocation = "$iconPath,0"
    $shortcut.Save()
}

Write-Output 'Dark Horse Browser was installed. Start it from the Start menu or desktop shortcut.'
