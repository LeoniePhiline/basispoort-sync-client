# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added 

_(none)_

### Changed

- Bumped tokio to `"1.23.1"`, due to [RUSTSEC-2023-0001](https://rustsec.org/advisories/RUSTSEC-2023-0001.html).

### Fixed

_(none)_

### Removed

_(none)_

## [0.2.1] - 2023-05-24

### Added

- Added some minor documentation.

## [0.2.0] - 2023-05-24

### Added

- Added a `CHANGELOG.md`.
- Implemented full "Hosted Lika" integration test.
- Added builder-style methods to `MethodDetails` and `ProductDetails`.
- `Environment`s can now be parsed from `&str`.

### Changed

- Changed from `&str` and `String` paths to [`&Path`](https://doc.rust-lang.org/std/path/struct.Path.html) and [`PathBuf`](https://doc.rust-lang.org/std/path/struct.PathBuf.html).
- Changed from `&str` and `String` URLs to [`url::Url`](https://docs.rs/url/latest/url/struct.Url.html), re-exported as `crate::Url`.
- Updated dependencies.
- Removed `impl Deref` from list response structs, as they are not smart pointers.
- Changed reading certificate and icon files from blocking to async.

## [0.1.0] - 2022-11-08

### Added

- Initial implementation.

[unreleased]: https://github.com/LeoniePhiline/showcase-dl/compare/v0.2.1...HEAD
[0.2.0]: https://github.com/LeoniePhiline/elopage-dl/releases/tag/v0.2.1
[0.2.0]: https://github.com/LeoniePhiline/elopage-dl/releases/tag/v0.2.0
[0.1.0]: https://github.com/LeoniePhiline/elopage-dl/releases/tag/v0.1.0
