// Copyright 2026 the Prep Authors
// SPDX-License-Identifier: Apache-2.0 OR MIT

use anyhow::{Context, ensure};

use crate::cmd::CargoTargets;
use crate::session::Session;
use crate::tools::cargo::{Cargo, CargoDeps};
use crate::ui;

/// Runs Clippy analysis on the given `targets`.
///
/// In `strict` mode warnings are treated as errors and Cargo version is locked.
pub fn run(session: &mut Session, strict: bool, targets: CargoTargets) -> anyhow::Result<()> {
    let mut cmd = if strict {
        let tools_cfg = session.config().tools();
        let rustup_ver_req = tools_cfg.rustup().clone();
        let ver_req = tools_cfg.rust().clone();
        let toolset = session.toolset();
        let deps = CargoDeps::new(rustup_ver_req);
        toolset.get::<Cargo>(&deps, &ver_req)?
    } else {
        let toolset = session.toolset();
        let deps = CargoDeps::new(None);
        toolset.get::<Cargo>(&deps, None)?
    };
    let mut cmd = cmd
        .current_dir(session.root_dir())
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
