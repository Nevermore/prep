// Copyright 2026 the Prep Authors
// SPDX-License-Identifier: Apache-2.0 OR MIT

use crate::cmd::{clippy, format};

/// Runs CI verification.
///
/// Can be ran in `extended` mode for more thorough checks.
///
/// Set `fail_fast` to `false` to run the checks to the end regardless of failure.
pub fn run(_extended: bool, fail_fast: bool) -> anyhow::Result<()> {
    let mut errs: Vec<anyhow::Error> = Vec::new();
    let mut step = |f: fn() -> anyhow::Result<()>| -> anyhow::Result<()> {
        if let Err(e) = f() {
            if fail_fast {
                return Err(e);
            }
            errs.push(e);
        }
        Ok(())
    };

    step(|| format::run(true))?;
    step(|| clippy::run(true))?;

    if errs.is_empty() {
        Ok(())
    } else {
        let mut msg = String::from("CI verification failed:\n");
        for (i, e) in errs.into_iter().enumerate() {
            msg.push_str(&format!("{}: {:#}\n", i + 1, e));
        }
        Err(anyhow::anyhow!(msg))
    }
}
