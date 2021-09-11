//! Package Info Traits

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

#[cfg(test)]
mod tests {
    use super::PackageInfo;
    use package_info_derive::PackageInfo;

    /// Struct for testing derived functions
    #[derive(PackageInfo)]
    struct TestStruct {}

    #[test]
    fn it_returns_populated_info_field() {
        assert_eq!(TestStruct::name(), Some(String::from("package_info")));
    }

    #[test]
    fn is_none_for_unpopulated_info_field() {
        assert_eq!(TestStruct::homepage(), None);
    }
}
