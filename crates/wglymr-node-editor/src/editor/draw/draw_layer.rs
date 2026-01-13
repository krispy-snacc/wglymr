#[repr(i32)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum DrawLayer {
    Grid = 0,
    Edges = 10,
    NodeBody = 20,
    NodeHeader = 30,
    NodeSockets = 40,
    NodeText = 50,
    Overlays = 100,
}
