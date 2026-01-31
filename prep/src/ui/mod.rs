// Copyright 2026 the Prep Authors
// SPDX-License-Identifier: Apache-2.0 OR MIT

pub mod help;

use std::ffi::OsStr;
use std::process::Command;

/// Prints the binary name and its arguments to stderr.
pub fn print_cmd(cmd: &Command) {
    let bin = cmd.get_program();
    let args = cmd.get_args().collect::<Vec<_>>().join(OsStr::new(" "));

    eprintln!("     Running `{} {}`", bin.display(), args.display());
}

pub fn print_help() {
    // TODO: Don't print ANSI codes when not supported by the environment.
    eprint!("{}", help::root_msg().ansi());
}
