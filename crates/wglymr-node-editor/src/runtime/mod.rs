use std::cell::RefCell;

pub mod api;
pub mod errors;
pub mod gpu;
pub mod logging;
pub mod scheduler;
pub mod view_registry;

use gpu::GpuContext;
use scheduler::Scheduler;
use view_registry::ViewRegistry;

// EditorRuntime is stored in thread_local storage.
// In WASM, this is effectively a single global runtime.

pub struct EditorRuntime {
    gpu: Option<GpuContext>,
    views: ViewRegistry,
    scheduler: Scheduler,
}

impl EditorRuntime {
    fn new() -> Self {
        Self {
            gpu: None,
            views: ViewRegistry::new(),
            scheduler: Scheduler::new(),
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
}

thread_local! {
    static RUNTIME: RefCell<EditorRuntime> = RefCell::new(EditorRuntime::new());
}
