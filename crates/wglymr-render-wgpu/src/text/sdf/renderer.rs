use wgpu::{BindGroup, BindGroupLayout, Buffer, Device, Queue, RenderPass, Sampler, TextureFormat};

use super::atlas::{GlyphAtlas, GlyphKey};
use super::batch::{SdfBatch, SdfGlyph};
use super::rasterizer::GlyphRasterizer;
use crate::gpu::ViewportResources;

const MAX_VERTICES: usize = 10000;

pub struct SdfTextRenderer {
    pipeline: wgpu::RenderPipeline,
    viewport: ViewportResources,
    vertex_buffer: Buffer,
    batch: SdfBatch,
    texture_layout: BindGroupLayout,
    sampler: Sampler,
    texture_bind_group: Option<BindGroup>,
    atlas: GlyphAtlas,
    rasterizer: GlyphRasterizer,
}

impl SdfTextRenderer {
    pub fn new(device: &Device, format: TextureFormat) -> Self {
        Self::with_font_data(device, format, None)
    }

    pub fn with_font_data(
        device: &Device,
        format: TextureFormat,
        font_data: Option<&'static [u8]>,
    ) -> Self {
        let viewport = ViewportResources::new(device);

        let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            label: Some("SDF Text Sampler"),
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            address_mode_w: wgpu::AddressMode::ClampToEdge,
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Linear,
            mipmap_filter: wgpu::FilterMode::Nearest,
            anisotropy_clamp: 1,
            ..Default::default()
        });

        let texture_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("SDF Text Bind Group Layout"),
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Texture {
                        sample_type: wgpu::TextureSampleType::Float { filterable: true },
                        view_dimension: wgpu::TextureViewDimension::D2,
                        multisampled: false,
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                    count: None,
                },
            ],
        });

        let pipeline = super::pipeline::create_pipeline(device, format, &viewport, &texture_layout);

        let vertex_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("SDF Text Vertex Buffer"),
            size: (MAX_VERTICES * std::mem::size_of::<super::batch::SdfVertex>())
                as wgpu::BufferAddress,
            usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        let atlas = GlyphAtlas::new(device, 2048, 2048);

        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("SDF Text Bind Group"),
            layout: &texture_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(atlas.view()),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::Sampler(&sampler),
                },
            ],
        });

        let rasterizer = if let Some(data) = font_data {
            GlyphRasterizer::from_font_data(data).unwrap_or_else(|_| GlyphRasterizer::new())
        } else {
            GlyphRasterizer::new()
        };

        Self {
            pipeline,
            viewport,
            vertex_buffer,
            batch: SdfBatch::new(),
            texture_layout,
            sampler,
            texture_bind_group: Some(bind_group),
            atlas,
            rasterizer,
        }
    }

    pub fn begin_frame(&mut self) {
        self.batch.clear();
    }

    pub fn set_viewport(&self, queue: &Queue, viewport: [f32; 2]) {
        self.viewport.update(queue, viewport);
    }

    pub fn set_layer(&mut self, layer: u8) {
        self.batch.set_layer(layer);
    }

    pub fn draw_glyph(
        &mut self,
        device: &Device,
        queue: &Queue,
        glyph_key: GlyphKey,
        screen_pos: [f32; 2],
        scale: f32,
        color: [f32; 4],
        layer: u8,
    ) {
        if let Some(atlas_glyph) = self.atlas.get(&glyph_key) {
            let glyph = SdfGlyph {
                screen_pos: [
                    screen_pos[0] + atlas_glyph.bearing_x as f32 * scale,
                    screen_pos[1] - atlas_glyph.bearing_y as f32 * scale,
                ],
                screen_size: [
                    atlas_glyph.width as f32 * scale,
                    atlas_glyph.height as f32 * scale,
                ],
                uv_min: atlas_glyph.uv_min,
                uv_max: atlas_glyph.uv_max,
                color,
                layer,
            };
            self.batch.push(glyph);
        } else if let Some(rasterized) = self
            .rasterizer
            .rasterize(glyph_key.glyph_id, glyph_key.pixel_size)
        {
            if let Some(atlas_glyph) = self.atlas.insert(
                queue,
                glyph_key,
                &rasterized.bitmap,
                rasterized.width,
                rasterized.height,
                rasterized.bearing_x,
                rasterized.bearing_y,
            ) {
                self.update_bind_group(device);

                let glyph = SdfGlyph {
                    screen_pos: [
                        screen_pos[0] + atlas_glyph.bearing_x as f32 * scale,
                        screen_pos[1] - atlas_glyph.bearing_y as f32 * scale,
                    ],
                    screen_size: [
                        atlas_glyph.width as f32 * scale,
                        atlas_glyph.height as f32 * scale,
                    ],
                    uv_min: atlas_glyph.uv_min,
                    uv_max: atlas_glyph.uv_max,
                    color,
                    layer,
                };
                self.batch.push(glyph);
            }
        }
    }

    fn update_bind_group(&mut self, device: &Device) {
        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("SDF Text Bind Group"),
            layout: &self.texture_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(self.atlas.view()),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::Sampler(&self.sampler),
                },
            ],
        });
        self.texture_bind_group = Some(bind_group);
    }

    pub fn finish_batch(&mut self) {
        self.batch.finish();
    }

    pub fn upload(&self, queue: &Queue) {
        if !self.batch.is_empty() {
            queue.write_buffer(
                &self.vertex_buffer,
                0,
                bytemuck::cast_slice(self.batch.vertices()),
            );
        }
    }

    pub fn render<'a>(&'a self, render_pass: &mut RenderPass<'a>) {
        if self.batch.is_empty() {
            return;
        }

        let texture_bind_group = match &self.texture_bind_group {
            Some(bg) => bg,
            None => return,
        };

        render_pass.set_pipeline(&self.pipeline);
        render_pass.set_bind_group(0, &self.viewport.bind_group, &[]);
        render_pass.set_bind_group(1, texture_bind_group, &[]);
        render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));

        for layer_range in self.batch.layers() {
            let start = layer_range.start * 6;
            let end = layer_range.end * 6;
            render_pass.draw(start..end, 0..1);
        }
    }
}
