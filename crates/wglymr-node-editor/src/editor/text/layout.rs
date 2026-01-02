use wglymr_render_wgpu::{GlyphAtlas, GlyphKey};
use wgpu::Queue;

use super::model::{RenderText, ShapedGlyph, TextBounds, TextStyle};

#[derive(Debug, Clone, Copy)]
pub struct FontMetrics {
    pub ascent: f32,
    pub descent: f32,
    pub line_height: f32,
}

impl Default for FontMetrics {
    fn default() -> Self {
        Self {
            ascent: 12.0,
            descent: 3.0,
            line_height: 18.0,
        }
    }
}

impl FontMetrics {
    pub fn for_size(font_size: f32) -> Self {
        let scale = font_size / 14.0;
        Self {
            ascent: 11.0 * scale,
            descent: 3.0 * scale,
            line_height: 16.0 * scale,
        }
    }
}

#[derive(Debug, Clone)]
pub struct FontConfig {
    pub default_size: f32,
}

impl Default for FontConfig {
    fn default() -> Self {
        Self { default_size: 14.0 }
    }
}

const GLYPH_WIDTH_FACTOR: f32 = 0.6;
const GLYPH_HEIGHT_FACTOR: f32 = 1.0;

pub struct TextShaper {
    _config: FontConfig,
}

impl TextShaper {
    pub fn new(config: FontConfig) -> Self {
        Self { _config: config }
    }

    pub fn shape(&self, text: &str, style: &TextStyle) -> RenderText {
        let font_size = style.font_size;
        let metrics = FontMetrics::for_size(font_size);
        let line_height = style.line_height.unwrap_or(metrics.line_height);

        let glyph_width = font_size * GLYPH_WIDTH_FACTOR;
        let glyph_height = font_size * GLYPH_HEIGHT_FACTOR;

        let mut glyphs = Vec::with_capacity(text.len());
        let mut x = 0.0f32;
        let mut y = metrics.ascent;
        let mut max_x = 0.0f32;
        let mut max_y = y;

        for ch in text.chars() {
            if ch == '\n' {
                max_x = max_x.max(x);
                x = 0.0;
                y += line_height;
                max_y = y;
                continue;
            }

            if ch == ' ' {
                x += glyph_width;
                continue;
            }

            if ch.is_control() {
                continue;
            }

            let glyph_id = ch as u32;

            glyphs.push(ShapedGlyph {
                glyph_id,
                position: [x, y - metrics.ascent],
                size: [glyph_width, glyph_height],
                uv_min: [0.0, 0.0],
                uv_max: [1.0, 1.0],
            });

            x += glyph_width;
        }

        max_x = max_x.max(x);
        max_y = max_y.max(y + metrics.descent);

        let bounds = TextBounds::new([0.0, 0.0], [max_x, max_y]);

        RenderText::new(glyphs, bounds, *style, metrics)
    }
}

pub struct TextLayout {
    shaper: TextShaper,
}

impl TextLayout {
    pub fn new(config: FontConfig) -> Self {
        Self {
            shaper: TextShaper::new(config),
        }
    }

    pub fn layout_text(&self, text: &str, style: &TextStyle) -> RenderText {
        self.shaper.shape(text, style)
    }

    pub fn layout_text_at(&self, text: &str, position: [f32; 2], style: &TextStyle) -> RenderText {
        let render_text = self.shaper.shape(text, style);
        render_text.offset(position)
    }

    pub fn measure_text(&self, text: &str, style: &TextStyle) -> TextBounds {
        self.shaper.shape(text, style).bounds
    }

