#!/bin/bash

# Build script for Tauri macOS application
set -e

echo "🍎 Building this.ssh for macOS..."

# Check if we're on macOS
if [[ "$OSTYPE" != "darwin"* ]]; then
    echo "⚠️  Warning: You're not on macOS. Cross-compilation may not work properly."
    echo "   For best results, build on a macOS machine."
    read -p "Continue anyway? (y/N): " -n 1 -r
    echo
    if [[ ! $REPLY =~ ^[Yy]$ ]]; then
        exit 1
    fi
fi

# Check if we're in the right directory
if [ ! -f "package.json" ] || [ ! -d "src-tauri" ]; then
    echo "❌ Error: Please run this script from the project root directory"
    exit 1
fi

# Install dependencies if needed
echo "📦 Installing dependencies..."
npm install

# Build the frontend
echo "🔨 Building Nuxt.js frontend..."
npm run generate

# Check if dist directory exists and contains index.html
if [ ! -f "dist/index.html" ]; then
    echo "❌ Error: dist/index.html not found. Frontend build failed."
    exit 1
fi

echo "✅ Frontend built successfully!"

# Build options
echo ""
echo "🎯 Choose your build target:"
echo "1) Intel Mac (x86_64)"
echo "2) Apple Silicon (ARM64)"
echo "3) Universal Binary (Intel + ARM64)"
echo "4) All targets"
read -p "Enter your choice (1-4): " choice

case $choice in
    1)
        echo "🔧 Building for Intel Mac (x86_64)..."
        npm run tauri:build:mac
        ;;
    2)
        echo "🔧 Building for Apple Silicon (ARM64)..."
        rustup target add aarch64-apple-darwin || true
        npm run tauri:build:mac-arm64
        ;;
    3)
        echo "🔧 Building Universal Binary (Intel + ARM64)..."
        rustup target add x86_64-apple-darwin || true
        rustup target add aarch64-apple-darwin || true
        npm run tauri:build:mac-universal
        ;;
    4)
        echo "🔧 Building for all Mac targets..."
        rustup target add x86_64-apple-darwin || true
        rustup target add aarch64-apple-darwin || true
        npm run tauri:build:mac
        npm run tauri:build:mac-arm64
        npm run tauri:build:mac-universal
        ;;
    *)
        echo "❌ Invalid choice. Exiting."
        exit 1
        ;;
esac

echo "✅ Mac build completed successfully!"
echo "📁 Check the 'src-tauri/target/release/bundle/' directory for your macOS packages:"
echo "   - .app bundle (macOS application)"
echo "   - .dmg installer (macOS disk image)"
echo "   - .pkg installer (macOS package installer)"

# List the generated files
if [ -d "src-tauri/target/release/bundle" ]; then
    echo ""
    echo "📋 Generated packages:"
    ls -la src-tauri/target/release/bundle/
fi

echo ""
echo "🚀 Your Mac app is ready for distribution!"
echo "💡 To create a .dmg file, you can use:"
echo "   create-dmg --volname 'this.ssh' --window-pos 200 120 --window-size 800 400 --icon-size 100 --icon 'this.ssh.app' 175 120 --hide-extension 'this.ssh.app' --app-drop-link 425 120 'thisdotssh.dmg' 'src-tauri/target/release/bundle/macos/'"
