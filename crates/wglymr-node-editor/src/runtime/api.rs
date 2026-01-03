use wglymr_color::Color;
use wglymr_render_wgpu::RoundedRect;

use crate::editor::wgpu_renderer::{world_to_screen, world_to_screen_size};

use super::EditorRuntime;
use super::errors::RuntimeError;
use crate::editor::text::{CosmicShaper, TEXT, render_shaped_text};

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

        let sdf_renderer = state.sdf_renderer.as_mut().ok_or_else(|| {
            RuntimeError::InvalidState("Sdf renderer not initialized".to_string())
        })?;

        let sdf_text_renderer = state.sdf_text_renderer.as_mut().ok_or_else(|| {
            RuntimeError::InvalidState("SDF Text renderer not initialized".to_string())
        })?;

        let pan = state.view.pan();
        let zoom = state.view.zoom();
        let viewport = [state.view.width() as f32, state.view.height() as f32];

        renderer.begin_frame();
        renderer.set_viewport(&gpu.queue, viewport);

        renderer.draw_grid(pan, zoom, viewport);
        renderer.upload(&gpu.queue);

        sdf_text_renderer.begin_frame();
        sdf_text_renderer.set_viewport(&gpu.queue, viewport);
        sdf_text_renderer.set_layer(4);

        let mut shaper = CosmicShaper::new();

        let shaped = shaper.shape_text("Hello Wglymr", 12.0, [-48.0, -48.0]);

        render_shaped_text(
            &shaped,
            &state.view,
            sdf_text_renderer,
            &gpu.device,
            &gpu.queue,
            Color::WHITE,
            TEXT,
        );

        sdf_text_renderer.finish_batch();
        sdf_text_renderer.upload(&gpu.queue);

        sdf_renderer.begin_frame();
        sdf_renderer.set_viewport(&gpu.queue, viewport);
        sdf_renderer.set_layer(2);

        sdf_renderer.draw_rounded_rect(&RoundedRect {
            min: world_to_screen([-50.0, -50.0], &state.view),
            max: world_to_screen([50.0, 50.0], &state.view),
            radius: world_to_screen_size(4.0, &state.view),
            border_width: world_to_screen_size(1.0, &state.view),
            fill_color: Color::NODE_BG,
            border_color: Color::BLACK,
        });

        sdf_renderer.finish_batch();
        sdf_renderer.upload(&gpu.queue);

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
            sdf_renderer.render(&mut render_pass);
            sdf_text_renderer.render(&mut render_pass);
        }

        gpu.queue.submit(std::iter::once(encoder.finish()));
        surface_texture.present();

        Ok(())
    }
}

use super::logging;
