// Copyright 2026 the Prep Authors
// SPDX-License-Identifier: Apache-2.0 OR MIT

use anyhow::{Context, Result, bail, ensure};
use semver::{Op, VersionReq};

use crate::cmd::CargoTargets;
use crate::session::Session;
use crate::tools::cargo::CargoDeps;
use crate::tools::clippy::{Clippy, ClippyDeps};
use crate::ui;

/// Runs Clippy analysis on the given `targets`.
///
/// In `strict` mode warnings are treated as errors and Cargo version is locked.
pub fn run(session: &mut Session, strict: bool, targets: CargoTargets) -> Result<()> {
    let rust_components = vec!["clippy".into()];
    let clippy = if strict {
        let tools_cfg = session.config().tools();
        let rustup_ver_req = tools_cfg.rustup().clone();
        let cargo_ver_req = tools_cfg.rust().clone();
        let toolset = session.toolset();
        let cargo_deps = CargoDeps::new(rustup_ver_req, rust_components);
        let ver_req = derive_version(&cargo_ver_req)?;
        let deps = ClippyDeps::new(cargo_deps, cargo_ver_req);
        toolset.get::<Clippy>(&deps, &ver_req)?
    } else {
        let toolset = session.toolset();
        let cargo_deps = CargoDeps::new(None, rust_components);
        let deps = ClippyDeps::new(cargo_deps, None);
        toolset.get::<Clippy>(&deps, None)?
    };

    let mut cmd = clippy.cmd();
    cmd.arg("--locked")
        .arg("--workspace")
        .args(targets.as_args())
        .arg("--all-features");
    if strict {
        cmd.args(["--", "-D", "warnings"]);
    }

    ui::print_cmd(&cmd);

    let status = cmd.status().context("failed to run cargo clippy")?;
    ensure!(status.success(), "cargo clippy failed: {status}");

    Ok(())
}

/// Derives the clippy version from the Rust toolchain version.
// NOTE: When we move to Rust toolchain names instead, the Clippy version could probably be any.
//       That is because if we only use a non-default clippy version with a single toolchain version
//       then there is no risk of getting an incorrect version back from the cache.
//       Clippy, by design, can be only called by the primary toolchain. So this would work fine.
fn derive_version(rust_ver_req: &VersionReq) -> Result<VersionReq> {
    if rust_ver_req.comparators.len() != 1 {
        bail!(
            "Only simple `=MAJOR.MINOR` version requirements are supported for the Rust toolchain, got: {}",
            rust_ver_req
        );
    }
    let ver_req_comp = rust_ver_req.comparators.first().unwrap();
    if ver_req_comp.op != Op::Exact
        || ver_req_comp.patch.is_some()
        || ver_req_comp.minor.is_none()
        || !ver_req_comp.pre.is_empty()
    {
        bail!(
            "Only simple `=MAJOR.MINOR` version requirements are supported for the Rust toolchain, got: {}",
            ver_req_comp
        );
    }

    let clippy_ver_req = VersionReq::parse(&format!(
        "=0.{}.{}",
        ver_req_comp.major,
        ver_req_comp.minor.unwrap()
    ))
    .context("failed to parse clippy version requirement")?;

    Ok(clippy_ver_req)
}
