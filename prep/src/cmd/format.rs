// Copyright 2026 the Prep Authors
// SPDX-License-Identifier: Apache-2.0 OR MIT

use anyhow::{Context, ensure};

use crate::session::Session;
use crate::tools::cargo;
use crate::ui;

/// Format the workspace
pub fn run(session: &Session, check: bool) -> anyhow::Result<()> {
    let mut cmd = cargo::new("")?;
    let mut cmd = cmd.current_dir(session.root_dir()).arg("fmt").arg("--all");
    if check {
        cmd = cmd.arg("--check");
    }

    ui::print_cmd(cmd);

    let status = cmd.status().context("failed to run cargo fmt")?;
    ensure!(status.success(), "cargo fmt failed: {status}");

    Ok(())
}
