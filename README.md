# RustyZip

A Python wrapper for Rust's `zip` crate that exposes a ZIP file writer with legacy ZipCrypto encryption support.

## ⚠️ Disclaimer

This project was primarily created using Cursor with LLMs. While it has been tested, please use it at your own risk. The code may contain unexpected behaviors or bugs.

## Background

This project is a simple wrapper over Rust's `zip` crate to support writing ZIP files with ZipCrypto encryption in Python. While there are existing solutions like [pyminizip](https://github.com/smihica/pyminizip), this project addresses several limitations:

- pyminizip lacks pre-built binary wheels on PyPI, making installation more complex
- pyminizip ships with an outdated version of zlib
- pyminizip's APIs are not very efficient as they only work with file paths instead of file objects or bytes

This project is not intended to be a full-featured Python ZIP library (there are already many excellent options available). Instead, it focuses specifically on providing support for writing ZIP files with the legacy ZipCrypto encryption, which seems to be missing from other Python ZIP libraries.

## Security Warning

⚠️ **IMPORTANT**: The ZipCrypto algorithm is considered insecure and should not be used for sensitive data. It is provided solely for compatibility with legacy systems. For secure encryption, consider using more modern alternatives.

## Requirements

- Python 3.9 or later
- Rust 1.70 or later (for building from source)

## Installation

```bash
pip install rusty-zip
```

## Usage

```python
from rusty_zip import ZipWriter

# Create a new encrypted ZIP file with ZipCrypto
with ZipWriter("example.zip", password=b"mypassword") as zip_file:
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

- `ZipWriter(path: str, password: Optional[bytes] = None)` - Creates a new ZIP file at the specified path. If a password is provided, files will be encrypted using ZipCrypto. The password must be bytes.

#### Methods

- `write_file(source_path: str, entry_name: str)` - Adds a file from disk to the ZIP archive.
- `write_bytes(data: bytes, entry_name: str)` - Adds bytes from memory to the ZIP archive. The data must be a bytes object.
- `close()` - Closes the ZIP file. This is automatically called when using the context manager.

## Features

- Create ZIP files with legacy ZipCrypto encryption
- Add files from disk or memory
- Simple Python API with Rust performance
- Cross-platform compatibility

## Development

### Building from Source

```bash
uv build
```

### Running Tests

```bash
uv run pytest
```

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgments

- [zip-rs](https://github.com/zip-rs/zip) - The Rust ZIP library this project wraps
- [PyO3](https://github.com/PyO3/pyo3) - Rust bindings for Python
