use wgpu::{Buffer, Device, Queue, RenderPass, TextureFormat};

use super::{RoundedRect, SdfBatch};
use crate::gpu::ViewportResources;

const MAX_VERTICES: usize = 10000;

pub struct SdfRenderer {
    pipeline: wgpu::RenderPipeline,
    viewport: ViewportResources,
    vertex_buffer: Buffer,
    batch: SdfBatch,
}

impl SdfRenderer {
    pub fn new(device: &Device, format: TextureFormat) -> Self {
        let viewport = ViewportResources::new(device);
        let pipeline = super::pipeline::create_pipeline(device, format, &viewport);

        let vertex_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("SDF Vertex Buffer"),
            size: (MAX_VERTICES * std::mem::size_of::<crate::sdf::SdfVertex>())
                as wgpu::BufferAddress,
            usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        Self {
            pipeline,
            viewport,
            vertex_buffer,
            batch: SdfBatch::new(),
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

    pub fn draw_rounded_rect(&mut self, rect: &RoundedRect) {
        self.batch.draw_rounded_rect(rect);
    }

    pub fn finish_batch(&mut self) {
        // Reserved for future batch management
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

        render_pass.set_pipeline(&self.pipeline);
        render_pass.set_bind_group(0, &self.viewport.bind_group, &[]);
        render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
        render_pass.draw(0..self.batch.vertices().len() as u32, 0..1);
    }
}
