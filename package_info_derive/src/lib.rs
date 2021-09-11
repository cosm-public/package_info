//! Derive macros for exposing Cargo Package information in Rust apps run as binaries
//!
//! Values are taken from environment variables exported by Cargo
//! @see https://doc.rust-lang.org/cargo/reference/environment-variables.html#environment-variables-cargo-sets-for-crates

extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn;

// Cargo environment variable names
const AUTHORS: &str = "CARGO_PKG_AUTHORS";
const DESCRIPTION: &str = "CARGO_PKG_DESCRIPTION";
const HOMEPAGE: &str = "CARGO_PKG_HOMEPAGE";
const LICENSE: &str = "CARGO_PKG_LICENSE";
const LICENSE_FILE: &str = "CARGO_PKG_LICENSE_FILE";
const NAME: &str = "CARGO_PKG_NAME";
const REPOSITORY: &str = "CARGO_PKG_REPOSITORY";
const VERSION: &str = "CARGO_PKG_VERSION";
const VERSION_MAJOR: &str = "CARGO_PKG_VERSION_MAJOR";
const VERSION_MINOR: &str = "CARGO_PKG_VERSION_MINOR";
const VERSION_PATCH: &str = "CARGO_PKG_VERSION_PATCH";
const VERSION_PRE: &str = "CARGO_PKG_VERSION_PRE";

/// PackageInfo derive macro
#[proc_macro_derive(PackageInfo)]
pub fn package_info_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();

    // Build the trait implementation
    impl_package_info(&ast)
}

/// Builds the PackageInfo trait implementation
fn impl_package_info(ast: &syn::DeriveInput) -> TokenStream {
    // Get the name of the Struct deriving the PackageInfo trait
    let struct_name = &ast.ident;
    // Handle any generics
    let generics = &ast.generics;
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    // Get the package info values from Cargo-generated environment variables
    let authors = get_from_env(AUTHORS);
    let description = get_from_env(DESCRIPTION);
    let homepage = get_from_env(HOMEPAGE);
    let license = get_from_env(LICENSE);
    let license_file = get_from_env(LICENSE_FILE);
    let package_name = get_from_env(NAME);
    let repository = get_from_env(REPOSITORY);
    let version = get_from_env(VERSION);
    let version_major = get_from_env(VERSION_MAJOR);
    let version_minor = get_from_env(VERSION_MINOR);
    let version_patch = get_from_env(VERSION_PATCH);
    let version_pre = get_from_env(VERSION_PRE);

    // Generate the implementation
    let gen = quote! {
        impl #impl_generics package_info::PackageInfo for #struct_name #ty_generics #where_clause {
            fn authors() -> Option<String> {
                if #authors.is_empty() {
                    None
                } else {
                    Some(#authors.to_string())
                }
            }

            fn description() -> Option<String> {
                if #description.is_empty() {
                    None
                } else {
                    Some(#description.to_string())
                }
            }

            fn homepage() -> Option<String> {
                if #homepage.is_empty() {
                    None
                } else {
                    Some(#homepage.to_string())
                }
            }

            fn license() -> Option<String> {
                if #license.is_empty() {
                    None
                } else {
                    Some(#license.to_string())
                }
            }

            fn license_file() -> Option<String> {
                if #license_file.is_empty() {
                    None
                } else {
                    Some(#license_file.to_string())
                }
            }

            fn name() -> Option<String> {
                if #package_name.is_empty() {
                    None
                } else {
                    Some(#package_name.to_string())
                }
            }

            fn repository() -> Option<String> {
                if #repository.is_empty() {
                    None
                } else {
                    Some(#repository.to_string())
                }
            }

            fn version() -> Option<String> {
                if #version.is_empty() {
                    None
                } else {
                    Some(#version.to_string())
                }
            }

            fn version_major() -> Option<String> {
                if #version_major.is_empty() {
                    None
                } else {
                    Some(#version_major.to_string())
                }
            }

            fn version_minor() -> Option<String> {
                if #version_minor.is_empty() {
                    None
                } else {
                    Some(#version_minor.to_string())
                }
            }

            fn version_patch() -> Option<String> {
                if #version_patch.is_empty() {
                    None
                } else {
                    Some(#version_patch.to_string())
                }
            }

            fn version_pre() -> Option<String> {
                if #version_pre.is_empty() {
                    None
                } else {
                    Some(#version_pre.to_string())
                }
            }
        }
    };
    gen.into()
}

/// Get an environment variable or empty String if the variable doesn't exist
fn get_from_env(var_name: &str) -> String {
    std::env::var(var_name).unwrap_or(String::new())
}
