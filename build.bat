@echo off
REM Hosts Editor Build Script for Windows
REM This script helps build the application

echo 🏗️  Hosts Editor Build Script
echo ==============================

REM Check if Rust is installed
cargo --version >nul 2>&1
if %ERRORLEVEL% NEQ 0 (
    echo ❌ Rust is not installed. Please install from https://rustup.rs/
    exit /b 1
)

REM Check if Node.js is installed
node --version >nul 2>&1
if %ERRORLEVEL% NEQ 0 (
    echo ❌ Node.js is not installed. Please install from https://nodejs.org/
    exit /b 1
)

echo ✅ Prerequisites check passed

REM Install dependencies if needed
if not exist "node_modules" (
    echo 📦 Installing Node.js dependencies...
    npm install
)

if "%1"=="dev" (
    echo 🔧 Starting development mode...
    npm run tauri:dev
    goto :end
)

if "%1"=="windows" (
    echo 🔨 Building for Windows...
    npm run tauri:build
    goto :success
)

if "%1"=="all" (
    echo 🌍 Building for all platforms...
    echo Note: Cross-compilation may require additional setup
    npm run tauri:build
    goto :success
)

REM Default: build for current platform
echo 🔨 Building for current platform...
npm run tauri:build

:success
echo.
echo 🎉 Build completed successfully!
echo.
echo 📁 Built files are located in:
echo    src-tauri\target\release\
echo    src-tauri\target\release\bundle\
echo.

:end
echo Usage: %0 [windows^|dev]