    pub fn rasterize_glyph(
        &self,
        glyph_id: u32,
        font_size: f32,
        atlas: &mut GlyphAtlas,
        queue: &Queue,
    ) -> Option<wglymr_render_wgpu::GlyphEntry> {
        let key = GlyphKey {
            id: glyph_id,
            size_px: (font_size * 2.0) as u16,
        };

        if let Some(entry) = atlas.get(&key) {
            return Some(*entry);
        }

        let width = (font_size * GLYPH_WIDTH_FACTOR * 2.0).ceil() as u16;
        let height = (font_size * GLYPH_HEIGHT_FACTOR * 2.0).ceil() as u16;

        let ch = char::from_u32(glyph_id).unwrap_or('?');
        let data = rasterize_char(ch, width, height);

        atlas.insert(queue, key, width, height, &data)
    }
}

fn rasterize_char(ch: char, width: u16, height: u16) -> Vec<u8> {
    let w = width as usize;
    let h = height as usize;
    let mut data = vec![0u8; w * h];

    let padding_x = (w as f32 * 0.15) as usize;
    let padding_y = (h as f32 * 0.15) as usize;
    let inner_w = w.saturating_sub(padding_x * 2);
    let inner_h = h.saturating_sub(padding_y * 2);

    if inner_w == 0 || inner_h == 0 {
        return data;
    }

    match ch {
        'A'..='Z' | 'a'..='z' => {
            rasterize_letter(ch, &mut data, w, h, padding_x, padding_y, inner_w, inner_h);
        }
        '0'..='9' => {
            rasterize_digit(ch, &mut data, w, h, padding_x, padding_y, inner_w, inner_h);
        }
        '.' => {
            let cx = w / 2;
            let cy = h - padding_y - inner_h / 8;
            let r = (inner_w.min(inner_h) / 8).max(1);
            fill_circle(&mut data, w, h, cx, cy, r);
        }
        ',' => {
            let cx = w / 2;
            let cy = h - padding_y;
            let r = (inner_w.min(inner_h) / 8).max(1);
            fill_circle(&mut data, w, h, cx, cy - r, r);
        }
        ':' => {
            let cx = w / 2;
            let r = (inner_w.min(inner_h) / 8).max(1);
            let top_y = padding_y + inner_h / 3;
            let bot_y = padding_y + inner_h * 2 / 3;
            fill_circle(&mut data, w, h, cx, top_y, r);
            fill_circle(&mut data, w, h, cx, bot_y, r);
        }
        '-' => {
            let y = h / 2;
            let x1 = padding_x;
            let x2 = w - padding_x;
            let thickness = (inner_h / 6).max(1);
            for dy in 0..thickness {
                for x in x1..x2 {
                    let idx = (y + dy) * w + x;
                    if idx < data.len() {
                        data[idx] = 255;
                    }
                }
            }
        }
        '+' => {
            let cx = w / 2;
            let cy = h / 2;
            let thickness = (inner_h / 6).max(1);
            let arm = inner_w.min(inner_h) / 2;
            for d in 0..arm {
                for _t in 0..thickness {
                    if cx + d < w {
                        data[cy * w + cx + d] = 255;
                    }
                    if cx >= d {
                        data[cy * w + cx - d] = 255;
                    }
                    if cy + d < h {
                        data[(cy + d) * w + cx] = 255;
                    }
                    if cy >= d {
                        data[(cy - d) * w + cx] = 255;
                    }
                }
            }
        }
        '_' => {
            let y = h - padding_y - 1;
            let x1 = padding_x;
            let x2 = w - padding_x;
            let thickness = (inner_h / 6).max(1);
            for dy in 0..thickness {
                for x in x1..x2 {
                    let idx = (y.saturating_sub(dy)) * w + x;
                    if idx < data.len() {
                        data[idx] = 255;
                    }
                }
            }
        }
        '(' | ')' => {
            let is_left = ch == '(';
            for iy in 0..inner_h {
                let t = iy as f32 / inner_h as f32;
                let curve = (t * std::f32::consts::PI).sin();
                let offset = (curve * (inner_w as f32 * 0.3)) as usize;
                let x = if is_left {
                    padding_x + inner_w / 3 + offset
                } else {
                    w - padding_x - inner_w / 3 - offset
                };
                let y = padding_y + iy;
                if x < w && y < h {
                    data[y * w + x] = 255;
                    if x + 1 < w {
                        data[y * w + x + 1] = 200;
                    }
                }
            }
        }
        '[' | ']' => {
            let is_left = ch == '[';
            let x_base = if is_left {
                padding_x + inner_w / 4
            } else {
                w - padding_x - inner_w / 4
            };
            for y in padding_y..(h - padding_y) {
                if x_base < w {
                    data[y * w + x_base] = 255;
                }
            }
            let bar_len = inner_w / 3;
            for dx in 0..bar_len {
                let x = if is_left {
                    x_base + dx
                } else {
                    x_base.saturating_sub(dx)
                };
                if x < w {
                    data[padding_y * w + x] = 255;
                    data[(h - padding_y - 1) * w + x] = 255;
                }
            }
        }
        '/' => {
            for iy in 0..inner_h {
                let x = padding_x + (inner_w * (inner_h - 1 - iy)) / inner_h.max(1);
                let y = padding_y + iy;
                if x < w && y < h {
                    data[y * w + x] = 255;
                }
            }
        }
        '\\' => {
            for iy in 0..inner_h {
                let x = padding_x + (inner_w * iy) / inner_h.max(1);
                let y = padding_y + iy;
                if x < w && y < h {
                    data[y * w + x] = 255;
                }
            }
        }
        _ => {
            for y in padding_y..(h - padding_y) {
                for x in padding_x..(w - padding_x) {
                    let on_border = y == padding_y
                        || y == h - padding_y - 1
                        || x == padding_x
                        || x == w - padding_x - 1;
                    if on_border {
                        data[y * w + x] = 255;
                    }
                }
            }
        }
    }

    data
}

