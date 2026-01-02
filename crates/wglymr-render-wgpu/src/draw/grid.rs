use crate::batch::PrimitiveBatch;

const GRID_WORLD_SPACING: f32 = 60.0;
const GRID_COLOR: [f32; 4] = [1.0, 1.0, 1.0, 0.03];

pub fn draw_grid(
    batch: &mut PrimitiveBatch,
    pan_world: [f32; 2], // world-space center
    zoom: f32,
    viewport: [f32; 2],
) {
    let spacing_px = GRID_WORLD_SPACING * zoom;
    if spacing_px < 2.0 {
        return;
    }

    let half_w = viewport[0] * 0.5;
    let half_h = viewport[1] * 0.5;

    // World origin projected into screen space
    let origin_screen_x = (-pan_world[0]) * zoom + half_w;
    let origin_screen_y = (-pan_world[1]) * zoom + half_h;

    // Stable grid offset
    let offset_x = origin_screen_x.rem_euclid(spacing_px);
    let offset_y = origin_screen_y.rem_euclid(spacing_px);

    let mut x = offset_x;
    while x <= viewport[0] {
        batch.line([x, 0.0], [x, viewport[1]], GRID_COLOR);
        x += spacing_px;
    }

    let mut y = offset_y;
    while y <= viewport[1] {
        batch.line([0.0, y], [viewport[0], y], GRID_COLOR);
        y += spacing_px;
    }
    // batch.line([0.0, 0.0], [viewport[0], viewport[1]], [1.0, 0.0, 0.0, 1.0]);
}
