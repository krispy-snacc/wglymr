use std::collections::HashMap;

use crate::document::adapter::DocumentAdapter;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ViewId(String);

impl ViewId {
    pub fn new(id: String) -> Self {
        Self(id)
    }
}

pub struct EditorView {
    pan: [f32; 2],
    zoom: f32,
    width: u32,
    height: u32,
}

impl EditorView {
    fn new() -> Self {
        Self {
            pan: [0.0, 0.0],
            zoom: 1.0,
            width: 800,
            height: 600,
        }
    }

    fn resize(&mut self, width: u32, height: u32) {
        self.width = width;
        self.height = height;
    }

    fn set_camera(&mut self, pan: [f32; 2], zoom: f32) {
        self.pan = pan;
        self.zoom = zoom;
    }
}

pub struct EditorEngine {
    views: HashMap<ViewId, EditorView>,
    document: Box<dyn DocumentAdapter>,
}

impl EditorEngine {
    pub fn new(document: Box<dyn DocumentAdapter>) -> Self {
        Self {
            views: HashMap::new(),
            document,
        }
    }

    pub fn create_view(&mut self, view_id: ViewId) {
        self.views.insert(view_id, EditorView::new());
    }

    pub fn destroy_view(&mut self, view_id: ViewId) {
        self.views.remove(&view_id);
    }

    pub fn resize_view(&mut self, view_id: ViewId, width: u32, height: u32) {
        if let Some(view) = self.views.get_mut(&view_id) {
            view.resize(width, height);
        }
    }

    pub fn set_view_camera(&mut self, view_id: ViewId, pan: [f32; 2], zoom: f32) {
        if let Some(view) = self.views.get_mut(&view_id) {
            view.set_camera(pan, zoom);
        }
    }

    pub fn draw_view(&mut self, view_id: ViewId) {
        if self.views.contains_key(&view_id) {
            // Rendering orchestration placeholder
        }
    }
}
