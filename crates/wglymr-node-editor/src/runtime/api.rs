use crate::editor::wgpu_renderer::world_to_screen;

use super::EditorRuntime;
use super::errors::RuntimeError;

impl EditorRuntime {
    pub fn init_engine(&mut self) -> Result<(), RuntimeError> {
        logging::log("Engine initialized");
        Ok(())
    }

    pub fn init_gpu(&mut self, gpu: super::gpu::GpuContext) -> Result<(), RuntimeError> {
        if self.gpu.is_some() {
            logging::log("GPU already initialized");
            return Ok(());
        }

        logging::log("GPU initialized");
        self.gpu = Some(gpu);
        Ok(())
    }

    pub fn create_view(&mut self, id: &str) -> Result<(), RuntimeError> {
        logging::log(&format!("Creating view: {}", id));
        self.views.create_view(id)?;
        Ok(())
    }

    pub fn destroy_view(&mut self, id: &str) -> Result<(), RuntimeError> {
        logging::log(&format!("Destroying view: {}", id));
        self.views.destroy_view(id)?;
        Ok(())
    }

    pub fn attach_view(
        &mut self,
        id: &str,
        surface: super::gpu::SurfaceHandle,
        width: u32,
        height: u32,
    ) -> Result<(), RuntimeError> {
        logging::log(&format!("Attaching view: {} ({}x{})", id, width, height));

        let gpu = self.gpu.as_ref().ok_or(RuntimeError::GpuNotInitialized)?;

        self.views.attach_view(id, surface, width, height, gpu)?;
        Ok(())
    }

    pub fn detach_view(&mut self, id: &str) -> Result<(), RuntimeError> {
        logging::log(&format!("Detaching view: {}", id));
        self.views.detach_view(id)?;
        Ok(())
    }

    pub fn resize_view(&mut self, id: &str, width: u32, height: u32) -> Result<(), RuntimeError> {
        logging::log(&format!("Resizing view {}: {}x{}", id, width, height));

        let gpu = self.gpu.as_ref().ok_or(RuntimeError::GpuNotInitialized)?;

        self.views.resize_view(id, width, height, gpu)?;
        Ok(())
    }

    pub fn set_view_camera(
        &mut self,
        id: &str,
        x: f32,
        y: f32,
        zoom: f32,
    ) -> Result<(), RuntimeError> {
        logging::log(&format!(
            "Setting camera for view {}: ({}, {}) @ {}",
            id, x, y, zoom
        ));
        self.views.set_view_camera(id, x, y, zoom)?;
        Ok(())
    }

    pub fn request_render(&mut self, id: &str) -> Result<(), RuntimeError> {
        self.scheduler.mark_dirty(id);
        Ok(())
    }

    pub fn set_visible(&mut self, id: &str, visible: bool) -> Result<(), RuntimeError> {
        logging::log(&format!("Setting visibility for view {}: {}", id, visible));
        self.views.set_visible(id, visible)?;
        Ok(())
    }

    pub fn render_dirty_views(&mut self) -> Result<(), RuntimeError> {
        if self.gpu.is_none() {
            return Err(RuntimeError::GpuNotInitialized);
        }

        let dirty_views: Vec<String> = self.scheduler.dirty_views().cloned().collect();

        for view_id in dirty_views {
            if let Some(state) = self.views.get(&view_id) {
                if state.attached && state.visible {
                    if let Err(e) = self.render_view_internal(&view_id) {
                        logging::error(&format!("Failed to render view {}: {}", view_id, e));
                    } else {
                        self.scheduler.clear_dirty(&view_id);
                    }
                }
            }
        }

        Ok(())
    }

    fn render_view_internal(&mut self, view_id: &str) -> Result<(), RuntimeError> {
        let gpu = self.gpu.as_ref().ok_or(RuntimeError::GpuNotInitialized)?;

        let state = self
            .views
            .get_mut(view_id)
            .ok_or_else(|| RuntimeError::ViewNotFound(view_id.to_string()))?;

        let surface = match &state.surface {
            Some(super::gpu::SurfaceHandle::Web(s)) => s,
            None => {
                return Err(RuntimeError::InvalidState(
                    "Surface not available".to_string(),
                ));
            }
        };

        state
            .config
            .as_ref()
            .ok_or_else(|| RuntimeError::InvalidState("Surface not configured".to_string()))?;

        let surface_texture = match surface.get_current_texture() {
            Ok(texture) => texture,
            Err(e) => {
                logging::error(&format!("Failed to get surface texture: {:?}", e));
                return Err(RuntimeError::RenderFailed(format!(
                    "Surface texture unavailable: {:?}",
                    e
                )));
            }
        };

        let view = surface_texture
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        let renderer = state
            .renderer
            .as_mut()
            .ok_or_else(|| RuntimeError::InvalidState("Renderer not initialized".to_string()))?;

        let text_renderer = state.text_renderer.as_mut().ok_or_else(|| {
            RuntimeError::InvalidState("Text renderer not initialized".to_string())
        })?;

        let pan = state.view.pan();
        let zoom = state.view.zoom();
        let viewport = [state.view.width() as f32, state.view.height() as f32];

        renderer.begin_frame();
        renderer.set_viewport(&gpu.queue, viewport);
        renderer.draw_rect(
            world_to_screen([0.0, 0.0], &state.view),
            world_to_screen([100.0, 100.0], &state.view),
            [1.0, 0.0, 0.0, 1.0],
        );
        renderer.draw_grid(pan, zoom, viewport);
        renderer.upload(&gpu.queue);

        text_renderer.begin_frame();
        text_renderer.set_viewport(&gpu.queue, viewport);
        text_renderer.set_layer(3);

        {
            use crate::editor::text::CosmicTextEngine;

            let mut cosmic_engine = CosmicTextEngine::new();

            let world_font_size = 12.0;
            let world_pos = [0.0, 0.0];
            let color = [1.0, 1.0, 1.0, 1.0];

            let (glyphs, font_size) =
                cosmic_engine.shape_text("Hello WGlymr", world_font_size, world_pos);

            cosmic_engine.render_glyphs(
                &glyphs,
                font_size,
                &state.view,
                text_renderer,
                &gpu.queue,
                &gpu.device,
                color,
                3,
            );
        }

        text_renderer.finish_batch();
        text_renderer.upload(&gpu.queue);

        let mut encoder = gpu
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            });

        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 0.05,
                            g: 0.05,
                            b: 0.065,
                            a: 1.0,
                        }),
                        store: wgpu::StoreOp::Store,
                    },
                    depth_slice: None,
                })],
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None,
            });

            renderer.render_lines(&mut render_pass);
            renderer.render_rects(&mut render_pass);
            text_renderer.render(&mut render_pass);
        }

        gpu.queue.submit(std::iter::once(encoder.finish()));
        surface_texture.present();

        Ok(())
    }
}

use super::logging;
