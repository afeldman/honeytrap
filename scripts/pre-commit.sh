#!/bin/bash
# Git pre-commit hook
# Run: ln -s ../../scripts/pre-commit.sh .git/hooks/pre-commit

set -e

echo "🔍 Running pre-commit checks..."

# Check if Make is available
if ! command -v make &> /dev/null; then
    echo "⚠️  Make not found, running cargo commands directly"
    
    # Format check
    echo "📝 Checking formatting..."
    cargo fmt --all -- --check
    
    # Clippy
    echo "🔎 Running clippy..."
    cargo clippy --all-targets --all-features -- -D warnings
    
    # Tests
    echo "🧪 Running tests..."
    cargo test --workspace
else
    # Use Makefile
    echo "📝 Checking formatting..."
    make fmt-check
    
    echo "🔎 Running clippy..."
    make lint
    
    echo "🧪 Running tests..."
    make test-unit
fi

echo "✅ All pre-commit checks passed!"
