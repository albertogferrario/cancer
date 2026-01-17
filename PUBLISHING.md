# Crates.io Publishing Checklist

Publishing checklist for ferro framework crates.

## Prerequisites

- [ ] `cargo login` with crates.io token
- [ ] Verify crates.io account has publish permissions
- [ ] All CI tests passing on master branch

## Pre-publish Verification

Run these checks before publishing any crate:

```bash
# Verify workspace builds
cargo build --workspace

# Run tests
cargo test --workspace

# Check for publishing issues
cargo publish --dry-run -p <crate-name>
```

## Publishing Order

Crates must be published in dependency order. Wait for each wave to be indexed on crates.io before proceeding to the next.

### Wave 1: Independent Crates (no internal dependencies)

These crates have no dependencies on other ferro crates and can be published in parallel:

```bash
cargo publish -p ferro-macros
cargo publish -p ferro-events
cargo publish -p ferro-queue
cargo publish -p ferro-notifications
cargo publish -p ferro-broadcast
cargo publish -p ferro-storage
cargo publish -p ferro-cache
cargo publish -p ferro-inertia
cargo publish -p ferro-mcp
```

**Wait 5-10 minutes for crates.io to index these crates before proceeding.**

### Wave 2: Main Framework

Depends on all Wave 1 crates:

```bash
cargo publish -p ferro
```

**Wait for crates.io to index ferro before proceeding.**

### Wave 3: CLI

Depends on ferro-mcp:

```bash
cargo publish -p ferro-cli
```

## Path Dependency Handling

Before publishing, path dependencies must be replaced with version-only dependencies. The current Cargo.toml files use both path and version:

```toml
# Current (works for publishing)
ferro-macros = { path = "../ferro-macros", version = "0.1" }
```

This format allows `cargo publish` to automatically use the version when uploading to crates.io.

If you see errors about path dependencies:
1. Comment out the `path = "..."` portion
2. Publish the crate
3. Restore the path for local development

## Post-publish Verification

After publishing each crate:

```bash
# Verify crate is available
cargo search <crate-name>

# Test installation from crates.io
cargo add <crate-name>
```

## Version Coordination

All ferro crates should maintain synchronized versions:

1. Update all Cargo.toml versions simultaneously
2. Commit version bump as single atomic change
3. Tag release: `git tag v<version>`
4. Publish in order above

## Troubleshooting

### "crate not found" errors during publish

The dependent crate isn't indexed yet. Wait 5-10 minutes and retry.

### "already uploaded" errors

The crate version already exists on crates.io. Bump the version number and try again.

### Path dependency rejection

Comment out `path = "..."` from the dependency declaration, publish, then restore.

## Crate Summary

| Crate | Package Name | Wave |
|-------|--------------|------|
| ferro-macros | ferro-macros | 1 |
| ferro-events | ferro-events | 1 |
| ferro-queue | ferro-queue | 1 |
| ferro-notifications | ferro-notifications | 1 |
| ferro-broadcast | ferro-broadcast | 1 |
| ferro-storage | ferro-storage | 1 |
| ferro-cache | ferro-cache | 1 |
| ferro-inertia | ferro-inertia | 1 |
| ferro-mcp | ferro-mcp | 1 |
| framework | ferro | 2 |
| ferro-cli | ferro-cli | 3 |
