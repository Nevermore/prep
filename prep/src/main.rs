// Copyright 2026 the Prep Authors
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! Prepare a Rust project for greatness.

mod cmd;
mod ui;

use clap::{CommandFactory, FromArgMatches, Parser, Subcommand};

use ui::help;

use crate::cmd::CargoTargets;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    #[command()]
    Ci {
        #[arg(short, long)]
        extended: bool,
        #[arg(short, long)]
        no_fail_fast: bool,
    },
    #[command(alias = "clp")]
    Clippy {
        #[arg(name = "crates", short, long, value_enum, default_value_t = CargoTargets::Main)]
        targets: CargoTargets,
        #[arg(short, long)]
        strict: bool,
    },
    #[command()]
    Copyright,
    #[command(alias = "fmt")]
    Format {
        #[arg(short, long)]
        check: bool,
    },
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
        Commands::Ci {
            extended,
            no_fail_fast,
        } => cmd::ci::run(extended, !no_fail_fast),
        Commands::Clippy { targets, strict } => cmd::clippy::run(targets, strict),
        Commands::Copyright => cmd::copyright::run(),
        Commands::Format { check } => cmd::format::run(check),
    }
}
