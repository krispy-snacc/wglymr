use wgpu::{BindGroup, Buffer, Device, Queue, RenderPass, TextureFormat};

use super::atlas::GlyphAtlas;
use super::batch::{GpuGlyph, TextBatch, TextVertex};
use super::pipeline::TextPipeline;
use crate::gpu::ViewportResources;

const MAX_TEXT_VERTICES: usize = 60000;

pub struct TextRenderer {
    atlas: GlyphAtlas,
    pipeline: TextPipeline,
    vertex_buffer: Buffer,
    batch: TextBatch,
    texture_bind_group: BindGroup,
}

impl TextRenderer {
    pub fn new(device: &Device, format: TextureFormat, viewport: &ViewportResources) -> Self {
        let atlas = GlyphAtlas::new(device);
        let pipeline = TextPipeline::new(device, format, viewport);

        let vertex_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Text Vertex Buffer"),
            size: (MAX_TEXT_VERTICES * std::mem::size_of::<TextVertex>()) as wgpu::BufferAddress,
            usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        let texture_bind_group = pipeline.create_texture_bind_group(device, atlas.texture_view());

        Self {
            atlas,
            pipeline,
            vertex_buffer,
            batch: TextBatch::new(),
            texture_bind_group,
        }
    }

    pub fn begin_frame(&mut self) {
        self.batch.clear();
    }

    pub fn atlas(&self) -> &GlyphAtlas {
        &self.atlas
    }

    pub fn atlas_mut(&mut self) -> &mut GlyphAtlas {
        &mut self.atlas
    }

    pub fn set_viewport(&self, queue: &Queue, viewport: [f32; 2]) {
        self.pipeline.update_viewport(queue, viewport);
    }

    pub fn set_layer(&mut self, layer: u8) {
        self.batch.set_layer(layer);
    }

    pub fn draw_glyph(&mut self, glyph: GpuGlyph) {
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

    pub fn rebuild_texture_bind_group(&mut self, device: &Device) {
        self.texture_bind_group = self
            .pipeline
            .create_texture_bind_group(device, self.atlas.texture_view());
    }

    pub fn render<'a>(&'a self, render_pass: &mut RenderPass<'a>) {
        if self.batch.is_empty() {
            return;
        }

        render_pass.set_pipeline(self.pipeline.pipeline());
        render_pass.set_bind_group(0, self.pipeline.viewport_bind_group(), &[]);
        render_pass.set_bind_group(1, &self.texture_bind_group, &[]);
        render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));

        for range in self.batch.layer_ranges() {
            render_pass.draw(range.clone(), 0..1);
        }
    }

    pub fn glyph_count(&self) -> usize {
        self.batch.vertex_count() / 6
    }
}
