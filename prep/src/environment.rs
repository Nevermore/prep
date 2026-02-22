// Copyright 2026 the Prep Authors
// SPDX-License-Identifier: Apache-2.0 OR MIT

use std::collections::BTreeMap;
use std::process::Command;

/// Set of environment variables for running a binary.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Environment {
    vars: BTreeMap<String, String>,
}

impl Environment {
    /// Creates a new default set of environment variables.
    pub fn new() -> Self {
        let mut vars = BTreeMap::new();
        vars.insert("RUSTUP_AUTO_INSTALL".into(), "0".into());
        Self { vars }
    }

    /// Sets a specific Rust toolchain.
    ///
    /// The `toolchain_name` must follow the [toolchain name specification].
    ///
    /// [toolchain name specification]: https://rust-lang.github.io/rustup/concepts/toolchains.html
    pub fn rust(mut self, toolchain_name: Option<String>) -> Self {
        const KEY: &str = "RUSTUP_TOOLCHAIN";
        if let Some(toolchain_name) = toolchain_name {
            self.vars.insert(KEY.into(), toolchain_name);
        } else {
            self.vars.remove(KEY);
        }
        self
    }

    /// Returns the underlying map.
    pub fn vars(&self) -> &BTreeMap<String, String> {
        &self.vars
    }

    /// Apply the environment variables to the command.
    pub fn apply<'a>(&self, mut cmd: &'a mut Command) -> &'a mut Command {
        for (k, v) in &self.vars {
            cmd = cmd.env(k, v);
        }
        cmd
    }
}
