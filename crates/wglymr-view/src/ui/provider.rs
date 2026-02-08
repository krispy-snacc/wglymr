use std::collections::HashMap;
use wglymr_color::Color;
use wglymr_document::{NodeDescriptor, SocketDescriptor};

use super::{NodeBodyUI, NodeHeaderUI, NodeUIDefinition, SocketUIDefinition, SocketVisualType};

pub trait NodeUIProvider {
    fn ui_definition(
        &self,
        node: &NodeDescriptor,
        socket_map: &HashMap<wglymr_document::SocketId, &SocketDescriptor>,
    ) -> NodeUIDefinition;
}

pub struct DefaultNodeUIProvider {
    pub header_height: f32,
    pub header_color: Color,
    pub body_color: Color,
    pub corner_radius: f32,
    pub padding: f32,
}

impl Default for DefaultNodeUIProvider {
    fn default() -> Self {
        Self {
            header_height: 30.0,
            header_color: Color::hex(0x3A3A3A),
            body_color: Color::hex(0x2A2A2A),
            corner_radius: 4.0,
            padding: 10.0,
        }
    }
}

impl NodeUIProvider for DefaultNodeUIProvider {
    fn ui_definition(
        &self,
        node: &NodeDescriptor,
        socket_map: &HashMap<wglymr_document::SocketId, &SocketDescriptor>,
    ) -> NodeUIDefinition {
        let header = NodeHeaderUI::new(
            node.node_kind.clone(),
            self.header_color,
            self.header_height,
        );

        let body = NodeBodyUI::new(self.body_color, self.corner_radius, self.padding);

        let inputs = node
            .inputs
            .iter()
            .filter_map(|socket_id| {
                socket_map.get(socket_id).map(|socket| {
                    SocketUIDefinition::new(
                        *socket_id,
                        socket.name.clone(),
                        SocketVisualType::Circle,
                    )
                })
            })
            .collect();

        let outputs = node
            .outputs
            .iter()
            .filter_map(|socket_id| {
                socket_map.get(socket_id).map(|socket| {
                    SocketUIDefinition::new(
                        *socket_id,
                        socket.name.clone(),
                        SocketVisualType::Circle,
                    )
                })
            })
            .collect();

        NodeUIDefinition::new(header, body, inputs, outputs)
    }
}
