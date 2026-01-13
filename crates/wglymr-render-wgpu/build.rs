use std::path::Path;
use std::process::Command;

fn main() {
    let font_path = "../../fonts/Roboto-Regular.ttf";
    let out_dir = "assets/generated";

    let atlas = format!("{}/roboto.msdf.png", out_dir);
    let metrics = format!("{}/roboto.metrics.json", out_dir);

    if Path::new(&atlas).exists() && Path::new(&metrics).exists() {
        println!("cargo:warning=Using existing MSDF assets (skip generation)");
        println!("cargo:rerun-if-changed={}", font_path);
        return;
    }

    std::fs::create_dir_all(out_dir).unwrap();

    let status = Command::new("msdfgen")
        .args([
            "-font",
            font_path,
            "-type",
            "msdf",
            "-format",
            "png",
            "-imageout",
            &atlas,
            "-json",
            &metrics,
            "-size",
            "256",
            "-pxrange",
            "8",
        ])
        .status();

    match status {
        Ok(exit_status) if exit_status.success() => {
            println!("cargo:warning=MSDF atlas generated successfully");
        }
        Ok(exit_status) => {
            panic!("msdfgen failed with exit code: {:?}", exit_status.code());
        }
        Err(e) => {
            panic!(
                "Failed to run msdfgen (ensure it's installed and on PATH): {}",
                e
            );
        }
    }

    println!("cargo:rerun-if-changed={}", font_path);
}
