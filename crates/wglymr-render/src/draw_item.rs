use super::draw_kind::DrawKind;
use super::draw_layer::DrawLayer;
use super::entity_metadata::EntityMetadata;

/// Opaque hit layer identifier (u8)
///
/// The render ABI doesn't interpret hit semantics - it just passes this value through.
/// Actual HitLayer enum lives in wglymr-view where hit testing happens.
/// This maintains render ABI purity.
pub type HitLayer = u8;

#[derive(Clone, Debug, PartialEq)]
pub struct DrawItem {
    pub draw_layer: DrawLayer,
    pub hit_layer: HitLayer,
    pub z: i32,
    pub depth: f32,
    pub kind: DrawKind,
    pub entity: EntityMetadata,
}

impl DrawItem {
    pub fn new(
        draw_layer: DrawLayer,
        hit_layer: HitLayer,
        z: i32,
        depth: f32,
        kind: DrawKind,
    ) -> Self {
        Self {
            draw_layer,
            hit_layer,
            z,
            depth,
            kind,
            entity: EntityMetadata::default(),
        }
    }

    pub fn with_entity(mut self, entity: EntityMetadata) -> Self {
        self.entity = entity;
        self
    }

    pub fn sort_key(&self) -> i32 {
        (self.draw_layer as i32 * 10_000) + self.z
    }
}
