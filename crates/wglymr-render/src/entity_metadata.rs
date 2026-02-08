/// Entity metadata for DrawItems to enable semantic interaction mapping.
/// Purely for CPU-side interaction logic; does not affect rendering.
/// Uses opaque u64 IDs to avoid coupling to document layer.
#[derive(Debug, Clone, PartialEq, Default)]
pub enum EntityMetadata {
    #[default]
    None,
    Node(u64),
    Socket {
        node_id: u64,
        socket_id: u64,
    },
    Edge(u64),
}
