// Copyright 2026 the Prep Authors
// SPDX-License-Identifier: Apache-2.0 OR MIT

use clap::Command;
use clap::builder::StyledStr;

use crate::ui;

/// Sets our custom help messages.
pub fn set(cmd: Command) -> Command {
    let cmd = cmd.override_help(root_msg());

    cmd.mut_subcommands(|scmd| {
        let name = scmd.get_name();
        if name == "ci" {
            scmd.override_help(ci_msg())
        } else if name == "format" {
            scmd.override_help(format_msg())
        } else if name == "clippy" {
            scmd.override_help(clippy_msg())
        } else {
            panic!("Sub-command '{name}' help message is not implemented");
        }
    })
}

/// Returns the main help message.
pub fn root_msg() -> StyledStr {
    let (gb, cb, bb) = ui::styles();
    let help = format!(
        "\
Prepare Rust projects for greatness.

{gb}Usage:{gb:#} {cb}prep{cb:#} {bb}[command] [options]{bb:#}

{gb}Commands:{gb:#}
  {cb}ci                  {cb:#}Verify for CI
  {cb}clippy         clp  {cb:#}Analyze with Clippy
  {cb}format         fmt  {cb:#}Format files
  {cb}help                {cb:#}Print help

{gb}Options:{gb:#}
  {cb}--help          -h  {cb:#}Print a help message for the provided command.
  {cb}--version       -V  {cb:#}Print version information.
"
    );

    StyledStr::from(help)
}

/// Returns the `ci` help message.
fn ci_msg() -> StyledStr {
    let (gb, cb, bb) = ui::styles();

    let help = format!(
        "\
Verify the Rust workspace for CI.

{gb}Usage:{gb:#} {cb}prep ci{cb:#} {bb}[options]{bb:#}

{gb}Options:{gb:#}
  {cb}--extended      -e  {cb:#}Run the extended verification suite.
  ····                    ······Good idea for actual CI, rarely useful for local prep.
  {cb}--no-fail-fast  -n  {cb:#}Keep going when encountering an error.
  {cb}--help          -h  {cb:#}Print this help message.
"
    )
    .replace("·", "");

    StyledStr::from(help)
}

/// Returns the `format` help message.
fn format_msg() -> StyledStr {
    let (gb, cb, bb) = ui::styles();

    let help = format!(
        "\
Format the Rust workspace with rustfmt.

{gb}Usage:{gb:#} {cb}prep fmt{cb:#}    {bb}[options]{bb:#}
····      ······ {cb}prep format{cb:#} {bb}[options]{bb:#}

{gb}Options:{gb:#}
  {cb}--check         -c  {cb:#}Verify that the workspace is already formatted.
  {cb}--help          -h  {cb:#}Print this help message.
"
    )
    .replace("·", "");

    StyledStr::from(help)
}

/// Returns the `clippy` help message.
fn clippy_msg() -> StyledStr {
    let (gb, cb, bb) = ui::styles();
    let help = format!(
        "\
Analyze the Rust workspace with Clippy.

{gb}Usage:{gb:#} {cb}prep clp{cb:#}    {bb}[options]{bb:#}
····      ······ {cb}prep clippy{cb:#} {bb}[options]{bb:#}

{gb}Options:{gb:#}
  {cb}--strict        -s  {cb:#}Treat warnings as errors.
  {cb}--help          -h  {cb:#}Print this help message.
"
    )
    .replace("·", "");

    StyledStr::from(help)
}
