## Catching missing dependency feature requirements

We don't use `--all-targets` because then even `--lib` and `--bins` are compiled with dev dependencies enabled.
That does not match how the package is compiled by users, in which case dev dependencies are not enabled.
Cargo does feature unification, which means that a dev dependency might transitively enable a regular dependency's feature that we need.
Checking with `--all-targets` would not find the missing explicit feature requirement.
This problem still applies to Cargo resolver version 3 and is unlikely to be solved any time soon.
Thus we split all the targets into two steps, one with `--lib --bins` and another with `--tests --benches --examples`.
Also, we can't actually give `--lib --bins` explicitly because then Cargo will error on binary-only packages.
Luckily the default behavior of Cargo with no explicit targets is the same as with `--lib --bins` but without the error.

## Glossary

* **Artifact** - Single output file produced by the Rust toolchain, e.g. a runnable executable or a linkable library.
* **Crate** - Rust compilation unit that will produce one or more artifacts from the same code.
              For example a library crate can result in both a `staticlib` and a `cdylib` artifact.
* **Package** - Collection of crates described by a single `Cargo.toml` file.
                This collection, a package, is what is uploaded to crates.io.
                In casual conversation in the Rust ecosystem, packages are often confusingly called crates.
* **Target (Cargo)** - Describes exactly one crate that will be built from a single root source file.
                       Available targets include `bin`, `lib`, `example`, `test`, and `bench`.
* **Target (triple)** - Describes the target platform, e.g. `x86_64-unknown-linux-gnu`.
