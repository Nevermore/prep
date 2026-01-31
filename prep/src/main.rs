// Copyright 2026 the Prep Authors
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! Prepare a Rust project for greatness.

mod cmd;
mod ui;

use clap::{CommandFactory, Parser, Subcommand};

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Clippy analysis
    Clippy,
    /// Format files
    Format,
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    let Some(command) = cli.command else {
        Cli::command().print_help().unwrap();
        return Ok(());
    };

    match command {
        Commands::Clippy => cmd::clippy::run(),
        Commands::Format => cmd::format::run(),
    }
}
