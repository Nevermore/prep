// Copyright 2026 the Prep Authors
// SPDX-License-Identifier: Apache-2.0 OR MIT

use std::path::PathBuf;

use anyhow::{Context, Result, bail, ensure};
use semver::{Op, Version, VersionReq};

use crate::tools::Tool;
use crate::tools::rustup::Rustup;
use crate::toolset::Toolset;
use crate::ui;

/// Cargo from the Rust toolchain.
pub struct Cargo;

/// Cargo dependencies.
pub struct CargoDeps {
    /// Rustup version requirement.
    rustup_ver_req: Option<VersionReq>,
}

impl CargoDeps {
    /// Creates new Cargo dependency requirements.
    ///
    /// `None` means that the default version will be used.
    pub fn new(rustup_ver_req: impl Into<Option<VersionReq>>) -> Self {
        Self {
            rustup_ver_req: rustup_ver_req.into(),
        }
    }
}

impl Tool for Cargo {
    type Deps = CargoDeps;

    const NAME: &str = "cargo";
    const BIN: &str = "cargo";

    fn set_up(
        toolset: &mut Toolset,
        deps: &Self::Deps,
        ver_req: &VersionReq,
    ) -> Result<(Version, PathBuf)> {
        if ver_req.comparators.len() != 1 {
            bail!(
                "Only simple `=MAJOR.MINOR` version requirements are supported for the Rust toolchain, got: {}",
                ver_req
            );
        }
        let ver_req_comp = ver_req.comparators.first().unwrap();
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
        let toolchain_ver = format!("{}.{}", ver_req_comp.major, ver_req_comp.minor.unwrap());

        // TODO: Call rustup toolchain install with correct components etc first

        let mut cmd = toolset.get::<Rustup>(&(), deps.rustup_ver_req.as_ref())?;
        let cmd = cmd
            .arg("which")
            .arg(Self::BIN)
            .args(["--toolchain", &toolchain_ver]);

        ui::print_cmd(cmd);

        let output = cmd
            .output()
            .context(format!("failed to run {}", Rustup::NAME))?;
        ensure!(
            output.status.success(),
            "{} failed: {}",
            Rustup::NAME,
            output.status
        );

        let path = String::from_utf8(output.stdout)
            .context(format!("{} output not valid UTF-8", Rustup::NAME))?;
        let path = path.trim();
        let path = PathBuf::from(path);

        let Some(version) = toolset
            .verify::<Self>(&path, ver_req)
            .context(format!("failed to verify {}", Self::NAME))?
        else {
            bail!(
                "{} was reported by {} but it doesn't seem to exist",
                path.display(),
                Rustup::NAME
            );
        };

        Ok((version, path))
    }
}
