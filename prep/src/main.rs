// Copyright 2026 the Prep Authors
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! Prepare a Rust project for greatness.

mod cmd;
mod config;
mod host;
mod session;
mod tools;
mod toolset;
mod ui;

use clap::{CommandFactory, FromArgMatches, Parser, Subcommand};

use ui::help;

use crate::cmd::CargoTargets;
use crate::session::Session;

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
        #[arg(short, long)]
        strict: bool,
        #[arg(name = "crates", short, long, value_enum, default_value_t = CargoTargets::Main)]
        targets: CargoTargets,
    },
    #[command()]
    Copyright {
        #[arg(short, long)]
        strict: bool,
    },
    #[command(alias = "fmt")]
    Format {
        #[arg(short, long)]
        strict: bool,
        #[arg(short, long)]
        check: bool,
    },
    #[command()]
    Init {
        #[arg(short, long, default_value_t = false)]
        force: bool,
    },
    #[command()]
    Tools {
        #[command(subcommand)]
        command: Option<ToolsCommands>,
    },
}

#[derive(Subcommand)]
enum ToolsCommands {
    #[command()]
    List,
}

fn main() -> anyhow::Result<()> {
    let ccmd = help::set(Cli::command());
    let matches = ccmd.get_matches();
    let cli = Cli::from_arg_matches(&matches).unwrap();

    let Some(command) = cli.command else {
        ui::print_help(ui::help::root_msg());
        return Ok(());
    };

    let mut session = Session::initialize()?;

    match command {
        Commands::Ci {
            extended,
            no_fail_fast,
        } => cmd::ci::run(&mut session, extended, !no_fail_fast),
        Commands::Clippy { strict, targets } => cmd::clippy::run(&mut session, strict, targets),
        Commands::Copyright { strict } => cmd::copyright::run(&mut session, strict),
        Commands::Format { strict, check } => cmd::format::run(&mut session, strict, check),
        Commands::Init { force } => cmd::init::run(&session, force),
        Commands::Tools { command } => {
            let Some(command) = command else {
                ui::print_help(ui::help::tools_msg());
                return Ok(());
            };
            match command {
                ToolsCommands::List => cmd::tools::list::run(&mut session),
            }
        }
    }
}
