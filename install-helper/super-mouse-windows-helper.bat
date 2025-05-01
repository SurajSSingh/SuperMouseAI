@echo off
setlocal enabledelayedexpansion

REM Set downloads folder path
set "DOWNLOADS_FOLDER=%USERPROFILE%\Downloads"

REM Verify downloads folder exists
if not exist "%DOWNLOADS_FOLDER%" (
    echo Error: Downloads folder not found at '%DOWNLOADS_FOLDER%'
    exit /b 1
)

REM Get JSON data, validate structure, extract and validate URL
powershell.exe -Command ^
    "$json = Invoke-WebRequest 'https://raw.githubusercontent.com/Example/App/refs/heads/main/release-update-latest.json' -UseBasicParsing | ConvertFrom-Json; ^
     if (-not $json) { Write-Error 'Invalid JSON format'; exit 1 }; ^
     if (-not $json.'platforms') { Write-Error 'Missing platforms section'; exit 1 }; ^
     if (-not $json.'platforms.windows-x86_64') { Write-Error 'Missing windows-x86_64 section'; exit 1 }; ^
     $url = $json.'platforms.windows-x86_64.url'; ^
     if ([string]::IsNullOrEmpty($url)) { Write-Error 'Empty URL found'; exit 1 }; ^
     if ($url -notmatch '^https://github\.com/[a-zA-Z0-9-]+/[a-zA-Z0-9-]+/releases/download/[a-zA-Z0-9.-]+/[a-zA-Z0-9-_.]+_[a-zA-Z0-9.-]+_x64-setup\.exe$') { ^
         Write-Error ('Invalid URL format. Expected GitHub release URL pattern'); exit 1; ^
     }; ^
     Write-Output $url" | ^
curl -L -o "%DOWNLOADS_FOLDER%\downloaded.exe"

REM Check if download was successful
if not exist "%DOWNLOADS_FOLDER%\downloaded.exe" (
    echo Error: Failed to download executable
    exit /b 1
)

REM Execute the downloaded file
"%DOWNLOADS_FOLDER%\downloaded.exe"