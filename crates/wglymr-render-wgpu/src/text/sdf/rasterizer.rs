use std::collections::HashMap;
use ttf_parser::{Face, GlyphId, OutlineBuilder};

pub struct RasterizedGlyph {
    pub bitmap: Vec<u8>,
    pub width: u32,
    pub height: u32,
    pub bearing_x: i32,
    pub bearing_y: i32,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct CacheKey {
    glyph_id: u16,
    pixel_size: u32,
}

pub struct GlyphRasterizer {
    font: Face<'static>,
    cache: HashMap<CacheKey, RasterizedGlyph>,
}

impl GlyphRasterizer {
    pub fn new() -> Self {
        static ROBOTO_FONT: &[u8] = include_bytes!("../../../../../fonts/DejaVuSans.ttf");
        Self::from_font_data(ROBOTO_FONT).expect("Failed to load default font")
    }

    pub fn from_font_data(font_data: &'static [u8]) -> Result<Self, String> {
        let font =
            Face::parse(font_data, 0).map_err(|_| "Failed to parse font data".to_string())?;

        Ok(Self {
            font,
            cache: HashMap::new(),
        })
    }

    pub fn rasterize(&mut self, glyph_id: u16, pixel_size: u32) -> Option<&RasterizedGlyph> {
        let key = CacheKey {
            glyph_id,
            pixel_size,
        };

        if !self.cache.contains_key(&key) {
            let glyph = self.rasterize_sdf(glyph_id, pixel_size)?;
            self.cache.insert(key, glyph);
        }

        self.cache.get(&key)
    }

    fn rasterize_sdf(&self, glyph_id: u16, pixel_size: u32) -> Option<RasterizedGlyph> {
        let gid = GlyphId(glyph_id);
        let units_per_em = self.font.units_per_em() as f32;
        let scale = pixel_size as f32 / units_per_em;

        let mut outline = VectorOutline::new(scale);
        self.font.outline_glyph(gid, &mut outline)?;

        if outline.points.is_empty() {
            return Some(RasterizedGlyph {
                bitmap: vec![],
                width: 0,
                height: 0,
                bearing_x: 0,
                bearing_y: 0,
            });
        }

        let bounds = outline.bounds();

        const PADDING: f32 = 3.0;
        const SDF_RANGE: f32 = 6.0;

        let min_x = bounds.0 - PADDING;
        let min_y = bounds.1 - PADDING;
        let max_x = bounds.2 + PADDING;
        let max_y = bounds.3 + PADDING;

        let width = ((max_x - min_x).ceil() as u32).max(1);
        let height = ((max_y - min_y).ceil() as u32).max(1);

        let mut bitmap = vec![0u8; (width * height) as usize];

        for py in 0..height {
            for px in 0..width {
                let world_x = min_x + px as f32 + 0.5;
                let world_y = max_y - py as f32 - 0.5;

                let signed_distance = outline.compute_signed_distance(world_x, world_y);
                let normalized = 0.5 + signed_distance / (SDF_RANGE * 2.0);
                let clamped = normalized.clamp(0.0, 1.0);

                bitmap[(py * width + px) as usize] = (clamped * 255.0) as u8;
            }
        }

        Some(RasterizedGlyph {
            bitmap,
            width,
            height,
            bearing_x: min_x as i32,
            bearing_y: max_y as i32,
        })
    }
}

struct VectorOutline {
    scale: f32,
    points: Vec<(f32, f32)>,
    contours: Vec<usize>,
}

impl VectorOutline {
    fn new(scale: f32) -> Self {
        Self {
            scale,
            points: Vec::new(),
            contours: Vec::new(),
        }
    }

    fn bounds(&self) -> (f32, f32, f32, f32) {
        if self.points.is_empty() {
            return (0.0, 0.0, 0.0, 0.0);
        }

        let mut min_x = f32::MAX;
        let mut min_y = f32::MAX;
        let mut max_x = f32::MIN;
        let mut max_y = f32::MIN;

        for &(x, y) in &self.points {
            min_x = min_x.min(x);
            min_y = min_y.min(y);
            max_x = max_x.max(x);
            max_y = max_y.max(y);
        }

        (min_x, min_y, max_x, max_y)
    }

