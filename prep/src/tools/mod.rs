// Copyright 2026 the Prep Authors
// SPDX-License-Identifier: Apache-2.0 OR MIT

pub mod cargo;
pub mod ripgrep;
pub mod rustup;

use std::io::ErrorKind;
use std::path::{Path, PathBuf};
use std::process::Command;

use anyhow::{Context, Result, ensure};
use regex::Regex;
use semver::{Version, VersionReq};

use crate::toolset::Toolset;
use crate::ui;

/// Generic Prep tool code.
pub trait Tool: Sized + 'static {
    type Deps;

    const NAME: &str;
    const BIN: &str;

    /// Sets up a version of the tool that meets the given `ver_req`.
    ///
    /// Returns the specific version and the path to the binary.
    ///
    /// Implementations must call [`toolset.verify`] to ensure the setup succeeded.
    ///
    /// [`toolset.verify`]: Toolset::verify
    fn set_up(
        toolset: &mut Toolset,
        deps: &Self::Deps,
        ver_req: &VersionReq,
    ) -> Result<(Version, PathBuf)>;

    /// Returns the [`Version`] of the binary at the given `path`.
    ///
    /// Returns `None` if the given `path` doesn't exist.
    fn extract_version(path: &Path) -> Result<Option<Version>> {
        let mut cmd = Command::new(path);
        let cmd = cmd.arg("-V");

        ui::print_cmd(cmd);

        let output = cmd.output();
        if output
            .as_ref()
            .is_err_and(|e| e.kind() == ErrorKind::NotFound)
        {
            return Ok(None);
        }
        let output = output.context(format!("failed to run '{}'", path.display()))?;
        ensure!(
            output.status.success(),
            "'{}' failed: {}",
            path.display(),
            output.status
        );

        let version = String::from_utf8(output.stdout)
            .context(format!("'{}' output not valid UTF-8", path.display()))?;
        let version = version
            .lines()
            .next()
            .context(format!("'{}' output was empty", path.display()))?;

        let re = Regex::new(r"^\S+\s+(\d+\.\d+\.\d+[^\s]*)")
            .expect("Version extraction regex was incorrect");
        let version = re
            .captures(version)
            .and_then(|c| c.get(1).map(|m| m.as_str()))
            .context(format!(
                "'{}' output didn't contain version",
                path.display()
            ))?;

        let version = Version::parse(version).context(format!(
            "failed to parse '{}' version '{version}'",
            path.display()
        ))?;

        Ok(Some(version))
    }
}
