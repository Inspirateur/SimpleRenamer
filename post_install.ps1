# This script adds the entry "Batch Rename" to the context menu for all files in Windows Explorer.
Write-Host " Setting the Batch Rename context menu (this may take a few minutes)..."
$installDir = Split-Path -Parent $dir
$guiPath = Join-Path -Path $installDir -ChildPath "current/gui.exe"
$regValue = "`"$guiPath`" `"%1`""
New-Item -Path "HKCU:\Software\Classes\*\Shell\Batch Rename" -Force | Out-Null
New-Item -Path "HKCU:\Software\Classes\*\Shell\Batch Rename\command" -Force | Out-Null
Set-Item -Path "HKCU:\Software\Classes\*\Shell\Batch Rename\command" -Value $regValue -Force
Write-Host " Registry key set successfully for Batch Rename in HKCU."
