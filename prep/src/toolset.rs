// Copyright 2026 the Prep Authors
// SPDX-License-Identifier: Apache-2.0 OR MIT

use std::any::TypeId;
use std::collections::{BTreeMap, HashMap};
use std::process::Command;

use anyhow::{Context, Result, bail};
use semver::{Version, VersionReq};

use crate::tools::Tool;

/// Collection of tools.
pub struct Toolset {
    /// Per-tool, per-version, binary executable paths.
    path_registry: HashMap<TypeId, BTreeMap<Version, String>>,
}

impl Toolset {
    /// Creates a new toolset.
    pub fn new() -> Self {
        Self {
            path_registry: HashMap::new(),
        }
    }

    /// Returns the tool-specific version->path map.
    fn tool_paths<T: 'static>(&mut self) -> &mut BTreeMap<Version, String> {
        let type_id = TypeId::of::<T>();
        self.path_registry.entry(type_id).or_default()
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
        // A specific version requirement was provided,
        // so we need to see if we can already fulfill it.
        let paths = self.tool_paths::<T>();
        // Iterate in reverse because we want to match with the highest possible version.
        for (version, path) in paths.iter().rev() {
            if ver_req.matches(version) {
                return Ok(Command::new(path));
            }
        }
        // No satisfactory version in the registry yet, so set it up.
        let (version, path) = T::set_up(self, deps, ver_req)?;
        // Prep the command before we move out 'path'.
        let cmd = Command::new(&path);
        // Save the setup path, which will probably overwrite the same path already saved by setup,
        // via Self::version(). That should be fine and idempotent and ensures cache when a setup
        // implementation doesn't call Self::version().
        let paths = self.tool_paths::<T>();
        paths.insert(version, path);
        // Return the result
        Ok(cmd)
    }

    /// Verifies that the given `path` is a binary for the given `ver_req` of the tool.
    ///
    /// Returns the specific `Version` of the tool, or `None` if the path doesn't exist.
    /// Errors if the `path` exists but is an unexpected version.
    pub fn verify<T: Tool>(&mut self, path: &str, ver_req: &VersionReq) -> Result<Option<Version>> {
        let version = self.version::<T>(path)?;
        let Some(version) = version else {
            return Ok(None);
        };
        if !ver_req.matches(&version) {
            bail!("expected {path} to satisfy {ver_req}, got: {version}");
        }
        Ok(Some(version))
    }

    /// Returns the default version information.
    ///
    /// Returns `None` if no default version is found.
    #[inline(always)]
    pub fn default_version<T: Tool>(&mut self) -> Result<Option<Version>> {
        self.version::<T>(T::BIN)
    }

    /// Returns the [`Version`] of the binary at the given `path`.
    ///
    /// Returns `None` if the given `path` doesn't exist.
    pub fn version<T: Tool>(&mut self, path: &str) -> Result<Option<Version>> {
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
}
