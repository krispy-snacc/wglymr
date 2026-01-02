use super::Command;
use crate::runtime::{EditorRuntime, errors::RuntimeError, logging};

/// Dispatch a command to the runtime.
///
/// This is the SINGLE ENTRY POINT for all command execution.
/// Routes commands to appropriate handlers and manages invalidation.
///
/// Returns Ok(()) on success, Err on validation failure.
pub fn dispatch(runtime: &mut EditorRuntime, command: Command) -> Result<(), RuntimeError> {
    logging::debug(&format!("Received command: {:?}", command));

    match command {
        Command::ViewPan { view_id, dx, dy } => handle_view_pan(runtime, &view_id, dx, dy),
        Command::ViewZoom {
            view_id,
            delta,
            center_x,
            center_y,
        } => handle_view_zoom(runtime, &view_id, delta, center_x, center_y),
        Command::ViewReset { view_id } => handle_view_reset(runtime, &view_id),
        Command::NodeAdd { node_type, x, y } => handle_node_add(runtime, &node_type, x, y),
    }
}

/// Handle ViewPan command
fn handle_view_pan(
    runtime: &mut EditorRuntime,
    view_id: &str,
    dx: f32,
    dy: f32,
) -> Result<(), RuntimeError> {
    let view_state = runtime
        .views()
        .get(view_id)
        .ok_or_else(|| RuntimeError::ViewNotFound(view_id.to_string()))?;

    let pan = view_state.view.pan();
    let zoom = view_state.view.zoom();

    // Convert screen-space delta => world-space delta
    let world_dx = dx / zoom;
    let world_dy = dy / zoom;

    // Invert direction: dragging right moves world left
    let new_pan_x = pan[0] - world_dx;
    let new_pan_y = pan[1] - world_dy;

    runtime
        .views_mut()
        .set_view_camera(view_id, new_pan_x, new_pan_y, zoom)?;
    runtime.scheduler_mut().mark_dirty(view_id);

    Ok(())
}

/// Handle ViewZoom command
fn handle_view_zoom(
    runtime: &mut EditorRuntime,
    view_id: &str,
    zoom_factor: f32,
    cursor_x: Option<f32>,
    cursor_y: Option<f32>,
) -> Result<(), RuntimeError> {
    let state = runtime
        .views()
        .get(view_id)
        .ok_or_else(|| RuntimeError::ViewNotFound(view_id.to_string()))?;

    let zoom_old = state.view.zoom();
    let pan_world = state.view.pan(); // NOW interpreted as world-space center
    let width = state.view.width() as f32;
    let height = state.view.height() as f32;

    const MIN_ZOOM: f32 = 0.1;
    const MAX_ZOOM: f32 = 10.0;

    let zoom_new = (zoom_old * zoom_factor).clamp(MIN_ZOOM, MAX_ZOOM);

    // If no cursor, zoom around center
    let (cx, cy) = match (cursor_x, cursor_y) {
        (Some(x), Some(y)) => (x, y),
        _ => (width * 0.5, height * 0.5),
    };

    // Convert cursor from screen => world (BEFORE zoom)
    let world_under_cursor_x = (cx - width * 0.5) / zoom_old + pan_world[0];
    let world_under_cursor_y = (cy - height * 0.5) / zoom_old + pan_world[1];

    // Adjust pan so the same world point stays under cursor
    let new_pan_x = world_under_cursor_x - (cx - width * 0.5) / zoom_new;
    let new_pan_y = world_under_cursor_y - (cy - height * 0.5) / zoom_new;

    runtime
        .views_mut()
        .set_view_camera(view_id, new_pan_x, new_pan_y, zoom_new)?;

    runtime.scheduler_mut().mark_dirty(view_id);

    Ok(())
}

/// Handle ViewReset command
fn handle_view_reset(runtime: &mut EditorRuntime, view_id: &str) -> Result<(), RuntimeError> {
    runtime
        .views()
        .get(view_id)
        .ok_or_else(|| RuntimeError::ViewNotFound(view_id.to_string()))?;

    runtime
        .views_mut()
        .set_view_camera(view_id, 0.0, 0.0, 1.0)?;
    runtime.scheduler_mut().mark_dirty(view_id);

    logging::debug(&format!("ViewReset: {}", view_id));
    Ok(())
}

/// Handle NodeAdd command
fn handle_node_add(
    runtime: &mut EditorRuntime,
    node_type: &str,
    x: f32,
    y: f32,
) -> Result<(), RuntimeError> {
    // TODO: Apply command to document when document integration is complete
    // For now, just mark all views dirty to demonstrate the pipeline works

    // Collect view IDs first to avoid borrow checker issues
    let view_ids: Vec<String> = runtime.views().iter().map(|(id, _)| id.clone()).collect();

    // Mark ALL views dirty since document changed
    for view_id in view_ids {
        runtime.scheduler_mut().mark_dirty(&view_id);
    }

    logging::log(&format!("NodeAdd: {} at ({}, {})", node_type, x, y));
    Ok(())
}
