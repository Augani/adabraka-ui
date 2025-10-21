# Publishing Checklist for adabraka-ui

## Before Publishing

### 1. Prerequisites
- [ ] Have a crates.io account
- [ ] Login to cargo: `cargo login <your-api-token>`
  - Get token from: https://crates.io/me

### 2. Verify Package Configuration
- [x] Package metadata in Cargo.toml is complete
  - [x] name, version, edition
  - [x] authors, license
  - [x] description (under 120 chars)
  - [x] repository, homepage, documentation
  - [x] keywords (max 5)
  - [x] categories
  - [x] readme
- [x] LICENSE file exists
- [x] README.md is comprehensive

### 3. Code Quality Checks
- [ ] `cargo check` passes without errors
- [ ] `cargo clippy` passes without warnings
- [ ] `cargo fmt --check` shows code is formatted
- [ ] All examples compile: `cargo build --examples`
- [ ] Documentation builds: `cargo doc --no-deps`

### 4. Package Verification
- [ ] Dry run package: `cargo package --allow-dirty`
- [ ] Check package contents: `cargo package --list`
- [ ] Verify package size is reasonable (< 10MB ideally)
- [ ] Test the packaged crate: `cargo package && cargo publish --dry-run`

### 5. Documentation
- [ ] Update CHANGELOG.md with version changes
- [ ] Ensure all public APIs have documentation
- [ ] Check documentation examples work
- [ ] Update version in README if mentioned

### 6. Git Repository
- [ ] All changes committed
- [ ] Create git tag for version: `git tag v0.1.0`
- [ ] Push tag to GitHub: `git push origin v0.1.0`

### 7. Final Checks
- [ ] Version number is correct in Cargo.toml
- [ ] No TODO or FIXME comments in public APIs
- [ ] Examples directory is ready for users
- [ ] CONTRIBUTING.md is up to date

## Publishing

### Commands to Run (in order)

```bash
# 1. Final check
cargo check

# 2. Format code
cargo fmt

# 3. Run clippy
cargo clippy

# 4. Build documentation
cargo doc --no-deps --open

# 5. Verify package
cargo package --allow-dirty

# 6. Test dry run
cargo publish --dry-run

# 7. Actually publish (no turning back!)
cargo publish
```

### After Publishing

- [ ] Verify on crates.io: https://crates.io/crates/adabraka-ui
- [ ] Check docs.rs builds: https://docs.rs/adabraka-ui
- [ ] Test installation: `cargo add adabraka-ui` in a test project
- [ ] Announce on:
  - [ ] GitHub Discussions
  - [ ] Reddit r/rust
  - [ ] Twitter/X
  - [ ] This Week in Rust (submit PR)

### Troubleshooting

#### Build Fails
- Check Rust version: `rustc --version`
- Ensure dependencies are up to date: `cargo update`
- Check for platform-specific issues

#### Package Too Large
- Add exclusions to Cargo.toml:
  ```toml
  [package]
  exclude = [
    "docs/",
    ".github/",
    "*.png",
    "*.jpg"
  ]
  ```

#### GPUI Compilation Issues
- Current issue: GPUI 0.2.0 requires nightly features
- Options:
  1. Wait for GPUI fix/update
  2. Use nightly Rust: `rustup default nightly`
  3. Document nightly requirement in README

## Current Status

⚠️ **Cannot publish yet** - GPUI 0.2.0 dependency has compilation issues on stable Rust.

### To Fix:
1. Wait for GPUI team to release updated version, OR
2. Switch to nightly Rust and document the requirement

### When Build Works:
1. Run all checks above
2. Use `cargo publish` to publish
3. Monitor docs.rs for documentation build
4. Test installation in a fresh project

## Version Management

### Semantic Versioning
- **0.1.0** - Initial release
- **0.1.x** - Bug fixes
- **0.x.0** - New features (pre-1.0)
- **1.0.0** - Stable API

### Releasing New Versions
1. Update version in Cargo.toml
2. Update CHANGELOG.md
3. Commit changes
4. Create git tag: `git tag vX.Y.Z`
5. Push: `git push origin vX.Y.Z`
6. Publish: `cargo publish`

## Notes

- First publish cannot be undone (version is permanent)
- Can yank versions if critically broken: `cargo yank --vers 0.1.0`
- Docs build automatically on docs.rs
- Allow ~10 minutes for docs.rs build after publishing
