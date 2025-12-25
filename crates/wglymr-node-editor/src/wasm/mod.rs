use wasm_bindgen::prelude::*;

use crate::document::adapter::BasicDocumentAdapter;
use crate::engine::{EditorEngine, ViewId};

static mut ENGINE: Option<EditorEngine> = None;

#[wasm_bindgen]
pub fn init_engine() {
    let adapter = BasicDocumentAdapter::new();
    let engine = EditorEngine::new(Box::new(adapter));
    unsafe {
        ENGINE = Some(engine);
    }
}

#[wasm_bindgen]
pub fn create_view(view_id: &str) {
    unsafe {
        if let Some(engine) = ENGINE.as_mut() {
            let id = ViewId::new(view_id.to_string());
            engine.create_view(id);
        }
    }
}

#[wasm_bindgen]
pub fn destroy_view(view_id: &str) {
    unsafe {
        if let Some(engine) = ENGINE.as_mut() {
            let id = ViewId::new(view_id.to_string());
            engine.destroy_view(id);
        }
    }
}

#[wasm_bindgen]
pub fn resize_view(view_id: &str, width: u32, height: u32) {
    unsafe {
        if let Some(engine) = ENGINE.as_mut() {
            let id = ViewId::new(view_id.to_string());
            engine.resize_view(id, width, height);
        }
    }
}

#[wasm_bindgen]
pub fn set_view_camera(view_id: &str, pan_x: f32, pan_y: f32, zoom: f32) {
    unsafe {
        if let Some(engine) = ENGINE.as_mut() {
            let id = ViewId::new(view_id.to_string());
            engine.set_view_camera(id, [pan_x, pan_y], zoom);
        }
    }
}
