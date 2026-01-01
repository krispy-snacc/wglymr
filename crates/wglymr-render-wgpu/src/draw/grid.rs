use crate::batch::PrimitiveBatch;

const GRID_WORLD_SPACING: f32 = 60.0;
const GRID_COLOR: [f32; 4] = [1.0, 1.0, 1.0, 0.03];

pub fn draw_grid(batch: &mut PrimitiveBatch, pan_world: [f32; 2], zoom: f32, viewport: [f32; 2]) {
    let spacing_px = GRID_WORLD_SPACING * zoom;

    if spacing_px < 2.0 {
        return;
    }

    let offset_x = (pan_world[0] * zoom).rem_euclid(spacing_px);
    let offset_y = (pan_world[1] * zoom).rem_euclid(spacing_px);

    let viewport_width = viewport[0];
    let viewport_height = viewport[1];

    let mut x = -offset_x;
    while x <= viewport_width {
        batch.line([x, 0.0], [x, viewport_height], GRID_COLOR);
        x += spacing_px;
    }

    let mut y = -offset_y;
    while y <= viewport_height {
        batch.line([0.0, y], [viewport_width, y], GRID_COLOR);
        y += spacing_px;
    }
}
