#!/bin/bash
set -e

# Install the pre-commit hooks
echo "Setting up pre-commit hooks..."
uv run pre-commit install

# Check if rustfmt is installed
if ! command -v rustfmt &> /dev/null; then
    echo "Installing rustfmt..."
    rustup component add rustfmt
fi

echo "Pre-commit hooks have been set up successfully!"
echo "The hooks will run automatically on git commit."
echo "You can also run them manually with: pre-commit run --all-files" 