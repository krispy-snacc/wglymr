use serde::{Deserialize, Serialize};

use crate::SocketId;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct LinkId(pub u64);

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Link {
    pub id: LinkId,
    pub from: SocketId,
    pub to: SocketId,
}
