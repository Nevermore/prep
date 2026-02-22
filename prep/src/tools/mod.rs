// Copyright 2026 the Prep Authors
// SPDX-License-Identifier: Apache-2.0 OR MIT

pub mod cargo;
pub mod clippy;
pub mod ripgrep;
pub mod rustfmt;
pub mod rustup;

use std::io::ErrorKind;
use std::path::{Path, PathBuf};
use std::process::Command;

use anyhow::{Context, Result, ensure};
use regex::Regex;
use semver::{Version, VersionReq};

use crate::environment::Environment;
use crate::toolset::Toolset;
use crate::ui;

/// Generic Prep tool code.
pub trait Tool: Sized + 'static {
    /// The type describing the dependencies of this tool.
    type Deps: Default;

    /// The name of the tool, used for both display and to identify the cargo package.
    const NAME: &str;
    /// The binary executable name.
    const BIN: &str;
    /// Whether the tool installation is managed by toolset.
    const MANAGED: bool;

    /// Returns the default binary context for this tool
    #[expect(unused_variables, reason = "default impl doesn't use deps")]
    fn default_binctx(toolset: &mut Toolset, deps: &Self::Deps) -> Result<BinCtx> {
        Ok(toolset.binctx(Self::BIN.into()))
    }

    /// Sets up a version of the tool that meets the given `ver_req`.
    ///
    /// Returns the specific version and the binary context.
    ///
    /// Implementations must call [`toolset.verify`] to ensure the setup succeeded.
    ///
    /// [`toolset.verify`]: Toolset::verify
    fn set_up(
        toolset: &mut Toolset,
        deps: &Self::Deps,
        ver_req: &VersionReq,
    ) -> Result<(BinCtx, Version)>;

    /// Returns the [`Version`] of the given binary context.
    ///
    /// Returns `None` if the given binary context's path doesn't exist.
    fn extract_version(binctx: &BinCtx) -> Result<Option<Version>> {
        let mut cmd = binctx.cmd();
        cmd.arg("--version");

        ui::print_cmd(&cmd);

        let output = cmd.output();
        if output
            .as_ref()
            .is_err_and(|e| e.kind() == ErrorKind::NotFound)
        {
            return Ok(None);
        }
        let output = output.context(format!("failed to run '{}'", binctx.path().display()))?;
        if output.status.code().is_some_and(|code| code == 1) {
            let error = String::from_utf8(output.stderr).context(format!(
                "'{}' output not valid UTF-8",
                binctx.path().display()
            ))?;
            if error.contains("error") && error.contains("is not installed") {
                return Ok(None);
            }
        }
        ensure!(
            output.status.success(),
            "'{}' failed: {}",
            binctx.path().display(),
            output.status
        );

        let version = String::from_utf8(output.stdout).context(format!(
            "'{}' output not valid UTF-8",
            binctx.path().display()
        ))?;
        let version = version
            .lines()
            .next()
            .context(format!("'{}' output was empty", binctx.path().display()))?;

        let re = Regex::new(r"^\S+\s+(\d+\.\d+\.\d+[^\s]*)")
            .expect("Version extraction regex was incorrect");
        let version = re
            .captures(version)
            .and_then(|c| c.get(1).map(|m| m.as_str()))
            .context(format!(
                "'{}' output didn't contain version",
                binctx.path().display()
            ))?;

        let version = Version::parse(version).context(format!(
            "failed to parse '{}' version '{version}'",
            binctx.path().display()
        ))?;

        Ok(Some(version))
    }
}

/// Binary executable context.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct BinCtx {
    path: PathBuf,
    working_dir: PathBuf,
    environment: Environment,
    args: Vec<String>,
}

impl BinCtx {
    /// Creates a new binary executable context.
    pub fn new(path: PathBuf, working_dir: PathBuf, environment: Environment) -> Self {
        Self {
            path,
            working_dir,
            environment,
            args: Vec::new(),
        }
    }

    /// Returns the binary context with the given base arguments.
    pub fn args(mut self, args: Vec<String>) -> Self {
        self.args = args;
        self
    }

    /// Creates a [`Command`] based on this binary context.
    pub fn cmd(&self) -> Command {
        let mut cmd = Command::new(&self.path);
        cmd.current_dir(&self.working_dir);
        self.environment.apply(&mut cmd);
        cmd.args(&self.args);
        cmd
    }

    /// Returns the underlying binary path.
    pub fn path(&self) -> &Path {
        &self.path
    }
}
