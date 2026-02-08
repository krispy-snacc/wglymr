use super::Command;
use crate::runtime::{errors::RuntimeError, logging, EditorRuntime};
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

fn handle_view_pan(
    runtime: &mut EditorRuntime,
    view_id: &str,
    dx: f32,
    dy: f32,
) -> Result<(), RuntimeError> {
    let engine_view_id = wglymr_app::ViewId::new(view_id.to_string());

    let view = runtime
        .engine()
        .get_view(&engine_view_id)
        .ok_or_else(|| RuntimeError::ViewNotFound(view_id.to_string()))?;

    let pan = view.pan();
    let zoom = view.zoom();
    let s = view.backing_scale();

    let world_dx = dx * s / zoom;
    let world_dy = dy * s / zoom;
    let new_pan_x = pan[0] - world_dx;
    let new_pan_y = pan[1] - world_dy;

    runtime
        .engine_mut()
        .set_view_camera(&engine_view_id, new_pan_x, new_pan_y, zoom);
    runtime.scheduler_mut().mark_dirty(view_id);

    Ok(())
}

fn handle_view_zoom(
    runtime: &mut EditorRuntime,
    view_id: &str,
    zoom_factor: f32,
    cursor_x: Option<f32>,
    cursor_y: Option<f32>,
) -> Result<(), RuntimeError> {
    let engine_view_id = wglymr_app::ViewId::new(view_id.to_string());

    let view = runtime
        .engine()
        .get_view(&engine_view_id)
        .ok_or_else(|| RuntimeError::ViewNotFound(view_id.to_string()))?;

    let zoom_old = view.zoom();
    let pan_world = view.pan();
    let width = view.backing_width() as f32;
    let height = view.backing_height() as f32;

    const MIN_ZOOM: f32 = 0.1;
    const MAX_ZOOM: f32 = 10.0;

    let zoom_new = (zoom_old * zoom_factor).clamp(MIN_ZOOM, MAX_ZOOM);
    let s = view.backing_scale();

    let (cx, cy) = match (cursor_x, cursor_y) {
        (Some(x), Some(y)) => (x * s, y * s),
        _ => (width * 0.5, height * 0.5),
    };

    let world_under_cursor_x = (cx - width * 0.5) / zoom_old + pan_world[0];
    let world_under_cursor_y = (cy - height * 0.5) / zoom_old + pan_world[1];

    let new_pan_x = world_under_cursor_x - (cx - width * 0.5) / zoom_new;
    let new_pan_y = world_under_cursor_y - (cy - height * 0.5) / zoom_new;

    runtime
        .engine_mut()
        .set_view_camera(&engine_view_id, new_pan_x, new_pan_y, zoom_new);

    runtime.scheduler_mut().mark_dirty(view_id);

    Ok(())
}

fn handle_view_reset(runtime: &mut EditorRuntime, view_id: &str) -> Result<(), RuntimeError> {
    let engine_view_id = wglymr_app::ViewId::new(view_id.to_string());

    runtime
        .engine()
        .get_view(&engine_view_id)
        .ok_or_else(|| RuntimeError::ViewNotFound(view_id.to_string()))?;

    runtime
        .engine_mut()
        .set_view_camera(&engine_view_id, 0.0, 0.0, 1.0);
    runtime.scheduler_mut().mark_dirty(view_id);

    logging::debug(&format!("ViewReset: {}", view_id));
    Ok(())
}

fn handle_node_add(
    runtime: &mut EditorRuntime,
    node_type: &str,
    x: f32,
    y: f32,
) -> Result<(), RuntimeError> {
    let view_ids: Vec<String> = runtime
        .gpu_views()
        .iter()
        .map(|(id, _)| id.as_str().to_string())
        .collect();
    for view_id in view_ids {
        runtime.scheduler_mut().mark_dirty(&view_id);
    }

    logging::log(&format!("NodeAdd: {} at ({}, {})", node_type, x, y));
    Ok(())
}
