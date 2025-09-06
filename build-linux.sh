#!/bin/bash

# Build script for Tauri Linux application
set -e

echo "🚀 Building this.ssh for Linux..."

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

# Build Tauri app for Linux
echo "🔧 Building Tauri application for Linux..."
npm run tauri:build:linux

echo "✅ Build completed successfully!"
echo "📁 Check the 'src-tauri/target/release/bundle/' directory for your Linux packages:"
echo "   - .deb package (Debian/Ubuntu)"
echo "   - .AppImage (Universal Linux)"
echo "   - .tar.gz (Generic Linux)"

# List the generated files
if [ -d "src-tauri/target/release/bundle" ]; then
    echo ""
    echo "📋 Generated packages:"
    ls -la src-tauri/target/release/bundle/
fi
