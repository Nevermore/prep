<div align="center">

# Prep

**Prepare Rust projects for greatness**

[![Latest published version.](https://img.shields.io/crates/v/prep.svg)](https://crates.io/crates/prep)
[![Dependency staleness status.](https://deps.rs/crate/prep/latest/status.svg)](https://deps.rs/crate/prep)
[![Apache 2.0 or MIT license.](https://img.shields.io/badge/license-Apache--2.0_OR_MIT-blue.svg)](#license)

</div>

Prep is a cross-platform CLI tool that provides Rust workspace verification in single short command.
You can just invoke `prep ci` and if the checks succeed then you can rest easy knowing that your PR won't fail CI.

## Motivation

Rust projects tend to have a wide variety of fairly complicated verification steps in CI.
These steps help ensure that the project stays consistent and keeps working in various scenarios.

However, these CI steps are either written as GitHub Action YAML files or as Bash scripts.
Running GitHub Actions requires a rather heavyweight Docker image, which expects a Unix userland, just like Bash scripts.
Additionally, they target ephemeral VMs, so they do a lot of tooling setup that isn't efficient for a local machine.
All of that meaning that you're really out of luck on Windows and even on Unix it's going to be needlessly cumbersome.

So you need to analyze the specifics of a project's CI and craft custom local scripts to emulate the CI steps.
That, or you just manually invoke a few Cargo commands and hope for the best, leading to frequent CI failures on your PRs.
Which sucks because CI tends to run at a lot slower speed than your local machine, especially due to cold build cache.

## Prep to the rescue

Before opening a PR you can just run `prep ci` locally and verify that everything matches CI expectations.
Because Prep aims for a rather robust set of verifications this will be beneficial even when the project still uses custom CI scripts.
However, for best results the project itself should run `prep ci` in its CI instead of custom scripts.
That way the only problems that remain uncaught locally are platform specific, which the CI will catch with its multi-platform job matrix.

## Installation

```sh
cargo install prep --locked
```

## Usage

```
Usage: prep [command] [options]

Commands:
       ci              Verify for CI.
  clp  clippy          Analyze with Clippy.
  fmt  format          Format with rustfmt.
       help            Print help for the provided command.

Options:
  -h   --help          Print help for the provided command.
  -V   --version       Print version information.
```

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.
