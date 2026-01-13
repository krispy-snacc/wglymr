#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DepthLayer {
    Grid,
    Edges,
    NodesInactive,
    NodesActive,
    NodesDragged,
    Overlay,
}

#[derive(Debug, Clone, Copy)]
pub struct DepthBand {
    pub near: f32,
    pub far: f32,
}

pub const DEPTH_BANDS: &[(DepthLayer, DepthBand)] = &[
    (DepthLayer::Grid,          DepthBand { near: 0.90, far: 1.00 }),
    (DepthLayer::Edges,         DepthBand { near: 0.65, far: 0.75 }),
    (DepthLayer::NodesInactive, DepthBand { near: 0.45, far: 0.55 }),
    (DepthLayer::NodesActive,   DepthBand { near: 0.30, far: 0.40 }),
    (DepthLayer::NodesDragged,  DepthBand { near: 0.15, far: 0.25 }),
    (DepthLayer::Overlay,       DepthBand { near: 0.05, far: 0.10 }),
];

// Local offsets inside a node band
pub const Z_BODY: f32   =  0.00;
pub const Z_HEADER: f32 = -0.01;
pub const Z_SOCKET: f32 = -0.02;
pub const Z_TEXT: f32   = -0.03;

pub fn resolve_depth(
    layer: DepthLayer,
    local_offset: f32,
) -> f32 {
    let band = DEPTH_BANDS
        .iter()
        .find(|(l, _)| *l == layer)
        .map(|(_, b)| *b)
        .expect("Depth band missing");

    let center = (band.near + band.far) * 0.5;
    (center + local_offset).clamp(band.near, band.far)
}
