// Copyright 2026 the Prep Authors
// SPDX-License-Identifier: Apache-2.0 OR MIT

pub mod help;
pub mod style;

use std::ffi::OsStr;
use std::process::Command;

use clap::builder::StyledStr;

/// Prints lines aligned lines with only the first line getting the header.
pub fn print_lines(header: &str, lines: &str) {
    for (idx, line) in lines.split("\n").enumerate() {
        if idx == 0 {
            eprintln!("{header} {line}");
        } else {
            eprintln!("             {line}");
        }
    }
}

/// Prints the binary name and its arguments to stderr.
pub fn print_cmd(cmd: &Command) {
    let bin = cmd.get_program();
    let args = cmd.get_args().collect::<Vec<_>>().join(OsStr::new(" "));

    let h = style::HEADER;
    eprintln!(
        "     {h}Running{h:#} `{} {}`",
        bin.display(),
        args.display()
    );
}

/// Prints the error with a colored prefix.
pub fn print_err(err: &str) {
    let e = style::ERROR;
    let header = format!("       {e}Error{e:#}");
    print_lines(&header, err);
}

/// Prints the warning with a colored prefix.
pub fn print_warn(warn: &str) {
    let w = style::WARN;
    let header = format!("     {w}Warning{w:#}");
    print_lines(&header, warn);
}

/// Prints the main help message.
pub fn print_help(msg: StyledStr) {
    // TODO: Don't print ANSI codes when not supported by the environment.
    eprint!("{}", msg.ansi());
}
