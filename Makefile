.PHONY: help install build build-linux build-mac build-mac-intel build-mac-arm64 build-mac-universal dev clean

# Default target
help:
	@echo "Available targets:"
	@echo "  install           - Install all dependencies"
	@echo "  build             - Build the application for all platforms"
	@echo "  build-linux       - Build the application for Linux only"
	@echo "  build-mac         - Build the application for macOS (Intel + ARM64)"
	@echo "  build-mac-intel   - Build the application for Intel Mac only"
	@echo "  build-mac-arm64   - Build the application for Apple Silicon Mac only"
	@echo "  build-mac-universal - Build universal binary for macOS"
	@echo "  dev               - Start development server"
	@echo "  clean             - Clean build artifacts"
	@echo "  run               - Run the built application"

# Install dependencies
install:
	@echo "📦 Installing dependencies..."
	npm install

# Build for all platforms
build: install
	@echo "🔨 Building for all platforms..."
	npm run generate
	npm run tauri:build

# Build for Linux only
build-linux: install
	@echo "🔨 Building for Linux..."
	npm run generate
	npm run tauri:build:linux

# Build for macOS (Intel + ARM64)
build-mac: install
	@echo "🍎 Building for macOS (Intel + ARM64)..."
	npm run generate
	npm run tauri:build:mac
	npm run tauri:build:mac-arm64

# Build for Intel Mac only
build-mac-intel: install
	@echo "🍎 Building for Intel Mac..."
	npm run generate
	npm run tauri:build:mac

# Build for Apple Silicon Mac only
build-mac-arm64: install
	@echo "🍎 Building for Apple Silicon Mac..."
	npm run generate
	npm run tauri:build:mac-arm64

# Build universal binary for macOS
build-mac-universal: install
	@echo "🍎 Building Universal Binary for macOS..."
	npm run generate
	npm run tauri:build:mac-universal

# Development mode
dev: install
	@echo "🚀 Starting development server..."
	npm run tauri:dev

# Clean build artifacts
clean:
	@echo "🧹 Cleaning build artifacts..."
	rm -rf .nuxt
	rm -rf dist
	rm -rf src-tauri/target
	rm -rf node_modules/.cache

# Run the built application
run:
	@echo "▶️  Running the application..."
	./src-tauri/target/release/app

# Show build info
info:
	@echo "📋 Project: this.ssh"
	@echo "📁 Frontend: Nuxt.js"
	@echo "🔧 Backend: Tauri (Rust)"
	@echo "🎯 Targets: Linux, macOS (Intel + ARM64)"
	@echo "🍎 Mac builds: Intel, ARM64, Universal"
