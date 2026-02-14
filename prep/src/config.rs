// Copyright 2026 the Prep Authors
// SPDX-License-Identifier: Apache-2.0 OR MIT

use semver::VersionReq;
use serde::{Deserialize, Serialize};

/// Prep configuration.
#[derive(Serialize, Deserialize)]
pub struct Config {
    /// Project configuration.
    #[serde(default = "Project::new")]
    project: Project,
    /// Tools configuration.
    #[serde(default = "Tools::new")]
    tools: Tools,
}

/// Project configuration.
#[derive(Serialize, Deserialize)]
pub struct Project {
    /// Project name.
    #[serde(default = "name_default")]
    name: String,
    /// Project License SPDX identifier.
    #[serde(default = "license_default")]
    license: String,
}

/// Tools configuration.
#[derive(Serialize, Deserialize)]
pub struct Tools {
    /// Rustup configuration.
    #[serde(default = "rustup_default")]
    rustup: VersionReq,
    /// Stable Rust toolchain configuration.
    #[serde(default = "rust_default")]
    rust: VersionReq,
    /// Ripgrep configuration.
    #[serde(default = "ripgrep_default")]
    ripgrep: VersionReq,
}

impl Config {
    /// Creates a new [`Config`] with default values.
    pub fn new() -> Self {
        Self {
            project: Project::new(),
            tools: Tools::new(),
        }
    }

    /// Returns the project configuration.
    pub fn project(&self) -> &Project {
        &self.project
    }

    /// Returns the tools configuration.
    pub fn tools(&self) -> &Tools {
        &self.tools
    }
}

impl Project {
    /// Creates a new [`Project`] with default values.
    pub fn new() -> Self {
        Self {
            name: name_default(),
            license: license_default(),
        }
    }

    /// Returns the project name.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns the project License.
    pub fn license(&self) -> &str {
        &self.license
    }
}

impl Tools {
    /// Creates a new [`Tools`] with default values.
    pub fn new() -> Self {
        Self {
            rustup: rustup_default(),
            rust: rust_default(),
            ripgrep: ripgrep_default(),
        }
    }

    /// Returns the configured Rustup version.
    pub fn rustup(&self) -> &VersionReq {
        &self.rustup
    }

    /// Returns the configured stable Rust toolchain version.
    pub fn rust(&self) -> &VersionReq {
        &self.rust
    }

    /// Returns the configured ripgrep version.
    pub fn ripgrep(&self) -> &VersionReq {
        &self.ripgrep
    }
}

/// Returns the default project name.
fn name_default() -> String {
    "Untitled".into()
}

/// Returns the default project license.
fn license_default() -> String {
    "Apache-2.0 OR MIT".into()
}

/// Returns the default Rustup version.
fn rustup_default() -> VersionReq {
    VersionReq::parse("=1").expect("default rustup version parsing failed")
}

/// Returns the default Rust version.
fn rust_default() -> VersionReq {
    VersionReq::parse("=1.93").expect("default rust version parsing failed")
}

/// Returns the default Ripgrep version.
fn ripgrep_default() -> VersionReq {
    VersionReq::parse("=14.1.1").expect("default ripgrep version parsing failed")
}
