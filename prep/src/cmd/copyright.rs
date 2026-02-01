// Copyright 2026 the Prep Authors
// SPDX-License-Identifier: Apache-2.0 OR MIT

use std::process::Command;

use anyhow::{Context, bail, ensure};
use time::UtcDateTime;

use crate::ui;
use crate::ui::style::{ERROR, HEADER, LITERAL, NOTE};

// TODO: Allow configuring the regex
// TODO: Allow excluding files from the check
// TODO: ALlow configuring the project name
// TODO: Allow configuring the license (or fetch it from Cargo.toml per-package?)

const REGEX: &str = r#"^// Copyright (19|20)[\d]{2} (.+ and )?the Prep Authors( and .+)?$\n^// SPDX-License-Identifier: Apache-2\.0 OR MIT$\n\n"#;

/// Verify copyright headers.
pub fn run() -> anyhow::Result<()> {
    let mut cmd = Command::new("rg");
    let cmd = cmd
        .arg(REGEX)
        .arg("--files-without-match")
        .arg("--multiline")
        .args(["-g", "*.rs"])
        .arg(".");

    ui::print_cmd(cmd);

    let output = cmd.output().context("failed to run ripgrep")?;

    // ripgrep exits with code 1 in case of no matches, code 2 in case of error
    ensure!(
        output.status.success() || output.status.code().is_some_and(|code| code == 1),
        "ripgrep failed: {}",
        output.status
    );

    if !output.stdout.is_empty() {
        print_missing(String::from_utf8(output.stdout).unwrap());
        bail!("failed copyright header verification");
    }

    let h = HEADER;
    eprintln!("    {h}Verified{h:#} all source files have correct copyright headers.");

    Ok(())
}

fn print_missing(msg: String) {
    let (e, l, n) = (ERROR, LITERAL, NOTE);
    let year = UtcDateTime::now().year();

    eprintln!("{e}The following files lack the correct copyright header:{e:#}");
    eprintln!("{l}{msg}{l:#}");
    eprintln!("{n}Please add the following header:{n:#}\n");
    eprintln!("// Copyright {year} the Prep Authors");
    eprintln!("// SPDX-License-Identifier: Apache-2.0 OR MIT");
    eprintln!("\n... rest of the file ...\n");
}
