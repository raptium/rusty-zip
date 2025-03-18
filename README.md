# RustyZip

A Python wrapper for Rust's `zip` crate that exposes a ZIP file writer with legacy ZipCrypto encryption support.

## Overview

This library provides Python bindings to Rust's `zip` crate, focusing on providing access to the legacy ZipCrypto encryption algorithm. While this encryption method is considered insecure by modern standards, it remains necessary for compatibility with older systems and software that only support this encryption method.

## Implementation Details

This library uses the unstable API of the Rust `zip` crate to access the legacy ZipCrypto encryption functionality. The encryption is implemented using the `with_deprecated_encryption` method from the `FileOptionsExt` trait in the `zip::unstable::write` module. This API is available without requiring any special feature flags.

## Security Warning

⚠️ **IMPORTANT**: The ZipCrypto algorithm is considered insecure and should not be used for sensitive data. It is provided solely for compatibility with legacy systems. For secure encryption, consider using more modern alternatives.

## Installation

```bash
pip install rustyzip
```

## Usage

```python
from rustyzip import ZipWriter

# Create a new encrypted ZIP file with ZipCrypto
with ZipWriter("example.zip", password="mypassword") as zip_file:
    # Add files to the ZIP with ZipCrypto encryption
    zip_file.write_file("path/to/file.txt", "file.txt")
    
    # Add data from memory (must be bytes)
    zip_file.write_bytes(b"Hello, world!", "hello.txt")
    
    # If you have a string, convert it to bytes first
    text = "This is a string"
    zip_file.write_bytes(text.encode('utf-8'), "string.txt")
```

## API Reference

### `ZipWriter`

#### Constructor

- `ZipWriter(path: str, password: Optional[str] = None)` - Creates a new ZIP file at the specified path. If a password is provided, files will be encrypted using ZipCrypto.

#### Methods

- `write_file(source_path: str, entry_name: str)` - Adds a file from disk to the ZIP archive.
- `write_bytes(data: bytes, entry_name: str)` - Adds bytes from memory to the ZIP archive. The data must be a bytes object.
- `close()` - Closes the ZIP file. This is automatically called when using the context manager.

## Features

- Create ZIP files with legacy ZipCrypto encryption
- Add files from disk or memory
- Simple Python API with Rust performance
- Cross-platform compatibility

## Building from Source

### Prerequisites

- Rust toolchain (1.70+)
- Python 3.8+
- Maturin (`pip install maturin`)

### Build Steps

```bash
# Clone the repository
git clone https://github.com/yourusername/rustyzip.git
cd rustyzip

# Install development dependencies
pip install -r requirements-dev.txt

# Build the package
maturin develop

# Or build in release mode
maturin develop --release
```

## Running the Example

The package includes an example script that demonstrates how to use the library:

```bash
python python_src/example.py
```

## Running Tests

You can run the tests using pytest:

```bash
# Run all tests
pytest

# Run tests with coverage report
pytest --cov=rustyzip
```

You can also run the test script directly:

```bash
python python_src/test.py
```

## Development

### Pre-commit Hooks

This project uses pre-commit hooks to ensure code quality. The hooks include:

- Ruff for Python linting and formatting
- cargo fmt for Rust formatting

To set up the pre-commit hooks:

```bash
# Install pre-commit and set up the hooks
./setup-hooks.sh

# Run the hooks manually on all files
pre-commit run --all-files
```

## Compatibility

The ZIP files created with this library using ZipCrypto encryption are compatible with most ZIP utilities that support password protection, including:

- 7-Zip
- WinZip
- WinRAR
- The `unzip` command-line tool (with the `-P` password option)

## License

This project is licensed under the MIT License - see the LICENSE file for details.

## Acknowledgments

- [zip-rs](https://github.com/zip-rs/zip) - The Rust ZIP library this project wraps
- [PyO3](https://github.com/PyO3/pyo3) - Rust bindings for Python
