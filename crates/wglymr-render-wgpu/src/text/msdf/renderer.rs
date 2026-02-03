use super::atlas::MsdfAtlas;
use super::cache::GlyphCache;
use super::glyph::GlyphKey;
use super::pipeline::MsdfPipeline;
use super::vertex::QuadBuilder;
use wgpu::{Device, Queue, RenderPass};

#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
struct ViewportUniform {
    viewport: [f32; 2],
    _padding: [f32; 2],
}

pub struct MsdfTextRenderer {
    atlas: MsdfAtlas,
    cache: GlyphCache,
    pipeline: Option<MsdfPipeline>,
    quad_builder: QuadBuilder,

    atlas_texture: Option<wgpu::Texture>,
    atlas_view: Option<wgpu::TextureView>,
    sampler: Option<wgpu::Sampler>,

    viewport_buffer: Option<wgpu::Buffer>,
    viewport_bind_group: Option<wgpu::BindGroup>,
    texture_bind_group: Option<wgpu::BindGroup>,

    viewport: [f32; 2],
    surface_format: wgpu::TextureFormat,
}

impl MsdfTextRenderer {
    pub fn new(_device: &Device, surface_format: wgpu::TextureFormat) -> Self {
        let atlas = MsdfAtlas::new();

        Self {
            atlas,
            cache: GlyphCache::new(),
            pipeline: None,
            quad_builder: QuadBuilder::new(),
            atlas_texture: None,
            atlas_view: None,
            sampler: None,
            viewport_buffer: None,
            viewport_bind_group: None,
            texture_bind_group: None,
            viewport: [1.0, 1.0],
            surface_format,
        }
    }

    fn ensure_pipeline(&mut self, device: &Device, format: wgpu::TextureFormat) {
        if self.pipeline.is_none() {
            self.pipeline = Some(MsdfPipeline::new(device, format));
        }
    }

    fn ensure_atlas_texture(&mut self, device: &Device, queue: &Queue) {
        if self.atlas_texture.is_none() {
            let texture = device.create_texture(&wgpu::TextureDescriptor {
                label: Some("MSDF Atlas Texture"),
                size: wgpu::Extent3d {
                    width: self.atlas.width(),
                    height: self.atlas.height(),
                    depth_or_array_layers: 1,
                },
                mip_level_count: 1,
                sample_count: 1,
                dimension: wgpu::TextureDimension::D2,
                format: wgpu::TextureFormat::Rgba8Unorm,
                usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
                view_formats: &[],
            });

            queue.write_texture(
                wgpu::TexelCopyTextureInfo {
                    texture: &texture,
                    mip_level: 0,
                    origin: wgpu::Origin3d::ZERO,
                    aspect: wgpu::TextureAspect::All,
                },
                self.atlas.texture_data(),
                wgpu::TexelCopyBufferLayout {
                    offset: 0,
                    bytes_per_row: Some(4 * self.atlas.width()),
                    rows_per_image: Some(self.atlas.height()),
                },
                wgpu::Extent3d {
                    width: self.atlas.width(),
                    height: self.atlas.height(),
                    depth_or_array_layers: 1,
                },
            );

            let view = texture.create_view(&wgpu::TextureViewDescriptor::default());

            let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
                label: Some("MSDF Sampler"),
                address_mode_u: wgpu::AddressMode::ClampToEdge,
                address_mode_v: wgpu::AddressMode::ClampToEdge,
                address_mode_w: wgpu::AddressMode::ClampToEdge,
                mag_filter: wgpu::FilterMode::Linear,
                min_filter: wgpu::FilterMode::Linear,
                mipmap_filter: wgpu::FilterMode::Nearest,
                ..Default::default()
            });

