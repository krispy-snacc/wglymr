// View lifecycle: create => attach => setVisible => resize => requestRender => detach => destroy

import { getWasmModule } from "./editorRuntime";

export function createEditorView(viewId: string): void {
    const wasm = getWasmModule();
    if (!wasm) {
        console.warn("Cannot create view: runtime not initialized");
        return;
    }
    wasm.create_view(viewId);
}

/**
 * Compute backing scale for HiDPI support and browser zoom handling.
 *
 * Clamps to [1.0, 2.0] to prevent excessive memory usage while maintaining
 * sharp rendering under typical zoom levels and display densities.
 */
function computeBackingScale(): number {
    const dpr = window.devicePixelRatio || 1.0;
    // Optional render_scale multiplier (default 1.0)
    const renderScale = 1.0;
    return Math.max(1.0, Math.min(2.0, dpr * renderScale));
}

export function attachEditorView(
    viewId: string,
    canvas: HTMLCanvasElement,
    cssWidth: number,
    cssHeight: number
): void {
    const wasm = getWasmModule();
    if (!wasm) {
        console.warn("Cannot attach view: runtime not initialized");
        return;
    }

    const backingScale = computeBackingScale();

    // Set CSS size (what the browser layout sees)
    canvas.style.width = `${cssWidth}px`;
    canvas.style.height = `${cssHeight}px`;

    // Set backing size (what WebGPU renders to)
    canvas.width = Math.floor(cssWidth * backingScale);
    canvas.height = Math.floor(cssHeight * backingScale);

    wasm.attach_view(viewId, canvas, cssWidth, cssHeight, backingScale);
}

export function setEditorViewVisible(viewId: string, visible: boolean): void {
    const wasm = getWasmModule();
    if (!wasm) {
        console.warn("Cannot set visibility: runtime not initialized");
        return;
    }
    wasm.set_visible(viewId, visible);
}

export function resizeEditorView(
    viewId: string,
    cssWidth: number,
    cssHeight: number
): void {
    const wasm = getWasmModule();
    if (!wasm) {
        console.warn("Cannot resize view: runtime not initialized");
        return;
    }

    const backingScale = computeBackingScale();

    const canvas = document.querySelector(
        `#node-editor-canvas-${viewId}`
    ) as HTMLCanvasElement;
    if (canvas) {
        canvas.style.width = `${cssWidth}px`;
        canvas.style.height = `${cssHeight}px`;
        canvas.width = Math.floor(cssWidth * backingScale);
        canvas.height = Math.floor(cssHeight * backingScale);
    }

    wasm.resize_view(viewId, cssWidth, cssHeight, backingScale);
}

export function detachEditorView(viewId: string): void {
    const wasm = getWasmModule();
    if (!wasm) {
        console.warn("Cannot detach view: runtime not initialized");
        return;
    }
    wasm.detach_view(viewId);
}

export function destroyEditorView(viewId: string): void {
    const wasm = getWasmModule();
    if (!wasm) {
        console.warn("Cannot destroy view: runtime not initialized");
        return;
    }
    wasm.destroy_view(viewId);
}

// TODO: Determine if this belongs in view lifecycle or render scheduler.
export function requestEditorViewRender(viewId: string): void {
    const wasm = getWasmModule();
    if (!wasm) {
        console.warn("Cannot request render: runtime not initialized");
        return;
    }
    wasm.request_render(viewId);
}
