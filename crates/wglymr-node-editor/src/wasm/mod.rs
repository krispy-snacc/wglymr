use std::cell::RefCell;
use std::collections::HashMap;

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::HtmlCanvasElement;
use wgpu::{Device, Queue, Surface, SurfaceConfiguration};

use crate::document::adapter::BasicDocumentAdapter;
use crate::engine::{EditorEngine, ViewId};
use wglymr_render_wgpu::PrimitiveRenderer;

thread_local! {
    static ENGINE: RefCell<Option<EditorEngine>> = RefCell::new(None);
    static WEB_CONTEXTS: RefCell<Option<HashMap<ViewId, WebViewContext>>> = RefCell::new(None);
}

#[wasm_bindgen(start)]
pub fn wasm_start() {
    console_error_panic_hook::set_once();
}

struct WebViewContext {
    canvas: HtmlCanvasElement,
    surface: Surface<'static>,
    device: Device,
    queue: Queue,
    config: SurfaceConfiguration,
    primitive_renderer: PrimitiveRenderer,
}

#[wasm_bindgen]
pub fn init_engine() {
    ENGINE.with(|engine| {
        let mut engine = engine.borrow_mut();
        if engine.is_some() {
            return;
        }
        let adapter = BasicDocumentAdapter::new();
        *engine = Some(EditorEngine::new(Box::new(adapter)));
    });

    WEB_CONTEXTS.with(|contexts| {
        let mut contexts = contexts.borrow_mut();
        if contexts.is_none() {
            *contexts = Some(HashMap::new());
        }
    });
}

#[wasm_bindgen]
pub async fn attach_view_canvas(view_id: &str, canvas_id: &str, width: u32, height: u32) {
    let engine_initialized = ENGINE.with(|engine| engine.borrow().is_some());
    if !engine_initialized {
        return;
    }

    let contexts_initialized = WEB_CONTEXTS.with(|contexts| contexts.borrow().is_some());
    if !contexts_initialized {
        return;
    }

    let view_id_obj = ViewId::new(view_id.to_string());

    let view_exists = WEB_CONTEXTS.with(|contexts| {
        contexts.borrow().as_ref().map_or(false, |ctx| ctx.contains_key(&view_id_obj))
    });
    if view_exists {
        return;
    }

    let window = match web_sys::window() {
        Some(w) => w,
        None => return,
    };
    let document = match window.document() {
        Some(d) => d,
        None => return,
    };
    let canvas = match document.get_element_by_id(canvas_id) {
        Some(elem) => match elem.dyn_into::<HtmlCanvasElement>() {
            Ok(c) => c,
            Err(_) => return,
        },
        None => return,
    };

    let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
        backends: wgpu::Backends::BROWSER_WEBGPU,
        ..Default::default()
    });

    let surface = match instance.create_surface(wgpu::SurfaceTarget::Canvas(canvas.clone())) {
        Ok(s) => s,
        Err(_) => return,
    };

    let adapter = match instance
        .request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::HighPerformance,
            compatible_surface: Some(&surface),
            force_fallback_adapter: false,
        })
        .await
    {
        Some(a) => a,
        None => return,
    };

    let (device, queue) = match adapter
        .request_device(
            &wgpu::DeviceDescriptor {
                label: Some("Editor Device"),
                required_features: wgpu::Features::empty(),
                required_limits: wgpu::Limits::downlevel_webgl2_defaults(),
                memory_hints: wgpu::MemoryHints::default(),
            },
            None,
        )
        .await
    {
        Ok(d) => d,
        Err(_) => return,
    };

    let surface_caps = surface.get_capabilities(&adapter);
    let surface_format = surface_caps
        .formats
        .iter()
        .copied()
        .find(|f| f.is_srgb())
        .unwrap_or(surface_caps.formats[0]);

    let config = SurfaceConfiguration {
        usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
        format: surface_format,
        width,
        height,
        present_mode: wgpu::PresentMode::Fifo,
        alpha_mode: surface_caps.alpha_modes[0],
        view_formats: vec![],
        desired_maximum_frame_latency: 2,
    };

    surface.configure(&device, &config);

    let primitive_renderer = PrimitiveRenderer::new(&device, surface_format);

    let context = WebViewContext {
        canvas,
        surface,
        device,
        queue,
        config,
        primitive_renderer,
    };

    WEB_CONTEXTS.with(|contexts| {
        let mut contexts = contexts.borrow_mut();
        if let Some(contexts) = contexts.as_mut() {
            contexts.insert(view_id_obj, context);
        }
    });
}

#[wasm_bindgen]
pub fn create_view(view_id: &str) {
    ENGINE.with(|engine| {
        let mut engine = engine.borrow_mut();
        if let Some(engine) = engine.as_mut() {
            let id = ViewId::new(view_id.to_string());
            if !engine.has_view(&id) {
                engine.create_view(id);
            }
        }
    });
}

#[wasm_bindgen]
pub fn destroy_view(view_id: &str) {
    let id = ViewId::new(view_id.to_string());
    
    ENGINE.with(|engine| {
        let mut engine = engine.borrow_mut();
        if let Some(engine) = engine.as_mut() {
            engine.destroy_view(id.clone());
        }
    });
    
    WEB_CONTEXTS.with(|contexts| {
        let mut contexts = contexts.borrow_mut();
        if let Some(contexts) = contexts.as_mut() {
            contexts.remove(&id);
        }
    });
}

#[wasm_bindgen]
pub fn resize_view(view_id: &str, width: u32, height: u32) {
    ENGINE.with(|engine| {
        let mut engine = engine.borrow_mut();
        if let Some(engine) = engine.as_mut() {
            let id = ViewId::new(view_id.to_string());
            engine.resize_view(id, width, height);
        }
    });
}

#[wasm_bindgen]
pub fn set_view_camera(view_id: &str, pan_x: f32, pan_y: f32, zoom: f32) {
    ENGINE.with(|engine| {
        let mut engine = engine.borrow_mut();
        if let Some(engine) = engine.as_mut() {
            let id = ViewId::new(view_id.to_string());
            engine.set_view_camera(id, [pan_x, pan_y], zoom);
        }
    });
}

#[wasm_bindgen]
pub fn render_view(view_id: &str) {
    let view_id_obj = ViewId::new(view_id.to_string());

    WEB_CONTEXTS.with(|contexts| {
        let mut contexts = contexts.borrow_mut();
        let contexts = match contexts.as_mut() {
            Some(c) => c,
            None => return,
        };
        let context = match contexts.get_mut(&view_id_obj) {
            Some(c) => c,
            None => return,
        };

        let surface_texture = match context.surface.get_current_texture() {
            Ok(t) => t,
            Err(_) => return,
        };

        let texture_view = surface_texture
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        let mut encoder = context
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            });

        context.primitive_renderer.set_camera(
            &context.queue,
            [0.0, 0.0],
            1.0,
        );

        context.primitive_renderer.draw_rect(
            &context.queue,
            [-0.5, -0.5],
            [0.5, 0.5],
            [1.0, 0.5, 0.0, 1.0]
        );
        
        context.primitive_renderer.draw_line(
            &context.queue,
            [-0.8, 0.8],
            [0.8, -0.8],
            [0.0, 1.0, 1.0, 1.0],
        );

        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &texture_view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 0.2,
                            g: 0.2,
                            b: 0.2,
                            a: 1.0,
                        }),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None,
            });

            context.primitive_renderer.render_rects(&mut render_pass, 6);
            context.primitive_renderer.render_lines(&mut render_pass, 2);
        }

        context.queue.submit(std::iter::once(encoder.finish()));
        surface_texture.present();
    });
}
