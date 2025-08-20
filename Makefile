.PHONY: help install build build-linux dev clean

# Default target
help:
	@echo "Available targets:"
	@echo "  install     - Install all dependencies"
	@echo "  build       - Build the application for all platforms"
	@echo "  build-linux - Build the application for Linux only"
	@echo "  dev         - Start development server"
	@echo "  clean       - Clean build artifacts"
	@echo "  run         - Run the built application"

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

# Development mode
dev: install
	@echo "🚀 Starting development server..."
	npm run tauri:dev

# Clean build artifacts
clean:
	@echo "🧹 Cleaning build artifacts..."
	rm -rf .nuxt
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
	@echo "🎯 Target: Linux (x86_64)"