fn rasterize_letter(
    ch: char,
    data: &mut [u8],
    w: usize,
    h: usize,
    px: usize,
    py: usize,
    iw: usize,
    ih: usize,
) {
    let upper = ch.to_ascii_uppercase();
    match upper {
        'O' | 'Q' | 'C' | 'G' | 'D' => {
            let cx = w / 2;
            let cy = h / 2;
            let rx = iw / 2;
            let ry = ih / 2;
            draw_ellipse(data, w, h, cx, cy, rx, ry);
            if upper == 'Q' {
                for i in 0..(iw / 3) {
                    let x = cx + rx / 2 + i;
                    let y = cy + ry / 2 + i;
                    if x < w && y < h {
                        data[y * w + x] = 255;
                    }
                }
            }
            if upper == 'G' {
                for x in (cx)..(w - px) {
                    data[cy * w + x] = 255;
                }
            }
            if upper == 'D' {
                for y in py..(h - py) {
                    data[y * w + px] = 255;
                }
            }
        }
        'I' | 'L' | 'T' | 'J' | 'F' | 'E' => {
            let cx = w / 2;
            match upper {
                'I' => {
                    for y in py..(h - py) {
                        data[y * w + cx] = 255;
                    }
                    for x in (cx - iw / 4)..(cx + iw / 4) {
                        data[py * w + x] = 255;
                        data[(h - py - 1) * w + x] = 255;
                    }
                }
                'L' => {
                    for y in py..(h - py) {
                        data[y * w + px] = 255;
                    }
                    for x in px..(w - px) {
                        data[(h - py - 1) * w + x] = 255;
                    }
                }
                'T' => {
                    for y in py..(h - py) {
                        data[y * w + cx] = 255;
                    }
                    for x in px..(w - px) {
                        data[py * w + x] = 255;
                    }
                }
                'J' => {
                    let right_x = w - px - 1;
                    for y in py..(h - py - ih / 3) {
                        data[y * w + right_x] = 255;
                    }
                    let cx_arc = cx;
                    let cy_arc = h - py - ih / 3;
                    for angle in 0..180 {
                        let rad = (angle as f32).to_radians();
                        let x = (cx_arc as f32 + rad.cos() * (iw / 3) as f32) as usize;
                        let y = (cy_arc as f32 + rad.sin() * (ih / 3) as f32) as usize;
                        if x < w && y < h {
                            data[y * w + x] = 255;
                        }
                    }
                }
                'F' => {
                    for y in py..(h - py) {
                        data[y * w + px] = 255;
                    }
                    for x in px..(w - px) {
                        data[py * w + x] = 255;
                    }
                    for x in px..(w - px - iw / 4) {
                        data[(h / 2) * w + x] = 255;
                    }
                }
                'E' => {
                    for y in py..(h - py) {
                        data[y * w + px] = 255;
                    }
                    for x in px..(w - px) {
                        data[py * w + x] = 255;
                        data[(h - py - 1) * w + x] = 255;
                    }
                    for x in px..(w - px - iw / 4) {
                        data[(h / 2) * w + x] = 255;
                    }
                }
                _ => {}
            }
        }
        'H' | 'N' | 'M' | 'W' | 'V' | 'A' | 'K' | 'X' | 'Y' | 'Z' => match upper {
            'H' => {
                for y in py..(h - py) {
                    data[y * w + px] = 255;
                    data[y * w + (w - px - 1)] = 255;
                }
                for x in px..(w - px) {
                    data[(h / 2) * w + x] = 255;
                }
            }
            'N' => {
                for y in py..(h - py) {
                    data[y * w + px] = 255;
                    data[y * w + (w - px - 1)] = 255;
                }
                for i in 0..ih {
                    let x = px + (i * iw) / ih;
                    let y = py + i;
                    if x < w && y < h {
                        data[y * w + x] = 255;
                    }
                }
            }
            'M' => {
                for y in py..(h - py) {
                    data[y * w + px] = 255;
                    data[y * w + (w - px - 1)] = 255;
                }
                let cx = w / 2;
                for i in 0..(ih / 2) {
                    let x_left = px + (i * (cx - px)) / (ih / 2).max(1);
                    let x_right = w - px - 1 - (i * (cx - px)) / (ih / 2).max(1);
                    let y = py + i;
                    if x_left < w && y < h {
                        data[y * w + x_left] = 255;
                    }
                    if x_right < w && y < h {
                        data[y * w + x_right] = 255;
                    }
                }
            }
            'W' => {
                for y in py..(h - py) {
                    data[y * w + px] = 255;
                    data[y * w + (w - px - 1)] = 255;
                }
                let cx = w / 2;
                for i in 0..(ih / 2) {
                    let x_left = px + (i * (cx - px)) / (ih / 2).max(1);
                    let x_right = w - px - 1 - (i * (cx - px)) / (ih / 2).max(1);
                    let y = h - py - 1 - i;
                    if x_left < w && y < h {
                        data[y * w + x_left] = 255;
                    }
                    if x_right < w && y < h {
                        data[y * w + x_right] = 255;
                    }
                }
            }
            'V' => {
                let cx = w / 2;
                for i in 0..ih {
                    let spread = ((ih - i) * iw / 2) / ih;
                    let x_left = cx.saturating_sub(spread);
                    let x_right = (cx + spread).min(w - 1);
                    let y = py + i;
                    if y < h {
                        data[y * w + x_left] = 255;
                        data[y * w + x_right] = 255;
                    }
                }
            }
            'A' => {
                let cx = w / 2;
                for i in 0..ih {
                    let spread = (i * iw / 2) / ih;
                    let x_left = cx.saturating_sub(spread);
                    let x_right = (cx + spread).min(w - 1);
                    let y = py + i;
                    if y < h {
                        data[y * w + x_left] = 255;
                        data[y * w + x_right] = 255;
                    }
                }
                let bar_y = py + ih * 2 / 3;
                let bar_spread = (ih * 2 / 3 * iw / 2) / ih;
                for x in (cx.saturating_sub(bar_spread))..(cx + bar_spread).min(w) {
                    data[bar_y * w + x] = 255;
                }
            }
            'K' => {
                for y in py..(h - py) {
                    data[y * w + px] = 255;
                }
                let cy = h / 2;
                for i in 0..(ih / 2) {
                    let x = px + iw / 3 + i;
                    let y_up = cy - i;
                    let y_down = cy + i;
                    if x < w {
                        if y_up >= py {
                            data[y_up * w + x] = 255;
                        }
                        if y_down < h - py {
                            data[y_down * w + x] = 255;
                        }
                    }
                }
            }
            'X' => {
                for i in 0..ih {
                    let x1 = px + (i * iw) / ih;
                    let x2 = w - px - 1 - (i * iw) / ih;
                    let y = py + i;
                    if y < h {
                        if x1 < w {
                            data[y * w + x1] = 255;
                        }
                        if x2 < w {
                            data[y * w + x2] = 255;
                        }
                    }
                }
            }
            'Y' => {
                let cx = w / 2;
                let cy = h / 2;
                for i in 0..(ih / 2) {
                    let spread = ((ih / 2 - i) * iw / 2) / (ih / 2).max(1);
                    let x_left = cx.saturating_sub(spread);
                    let x_right = (cx + spread).min(w - 1);
                    let y = py + i;
                    if y < h {
                        data[y * w + x_left] = 255;
                        data[y * w + x_right] = 255;
                    }
                }
                for y in cy..(h - py) {
                    data[y * w + cx] = 255;
                }
            }
            'Z' => {
                for x in px..(w - px) {
                    data[py * w + x] = 255;
                    data[(h - py - 1) * w + x] = 255;
                }
                for i in 0..ih {
                    let x = w - px - 1 - (i * iw) / ih;
                    let y = py + i;
                    if x < w && y < h {
                        data[y * w + x] = 255;
                    }
                }
            }
            _ => {}
        },
        'B' | 'P' | 'R' | 'S' | 'U' => match upper {
            'B' => {
                for y in py..(h - py) {
                    data[y * w + px] = 255;
                }
                let rx = iw / 3;
                let ry = ih / 4;
                draw_ellipse(data, w, h, px + rx, py + ry, rx, ry);
                draw_ellipse(data, w, h, px + rx, h - py - ry, rx, ry);
            }
            'P' => {
                for y in py..(h - py) {
                    data[y * w + px] = 255;
                }
                let rx = iw / 3;
                let ry = ih / 4;
                draw_ellipse(data, w, h, px + rx, py + ry + ry / 2, rx, ry);
            }
            'R' => {
                for y in py..(h - py) {
                    data[y * w + px] = 255;
                }
                let rx = iw / 3;
                let ry = ih / 4;
                draw_ellipse(data, w, h, px + rx, py + ry + ry / 2, rx, ry);
                let mid_y = py + ih / 2;
                for i in 0..(ih / 2) {
                    let x = px + iw / 3 + (i * iw / 2) / (ih / 2).max(1);
                    let y = mid_y + i;
                    if x < w && y < h {
                        data[y * w + x] = 255;
                    }
                }
            }
            'S' => {
                let rx = iw / 2;
                let ry = ih / 4;
                for angle in 90..270 {
                    let rad = (angle as f32).to_radians();
                    let x = (w / 2) as f32 + rad.cos() * rx as f32;
                    let y = (py + ry) as f32 + rad.sin() * ry as f32;
                    if x >= 0.0 && (x as usize) < w && (y as usize) < h {
                        data[y as usize * w + x as usize] = 255;
                    }
                }
                for angle in 270..360 {
                    let rad = (angle as f32).to_radians();
                    let x = (w / 2) as f32 + rad.cos() * rx as f32;
                    let y = (h - py - ry) as f32 + rad.sin() * ry as f32;
                    if x >= 0.0 && (x as usize) < w && (y as usize) < h {
                        data[y as usize * w + x as usize] = 255;
                    }
                }
                for angle in 0..90 {
                    let rad = (angle as f32).to_radians();
                    let x = (w / 2) as f32 + rad.cos() * rx as f32;
                    let y = (h - py - ry) as f32 + rad.sin() * ry as f32;
                    if x >= 0.0 && (x as usize) < w && (y as usize) < h {
                        data[y as usize * w + x as usize] = 255;
                    }
                }
            }
            'U' => {
                for y in py..(h - py - ih / 3) {
                    data[y * w + px] = 255;
                    data[y * w + (w - px - 1)] = 255;
                }
                let cx = w / 2;
                let cy = h - py - ih / 3;
                for angle in 0..180 {
                    let rad = (angle as f32).to_radians();
                    let x = cx as f32 + rad.cos() * (iw / 2) as f32;
                    let y = cy as f32 + rad.sin() * (ih / 3) as f32;
                    if x >= 0.0 && (x as usize) < w && (y as usize) < h {
                        data[y as usize * w + x as usize] = 255;
                    }
                }
            }
            _ => {}
        },
        _ => {
            for y in py..(h - py) {
                for x in px..(w - px) {
                    let on_border = y == py || y == h - py - 1 || x == px || x == w - px - 1;
                    if on_border {
                        data[y * w + x] = 255;
                    }
                }
            }
        }
    }
}

