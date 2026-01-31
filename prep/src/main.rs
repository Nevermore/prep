// Copyright 2026 the Prep Authors
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! Prepare a Rust project for greatness.

mod cmd;
mod ui;

use clap::{CommandFactory, FromArgMatches, Parser, Subcommand};

use ui::help;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    #[command(alias = "clp")]
    Clippy,
    #[command(alias = "fmt")]
    Format,
}

fn main() -> anyhow::Result<()> {
    let ccmd = help::set(Cli::command());
    let matches = ccmd.get_matches();
    let cli = Cli::from_arg_matches(&matches).unwrap();

    let Some(command) = cli.command else {
        ui::print_help();
        return Ok(());
    };

    match command {
        Commands::Clippy => cmd::clippy::run(),
        Commands::Format => cmd::format::run(),
    }
}
