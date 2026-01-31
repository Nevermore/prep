// Copyright 2026 the Prep Authors
// SPDX-License-Identifier: Apache-2.0 OR MIT

use clap::Command;
use clap::builder::StyledStr;
use clap::builder::styling::{Color, Style};

/// Sets our custom help messages.
pub fn set(cmd: Command) -> Command {
    let cmd = cmd.override_help(root_msg());

    cmd.mut_subcommands(|mut scmd| {
        let name = scmd.get_name();
        if name == "format" {
            scmd = scmd.override_help(format_msg());
        } else if name == "clippy" {
            scmd = scmd.override_help(clippy_msg());
        } else {
            panic!("Sub-command '{name}' help message is not implemented");
        }
        scmd
    })
}

fn color(index: u8) -> Option<Color> {
    Some(Color::Ansi256(index.into()))
}

/// Returns `(header, cmd_or_arg, optional)` styles.
///
/// These correspond to `(green, cyan, blue)`.
fn styles() -> (Style, Style, Style) {
    let g = Style::new().fg_color(color(10)); // Green
    let c = Style::new().fg_color(color(14)); // Cyan
    let b = Style::new().fg_color(color(39)); // Blue
    (g, c, b)
}

/// Returns the main help message.
pub fn root_msg() -> StyledStr {
    let (gb, cb, bb) = styles();
    let help = format!(
        "\
Prepare Rust projects for greatness.

{gb}Usage:{gb:#} {cb}prep{cb:#} {bb}[command] [options]{bb:#}

{gb}Commands:{gb:#}
  {cb}clp  clippy    {cb:#}Clippy analysis
  {cb}fmt  format    {cb:#}Format files
  {cb}     help      {cb:#}Print help

{gb}Options:{gb:#}
  {cb}-h  --help     {cb:#}Print help
  {cb}-V  --version  {cb:#}Print version
"
    );

    StyledStr::from(help)
}

/// Returns the `format` help message.
fn format_msg() -> StyledStr {
    let (gb, cb, bb) = styles();

    let help = format!(
        "\
Format the Rust workspace with rustfmt.

{gb}Usage:{gb:#} {cb}prep fmt{cb:#}    {bb}[options]{bb:#}
····      ······ {cb}prep format{cb:#} {bb}[options]{bb:#}

{gb}Options:{gb:#}
  {cb}-h  --help     {cb:#}Print help
"
    )
    .replace("·", "");

    StyledStr::from(help)
}

/// Returns the `clippy` help message.
fn clippy_msg() -> StyledStr {
    let (gb, cb, bb) = styles();
    let help = format!(
        "\
Analyze the Rust workspace with Clippy.

{gb}Usage:{gb:#} {cb}prep clp{cb:#}    {bb}[options]{bb:#}
····      ······ {cb}prep clippy{cb:#} {bb}[options]{bb:#}

{gb}Options:{gb:#}
  {cb}-h  --help     {cb:#}Print help
"
    )
    .replace("·", "");

    StyledStr::from(help)
}
