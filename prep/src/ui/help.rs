// Copyright 2026 the Prep Authors
// SPDX-License-Identifier: Apache-2.0 OR MIT

use clap::Command;
use clap::builder::StyledStr;

use crate::ui::style::{HEADER, LITERAL, PLACEHOLDER};

/// Sets our custom help messages.
pub fn set(cmd: Command) -> Command {
    let cmd = cmd.override_help(root_msg());

    cmd.mut_subcommands(|scmd| {
        let name = scmd.get_name();
        if name == "ci" {
            scmd.override_help(ci_msg())
        } else if name == "clippy" {
            scmd.override_help(clippy_msg())
        } else if name == "copyright" {
            scmd.override_help(copyright_msg())
        } else if name == "format" {
            scmd.override_help(format_msg())
        } else if name == "init" {
            scmd.override_help(init_msg())
        } else if name == "tools" {
            scmd.override_help(tools_msg())
        } else {
            panic!("Sub-command '{name}' help message is not implemented");
        }
    })
}

/// Returns the main help message.
pub fn root_msg() -> StyledStr {
    let (h, l, p) = (HEADER, LITERAL, PLACEHOLDER);
    let help = format!(
        "\
Prepare Rust projects for greatness.

{h}Usage:{h:#} {l}prep{l:#} {p}[command] [options]{p:#}

{h}Commands:{h:#}
  {l}     ci              {l:#}Verify for CI.
  {l}clp  clippy          {l:#}Analyze with Clippy.
  {l}     copyright       {l:#}Verify copyright headers.
  {l}fmt  format          {l:#}Format with rustfmt.
  {l}     init            {l:#}Initialize Prep configuration.
  {l}     help            {l:#}Print help for the provided command.

{h}Options:{h:#}
  {l}-h   --help          {l:#}Print help for the provided command.
  {l}-V   --version       {l:#}Print version information.
"
    );

    StyledStr::from(help)
}

/// Returns the `ci` help message.
fn ci_msg() -> StyledStr {
    let (h, l, p) = (HEADER, LITERAL, PLACEHOLDER);
    let help = format!(
        "\
Verify the Rust workspace for CI.

{h}Usage:{h:#} {l}prep ci{l:#} {p}[options]{p:#}

{h}Options:{h:#}
  {l}-e   --extended      {l:#}Run the extended verification suite.
  ···                     ·····Good idea for actual CI, rarely useful for local prep.
  {l}-n   --no-fail-fast  {l:#}Keep going when encountering an error.
  {l}-h   --help          {l:#}Print this help message.
"
    )
    .replace("·", "");

    StyledStr::from(help)
}

/// Returns the `clippy` help message.
fn clippy_msg() -> StyledStr {
    let (h, l, p) = (HEADER, LITERAL, PLACEHOLDER);
    let help = format!(
        "\
Analyze the Rust workspace with Clippy.

{h}Usage:{h:#} {l}prep clp{l:#}    {p}[options]{p:#}
···      ····· {l}prep clippy{l:#} {p}[options]{p:#}

{h}Options:{h:#}
  {l}-s   --strict        {l:#}Use locked Rust toolchain version and treat warnings as errors.
  {l}-c   --crates <val>  {l:#}Target specified crates. Possible values:
  ···                     ·····{p}main{p:#} -> Binaries and the main library. (default)
  ···                     ·····{p}aux{p:#}  -> Examples, tests, and benches.
  ···                     ·····{p}all{p:#}  -> All of the above.
  {l}-h   --help          {l:#}Print this help message.
"
    )
    .replace("·", "");

    StyledStr::from(help)
}

/// Returns the `copyright` help message.
fn copyright_msg() -> StyledStr {
    let (h, l) = (HEADER, LITERAL);
    let help = format!(
        "\
Verify that all Rust source files have the correct copyright header.

{h}Usage:{h:#} {l}prep copyright{l:#}

{h}Options:{h:#}
  {l}-s   --strict        {l:#}Use locked ripgrep version.
  {l}-h   --help          {l:#}Print this help message.
"
    )
    .replace("·", "");

    StyledStr::from(help)
}

/// Returns the `format` help message.
fn format_msg() -> StyledStr {
    let (h, l, p) = (HEADER, LITERAL, PLACEHOLDER);
    let help = format!(
        "\
Format the Rust workspace with rustfmt.

{h}Usage:{h:#} {l}prep fmt{l:#}    {p}[options]{p:#}
···      ····· {l}prep format{l:#} {p}[options]{p:#}

{h}Options:{h:#}
  {l}-s   --strict        {l:#}Use locked Rust toolchain version.
  {l}-c   --check         {l:#}Verify that the workspace is already formatted.
  {l}-h   --help          {l:#}Print this help message.
"
    )
    .replace("·", "");

    StyledStr::from(help)
}

/// Returns the `init` help message.
fn init_msg() -> StyledStr {
    let (h, l, p) = (HEADER, LITERAL, PLACEHOLDER);
    let help = format!(
        "\
Initialize Prep configuration for this Rust workspace.

{h}Usage:{h:#} {l}prep init{l:#} {p}[options]{p:#}

{h}Options:{h:#}
  {l}-f   --force         {l:#}Overwrite existing configuration.
  {l}-h   --help          {l:#}Print this help message.
"
    )
    .replace("·", "");

    StyledStr::from(help)
}

/// Returns the tools help message.
pub fn tools_msg() -> StyledStr {
    let (h, l, p) = (HEADER, LITERAL, PLACEHOLDER);
    let help = format!(
        "\
Manage all the tools that Prep uses.

{h}Usage:{h:#} {l}prep tools{l:#} {p}[command] [options]{p:#}

{h}Commands:{h:#}
  {l}     list            {l:#}List information about all the tools.
  {l}     help            {l:#}Print help for the provided command.

{h}Options:{h:#}
  {l}-h   --help          {l:#}Print help for the provided command.
"
    );

    StyledStr::from(help)
}
