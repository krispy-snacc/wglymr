use super::font::FontFace;
use std::collections::HashMap;

/// Multi-font registry for future extensibility
/// Currently supports single font (Roboto)
pub struct FontRegistry {
    fonts: HashMap<String, FontFace>,
    default_font: String,
}

impl FontRegistry {
    pub fn new() -> Self {
        Self {
            fonts: HashMap::new(),
            default_font: String::new(),
        }
    }

    pub fn register_font(&mut self, font: FontFace) {
        let name = font.name().to_string();
        if self.fonts.is_empty() {
            self.default_font = name.clone();
        }
        self.fonts.insert(name, font);
    }

    pub fn get_font(&self, name: &str) -> Option<&FontFace> {
        self.fonts.get(name)
    }

    pub fn default_font(&self) -> Option<&FontFace> {
        self.fonts.get(&self.default_font)
    }
}

impl Default for FontRegistry {
    fn default() -> Self {
        Self::new()
    }
}
