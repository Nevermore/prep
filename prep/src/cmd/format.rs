// Copyright 2026 the Prep Authors
// SPDX-License-Identifier: Apache-2.0 OR MIT

use std::process::Command;

use anyhow::{Context, ensure};

use crate::ui;

/// Format the workspace
pub fn run() -> anyhow::Result<()> {
    let mut cmd = Command::new("cargo");
    let cmd = cmd.arg("fmt").arg("--all");

    ui::print_cmd(cmd);

    let status = cmd.status().context("failed to run cargo fmt")?;
    ensure!(status.success(), "cargo fmt failed: {status}");

    Ok(())
}
