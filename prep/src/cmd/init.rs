// Copyright 2026 the Prep Authors
// SPDX-License-Identifier: Apache-2.0 OR MIT

use anyhow::Result;

use crate::session::Session;
use crate::ui;

/// Initialize the prep configuration
pub fn run(session: &Session, force: bool) -> Result<()> {
    if !force && session.config_path().exists() {
        ui::print_err(
            "Prep configuration already exists, aborting.\n\
			Use --force if you intended to overwrite the previous config.",
        );
        return Ok(());
    }

    // TODO: Instead of just saving the session's config values,
    //       run an interactive TUI for choosing overrides.
    session.save_config()?;

    Ok(())
}
