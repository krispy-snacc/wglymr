use std::collections::HashSet;

pub struct Scheduler {
    dirty_views: HashSet<String>,
}

impl Default for Scheduler {
    fn default() -> Self {
        Self::new()
    }
}

impl Scheduler {
    pub fn new() -> Self {
        Self {
            dirty_views: HashSet::new(),
        }
    }

    pub fn mark_dirty(&mut self, view_id: &str) {
        self.dirty_views.insert(view_id.to_string());
    }

    pub fn is_dirty(&self, view_id: &str) -> bool {
        self.dirty_views.contains(view_id)
    }

    pub fn clear_dirty(&mut self, view_id: &str) {
        self.dirty_views.remove(view_id);
    }

    pub fn clear_all(&mut self) {
        self.dirty_views.clear();
    }

    pub fn dirty_views(&self) -> impl Iterator<Item = &String> {
        self.dirty_views.iter()
    }
}
