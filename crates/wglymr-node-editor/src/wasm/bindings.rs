use crate::runtime::EditorRuntime;
use crate::runtime::gpu::SurfaceHandle;
use crate::runtime::logging::RuntimeLogger;
use wasm_bindgen::prelude::*;
use web_sys::HtmlCanvasElement;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
    #[wasm_bindgen(js_namespace = console)]
    fn warn(s: &str);
    #[wasm_bindgen(js_namespace = console)]
    fn error(s: &str);
}

struct ConsoleLogger;

impl RuntimeLogger for ConsoleLogger {
    fn log(&self, message: &str) {
        log(message);
    }

    fn warn(&self, message: &str) {
        warn(&format!("[WARN] {}", message));
    }

    fn error(&self, message: &str) {
        error(&format!("[ERROR] {}", message));
    }

    fn debug(&self, message: &str) {
        log(&format!("[DEBUG] {}", message));
    }
}

static CONSOLE_LOGGER: ConsoleLogger = ConsoleLogger;

#[wasm_bindgen]
pub fn init_engine() {
    crate::runtime::logging::set_logger(&CONSOLE_LOGGER);

    EditorRuntime::with(|rt| {
        if let Err(e) = rt.init_engine() {
            crate::runtime::logging::error(&format!("Failed to initialize engine: {}", e));
        }
    });
}

#[wasm_bindgen]
pub async fn init_gpu() -> Result<(), JsValue> {
    let instance = wgpu::Instance::new(&wgpu::InstanceDescriptor {
        backends: wgpu::Backends::BROWSER_WEBGPU | wgpu::Backends::GL,
        ..Default::default()
    });

    let adapter = instance
        .request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::HighPerformance,
            compatible_surface: None,
            force_fallback_adapter: false,
        })
        .await
        .map_err(|e| JsValue::from_str(&format!("Failed to request adapter: {:?}", e)))?;

    let (device, queue) = adapter
        .request_device(&wgpu::DeviceDescriptor {
            label: Some("Device"),
            required_features: wgpu::Features::empty(),
            required_limits: wgpu::Limits::downlevel_webgl2_defaults(),
            memory_hints: Default::default(),
            trace: wgpu::Trace::Off,
        })
        .await
        .map_err(|e| JsValue::from_str(&format!("Failed to request device: {:?}", e)))?;

    let gpu_context = crate::runtime::gpu::GpuContext::new(instance, adapter, device, queue);

    EditorRuntime::with(|rt| {
        if let Err(e) = rt.init_gpu(gpu_context) {
            crate::runtime::logging::error(&format!("Failed to initialize GPU: {}", e));
        }
    });

    Ok(())
}

#[wasm_bindgen]
pub fn create_view(id: &str) {
    EditorRuntime::with(|rt| {
        if let Err(e) = rt.create_view(id) {
            crate::runtime::logging::error(&format!("Failed to create view: {}", e));
        }
    });
}

#[wasm_bindgen]
pub fn destroy_view(id: &str) {
    EditorRuntime::with(|rt| {
        if let Err(e) = rt.destroy_view(id) {
            crate::runtime::logging::error(&format!("Failed to destroy view: {}", e));
        }
    });
}

#[wasm_bindgen]
pub fn attach_view(
    id: &str,
    canvas: HtmlCanvasElement,
    css_width: u32,
    css_height: u32,
    backing_scale: f32,
) {
    EditorRuntime::with(|rt| {
        let gpu = match rt.gpu() {
            Some(gpu) => gpu,
            None => {
                crate::runtime::logging::error("GPU not initialized - call init_gpu first");
                return;
            }
        };

        let surface = match gpu
            .instance
            .create_surface(wgpu::SurfaceTarget::Canvas(canvas))
        {
            Ok(surf) => surf,
            Err(e) => {
                crate::runtime::logging::error(&format!("Failed to create surface: {:?}", e));
                return;
            }
        };

        let surface_handle = SurfaceHandle::Web(surface);

        if let Err(e) = rt.attach_view(id, surface_handle, css_width, css_height, backing_scale) {
            crate::runtime::logging::error(&format!("Failed to attach view: {}", e));
        }
    });
}

#[wasm_bindgen]
pub fn detach_view(id: &str) {
    EditorRuntime::with(|rt| {
        if let Err(e) = rt.detach_view(id) {
            crate::runtime::logging::error(&format!("Failed to detach view: {}", e));
        }
    });
}

#[wasm_bindgen]
pub fn resize_view(id: &str, css_width: u32, css_height: u32, backing_scale: f32) {
    EditorRuntime::with(|rt| {
        if let Err(e) = rt.resize_view(id, css_width, css_height, backing_scale) {
            crate::runtime::logging::error(&format!("Failed to resize view: {}", e));
        }
    });
}

