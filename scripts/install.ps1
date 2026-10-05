$ErrorActionPreference = 'Stop'

# Remove old version and release DLLs
Get-AppxPackage -Name 'ViewRepoOnline' | Remove-AppxPackage

# Rebuild
& (Join-Path $PSScriptRoot 'build.ps1')

# Install
$manifest = Join-Path (Split-Path $PSScriptRoot -Parent) 'dist\AppxManifest.xml'
Add-AppxPackage -Register $manifest -ForceApplicationShutdown

Write-Host 'Installed!'
