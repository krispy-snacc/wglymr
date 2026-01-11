use std::cell::RefCell;

use crate::engine::EditorEngine;

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
use view_registry::GpuViewRegistry;

pub struct EditorRuntime {
    gpu: Option<GpuContext>,
    gpu_views: GpuViewRegistry,
    scheduler: Scheduler,
    engine: EditorEngine,

    #[cfg(target_arch = "wasm32")]
    render_loop: RenderLoop,
}

impl EditorRuntime {
    fn new() -> Self {
        // TEMPORARY: Using test adapter for visual validation
        // TODO: Replace with real graph adapter when integration is complete
        let document = Box::new(crate::document::test_adapter::TestDocumentAdapter::new());
        let engine = EditorEngine::new(document);

        Self {
            gpu: None,
            gpu_views: GpuViewRegistry::new(),
            scheduler: Scheduler::new(),
            engine,
            #[cfg(target_arch = "wasm32")]
            render_loop: RenderLoop::new(),
        }
    }

    pub fn with<R, F>(f: F) -> R
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

    pub fn gpu_views(&self) -> &GpuViewRegistry {
        &self.gpu_views
    }

    pub fn gpu_views_mut(&mut self) -> &mut GpuViewRegistry {
        &mut self.gpu_views
    }

    pub fn engine(&self) -> &EditorEngine {
        &self.engine
    }

    pub fn engine_mut(&mut self) -> &mut EditorEngine {
        &mut self.engine
    }

    pub fn scheduler(&self) -> &Scheduler {
        &self.scheduler
    }

    pub fn scheduler_mut(&mut self) -> &mut Scheduler {
        &mut self.scheduler
    }

    #[cfg(target_arch = "wasm32")]
    pub fn render_loop_mut(&mut self) -> &mut RenderLoop {
        &mut self.render_loop
    }

    /// Called every frame by the render loop.
    /// Blender-exact redraw scheduling:
    /// - Modal operator active → render ALL views EVERY FRAME
    /// - Operator just finished → render ALL views ONCE
    /// - Otherwise → render only dirty views
    pub fn tick(&mut self) {
        let modal_active = self.engine.is_modal_active();
        let operator_finished = self.engine.operator_just_finished();

        if modal_active {
            // logging::warn("Refreshing views: Modal Active");
            if let Err(e) = self.render_all_views() {
                logging::error(&format!("Modal render failed: {}", e));
            }
        } else if operator_finished {
            // logging::warn("Refreshing views: Operator Finished");
            if let Err(e) = self.render_all_views() {
                logging::error(&format!("Finish render failed: {}", e));
            }
            self.engine.clear_operator_finished_flag();
        } else if self.scheduler.dirty_views().count() > 0 {
            // logging::warn("Refreshing views: View Dirty");
            if let Err(e) = self.render_dirty_views() {
                logging::error(&format!("Dirty render failed: {}", e));
            }
        }
    }
}

thread_local! {
    static RUNTIME: RefCell<EditorRuntime> = RefCell::new(EditorRuntime::new());
}
