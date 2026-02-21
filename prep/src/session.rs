// Copyright 2026 the Prep Authors
// SPDX-License-Identifier: Apache-2.0 OR MIT

#![expect(unused, reason = "for the future")]

use std::path::{Path, PathBuf};
use std::{env, fs};

use anyhow::{Context, Result, bail};
use cargo_metadata::MetadataCommand;
use directories::ProjectDirs;

use crate::config::Config;
use crate::tools::Tool;
use crate::tools::cargo::{Cargo, CargoDeps};
use crate::toolset::Toolset;

const ORG_TLD: &str = "ee";
const ORG_NAME: &str = "Nevermore";
const APP_NAME: &str = "Prep";

const PREP_DIR: &str = ".prep";
const CONFIG_FILE: &str = "prep.toml";

/// Information about the current runtime session.
pub struct Session {
    /// The project root directory.
    root_dir: PathBuf,
    /// The project's prep directory.
    prep_dir: PathBuf,
    /// The project's prep config path.
    config_path: PathBuf,
    /// OS specific directories.
    project_dirs: ProjectDirs,

    /// Active configuration.
    config: Config,
    /// Toolset.
    toolset: Toolset,
}

impl Session {
    /// Initializes and returns a fresh [`Session`].
    ///
    /// This function will also Load the configuration file.
    pub fn initialize() -> Result<Session> {
        // Attempt to find an existing config file
        let current_dir = env::current_dir().context("failed to get current directory")?;
        let current_dir = current_dir
            .canonicalize()
            .context("failed to canonicalize current directory")?;
        let root_dir =
            find_root_dir(&current_dir).context("failed to look for Prep config file")?;

        let project_dirs = ProjectDirs::from(ORG_TLD, ORG_NAME, APP_NAME)
            .context("failed to get OS specific directories")?;
        let tools_dir = project_dirs.data_local_dir().to_path_buf();

        let mut toolset = Toolset::new(tools_dir).context("failed to initialize toolset")?;

        // Fall back to the Cargo workspace root
        let root_dir = match root_dir {
            Some(root_dir) => root_dir,
            None => {
                let cargo_deps = CargoDeps::new(None);
                let cmd = toolset.get::<Cargo>(&cargo_deps, None)?;
                let metadata = MetadataCommand::new()
                    .cargo_path(cmd.get_program())
                    .exec()
                    .context("failed to fetch Cargo metadata")?;
                let workspace_dir = metadata.workspace_root.into_std_path_buf();
                workspace_dir
                    .canonicalize()
                    .context("failed to canonicalize Cargo workspace dir")?
            }
        };

        let prep_dir = root_dir.join(PREP_DIR);
        let config_path = prep_dir.join(CONFIG_FILE);

        // Attempt to load the config
        let config = if config_path.exists() {
            Self::load_config(&config_path)?
        } else {
            Config::new()
        };

        let session = Session {
            root_dir,
            prep_dir,
            config_path,
            project_dirs,
            config,
            toolset,
        };

        Ok(session)
    }

    /// Returns the project root directory.
    pub fn root_dir(&self) -> &Path {
        &self.root_dir
    }

    /// Returns the project's prep directory.
    pub fn prep_dir(&self) -> &Path {
        &self.prep_dir
    }

    /// Returns the project's prep config path.
    pub fn config_path(&self) -> &Path {
        &self.config_path
    }

    /// Returns the OS-specific directories.
    pub fn project_dirs(&self) -> &ProjectDirs {
        &self.project_dirs
    }

    /// Returns the project's prep config.
    pub fn config(&self) -> &Config {
        &self.config
    }

    /// Returns this session's toolset.
    pub fn toolset(&mut self) -> &mut Toolset {
        &mut self.toolset
    }

    /// Ensures that the prep directory exists.
    pub fn ensure_prep_dir(&self) -> Result<()> {
        if !self.prep_dir.exists() {
            fs::create_dir(&self.prep_dir).context(format!(
                "failed to create Prep directory: {}",
                self.prep_dir.display()
            ))?;
        } else if !self.prep_dir.is_dir() {
            bail!(
                "Prep directory path taken but not a directory: {}",
                self.prep_dir.display()
            );
        }
        Ok(())
    }

    /// Loads the configuration from file.
    pub fn load_config(config_path: &Path) -> anyhow::Result<Config> {
        let config_toml = fs::read(config_path).context(format!(
            "failed to read config file '{}'",
            config_path.display()
        ))?;
        let config: Config =
            toml::from_slice(&config_toml).context("failed to parse config TOML")?;
        Ok(config)
    }

    /// Saves the configuration to file.
    pub fn save_config(&self) -> anyhow::Result<()> {
        self.ensure_prep_dir()?;
        let config_toml =
            toml::to_string(&self.config).context("failed to generate config TOML")?;
        fs::write(&self.config_path, &config_toml).context(format!(
            "failed to write config file '{}'",
            self.config_path.display()
        ))?;
        Ok(())
    }
}

/// Returns the root directory that contains the prep directory with a config file.
fn find_root_dir(dir: &Path) -> anyhow::Result<Option<PathBuf>> {
    let p = dir.join(PREP_DIR).join(CONFIG_FILE);
    if p.is_file() {
        return Ok(Some(dir.to_path_buf()));
    }
    if let Some(parent) = dir.parent() {
        return find_root_dir(parent);
    }
    Ok(None)
}
