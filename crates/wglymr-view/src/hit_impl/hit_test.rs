use crate::hit_layer::HitLayer;
use wglymr_render::{DrawItem, DrawKind};

#[derive(Debug, Clone)]
pub struct HitResult {
    pub item_index: usize,
    pub hit_layer: u8,
    pub depth: f32,
}

/// Blender-style hit-testing: semantic regions only
/// NoHit elements (text, decorations) are automatically excluded
pub fn hit_test(mouse_pos: [f32; 2], draw_items: &[DrawItem]) -> Option<HitResult> {
    let mut candidates: Vec<(usize, u8, f32)> = draw_items
        .iter()
        .enumerate()
        .filter_map(|(idx, item)| {
            // Exclude decorative elements (Blender-correct: only semantic regions)
            if item.hit_layer == HitLayer::NoHit as u8 {
                return None;
            }

            if item_contains_point(item, mouse_pos) {
                Some((idx, item.hit_layer, item.depth))
            } else {
                None
            }
        })
        .collect();

    if candidates.is_empty() {
        return None;
    }

    // Sort by HitLayer (descending - higher priority wins)
    // then by depth (ascending - closer wins)
    candidates.sort_by(|a, b| {
        b.1.cmp(&a.1)
            .then_with(|| a.2.partial_cmp(&b.2).unwrap_or(std::cmp::Ordering::Equal))
    });

    let (item_index, hit_layer, depth) = candidates[0];
    Some(HitResult {
        item_index,
        hit_layer,
        depth,
    })
}

fn item_contains_point(item: &DrawItem, point: [f32; 2]) -> bool {
    match &item.kind {
        DrawKind::Circle(circle) => {
            let dist = distance_to_point(point, circle.center);
            dist <= circle.radius
        }
        DrawKind::RoundedRect(rect) => {
            let max = [
                rect.position[0] + rect.size[0],
                rect.position[1] + rect.size[1],
            ];
            point_in_rect(point, rect.position, max)
        }
        DrawKind::Rect(rect) => {
            let max = [
                rect.position[0] + rect.size[0],
                rect.position[1] + rect.size[1],
            ];
            point_in_rect(point, rect.position, max)
        }
        DrawKind::Line(line) => {
            let tolerance = line.thickness * 0.5 + 2.0;
            distance_to_segment(point, line.start, line.end) <= tolerance
        }
        DrawKind::Glyph(glyph) => {
            let approx_width = glyph.text.len() as f32 * glyph.font_size * 0.6;
            let approx_height = glyph.font_size;
            let max = [
                glyph.world_position[0] + approx_width,
                glyph.world_position[1] + approx_height,
            ];
            point_in_rect(point, glyph.world_position, max)
        }
    }
}

fn point_in_rect(point: [f32; 2], min: [f32; 2], max: [f32; 2]) -> bool {
    point[0] >= min[0] && point[0] <= max[0] && point[1] >= min[1] && point[1] <= max[1]
}

fn distance_to_point(p1: [f32; 2], p2: [f32; 2]) -> f32 {
    let dx = p1[0] - p2[0];
    let dy = p1[1] - p2[1];
    (dx * dx + dy * dy).sqrt()
}

fn distance_to_segment(p: [f32; 2], a: [f32; 2], b: [f32; 2]) -> f32 {
    let dx = b[0] - a[0];
    let dy = b[1] - a[1];
    let length_squared = dx * dx + dy * dy;

    if length_squared < 1e-8 {
        return distance_to_point(p, a);
    }

    let t = ((p[0] - a[0]) * dx + (p[1] - a[1]) * dy) / length_squared;
    let t = t.clamp(0.0, 1.0);

    let closest = [a[0] + t * dx, a[1] + t * dy];
    distance_to_point(p, closest)
}
