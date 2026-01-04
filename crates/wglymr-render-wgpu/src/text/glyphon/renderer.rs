use glyphon::{
    Attrs, Buffer, Cache, Color as GlyphonColor, Family, FontSystem, Metrics, Resolution, Shaping,
    SwashCache, TextArea, TextAtlas, TextBounds, TextRenderer, Viewport,
};
use wglymr_color::Color;
use wgpu::{Device, MultisampleState, Queue, RenderPass, TextureFormat};

const ROBOTO_FONT_DATA: &[u8] = include_bytes!("../../../../../fonts/Roboto-Regular.ttf");

pub struct GlyphonTextRenderer {
    font_system: FontSystem,
    text_atlas: TextAtlas,
    text_renderer: TextRenderer,
    cache: SwashCache,
    #[allow(dead_code)]
    glyphon_cache: Cache,
    viewport: Viewport,
    viewport_width: f32,
    viewport_height: f32,
    text_buffers: Vec<TextBufferEntry>,
    needs_prepare: bool,
}

struct TextBufferEntry {
    buffer: Buffer,
    position: [f32; 2],
    color: Color,
    layer: u8,
}

impl GlyphonTextRenderer {
    pub fn new(device: &Device, queue: &Queue, format: TextureFormat) -> Self {
        let mut font_system = FontSystem::new();

        font_system
            .db_mut()
            .load_font_data(ROBOTO_FONT_DATA.to_vec());

        let swash_cache = SwashCache::new();
        let cache = Cache::new(device);
        let mut text_atlas = TextAtlas::new(device, queue, &cache, format);

        let text_renderer =
            TextRenderer::new(&mut text_atlas, device, MultisampleState::default(), None);

        let viewport = Viewport::new(device, &cache);

        Self {
            font_system,
            text_atlas,
            text_renderer,
            cache: swash_cache,
            glyphon_cache: cache,
            viewport,
            viewport_width: 800.0,
            viewport_height: 600.0,
            text_buffers: Vec::new(),
            needs_prepare: false,
        }
    }

    pub fn begin_frame(&mut self) {
        self.text_buffers.clear();
        self.needs_prepare = false;
    }

    pub fn set_viewport(&mut self, queue: &Queue, viewport: [f32; 2]) {
        self.viewport_width = viewport[0];
        self.viewport_height = viewport[1];
        self.viewport.update(
            queue,
            Resolution {
                width: viewport[0] as u32,
                height: viewport[1] as u32,
            },
        );
    }

    pub fn draw_text(
        &mut self,
        text: &str,
        screen_position: [f32; 2],
        font_size_px: f32,
        color: Color,
        layer: u8,
    ) {
        let mut buffer = Buffer::new(
            &mut self.font_system,
            Metrics::new(font_size_px, font_size_px),
        );

        buffer.set_size(
            &mut self.font_system,
            Some(self.viewport_width),
            Some(self.viewport_height),
        );

        buffer.set_text(
            &mut self.font_system,
            text,
            &Attrs::new().family(Family::Name("Roboto")),
            Shaping::Advanced,
        );

        buffer.shape_until_scroll(&mut self.font_system, false);

        self.text_buffers.push(TextBufferEntry {
            buffer,
            position: screen_position,
            color,
            layer,
        });
        self.needs_prepare = true;
    }

    pub fn finish_batch(&mut self) {
        self.text_buffers.sort_by_key(|entry| entry.layer);
    }

    pub fn upload(&mut self, device: &Device, queue: &Queue) {
        if !self.needs_prepare || self.text_buffers.is_empty() {
            return;
        }

        let text_areas: Vec<TextArea> = self
            .text_buffers
            .iter()
            .map(|entry| {
                let [r, g, b, a] = entry.color.to_rgba_srgb();
                TextArea {
                    buffer: &entry.buffer,
                    left: entry.position[0],
                    top: entry.position[1],
                    scale: 1.0,
                    bounds: TextBounds {
                        left: 0,
                        top: 0,
                        right: self.viewport_width as i32,
                        bottom: self.viewport_height as i32,
                    },
                    default_color: GlyphonColor::rgba(
                        (r * 255.0) as u8,
                        (g * 255.0) as u8,
                        (b * 255.0) as u8,
                        (a * 255.0) as u8,
                    ),
                    custom_glyphs: &[],
                }
            })
            .collect();

        self.text_renderer
            .prepare(
                device,
                queue,
                &mut self.font_system,
                &mut self.text_atlas,
                &self.viewport,
                text_areas,
                &mut self.cache,
            )
            .expect("Failed to prepare text rendering");

        self.needs_prepare = false;
    }

    pub fn render<'a>(&'a mut self, render_pass: &mut RenderPass<'a>) {
        if self.text_buffers.is_empty() {
            return;
        }

        self.text_renderer
            .render(&self.text_atlas, &self.viewport, render_pass)
            .expect("Failed to render text");
    }
}
