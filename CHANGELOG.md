# Changelog

All notable changes to this project are documented in this file. The format is
based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/), and this
project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).
Versions are normalised to [PEP 440](https://peps.python.org/pep-0440/) on PyPI,
so `0.1.0-alpha.2` appears there as `0.1.0a2`.

## [Unreleased]

## [0.1.0] - 2026-09-10

First stable release. The `0.1.0-alpha.*` pre-releases are superseded and the
public API is unchanged: `ZipWriter(path_or_file_like, password=None)` with
`write_file`, `write_bytes` and `close`.

### Added

- Support for Python 3.14.

### Changed

- **Rust 1.88 or later is now required** (was 1.70), driven by pyo3 0.29 and by
  `time` >= 0.3.47, which `zip` pulls in.
- `zip` upgraded from 2.2.3 to 8.6.0, and built without its default features
  (`default-features = false, features = ["deflate", "time"]`). Since this crate
  only writes Deflated + ZipCrypto entries, the AES, bzip2, zstd, lzma and ppmd
  implementations and their C build dependencies are no longer compiled in: the
  dependency tree shrank from 87 to 51 crates.
- `maturin` is no longer a runtime dependency, so installing this package no
  longer pulls in a Rust build tool. The package has no runtime dependencies.

### Behaviour changes

- **An empty ZipCrypto password is now rejected** with `ValueError` instead of
  silently writing an archive whose key is trivially derivable. `zip` >= 3
  refuses empty passwords for the same reason.
- Encrypted entries now carry a data descriptor (general purpose flag bit 3),
  because `zip` >= 3 always uses one for ZipCrypto, even when the writer can
  seek. Archives are a few bytes larger per entry. Readability was verified
  against Info-ZIP, libarchive and 7-Zip.

### Security

- `zip` 2.2.3 was **yanked** as part of the response to
  [CVE-2025-29787](https://rustsec.org/advisories/RUSTSEC-2025-0168.html)
  (RUSTSEC-2025-0168, incorrect path canonicalization during extraction leading
  to arbitrary file write). This crate only writes archives and never called the
  affected functions, but the yanked version blocked `cargo install` and failed
  downstream `cargo audit`. Fixed by the `zip` 8.6.0 upgrade.
- `time` 0.3.39 -> 0.3.55, fixing
  [RUSTSEC-2026-0009](https://rustsec.org/advisories/RUSTSEC-2026-0009.html)
  (CVE-2026-25727, denial of service via stack exhaustion in RFC 2822 parsing).
- Dropped the `rand` 0.8.5 dependency, which carries the unsound warning
  RUSTSEC-2026-0097, as a side effect of the `zip` upgrade.

### Infrastructure

- Added CI jobs running the test suite on Python 3.9, 3.12 and 3.14, and made
  releases depend on them. Previously CI only built wheels and never ran tests.
- Added a `cargo audit --deny warnings` job (which also fails on yanked and
  unsound crates), running on pull requests, on `main`, and weekly.
- All GitHub Actions are now pinned to full commit SHAs instead of mutable tags,
  moved off Node 20 versions (removed from GitHub runners on 2026-09-23), and
  the retired `macos-13` runner was replaced with `macos-15-intel`.
- Added Dependabot configuration for GitHub Actions, Cargo, uv and pre-commit.

## [0.1.0-alpha.2] - 2025-03-18

Pre-release.

- Documented the `ZipWriter` constructor parameters and the supported file-like
  objects.

## [0.1.0-alpha.1] - 2025-03-18

Initial pre-release.

- `ZipWriter` writing Deflated entries with optional legacy ZipCrypto
  encryption, from either a path or a file-like object.

[Unreleased]: https://github.com/raptium/rusty-zip/compare/v0.1.0...HEAD
[0.1.0]: https://github.com/raptium/rusty-zip/releases/tag/v0.1.0
[0.1.0-alpha.2]: https://github.com/raptium/rusty-zip/releases/tag/v0.1.0-alpha.2
[0.1.0-alpha.1]: https://github.com/raptium/rusty-zip/releases/tag/v0.1.0-alpha.1
