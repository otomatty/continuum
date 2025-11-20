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

# Note: Tailwind CSS is already included in style/main.scss
# Leptos will compile the SCSS file automatically
echo "📦 Styles will be compiled by Leptos..."
echo ""

# Start the development server
echo "🌐 Starting development server on http://127.0.0.1:3000"
echo "   Press Ctrl+C to stop"
echo ""

cargo leptos watch

