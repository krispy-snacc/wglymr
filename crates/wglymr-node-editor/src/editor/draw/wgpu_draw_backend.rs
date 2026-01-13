use super::{DrawItem, DrawKind};
use wglymr_render_wgpu::{MsdfTextRenderer, PrimitiveRenderer, RoundedRect, SdfRenderer};
use wgpu::{Device, Queue, RenderPass};

enum DrawCommand {
    Line {
        start: [f32; 2],
        end: [f32; 2],
        color: wglymr_color::Color,
        depth: f32,
    },
    Rect {
        min: [f32; 2],
        max: [f32; 2],
        color: wglymr_color::Color,
        depth: f32,
    },
    RoundedRect {
        rect: RoundedRect,
    },
    Circle {
        center: [f32; 2],
        radius: f32,
        color: wglymr_color::Color,
        filled: bool,
        depth: f32,
    },
    Text {
        text: String,
        pos: [f32; 2],
        size: f32,
        color: wglymr_color::Color,
        depth: f32,
    },
}

pub struct WgpuDrawBackend {
    primitive_renderer: PrimitiveRenderer,
    sdf_renderer: SdfRenderer,
    text_renderer: MsdfTextRenderer,
    viewport: [f32; 2],
    pan: [f32; 2],
    zoom: f32,
    draw_commands: Vec<DrawCommand>,
}

impl WgpuDrawBackend {
    pub fn new(
        primitive_renderer: PrimitiveRenderer,
        sdf_renderer: SdfRenderer,
        text_renderer: MsdfTextRenderer,
    ) -> Self {
        Self {
            primitive_renderer,
            sdf_renderer,
            text_renderer,
            viewport: [800.0, 600.0],
            pan: [0.0, 0.0],
            zoom: 1.0,
            draw_commands: Vec::new(),
        }
    }

    pub fn set_viewport(&mut self, queue: &Queue, viewport: [f32; 2]) {
        self.viewport = viewport;
        self.primitive_renderer.set_viewport(queue, viewport);
        self.sdf_renderer.set_viewport(queue, viewport);
        self.text_renderer.set_viewport(queue, viewport);
    }

    pub fn set_camera(&mut self, pan: [f32; 2], zoom: f32) {
        self.pan = pan;
        self.zoom = zoom;
    }

    fn world_to_screen(&self, point: [f32; 2]) -> [f32; 2] {
        [
            (point[0] - self.pan[0]) * self.zoom + 0.5 * self.viewport[0],
            (point[1] - self.pan[1]) * self.zoom + 0.5 * self.viewport[1],
        ]
    }

    fn world_to_screen_size(&self, size: f32) -> f32 {
        size * self.zoom
    }

    pub fn begin_frame(&mut self) {
        self.primitive_renderer.begin_frame();
        self.sdf_renderer.begin_frame();
        self.text_renderer.begin_frame();
        self.draw_commands.clear();
    }

    pub fn emit(&mut self, item: &DrawItem) {
        match &item.kind {
            DrawKind::Line(line) => {
                let screen_start = self.world_to_screen(line.start);
                let screen_end = self.world_to_screen(line.end);
                self.draw_commands.push(DrawCommand::Line {
                    start: screen_start,
                    end: screen_end,
                    color: line.color,
                    depth: item.depth,
                });
            }
            DrawKind::Rect(rect) => {
                let screen_min = self.world_to_screen(rect.position);
                let screen_max = self.world_to_screen([
                    rect.position[0] + rect.size[0],
                    rect.position[1] + rect.size[1],
                ]);
                self.draw_commands.push(DrawCommand::Rect {
                    min: screen_min,
                    max: screen_max,
                    color: rect.color,
                    depth: item.depth,
                });
            }
            DrawKind::RoundedRect(rrect) => {
                let screen_min = self.world_to_screen(rrect.position);
                let screen_max = self.world_to_screen([
                    rrect.position[0] + rrect.size[0],
                    rrect.position[1] + rrect.size[1],
                ]);
                let screen_radius = self.world_to_screen_size(rrect.corner_radius);
                let rounded_rect = RoundedRect::new(screen_min, screen_max)
                    .with_radius(screen_radius)
                    .with_fill_color(rrect.color)
                    .with_depth(item.depth);
                self.draw_commands
                    .push(DrawCommand::RoundedRect { rect: rounded_rect });
            }
            DrawKind::Circle(circle) => {
                let screen_center = self.world_to_screen(circle.center);
                let screen_radius = self.world_to_screen_size(circle.radius);
                self.draw_commands.push(DrawCommand::Circle {
                    center: screen_center,
                    radius: screen_radius,
                    color: circle.color,
                    filled: circle.filled,
                    depth: item.depth,
                });
            }
            DrawKind::Glyph(glyph) => {
                let screen_pos = self.world_to_screen(glyph.world_position);
                let screen_font_size = self.world_to_screen_size(glyph.font_size).max(1.0);
                self.draw_commands.push(DrawCommand::Text {
                    text: glyph.text.clone(),
                    pos: screen_pos,
                    size: screen_font_size,
                    color: glyph.color,
                    depth: item.depth,
                });
            }
        }
    }

    pub fn flush(&mut self, device: &Device, queue: &Queue) {
        for cmd in &self.draw_commands {
            match cmd {
                DrawCommand::Line {
                    start,
                    end,
                    color,
                    depth,
                } => {
                    self.primitive_renderer
                        .draw_line(*start, *end, *color, *depth);
                }
                DrawCommand::Rect {
                    min,
                    max,
                    color,
                    depth,
                } => {
                    self.primitive_renderer
                        .draw_rect(*min, *max, *color, *depth);
                }
                DrawCommand::RoundedRect { rect } => {
                    self.sdf_renderer.draw_rounded_rect(rect);
                }
                DrawCommand::Circle {
                    center,
                    radius,
                    color,
                    filled,
                    depth,
                } => {
                    if *filled {
                        let rect = RoundedRect::new(
                            [center[0] - radius, center[1] - radius],
                            [center[0] + radius, center[1] + radius],
                        )
                        .with_radius(*radius)
                        .with_fill_color(*color)
                        .with_depth(*depth);
                        self.sdf_renderer.draw_rounded_rect(&rect);
                    }
                }
                DrawCommand::Text {
                    text,
                    pos,
                    size,
                    color,
                    depth,
                } => {
                    self.text_renderer
                        .draw_text(text, *pos, *size, *color, *depth);
                }
            }
        }

        self.primitive_renderer.upload(queue);
        self.sdf_renderer.upload(queue);
        self.text_renderer.upload(device, queue);
    }

    pub fn render<'a>(&'a mut self, pass: &mut RenderPass<'a>) {
        self.primitive_renderer.render_lines(pass);
        self.primitive_renderer.render_rects(pass);
        self.sdf_renderer.render(pass);
        self.text_renderer.render(pass);
    }

    pub fn primitive_renderer_mut(&mut self) -> &mut PrimitiveRenderer {
        &mut self.primitive_renderer
    }
}
