use wgpu::{Buffer, Device, Queue, RenderPass, TextureFormat};

use super::PrimitiveBatch;
use super::{PrimitivePipelines, ViewportResources, create_primitive_pipelines};

const MAX_VERTICES: usize = 10000;

pub struct PrimitiveRenderer {
    pipelines: PrimitivePipelines,
    viewport: ViewportResources,
    vertex_buffer: Buffer,
    batch: PrimitiveBatch,
}

impl PrimitiveRenderer {
    pub fn new(device: &Device, format: TextureFormat) -> Self {
        let viewport = ViewportResources::new(device);
        let pipelines = create_primitive_pipelines(device, format, &viewport.layout);

        let vertex_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Primitive Vertex Buffer"),
            size: (MAX_VERTICES * std::mem::size_of::<super::Vertex>()) as wgpu::BufferAddress,
            usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        Self {
            pipelines,
            viewport,
            vertex_buffer,
            batch: PrimitiveBatch::new(),
        }
    }

    pub fn begin_frame(&mut self) {
        self.batch.clear();
    }

    pub fn set_viewport(&self, queue: &Queue, viewport: [f32; 2]) {
        self.viewport.update(queue, viewport);
    }

    pub fn batch(&mut self) -> &mut PrimitiveBatch {
        &mut self.batch
    }

    pub fn draw_line(
        &mut self,
        from: [f32; 2],
        to: [f32; 2],
        color: wglymr_color::Color,
        depth: f32,
    ) {
        self.batch.line(from, to, color, depth);
    }

    pub fn draw_rect(
        &mut self,
        min: [f32; 2],
        max: [f32; 2],
        color: wglymr_color::Color,
        depth: f32,
    ) {
        self.batch.rect(min, max, color, depth);
    }

    pub fn draw_grid(&mut self, pan_world: [f32; 2], zoom: f32, viewport: [f32; 2], depth: f32) {
        super::draw_grid(&mut self.batch, pan_world, zoom, viewport, depth);
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

    pub fn render_lines<'a>(&'a self, render_pass: &mut RenderPass<'a>) {
        render_pass.set_pipeline(&self.pipelines.line);
        render_pass.set_bind_group(0, &self.viewport.bind_group, &[]);
        render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));

        for range in self.batch.line_ranges() {
            render_pass.draw(range.clone(), 0..1);
        }
    }

    pub fn render_rects<'a>(&'a self, render_pass: &mut RenderPass<'a>) {
        render_pass.set_pipeline(&self.pipelines.rect);
        render_pass.set_bind_group(0, &self.viewport.bind_group, &[]);
        render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));

        for range in self.batch.rect_ranges() {
            render_pass.draw(range.clone(), 0..1);
        }
    }
}
