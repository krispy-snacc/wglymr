use wglymr_render_wgpu::msdf::{FontFace, MSDFAtlas};
use wgpu::{Device, Queue};

/// Loads the Roboto MSDF atlas and creates a FontFace
///
/// MSDF atlas must be pre-generated offline using msdf-atlas-gen:
/// ```bash
/// msdf-atlas-gen -font roboto.ttf -type msdf -format png -imageout roboto_msdf.png -json roboto_msdf.json \
///   -charset charset.txt -size 32 -pxrange 4
/// ```
///
/// charset.txt should contain all required characters:
/// - ASCII printable (0x20-0x7E)
/// - Common Unicode ranges (Latin Extended, etc.)
///
/// Place generated files in: crates/wglymr-node-editor/fonts/
/// - roboto_msdf.png
/// - roboto_msdf.json

pub fn load_roboto_msdf(device: &Device, queue: &Queue) -> Result<FontFace, String> {
    static PNG: &[u8] = include_bytes!("../../../fonts/roboto_msdf.png");
    static JSON: &str = include_str!("../../../fonts/roboto_msdf.json");

    let atlas = MSDFAtlas::from_bytes(device, queue, PNG, JSON)?;
    Ok(FontFace::from_msdf_atlas("roboto".to_string(), atlas))
}
