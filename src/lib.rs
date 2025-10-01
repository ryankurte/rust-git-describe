#![doc = include_str!("../README.md")]

use anyhow::Ok;
use git2::{DescribeFormatOptions, DescribeOptions, Repository};

/// GitVersion configuration structure
///
/// This should usually be instantiated with `GitVersionCfg::default()` which
/// will automatically load the current package name and set the repository path
/// to the current directory. Both fields can be overridden if necessary.
#[derive(Clone, Debug, PartialEq)]
pub struct GitVersionCfg {
    /// The name of the environment variable to set
    pub var_name: String,
    /// The path to the git repository
    pub repo_path: String,
}

impl Default for GitVersionCfg {
    fn default() -> Self {
        Self {
            var_name: env!("CARGO_PKG_NAME").to_uppercase() + "_GIT_VERSION",
            repo_path: "./".to_string(),
        }
    }
}

/// Retrieves the current git version description from the repository
/// located at `repo_path` using `git describe --tags --dirty=+` and
/// exports this to a `PACKAGE_NAME_GIT_VERSION` environmental variable
/// for use in the build process.
/// This function should be called from build.rs.
pub fn export_version(repo_path: &str) -> anyhow::Result<()> {
    // Fetch version string
    let git_desc = build_get_version(repo_path)?;
    // Export to environment variable
    println!(
        "cargo:rustc-env={}_GIT_VERSION={}",
        env!("CARGO_PKG_NAME"),
        git_desc
    );
    Ok(())
}

/// Fetch the git version string from the environment variable set at build time
/// by `GitVersionCfg::export_var()`. This will be in the format produced by
/// `git describe --tags --dirty=+`.
pub fn get_version() -> &'static str {
    env!(concat!(env!("CARGO_PKG_NAME"), "_GIT_VERSION"))
}

/// Retrieves the current git version description from the repository
/// located at `repo_path` using `git describe --tags --dirty=+`
fn build_get_version(repo_path: &str) -> anyhow::Result<String> {
    // Connect to the repository
    let repo = Repository::open(repo_path)?;

    // Build the git description
    let git_desc = repo
        .describe(DescribeOptions::default().describe_tags())
        .map(|desc| desc.format(Some(DescribeFormatOptions::default().dirty_suffix("+"))))
        .flatten()
        .map_err(|e| anyhow::anyhow!("Failed to describe git repository: {}", e))?;

    Ok(git_desc)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_version() {
        build_get_version(&"./").unwrap();
    }

    #[test]
    fn export_var() {
        export_version(&"./").unwrap();
    }
}
