fn main() {
    // This build script is reserved for future font asset generation
    // Currently, generated assets are checked into the repo
    // Run `cargo run -p xtask -- fonts` to regenerate fonts

    println!("cargo:rerun-if-changed=assets/fonts/");
    println!("cargo:rerun-if-changed=assets/generated/");
}
