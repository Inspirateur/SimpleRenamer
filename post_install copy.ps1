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

# Set the registry key based on admin privileges
if (Test-IsAdmin) {
    # Running as admin, set the key in HKCR
    New-Item -Path "HKCR:\*\Shell\Batch Rename" -Force | Out-Null
    New-Item -Path "HKCR:\*\Shell\Batch Rename\command" -Force | Out-Null
    Set-ItemProperty -Path $regPathAdmin -Name "(default)" -Value $regValue
    Write-Host "Registry key set successfully for Batch Rename in HKCR."
} else {
    # Not running as admin, set the key in HKCU
    New-Item -Path "HKCU:\Software\Classes\*\Shell\Batch Rename" -Force | Out-Null
    New-Item -Path "HKCU:\Software\Classes\*\Shell\Batch Rename\command" -Force | Out-Null
    Set-ItemProperty -Path $regPathUser -Name "(default)" -Value $regValue
    Write-Host "Registry key set successfully for Batch Rename in HKCU."
}