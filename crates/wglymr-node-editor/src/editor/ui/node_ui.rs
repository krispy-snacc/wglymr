use wglymr_color::Color;

use super::socket_ui::SocketUIDefinition;

#[derive(Debug, Clone)]
pub struct NodeHeaderUI {
    pub title: String,
    pub color: Color,
    pub height: f32,
    pub icon: Option<IconId>,
}

impl NodeHeaderUI {
    pub fn new(title: String, color: Color, height: f32) -> Self {
        Self {
            title,
            color,
            height,
            icon: None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct IconId(pub u32);

#[derive(Debug, Clone)]
pub struct NodeBodyUI {
    pub background: Color,
    pub corner_radius: f32,
    pub padding: f32,
}

impl NodeBodyUI {
    pub fn new(background: Color, corner_radius: f32, padding: f32) -> Self {
        Self {
            background,
            corner_radius,
            padding,
        }
    }
}

#[derive(Debug, Clone)]
pub struct NodeUIDefinition {
    pub header: NodeHeaderUI,
    pub body: NodeBodyUI,
    pub inputs: Vec<SocketUIDefinition>,
    pub outputs: Vec<SocketUIDefinition>,
}

impl NodeUIDefinition {
    pub fn new(
        header: NodeHeaderUI,
        body: NodeBodyUI,
        inputs: Vec<SocketUIDefinition>,
        outputs: Vec<SocketUIDefinition>,
    ) -> Self {
        Self {
            header,
            body,
            inputs,
            outputs,
        }
    }
}
