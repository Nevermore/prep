// Copyright 2026 the Prep Authors
// SPDX-License-Identifier: Apache-2.0 OR MIT

use serde::{Deserialize, Serialize};

/// Prep project configuration.
#[derive(Serialize, Deserialize)]
pub struct Config {
    /// The project configuration.
    project: Project,
}

#[derive(Serialize, Deserialize)]
pub struct Project {
    /// The project name.
    name: String,
    /// The project License SPDX identifier.
    license: String,
}

impl Config {
    /// Creates a new [`Config`] with default values.
    pub fn new() -> Self {
        Self {
            project: Project::new(),
        }
    }

    /// Returns the project configuration.
    pub fn project(&self) -> &Project {
        &self.project
    }
}

impl Project {
    /// Creates a new [`Project`] with default values.
    pub fn new() -> Self {
        Self {
            name: "Untitled".into(),
            license: "Apache-2.0 OR MIT".into(),
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
