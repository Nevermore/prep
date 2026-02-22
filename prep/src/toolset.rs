// Copyright 2026 the Prep Authors
// SPDX-License-Identifier: Apache-2.0 OR MIT

use std::collections::{BTreeMap, HashMap};
use std::fs;
use std::path::{Path, PathBuf};

use anyhow::{Context, Result, bail};
use semver::{Version, VersionReq};
use serde::{Deserialize, Serialize};
use time::{Date, UtcDateTime};

use crate::environment::Environment;
use crate::tools::{BinCtx, Tool};
use crate::ui;

const MANIFEST_NAME: &str = "tools.toml";

/// Collection of tools.
pub struct Toolset {
    tools_dir: PathBuf,
    working_dir: PathBuf,
    manifest_path: PathBuf,

    manifest: Manifest,

    environment: Environment,

    /// Binary context mapping to its info.
    ///
    /// All the entries in this map have been verified to exist and be the specified version.
    /// With that verification having happened during the lifetime of this specific process.
    bins: HashMap<BinCtx, BinInfo>,
}

struct BinInfo {
    name: String,
    version: Version,
}

impl Toolset {
    /// Creates a new toolset.
    pub fn new(tools_dir: PathBuf, working_dir: PathBuf, environment: Environment) -> Result<Self> {
        let manifest_path = tools_dir.join(MANIFEST_NAME);

        // Attempt to load the manifest
        let manifest = if manifest_path.exists() {
            Self::load_manifest(&manifest_path)?
        } else {
            Manifest::new()
        };

        let this = Self {
            tools_dir,
            working_dir,
            manifest_path,
            manifest,
            environment,
            bins: HashMap::new(),
        };

        Ok(this)
    }

    /// Returns a reference to the default environment.
    pub fn environment(&self) -> &Environment {
        &self.environment
    }

    /// Returns the default working directory.
    pub fn working_dir(&self) -> &Path {
        &self.working_dir
    }

    /// Returns a new [`BinCtx`] with default working dir and environment variables.
    pub fn binctx(&self, path: PathBuf) -> BinCtx {
        BinCtx::new(path, self.working_dir.clone(), self.environment.clone())
    }

    /// Get a specific tool that meets the given version requirement
    /// and uses the specified dependencies.
    ///
    /// `None` as the version requirement means that the default version will be used.
    pub fn get<'a, T: Tool>(
        &mut self,
        deps: &T::Deps,
        ver_req: impl Into<Option<&'a VersionReq>>,
    ) -> Result<BinCtx> {
        // Check if we can just return the default version,
        // i.e. just the binary name with no detailed path.
        let Some(ver_req) = ver_req.into() else {
            return T::default_binctx(self, deps);
        };

        // A specific version requirement was provided.
        // First check if we have already used a version of this tool
        // during this session which meets the requirements.
        if let Some(binctx) = self
            .bins
            .iter()
            .find(|(_, info)| info.name == T::NAME && ver_req.matches(&info.version))
            .map(|(binctx, _)| binctx)
        {
            return Ok(binctx.clone());
        }

        let today = UtcDateTime::now().date();

        // No immediately usable version satisfies the requirements.
        // Check if our manifest has any installation info that matches this requirement.
        if T::MANAGED
            && let Some((version, path)) = self.manifest.get(T::NAME, ver_req)
        {
            // Relative paths are relative to the tools directory.
            let path = if path.is_relative() {
                self.tools_dir.join(path)
            } else {
                path
            };
            let binctx = self.binctx(path);
            // Verify that it still exists and is the correct version.
            let exact_ver_req = VersionReq::parse(&format!("={}", version)).context(format!(
                "failed to convert version '{}' to exact version requirement",
                version
            ))?;
            if self.verify::<T>(&binctx, &exact_ver_req)?.is_some() {
                // Check whether the last use date in the manifest needs updating.
                if self.manifest.mark_used(T::NAME, &version, today) {
                    self.save_manifest()
                        .context("failed to save tool manifest")?;
                }
                return Ok(binctx);
            } else {
                // It no longer exists, remove the manifest entry.
                ui::print_warn(&format!(
                    "{} {} installation at '{}' no longer exists, removing it from the manifest",
                    T::NAME,
                    version,
                    binctx.path().display()
                ));
                if self.manifest.remove(T::NAME, &version) {
                    self.save_manifest()
                        .context("failed to save tool manifest")?;
                }
            }
        }

        // No satisfactory version in the manifest either.
        // See if there is a default installation and if it meets the requirements.
        let binctx = T::default_binctx(self, deps)?;
        if let Some(version) = self
            .version::<T>(&binctx)
            .context(format!("failed to get the default {} version", T::NAME))?
            && ver_req.matches(&version)
        {
            return Ok(binctx);
        }

        // No satisfactory available anywhere, need to set it up.
        let (binctx, version) = T::set_up(self, deps, ver_req)?;

        // Update the manifest if this tool is supposed to be managed by the toolset.
        if T::MANAGED {
            // Strip the tools directory prefix if it has it.
            let save_path = binctx
                .path()
                .strip_prefix(&self.tools_dir)
                .unwrap_or(binctx.path());

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
        Ok(binctx)
    }

    /// Verifies that the given `path` is a binary for the given `ver_req` of the tool.
    ///
    /// Returns the specific `Version` of the tool, or `None` if the path doesn't exist.
    /// Errors if the `path` exists but is an unexpected version.
    pub fn verify<T: Tool>(
        &mut self,
        binctx: &BinCtx,
        ver_req: &VersionReq,
    ) -> Result<Option<Version>> {
        let version = self.version::<T>(binctx)?;
        let Some(version) = version else {
            return Ok(None);
        };
        if !ver_req.matches(&version) {
            bail!(
                "expected {} at '{}' to satisfy {ver_req}, got: {version}",
                T::NAME,
                binctx.path().display()
            );
        }
        Ok(Some(version))
    }

    /// Returns the [`Version`] of the binary context.
    ///
    /// Returns `None` if the given binary context's path doesn't exist.
    pub fn version<T: Tool>(&mut self, binctx: &BinCtx) -> Result<Option<Version>> {
        // First check if we already know this binary context's version.
        if let Some(info) = self.bins.get(binctx)
            && info.name == T::NAME
        {
            return Ok(Some(info.version.clone()));
        }

        // New binary context, or at least for this tool, so we need to figure out the version.
        let Some(version) =
            T::extract_version(binctx).context(format!("failed to extract {} version", T::NAME))?
        else {
            return Ok(None);
        };

        // Cache the result in the registry
        self.bins.insert(
            binctx.clone(),
            BinInfo {
                name: T::NAME.into(),
                version: version.clone(),
            },
        );

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
