use std::ops::Range;

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct TextVertex {
    pub position: [f32; 2],
    pub uv: [f32; 2],
    pub color: [f32; 4],
}

#[derive(Debug, Clone, Copy)]
pub struct GpuGlyph {
    pub screen_pos: [f32; 2],
    pub size: [f32; 2],
    pub uv_min: [f32; 2],
    pub uv_max: [f32; 2],
    pub color: [f32; 4],
    pub layer: u8,
}

pub struct TextBatch {
    vertices: Vec<TextVertex>,
    layers: Vec<Range<u32>>,
    current_layer: u8,
    layer_start: u32,
}

impl Default for TextBatch {
    fn default() -> Self {
        Self::new()
    }
}

impl TextBatch {
    pub fn new() -> Self {
        Self {
            vertices: Vec::new(),
            layers: Vec::new(),
            current_layer: 0,
            layer_start: 0,
        }
    }

    pub fn clear(&mut self) {
        self.vertices.clear();
        self.layers.clear();
        self.current_layer = 0;
        self.layer_start = 0;
    }

    pub fn set_layer(&mut self, layer: u8) {
        if layer != self.current_layer && !self.vertices.is_empty() {
            let end = self.vertices.len() as u32;
            if end > self.layer_start {
                self.layers.push(self.layer_start..end);
            }
            self.layer_start = end;
        }
        self.current_layer = layer;
    }

    pub fn push(&mut self, glyph: GpuGlyph) {
        if glyph.layer != self.current_layer {
            self.set_layer(glyph.layer);
        }

        let [x, y] = glyph.screen_pos;
        let [w, h] = glyph.size;

        let top_left = TextVertex {
            position: [x, y],
            uv: glyph.uv_min,
            color: glyph.color,
        };
        let top_right = TextVertex {
            position: [x + w, y],
            uv: [glyph.uv_max[0], glyph.uv_min[1]],
            color: glyph.color,
        };
        let bottom_right = TextVertex {
            position: [x + w, y + h],
            uv: glyph.uv_max,
            color: glyph.color,
        };
        let bottom_left = TextVertex {
            position: [x, y + h],
            uv: [glyph.uv_min[0], glyph.uv_max[1]],
            color: glyph.color,
        };

        self.vertices.push(top_left);
        self.vertices.push(top_right);
        self.vertices.push(bottom_right);
        self.vertices.push(top_left);
        self.vertices.push(bottom_right);
        self.vertices.push(bottom_left);
    }

    pub fn finish(&mut self) {
        let end = self.vertices.len() as u32;
        if end > self.layer_start {
            self.layers.push(self.layer_start..end);
        }
        self.layer_start = end;
    }

    pub fn vertices(&self) -> &[TextVertex] {
        &self.vertices
    }

    pub fn layer_ranges(&self) -> &[Range<u32>] {
        &self.layers
    }

    pub fn is_empty(&self) -> bool {
        self.vertices.is_empty()
    }

    pub fn vertex_count(&self) -> usize {
        self.vertices.len()
    }
}
