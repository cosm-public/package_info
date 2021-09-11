# Package Info

Package Info offers access to Cargo manifest information from within your Rust code.
Actually, Cargo offers that by exposing it via [environment variables](https://doc.rust-lang.org/cargo/reference/environment-variables.html#environment-variables-cargo-sets-for-crates).
However, if you are running your application from binary (as opposed to via `cargo run`),
those environment variables are no longer present.

Package Info works around this issue by capturing the environment variable values during
compile and embedding them in your code via the derived trait `PackageInfo`.  You must add *both*
`package_info` and `package_info_derive` as dependencies to your project.

## Usage

```rust
use package_info::PackageInfo;        // the trait
use package_info_derive::PackageInfo; // the derive macro

#[derive(PackageInfo)]
struct CargoPackageInfo {}

fn main() {
    println!("{}", CargoPackageInfo::name().unwrap());
}
```

The `PackageInfo` trait provides the following functions, all of which return a `Option<String>`:
```rust
/// Accessor functions to common Cargo package information as specified in Cargo.toml
pub trait PackageInfo {
    /// Colon separated list of authors from the manifest of the package
    fn authors() -> Option<String>;
    /// The description from the manifest of the package
    fn description() -> Option<String>;
    /// The home page from the manifest of the package
    fn homepage() -> Option<String>;
    /// The license from the manifest of the package
    fn license() -> Option<String>;
    /// The license file from the manifest of the package
    fn license_file() -> Option<String>;
    /// The name of the package
    fn name() -> Option<String>;
    /// The repository from the manifest of the package
    fn repository() -> Option<String>;
    /// The full version of the package
    fn version() -> Option<String>;
    /// The major version of the package
    fn version_major() -> Option<String>;
    /// The minor version of the package
    fn version_minor() -> Option<String>;
    /// The patch version of the package
    fn version_patch() -> Option<String>;
    /// The pre-release version of the package
    fn version_pre() -> Option<String>;
}
```
