// Copyright 2026 the Prep Authors
// SPDX-License-Identifier: Apache-2.0 OR MIT

use anyhow::{Context, Result, ensure};
use semver::VersionReq;

use crate::session::Session;
use crate::tools::cargo::CargoDeps;
use crate::tools::rustfmt::{Rustfmt, RustfmtDeps};
use crate::ui;

/// Format the workspace.
///
/// In `strict` mode Cargo version is locked.
pub fn run(session: &mut Session, strict: bool, check: bool) -> Result<()> {
    let rust_components = vec!["rustfmt".into()];
    let rustfmt = if strict {
        let tools_cfg = session.config().tools();
        let rustup_ver_req = tools_cfg.rustup().clone();
        let cargo_ver_req = tools_cfg.rust().clone();
        let toolset = session.toolset();
        let cargo_deps = CargoDeps::new(rustup_ver_req, rust_components);
        let deps = RustfmtDeps::new(cargo_deps, cargo_ver_req);
        let ver_req = VersionReq::parse("=1.8.0-stable")?; // TODO: Replace with a custom 'Any' (NOT None!)
        toolset.get::<Rustfmt>(&deps, &ver_req)?
    } else {
        let toolset = session.toolset();
        let cargo_deps = CargoDeps::new(None, rust_components);
        let deps = RustfmtDeps::new(cargo_deps, None);
        toolset.get::<Rustfmt>(&deps, None)?
    };

    let mut cmd = rustfmt.cmd();
    cmd.arg("--all");
    if check {
        cmd.arg("--check");
    }

    ui::print_cmd(&cmd);

    let status = cmd.status().context("failed to run cargo fmt")?;
    ensure!(status.success(), "cargo fmt failed: {status}");

    Ok(())
}
