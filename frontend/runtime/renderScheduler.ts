// Centralized render request layer.
// Rendering is event-driven - views only render when explicitly requested.
// Panels must not drive rendering directly - use this scheduler.

import { getWasmModule } from "./editorRuntime";

// Request a render for the specified view.
// TODO: Add scheduling logic (throttling, prioritization, etc.).
export function requestEditorRender(viewId: string): void {
    const wasm = getWasmModule();
    if (!wasm) {
        console.warn("Cannot request render: runtime not initialized");
        return;
    }
    wasm.request_render(viewId);
}
