# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

<!-- next-header -->

## [Unreleased] <!-- release-date -->

### Added

- Add institutions service integration test.

### Changed

- REST client logs and deserializes request and response payloads, response status and response headers, simplifying specialized service clients.
- Refactor hosted license provider model structs and client into separate submodules.

## [0.3.1] - 2023-05-25

### Added

- Add support for `code` and `icon_url` fields on `MethodDetails` and `ProductDetails`.
- Add `cargo test` as pre-release hook.

## [0.3.0] - 2023-05-25

### BREAKING CHANGES

- Translate "Lika" as "License Provider". This renames `HostedSitesClient` to `HostedLicenseProviderClient`.

### Added 

- When `RUST_LOG=basispoort_sync_client=info` (or `debug`, `trace`), then the REST client will print the configured environment and base URL upon creation.
- Introduce crate features to toggle `institutions` API client and `hosted-license-provider` API client on or off. On by default.
- Replace leaky abstraction names `post_*` and `put_*` by `create` / `update` / `set`.
  - `post_method`: `create_method`
  - `put_method`: `update_method`
  - `put_method_user_ids`: `set_method_user_ids`
  - `put_method_user_chain_ids`: `set_method_user_chain_ids`
  - `post_product`: `create_product`
  - `put_product`: `update_product`
  - `put_method_user_ids`: `set_method_user_ids`
  - `put_method_user_chain_ids`: `set_method_user_chain_ids`
- Rename `site` (generic term over "methode", "product") to application. This renames `SiteTag` to `ApplicationTag`.

### Changed

- Configure REST client with base URL rather than base hostname.
- Change wording to present tense in `CHANGELOG.md`

## [0.2.2] - 2023-05-24

### Changed

- Bump tokio to `"1.23.1"`, due to [RUSTSEC-2023-0001](https://rustsec.org/advisories/RUSTSEC-2023-0001.html).

## [0.2.1] - 2023-05-24

### Added

- Add some minor documentation.

## [0.2.0] - 2023-05-24

### Added

- Add a `CHANGELOG.md`.
- Implement full "Hosted Lika" integration test.
- Add builder-style methods to `MethodDetails` and `ProductDetails`.
- `Environment`s can now be parsed from `&str`.

### Changed

- Change from `&str` and `String` paths to [`&Path`](https://doc.rust-lang.org/std/path/struct.Path.html) and [`PathBuf`](https://doc.rust-lang.org/std/path/struct.PathBuf.html).
- Change from `&str` and `String` URLs to [`url::Url`](https://docs.rs/url/latest/url/struct.Url.html), re-exported as `crate::Url`.
- Update dependencies.
- Remove `impl Deref` from list response structs, as they are not smart pointers.
- Change reading certificate and icon files from blocking to async.

## [0.1.0] - 2022-11-08

### Added

- Initial implementation.

<!-- next-url -->
[Unreleased]: https://github.com/LeoniePhiline/basispoort-sync-client/compare/v0.3.1...HEAD
[0.3.1]: https://github.com/LeoniePhiline/basispoort-sync-client/compare/v0.3.0...v0.3.1
[0.3.0]: https://github.com/LeoniePhiline/basispoort-sync-client/compare/v0.2.2...v0.3.0
[0.2.2]: https://github.com/LeoniePhiline/basispoort-sync-client/releases/tag/v0.2.2
[0.2.1]: https://github.com/LeoniePhiline/basispoort-sync-client/releases/tag/v0.2.1
[0.2.0]: https://github.com/LeoniePhiline/basispoort-sync-client/releases/tag/v0.2.0
[0.1.0]: https://github.com/LeoniePhiline/basispoort-sync-client/releases/tag/v0.1.0
