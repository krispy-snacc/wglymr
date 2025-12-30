use wgpu::{BindGroup, Buffer, Device, Queue, RenderPass, RenderPipeline};

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
struct Vertex {
    position: [f32; 2],
    color: [f32; 4],
}

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
struct ViewportUniform {
    viewport: [f32; 2],
    _padding: [f32; 2],
}

pub struct PrimitiveRenderer {
    line_pipeline: RenderPipeline,
    rect_pipeline: RenderPipeline,
    vertex_buffer: Buffer,
    viewport_buffer: Buffer,
    viewport_bind_group: BindGroup,
    vertices: Vec<Vertex>,
    rect_ranges: Vec<std::ops::Range<u32>>,
    line_ranges: Vec<std::ops::Range<u32>>,
}

impl PrimitiveRenderer {
    pub fn new(device: &Device, format: wgpu::TextureFormat) -> Self {
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Primitive Shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("shaders/primitive.wgsl").into()),
        });

        let vertex_buffer_layout = wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 0,
                    format: wgpu::VertexFormat::Float32x2,
                },
                wgpu::VertexAttribute {
                    offset: std::mem::size_of::<[f32; 2]>() as wgpu::BufferAddress,
                    shader_location: 1,
                    format: wgpu::VertexFormat::Float32x4,
                },
            ],
        };

        let viewport_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Viewport Uniform Buffer"),
            size: std::mem::size_of::<ViewportUniform>() as wgpu::BufferAddress,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        let viewport_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: Some("Viewport Bind Group Layout"),
                entries: &[wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::VERTEX,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                }],
            });

        let viewport_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Viewport Bind Group"),
            layout: &viewport_bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: viewport_buffer.as_entire_binding(),
            }],
        });

        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Primitive Pipeline Layout"),
            bind_group_layouts: &[&viewport_bind_group_layout],
            push_constant_ranges: &[],
        });

        let line_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Line Pipeline"),
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: Some("vs_main"),
                buffers: &[vertex_buffer_layout.clone()],
                compilation_options: Default::default(),
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: Some("fs_main"),
                targets: &[Some(wgpu::ColorTargetState {
                    format,
                    blend: Some(wgpu::BlendState::ALPHA_BLENDING),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
                compilation_options: Default::default(),
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::LineList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: None,
                polygon_mode: wgpu::PolygonMode::Fill,
                unclipped_depth: false,
                conservative: false,
            },
            depth_stencil: None,
            multisample: wgpu::MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            multiview: None,
            cache: None,
        });

        let rect_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Rectangle Pipeline"),
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: Some("vs_main"),
                buffers: &[vertex_buffer_layout],
                compilation_options: Default::default(),
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: Some("fs_main"),
                targets: &[Some(wgpu::ColorTargetState {
                    format,
                    blend: Some(wgpu::BlendState::ALPHA_BLENDING),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
                compilation_options: Default::default(),
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: None,
                polygon_mode: wgpu::PolygonMode::Fill,
                unclipped_depth: false,
                conservative: false,
            },
            depth_stencil: None,
            multisample: wgpu::MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            multiview: None,
            cache: None,
        });

        let max_vertices = 10000;
        let vertex_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Primitive Vertex Buffer"),
            size: (max_vertices * std::mem::size_of::<Vertex>()) as wgpu::BufferAddress,
            usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        Self {
            line_pipeline,
            rect_pipeline,
            vertex_buffer,
            viewport_buffer,
            viewport_bind_group,
            vertices: Vec::new(),
            rect_ranges: Vec::new(),
            line_ranges: Vec::new(),
        }
    }

    pub fn begin_frame(&mut self) {
        self.vertices.clear();
        self.rect_ranges.clear();
        self.line_ranges.clear();
    }

    pub fn set_viewport(&mut self, queue: &Queue, viewport: [f32; 2]) {
        let uniform = ViewportUniform {
            viewport,
            _padding: [0.0, 0.0],
        };
        queue.write_buffer(&self.viewport_buffer, 0, bytemuck::cast_slice(&[uniform]));
    }

    pub fn draw_line(&mut self, from: [f32; 2], to: [f32; 2], color: [f32; 4]) {
        let start = self.vertices.len() as u32;

        self.vertices.push(Vertex {
            position: from,
            color,
        });
        self.vertices.push(Vertex {
            position: to,
            color,
        });

        let end = self.vertices.len() as u32;
        self.line_ranges.push(start..end);
    }

    pub fn draw_rect(&mut self, min: [f32; 2], max: [f32; 2], color: [f32; 4]) {
        let start = self.vertices.len() as u32;

        self.vertices.push(Vertex {
            position: [min[0], min[1]],
            color,
        });
        self.vertices.push(Vertex {
            position: [max[0], min[1]],
            color,
        });
        self.vertices.push(Vertex {
            position: [max[0], max[1]],
            color,
        });
        self.vertices.push(Vertex {
            position: [min[0], min[1]],
            color,
        });
        self.vertices.push(Vertex {
            position: [max[0], max[1]],
            color,
        });
        self.vertices.push(Vertex {
            position: [min[0], max[1]],
            color,
        });

        let end = self.vertices.len() as u32;
        self.rect_ranges.push(start..end);
    }

    /// Draw infinite grid in pixel space.
    /// Grid is stable during pan and scales naturally with zoom.
    pub fn draw_grid(&mut self, pan_world: [f32; 2], zoom: f32, viewport: [f32; 2]) {
        const GRID_WORLD_SPACING: f32 = 100.0;
        const GRID_COLOR: [f32; 4] = [1.0, 1.0, 1.0, 0.03];

        // Convert world spacing → screen spacing
        let spacing_px = GRID_WORLD_SPACING * zoom;

        if spacing_px < 2.0 {
            // Avoid drawing absurdly dense grids
            return;
        }

        let offset_x = (pan_world[0] * zoom).rem_euclid(spacing_px);
        let offset_y = (pan_world[1] * zoom).rem_euclid(spacing_px);

        let viewport_width = viewport[0];
        let viewport_height = viewport[1];

        let mut x = -offset_x;
        while x <= viewport_width {
            self.draw_line([x, 0.0], [x, viewport_height], GRID_COLOR);
            x += spacing_px;
        }

        let mut y = -offset_y;
        while y <= viewport_height {
            self.draw_line([0.0, y], [viewport_width, y], GRID_COLOR);
            y += spacing_px;
        }
    }

    pub fn upload(&self, queue: &Queue) {
        if !self.vertices.is_empty() {
            queue.write_buffer(&self.vertex_buffer, 0, bytemuck::cast_slice(&self.vertices));
        }
    }

    pub fn render_lines<'a>(&'a self, render_pass: &mut RenderPass<'a>) {
        render_pass.set_pipeline(&self.line_pipeline);
        render_pass.set_bind_group(0, &self.viewport_bind_group, &[]);
        render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));

        for range in &self.line_ranges {
            render_pass.draw(range.clone(), 0..1);
        }
    }

    pub fn render_rects<'a>(&'a self, render_pass: &mut RenderPass<'a>) {
        render_pass.set_pipeline(&self.rect_pipeline);
        render_pass.set_bind_group(0, &self.viewport_bind_group, &[]);
        render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));

        for range in &self.rect_ranges {
            render_pass.draw(range.clone(), 0..1);
        }
    }
}
