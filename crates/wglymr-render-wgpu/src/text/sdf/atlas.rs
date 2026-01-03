use std::collections::HashMap;
use wgpu::{Device, Queue, Texture, TextureView};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct GlyphKey {
    pub font_id: u32,
    pub glyph_id: u16,
    pub pixel_size: u32,
}

#[derive(Debug, Clone)]
pub struct AtlasGlyph {
    pub uv_min: [f32; 2],
    pub uv_max: [f32; 2],
    pub width: u32,
    pub height: u32,
    pub bearing_x: i32,
    pub bearing_y: i32,
}

pub struct GlyphAtlas {
    texture: Texture,
    view: TextureView,
    width: u32,
    height: u32,
    current_x: u32,
    current_y: u32,
    row_height: u32,
    glyphs: HashMap<GlyphKey, AtlasGlyph>,
}

impl GlyphAtlas {
    pub fn new(device: &Device, width: u32, height: u32) -> Self {
        let texture = device.create_texture(&wgpu::TextureDescriptor {
            label: Some("SDF Glyph Atlas"),
            size: wgpu::Extent3d {
                width,
                height,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::R8Unorm,
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
            view_formats: &[],
        });

        let view = texture.create_view(&wgpu::TextureViewDescriptor::default());

        Self {
            texture,
            view,
            width,
            height,
            current_x: 0,
            current_y: 0,
            row_height: 0,
            glyphs: HashMap::new(),
        }
    }

    pub fn insert(
        &mut self,
        queue: &Queue,
        key: GlyphKey,
        bitmap: &[u8],
        width: u32,
        height: u32,
        bearing_x: i32,
        bearing_y: i32,
    ) -> Option<AtlasGlyph> {
        if self.glyphs.contains_key(&key) {
            return self.glyphs.get(&key).cloned();
        }

        const PADDING: u32 = 2;
        let padded_width = width + PADDING * 2;
        let padded_height = height + PADDING * 2;

        if self.current_x + padded_width > self.width {
            self.current_x = 0;
            self.current_y += self.row_height;
            self.row_height = 0;
        }

        if self.current_y + padded_height > self.height {
            return None;
        }

        if width > 0 && height > 0 {
            queue.write_texture(
                wgpu::TexelCopyTextureInfo {
                    texture: &self.texture,
                    mip_level: 0,
                    origin: wgpu::Origin3d {
                        x: self.current_x + PADDING,
                        y: self.current_y + PADDING,
                        z: 0,
                    },
                    aspect: wgpu::TextureAspect::All,
                },
                bitmap,
                wgpu::TexelCopyBufferLayout {
                    offset: 0,
                    bytes_per_row: Some(width),
                    rows_per_image: Some(height),
                },
                wgpu::Extent3d {
                    width,
                    height,
                    depth_or_array_layers: 1,
                },
            );
        }

        let uv_min = [
            (self.current_x + PADDING) as f32 / self.width as f32,
            (self.current_y + PADDING) as f32 / self.height as f32,
        ];
        let uv_max = [
            (self.current_x + PADDING + width) as f32 / self.width as f32,
            (self.current_y + PADDING + height) as f32 / self.height as f32,
        ];

        let glyph = AtlasGlyph {
            uv_min,
            uv_max,
            width,
            height,
            bearing_x,
            bearing_y,
        };

        self.glyphs.insert(key, glyph.clone());

        self.current_x += padded_width;
        self.row_height = self.row_height.max(padded_height);

        Some(glyph)
    }

    pub fn get(&self, key: &GlyphKey) -> Option<&AtlasGlyph> {
        self.glyphs.get(key)
    }

    pub fn view(&self) -> &TextureView {
        &self.view
    }
}
