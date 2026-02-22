// Copyright 2026 the Prep Authors
// SPDX-License-Identifier: Apache-2.0 OR MIT

use anyhow::{Context, Result, bail};
use semver::{Version, VersionReq};

use crate::tools::{BinCtx, Tool};
use crate::toolset::Toolset;
use crate::ui;

/// Rustup for installing and managing Rust toolchains.
pub struct Rustup;

impl Tool for Rustup {
    type Deps = ();

    const NAME: &str = "rustup";
    const BIN: &str = "rustup";
    const MANAGED: bool = false;

    fn set_up(
        toolset: &mut Toolset,
        deps: &Self::Deps,
        ver_req: &VersionReq,
    ) -> Result<(BinCtx, Version)> {
        // Check if the default Rustup installation already meets the requirement.
        let binctx = Self::default_binctx(toolset, deps)?;

        let Some(version) = toolset
            .verify::<Self>(&binctx, ver_req)
            .context(format!("failed to verify {}", Self::NAME))?
        else {
            ui::print_err(
                "\
				Prep requires rustup to function.\n\
				\n\
				There is no automatic setup implemented for it, sorry.\n\
				Please go to https://rustup.rs/ and install it manually.\n\
				\n\
				If you already have rustup installed then this error here is probably a bug.\n\
				Please report it at https://github.com/Nevermore/prep\n\
				",
            );
            bail!("{} not found", Self::NAME);
        };

        Ok((binctx, version))
    }
}
