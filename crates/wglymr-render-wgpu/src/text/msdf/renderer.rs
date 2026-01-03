use wgpu::{BindGroup, BindGroupLayout, Buffer, Device, Queue, RenderPass, Sampler, TextureFormat};

use super::batch::{MSDFBatch, MSDFGlyph};
use super::font::FontFace;
use crate::gpu::ViewportResources;

const MAX_VERTICES: usize = 10000;

pub struct MSDFTextRenderer {
    pipeline: wgpu::RenderPipeline,
    viewport: ViewportResources,
    vertex_buffer: Buffer,
    batch: MSDFBatch,
    texture_layout: BindGroupLayout,
    sampler: Sampler,
    texture_bind_group: Option<BindGroup>,
    font: Option<FontFace>,
}

impl MSDFTextRenderer {
    pub fn new(device: &Device, format: TextureFormat) -> Self {
        let viewport = ViewportResources::new(device);

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

        let texture_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("MSDF Texture Bind Group Layout"),
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
            label: Some("MSDF Vertex Buffer"),
            size: (MAX_VERTICES * std::mem::size_of::<super::batch::MSDFVertex>())
                as wgpu::BufferAddress,
            usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        Self {
            pipeline,
            viewport,
            vertex_buffer,
            batch: MSDFBatch::new(),
            texture_layout,
            sampler,
            texture_bind_group: None,
            font: None,
        }
    }

    pub fn set_font(&mut self, device: &Device, font: FontFace) {
        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("MSDF Texture Bind Group"),
            layout: &self.texture_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(font.atlas().texture_view()),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::Sampler(&self.sampler),
                },
            ],
        });
        self.texture_bind_group = Some(bind_group);
        self.font = Some(font);
    }

    pub fn font(&self) -> Option<&FontFace> {
        self.font.as_ref()
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

    pub fn draw_glyph(&mut self, glyph: MSDFGlyph) {
        self.batch.push(glyph);
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
