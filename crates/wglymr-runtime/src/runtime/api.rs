use super::errors::RuntimeError;
use super::EditorRuntime;
use wglymr_app::ViewId;

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
        let view_id = ViewId::new(id.to_string());
        self.engine.create_view(view_id);
        self.gpu_views.create_view(id)?;
        Ok(())
    }

    pub fn destroy_view(&mut self, id: &str) -> Result<(), RuntimeError> {
        logging::log(&format!("Destroying view: {}", id));
        let view_id = ViewId::new(id.to_string());
        if let Err(e) = self.gpu_views.destroy_view(id) {
            logging::error(&format!("Failed to destroy GPU view state: {}", e));
        }
        self.engine.destroy_view(&view_id);
        self.scheduler.clear_dirty(id);
        Ok(())
    }

    pub fn attach_view(
        &mut self,
        id: &str,
        surface: super::gpu::SurfaceHandle,
        css_width: u32,
        css_height: u32,
        backing_scale: f32,
    ) -> Result<(), RuntimeError> {
        let backing_width = (css_width as f32 * backing_scale) as u32;
        let backing_height = (css_height as f32 * backing_scale) as u32;

        logging::log(&format!("Attaching view {}", id));

        let gpu = self.gpu.as_ref().ok_or(RuntimeError::GpuNotInitialized)?;

        let view_id = ViewId::new(id.to_string());

        if !self.engine.has_view(&view_id) {
            return Err(RuntimeError::ViewNotFound(id.to_string()));
        }

        self.engine.resize_view(
            &view_id,
            css_width,
            css_height,
            backing_width,
            backing_height,
        );

        self.gpu_views
            .attach_view(id, surface, backing_width, backing_height, gpu)?;
        Ok(())
    }

    pub fn detach_view(&mut self, id: &str) -> Result<(), RuntimeError> {
        logging::log(&format!("Detaching view: {}", id));
        self.gpu_views.detach_view(id)?;
        Ok(())
    }

    pub fn resize_view(
        &mut self,
        id: &str,
        css_width: u32,
        css_height: u32,
        backing_scale: f32,
    ) -> Result<(), RuntimeError> {
        let backing_width = (css_width as f32 * backing_scale) as u32;
        let backing_height = (css_height as f32 * backing_scale) as u32;

        logging::log(&format!("Resizing view {}", id));

        let gpu = self.gpu.as_ref().ok_or(RuntimeError::GpuNotInitialized)?;

        let view_id = ViewId::new(id.to_string());

        if !self.engine.has_view(&view_id) {
            return Err(RuntimeError::ViewNotFound(id.to_string()));
        }

        self.engine.resize_view(
            &view_id,
            css_width,
            css_height,
            backing_width,
            backing_height,
        );

        self.gpu_views
            .reconfigure_surface(id, backing_width, backing_height, gpu)?;
        Ok(())
    }

    pub fn set_view_camera(
        &mut self,
        id: &str,
        x: f32,
        y: f32,
        zoom: f32,
    ) -> Result<(), RuntimeError> {
        logging::log(&format!("Setting camera for view {}", id));
        let view_id = ViewId::new(id.to_string());
        self.engine.set_view_camera(&view_id, x, y, zoom);
        Ok(())
    }

    pub fn request_render(&mut self, id: &str) -> Result<(), RuntimeError> {
        self.scheduler.mark_dirty(id);
        Ok(())
    }

    pub fn handle_mouse_move(
        &mut self,
        id: &str,
        screen_x: f32,
        screen_y: f32,
        shift: bool,
        ctrl: bool,
        alt: bool,
    ) -> Result<(), RuntimeError> {
        let event = wglymr_interaction::MouseEvent {
            kind: wglymr_interaction::MouseEventKind::Move,
            screen_pos: [screen_x, screen_y],
        };
        let modifiers = wglymr_interaction::KeyModifiers { shift, ctrl, alt };
        let view_id = ViewId::new(id.to_string());

        self.engine.handle_mouse_event(&view_id, event, modifiers);
        self.scheduler.mark_dirty(id);
        Ok(())
    }

    // Mouse event handlers mirror web API structure - 8 params are acceptable
    #[allow(clippy::too_many_arguments)]
    pub fn handle_mouse_down(
        &mut self,
        id: &str,
        screen_x: f32,
        screen_y: f32,
        button: u8,
        shift: bool,
        ctrl: bool,
        alt: bool,
    ) -> Result<(), RuntimeError> {
        let mouse_button = match button {
            0 => wglymr_interaction::MouseButton::Left,
            1 => wglymr_interaction::MouseButton::Middle,
            2 => wglymr_interaction::MouseButton::Right,
            _ => return Ok(()),
        };

        let event = wglymr_interaction::MouseEvent {
            kind: wglymr_interaction::MouseEventKind::Down(mouse_button),
            screen_pos: [screen_x, screen_y],
        };
        let modifiers = wglymr_interaction::KeyModifiers { shift, ctrl, alt };
        let view_id = ViewId::new(id.to_string());

        self.engine.handle_mouse_event(&view_id, event, modifiers);
        self.scheduler.mark_dirty(id);
        Ok(())
    }

    #[allow(clippy::too_many_arguments)]
    pub fn handle_mouse_up(
        &mut self,
        id: &str,
        screen_x: f32,
        screen_y: f32,
        button: u8,
        shift: bool,
        ctrl: bool,
        alt: bool,
    ) -> Result<(), RuntimeError> {
        let mouse_button = match button {
            0 => wglymr_interaction::MouseButton::Left,
            1 => wglymr_interaction::MouseButton::Middle,
            2 => wglymr_interaction::MouseButton::Right,
            _ => return Ok(()),
        };

        let event = wglymr_interaction::MouseEvent {
            kind: wglymr_interaction::MouseEventKind::Up(mouse_button),
            screen_pos: [screen_x, screen_y],
        };
        let modifiers = wglymr_interaction::KeyModifiers { shift, ctrl, alt };
        let view_id = ViewId::new(id.to_string());

        self.engine.handle_mouse_event(&view_id, event, modifiers);
        self.scheduler.mark_dirty(id);
        Ok(())
    }

    #[allow(clippy::too_many_arguments)]
    pub fn handle_mouse_enter(
        &mut self,
        id: &str,
        screen_x: f32,
        screen_y: f32,
        button: u8,
        shift: bool,
        ctrl: bool,
        alt: bool,
    ) -> Result<(), RuntimeError> {
        let mouse_button = match button {
            0 => wglymr_interaction::MouseButton::Left,
            1 => wglymr_interaction::MouseButton::Middle,
            2 => wglymr_interaction::MouseButton::Right,
            _ => return Ok(()),
        };

        let event = wglymr_interaction::MouseEvent {
            kind: wglymr_interaction::MouseEventKind::Enter(mouse_button),
            screen_pos: [screen_x, screen_y],
        };
        let modifiers = wglymr_interaction::KeyModifiers { shift, ctrl, alt };
        let view_id = ViewId::new(id.to_string());

        self.engine.handle_mouse_event(&view_id, event, modifiers);
        self.scheduler.mark_dirty(id);
        Ok(())
    }

    #[allow(clippy::too_many_arguments)]
    pub fn handle_mouse_leave(
        &mut self,
        id: &str,
        screen_x: f32,
        screen_y: f32,
        button: u8,
        shift: bool,
        ctrl: bool,
        alt: bool,
    ) -> Result<(), RuntimeError> {
        let mouse_button = match button {
            0 => wglymr_interaction::MouseButton::Left,
            1 => wglymr_interaction::MouseButton::Middle,
            2 => wglymr_interaction::MouseButton::Right,
            _ => return Ok(()),
        };

        let event = wglymr_interaction::MouseEvent {
            kind: wglymr_interaction::MouseEventKind::Leave(mouse_button),
            screen_pos: [screen_x, screen_y],
        };
        let modifiers = wglymr_interaction::KeyModifiers { shift, ctrl, alt };
        let view_id = ViewId::new(id.to_string());

        self.engine.handle_mouse_event(&view_id, event, modifiers);
        self.scheduler.mark_dirty(id);
        Ok(())
    }

    pub fn set_visible(&mut self, id: &str, visible: bool) -> Result<(), RuntimeError> {
        self.gpu_views.set_visible(id, visible)?;
        Ok(())
    }

    pub fn render_dirty_views(&mut self) -> Result<(), RuntimeError> {
        if self.gpu.is_none() {
            return Err(RuntimeError::GpuNotInitialized);
        }

        let dirty_views: Vec<String> = self.scheduler.dirty_views().cloned().collect();

        for view_id in dirty_views {
            if let Some(state) = self.gpu_views.get(&view_id) {
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

    pub fn render_all_views(&mut self) -> Result<(), RuntimeError> {
        if self.gpu.is_none() {
            return Err(RuntimeError::GpuNotInitialized);
        }

        let all_views: Vec<String> = self.gpu_views.all_view_ids().collect();

        for view_id in all_views {
            if let Some(state) = self.gpu_views.get(&view_id) {
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

        let gpu_state = self
            .gpu_views
            .get_mut(view_id)
            .ok_or_else(|| RuntimeError::ViewNotFound(view_id.to_string()))?;

        let surface = match &gpu_state.surface {
            Some(super::gpu::SurfaceHandle::Web(s)) => s,
            None => {
                return Err(RuntimeError::InvalidState(
                    "Surface not available".to_string(),
                ));
            }
        };

        gpu_state
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

        let draw_backend = gpu_state.draw_backend.as_mut().ok_or_else(|| {
            RuntimeError::InvalidState("Draw backend not initialized".to_string())
        })?;

        let engine_view_id = ViewId::new(view_id.to_string());

        let editor_view = self
            .engine
            .get_view(&engine_view_id)
            .ok_or_else(|| RuntimeError::ViewNotFound(view_id.to_string()))?;

        let pan = editor_view.pan();
        let zoom = editor_view.zoom();
        let viewport = [
            editor_view.backing_width() as f32,
            editor_view.backing_height() as f32,
        ];

        draw_backend.begin_frame();
        draw_backend.set_viewport(&gpu.queue, viewport);
        draw_backend.set_camera(pan, zoom);

        let grid_depth = wglymr_view::resolve_depth(wglymr_view::DepthLayer::Grid, 0.0);
        draw_backend
            .primitive_renderer_mut()
            .draw_grid(pan, zoom, viewport, grid_depth);

        self.engine.draw_view(&engine_view_id);

        let editor_view = self
            .engine
            .get_view(&engine_view_id)
            .ok_or_else(|| RuntimeError::ViewNotFound(view_id.to_string()))?;

        for draw_item in &editor_view.draw_items {
            draw_backend.emit(draw_item);
        }

        draw_backend.flush(&gpu.device, &gpu.queue);

        let depth_view = gpu_state
            .depth_view
            .as_ref()
            .ok_or_else(|| RuntimeError::InvalidState("Depth view not available".to_string()))?;

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
                            r: 18.0 / 255.0,
                            g: 18.0 / 255.0,
                            b: 18.0 / 255.0,
                            a: 1.0,
                        }),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                    view: depth_view,
                    depth_ops: Some(wgpu::Operations {
                        load: wgpu::LoadOp::Clear(1.0),
                        store: wgpu::StoreOp::Store,
                    }),
                    stencil_ops: None,
                }),
                timestamp_writes: None,
                occlusion_query_set: None,
            });

            draw_backend.render(&mut render_pass);
        }

        gpu.queue.submit(std::iter::once(encoder.finish()));
        surface_texture.present();

        Ok(())
    }
}

use super::logging;
