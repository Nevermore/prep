// Copyright 2026 the Prep Authors
// SPDX-License-Identifier: Apache-2.0 OR MIT

use anyhow::{Context, Result, bail, ensure};
use semver::{Op, Version, VersionReq};

use crate::tools::rustup::Rustup;
use crate::tools::{BinCtx, Tool};
use crate::toolset::Toolset;
use crate::ui;

/// Cargo from the Rust toolchain.
pub struct Cargo;

/// Cargo dependencies.
#[derive(Default)]
pub struct CargoDeps {
    /// Rustup version requirement.
    rustup_ver_req: Option<VersionReq>,
    /// Rust toolchain components.
    components: Vec<String>,
}

impl CargoDeps {
    /// Creates new Cargo dependency requirements.
    ///
    /// `None` means that the default version will be used.
    pub fn new(rustup_ver_req: impl Into<Option<VersionReq>>, components: Vec<String>) -> Self {
        Self {
            rustup_ver_req: rustup_ver_req.into(),
            components,
        }
    }
}

impl Tool for Cargo {
    type Deps = CargoDeps;

    const NAME: &str = "cargo";
    const BIN: &str = "cargo";
    const MANAGED: bool = false;

    fn set_up(
        toolset: &mut Toolset,
        deps: &Self::Deps,
        ver_req: &VersionReq,
    ) -> Result<(BinCtx, Version)> {
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
        let toolchain_name = format!("{}.{}", ver_req_comp.major, ver_req_comp.minor.unwrap());

        // TODO: Maybe worth checking if it's already installed first? A non-network command probably then.
        // TODO: Perhaps worth doing component delta checks and using the component commands to manage them.
        //       Because then there wouldn't be a toolchain 1.92.0 -> 1.92.1 update just because of component.

        // Set up the toolchain
        let rustup = toolset.get::<Rustup>(&(), deps.rustup_ver_req.as_ref())?;
        let mut cmd = rustup.cmd();
        cmd.arg("toolchain")
            .arg("install")
            .arg(&toolchain_name)
            .arg("--no-self-update")
            .args(["--profile", "minimal"]);

        if !deps.components.is_empty() {
            cmd.args(["--component", &deps.components.join(",")]);
        }

        ui::print_cmd(&cmd);

        let status = cmd
            .status()
            .context(format!("failed to run {}", Rustup::NAME))?;
        ensure!(status.success(), "{} failed: {status}", Rustup::NAME);

        // We need to configure the toolchain version via an environment variable.
        // This is because we want to run the correct rustfmt version when invoking `cargo fmt`.
        // Directly running the correct cargo executable is not enough
        // as it will still choose the default version of rustfmt.
        // Unlike things like cargo-clippy or cargo-nextest, `cargo fmt` is not
        // just a simple wrapper over rustfmt so we can't easily run it directly.
        let environment = toolset.environment().clone().rust(Some(toolchain_name));
        // Given that we use the environment for version control, we'll just use the binary name.
        let binctx = BinCtx::new(
            Self::BIN.into(),
            toolset.working_dir().to_path_buf(),
            environment,
        );

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
