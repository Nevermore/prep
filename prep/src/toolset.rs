// Copyright 2026 the Prep Authors
// SPDX-License-Identifier: Apache-2.0 OR MIT

use std::collections::{BTreeMap, HashMap};
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

use anyhow::{Context, Result, bail};
use semver::{Version, VersionReq};
use serde::{Deserialize, Serialize};
use time::{Date, UtcDateTime};

use crate::tools::Tool;
use crate::ui;

const MANIFEST_NAME: &str = "tools.toml";

/// Collection of tools.
pub struct Toolset {
    tools_dir: PathBuf,
    manifest_path: PathBuf,

    manifest: Manifest,

    /// Per-tool, per-version, binary executable paths.
    ///
    /// All the entries in this registry have been verified to exist and be the correct version.
    /// With that verification having happened during the lifetime of this specific process.
    path_registry: HashMap<String, BTreeMap<Version, PathBuf>>,
}

impl Toolset {
    /// Creates a new toolset.
    pub fn new(tools_dir: PathBuf) -> Result<Self> {
        let manifest_path = tools_dir.join(MANIFEST_NAME);

        // Attempt to load the manifest
        let manifest = if manifest_path.exists() {
            Self::load_manifest(&manifest_path)?
        } else {
            Manifest::new()
        };

        let this = Self {
            tools_dir,
            manifest_path,
            manifest,
            path_registry: HashMap::new(),
        };

        Ok(this)
    }

    /// Returns the verified, tool-specific, version->path map.
    fn tool_paths<T: Tool>(&mut self) -> &mut BTreeMap<Version, PathBuf> {
        self.path_registry.entry(T::NAME.to_string()).or_default()
    }

    /// Get a specific tool that meets the given version requirement
    /// and uses the specified dependencies.
    ///
    /// `None` as the version requirement means that the default version will be used.
    pub fn get<'a, T: Tool>(
        &mut self,
        deps: &T::Deps,
        ver_req: impl Into<Option<&'a VersionReq>>,
    ) -> Result<Command> {
        // Check if we can just return the default version,
        // i.e. just the binary name with no detailed path.
        let Some(ver_req) = ver_req.into() else {
            return Ok(Command::new(T::BIN));
        };

        // A specific version requirement was provided.
        // First check if we have already used this tool during this session.
        let paths = self.tool_paths::<T>();
        // Iterate in reverse because we want to match with the highest possible version.
        for (version, path) in paths.iter().rev() {
            if ver_req.matches(version) {
                return Ok(Command::new(path));
            }
        }

        let today = UtcDateTime::now().date();

        // No satisfactory version in the registry yet.
        // Check if our manifest has any installation info that matches this requirement.
        if let Some((version, path)) = self.manifest.get(T::NAME, ver_req) {
            // Relative paths are relative to the tools directory.
            let path = if path.is_relative() {
                self.tools_dir.join(path)
            } else {
                path
            };
            // Verify that it still exists and is the correct version.
            let exact_ver_req = VersionReq::parse(&format!("={}", version)).context(format!(
                "failed to convert version '{}' to exact version requirement",
                version
            ))?;
            if self.verify::<T>(&path, &exact_ver_req)?.is_some() {
                // Check whether the last use date in the manifest needs updating.
                if self.manifest.mark_used(T::NAME, &version, today) {
                    self.save_manifest()
                        .context("failed to save tool manifest")?;
                }
                return Ok(Command::new(path));
            } else {
                // It no longer exists, remove the manifest entry.
                ui::print_warn(&format!(
                    "{} {} installation at '{}' no longer exists, removing it from the manifest",
                    T::NAME,
                    version,
                    path.display()
                ));
                if self.manifest.remove(T::NAME, &version) {
                    self.save_manifest()
                        .context("failed to save tool manifest")?;
                }
            }
        }

        // No satisfactory version in the manifest either.
        // See if there is a default installation and if it meets the requirements.
        if let Some(version) = self
            .default_version::<T>()
            .context(format!("failed to get the default {} version", T::NAME))?
            && ver_req.matches(&version)
        {
            return Ok(Command::new(T::BIN));
        }

        // No satisfactory available anywhere, need to set it up.
        let (version, path) = T::set_up(self, deps, ver_req)?;

        // Strip the tools directory prefix if it has it.
        let save_path = path.strip_prefix(&self.tools_dir).unwrap_or(&path);

        // Update the manifest if the save path is not literally just the binary name.
        if save_path != T::BIN {
            self.manifest.set(
                T::NAME.to_string(),
                version.clone(),
                save_path.to_path_buf(),
                today,
            );
            self.save_manifest()
                .context("failed to save tool manifest")?;
        }

        // Return the result
        Ok(Command::new(&path))
    }

    /// Verifies that the given `path` is a binary for the given `ver_req` of the tool.
    ///
    /// Returns the specific `Version` of the tool, or `None` if the path doesn't exist.
    /// Errors if the `path` exists but is an unexpected version.
    pub fn verify<T: Tool>(
        &mut self,
        path: &Path,
        ver_req: &VersionReq,
    ) -> Result<Option<Version>> {
        let version = self.version::<T>(path)?;
        let Some(version) = version else {
            return Ok(None);
        };
        if !ver_req.matches(&version) {
            bail!(
                "expected {} to satisfy {ver_req}, got: {version}",
                path.display()
            );
        }
        Ok(Some(version))
    }

    /// Returns the default version information.
    ///
    /// Returns `None` if no default version is found.
    #[inline(always)]
    pub fn default_version<T: Tool>(&mut self) -> Result<Option<Version>> {
        self.version::<T>(&PathBuf::from(T::BIN))
    }

    /// Returns the [`Version`] of the binary at the given `path`.
    ///
    /// Returns `None` if the given `path` doesn't exist.
    pub fn version<T: Tool>(&mut self, path: &Path) -> Result<Option<Version>> {
        // First check if we already know this path's version.
        let paths = self.tool_paths::<T>();
        for (version, ver_path) in paths.iter() {
            if path == ver_path {
                return Ok(Some(version.clone()));
            }
        }

        // Brand new path, so we need to figure out the version.
        let Some(version) =
            T::extract_version(path).context(format!("failed to extract {} version", T::NAME))?
        else {
            return Ok(None);
        };

        // Cache the result in the registry
        paths.insert(version.clone(), path.into());

        Ok(Some(version))
    }

    /// Returns the directory where the tool binary should be installed.
    pub fn install_dir(&self, name: &str, version: &Version) -> PathBuf {
        self.tools_dir.join(name).join(version.to_string())
    }

    /// Returns the temporary directory where the tool binary can be installed,
    /// before being moved to the correct [`install_dir`].
    ///
    /// This temporary directory is expected to not exist and should be cleaned up by the caller.
    /// For safety reasons, when deleting, only delete explicitly those files which are expected.
    /// Then delete the directory when it is empty and error out otherwise.
    pub fn temp_install_dir(&self, name: &str) -> PathBuf {
        self.tools_dir.join(format!("temp-{name}"))
    }

    /// Ensures that the tools directory exists.
    pub fn ensure_tools_dir(&self) -> Result<()> {
        if !self.tools_dir.exists() {
            fs::create_dir_all(&self.tools_dir).context(format!(
                "failed to create tools directory: {}",
                self.tools_dir.display()
            ))?;
        } else if !self.tools_dir.is_dir() {
            bail!(
                "tools directory path taken but not a directory: {}",
                self.tools_dir.display()
            );
        }
        Ok(())
    }

    /// Loads the tool manifest from file.
    pub fn load_manifest(path: &Path) -> Result<Manifest> {
        let manifest_toml = fs::read(path).context(format!(
            "failed to read tool manifest file '{}'",
            path.display()
        ))?;
        let manifest: Manifest =
            toml::from_slice(&manifest_toml).context("failed to parse tool manifest TOML")?;
        Ok(manifest)
    }

    /// Saves the tool manifest to file.
    // TODO: Should check before writing whether it has been modified since we loaded it,
    //       or even use locking. Otherwise parallel Prep usage is broken.
    pub fn save_manifest(&self) -> Result<()> {
        self.ensure_tools_dir()?;
        let manifest_toml =
            toml::to_string(&self.manifest).context("failed to generate tool manifest TOML")?;
        fs::write(&self.manifest_path, &manifest_toml).context(format!(
            "failed to write tool manifest file '{}'",
            self.manifest_path.display()
        ))?;
        Ok(())
    }
}

