#define AppName "Spectral Viewer"
#define AppVersion "0.2.0"
#define AppPublisher "Spectral Viewer"
#define AppExeName "spectral-viewer.exe"

[Setup]
AppId={{E8403079-5B8A-47CE-AF70-315DF3322B98}
AppName={#AppName}
AppVersion={#AppVersion}
AppPublisher={#AppPublisher}
DefaultDirName={localappdata}\Programs\{#AppName}
DefaultGroupName={#AppName}
DisableProgramGroupPage=yes
PrivilegesRequired=lowest
OutputDir=..\dist
OutputBaseFilename=SpectralViewer-Setup-{#AppVersion}
Compression=lzma2
SolidCompression=yes
WizardStyle=modern
CloseApplications=yes
ChangesAssociations=yes
SetupLogging=yes
SetupIconFile=..\assets\icon.ico
UninstallDisplayIcon={app}\{#AppExeName}
ArchitecturesAllowed=x64compatible
ArchitecturesInstallIn64BitMode=x64compatible
VersionInfoVersion={#AppVersion}.0
VersionInfoProductName={#AppName}
VersionInfoProductVersion={#AppVersion}

[Tasks]
Name: "contextmenu"; Description: "Add ""Open with Spectral Viewer"" to supported image file context menus"; Flags: checkedonce

[Files]
Source: "..\target\release\{#AppExeName}"; DestDir: "{app}"; Flags: ignoreversion
Source: "..\assets\icon.ico"; DestDir: "{app}"; DestName: "spectral-viewer.ico"; Flags: ignoreversion
Source: "..\README.md"; DestDir: "{app}"; Flags: ignoreversion

[Icons]
Name: "{group}\{#AppName}"; Filename: "{app}\{#AppExeName}"; IconFilename: "{app}\spectral-viewer.ico"

[Registry]
; Register Spectral Viewer as an installed image application without changing user defaults.
Root: HKCU; Subkey: "Software\Classes\Applications\{#AppExeName}"; ValueType: string; ValueName: "FriendlyAppName"; ValueData: "{#AppName}"; Flags: uninsdeletekey
Root: HKCU; Subkey: "Software\Classes\Applications\{#AppExeName}\DefaultIcon"; ValueType: string; ValueName: ""; ValueData: """{app}\{#AppExeName}"""
Root: HKCU; Subkey: "Software\Classes\Applications\{#AppExeName}\shell\open\command"; ValueType: string; ValueName: ""; ValueData: """{app}\{#AppExeName}"" ""%1"""
Root: HKCU; Subkey: "Software\Classes\Applications\{#AppExeName}\SupportedTypes"; ValueType: string; ValueName: ".bmp"; ValueData: ""
Root: HKCU; Subkey: "Software\Classes\Applications\{#AppExeName}\SupportedTypes"; ValueType: string; ValueName: ".gif"; ValueData: ""
Root: HKCU; Subkey: "Software\Classes\Applications\{#AppExeName}\SupportedTypes"; ValueType: string; ValueName: ".ico"; ValueData: ""
Root: HKCU; Subkey: "Software\Classes\Applications\{#AppExeName}\SupportedTypes"; ValueType: string; ValueName: ".jpeg"; ValueData: ""
Root: HKCU; Subkey: "Software\Classes\Applications\{#AppExeName}\SupportedTypes"; ValueType: string; ValueName: ".jpg"; ValueData: ""
Root: HKCU; Subkey: "Software\Classes\Applications\{#AppExeName}\SupportedTypes"; ValueType: string; ValueName: ".pbm"; ValueData: ""
Root: HKCU; Subkey: "Software\Classes\Applications\{#AppExeName}\SupportedTypes"; ValueType: string; ValueName: ".pgm"; ValueData: ""
Root: HKCU; Subkey: "Software\Classes\Applications\{#AppExeName}\SupportedTypes"; ValueType: string; ValueName: ".png"; ValueData: ""
Root: HKCU; Subkey: "Software\Classes\Applications\{#AppExeName}\SupportedTypes"; ValueType: string; ValueName: ".pnm"; ValueData: ""
Root: HKCU; Subkey: "Software\Classes\Applications\{#AppExeName}\SupportedTypes"; ValueType: string; ValueName: ".ppm"; ValueData: ""
Root: HKCU; Subkey: "Software\Classes\Applications\{#AppExeName}\SupportedTypes"; ValueType: string; ValueName: ".qoi"; ValueData: ""
Root: HKCU; Subkey: "Software\Classes\Applications\{#AppExeName}\SupportedTypes"; ValueType: string; ValueName: ".tif"; ValueData: ""
Root: HKCU; Subkey: "Software\Classes\Applications\{#AppExeName}\SupportedTypes"; ValueType: string; ValueName: ".tiff"; ValueData: ""
Root: HKCU; Subkey: "Software\Classes\Applications\{#AppExeName}\SupportedTypes"; ValueType: string; ValueName: ".webp"; ValueData: ""

Root: HKCU; Subkey: "Software\Classes\SpectralViewer.Image"; ValueType: string; ValueName: ""; ValueData: "Spectral Viewer Image"; Flags: uninsdeletekey
Root: HKCU; Subkey: "Software\Classes\SpectralViewer.Image\DefaultIcon"; ValueType: string; ValueName: ""; ValueData: """{app}\{#AppExeName}"""
Root: HKCU; Subkey: "Software\Classes\SpectralViewer.Image\shell\open\command"; ValueType: string; ValueName: ""; ValueData: """{app}\{#AppExeName}"" ""%1"""

Root: HKCU; Subkey: "Software\Classes\.bmp\OpenWithProgids"; ValueType: string; ValueName: "SpectralViewer.Image"; ValueData: ""; Flags: uninsdeletevalue
Root: HKCU; Subkey: "Software\Classes\.gif\OpenWithProgids"; ValueType: string; ValueName: "SpectralViewer.Image"; ValueData: ""; Flags: uninsdeletevalue
Root: HKCU; Subkey: "Software\Classes\.ico\OpenWithProgids"; ValueType: string; ValueName: "SpectralViewer.Image"; ValueData: ""; Flags: uninsdeletevalue
Root: HKCU; Subkey: "Software\Classes\.jpeg\OpenWithProgids"; ValueType: string; ValueName: "SpectralViewer.Image"; ValueData: ""; Flags: uninsdeletevalue
Root: HKCU; Subkey: "Software\Classes\.jpg\OpenWithProgids"; ValueType: string; ValueName: "SpectralViewer.Image"; ValueData: ""; Flags: uninsdeletevalue
Root: HKCU; Subkey: "Software\Classes\.pbm\OpenWithProgids"; ValueType: string; ValueName: "SpectralViewer.Image"; ValueData: ""; Flags: uninsdeletevalue
Root: HKCU; Subkey: "Software\Classes\.pgm\OpenWithProgids"; ValueType: string; ValueName: "SpectralViewer.Image"; ValueData: ""; Flags: uninsdeletevalue
Root: HKCU; Subkey: "Software\Classes\.png\OpenWithProgids"; ValueType: string; ValueName: "SpectralViewer.Image"; ValueData: ""; Flags: uninsdeletevalue
Root: HKCU; Subkey: "Software\Classes\.pnm\OpenWithProgids"; ValueType: string; ValueName: "SpectralViewer.Image"; ValueData: ""; Flags: uninsdeletevalue
Root: HKCU; Subkey: "Software\Classes\.ppm\OpenWithProgids"; ValueType: string; ValueName: "SpectralViewer.Image"; ValueData: ""; Flags: uninsdeletevalue
Root: HKCU; Subkey: "Software\Classes\.qoi\OpenWithProgids"; ValueType: string; ValueName: "SpectralViewer.Image"; ValueData: ""; Flags: uninsdeletevalue
Root: HKCU; Subkey: "Software\Classes\.tif\OpenWithProgids"; ValueType: string; ValueName: "SpectralViewer.Image"; ValueData: ""; Flags: uninsdeletevalue
Root: HKCU; Subkey: "Software\Classes\.tiff\OpenWithProgids"; ValueType: string; ValueName: "SpectralViewer.Image"; ValueData: ""; Flags: uninsdeletevalue
Root: HKCU; Subkey: "Software\Classes\.webp\OpenWithProgids"; ValueType: string; ValueName: "SpectralViewer.Image"; ValueData: ""; Flags: uninsdeletevalue

Root: HKCU; Subkey: "Software\Spectral Viewer\Capabilities"; ValueType: string; ValueName: "ApplicationName"; ValueData: "{#AppName}"; Flags: uninsdeletekey
Root: HKCU; Subkey: "Software\Spectral Viewer\Capabilities"; ValueType: string; ValueName: "ApplicationDescription"; ValueData: "A fast, minimal desktop image viewer"
Root: HKCU; Subkey: "Software\Spectral Viewer\Capabilities"; ValueType: string; ValueName: "ApplicationIcon"; ValueData: """{app}\{#AppExeName}"""
Root: HKCU; Subkey: "Software\Spectral Viewer\Capabilities\FileAssociations"; ValueType: string; ValueName: ".bmp"; ValueData: "SpectralViewer.Image"
Root: HKCU; Subkey: "Software\Spectral Viewer\Capabilities\FileAssociations"; ValueType: string; ValueName: ".gif"; ValueData: "SpectralViewer.Image"
Root: HKCU; Subkey: "Software\Spectral Viewer\Capabilities\FileAssociations"; ValueType: string; ValueName: ".ico"; ValueData: "SpectralViewer.Image"
Root: HKCU; Subkey: "Software\Spectral Viewer\Capabilities\FileAssociations"; ValueType: string; ValueName: ".jpeg"; ValueData: "SpectralViewer.Image"
Root: HKCU; Subkey: "Software\Spectral Viewer\Capabilities\FileAssociations"; ValueType: string; ValueName: ".jpg"; ValueData: "SpectralViewer.Image"
Root: HKCU; Subkey: "Software\Spectral Viewer\Capabilities\FileAssociations"; ValueType: string; ValueName: ".pbm"; ValueData: "SpectralViewer.Image"
Root: HKCU; Subkey: "Software\Spectral Viewer\Capabilities\FileAssociations"; ValueType: string; ValueName: ".pgm"; ValueData: "SpectralViewer.Image"
Root: HKCU; Subkey: "Software\Spectral Viewer\Capabilities\FileAssociations"; ValueType: string; ValueName: ".png"; ValueData: "SpectralViewer.Image"
Root: HKCU; Subkey: "Software\Spectral Viewer\Capabilities\FileAssociations"; ValueType: string; ValueName: ".pnm"; ValueData: "SpectralViewer.Image"
Root: HKCU; Subkey: "Software\Spectral Viewer\Capabilities\FileAssociations"; ValueType: string; ValueName: ".ppm"; ValueData: "SpectralViewer.Image"
Root: HKCU; Subkey: "Software\Spectral Viewer\Capabilities\FileAssociations"; ValueType: string; ValueName: ".qoi"; ValueData: "SpectralViewer.Image"
Root: HKCU; Subkey: "Software\Spectral Viewer\Capabilities\FileAssociations"; ValueType: string; ValueName: ".tif"; ValueData: "SpectralViewer.Image"
Root: HKCU; Subkey: "Software\Spectral Viewer\Capabilities\FileAssociations"; ValueType: string; ValueName: ".tiff"; ValueData: "SpectralViewer.Image"
Root: HKCU; Subkey: "Software\Spectral Viewer\Capabilities\FileAssociations"; ValueType: string; ValueName: ".webp"; ValueData: "SpectralViewer.Image"
Root: HKCU; Subkey: "Software\RegisteredApplications"; ValueType: string; ValueName: "{#AppName}"; ValueData: "Software\Spectral Viewer\Capabilities"; Flags: uninsdeletevalue

; Optional separate context-menu command.
Root: HKCU; Subkey: "Software\Classes\SystemFileAssociations\.bmp\shell\SpectralViewer"; ValueType: string; ValueName: ""; ValueData: "Open with Spectral Viewer"; Flags: uninsdeletekey; Tasks: contextmenu
Root: HKCU; Subkey: "Software\Classes\SystemFileAssociations\.bmp\shell\SpectralViewer"; ValueType: string; ValueName: "Icon"; ValueData: """{app}\{#AppExeName}"""; Tasks: contextmenu
Root: HKCU; Subkey: "Software\Classes\SystemFileAssociations\.bmp\shell\SpectralViewer\command"; ValueType: string; ValueName: ""; ValueData: """{app}\{#AppExeName}"" ""%1"""; Tasks: contextmenu

Root: HKCU; Subkey: "Software\Classes\SystemFileAssociations\.gif\shell\SpectralViewer"; ValueType: string; ValueName: ""; ValueData: "Open with Spectral Viewer"; Flags: uninsdeletekey; Tasks: contextmenu
Root: HKCU; Subkey: "Software\Classes\SystemFileAssociations\.gif\shell\SpectralViewer"; ValueType: string; ValueName: "Icon"; ValueData: """{app}\{#AppExeName}"""; Tasks: contextmenu
Root: HKCU; Subkey: "Software\Classes\SystemFileAssociations\.gif\shell\SpectralViewer\command"; ValueType: string; ValueName: ""; ValueData: """{app}\{#AppExeName}"" ""%1"""; Tasks: contextmenu

Root: HKCU; Subkey: "Software\Classes\SystemFileAssociations\.ico\shell\SpectralViewer"; ValueType: string; ValueName: ""; ValueData: "Open with Spectral Viewer"; Flags: uninsdeletekey; Tasks: contextmenu
Root: HKCU; Subkey: "Software\Classes\SystemFileAssociations\.ico\shell\SpectralViewer"; ValueType: string; ValueName: "Icon"; ValueData: """{app}\{#AppExeName}"""; Tasks: contextmenu
Root: HKCU; Subkey: "Software\Classes\SystemFileAssociations\.ico\shell\SpectralViewer\command"; ValueType: string; ValueName: ""; ValueData: """{app}\{#AppExeName}"" ""%1"""; Tasks: contextmenu

Root: HKCU; Subkey: "Software\Classes\SystemFileAssociations\.jpeg\shell\SpectralViewer"; ValueType: string; ValueName: ""; ValueData: "Open with Spectral Viewer"; Flags: uninsdeletekey; Tasks: contextmenu
Root: HKCU; Subkey: "Software\Classes\SystemFileAssociations\.jpeg\shell\SpectralViewer"; ValueType: string; ValueName: "Icon"; ValueData: """{app}\{#AppExeName}"""; Tasks: contextmenu
Root: HKCU; Subkey: "Software\Classes\SystemFileAssociations\.jpeg\shell\SpectralViewer\command"; ValueType: string; ValueName: ""; ValueData: """{app}\{#AppExeName}"" ""%1"""; Tasks: contextmenu

Root: HKCU; Subkey: "Software\Classes\SystemFileAssociations\.jpg\shell\SpectralViewer"; ValueType: string; ValueName: ""; ValueData: "Open with Spectral Viewer"; Flags: uninsdeletekey; Tasks: contextmenu
Root: HKCU; Subkey: "Software\Classes\SystemFileAssociations\.jpg\shell\SpectralViewer"; ValueType: string; ValueName: "Icon"; ValueData: """{app}\{#AppExeName}"""; Tasks: contextmenu
Root: HKCU; Subkey: "Software\Classes\SystemFileAssociations\.jpg\shell\SpectralViewer\command"; ValueType: string; ValueName: ""; ValueData: """{app}\{#AppExeName}"" ""%1"""; Tasks: contextmenu

Root: HKCU; Subkey: "Software\Classes\SystemFileAssociations\.pbm\shell\SpectralViewer"; ValueType: string; ValueName: ""; ValueData: "Open with Spectral Viewer"; Flags: uninsdeletekey; Tasks: contextmenu
Root: HKCU; Subkey: "Software\Classes\SystemFileAssociations\.pbm\shell\SpectralViewer"; ValueType: string; ValueName: "Icon"; ValueData: """{app}\{#AppExeName}"""; Tasks: contextmenu
Root: HKCU; Subkey: "Software\Classes\SystemFileAssociations\.pbm\shell\SpectralViewer\command"; ValueType: string; ValueName: ""; ValueData: """{app}\{#AppExeName}"" ""%1"""; Tasks: contextmenu

Root: HKCU; Subkey: "Software\Classes\SystemFileAssociations\.pgm\shell\SpectralViewer"; ValueType: string; ValueName: ""; ValueData: "Open with Spectral Viewer"; Flags: uninsdeletekey; Tasks: contextmenu
Root: HKCU; Subkey: "Software\Classes\SystemFileAssociations\.pgm\shell\SpectralViewer"; ValueType: string; ValueName: "Icon"; ValueData: """{app}\{#AppExeName}"""; Tasks: contextmenu
Root: HKCU; Subkey: "Software\Classes\SystemFileAssociations\.pgm\shell\SpectralViewer\command"; ValueType: string; ValueName: ""; ValueData: """{app}\{#AppExeName}"" ""%1"""; Tasks: contextmenu

Root: HKCU; Subkey: "Software\Classes\SystemFileAssociations\.png\shell\SpectralViewer"; ValueType: string; ValueName: ""; ValueData: "Open with Spectral Viewer"; Flags: uninsdeletekey; Tasks: contextmenu
Root: HKCU; Subkey: "Software\Classes\SystemFileAssociations\.png\shell\SpectralViewer"; ValueType: string; ValueName: "Icon"; ValueData: """{app}\{#AppExeName}"""; Tasks: contextmenu
Root: HKCU; Subkey: "Software\Classes\SystemFileAssociations\.png\shell\SpectralViewer\command"; ValueType: string; ValueName: ""; ValueData: """{app}\{#AppExeName}"" ""%1"""; Tasks: contextmenu

Root: HKCU; Subkey: "Software\Classes\SystemFileAssociations\.pnm\shell\SpectralViewer"; ValueType: string; ValueName: ""; ValueData: "Open with Spectral Viewer"; Flags: uninsdeletekey; Tasks: contextmenu
Root: HKCU; Subkey: "Software\Classes\SystemFileAssociations\.pnm\shell\SpectralViewer"; ValueType: string; ValueName: "Icon"; ValueData: """{app}\{#AppExeName}"""; Tasks: contextmenu
Root: HKCU; Subkey: "Software\Classes\SystemFileAssociations\.pnm\shell\SpectralViewer\command"; ValueType: string; ValueName: ""; ValueData: """{app}\{#AppExeName}"" ""%1"""; Tasks: contextmenu

Root: HKCU; Subkey: "Software\Classes\SystemFileAssociations\.ppm\shell\SpectralViewer"; ValueType: string; ValueName: ""; ValueData: "Open with Spectral Viewer"; Flags: uninsdeletekey; Tasks: contextmenu
Root: HKCU; Subkey: "Software\Classes\SystemFileAssociations\.ppm\shell\SpectralViewer"; ValueType: string; ValueName: "Icon"; ValueData: """{app}\{#AppExeName}"""; Tasks: contextmenu
Root: HKCU; Subkey: "Software\Classes\SystemFileAssociations\.ppm\shell\SpectralViewer\command"; ValueType: string; ValueName: ""; ValueData: """{app}\{#AppExeName}"" ""%1"""; Tasks: contextmenu

Root: HKCU; Subkey: "Software\Classes\SystemFileAssociations\.qoi\shell\SpectralViewer"; ValueType: string; ValueName: ""; ValueData: "Open with Spectral Viewer"; Flags: uninsdeletekey; Tasks: contextmenu
Root: HKCU; Subkey: "Software\Classes\SystemFileAssociations\.qoi\shell\SpectralViewer"; ValueType: string; ValueName: "Icon"; ValueData: """{app}\{#AppExeName}"""; Tasks: contextmenu
Root: HKCU; Subkey: "Software\Classes\SystemFileAssociations\.qoi\shell\SpectralViewer\command"; ValueType: string; ValueName: ""; ValueData: """{app}\{#AppExeName}"" ""%1"""; Tasks: contextmenu

Root: HKCU; Subkey: "Software\Classes\SystemFileAssociations\.tif\shell\SpectralViewer"; ValueType: string; ValueName: ""; ValueData: "Open with Spectral Viewer"; Flags: uninsdeletekey; Tasks: contextmenu
Root: HKCU; Subkey: "Software\Classes\SystemFileAssociations\.tif\shell\SpectralViewer"; ValueType: string; ValueName: "Icon"; ValueData: """{app}\{#AppExeName}"""; Tasks: contextmenu
Root: HKCU; Subkey: "Software\Classes\SystemFileAssociations\.tif\shell\SpectralViewer\command"; ValueType: string; ValueName: ""; ValueData: """{app}\{#AppExeName}"" ""%1"""; Tasks: contextmenu

Root: HKCU; Subkey: "Software\Classes\SystemFileAssociations\.tiff\shell\SpectralViewer"; ValueType: string; ValueName: ""; ValueData: "Open with Spectral Viewer"; Flags: uninsdeletekey; Tasks: contextmenu
Root: HKCU; Subkey: "Software\Classes\SystemFileAssociations\.tiff\shell\SpectralViewer"; ValueType: string; ValueName: "Icon"; ValueData: """{app}\{#AppExeName}"""; Tasks: contextmenu
Root: HKCU; Subkey: "Software\Classes\SystemFileAssociations\.tiff\shell\SpectralViewer\command"; ValueType: string; ValueName: ""; ValueData: """{app}\{#AppExeName}"" ""%1"""; Tasks: contextmenu

Root: HKCU; Subkey: "Software\Classes\SystemFileAssociations\.webp\shell\SpectralViewer"; ValueType: string; ValueName: ""; ValueData: "Open with Spectral Viewer"; Flags: uninsdeletekey; Tasks: contextmenu
Root: HKCU; Subkey: "Software\Classes\SystemFileAssociations\.webp\shell\SpectralViewer"; ValueType: string; ValueName: "Icon"; ValueData: """{app}\{#AppExeName}"""; Tasks: contextmenu
Root: HKCU; Subkey: "Software\Classes\SystemFileAssociations\.webp\shell\SpectralViewer\command"; ValueType: string; ValueName: ""; ValueData: """{app}\{#AppExeName}"" ""%1"""; Tasks: contextmenu

[Run]
Filename: "{app}\{#AppExeName}"; Description: "Launch Spectral Viewer"; Flags: nowait postinstall skipifsilent

[Code]
procedure RemoveContextMenuEntries;
begin
  RegDeleteKeyIncludingSubkeys(HKCU, 'Software\Classes\SystemFileAssociations\.bmp\shell\SpectralViewer');
  RegDeleteKeyIncludingSubkeys(HKCU, 'Software\Classes\SystemFileAssociations\.gif\shell\SpectralViewer');
  RegDeleteKeyIncludingSubkeys(HKCU, 'Software\Classes\SystemFileAssociations\.ico\shell\SpectralViewer');
  RegDeleteKeyIncludingSubkeys(HKCU, 'Software\Classes\SystemFileAssociations\.jpeg\shell\SpectralViewer');
  RegDeleteKeyIncludingSubkeys(HKCU, 'Software\Classes\SystemFileAssociations\.jpg\shell\SpectralViewer');
  RegDeleteKeyIncludingSubkeys(HKCU, 'Software\Classes\SystemFileAssociations\.pbm\shell\SpectralViewer');
  RegDeleteKeyIncludingSubkeys(HKCU, 'Software\Classes\SystemFileAssociations\.pgm\shell\SpectralViewer');
  RegDeleteKeyIncludingSubkeys(HKCU, 'Software\Classes\SystemFileAssociations\.png\shell\SpectralViewer');
  RegDeleteKeyIncludingSubkeys(HKCU, 'Software\Classes\SystemFileAssociations\.pnm\shell\SpectralViewer');
  RegDeleteKeyIncludingSubkeys(HKCU, 'Software\Classes\SystemFileAssociations\.ppm\shell\SpectralViewer');
  RegDeleteKeyIncludingSubkeys(HKCU, 'Software\Classes\SystemFileAssociations\.qoi\shell\SpectralViewer');
  RegDeleteKeyIncludingSubkeys(HKCU, 'Software\Classes\SystemFileAssociations\.tif\shell\SpectralViewer');
  RegDeleteKeyIncludingSubkeys(HKCU, 'Software\Classes\SystemFileAssociations\.tiff\shell\SpectralViewer');
  RegDeleteKeyIncludingSubkeys(HKCU, 'Software\Classes\SystemFileAssociations\.webp\shell\SpectralViewer');
end;

procedure CurStepChanged(CurStep: TSetupStep);
begin
  if (CurStep = ssPostInstall) and not WizardIsTaskSelected('contextmenu') then
    RemoveContextMenuEntries;
end;
