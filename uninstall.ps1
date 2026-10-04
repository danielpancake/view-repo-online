$ErrorActionPreference = 'Stop'

# Uninstall
Get-AppxPackage -Name 'ViewRepoOnline' | Remove-AppxPackage

Write-Host 'Uninstalled!'
