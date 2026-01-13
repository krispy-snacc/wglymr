use super::errors::RuntimeError;
use super::gpu::SurfaceHandle;
use crate::engine::ViewId;
use std::collections::HashMap;
use wglymr_render_wgpu::{MsdfTextRenderer, PrimitiveRenderer, SdfRenderer};

pub struct GpuViewState {
    pub visible: bool,
    pub attached: bool,
    pub surface: Option<SurfaceHandle>,
    pub config: Option<wgpu::SurfaceConfiguration>,
    pub primitive_renderer: Option<PrimitiveRenderer>,
    pub sdf_renderer: Option<SdfRenderer>,
    pub msdf_text_renderer: Option<MsdfTextRenderer>,
}

impl GpuViewState {
    fn new() -> Self {
        Self {
            visible: false,
            attached: false,
            surface: None,
            config: None,
            primitive_renderer: None,
            sdf_renderer: None,
            msdf_text_renderer: None,
        }
    }
}

pub struct GpuViewRegistry {
    views: HashMap<ViewId, GpuViewState>,
}

impl GpuViewRegistry {
    pub fn new() -> Self {
        Self {
            views: HashMap::new(),
        }
    }

    pub fn create_view(&mut self, id: &str) -> Result<(), RuntimeError> {
        let view_id = ViewId::new(id.to_string());
        if self.views.contains_key(&view_id) {
            return Err(RuntimeError::ViewAlreadyExists(id.to_string()));
        }

        let state = GpuViewState::new();
        self.views.insert(view_id, state);

        Ok(())
    }

    pub fn destroy_view(&mut self, id: &str) -> Result<(), RuntimeError> {
        let view_id = ViewId::new(id.to_string());
        if self.views.get(&view_id).is_some() {
            self.detach_view(id)?;
        }
        if self.views.remove(&view_id).is_none() {
            return Err(RuntimeError::ViewNotFound(id.to_string()));
        }
        Ok(())
    }

    pub fn attach_view(
        &mut self,
        id: &str,
        surface: SurfaceHandle,
        backing_width: u32,
        backing_height: u32,
        gpu: &super::gpu::GpuContext,
    ) -> Result<(), RuntimeError> {
        let view_id = ViewId::new(id.to_string());
        let state = self
            .views
            .get_mut(&view_id)
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
            width: backing_width,
            height: backing_height,
            present_mode: wgpu::PresentMode::Fifo,
            alpha_mode: capabilities.alpha_modes[0],
            view_formats: vec![],
            desired_maximum_frame_latency: 2,
        };

        wgpu_surface.configure(&gpu.device, &config);

        let primitive_renderer = PrimitiveRenderer::new(&gpu.device, format);
        let sdf_renderer = SdfRenderer::new(&gpu.device, format);
        let msdf_text_renderer = MsdfTextRenderer::new(&gpu.device, format);

        state.surface = Some(surface);
        state.config = Some(config);
        state.primitive_renderer = Some(primitive_renderer);
        state.sdf_renderer = Some(sdf_renderer);
        state.msdf_text_renderer = Some(msdf_text_renderer);
        state.attached = true;

        Ok(())
    }

    pub fn detach_view(&mut self, id: &str) -> Result<(), RuntimeError> {
        let view_id = ViewId::new(id.to_string());
        let state = self
            .views
            .get_mut(&view_id)
            .ok_or_else(|| RuntimeError::ViewNotFound(id.to_string()))?;

        state.surface = None;
        state.config = None;
        state.primitive_renderer = None;
        state.sdf_renderer = None;
        state.msdf_text_renderer = None;
        state.attached = false;
        Ok(())
    }

    pub fn reconfigure_surface(
        &mut self,
        id: &str,
        backing_width: u32,
        backing_height: u32,
        gpu: &super::gpu::GpuContext,
    ) -> Result<(), RuntimeError> {
        let view_id = ViewId::new(id.to_string());
        let state = self
            .views
            .get_mut(&view_id)
            .ok_or_else(|| RuntimeError::ViewNotFound(id.to_string()))?;

        if state.attached {
            if let (Some(surface), Some(config)) = (&state.surface, &mut state.config) {
                let wgpu_surface = match surface {
                    SurfaceHandle::Web(s) => s,
                };

                config.width = backing_width;
                config.height = backing_height;
                wgpu_surface.configure(&gpu.device, config);
            }
        }

        Ok(())
    }

    pub fn resize_view(
        &mut self,
        id: &str,
        backing_width: u32,
        backing_height: u32,
        gpu: &super::gpu::GpuContext,
    ) -> Result<(), RuntimeError> {
        let view_id = ViewId::new(id.to_string());
        let state = self
            .views
            .get_mut(&view_id)
            .ok_or_else(|| RuntimeError::ViewNotFound(id.to_string()))?;

        if state.attached {
            if let (Some(surface), Some(config)) = (&state.surface, &mut state.config) {
                let wgpu_surface = match surface {
                    SurfaceHandle::Web(s) => s,
                };

                config.width = backing_width;
                config.height = backing_height;
                wgpu_surface.configure(&gpu.device, config);
            }
        }

        Ok(())
    }

    pub fn get(&self, id: &str) -> Option<&GpuViewState> {
        let view_id = ViewId::new(id.to_string());
        self.views.get(&view_id)
    }

    pub fn get_mut(&mut self, id: &str) -> Option<&mut GpuViewState> {
        let view_id = ViewId::new(id.to_string());
        self.views.get_mut(&view_id)
    }

    pub fn iter(&self) -> impl Iterator<Item = (&ViewId, &GpuViewState)> {
        self.views.iter()
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = (&ViewId, &mut GpuViewState)> {
        self.views.iter_mut()
    }

    pub fn set_visible(&mut self, id: &str, visible: bool) -> Result<(), RuntimeError> {
        let view_id = ViewId::new(id.to_string());
        let state = self
            .views
            .get_mut(&view_id)
            .ok_or_else(|| RuntimeError::ViewNotFound(id.to_string()))?;
        state.visible = visible;
        Ok(())
    }

    pub fn all_view_ids(&self) -> impl Iterator<Item = String> + '_ {
        self.views
            .keys()
            .map(|view_id| view_id.as_str().to_string())
    }
}
