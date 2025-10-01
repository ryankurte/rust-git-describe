# git-describe

A library to fetch and export git version information during the build process

## Usage

Load and export the variable from build.rs:
```rust
// Export the current git describe version string to an environmental variable
git_describe::export_version(".");
```

Use the variable elsewhere within the package:
```rust
// Fetch the version from the environmental variable
let version = git_describe::get_version();
```
