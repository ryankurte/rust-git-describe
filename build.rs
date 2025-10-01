use git_describe::GitVersionCfg;

fn main() -> anyhow::Result<()> {
    // Export the current git describe version string to an environmental variable
    let cfg = GitVersionCfg::default().export_var()?;
    Ok(())
}
