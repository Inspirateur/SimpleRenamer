# Scoop post-install script for setting registry key
# This script sets the context menu option for "Batch Rename" to point to the installed gui.exe

# Get the installation path of the current script
$installDir = Split-Path -Parent $dir
$guiPath = Join-Path -Path $installDir -ChildPath "gui.exe"

# Define the registry paths and values
$regPathAdmin = "HKCR\*\Shell\Batch Rename\command"
$regPathUser = "HKCU\Software\Classes\*\Shell\Batch Rename\command"
$regValue = "`"$guiPath`" `"%1`""

# Function to check if the script is running as admin
function Test-IsAdmin {
    $currentUser = New-Object Security.Principal.WindowsPrincipal [Security.Principal.WindowsIdentity]::GetCurrent()
    return $currentUser.IsInRole([Security.Principal.WindowsBuiltInRole]::Administrator)
}

if (Test-IsAdmin) {
    Write-Host "Running as admin, setting registry key in HKCR."
} else {
    Write-Host "Not running as admin, setting registry key in HKCU."
}