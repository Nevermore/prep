// Copyright 2026 the Prep Authors
// SPDX-License-Identifier: Apache-2.0 OR MIT

use clap::ValueEnum;

pub mod ci;
pub mod clippy;
pub mod copyright;
pub mod format;
pub mod init;

/// Cargo targets.
#[derive(Copy, Clone, Debug, PartialEq, Eq, ValueEnum)]
pub enum CargoTargets {
    /// All targets, i.e. `--lib --bins --examples --tests --benches`.
    All,
    /// Main targets are `--lib` and `--bins`.
    Main,
    /// Auxiliary targets are `--examples`, `--tests`, and `--benches`.
    #[value(name = "aux")]
    Auxiliary,
}

impl CargoTargets {
    /// Returns the Cargo flag arguments corresponding to `self`.
    pub fn as_args(&self) -> Vec<&str> {
        match self {
            Self::All => vec!["--all-targets"],
            // --lib --bins would produce a Cargo error if there are no binary targets,
            // but luckily providing no targets gives the same behavior without the error.
            Self::Main => vec![],
            Self::Auxiliary => vec!["--examples", "--tests", "--benches"],
        }
    }
}
