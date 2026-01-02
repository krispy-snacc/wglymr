use std::collections::HashMap;

use super::layout::TextLayout;
use super::model::{RenderText, TextStyle};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TextLayoutKey {
    pub text: String,
    pub font_size_tenths: u32,
}

impl TextLayoutKey {
    pub fn new(text: &str, font_size: f32) -> Self {
        Self {
            text: text.to_string(),
            font_size_tenths: (font_size * 10.0) as u32,
        }
    }
}

pub struct TextLayoutCache {
    cache: HashMap<TextLayoutKey, RenderText>,
    max_entries: usize,
}

impl Default for TextLayoutCache {
    fn default() -> Self {
        Self::new(1024)
    }
}

impl TextLayoutCache {
    pub fn new(max_entries: usize) -> Self {
        Self {
            cache: HashMap::new(),
            max_entries,
        }
    }

    pub fn get_or_layout(
        &mut self,
        layout: &TextLayout,
        text: &str,
        style: &TextStyle,
    ) -> RenderText {
        let key = TextLayoutKey::new(text, style.font_size);

        if let Some(cached) = self.cache.get(&key) {
            let mut result = cached.clone();
            result.style = *style;
            return result;
        }

        if self.cache.len() >= self.max_entries {
            self.cache.clear();
        }

        let render_text = layout.layout_text(text, style);
        self.cache.insert(key, render_text.clone());
        render_text
    }

    pub fn get_or_layout_at(
        &mut self,
        layout: &TextLayout,
        text: &str,
        position: [f32; 2],
        style: &TextStyle,
    ) -> RenderText {
        let base = self.get_or_layout(layout, text, style);
        base.offset(position)
    }

    pub fn invalidate(&mut self, text: &str, font_size: f32) {
        let key = TextLayoutKey::new(text, font_size);
        self.cache.remove(&key);
    }

    pub fn clear(&mut self) {
        self.cache.clear();
    }

    pub fn len(&self) -> usize {
        self.cache.len()
    }

    pub fn is_empty(&self) -> bool {
        self.cache.is_empty()
    }
}
