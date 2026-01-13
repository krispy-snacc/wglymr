use bytemuck::{Pod, Zeroable};

/// Vertex format for MSDF text rendering
/// Each glyph is rendered as a quad (4 vertices, 6 indices)
#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct MsdfVertex {
    pub position: [f32; 2],
    pub uv: [f32; 2],
    pub color: [f32; 4],
    pub depth: f32,
}

impl MsdfVertex {
    pub fn desc() -> wgpu::VertexBufferLayout<'static> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<MsdfVertex>() as wgpu::BufferAddress,
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
                    format: wgpu::VertexFormat::Float32x2,
                },
                wgpu::VertexAttribute {
                    offset: std::mem::size_of::<[f32; 4]>() as wgpu::BufferAddress,
                    shader_location: 2,
                    format: wgpu::VertexFormat::Float32x4,
                },
                wgpu::VertexAttribute {
                    offset: (std::mem::size_of::<[f32; 2]>()
                        + std::mem::size_of::<[f32; 2]>()
                        + std::mem::size_of::<[f32; 4]>())
                        as wgpu::BufferAddress,
                    shader_location: 3,
                    format: wgpu::VertexFormat::Float32,
                },
            ],
        }
    }
}

/// Helper to build quads for glyphs
pub struct QuadBuilder {
    vertices: Vec<MsdfVertex>,
    indices: Vec<u16>,
}

impl QuadBuilder {
    pub fn new() -> Self {
        Self {
            vertices: Vec::new(),
            indices: Vec::new(),
        }
    }

    pub fn add_quad(
        &mut self,
        screen_pos: [f32; 2],
        size: [f32; 2],
        uv_min: [f32; 2],
        uv_max: [f32; 2],
        color: [f32; 4],
        depth: f32,
    ) {
        let base_index = self.vertices.len() as u16;

        let x0 = screen_pos[0];
        let y0 = screen_pos[1];
        let x1 = x0 + size[0];
        let y1 = y0 + size[1];

        self.vertices.extend_from_slice(&[
            MsdfVertex {
                position: [x0, y0],
                uv: [uv_min[0], uv_min[1]],
                color,
                depth,
            },
            MsdfVertex {
                position: [x1, y0],
                uv: [uv_max[0], uv_min[1]],
                color,
                depth,
            },
            MsdfVertex {
                position: [x1, y1],
                uv: [uv_max[0], uv_max[1]],
                color,
                depth,
            },
            MsdfVertex {
                position: [x0, y1],
                uv: [uv_min[0], uv_max[1]],
                color,
                depth,
            },
        ]);

        self.indices.extend_from_slice(&[
            base_index,
            base_index + 1,
            base_index + 2,
            base_index,
            base_index + 2,
            base_index + 3,
        ]);
    }

    pub fn vertices(&self) -> &[MsdfVertex] {
        &self.vertices
    }

    pub fn indices(&self) -> &[u16] {
        &self.indices
    }

    pub fn clear(&mut self) {
        self.vertices.clear();
        self.indices.clear();
    }

    pub fn is_empty(&self) -> bool {
        self.vertices.is_empty()
    }
}

impl Default for QuadBuilder {
    fn default() -> Self {
        Self::new()
    }
}
