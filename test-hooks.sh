#!/bin/bash
set -e

echo "Testing pre-commit hooks..."

# Run pre-commit on all files
echo "Running pre-commit on all files..."
uv run pre-commit run --all-files

echo "Pre-commit hooks test completed successfully!" 