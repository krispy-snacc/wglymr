use super::errors::RuntimeError;
use super::gpu::SurfaceHandle;
use crate::engine::EditorView;
use std::collections::HashMap;

pub type ViewId = String;

pub struct ViewState {
    pub view: EditorView,
    pub visible: bool,
    pub attached: bool,
    pub surface: Option<SurfaceHandle>,
    pub config: Option<wgpu::SurfaceConfiguration>,
    pub width: u32,
    pub height: u32,
}

impl ViewState {
    fn new(view: EditorView) -> Self {
        Self {
            view,
            visible: false,
            attached: false,
            surface: None,
            config: None,
            width: 0,
            height: 0,
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
        width: u32,
        height: u32,
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

        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format,
            width,
            height,
            present_mode: wgpu::PresentMode::Fifo,
            alpha_mode: capabilities.alpha_modes[0],
            view_formats: vec![],
            desired_maximum_frame_latency: 2,
        };

        wgpu_surface.configure(&gpu.device, &config);

        state.surface = Some(surface);
        state.config = Some(config);
        state.width = width;
        state.height = height;
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
        state.attached = false;
        Ok(())
    }

    pub fn resize_view(
        &mut self,
        id: &str,
        width: u32,
        height: u32,
        gpu: &super::gpu::GpuContext,
    ) -> Result<(), RuntimeError> {
        let state = self
            .views
            .get_mut(id)
            .ok_or_else(|| RuntimeError::ViewNotFound(id.to_string()))?;

        state.width = width;
        state.height = height;

        if state.attached {
            if let (Some(surface), Some(config)) = (&state.surface, &mut state.config) {
                let wgpu_surface = match surface {
                    SurfaceHandle::Web(s) => s,
                };

                config.width = width;
                config.height = height;
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
