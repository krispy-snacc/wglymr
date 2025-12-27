use std::fmt;

#[derive(Debug, Clone)]
pub enum RuntimeError {
    ViewNotFound(String),
    ViewAlreadyExists(String),
    GpuNotInitialized,
    GpuInitializationFailed(String),
    SurfaceCreationFailed(String),
    RenderFailed(String),
    InvalidState(String),
}

impl fmt::Display for RuntimeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RuntimeError::ViewNotFound(id) => write!(f, "View not found: {}", id),
            RuntimeError::ViewAlreadyExists(id) => write!(f, "View already exists: {}", id),
            RuntimeError::GpuNotInitialized => write!(f, "GPU not initialized"),
            RuntimeError::GpuInitializationFailed(msg) => write!(f, "GPU initialization failed: {}", msg),
            RuntimeError::SurfaceCreationFailed(msg) => write!(f, "Surface creation failed: {}", msg),
            RuntimeError::RenderFailed(msg) => write!(f, "Render failed: {}", msg),
            RuntimeError::InvalidState(msg) => write!(f, "Invalid state: {}", msg),
        }
    }
}

impl std::error::Error for RuntimeError {}
