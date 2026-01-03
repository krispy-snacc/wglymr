use crate::Color;

pub struct Theme {
    pub node_background: Color,
    pub node_border: Color,
    pub text_primary: Color,
}

impl Default for Theme {
    fn default() -> Self {
        Self {
            node_background: Color::NODE_BG,
            node_border: Color::NODE_BORDER,
            text_primary: Color::TEXT_PRIMARY,
        }
    }
}
