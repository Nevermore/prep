// Copyright 2026 the Prep Authors
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! Prep's build script.

use std::env;

fn main() {
    let target = env::var("TARGET").expect("failed to read TARGET environment variable");
    println!("cargo:rustc-env=PREP_HOST_TRIPLE={target}");
}
