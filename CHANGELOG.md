# Changelog

## [Unreleased]

### Added

## [0.2.0] - 2026-02-07

### Added

* `init` command to set up custom per-project Prep configuration. ([#23] by [@xStrom])
* `copyright` command to easily verify that all Rust source files have correct copyright headers. ([#22] by [@xStrom])
* `--crates <main|aux|all>` option to the `clippy` command. ([#18] by [@xStrom])
* Automatic Cargo installation via `rustup`. ([#24] by [@xStrom])
* Ability to run from within a sub-directory of a workspace as opposed to just from the workspace root. ([#23] by [@xStrom])

### Changed

* `ci` `clippy` now uses `--crates all` by default and does two separate checks with `--crates main` and `--crates aux` in extended mode. ([#18] by [@xStrom])

## [0.1.0] - 2026-01-31

### Added

* `clippy` command to easily run `cargo clippy --workspace --all-features --locked`. ([#2] by [@xStrom])
* `format` command to easily run `cargo fmt --all`. ([#3] by [@xStrom])
* `ci` command to easily run `format` and `clippy` in strict verification mode. ([#5] by [@xStrom])

[@xStrom]: https://github.com/xStrom

[#2]: https://github.com/Nevermore/prep/pull/2
[#3]: https://github.com/Nevermore/prep/pull/3
[#5]: https://github.com/Nevermore/prep/pull/5
[#18]: https://github.com/Nevermore/prep/pull/18
[#22]: https://github.com/Nevermore/prep/pull/22
[#23]: https://github.com/Nevermore/prep/pull/23
[#24]: https://github.com/Nevermore/prep/pull/24

[Unreleased]: https://github.com/Nevermore/prep/compare/v0.2.0...HEAD
[0.2.0]: https://github.com/Nevermore/prep/compare/v0.1.0...v0.2.0
[0.1.0]: https://github.com/Nevermore/prep/compare/v0.0.0...v0.1.0
