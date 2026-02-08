# wglymr-font-assets

Font asset management for WGLYMR.

## Purpose

This crate owns and manages all font assets used by the editor:

- **Raw font files** (.ttf) in `assets/fonts/`
- **Generated MSDF atlases** for scalable UI text in `assets/generated/`
- **Generated bitmap atlases** for pixel-aligned text in `assets/generated/`

## Architecture

Per BUILD.md, this crate:

- **Owns** all font files and generated atlases
- **Embeds** assets into the binary at compile time
- **Provides** typed accessors for font data
- **Does not contain** any rendering logic

Rendering logic lives in `wglymr-render-wgpu`.

## Font Generation

Font atlases are **explicitly generated** and **checked into the repository**.

To regenerate fonts:

```bash
cargo run -p xtask -- fonts
```

This ensures:

- Fast builds (no generation during normal compilation)
- Reproducibility (deterministic outputs)
- CI stability (no external tool dependencies)

## Font Rendering Modes

WGLYMR supports multiple text rendering modes:

- **MSDF** (Multi-channel Signed Distance Field) for scalable UI text
- **Bitmap fonts** for pixel-aligned or small-size text

The renderer chooses the appropriate mode at runtime based on scale and context.

## Usage

```rust
use wglymr_font_assets::{FontFamily, get_msdf_atlas, get_bitmap_atlas};

// Get MSDF atlas for scalable text
let atlas = get_msdf_atlas(FontFamily::Roboto).unwrap();

// Get bitmap atlas for pixel-perfect text
let bitmap = get_bitmap_atlas(FontFamily::Mono).unwrap();
```

## Directory Structure

```
wglymr-font-assets/
├── assets/
│   ├── fonts/              # Raw .ttf files
│   └── generated/          # Generated atlases (checked in)
├── src/
│   ├── lib.rs
│   ├── fonts.rs           # Raw font access
│   ├── msdf.rs            # MSDF atlas access
│   └── bitmap.rs          # Bitmap atlas access
└── build.rs               # Build-time embedding
```

## Dependencies

This crate has **no external crate dependencies** - it only provides data.

Font generation (via xtask) may require external tools like `msdfgen`.
