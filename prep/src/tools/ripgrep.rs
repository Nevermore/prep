// Copyright 2026 the Prep Authors
// SPDX-License-Identifier: Apache-2.0 OR MIT

use std::fs;
use std::path::{Path, PathBuf};

use anyhow::{Context, Result, bail, ensure};
use semver::{Op, Version, VersionReq};

use crate::tools::Tool;
use crate::tools::cargo::{Cargo, CargoDeps};
use crate::toolset::Toolset;
use crate::{host, ui};

/// Ripgrep.
pub struct Ripgrep;

/// Ripgrep dependencies.
pub struct RipgrepDeps {
    /// Cargo dependencies.
    cargo_deps: CargoDeps,
    /// Cargo version requirement.
    cargo_ver_req: Option<VersionReq>,
}

impl RipgrepDeps {
    /// Creates new Ripgrep dependency requirements.
    ///
    /// `None` means that the default version will be used.
    pub fn new(cargo_deps: CargoDeps, cargo_ver_req: impl Into<Option<VersionReq>>) -> Self {
        Self {
            cargo_deps,
            cargo_ver_req: cargo_ver_req.into(),
        }
    }
}

impl Tool for Ripgrep {
    type Deps = RipgrepDeps;

    const NAME: &str = "ripgrep";
    const BIN: &str = "rg";

    fn set_up(
        toolset: &mut Toolset,
        deps: &Self::Deps,
        ver_req: &VersionReq,
    ) -> Result<(Version, PathBuf)> {
        if ver_req.comparators.len() != 1 {
            bail!(
                "Only simple `=MAJOR.MINOR.PATCH` version requirements \
                are supported for ripgrep, got: {}",
                ver_req
            );
        }
        let ver_req_comp = ver_req.comparators.first().unwrap();
        if ver_req_comp.op != Op::Exact
            || ver_req_comp.minor.is_none()
            || ver_req_comp.patch.is_none()
            || !ver_req_comp.pre.is_empty()
        {
            bail!(
                "Only simple `=MAJOR.MINOR.PATCH` version requirements \
                are supported for ripgrep, got: {}",
                ver_req_comp
            );
        }
        let version = Version::new(
            ver_req_comp.major,
            ver_req_comp.minor.unwrap(),
            ver_req_comp.patch.unwrap(),
        );

        // Prepare the install directory
        let install_dir = toolset.install_dir(Self::NAME, &version);
        if install_dir.exists() {
            if !empty_dir(&install_dir)? {
                bail!(
                    "{} install directory '{}' unexpectedly already exists \
                    and is not an empty directory, aborting for safety.",
                    Self::NAME,
                    install_dir.display()
                );
            }
        } else {
            fs::create_dir_all(&install_dir).context(format!(
                "failed to create install directory '{}'",
                install_dir.display()
            ))?;
        }

        // Install it with Cargo
        let mut cmd = toolset.get::<Cargo>(&deps.cargo_deps, deps.cargo_ver_req.as_ref())?;

        let temp_install_dir = toolset.temp_install_dir(Self::NAME);
        if temp_install_dir.exists() && !empty_dir(&temp_install_dir)? {
            bail!(
                "Temporary {} install directory '{}' unexpectedly already exists \
                and is not an empty directory, aborting for safety.",
                Self::NAME,
                temp_install_dir.display()
            );
        }

        let cmd = cmd
            .arg("install")
            .arg(Self::NAME)
            .arg("--locked")
            .args(["--version", &version.to_string()])
            .arg("--root")
            .arg(temp_install_dir.as_os_str());

        ui::print_cmd(cmd);

        let status = cmd.status().context("failed to run cargo install")?;
        ensure!(status.success(), "cargo install failed: {status}");

        // Copy the binary to the install directory
        let manifest_a = temp_install_dir.join(".crates.toml");
        let manifest_b = temp_install_dir.join(".crates2.json");
        let bin_name = host::executable_name(Self::BIN);
        let bin_src_dir = temp_install_dir.join("bin");
        let bin_src = bin_src_dir.join(&bin_name);
        let bin_dst = install_dir.join(&bin_name);

        if !bin_src.exists() {
            bail!(
                "{} binary at '{}' unexpectedly not found, aborting.",
                Self::NAME,
                bin_src.display()
            );
        }
        if bin_dst.exists() {
            bail!(
                "{} binary at '{}' unexpectedly already exists, aborting.",
                Self::NAME,
                bin_dst.display()
            );
        }
        fs::copy(&bin_src, &bin_dst).context(format!(
            "failed to copy {} binary from '{}' to '{}'",
            Self::NAME,
            bin_src.display(),
            bin_dst.display()
        ))?;

        // Safely clean up the temporary directory
        fs::remove_file(&bin_src).context(format!(
            "failed to remove {} binary at '{}'",
            Self::NAME,
            bin_src.display()
        ))?;
        fs::remove_dir(&bin_src_dir).context(format!(
            "failed to remove temporary directory '{}'",
            bin_src_dir.display()
        ))?;
        fs::remove_file(&manifest_a).context(format!(
            "failed to remove temporary manifest file at '{}'",
            manifest_a.display()
        ))?;
        fs::remove_file(&manifest_b).context(format!(
            "failed to remove temporary manifest file at '{}'",
            manifest_b.display()
        ))?;
        fs::remove_dir(&temp_install_dir).context(format!(
            "failed to remove temporary directory '{}'",
            temp_install_dir.display()
        ))?;

        // Verify that the installed version is correct
        let version = toolset
            .verify::<Self>(&bin_dst, ver_req)
            .context(format!("failed to verify {}", Self::NAME))?;
        let Some(version) = version else {
            bail!(
                "'{}' was just installed but now was no longer found",
                bin_dst.display()
            );
        };
        Ok((version, bin_dst))
    }
}

/// Returns `true` if `path` is a directory and is empty.
fn empty_dir(path: &Path) -> Result<bool> {
    if !path.is_dir() {
        return Ok(false);
    }
    let read_dir = path
        .read_dir()
        .context(format!("failed to read directory '{}'", path.display()))?;
    Ok(read_dir.count() == 0)
}
