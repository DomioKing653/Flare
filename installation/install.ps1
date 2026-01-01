$RepoUrl = "https://github.com/DomioKing653/Flare"
$RepoDir = "flare"
$BinName = "flarec.exe"
$InstallDir = "$env:LOCALAPPDATA\flarec\bin"

Write-Host "Cloning repo..."
if (!(Test-Path $RepoDir)) {
    git clone $RepoUrl
}

Set-Location $RepoDir

Write-Host "Building..."
cargo build --bin flarec --release

Write-Host "Installing to $InstallDir"
New-Item -ItemType Directory -Force -Path $InstallDir | Out-Null
Copy-Item "target\release\$BinName" "$InstallDir\$BinName" -Force

# Přidání do PATH (user-level)
$CurrentPath = [Environment]::GetEnvironmentVariable("PATH", "User")
if ($CurrentPath -notlike "*$InstallDir*") {
    [Environment]::SetEnvironmentVariable(
        "PATH",
        "$CurrentPath;$InstallDir",
        "User"
    )
    Write-Host "Added to path(restart shell)"
}

Write-Host "Installation is complete try:'flarec'"
