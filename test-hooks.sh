#!/bin/bash
set -e

echo "Testing pre-commit hooks..."

# Make sure pre-commit is installed
if ! command -v pre-commit &> /dev/null; then
    echo "Error: pre-commit is not installed. Run ./setup-hooks.sh first."
    exit 1
fi

# Run pre-commit on all files
echo "Running pre-commit on all files..."
pre-commit run --all-files

echo "Pre-commit hooks test completed successfully!" 