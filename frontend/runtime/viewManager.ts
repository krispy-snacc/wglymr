// View lifecycle: create → attach → setVisible → resize → requestRender → detach → destroy

import { getWasmModule } from "./editorRuntime";

export function createEditorView(viewId: string): void {
    const wasm = getWasmModule();
    if (!wasm) {
        console.warn("Cannot create view: runtime not initialized");
        return;
    }
    wasm.create_view(viewId);
}

export function attachEditorView(
    viewId: string,
    canvas: HTMLCanvasElement,
    width: number,
    height: number
): void {
    const wasm = getWasmModule();
    if (!wasm) {
        console.warn("Cannot attach view: runtime not initialized");
        return;
    }
    wasm.attach_view(viewId, canvas, width, height);
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
    width: number,
    height: number
): void {
    const wasm = getWasmModule();
    if (!wasm) {
        console.warn("Cannot resize view: runtime not initialized");
        return;
    }
    wasm.resize_view(viewId, width, height);
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
