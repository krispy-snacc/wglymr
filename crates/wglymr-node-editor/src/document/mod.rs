// Document state management
// Handles the authoritative graph state, independent of UI

pub mod adapter;
pub mod commands;
pub mod descriptors;

// TEMPORARY: Test adapter for visual validation
// Remove when real graph integration is complete
pub mod test_adapter;
