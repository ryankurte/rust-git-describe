#![doc = include_str!("../README.md")]

#[cfg(feature = "build")]
use git2::{DescribeFormatOptions, DescribeOptions, Repository};

/// Retrieves the current git version description from the repository
/// located at `repo_path` using `git describe --tags --dirty=+` and
/// exports this to a `PACKAGE_NAME_GIT_VERSION` environmental variable
/// for use in the build process.
/// This function should be called from build.rs.
#[cfg(feature = "build")]
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
pub const fn get_version() -> &'static str {
    env!(concat!(env!("CARGO_PKG_NAME"), "_GIT_VERSION"))
}

/// Retrieves the current git version description from the repository
/// located at `repo_path` using `git describe --tags --dirty=+`
#[cfg(feature = "build")]
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
    #[cfg(feature = "build")]
    fn get_version() {
        build_get_version(&"./").unwrap();
    }

    #[test]
    #[cfg(feature = "build")]
    fn export_var() {
        export_version(&"./").unwrap();
    }
}
