use std::cell::RefCell;

pub mod api;
#[cfg(target_arch = "wasm32")]
pub mod browser_render_loop;
pub mod commands;
pub mod errors;
pub mod gpu;
pub mod logging;
pub mod scheduler;
pub mod view_registry;

#[cfg(target_arch = "wasm32")]
use browser_render_loop::BrowserRenderLoop as RenderLoop;
use gpu::GpuContext;
use scheduler::Scheduler;
use view_registry::ViewRegistry;

// EditorRuntime is stored in thread_local storage.
// In WASM, this is effectively a single global runtime.

pub struct EditorRuntime {
    gpu: Option<GpuContext>,
    views: ViewRegistry,
    scheduler: Scheduler,

    // Platform-specific render scheduler.
    // In WASM builds, this is requestAnimationFrame-based.
    // Native backends will provide a different implementation.
    render_loop: RenderLoop,
}

impl EditorRuntime {
    fn new() -> Self {
        Self {
            gpu: None,
            views: ViewRegistry::new(),
            scheduler: Scheduler::new(),
            render_loop: RenderLoop::new(),
        }
    }

    pub fn with<F, R>(f: F) -> R
    where
        F: FnOnce(&mut EditorRuntime) -> R,
    {
        RUNTIME.with(|rt| {
            let mut runtime = rt.borrow_mut();
            f(&mut runtime)
        })
    }

    pub fn gpu(&self) -> Option<&GpuContext> {
        self.gpu.as_ref()
    }

    pub fn gpu_mut(&mut self) -> Option<&mut GpuContext> {
        self.gpu.as_mut()
    }

    pub fn views(&self) -> &ViewRegistry {
        &self.views
    }

    pub fn views_mut(&mut self) -> &mut ViewRegistry {
        &mut self.views
    }

    pub fn scheduler(&self) -> &Scheduler {
        &self.scheduler
    }

    pub fn scheduler_mut(&mut self) -> &mut Scheduler {
        &mut self.scheduler
    }

    pub fn render_loop_mut(&mut self) -> &mut RenderLoop {
        &mut self.render_loop
    }

    /// Called every frame by the render loop.
    /// Checks if any views are dirty and renders them.
    /// Does nothing if no views are dirty.
    pub fn tick(&mut self) {
        if self.scheduler.dirty_views().count() > 0 {
            if let Err(e) = self.render_dirty_views() {
                logging::error(&format!("Tick render failed: {}", e));
            }
        }
    }
}

thread_local! {
    static RUNTIME: RefCell<EditorRuntime> = RefCell::new(EditorRuntime::new());
}
