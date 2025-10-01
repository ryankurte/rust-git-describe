
fn main() {
    // Fake the version export here for testing / examples
    println!(
        "cargo:rustc-env={}_GIT_VERSION={}",
        env!("CARGO_PKG_NAME"),
        env!("CARGO_PKG_VERSION")
    );
}
