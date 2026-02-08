mod dispatcher;

pub use dispatcher::dispatch;

use serde::{Deserialize, Serialize};

/// Runtime command - represents user intent that mutates editor state.
///
/// Commands are:
/// - Serializable (can cross WASM boundary)
/// - Stateless (no runtime references)
/// - Deterministic (same command + same state = same result)
/// - Tagged with "type" for JSON deserialization from frontend
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum Command {
    #[serde(rename = "view.pan")]
    ViewPan {
        #[serde(rename = "viewId")]
        view_id: String,
        dx: f32,
        dy: f32,
    },

    #[serde(rename = "view.zoom")]
    ViewZoom {
        #[serde(rename = "viewId")]
        view_id: String,
        delta: f32,
        #[serde(rename = "centerX", skip_serializing_if = "Option::is_none")]
        center_x: Option<f32>,
        #[serde(rename = "centerY", skip_serializing_if = "Option::is_none")]
        center_y: Option<f32>,
    },

    #[serde(rename = "view.reset")]
    ViewReset {
        #[serde(rename = "viewId")]
        view_id: String,
    },

    #[serde(rename = "node.add")]
    NodeAdd {
        #[serde(rename = "nodeType")]
        node_type: String,
        x: f32,
        y: f32,
    },
}
