// Copyright 2026 the Prep Authors
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! Color styles based on Cargo color choices.

#![allow(unused)]

use clap::builder::styling::{AnsiColor, Style};

const USE_WINDOWS_COLORS: bool = cfg!(windows);
const BRIGHT_BLUE: Style = if USE_WINDOWS_COLORS {
    AnsiColor::BrightCyan.on_default()
} else {
    AnsiColor::BrightBlue.on_default()
};
const YELLOW: Style = if USE_WINDOWS_COLORS {
    AnsiColor::BrightYellow.on_default()
} else {
    AnsiColor::Yellow.on_default()
};
pub const EMPHASIS: Style = if USE_WINDOWS_COLORS {
    AnsiColor::BrightWhite.on_default()
} else {
    Style::new()
}
.bold();

pub const NOP: Style = Style::new();
pub const INFO: Style = BRIGHT_BLUE.bold();
pub const NOTE: Style = AnsiColor::BrightGreen.on_default().bold();
pub const HELP: Style = AnsiColor::BrightCyan.on_default().bold();
pub const WARN: Style = YELLOW.bold();
pub const ERROR: Style = AnsiColor::BrightRed.on_default().bold();
pub const GOOD: Style = AnsiColor::BrightGreen.on_default().bold();
pub const VALID: Style = AnsiColor::BrightCyan.on_default().bold();
pub const INVALID: Style = WARN;
pub const TRANSIENT: Style = HELP;
pub const HEADER: Style = AnsiColor::BrightGreen.on_default().bold();
pub const USAGE: Style = AnsiColor::BrightGreen.on_default().bold();
pub const LITERAL: Style = AnsiColor::BrightCyan.on_default().bold();
pub const PLACEHOLDER: Style = AnsiColor::Cyan.on_default();
pub const LINE_NUM: Style = BRIGHT_BLUE.bold();
pub const CONTEXT: Style = BRIGHT_BLUE.bold();
pub const ADDITION: Style = AnsiColor::BrightGreen.on_default();
pub const REMOVAL: Style = AnsiColor::BrightRed.on_default();

pub const UPDATE_ADDED: Style = NOTE;
pub const UPDATE_REMOVED: Style = ERROR;
pub const UPDATE_UPGRADED: Style = GOOD;
pub const UPDATE_DOWNGRADED: Style = WARN;
pub const UPDATE_UNCHANGED: Style = Style::new().bold();

pub const DEP_NORMAL: Style = Style::new().dimmed();
pub const DEP_BUILD: Style = AnsiColor::Blue.on_default().bold();
pub const DEP_DEV: Style = AnsiColor::Cyan.on_default().bold();
pub const DEP_FEATURE: Style = AnsiColor::Magenta.on_default().dimmed();

pub const TABLE_HEADER: Style = Style::new().bold().underline();
