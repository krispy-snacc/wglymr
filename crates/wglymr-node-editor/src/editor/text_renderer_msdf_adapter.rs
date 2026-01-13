use crate::editor::renderer::NodeEditorTextRenderer;
use crate::editor::text::GlyphRun;
use crate::editor::wgpu_renderer::{world_to_screen, world_to_screen_size};
use crate::engine::{EditorView, GlobalInteractionState};
use wglymr_render_wgpu::MsdfTextRenderer;

fn get_drag_offset(
    node_id: Option<crate::document::commands::NodeId>,
    global: &GlobalInteractionState,
) -> [f32; 2] {
    if let Some(nid) = node_id {
        if let Some(drag) = &global.node_drag {
            if drag.node_ids.contains(&nid) {
                return drag.drag_delta;
            }
        }
    }
    [0.0, 0.0]
}

impl NodeEditorTextRenderer for MsdfTextRenderer {
    fn begin_frame(&mut self) {
        self.begin_frame();
    }

    fn draw_runs(&mut self, view: &EditorView, global: &GlobalInteractionState, runs: &[GlyphRun]) {
        for run in runs {
            let drag_offset = get_drag_offset(run.node_id, global);
            let world_pos = [
                run.world_position[0] + drag_offset[0],
                run.world_position[1] + drag_offset[1],
            ];
            let screen_pos = world_to_screen(world_pos, view);
            let font_size = world_to_screen_size(run.font_size, view).max(1.0);
            self.draw_text(&run.text, screen_pos, font_size, run.color);
        }
    }

    fn draw_text_immediate(
        &mut self,
        text: &str,
        screen_position: [f32; 2],
        font_size_px: f32,
        color: wglymr_color::Color,
    ) {
        self.draw_text(text, screen_position, font_size_px, color);
    }

    fn upload(&mut self, device: &wgpu::Device, queue: &wgpu::Queue) {
        self.upload(device, queue);
    }

    fn render<'a>(&'a mut self, render_pass: &mut wgpu::RenderPass<'a>) {
        self.render(render_pass);
    }
}
