$ErrorActionPreference = 'Stop'

$dist = Join-Path $PSScriptRoot 'dist'

$target = 'x86_64-pc-windows-msvc'
$release = Join-Path $PSScriptRoot "target\$target\release"
$package = Join-Path $PSScriptRoot 'package'

# Get cargo
$cargo = (Get-Command cargo.exe -ErrorAction SilentlyContinue | Select-Object -First 1).Source
if (-not $cargo) {
    $cargo = Join-Path $env:USERPROFILE '.cargo\bin\cargo.exe'
}
if (-not (Test-Path -LiteralPath $cargo)) {
    throw 'Install Rust with the default MSVC toolchain, then rerun this script'
}

# Build
& $cargo build --release --target $target
if ($LASTEXITCODE -ne 0) {
    throw 'Rust build failed. See the compiler errors above'
}

# Copy to a clean dist
if (Test-Path -LiteralPath $dist) {
    Remove-Item -LiteralPath $dist -Recurse -Force
}

New-Item -ItemType Directory -Path $dist | Out-Null
Copy-Item -LiteralPath "$package\AppxManifest.xml" -Destination $dist -Force
Copy-Item -LiteralPath "$package\Assets" -Destination $dist -Recurse -Force
Copy-Item -LiteralPath "$release\view_repo_online.dll" -Destination $dist -Force
Copy-Item -LiteralPath "$release\ViewRepoOnline.exe" -Destination $dist -Force
