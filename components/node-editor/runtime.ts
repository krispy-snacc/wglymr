let wasmModule: any = null;
let initializationPromise: Promise<void> | null = null;
let isInitialized = false;

async function initializeRuntime(): Promise<void> {
    if (isInitialized) return;

    const wasm = await import("../../wasm-pkg/wglymr_node_editor.js");
    wasmModule = wasm;

    await wasm.default();
    await wasm.init_gpu();
    wasm.init_engine();
    wasm.start_render_loop();

    isInitialized = true;
}

export async function ensureRuntimeReady(): Promise<void> {
    if (isInitialized) return;

    if (!initializationPromise) {
        initializationPromise = initializeRuntime();
    }

    await initializationPromise;
}

export function createView(viewId: string): void {
    if (!wasmModule) throw new Error("Runtime not initialized");
    wasmModule.create_view(viewId);
}

export function destroyView(viewId: string): void {
    if (!wasmModule) throw new Error("Runtime not initialized");
    wasmModule.destroy_view(viewId);
}

export function attachView(
    viewId: string,
    canvas: HTMLCanvasElement,
    width: number,
    height: number
): void {
    if (!wasmModule) throw new Error("Runtime not initialized");
    wasmModule.attach_view(viewId, canvas, width, height);
}

export function detachView(viewId: string): void {
    if (!wasmModule) throw new Error("Runtime not initialized");
    wasmModule.detach_view(viewId);
}

export function resizeView(
    viewId: string,
    width: number,
    height: number
): void {
    if (!wasmModule) throw new Error("Runtime not initialized");
    wasmModule.resize_view(viewId, width, height);
}

export function setVisible(viewId: string, visible: boolean): void {
    if (!wasmModule) throw new Error("Runtime not initialized");
    wasmModule.set_visible(viewId, visible);
}

export function requestRender(viewId: string): void {
    if (!wasmModule) throw new Error("Runtime not initialized");
    wasmModule.request_render(viewId);
}
