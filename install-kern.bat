@echo off
REM Kern Installation Script for Windows
REM This script downloads and installs the Kern compiler executable

setlocal enabledelayedexpansion

echo Installing Kern Programming Language...
echo.

REM Create installation directory
set INSTALL_DIR=C:\Program Files\Kern
if not exist "!INSTALL_DIR!" (
    mkdir "!INSTALL_DIR!"
    echo Created directory: !INSTALL_DIR!
)

REM Download the latest executable from GitHub Releases
REM Replace 'your-username' with actual GitHub username
set GITHUB_URL=https://github.com/trispn/KERN/releases/download/latest/kern_compiler.exe

echo Downloading kern_compiler.exe from GitHub...
powershell -Command "(New-Object Net.WebClient).DownloadFile('!GITHUB_URL!', '!INSTALL_DIR!\kern_compiler.exe')"

if exist "!INSTALL_DIR!\kern_compiler.exe" (
    echo ✓ Downloaded successfully!
) else (
    echo ✗ Download failed. Check your internet connection and GitHub URL.
    pause
    exit /b 1
)

REM Add to PATH
echo Adding Kern to system PATH...
setx PATH "!PATH!;!INSTALL_DIR!"

echo.
echo ============================================
echo ✓ Kern installed successfully!
echo ============================================
echo.
echo Installation directory: !INSTALL_DIR!
echo.
echo To use Kern:
echo 1. Close and reopen your terminal
echo 2. Run: kern_compiler --help
echo.
pause
