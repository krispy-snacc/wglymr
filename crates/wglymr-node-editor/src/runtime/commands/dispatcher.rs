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

    let current_pan = view_state.view.pan();
    let current_zoom = view_state.view.zoom();

    let new_x = current_pan[0] + dx;
    let new_y = current_pan[1] + dy;

    runtime
        .views_mut()
        .set_view_camera(view_id, new_x, new_y, current_zoom)?;
    runtime.scheduler_mut().mark_dirty(view_id);

    logging::debug(&format!("ViewPan: {} ({}, {})", view_id, dx, dy));
    Ok(())
}

/// Handle ViewZoom command
fn handle_view_zoom(
    runtime: &mut EditorRuntime,
    view_id: &str,
    delta: f32,
    center_x: Option<f32>,
    center_y: Option<f32>,
) -> Result<(), RuntimeError> {
    let view_state = runtime
        .views()
        .get(view_id)
        .ok_or_else(|| RuntimeError::ViewNotFound(view_id.to_string()))?;

    let current_pan = view_state.view.pan();
    let zoom_old = view_state.view.zoom();
    let viewport_width = view_state.width as f32;
    let viewport_height = view_state.height as f32;

    const MIN_ZOOM: f32 = 0.1;
    const MAX_ZOOM: f32 = 10.0;
    let zoom_new = (zoom_old * delta).clamp(MIN_ZOOM, MAX_ZOOM);

    let (new_pan_x, new_pan_y) = if let (Some(cx), Some(cy)) = (center_x, center_y) {
        let viewport_center_x = viewport_width / 2.0;
        let viewport_center_y = viewport_height / 2.0;

        let world_x = (cx - viewport_center_x) / zoom_old + current_pan[0];
        let world_y = (cy - viewport_center_y) / zoom_old + current_pan[1];

        let new_pan_x = world_x - (cx - viewport_center_x) / zoom_new;
        let new_pan_y = world_y - (cy - viewport_center_y) / zoom_new;

        (new_pan_x, new_pan_y)
    } else {
        (current_pan[0], current_pan[1])
    };

    runtime
        .views_mut()
        .set_view_camera(view_id, new_pan_x, new_pan_y, zoom_new)?;
    runtime.scheduler_mut().mark_dirty(view_id);

    logging::debug(&format!("ViewZoom: {} ({})", view_id, delta));
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
