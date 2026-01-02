use crate::editor::render_model::{RenderEdge, RenderNode};
use crate::editor::renderer::NodeEditorRenderer;
use crate::editor::text::{FontConfig, RenderText, TextLayout, TextLayoutCache, TextStyle};
use crate::engine::EditorView;

use wglymr_render_wgpu::{GpuGlyph, TextRenderer};

/// Render layer constants for z-ordering.
/// Lower values are rendered first (behind).
pub mod layers {
    pub const GRID: u8 = 0;
    pub const EDGES: u8 = 1;
    pub const NODES: u8 = 2;
    pub const NODE_TEXT: u8 = 3;
    pub const WIDGETS: u8 = 4;
}

fn world_to_screen(point: [f32; 2], view: &EditorView) -> [f32; 2] {
    let pan = view.pan();
    let zoom = view.zoom();
    let width = view.width() as f32;
    let height = view.height() as f32;

    [
        (point[0] - pan[0]) * zoom + width / 2.0,
        (point[1] - pan[1]) * zoom + height / 2.0,
    ]
}

fn _world_to_screen_size(size: f32, view: &EditorView) -> f32 {
    size * view.zoom()
}

pub struct WgpuNodeEditorRenderer<'a> {
    primitive_renderer: &'a mut wglymr_render_wgpu::PrimitiveRenderer,
}

impl<'a> WgpuNodeEditorRenderer<'a> {
    pub fn new(primitive_renderer: &'a mut wglymr_render_wgpu::PrimitiveRenderer) -> Self {
        Self { primitive_renderer }
    }
}

impl<'a> NodeEditorRenderer for WgpuNodeEditorRenderer<'a> {
    fn draw_node(&mut self, node: &RenderNode, view: &EditorView) {
        let screen_min = world_to_screen(node.bounds.min, view);
        let screen_max = world_to_screen(node.bounds.max, view);
        let color = if node.selected {
            [0.3, 0.3, 0.5, 1.0]
        } else {
            [0.2, 0.2, 0.3, 1.0]
        };
        self.primitive_renderer.draw_rect(screen_min, screen_max, color);
    }

    fn draw_edge(&mut self, edge: &RenderEdge, view: &EditorView) {
        let screen_from = world_to_screen(edge.from, view);
        let screen_to = world_to_screen(edge.to, view);
        let color = [0.8, 0.8, 0.8, 1.0];
        self.primitive_renderer.draw_line(screen_from, screen_to, color);
    }
}

/// Text rendering context for the node editor.
/// Handles layout, caching, and submission to the GPU text renderer.
pub struct NodeEditorTextRenderer {
    layout: TextLayout,
    cache: TextLayoutCache,
    default_style: TextStyle,
}

impl NodeEditorTextRenderer {
    pub fn new() -> Self {
        Self {
            layout: TextLayout::new(FontConfig::default()),
            cache: TextLayoutCache::default(),
            default_style: TextStyle {
                font_size: 14.0,
                color: [1.0, 1.0, 1.0, 1.0],
                line_height: None,
            },
        }
    }

    pub fn with_font_size(font_size: f32) -> Self {
        Self {
            layout: TextLayout::new(FontConfig { default_size: font_size }),
            cache: TextLayoutCache::default(),
            default_style: TextStyle {
                font_size,
                color: [1.0, 1.0, 1.0, 1.0],
                line_height: None,
            },
        }
    }

    /// Layout text in world space and cache the result.
    pub fn layout_text(&mut self, text: &str, style: &TextStyle) -> RenderText {
        self.cache.get_or_layout(&self.layout, text, style)
    }

    /// Layout text at a specific world position.
    pub fn layout_text_at(&mut self, text: &str, position: [f32; 2], style: &TextStyle) -> RenderText {
        self.cache.get_or_layout_at(&self.layout, text, position, style)
    }

    /// Submit render text to GPU, transforming from world to screen space.
    pub fn submit_text(
        &mut self,
        render_text: &RenderText,
        view: &EditorView,
        text_renderer: &mut TextRenderer,
        queue: &wgpu::Queue,
        layer: u8,
    ) {
        let zoom = view.zoom();

        for glyph in &render_text.glyphs {
            // Transform world position to screen position
            let screen_pos = world_to_screen(glyph.position, view);
            let screen_size = [
                glyph.size[0] * zoom,
                glyph.size[1] * zoom,
            ];

            // Rasterize glyph if needed and get UV coordinates
            let screen_font_size = render_text.style.font_size * zoom;
            if let Some(entry) = self.layout.rasterize_glyph(
                glyph.glyph_id,
                screen_font_size,
                text_renderer.atlas_mut(),
                queue,
            ) {
                text_renderer.draw_glyph(GpuGlyph {
                    screen_pos,
                    size: screen_size,
                    uv_min: entry.uv_min,
                    uv_max: entry.uv_max,
                    color: render_text.style.color,
                    layer,
                });
            }
        }
    }

    /// Render a node's title text.
    pub fn draw_node_title(
        &mut self,
        node: &RenderNode,
        view: &EditorView,
        text_renderer: &mut TextRenderer,
        queue: &wgpu::Queue,
    ) {
        if node.title.is_empty() {
            return;
        }

        let style = TextStyle {
            font_size: 14.0,
            color: [1.0, 1.0, 1.0, 1.0],
            line_height: None,
        };

        // Position text at title bounds (world space)
        let title_pos = [
            node.title_bounds.min[0] + 4.0, // Small padding
            node.title_bounds.min[1] + 2.0,
        ];

        let render_text = self.layout_text_at(&node.title, title_pos, &style);
        self.submit_text(&render_text, view, text_renderer, queue, layers::NODE_TEXT);
    }

    /// Clear the layout cache.
    pub fn clear_cache(&mut self) {
        self.cache.clear();
    }

    pub fn default_style(&self) -> &TextStyle {
        &self.default_style
    }
}

impl Default for NodeEditorTextRenderer {
    fn default() -> Self {
        Self::new()
    }
}
