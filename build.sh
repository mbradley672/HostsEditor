#!/bin/bash

# Hosts Editor Build Script
# This script helps build the application for different platforms

set -e

echo "🏗️  Hosts Editor Build Script"
echo "=============================="

# Check if Rust is installed
if ! command -v cargo &> /dev/null; then
    echo "❌ Rust is not installed. Please install from https://rustup.rs/"
    exit 1
fi

# Check if Node.js is installed
if ! command -v node &> /dev/null; then
    echo "❌ Node.js is not installed. Please install from https://nodejs.org/"
    exit 1
fi

echo "✅ Prerequisites check passed"

# Install dependencies if needed
if [ ! -d "node_modules" ]; then
    echo "📦 Installing Node.js dependencies..."
    npm install
fi

# Function to build for specific target
build_target() {
    local target=$1
    local name=$2
    
    echo "🔨 Building for $name ($target)..."
    
    if [ "$target" = "current" ]; then
        npm run tauri:build
    else
        # Install target if not already installed
        rustup target add $target
        npm run tauri build -- --target $target
    fi
    
    echo "✅ Build completed for $name"
}

# Parse command line arguments
case "$1" in
    "windows")
        build_target "x86_64-pc-windows-msvc" "Windows"
        ;;
    "macos")
        build_target "x86_64-apple-darwin" "macOS"
        ;;
    "linux")
        build_target "x86_64-unknown-linux-gnu" "Linux"
        ;;
    "all")
        echo "🌍 Building for all platforms..."
        build_target "x86_64-pc-windows-msvc" "Windows"
        build_target "x86_64-apple-darwin" "macOS"
        build_target "x86_64-unknown-linux-gnu" "Linux"
        ;;
    "dev")
        echo "🔧 Starting development mode..."
        npm run tauri:dev
        ;;
    *)
        echo "🔨 Building for current platform..."
        build_target "current" "Current Platform"
        ;;
esac

echo ""
echo "🎉 Build completed successfully!"
echo ""
echo "📁 Built files are located in:"
echo "   src-tauri/target/release/"
echo "   src-tauri/target/release/bundle/"
echo ""
echo "Usage: $0 [windows|macos|linux|all|dev]"
