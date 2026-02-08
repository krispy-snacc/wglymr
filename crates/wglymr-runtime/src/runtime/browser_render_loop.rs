use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

use super::logging;

/// Manages the requestAnimationFrame render loop.
///
/// SAFETY:
/// - WASM is single-threaded, so no additional synchronization is needed
/// - The closure must be retained to prevent JS from dropping the callback
/// - This is platform-specific (browser RAF) and will be abstracted for native backends
///
/// LIFECYCLE:
/// - start() creates and stores a closure, then schedules the first RAF
/// - The callback invokes EditorRuntime::tick()
/// - The callback re-schedules itself ONLY if running == true
/// - stop() sets running = false, preventing re-scheduling
pub struct BrowserRenderLoop {
    running: Rc<RefCell<bool>>,
    closure: Rc<RefCell<Option<Closure<dyn FnMut()>>>>,
}

impl BrowserRenderLoop {
    pub fn new() -> Self {
        Self {
            running: Rc::new(RefCell::new(false)),
            closure: Rc::new(RefCell::new(None)),
        }
    }

    /// Start the render loop.
    /// Does nothing if already running.
    pub fn start(&mut self) {
        if *self.running.borrow() {
            return;
        }

        *self.running.borrow_mut() = true;
        logging::log("Render loop started");

        let running_flag = self.running.clone();
        let closure_container = self.closure.clone();

        let callback = Closure::wrap(Box::new(move || {
            // Call the runtime tick
            super::EditorRuntime::with(|rt| rt.tick());

            // Re-schedule if still running
            if *running_flag.borrow() {
                let window = web_sys::window().expect("RenderLoop: window object unavailable");
                let closure_ref = closure_container.borrow();
                if let Some(closure) = closure_ref.as_ref() {
                    window
                        .request_animation_frame(closure.as_ref().unchecked_ref())
                        .expect("RenderLoop: requestAnimationFrame failed");
                }
            }
        }) as Box<dyn FnMut()>);

        // Store the closure BEFORE scheduling RAF to prevent race condition
        *self.closure.borrow_mut() = Some(callback);

        // Schedule the first RAF using the stored closure
        let window = web_sys::window().expect("RenderLoop: window object unavailable");
        let closure_ref = self.closure.borrow();
        if let Some(closure) = closure_ref.as_ref() {
            window
                .request_animation_frame(closure.as_ref().unchecked_ref())
                .expect("RenderLoop: requestAnimationFrame failed");
        }
    }

    /// Stop the render loop.
    /// The loop will naturally stop re-scheduling itself.
    pub fn stop(&mut self) {
        if !*self.running.borrow() {
            return;
        }

        *self.running.borrow_mut() = false;
        logging::log("Render loop stopped");
    }
}
