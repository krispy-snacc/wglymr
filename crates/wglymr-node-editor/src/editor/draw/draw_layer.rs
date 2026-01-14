#[repr(i32)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum DrawLayer {
    Background = 0,
    Edges = 10,
    NodeBody = 20,
    NodeHeader = 30,
    NodeSockets = 40,
    NodeText = 50,
    Overlay = 100,
}
