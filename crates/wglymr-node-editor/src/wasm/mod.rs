use std::cell::RefCell;
use std::collections::HashMap;

use wasm_bindgen::JsCast;
use wasm_bindgen::prelude::*;
use web_sys::HtmlCanvasElement;
use wgpu::{Device, Queue, Surface, SurfaceConfiguration};

use crate::document::adapter::BasicDocumentAdapter;
use crate::engine::{EditorEngine, ViewId};
use wglymr_render_wgpu::PrimitiveRenderer;

struct GpuContext {
    instance: wgpu::Instance,
    adapter: wgpu::Adapter,
    device: Device,
    queue: Queue,
}

thread_local! {
    static ENGINE: RefCell<Option<EditorEngine>> = RefCell::new(None);
    static WEB_CONTEXTS: RefCell<Option<HashMap<ViewId, WebViewContext>>> = RefCell::new(None);
    static GPU: RefCell<Option<GpuContext>> = RefCell::new(None);
}

#[wasm_bindgen(start)]
pub fn wasm_start() {
    console_error_panic_hook::set_once();
}

#[wasm_bindgen]
pub async fn init_gpu() -> Result<(), JsValue> {
    let gpu_initialized = GPU.with(|gpu| gpu.borrow().is_some());
    if gpu_initialized {
        return Ok(());
    }

    let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
        backends: wgpu::Backends::BROWSER_WEBGPU,
        ..Default::default()
    });

    let adapter = instance
        .request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::HighPerformance,
            compatible_surface: None,
            force_fallback_adapter: false,
        })
        .await
        .ok_or_else(|| JsValue::from_str("Failed to request adapter"))?;

    let (device, queue) = adapter
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
        .map_err(|e| JsValue::from_str(&format!("Failed to request device: {:?}", e)))?;

    GPU.with(|gpu| {
        *gpu.borrow_mut() = Some(GpuContext {
            instance,
            adapter,
            device,
            queue,
        });
    });

    Ok(())
}

struct WebViewContext {
    canvas: HtmlCanvasElement,
    surface: Surface<'static>,
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
    let view_id_obj = ViewId::new(view_id.to_string());

    WEB_CONTEXTS.with(|contexts| {
        let contexts = contexts.borrow();
        if let Some(ctxs) = contexts.as_ref() {
            if ctxs.contains_key(&view_id_obj) {
                // Already attached, do nothing
                return;
            }
        }
    });

    let engine_initialized = ENGINE.with(|engine| engine.borrow().is_some());
    if !engine_initialized {
        return;
    }

    let contexts_initialized = WEB_CONTEXTS.with(|contexts| contexts.borrow().is_some());
    if !contexts_initialized {
        return;
    }

    let gpu_initialized = GPU.with(|gpu| gpu.borrow().is_some());
    if !gpu_initialized {
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

    GPU.with(|gpu| {
        let gpu = gpu.borrow();
        let gpu = match gpu.as_ref() {
            Some(g) => g,
            None => return,
        };

        let surface = match gpu
            .instance
            .create_surface(wgpu::SurfaceTarget::Canvas(canvas.clone()))
        {
            Ok(s) => s,
            Err(_) => return,
        };

        let surface_caps = surface.get_capabilities(&gpu.adapter);
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

        surface.configure(&gpu.device, &config);

        let primitive_renderer = PrimitiveRenderer::new(&gpu.device, surface_format);

        let context = WebViewContext {
            canvas,
            surface,
            config,
            primitive_renderer,
        };

        WEB_CONTEXTS.with(|contexts| {
            let mut contexts = contexts.borrow_mut();
            if let Some(contexts) = contexts.as_mut() {
                contexts.insert(view_id_obj, context);
            }
        });
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
    let id = ViewId::new(view_id.to_string());

    ENGINE.with(|engine| {
        let mut engine = engine.borrow_mut();
        if let Some(engine) = engine.as_mut() {
            engine.resize_view(id.clone(), width, height);
        }
    });

    GPU.with(|gpu| {
        let gpu = gpu.borrow();
        let gpu = match gpu.as_ref() {
            Some(g) => g,
            None => return,
        };

        WEB_CONTEXTS.with(|contexts| {
            let mut contexts = contexts.borrow_mut();
            if let Some(contexts) = contexts.as_mut() {
                if let Some(context) = contexts.get_mut(&id) {
                    context.config.width = width;
                    context.config.height = height;
                    context.surface.configure(&gpu.device, &context.config);
                }
            }
        });
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

    GPU.with(|gpu| {
        let gpu = gpu.borrow();
        let gpu = match gpu.as_ref() {
            Some(g) => g,
            None => return,
        };

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

            let mut encoder = gpu
                .device
                .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                    label: Some("Render Encoder"),
                });

            context.primitive_renderer.begin_frame();

            let viewport = [context.config.width as f32, context.config.height as f32];
            context
                .primitive_renderer
                .set_viewport(&gpu.queue, viewport);

            ENGINE.with(|engine| {
                let mut engine = engine.borrow_mut();
                if let Some(engine) = engine.as_mut() {
                    engine.draw_view(&view_id_obj, &gpu.queue, &mut context.primitive_renderer);
                }
            });

            {
                let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                    label: Some("Render Pass"),
                    color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                        view: &texture_view,
                        resolve_target: None,
                        ops: wgpu::Operations {
                            load: wgpu::LoadOp::Clear(wgpu::Color {
                                r: 0.1,
                                g: 0.1,
                                b: 0.15,
                                a: 1.0,
                            }),
                            store: wgpu::StoreOp::Store,
                        },
                    })],
                    depth_stencil_attachment: None,
                    timestamp_writes: None,
                    occlusion_query_set: None,
                });

                context.primitive_renderer.render_rects(&mut render_pass);
                context.primitive_renderer.render_lines(&mut render_pass);
            }

            gpu.queue.submit(std::iter::once(encoder.finish()));
            surface_texture.present();
        });
    });
}
