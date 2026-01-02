use wgpu::{Device, Extent3d, Queue, Texture, TextureView};

const ATLAS_SIZE: u32 = 1024;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct GlyphKey {
    pub id: u32,
    pub size_px: u16,
}

#[derive(Debug, Clone, Copy)]
pub struct GlyphEntry {
    pub uv_min: [f32; 2],
    pub uv_max: [f32; 2],
    pub size_px: [u16; 2],
}

struct AtlasRegion {
    x: u32,
    y: u32,
    row_height: u32,
}

pub struct GlyphAtlas {
    texture: Texture,
    view: TextureView,
    entries: std::collections::HashMap<GlyphKey, GlyphEntry>,
    current_region: AtlasRegion,
    size: u32,
}

impl GlyphAtlas {
    pub fn new(device: &Device) -> Self {
        let texture = device.create_texture(&wgpu::TextureDescriptor {
            label: Some("Glyph Atlas Texture"),
            size: Extent3d {
                width: ATLAS_SIZE,
                height: ATLAS_SIZE,
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
            entries: std::collections::HashMap::new(),
            current_region: AtlasRegion {
                x: 0,
                y: 0,
                row_height: 0,
            },
            size: ATLAS_SIZE,
        }
    }

    pub fn texture_view(&self) -> &TextureView {
        &self.view
    }

    pub fn get(&self, key: &GlyphKey) -> Option<&GlyphEntry> {
        self.entries.get(key)
    }

    pub fn contains(&self, key: &GlyphKey) -> bool {
        self.entries.contains_key(key)
    }

    pub fn insert(
        &mut self,
        queue: &Queue,
        key: GlyphKey,
        width: u16,
        height: u16,
        data: &[u8],
    ) -> Option<GlyphEntry> {
        if self.entries.contains_key(&key) {
            return self.entries.get(&key).copied();
        }

        let (x, y) = self.allocate(width as u32, height as u32)?;

        queue.write_texture(
            wgpu::TexelCopyTextureInfo {
                texture: &self.texture,
                mip_level: 0,
                origin: wgpu::Origin3d { x, y, z: 0 },
                aspect: wgpu::TextureAspect::All,
            },
            data,
            wgpu::TexelCopyBufferLayout {
                offset: 0,
                bytes_per_row: Some(width as u32),
                rows_per_image: Some(height as u32),
            },
            Extent3d {
                width: width as u32,
                height: height as u32,
                depth_or_array_layers: 1,
            },
        );

        let atlas_size_f = self.size as f32;
        let entry = GlyphEntry {
            uv_min: [x as f32 / atlas_size_f, y as f32 / atlas_size_f],
            uv_max: [
                (x + width as u32) as f32 / atlas_size_f,
                (y + height as u32) as f32 / atlas_size_f,
            ],
            size_px: [width, height],
        };

        self.entries.insert(key, entry);
        Some(entry)
    }

    fn allocate(&mut self, width: u32, height: u32) -> Option<(u32, u32)> {
        const PADDING: u32 = 1;

        if self.current_region.x + width + PADDING > self.size {
            self.current_region.x = 0;
            self.current_region.y += self.current_region.row_height + PADDING;
            self.current_region.row_height = 0;
        }

        if self.current_region.y + height + PADDING > self.size {
            return None;
        }

        let x = self.current_region.x;
        let y = self.current_region.y;

        self.current_region.x += width + PADDING;
        self.current_region.row_height = self.current_region.row_height.max(height);

        Some((x, y))
    }

    pub fn clear(&mut self) {
        self.entries.clear();
        self.current_region = AtlasRegion {
            x: 0,
            y: 0,
            row_height: 0,
        };
    }
}
