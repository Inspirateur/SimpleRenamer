# Scoop post-install script for setting registry key
# This script sets the context menu option for "Batch Rename" to point to the installed gui.exe

# Get the installation path of the current script
$installDir = Split-Path -Parent $dir
$guiPath = Join-Path -Path $installDir -ChildPath "gui.exe"

# Define the registry paths and values
$regPathAdmin = "HKCR\*\Shell\Batch Rename\command"
$regPathUser = "HKCU\Software\Classes\*\Shell\Batch Rename\command"
$regValue = "`"$guiPath`" `"%1`""

Write-Host $regValue