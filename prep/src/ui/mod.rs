// Copyright 2026 the Prep Authors
// SPDX-License-Identifier: Apache-2.0 OR MIT

pub mod help;

use std::ffi::OsStr;
use std::process::Command;

use clap::builder::StyledStr;
use clap::builder::styling::{Color, Style};

/// Prints the binary name and its arguments to stderr.
pub fn print_cmd(cmd: &Command) {
    let bin = cmd.get_program();
    let args = cmd.get_args().collect::<Vec<_>>().join(OsStr::new(" "));

    let (g, _, _) = styles();

    let msg = format!(
        "     {g}Running{g:#} `{} {}`",
        bin.display(),
        args.display()
    );
    let msg = StyledStr::from(msg);

    eprintln!("{}", msg.ansi());
}

/// Prints the main help message.
pub fn print_help() {
    // TODO: Don't print ANSI codes when not supported by the environment.
    eprint!("{}", help::root_msg().ansi());
}

/// Returns the corresponding ANSI 256-bit color.
pub fn color(index: u8) -> Option<Color> {
    Some(Color::Ansi256(index.into()))
}

/// Returns `(header, cmd_or_arg, optional)` styles.
///
/// These correspond to `(green, cyan, blue)`.
pub fn styles() -> (Style, Style, Style) {
    // TODO: Verify that especially the green color matches Cargo on non-Windows terminals.
    let g = Style::new().fg_color(color(10)); // Green
    let c = Style::new().fg_color(color(14)); // Cyan
    let b = Style::new().fg_color(color(39)); // Blue
    (g, c, b)
}
