use super::errors::RuntimeError;
use super::gpu::SurfaceHandle;
use crate::engine::EditorView;
use std::collections::HashMap;
use wglymr_render_wgpu::{GlyphonTextRenderer, PrimitiveRenderer, SdfRenderer};

pub type ViewId = String;

pub struct ViewState {
    pub view: EditorView,
    pub visible: bool,
    pub attached: bool,
    pub surface: Option<SurfaceHandle>,
    pub config: Option<wgpu::SurfaceConfiguration>,
    pub renderer: Option<PrimitiveRenderer>,
    pub sdf_renderer: Option<SdfRenderer>,
    pub glyphon_text_renderer: Option<GlyphonTextRenderer>,
}

impl ViewState {
    fn new(view: EditorView) -> Self {
        Self {
            view,
            visible: false,
            attached: false,
            surface: None,
            config: None,
            renderer: None,
            sdf_renderer: None,
            glyphon_text_renderer: None,
        }
    }
}

pub struct ViewRegistry {
    views: HashMap<ViewId, ViewState>,
}

impl ViewRegistry {
    pub fn new() -> Self {
        Self {
            views: HashMap::new(),
        }
    }

    pub fn create_view(&mut self, id: &str) -> Result<(), RuntimeError> {
        if self.views.contains_key(id) {
            return Err(RuntimeError::ViewAlreadyExists(id.to_string()));
        }

        let view = EditorView::new();
        let state = ViewState::new(view);
        self.views.insert(id.to_string(), state);

        Ok(())
    }

    pub fn destroy_view(&mut self, id: &str) -> Result<(), RuntimeError> {
        if self.views.get(id).is_some() {
            self.detach_view(id)?;
        }
        if self.views.remove(id).is_none() {
            return Err(RuntimeError::ViewNotFound(id.to_string()));
        }
        Ok(())
    }

    pub fn attach_view(
        &mut self,
        id: &str,
        surface: SurfaceHandle,
        css_width: u32,
        css_height: u32,
        backing_scale: f32,
        gpu: &super::gpu::GpuContext,
    ) -> Result<(), RuntimeError> {
        let state = self
            .views
            .get_mut(id)
            .ok_or_else(|| RuntimeError::ViewNotFound(id.to_string()))?;

        if state.attached {
            state.surface = None;
            state.config = None;
            state.attached = false;
        }

        let wgpu_surface = match &surface {
            SurfaceHandle::Web(s) => s,
        };

        let capabilities = wgpu_surface.get_capabilities(&gpu.adapter);
        let format = capabilities
            .formats
            .iter()
            .find(|f| f.is_srgb())
            .copied()
            .unwrap_or(capabilities.formats[0]);

        // Compute backing dimensions from CSS size and scale
        // This is the actual pixel resolution that WebGPU will render to
        let backing_width = (css_width as f32 * backing_scale) as u32;
        let backing_height = (css_height as f32 * backing_scale) as u32;

        // Surface config uses backing dimensions (the actual render target size)
        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format,
            width: backing_width,
            height: backing_height,
            present_mode: wgpu::PresentMode::Fifo,
            alpha_mode: capabilities.alpha_modes[0],
            view_formats: vec![],
            desired_maximum_frame_latency: 2,
        };

        wgpu_surface.configure(&gpu.device, &config);

        let renderer = PrimitiveRenderer::new(&gpu.device, format);
        let sdf_renderer = SdfRenderer::new(&gpu.device, format);
        let glyphon_text_renderer = GlyphonTextRenderer::new(&gpu.device, &gpu.queue, format);

        state.surface = Some(surface);
        state.config = Some(config);
        state.renderer = Some(renderer);
        state.sdf_renderer = Some(sdf_renderer);
        state.glyphon_text_renderer = Some(glyphon_text_renderer);

        state.view.resize(css_width, css_height, backing_scale);
        state.attached = true;

        Ok(())
    }

    pub fn detach_view(&mut self, id: &str) -> Result<(), RuntimeError> {
        let state = self
            .views
            .get_mut(id)
            .ok_or_else(|| RuntimeError::ViewNotFound(id.to_string()))?;

        state.surface = None;
        state.config = None;
        state.renderer = None;
        state.sdf_renderer = None;
        state.glyphon_text_renderer = None;
        state.attached = false;
        Ok(())
    }

    pub fn resize_view(
        &mut self,
        id: &str,
        css_width: u32,
        css_height: u32,
        backing_scale: f32,
        gpu: &super::gpu::GpuContext,
    ) -> Result<(), RuntimeError> {
        let state = self
            .views
            .get_mut(id)
            .ok_or_else(|| RuntimeError::ViewNotFound(id.to_string()))?;

        state.view.resize(css_width, css_height, backing_scale);

        if state.attached {
            if let (Some(surface), Some(config)) = (&state.surface, &mut state.config) {
                let wgpu_surface = match surface {
                    SurfaceHandle::Web(s) => s,
                };

                config.width = state.view.backing_width();
                config.height = state.view.backing_height();
                wgpu_surface.configure(&gpu.device, config);
            }
        }

        Ok(())
    }

    pub fn set_view_camera(
        &mut self,
        id: &str,
        x: f32,
        y: f32,
        zoom: f32,
    ) -> Result<(), RuntimeError> {
        let state = self
            .views
            .get_mut(id)
            .ok_or_else(|| RuntimeError::ViewNotFound(id.to_string()))?;

        state.view.set_camera([x, y], zoom);
        Ok(())
    }

    pub fn get(&self, id: &str) -> Option<&ViewState> {
        self.views.get(id)
    }

    pub fn get_mut(&mut self, id: &str) -> Option<&mut ViewState> {
        self.views.get_mut(id)
    }

    pub fn iter(&self) -> impl Iterator<Item = (&ViewId, &ViewState)> {
        self.views.iter()
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = (&ViewId, &mut ViewState)> {
        self.views.iter_mut()
    }

    pub fn set_visible(&mut self, id: &str, visible: bool) -> Result<(), RuntimeError> {
        let state = self
            .views
            .get_mut(id)
            .ok_or_else(|| RuntimeError::ViewNotFound(id.to_string()))?;

        state.visible = visible;
        Ok(())
    }
}
