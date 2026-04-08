#ifndef MyAppVersion
  #define MyAppVersion "0.1.0"
#endif

#ifndef MySourceDir
  #define MySourceDir "..\..\dist\rusty-breakout-windows-x64"
#endif

#ifndef MyOutputDir
  #define MyOutputDir "..\..\dist"
#endif

[Setup]
AppId={{F412C7A4-8C37-4708-B972-65AFD564B44D}
AppName=Rusty Breakout
AppVersion={#MyAppVersion}
AppPublisher=rusty-breakout contributors
AppPublisherURL=https://github.com/rajandiappan/rusty-breakout
AppSupportURL=https://github.com/rajandiappan/rusty-breakout/issues
AppUpdatesURL=https://github.com/rajandiappan/rusty-breakout/releases
DefaultDirName={localappdata}\Programs\Rusty Breakout
DefaultGroupName=Rusty Breakout
DisableProgramGroupPage=yes
LicenseFile=..\..\LICENSE
OutputDir={#MyOutputDir}
OutputBaseFilename=rusty-breakout-setup-x64
Compression=lzma
SolidCompression=yes
WizardStyle=modern
PrivilegesRequired=lowest
ArchitecturesAllowed=x64compatible
ArchitecturesInstallIn64BitMode=x64compatible
UninstallDisplayIcon={app}\breakout.exe

[Tasks]
Name: "desktopicon"; Description: "Create a desktop shortcut"; GroupDescription: "Additional icons:"

[Files]
Source: "{#MySourceDir}\*"; DestDir: "{app}"; Flags: ignoreversion recursesubdirs createallsubdirs

[Icons]
Name: "{group}\Rusty Breakout"; Filename: "{app}\breakout.exe"
Name: "{group}\Uninstall Rusty Breakout"; Filename: "{uninstallexe}"
Name: "{autodesktop}\Rusty Breakout"; Filename: "{app}\breakout.exe"; Tasks: desktopicon

[Run]
Filename: "{app}\breakout.exe"; Description: "Launch Rusty Breakout"; Flags: nowait postinstall skipifsilent
