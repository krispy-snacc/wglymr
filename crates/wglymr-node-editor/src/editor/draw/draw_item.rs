use super::draw_kind::DrawKind;
use super::draw_layer::DrawLayer;

#[derive(Clone, Debug, PartialEq)]
pub struct DrawItem {
    pub layer: DrawLayer,
    pub z: i32,
    pub depth: f32,
    pub kind: DrawKind,
}

impl DrawItem {
    pub fn new(layer: DrawLayer, z: i32, depth: f32, kind: DrawKind) -> Self {
        Self { layer, z, depth, kind }
    }

    pub fn sort_key(&self) -> i32 {
        (self.layer as i32 * 10_000) + self.z
    }
}
