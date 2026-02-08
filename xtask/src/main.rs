use anyhow::{Context, Result};
use std::env;
use std::process::{Command, exit};

fn main() {
    if let Err(e) = try_main() {
        eprintln!("Error: {:#}", e);
        exit(1);
    }
}

fn try_main() -> Result<()> {
    let task = env::args().nth(1);

    match task.as_deref() {
        Some("check") => check(),
        Some("test") => test(),
        Some("clippy") => clippy(),
        Some("fmt") => fmt(),
        Some("web") => web(),
        Some("fonts") => fonts(),
        _ => {
            print_help();
            Ok(())
        }
    }
}

fn print_help() {
    eprintln!(
        "\
WGLYMR Build Automation (xtask)

USAGE:
    cargo run -p xtask -- <COMMAND>

COMMANDS:
    check      Check all workspace crates
    test       Run all tests
    clippy     Run clippy lints
    fmt        Format all code
    web        Build web frontend with wasm-pack
    fonts      Generate font atlases (MSDF and bitmap)
"
    );
}

fn check() -> Result<()> {
    println!("Running cargo check on workspace...");

    let status = Command::new("cargo")
        .args(["check", "--workspace"])
        .status()
        .context("Failed to execute cargo check")?;

    if !status.success() {
        anyhow::bail!("cargo check failed");
    }

    println!("✓ Check completed successfully");
    Ok(())
}

fn test() -> Result<()> {
    println!("Running cargo test on workspace...");

    let status = Command::new("cargo")
        .args(["test", "--workspace"])
        .status()
        .context("Failed to execute cargo test")?;

    if !status.success() {
        anyhow::bail!("cargo test failed");
    }

    println!("✓ Tests completed successfully");
    Ok(())
}

fn clippy() -> Result<()> {
    println!("Running cargo clippy on workspace...");

    let status = Command::new("cargo")
        .args([
            "clippy",
            "--workspace",
            "--all-targets",
            "--",
            "-D",
            "warnings",
        ])
        .status()
        .context("Failed to execute cargo clippy")?;

    if !status.success() {
        anyhow::bail!("cargo clippy failed");
    }

    println!("✓ Clippy completed successfully");
    Ok(())
}

fn fmt() -> Result<()> {
    println!("Running cargo fmt on workspace...");

    let status = Command::new("cargo")
        .args(["fmt", "--all", "--check"])
        .status()
        .context("Failed to execute cargo fmt")?;

    if !status.success() {
        anyhow::bail!("cargo fmt found formatting issues - run 'cargo fmt --all' to fix");
    }

    println!("✓ Format check completed successfully");
    Ok(())
}

fn web() -> Result<()> {
    println!("Building web frontend with wasm-pack...");

    let status = Command::new("wasm-pack")
        .args([
            "build",
            "crates/wglymr-frontend-web",
            "--target",
            "web",
            "--out-dir",
            "pkg",
        ])
        .status()
        .context("Failed to execute wasm-pack - ensure wasm-pack is installed")?;

    if !status.success() {
        anyhow::bail!("wasm-pack build failed");
    }

    println!("✓ Web frontend built successfully");
    println!("  Output: crates/wglymr-frontend-web/pkg/");
    Ok(())
}

fn fonts() -> Result<()> {
    println!("Generating font atlases...");
    println!("Note: Font generation requires external tools (msdfgen) to be available");
    println!("This command regenerates MSDF and bitmap atlases in wglymr-font-assets");

    println!("\n⚠️  Font generation not yet fully implemented");
    println!("Generated assets should be checked into the repository once created");

    Ok(())
}
