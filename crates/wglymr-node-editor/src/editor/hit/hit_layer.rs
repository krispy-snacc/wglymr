/// Blender-style semantic interaction regions
/// Text and other decorations do NOT appear here - they use NoHit
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum HitLayer {
    NoHit,
    Background,
    Edges,
    NodeBody,
    NodeHeader,
    NodeSockets,
    Overlay,
}
