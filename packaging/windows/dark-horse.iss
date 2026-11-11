#define AppName "Dark Horse Browser"
#ifndef AppVersion
  #define AppVersion "0.1.0"
#endif

[Setup]
AppId={{6722D05B-5029-48F9-9BC5-399A25B974A1}
AppName={#AppName}
AppVersion={#AppVersion}
AppPublisher=Dark Horse Project
DefaultDirName={localappdata}\Programs\Dark Horse Browser
DefaultGroupName=Dark Horse Browser
PrivilegesRequired=lowest
ArchitecturesAllowed=x64
ArchitecturesInstallIn64BitMode=x64
OutputDir=..\..\target\installer
OutputBaseFilename=dark-horse-windows-x64-setup
SetupIconFile=..\..\assets\dark-horse-icon.ico
UninstallDisplayIcon={app}\dark-horse-icon.ico
Compression=lzma2
SolidCompression=yes
WizardStyle=modern

[Tasks]
Name: "desktopicon"; Description: "Create a desktop shortcut"; GroupDescription: "Additional icons:"; Flags: unchecked

[Files]
Source: "..\..\target\release\dark-horse.exe"; DestDir: "{app}"; Flags: ignoreversion
Source: "..\..\assets\dark-horse-icon.ico"; DestDir: "{app}"; Flags: ignoreversion

[Icons]
Name: "{autoprograms}\Dark Horse Browser"; Filename: "{app}\dark-horse.exe"; WorkingDir: "{app}"; IconFilename: "{app}\dark-horse-icon.ico"
Name: "{autodesktop}\Dark Horse Browser"; Filename: "{app}\dark-horse.exe"; WorkingDir: "{app}"; IconFilename: "{app}\dark-horse-icon.ico"; Tasks: desktopicon

[Run]
Filename: "{app}\dark-horse.exe"; Description: "Launch Dark Horse Browser"; Flags: nowait postinstall skipifsilent
