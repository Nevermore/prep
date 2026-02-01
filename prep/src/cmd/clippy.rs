// Copyright 2026 the Prep Authors
// SPDX-License-Identifier: Apache-2.0 OR MIT

use std::process::Command;

use anyhow::{Context, ensure};

use crate::cmd::CargoTargets;
use crate::ui;

/// Runs Clippy analysis on the given `targets`.
///
/// In `strict` mode warnings are treated as errors.
pub fn run(targets: CargoTargets, strict: bool) -> anyhow::Result<()> {
    let mut cmd = Command::new("cargo");
    let mut cmd = cmd
        .arg("clippy")
        .arg("--locked")
        .arg("--workspace")
        .args(targets.as_args())
        .arg("--all-features");
    if strict {
        cmd = cmd.args(["--", "-D", "warnings"]);
    }

    ui::print_cmd(cmd);

    let status = cmd.status().context("failed to run cargo clippy")?;
    ensure!(status.success(), "cargo clippy failed: {status}");

    Ok(())
}