#[wasm_bindgen]
pub fn set_view_camera(id: &str, x: f32, y: f32, zoom: f32) {
    EditorRuntime::with(|rt| {
        if let Err(e) = rt.set_view_camera(id, x, y, zoom) {
            crate::runtime::logging::error(&format!("Failed to set view camera: {}", e));
        }
    });
}

#[wasm_bindgen]
pub fn request_render(id: &str) {
    EditorRuntime::with(|rt| {
        if let Err(e) = rt.request_render(id) {
            crate::runtime::logging::error(&format!("Failed to request render: {}", e));
        }
    });
}

#[wasm_bindgen]
pub fn set_visible(id: &str, visible: bool) {
    EditorRuntime::with(|rt| {
        if let Err(e) = rt.set_visible(id, visible) {
            crate::runtime::logging::error(&format!("Failed to set visibility: {}", e));
        }
    });
}

#[wasm_bindgen]
pub fn render() {
    EditorRuntime::with(|rt| {
        if let Err(e) = rt.render_dirty_views() {
            crate::runtime::logging::error(&format!("Failed to render: {}", e));
        }
    });
}

#[wasm_bindgen]
pub fn start_render_loop() {
    EditorRuntime::with(|rt| {
        rt.render_loop_mut().start();
    });
}

#[wasm_bindgen]
pub fn stop_render_loop() {
    EditorRuntime::with(|rt| {
        rt.render_loop_mut().stop();
    });
}

#[wasm_bindgen]
pub fn handle_mouse_move(
    id: &str,
    screen_x: f32,
    screen_y: f32,
    shift: bool,
    ctrl: bool,
    alt: bool,
) {
    EditorRuntime::with(|rt| {
        if let Err(e) = rt.handle_mouse_move(id, screen_x, screen_y, shift, ctrl, alt) {
            crate::runtime::logging::error(&format!("Failed to handle mouse move: {}", e));
        }
    });
}

#[wasm_bindgen]
pub fn handle_mouse_down(
    id: &str,
    screen_x: f32,
    screen_y: f32,
    button: u8,
    shift: bool,
    ctrl: bool,
    alt: bool,
) {
    EditorRuntime::with(|rt| {
        if let Err(e) = rt.handle_mouse_down(id, screen_x, screen_y, button, shift, ctrl, alt) {
            crate::runtime::logging::error(&format!("Failed to handle mouse down: {}", e));
        }
    });
}

#[wasm_bindgen]
pub fn handle_mouse_up(
    id: &str,
    screen_x: f32,
    screen_y: f32,
    button: u8,
    shift: bool,
    ctrl: bool,
    alt: bool,
) {
    EditorRuntime::with(|rt| {
        if let Err(e) = rt.handle_mouse_up(id, screen_x, screen_y, button, shift, ctrl, alt) {
            crate::runtime::logging::error(&format!("Failed to handle mouse up: {}", e));
        }
    });
}

#[wasm_bindgen]
pub fn handle_mouse_enter(
    id: &str,
    screen_x: f32,
    screen_y: f32,
    button: u8,
    shift: bool,
    ctrl: bool,
    alt: bool,
) {
    EditorRuntime::with(|rt| {
        if let Err(e) = rt.handle_mouse_enter(id, screen_x, screen_y, button, shift, ctrl, alt) {
            crate::runtime::logging::error(&format!("Failed to handle mouse enter: {}", e));
        }
    });
}

#[wasm_bindgen]
pub fn handle_mouse_leave(
    id: &str,
    screen_x: f32,
    screen_y: f32,
    button: u8,
    shift: bool,
    ctrl: bool,
    alt: bool,
) {
    EditorRuntime::with(|rt| {
        if let Err(e) = rt.handle_mouse_leave(id, screen_x, screen_y, button, shift, ctrl, alt) {
            crate::runtime::logging::error(&format!("Failed to handle mouse leave: {}", e));
        }
    });
}

/// Dispatch a command from the frontend to the runtime.
///
/// This is the SINGLE ENTRY POINT for all user-initiated actions.
/// Frontend constructs command JSON and calls this function.
///
/// Commands are deserialized from JSON, executed, and the result is logged.
/// Errors do NOT panic - they are logged and the function returns normally.
#[wasm_bindgen]
pub fn dispatch_command(json: &str) {
    use crate::runtime::commands::{Command, dispatch};
    // crate::runtime::logging::debug(&format!("Received json command: {:?}", json));

    let command: Command = match serde_json::from_str(json) {
        Ok(cmd) => cmd,
        Err(e) => {
            crate::runtime::logging::error(&format!("Failed to deserialize command: {}", e));
            return;
        }
    };

    EditorRuntime::with(|rt| {
        if let Err(e) = dispatch(rt, command) {
            crate::runtime::logging::error(&format!("Command execution failed: {}", e));
        }
    });
}