fn rasterize_digit(
    ch: char,
    data: &mut [u8],
    w: usize,
    h: usize,
    px: usize,
    py: usize,
    iw: usize,
    ih: usize,
) {
    let cx = w / 2;
    let cy = h / 2;

    match ch {
        '0' => {
            draw_ellipse(data, w, h, cx, cy, iw / 2, ih / 2);
        }
        '1' => {
            for y in py..(h - py) {
                data[y * w + cx] = 255;
            }
            for i in 0..(ih / 4) {
                let x = cx - i;
                let y = py + i;
                if x > 0 && y < h {
                    data[y * w + x] = 255;
                }
            }
            for x in (cx - iw / 4)..(cx + iw / 4) {
                data[(h - py - 1) * w + x] = 255;
            }
        }
        '2' => {
            let ry = ih / 4;
            for angle in 180..360 {
                let rad = (angle as f32).to_radians();
                let x = cx as f32 + rad.cos() * (iw / 2) as f32;
                let y = (py + ry) as f32 + rad.sin() * ry as f32;
                if x >= 0.0 && (x as usize) < w && (y as usize) < h {
                    data[y as usize * w + x as usize] = 255;
                }
            }
            for i in 0..(ih / 2) {
                let x = w - px - 1 - (i * iw) / (ih / 2).max(1);
                let y = py + ih / 2 + i;
                if x < w && y < h {
                    data[y * w + x] = 255;
                }
            }
            for x in px..(w - px) {
                data[(h - py - 1) * w + x] = 255;
            }
        }
        '3' => {
            let ry = ih / 4;
            for angle in 180..360 {
                let rad = (angle as f32).to_radians();
                let x = cx as f32 + rad.cos() * (iw / 2) as f32;
                let y = (py + ry) as f32 + rad.sin() * ry as f32;
                if x >= 0.0 && (x as usize) < w && (y as usize) < h {
                    data[y as usize * w + x as usize] = 255;
                }
            }
            for angle in 0..180 {
                let rad = (angle as f32).to_radians();
                let x = cx as f32 + rad.cos() * (iw / 2) as f32;
                let y = (h - py - ry) as f32 + rad.sin() * ry as f32;
                if x >= 0.0 && (x as usize) < w && (y as usize) < h {
                    data[y as usize * w + x as usize] = 255;
                }
            }
        }
        '4' => {
            let cross_y = py + ih * 2 / 3;
            for y in py..cross_y {
                data[y * w + px] = 255;
            }
            for x in px..(w - px) {
                data[cross_y * w + x] = 255;
            }
            for y in py..(h - py) {
                data[y * w + (w - px - iw / 3)] = 255;
            }
        }
        '5' => {
            for x in px..(w - px) {
                data[py * w + x] = 255;
            }
            for y in py..cy {
                data[y * w + px] = 255;
            }
            for x in px..(w - px - iw / 4) {
                data[cy * w + x] = 255;
            }
            let ry = ih / 4;
            for angle in 270..360 {
                let rad = (angle as f32).to_radians();
                let x = cx as f32 + rad.cos() * (iw / 2) as f32;
                let y = (h - py - ry) as f32 + rad.sin() * ry as f32;
                if x >= 0.0 && (x as usize) < w && (y as usize) < h {
                    data[y as usize * w + x as usize] = 255;
                }
            }
            for angle in 0..90 {
                let rad = (angle as f32).to_radians();
                let x = cx as f32 + rad.cos() * (iw / 2) as f32;
                let y = (h - py - ry) as f32 + rad.sin() * ry as f32;
                if x >= 0.0 && (x as usize) < w && (y as usize) < h {
                    data[y as usize * w + x as usize] = 255;
                }
            }
        }
        '6' => {
            draw_ellipse(data, w, h, cx, h - py - ih / 3, iw / 2, ih / 3);
            for y in (py + ih / 4)..(h - py - ih / 3) {
                data[y * w + px] = 255;
            }
            let ry = ih / 4;
            for angle in 180..270 {
                let rad = (angle as f32).to_radians();
                let x = cx as f32 + rad.cos() * (iw / 2) as f32;
                let y = (py + ry) as f32 + rad.sin() * ry as f32;
                if x >= 0.0 && (x as usize) < w && (y as usize) < h {
                    data[y as usize * w + x as usize] = 255;
                }
            }
        }
        '7' => {
            for x in px..(w - px) {
                data[py * w + x] = 255;
            }
            for i in 0..ih {
                let x = w - px - 1 - (i * iw / 3) / ih;
                let y = py + i;
                if x < w && y < h {
                    data[y * w + x] = 255;
                }
            }
        }
        '8' => {
            let ry = ih / 4;
            draw_ellipse(data, w, h, cx, py + ry, iw / 2, ry);
            draw_ellipse(data, w, h, cx, h - py - ry, iw / 2, ry);
        }
        '9' => {
            draw_ellipse(data, w, h, cx, py + ih / 3, iw / 2, ih / 3);
            for y in (py + ih / 3)..(h - py - ih / 4) {
                data[y * w + (w - px - 1)] = 255;
            }
            let ry = ih / 4;
            for angle in 0..90 {
                let rad = (angle as f32).to_radians();
                let x = cx as f32 + rad.cos() * (iw / 2) as f32;
                let y = (h - py - ry) as f32 + rad.sin() * ry as f32;
                if x >= 0.0 && (x as usize) < w && (y as usize) < h {
                    data[y as usize * w + x as usize] = 255;
                }
            }
        }
        _ => {}
    }
}

fn draw_ellipse(data: &mut [u8], w: usize, h: usize, cx: usize, cy: usize, rx: usize, ry: usize) {
    for angle in 0..360 {
        let rad = (angle as f32).to_radians();
        let x = (cx as f32 + rad.cos() * rx as f32) as usize;
        let y = (cy as f32 + rad.sin() * ry as f32) as usize;
        if x < w && y < h {
            data[y * w + x] = 255;
        }
    }
}

fn fill_circle(data: &mut [u8], w: usize, h: usize, cx: usize, cy: usize, r: usize) {
    for dy in 0..=r {
        for dx in 0..=r {
            if dx * dx + dy * dy <= r * r {
                let positions = [
                    (cx + dx, cy + dy),
                    (cx.saturating_sub(dx), cy + dy),
                    (cx + dx, cy.saturating_sub(dy)),
                    (cx.saturating_sub(dx), cy.saturating_sub(dy)),
                ];
                for (x, y) in positions {
                    if x < w && y < h {
                        data[y * w + x] = 255;
                    }
                }
            }
        }
    }
}
