// Copyright 2026 the Prep Authors
// SPDX-License-Identifier: Apache-2.0 OR MIT

use anyhow::Result;

use crate::session::Session;
use crate::tools::Tool;
use crate::tools::cargo::Cargo;
use crate::tools::ripgrep::Ripgrep;
use crate::tools::rustup::Rustup;
use crate::toolset::Toolset;
use crate::ui::style::TABLE_HEADER;

/// List information on all the tools in the toolset.
pub fn run(session: &mut Session) -> Result<()> {
    let tools = session.config().tools();

    let rustup_locked = format!("{}", tools.rustup());
    let rust_locked = format!("{}", tools.rust());
    let rg_locked = format!("{}", tools.ripgrep());

    let toolset = session.toolset();

    let rustup_global = default_version::<Rustup>(toolset)?;
    let rust_global = default_version::<Cargo>(toolset)?;
    let rg_global = default_version::<Ripgrep>(toolset)?;

    fn cell(s: &str, len: usize) -> String {
        let mut s = String::from(s);
        s.push_str(&" ".repeat(len.saturating_sub(s.len())));
        s
    }

    const NLEN: usize = 7;
    const LLEN: usize = 16;
    const GLEN: usize = 15;

    let h = TABLE_HEADER;
    let info = format!(
        "\
{h}Name{h:#}     {h}Required version{h:#}  {h}Default version{h:#}
···{}··········  ···{}···················  ···{}··················
···{}··········  ···{}···················  ···{}··················
···{}··········  ···{}···················  ···{}··················
",
        cell("Rustup", NLEN),
        cell(rustup_locked.trim_start_matches('='), LLEN),
        cell(&rustup_global, GLEN),
        cell("Rust", NLEN),
        cell(rust_locked.trim_start_matches('='), LLEN),
        cell(&rust_global, GLEN),
        cell("Ripgrep", NLEN),
        cell(rg_locked.trim_start_matches('='), LLEN),
        cell(&rg_global, GLEN),
    )
    .replace("·", "");

    eprint!("{}", info);

    Ok(())
}

fn default_version<T: Tool>(toolset: &mut Toolset) -> Result<String> {
    let deps = T::Deps::default();
    let binctx = T::default_binctx(toolset, &deps)?;
    let version = toolset
        .version::<T>(&binctx)?
        .map(|v| format!("{v}"))
        .unwrap_or_else(|| "None".into());
    Ok(version)
}
