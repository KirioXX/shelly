#Requires -Version 5.1

$ErrorActionPreference = "Stop"

$Repo = "KirioXX/shelly"
$ApiUrl = "https://api.github.com/repos/${Repo}/releases/latest"

function Say($msg) {
    Write-Host "→ $msg" -ForegroundColor Cyan
}

function Warn($msg) {
    Write-Host "⚠ $msg" -ForegroundColor Yellow
}

function Error($msg) {
    Write-Host "✗ $msg" -ForegroundColor Red
}

function Success($msg) {
    Write-Host "✓ $msg" -ForegroundColor Green
}

function Get-LatestRelease {
    Say "Fetching latest release info..."

    try {
        $headers = @{ "User-Agent" = "shelly-installer" }
        $response = Invoke-RestMethod -Uri $ApiUrl -Headers $headers -UseBasicParsing
    } catch {
        Error "Failed to fetch release info: $_"
        exit 1
    }

    $script:Tag = $response.tag_name
    if (-not $Tag) {
        Error "Could not determine latest release tag."
        exit 1
    }

    Say "Latest release: $Tag"
    return $response
}

function Find-Asset($release) {
    Say "Finding asset for windows-latest..."

    $pattern = "shelly-windows-latest-${Tag}.zip"
    $fallbackPattern = "shelly-windows-latest"

    $asset = $release.assets | Where-Object { $_.name -eq $pattern } | Select-Object -First 1

    if (-not $asset) {
        $asset = $release.assets | Where-Object {
            $_.name -like "${fallbackPattern}*.zip"
        } | Select-Object -First 1
    }

    if (-not $asset) {
        Error "Could not find a release asset for Windows."
        Write-Host "Available assets:"
        $release.assets | Where-Object { $_.name -like "shelly-*" } | ForEach-Object {
            Write-Host "  - $($_.name)"
        }
        exit 1
    }

    Say "Asset URL: $($asset.browser_download_url)"
    return $asset.browser_download_url
}

function Download-Asset($url) {
    Say "Downloading..."

    $script:TmpDir = Join-Path $env:TEMP ("shelly-install-" + [System.Guid]::NewGuid().ToString().Substring(0, 8))
    New-Item -ItemType Directory -Path $TmpDir | Out-Null

    $script:ArchivePath = Join-Path $TmpDir "shelly.zip"

    try {
        Invoke-WebRequest -Uri $url -OutFile $ArchivePath -UseBasicParsing
    } catch {
        Error "Download failed: $_"
        exit 1
    }

    $size = (Get-Item $ArchivePath).Length
    Success "Downloaded $("{0:N0}" -f $size) bytes"
}

function Extract-Archive {
    Say "Extracting archive..."

    $extractedDir = Join-Path $TmpDir "extracted"
    Expand-Archive -Path $ArchivePath -DestinationPath $extractedDir -Force

    $script:BinaryPath = Join-Path $extractedDir "shelly.exe"
    if (-not (Test-Path $BinaryPath)) {
        # Search one level deep
        $found = Get-ChildItem -Path $extractedDir -Filter "shelly.exe" -Recurse -Depth 1 | Select-Object -First 1
        if ($found) {
            $script:BinaryPath = $found.FullName
        }
    }

    if (-not (Test-Path $BinaryPath)) {
        Error "Binary not found after extraction."
        Write-Host "Contents of ${extractedDir}:"
        Get-ChildItem -Path $extractedDir -Recurse | ForEach-Object { Write-Host "  $($_.FullName)" }
        exit 1
    }

    Success "Extracted shelly.exe"
}

function Install-Binary {
    if ($env:INSTALL_DIR) {
        $script:InstallDir = $env:INSTALL_DIR
    } elseif ($env:CARGO_HOME) {
        $script:InstallDir = Join-Path $env:CARGO_HOME "bin"
    } else {
        $script:InstallDir = Join-Path $env:USERPROFILE ".cargo\bin"
    }

    if (-not (Test-Path $InstallDir)) {
        New-Item -ItemType Directory -Path $InstallDir -Force | Out-Null
    }

    $script:Dest = Join-Path $InstallDir "shelly.exe"

    Say "Installing to $Dest..."

    if (Test-Path $Dest) {
        Warn "Existing binary found at $Dest"
        $backup = "${Dest}.backup"
        if (Test-Path $backup) { Remove-Item $backup -Force }
        Move-Item $Dest $backup
    }

    Copy-Item $BinaryPath $Dest -Force
    Success "Installed shelly $Tag to $Dest"
}

function Check-Path {
    $resolved = (Resolve-Path $InstallDir).Path
    $inPath = $env:Path -split ";" | ForEach-Object { $_.TrimEnd("\\") } | Where-Object { $_ -eq $resolved }

    if ($inPath) {
        Success "$InstallDir is already in your PATH"
    } else {
        Warn "$InstallDir is not in your PATH."
        Write-Host ""
        Write-Host "  Add this to your PowerShell profile or system PATH:"
        Write-Host "    $InstallDir"
        Write-Host ""
    }
}

function Verify-Install {
    try {
        $output = & $Dest --version 2>$null
        if ($LASTEXITCODE -eq 0) {
            Success "Installation verified: $output"
        } else {
            Warn "Installation succeeded but version check failed."
        }
    } catch {
        Warn "Installation succeeded but shelly.exe is not in your current PATH."
    }
}

function Cleanup {
    if ($TmpDir -and (Test-Path $TmpDir)) {
        Remove-Item $TmpDir -Recurse -Force -ErrorAction SilentlyContinue
    }
}

function Main {
    Write-Host ""
    Write-Host "  🐚 Shelly Installer (Windows)"
    Write-Host ""

    try {
        $release = Get-LatestRelease
        $assetUrl = Find-Asset $release
        Download-Asset $assetUrl
        Extract-Archive
        Install-Binary
        Check-Path
        Verify-Install

        Write-Host ""
        Success "Done! Run 'shelly setup' to configure."
        Write-Host ""
    } finally {
        Cleanup
    }
}

Main
