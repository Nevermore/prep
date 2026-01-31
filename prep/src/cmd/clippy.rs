// Copyright 2026 the Prep Authors
// SPDX-License-Identifier: Apache-2.0 OR MIT

use std::process::Command;

use anyhow::{Context, ensure};

use crate::ui;

/// Runs Clippy analysis
pub fn run() -> anyhow::Result<()> {
    let mut cmd = Command::new("cargo");
    let cmd = cmd.arg("clippy").arg("--workspace").arg("--all-features");

    ui::print_cmd(cmd);

    let status = cmd.status().context("failed to run cargo clippy")?;
    ensure!(status.success(), "cargo clippy failed: {status}");

    Ok(())
}
