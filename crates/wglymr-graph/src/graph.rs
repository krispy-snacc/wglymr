use std::collections::HashMap;

use glam::Vec2;
use serde::{Deserialize, Serialize};

use crate::{
    GraphError, Link, LinkId, Node, NodeId, NodeKind, Socket, SocketDirection, SocketId, ValueType,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Graph {
    nodes: HashMap<NodeId, Node>,
    sockets: HashMap<SocketId, Socket>,
    links: HashMap<LinkId, Link>,

    next_node_id: u64,
    next_socket_id: u64,
    next_link_id: u64,
}

impl Graph {
    pub fn new() -> Self {
        Self {
            nodes: HashMap::new(),
            sockets: HashMap::new(),
            links: HashMap::new(),
            next_node_id: 0,
            next_socket_id: 0,
            next_link_id: 0,
        }
    }

    pub fn add_node(
        &mut self,
        kind: NodeKind,
        position: Vec2,
        inputs: Vec<(String, ValueType)>,
        outputs: Vec<(String, ValueType)>,
    ) -> NodeId {
        let node_id = NodeId(self.next_node_id);
        self.next_node_id += 1;

        let mut input_ids = Vec::new();
        let mut output_ids = Vec::new();

        for (name, value_type) in inputs {
            let socket_id = SocketId(self.next_socket_id);
            self.next_socket_id += 1;

            self.sockets.insert(
                socket_id,
                Socket {
                    id: socket_id,
                    node: node_id,
                    direction: SocketDirection::Input,
                    value_type,
                    name,
                },
            );
            input_ids.push(socket_id);
        }

        for (name, value_type) in outputs {
            let socket_id = SocketId(self.next_socket_id);
            self.next_socket_id += 1;

            self.sockets.insert(
                socket_id,
                Socket {
                    id: socket_id,
                    node: node_id,
                    direction: SocketDirection::Output,
                    value_type,
                    name,
                },
            );
            output_ids.push(socket_id);
        }

        let node = Node {
            id: node_id,
            kind,
            inputs: input_ids,
            outputs: output_ids,
            position,
        };

        self.nodes.insert(node_id, node);
        node_id
    }

    pub fn connect(&mut self, from: SocketId, to: SocketId) -> Result<LinkId, GraphError> {
        let from_socket = self
            .sockets
            .get(&from)
            .ok_or(GraphError::SocketNotFound { socket: from })?;
        let to_socket = self
            .sockets
            .get(&to)
            .ok_or(GraphError::SocketNotFound { socket: to })?;

        if from_socket.direction != SocketDirection::Output {
            return Err(GraphError::WrongDirection {
                expected: SocketDirection::Output,
                found: from_socket.direction,
            });
        }
        if to_socket.direction != SocketDirection::Input {
            return Err(GraphError::WrongDirection {
                expected: SocketDirection::Input,
                found: to_socket.direction,
            });
        }

        if from_socket.value_type != to_socket.value_type {
            return Err(GraphError::TypeMismatch {
                from: from_socket.value_type,
                to: to_socket.value_type,
            });
        }

        if self.links_into(to).next().is_some() {
            return Err(GraphError::InputAlreadyConnected);
        }

        let link_id = LinkId(self.next_link_id);
        self.next_link_id += 1;

        let link = Link {
            id: link_id,
            from,
            to,
        };

        self.links.insert(link_id, link);
        Ok(link_id)
    }

    pub fn disconnect(&mut self, link: LinkId) -> bool {
        self.links.remove(&link).is_some()
    }

    pub fn node(&self, id: NodeId) -> Option<&Node> {
        self.nodes.get(&id)
    }

    pub fn socket(&self, id: SocketId) -> Option<&Socket> {
        self.sockets.get(&id)
    }

    pub fn link(&self, id: LinkId) -> Option<&Link> {
        self.links.get(&id)
    }

    pub fn links_into(&self, socket: SocketId) -> impl Iterator<Item = &Link> {
        self.links.values().filter(move |link| link.to == socket)
    }

    pub fn links_out_of(&self, socket: SocketId) -> impl Iterator<Item = &Link> {
        self.links.values().filter(move |link| link.from == socket)
    }

    #[cfg(feature = "debug-graph")]
    pub fn check_invariants(&self) -> Result<(), GraphError> {
        for socket in self.sockets.values() {
            if !self.nodes.contains_key(&socket.node) {
                return Err(GraphError::NodeNotFound { node: socket.node });
            }
        }

        for link in self.links.values() {
            if !self.sockets.contains_key(&link.from) {
                return Err(GraphError::SocketNotFound { socket: link.from });
            }
            if !self.sockets.contains_key(&link.to) {
                return Err(GraphError::SocketNotFound { socket: link.to });
            }

            let from_socket = &self.sockets[&link.from];
            let to_socket = &self.sockets[&link.to];

            if from_socket.direction != SocketDirection::Output {
                return Err(GraphError::WrongDirection {
                    expected: SocketDirection::Output,
                    found: from_socket.direction,
                });
            }

            if to_socket.direction != SocketDirection::Input {
                return Err(GraphError::WrongDirection {
                    expected: SocketDirection::Input,
                    found: to_socket.direction,
                });
            }
        }

        Ok(())
    }
}

impl Default for Graph {
    fn default() -> Self {
        Self::new()
    }
}