            self.atlas_texture = Some(texture);
            self.atlas_view = Some(view);
            self.sampler = Some(sampler);
        }
    }

    fn ensure_viewport_resources(&mut self, device: &Device) {
        if self.viewport_buffer.is_none() {
            let buffer = device.create_buffer(&wgpu::BufferDescriptor {
                label: Some("MSDF Viewport Buffer"),
                size: std::mem::size_of::<ViewportUniform>() as u64,
                usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
                mapped_at_creation: false,
            });

            let pipeline = self.pipeline.as_ref().unwrap();
            let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
                label: Some("MSDF Viewport Bind Group"),
                layout: pipeline.viewport_bind_group_layout(),
                entries: &[wgpu::BindGroupEntry {
                    binding: 0,
                    resource: buffer.as_entire_binding(),
                }],
            });

            self.viewport_buffer = Some(buffer);
            self.viewport_bind_group = Some(bind_group);
        }
    }

    fn ensure_texture_bind_group(&mut self, device: &Device) {
        if self.texture_bind_group.is_none() {
            let pipeline = self.pipeline.as_ref().unwrap();
            let view = self.atlas_view.as_ref().unwrap();
            let sampler = self.sampler.as_ref().unwrap();

            let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
                label: Some("MSDF Texture Bind Group"),
                layout: pipeline.texture_bind_group_layout(),
                entries: &[
                    wgpu::BindGroupEntry {
                        binding: 0,
                        resource: wgpu::BindingResource::TextureView(view),
                    },
                    wgpu::BindGroupEntry {
                        binding: 1,
                        resource: wgpu::BindingResource::Sampler(sampler),
                    },
                ],
            });

            self.texture_bind_group = Some(bind_group);
        }
    }

    pub fn begin_frame(&mut self) {
        self.quad_builder.clear();
    }

    pub fn set_viewport(&mut self, queue: &Queue, viewport: [f32; 2]) {
        self.viewport = viewport;

        if let Some(buffer) = &self.viewport_buffer {
            let uniform = ViewportUniform {
                viewport,
                _padding: [0.0, 0.0],
            };
            queue.write_buffer(buffer, 0, bytemuck::cast_slice(&[uniform]));
        }
    }

    pub fn draw_text(
        &mut self,
        text: &str,
        screen_position: [f32; 2],
        font_size_px: f32,
        color: wglymr_color::Color,
        depth: f32,
    ) {
        let font_id = 0;
        let pixel_size = font_size_px.max(1.0) as u16;

        let mut x = screen_position[0];
        let y = screen_position[1];

        let distance_range = self.atlas.distance_range();
        let em_size = self.atlas.em_size();
        let pixel_range = distance_range * (font_size_px / em_size);

        for ch in text.chars() {
            let glyph_id = ch as u16;
            let key = GlyphKey::new(font_id, glyph_id, pixel_size);

            let cached = if let Some(cached) = self.cache.get(&key) {
                cached
            } else {
                if let Some(glyph) = self.atlas.get_glyph(key) {
                    self.cache.insert(glyph);
                    self.cache.get(&key).unwrap()
                } else {
                    x += font_size_px * 0.5;
                    continue;
                }
            };

            let pos = [x + cached.metrics.bearing_x, y - cached.metrics.bearing_y];

            let size = [cached.metrics.width, cached.metrics.height];

            self.quad_builder.add_quad(
                pos,
                size,
                cached.uv.min,
                cached.uv.max,
                color.to_gpu_srgb(),
                depth,
                pixel_range,
            );

            x += cached.metrics.advance_x;
        }
    }

    pub fn upload(&mut self, device: &Device, queue: &Queue) {
        if self.quad_builder.is_empty() {
            return;
        }

        self.ensure_pipeline(device, self.surface_format);
        self.ensure_atlas_texture(device, queue);
        self.ensure_viewport_resources(device);
        self.ensure_texture_bind_group(device);

        let pipeline = self.pipeline.as_mut().unwrap();
        pipeline.upload_vertices(device, queue, self.quad_builder.vertices());
        pipeline.upload_indices(device, queue, self.quad_builder.indices());
    }

    pub fn render<'a>(&'a self, pass: &mut RenderPass<'a>) {
        if self.quad_builder.is_empty() {
            return;
        }

        let pipeline = self.pipeline.as_ref().unwrap();
        let viewport_bind_group = self.viewport_bind_group.as_ref().unwrap();
        let texture_bind_group = self.texture_bind_group.as_ref().unwrap();

        pipeline.render(
            pass,
            viewport_bind_group,
            texture_bind_group,
            self.quad_builder.indices().len() as u32,
        );
    }
}
