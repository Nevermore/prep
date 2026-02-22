// Copyright 2026 the Prep Authors
// SPDX-License-Identifier: Apache-2.0 OR MIT

use anyhow::{Context, Result, bail};
use semver::{Version, VersionReq};

use crate::tools::cargo::{Cargo, CargoDeps};
use crate::tools::{BinCtx, Tool};
use crate::toolset::Toolset;

/// Rustfmt from the Rust toolchain.
pub struct Rustfmt;

/// Rustfmt dependencies.
#[derive(Default)]
pub struct RustfmtDeps {
    /// Cargo dependencies.
    cargo_deps: CargoDeps,
    /// Cargo version requirement.
    cargo_ver_req: Option<VersionReq>,
}

impl RustfmtDeps {
    /// Creates new Rustfmt dependency requirements.
    ///
    /// `None` means that the default version will be used.
    pub fn new(cargo_deps: CargoDeps, cargo_ver_req: impl Into<Option<VersionReq>>) -> Self {
        Self {
            cargo_deps,
            cargo_ver_req: cargo_ver_req.into(),
        }
    }
}

impl Tool for Rustfmt {
    type Deps = RustfmtDeps;

    const NAME: &str = "rustfmt";
    const BIN: &str = "cargo";
    const MANAGED: bool = false;

    fn default_binctx(toolset: &mut Toolset, deps: &Self::Deps) -> Result<BinCtx> {
        let cargo = toolset.get::<Cargo>(&deps.cargo_deps, deps.cargo_ver_req.as_ref())?;
        let binctx = cargo.args(vec!["fmt".into()]);
        Ok(binctx)
    }

    fn set_up(
        toolset: &mut Toolset,
        deps: &Self::Deps,
        ver_req: &VersionReq,
    ) -> Result<(BinCtx, Version)> {
        // Directly calling set_up will bypass the toolset cache
        // and allows us to set up the potentially missing components.
        let cargo_ver_req = deps.cargo_ver_req.as_ref().unwrap_or_else(|| {
            panic!(
                "{} set up requires a specific {} version dependency",
                Self::NAME,
                Cargo::NAME
            )
        });
        let (cargo, _) = Cargo::set_up(toolset, &deps.cargo_deps, cargo_ver_req)
            .context(format!("failed to set up {}", Cargo::NAME))?;

        let binctx = cargo.args(vec!["fmt".into()]);

        // Verify that it actually works.
        let Some(version) = toolset
            .verify::<Self>(&binctx, ver_req)
            .context(format!("failed to verify {}", Self::NAME))?
        else {
            bail!(
                "'{}' was just installed but now was no longer found",
                binctx.path().display()
            );
        };

        Ok((binctx, version))
    }
}
