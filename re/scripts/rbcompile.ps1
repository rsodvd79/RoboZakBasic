# Black-box del compilatore roboBASIC: usa rbv2.ocx registrato (COM a 32 bit).
# Uso: powershell -File rbcompile.ps1 <file.bas|cartella> [-ConType 4] [-ConID 2]
# Per ogni .bas scrive <nome>.hex (ObjCode in hex) e stampa i metadati.
param([Parameter(Mandatory)][string]$Path, [int]$ConType = 4, [int]$ConID = 2)
if ([Environment]::Is64BitProcess) {
    & "$env:WINDIR\SysWOW64\WindowsPowerShell\v1.0\powershell.exe" -NoProfile -File $PSCommandPath $Path -ConType $ConType -ConID $ConID
    return
}
$o = New-Object -ComObject RBv2.ROBOBASICv2
$o.Con_Type = $ConType
$o.Con_ID = $ConID
$files = if (Test-Path $Path -PathType Container) { Get-ChildItem $Path -Filter *.bas } else { Get-Item $Path }
foreach ($f in $files) {
    $src = [IO.File]::ReadAllText($f.FullName)
    $o.Compile($src)
    $hex = ''
    if ($o.Bas_Error -eq 0) {
        $hex = ($o.ObjCode | ForEach-Object { '{0:X2}' -f [int]$_ }) -join ' '
        Set-Content -Path ([IO.Path]::ChangeExtension($f.FullName, '.hex')) -Value $hex
    }
    "== $($f.Name) err=$($o.Bas_Error) line=$($o.Bas_Error_Line) bytes=$($o.TotalBytes) vars=$($o.TotalVars)"
    if ($o.Bas_Error -ne 0) { "   $($o.Bas_Error_Msg -replace '\s+',' ')" } else { "   $hex" }
}