/// The installed tools manifest.
#[derive(Serialize, Deserialize)]
pub struct Manifest {
    #[serde(default)]
    tools: HashMap<String, BTreeMap<Version, Installation>>,
}

/// Information about a tool installation.
#[derive(Serialize, Deserialize)]
pub struct Installation {
    path: PathBuf,
    used: Date,
}

impl Manifest {
    /// Creates a new tool manifest.
    pub fn new() -> Self {
        Self {
            tools: HashMap::new(),
        }
    }

    /// Returns the installation path and version of the specified tool.
    ///
    /// The returned version is guaranteed to match the specified version requirement.
    ///
    /// Returns `None` if no known installation satisfies the requirement.
    pub fn get(&self, name: &str, ver_req: &VersionReq) -> Option<(Version, PathBuf)> {
        if let Some(tool) = self.tools.get(name) {
            // Iterate in reverse because we want to match with the highest possible version.
            for (version, installation) in tool.iter().rev() {
                if ver_req.matches(version) {
                    return Some((version.clone(), installation.path.clone()));
                }
            }
        }
        None
    }

    /// Sets the given tool's `version` to `path`.
    pub fn set(&mut self, name: String, version: Version, path: PathBuf, today: Date) {
        let tool = self.tools.entry(name).or_default();
        // Remove any other versions that still think this path serves them.
        tool.retain(|_, i| i.path != path);
        // Add the new correct entry.
        tool.insert(version, Installation { path, used: today });
    }

    /// Removes the given tool's `version` from the manifest.
    ///
    /// Returns `true` if anything was changed.
    pub fn remove(&mut self, name: &str, version: &Version) -> bool {
        if let Some(tool) = self.tools.get_mut(name) {
            tool.remove(version).is_some()
        } else {
            false
        }
    }

    /// Sets the last used date of the specified tool version.
    ///
    /// Returns `true` if anything was changed.
    pub fn mark_used(&mut self, name: &str, version: &Version, today: Date) -> bool {
        if let Some(tool) = self.tools.get_mut(name)
            && let Some(installation) = tool.get_mut(version)
            && installation.used < today
        {
            installation.used = today;
            return true;
        }
        false
    }
}
