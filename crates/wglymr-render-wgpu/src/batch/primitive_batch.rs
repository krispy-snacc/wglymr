use std::ops::Range;

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex {
    pub position: [f32; 2],
    pub color: [f32; 4],
}

pub struct PrimitiveBatch {
    pub(crate) vertices: Vec<Vertex>,
    pub(crate) line_ranges: Vec<Range<u32>>,
    pub(crate) rect_ranges: Vec<Range<u32>>,
}

impl Default for PrimitiveBatch {
    fn default() -> Self {
        Self::new()
    }
}

impl PrimitiveBatch {
    pub fn new() -> Self {
        Self {
            vertices: Vec::new(),
            line_ranges: Vec::new(),
            rect_ranges: Vec::new(),
        }
    }

    pub fn clear(&mut self) {
        self.vertices.clear();
        self.line_ranges.clear();
        self.rect_ranges.clear();
    }

    pub fn line(&mut self, from: [f32; 2], to: [f32; 2], color: [f32; 4]) {
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

    pub fn rect(&mut self, min: [f32; 2], max: [f32; 2], color: [f32; 4]) {
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

    pub fn vertices(&self) -> &[Vertex] {
        &self.vertices
    }

    pub fn line_ranges(&self) -> &[Range<u32>] {
        &self.line_ranges
    }

    pub fn rect_ranges(&self) -> &[Range<u32>] {
        &self.rect_ranges
    }

    pub fn is_empty(&self) -> bool {
        self.vertices.is_empty()
    }
}
