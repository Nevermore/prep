// Copyright 2026 the Prep Authors
// SPDX-License-Identifier: Apache-2.0 OR MIT

/// The triple that Prep was compiled for and thus is running on.
#[expect(dead_code, reason = "for the future")]
pub const TRIPLE: &str = env!("PREP_HOST_TRIPLE");

/// Returns the executable file name.
///
/// Appends `.exe` on Windows, does nothing on other platforms.
pub fn executable_name(name: &str) -> String {
    //#[cfg(target_os = "windows")]
    if cfg!(target_os = "windows") {
        format!("{name}.exe")
    } else {
        name.to_string()
    }
}
