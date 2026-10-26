# Compila i casi di re/samples/cases.txt ("nome = riga1|riga2|...") e salva .bas/.hex in re/samples.
# Uso: powershell -File rbcases.ps1 [-ConType 4] [-ConID 2] [-Filter regex]
param([int]$ConType = 4, [int]$ConID = 2, [string]$Filter = '.')
if ([Environment]::Is64BitProcess) {
    & "$env:WINDIR\SysWOW64\WindowsPowerShell\v1.0\powershell.exe" -NoProfile -File $PSCommandPath -ConType $ConType -ConID $ConID -Filter $Filter
    return
}
$dir = Join-Path $PSScriptRoot '..\samples'
$o = New-Object -ComObject RBv2.ROBOBASICv2
$o.Con_Type = $ConType
$o.Con_ID = $ConID
foreach ($line in Get-Content (Join-Path $dir 'cases.txt')) {
    if ($line -notmatch '^\s*([\w.-]+)\s*=\s*(.*)$') { continue }
    $name = $Matches[1]
    $body = $Matches[2]
    if ($name -notmatch $Filter) { continue }
    $src = ($body -split '\|') -join "`r`n"
    $src += "`r`n"
    [IO.File]::WriteAllText((Join-Path $dir "$name.bas"), $src)
    $o.Compile($src)
    if ($o.Bas_Error -eq 0) {
        $hex = ($o.ObjCode | ForEach-Object { '{0:X2}' -f [int]$_ }) -join ' '
        Set-Content (Join-Path $dir "$name.hex") $hex
        '{0,-12} {1}' -f $name, $hex
    } else {
        '{0,-12} ERR{1} {2}' -f $name, $o.Bas_Error, ($o.Bas_Error_Msg -replace '\s+', ' ')
    }
}
