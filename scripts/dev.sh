#!/bin/bash

# Development server startup script
# This script builds styles and starts the development server

set -e

echo "🚀 Starting Continuum Development Server"
echo ""

# Check if cargo-leptos is installed
if ! command -v cargo-leptos &> /dev/null; then
    echo "❌ cargo-leptos is not installed"
    echo "   Install it with: cargo install cargo-leptos --locked"
    exit 1
fi

# Build Tailwind CSS with DaisyUI
echo "📦 Building Tailwind CSS styles..."
bun run build:css
echo ""

# Start the development server
echo "🌐 Starting development server on http://127.0.0.1:3000"
echo "   Press Ctrl+C to stop"
echo ""

cargo leptos watch