    fn compute_signed_distance(&self, px: f32, py: f32) -> f32 {
        if self.points.is_empty() {
            return 0.0;
        }

        let mut min_distance = f32::MAX;
        let mut winding = 0i32;
        let mut contour_start = 0;

        for &contour_end in &self.contours {
            for i in contour_start..contour_end {
                let next = if i + 1 < contour_end {
                    i + 1
                } else {
                    contour_start
                };

                let (x0, y0) = self.points[i];
                let (x1, y1) = self.points[next];

                let dist = point_to_segment_distance(px, py, x0, y0, x1, y1);
                min_distance = min_distance.min(dist);

                if (y0 <= py && y1 > py) || (y1 <= py && y0 > py) {
                    let t = (py - y0) / (y1 - y0);
                    let x_intersect = x0 + t * (x1 - x0);
                    if px < x_intersect {
                        if y1 > y0 {
                            winding += 1;
                        } else {
                            winding -= 1;
                        }
                    }
                }
            }

            contour_start = contour_end;
        }

        if winding != 0 {
            min_distance
        } else {
            -min_distance
        }
    }
}

impl OutlineBuilder for VectorOutline {
    fn move_to(&mut self, x: f32, y: f32) {
        if !self.points.is_empty() {
            self.contours.push(self.points.len());
        }
        self.points.push((x * self.scale, y * self.scale));
    }

    fn line_to(&mut self, x: f32, y: f32) {
        self.points.push((x * self.scale, y * self.scale));
    }

    fn quad_to(&mut self, x1: f32, y1: f32, x: f32, y: f32) {
        if let Some(&(x0, y0)) = self.points.last() {
            for i in 1..=10 {
                let t = i as f32 / 10.0;
                let s = 1.0 - t;
                let qx = s * s * x0 + 2.0 * s * t * x1 * self.scale + t * t * x * self.scale;
                let qy = s * s * y0 + 2.0 * s * t * y1 * self.scale + t * t * y * self.scale;
                self.points.push((qx, qy));
            }
        }
    }

    fn curve_to(&mut self, x1: f32, y1: f32, x2: f32, y2: f32, x: f32, y: f32) {
        if let Some(&(x0, y0)) = self.points.last() {
            for i in 1..=10 {
                let t = i as f32 / 10.0;
                let s = 1.0 - t;
                let cx = s * s * s * x0
                    + 3.0 * s * s * t * x1 * self.scale
                    + 3.0 * s * t * t * x2 * self.scale
                    + t * t * t * x * self.scale;
                let cy = s * s * s * y0
                    + 3.0 * s * s * t * y1 * self.scale
                    + 3.0 * s * t * t * y2 * self.scale
                    + t * t * t * y * self.scale;
                self.points.push((cx, cy));
            }
        }
    }

    fn close(&mut self) {
        if !self.points.is_empty() {
            self.contours.push(self.points.len());
        }
    }
}

fn point_to_segment_distance(px: f32, py: f32, x0: f32, y0: f32, x1: f32, y1: f32) -> f32 {
    let dx = x1 - x0;
    let dy = y1 - y0;
    let len_sq = dx * dx + dy * dy;

    if len_sq < 1e-6 {
        let dpx = px - x0;
        let dpy = py - y0;
        return (dpx * dpx + dpy * dpy).sqrt();
    }

    let t = ((px - x0) * dx + (py - y0) * dy) / len_sq;
    let t = t.clamp(0.0, 1.0);

    let closest_x = x0 + t * dx;
    let closest_y = y0 + t * dy;

    let dpx = px - closest_x;
    let dpy = py - closest_y;
    (dpx * dpx + dpy * dpy).sqrt()
}

impl Default for GlyphRasterizer {
    fn default() -> Self {
        Self::new()
    }
}
