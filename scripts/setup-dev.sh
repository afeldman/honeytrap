#!/bin/bash
# Setup development environment

set -e

echo "🛠️  Setting up HoneyTrap development environment..."

# Check if Rust is installed
if ! command -v rustc &> /dev/null; then
    echo "❌ Rust not found. Please install from https://rustup.rs"
    exit 1
fi

echo "✓ Rust $(rustc --version)"

# Install development dependencies
echo "📦 Installing development dependencies..."
make deps

# Setup git hooks
echo "🪝 Setting up git hooks..."
mkdir -p .git/hooks
ln -sf ../../scripts/pre-commit.sh .git/hooks/pre-commit
chmod +x .git/hooks/pre-commit
chmod +x scripts/pre-commit.sh

echo "✓ Git hooks configured"

# Build project
echo "🔨 Building project..."
make build

# Run tests
echo "🧪 Running tests..."
make test

echo ""
echo "✅ Development environment setup complete!"
echo ""
echo "Available commands:"
echo "  make help       - Show all available targets"
echo "  make dev        - Run development server with auto-reload"
echo "  make test       - Run tests"
echo "  make docker     - Start with docker-compose"
echo ""
