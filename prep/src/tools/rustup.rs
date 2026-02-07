// Copyright 2026 the Prep Authors
// SPDX-License-Identifier: Apache-2.0 OR MIT

use std::io::ErrorKind;
use std::process::Command;
use std::sync::RwLock;

use anyhow::{Context, Result, bail, ensure};

use crate::ui;

/// Rustup executable name.
const BIN: &str = "rustup";

/// Whether rustup is ready to use.
static READY: RwLock<bool> = RwLock::new(false);

/// Returns the rustup command.
pub fn new() -> Result<Command> {
    if !*READY.read().expect("rustup setup lock poisoned") {
        let mut ready = READY.write().expect("rustup setup lock poisoned");
        if !*ready {
            set_up().context("failed to set up rustup")?;
            *ready = true;
        }
    }
    Ok(Command::new(BIN))
}

/// Ensures that rustup is installed and ready to use.
pub fn set_up() -> Result<()> {
    // Check if rustup is already available
    let found = verify()?;
    if !found {
        ui::print_err(
            "\
			Prep requires rustup v1 to function.\n\
			\n\
			There is no automatic setup implemented for it, sorry.\n\
			Please go to https://rustup.rs/ and install it manually.\n\
			\n\
			If you already have rustup installed then this error here is probably a bug.\n\
			Please report it at https://github.com/Nevermore/prep\n\
			",
        );
        bail!("rustup not found");
    }
    Ok(())
}

/// Returns `true` if rustup v1 was found, `false` if no rustup was found.
///
/// Other versions will return an error.
pub fn verify() -> Result<bool> {
    let mut cmd = Command::new(BIN);
    let cmd = cmd.arg("-V");

    ui::print_cmd(cmd);

    let output = cmd.output();
    if output
        .as_ref()
        .is_err_and(|e| e.kind() == ErrorKind::NotFound)
    {
        return Ok(false);
    }
    let output = output.context("failed to run rustup")?;
    ensure!(output.status.success(), "rustup failed: {}", output.status);

    let version = String::from_utf8(output.stdout).context("rustup output not valid UTF-8")?;

    if !version.starts_with("rustup 1.") {
        bail!("expected rustup v1, got: {version}");
    }

    Ok(true)
}
