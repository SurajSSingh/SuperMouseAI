@echo off
setlocal enabledelayedexpansion

REM Set downloads folder path
set "DOWNLOADS_FOLDER=%USERPROFILE%\Downloads"

REM Verify downloads folder exists
if not exist "%DOWNLOADS_FOLDER%" (
    echo Error: Downloads folder not found at '%DOWNLOADS_FOLDER%'
    exit /b 1
)

REM Get JSON data and extract URL
curl -s -f "https://raw.githubusercontent.com/Example/App/refs/heads/main/release-update-latest.json" > temp.json
if !errorlevel! neq 0 (
    echo Error: Failed to fetch JSON data
    exit /b 1
)

REM Read and parse JSON
set "url="
for /f "tokens=2 delims=:," %%a in ('findstr /m /c:"\"platforms\.windows-x86_64\.url\":" temp.json') do (
    set "url=%%a"
    goto :found
)
:found
if "%url%"=="" (
    echo Error: Missing or invalid URL in JSON
    del /q temp.json
    exit /b 1
)

REM Validate URL format
echo "%url%" | findstr /r /c:"^https://github\.com/[a-zA-Z0-9-]\+/[a-zA-Z0-9-]\+/releases/download/[a-zA-Z0-9.-]\+/[a-zA-Z0-9-_.]\+[a-zA-Z0-9.-]_x64-setup\.exe$" >nul 2>&1
if !errorlevel! neq 0 (
    echo Error: Invalid URL format
    del /q temp.json
    exit /b 1
)

REM Download the executable
curl -L "%url%" -o "%DOWNLOADS_FOLDER%\downloaded.exe"

REM Check if download was successful
if not exist "%DOWNLOADS_FOLDER%\downloaded.exe" (
    echo Error: Failed to download executable
    del /q temp.json
    exit /b 1
)

REM Execute the downloaded file
"%DOWNLOADS_FOLDER%\downloaded.exe"

REM Clean up temporary file
del /q temp.json