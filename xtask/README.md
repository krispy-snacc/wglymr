# xtask

Build automation for WGLYMR.

## Purpose

Per BUILD.md, `xtask` is the **single automation entry point** for the workspace.

All build automation:

- Lives in this crate
- Is written in Rust (no shell scripts or Makefiles)
- Uses standard Cargo commands under the hood

## Available Commands

```bash
# Check all workspace crates
cargo run -p xtask -- check

# Run all tests
cargo run -p xtask -- test

# Run clippy lints
cargo run -p xtask -- clippy

# Check code formatting
cargo run -p xtask -- fmt

# Build web frontend with wasm-pack
cargo run -p xtask -- web

# Generate font atlases (MSDF and bitmap)
cargo run -p xtask -- fonts
```

## Architecture Philosophy

From BUILD.md:

- **No shell scripts** - All automation is Rust code
- **No Makefiles** - Cargo is the only build orchestrator
- **Explicit invocation** - Asset generation is not implicit
- **CI alignment** - CI uses the same xtask commands

## Usage in CI

CI should invoke xtask exclusively:

```yaml
- run: cargo run -p xtask -- check
- run: cargo run -p xtask -- test
- run: cargo run -p xtask -- clippy
```

## Adding New Tasks

To add a new automation task:

1. Add a new function in `main.rs`
2. Add the command to the match statement
3. Add it to `print_help()`
4. Document it in this README

Changes affecting build behavior require an ADR per BUILD.md section 12.
