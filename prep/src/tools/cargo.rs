// Copyright 2026 the Prep Authors
// SPDX-License-Identifier: Apache-2.0 OR MIT

use std::collections::HashMap;
use std::io::ErrorKind;
use std::process::Command;
use std::sync::{LazyLock, RwLock};

use anyhow::{Context, Result, bail, ensure};

use crate::tools::rustup;
use crate::ui;

/// Cargo executable name.
const BIN: &str = "cargo";

/// Toolchain version -> Cargo path
static PATHS: LazyLock<RwLock<HashMap<String, String>>> =
    LazyLock::new(|| RwLock::new(HashMap::new()));

/// Returns the cargo command.
///
/// Provide an empty `version` to pick the default cargo version.
pub fn new(version: &str) -> Result<Command> {
    let paths = PATHS.read().expect("cargo setup lock poisoned");
    if let Some(path) = paths.get(version) {
        return Ok(Command::new(path));
    }
    drop(paths);
    let mut paths = PATHS.write().expect("cargo setup lock poisoned");
    if let Some(path) = paths.get(version) {
        return Ok(Command::new(path));
    }
    let path = set_up(version)?;
    let cmd = Command::new(&path);
    paths.insert(version.into(), path);
    Ok(cmd)
}

/// Ensures that Cargo is installed and ready to use.
pub fn set_up(version: &str) -> Result<String> {
    // TODO: Call rustup toolchain install with correct components etc first

    let mut cmd = rustup::new()?;
    let mut cmd = cmd.arg("which").arg(BIN);
    if !version.is_empty() {
        cmd = cmd.args(["--toolchain", version]);
    }

    ui::print_cmd(cmd);

    let output = cmd.output().context("failed to run rustup")?;
    ensure!(output.status.success(), "rustup failed: {}", output.status);

    let path = String::from_utf8(output.stdout).context("rustup output not valid UTF-8")?;
    let path = path.trim();

    if !verify(path, version)? {
        bail!("cargo not found");
    }

    Ok(path.into())
}

/// Returns `true` if Cargo was found, `false` if no Cargo was found.
///
/// Other versions will return an error.
pub fn verify(path: &str, version: &str) -> Result<bool> {
    let mut cmd = Command::new(path);
    let cmd = cmd.arg("-V");

    ui::print_cmd(cmd);

    let output = cmd.output();
    if output
        .as_ref()
        .is_err_and(|e| e.kind() == ErrorKind::NotFound)
    {
        return Ok(false);
    }
    let output = output.context("failed to run cargo")?;
    ensure!(output.status.success(), "cargo failed: {}", output.status);

    let cmd_version = String::from_utf8(output.stdout).context("cargo output not valid UTF-8")?;

    let expected = format!("cargo {version}");
    if !cmd_version.starts_with(&expected) {
        bail!("expected {expected}, got: {cmd_version}");
    }

    Ok(true)
}
