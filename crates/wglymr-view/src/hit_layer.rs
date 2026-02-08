// Hit testing layer definitions
// These are semantic concepts used during interaction, not rendering

/// Hit layer priority for interaction
/// Determines which visual element should respond to user input
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum HitLayer {
    None,
    NoHit,
    NodeBody,
    NodeHeader,
    Socket,
    NodeSockets,
    Edge,
    Edges,
}

impl HitLayer {
    /// Check if this layer can be interacted with
    pub fn is_interactive(self) -> bool {
        !matches!(self, HitLayer::None | HitLayer::NoHit)
    }

    /// Get priority value (higher = more important)
    pub fn priority(self) -> u8 {
        match self {
            HitLayer::None => 0,
            HitLayer::NoHit => 0,
            HitLayer::Edges => 1,
            HitLayer::Edge => 1,
            HitLayer::NodeBody => 2,
            HitLayer::NodeHeader => 3,
            HitLayer::NodeSockets => 4,
            HitLayer::Socket => 4,
        }
    }
}
