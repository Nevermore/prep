// Copyright 2026 the Prep Authors
// SPDX-License-Identifier: Apache-2.0 OR MIT

use crate::cmd::{CargoTargets, clippy, copyright, format};
use crate::session::Session;

/// Runs CI verification.
///
/// Can be ran in `extended` mode for more thorough checks.
///
/// Set `fail_fast` to `false` to run the checks to the end regardless of failure.
pub fn run(session: &mut Session, extended: bool, fail_fast: bool) -> anyhow::Result<()> {
    let mut errs: Vec<anyhow::Error> = Vec::new();
    let mut step = |f: &mut dyn FnMut() -> anyhow::Result<()>| -> anyhow::Result<()> {
        if let Err(e) = f() {
            if fail_fast {
                return Err(e);
            }
            errs.push(e);
        }
        Ok(())
    };

    step(&mut || copyright::run(session, true))?;
    step(&mut || format::run(session, true, true))?;

    if extended {
        // We need to avoid --all-targets because it will unify dev and regular dep features.
        step(&mut || clippy::run(session, true, CargoTargets::Main))?;
        step(&mut || clippy::run(session, true, CargoTargets::Auxiliary))?;
    } else {
        // Slightly faster due to shared build cache,
        // but will miss unified feature bugs.
        step(&mut || clippy::run(session, true, CargoTargets::All))?;
    }

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
