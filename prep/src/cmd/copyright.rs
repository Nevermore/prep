// Copyright 2026 the Prep Authors
// SPDX-License-Identifier: Apache-2.0 OR MIT

use std::process::Command;

use anyhow::{Context, bail, ensure};
use time::UtcDateTime;

use crate::session::Session;
use crate::ui;
use crate::ui::style::{ERROR, HEADER, LITERAL, NOTE};

// TODO: Allow configuring the regex
// TODO: Allow excluding files from the check

/// Verify copyright headers.
///
/// In `strict` mode ripgrep version is locked.
pub fn run(session: &mut Session, _strict: bool) -> anyhow::Result<()> {
    let config = session.config();
    let project = config.project();
    let header_regex = header_regex(project.name(), project.license());

    // TODO: Strict mode for ripgrep.
    let mut cmd = Command::new("rg");
    let cmd = cmd
        .current_dir(session.root_dir())
        .arg(header_regex)
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
        print_missing(
            project.name(),
            project.license(),
            String::from_utf8(output.stdout).unwrap(),
        );
        bail!("failed copyright header verification");
    }

    let h = HEADER;
    eprintln!("    {h}Verified{h:#} all source files have correct copyright headers.");

    Ok(())
}

fn header_regex(name: &str, license: &str) -> String {
    let name = regex::escape(name);
    let license = regex::escape(license);

    let mut re = String::new();
    re.push_str(r#"^// Copyright (19|20)[\d]{2} (.+ and )?the "#);
    re.push_str(&name);
    re.push_str(r#" Authors( and .+)?$\n^// SPDX-License-Identifier: "#);
    re.push_str(&license);
    re.push_str(r#"$\n\n"#);
    re
}

fn print_missing(name: &str, license: &str, msg: String) {
    let (e, l, n) = (ERROR, LITERAL, NOTE);
    let year = UtcDateTime::now().year();

    eprintln!("{e}The following files lack the correct copyright header:{e:#}");
    eprintln!("{l}{msg}{l:#}");
    eprintln!("{n}Please add the following header:{n:#}\n");
    eprintln!("// Copyright {year} the {name} Authors");
    eprintln!("// SPDX-License-Identifier: {license}");
    eprintln!("\n... rest of the file ...\n");
}
