// Global editor runtime lifecycle.
// Panels must not initialize WASM directly - use ensureEditorRuntimeReady().

let wasmModule: any = null;
let initializationPromise: Promise<void> | null = null;
let isInitialized = false;

async function initializeRuntime(): Promise<void> {
    if (isInitialized) return;

    try {
        const wasm = await import("../wasm-pkg/wglymr_frontend_web.js");
        wasmModule = wasm;

        await wasm.default();
        await wasm.init_gpu();
        wasm.init_engine();
        wasm.start_render_loop();

        isInitialized = true;
    } catch (error) {
        console.error("[EditorRuntime] Failed to initialize:", error);
        initializationPromise = null;
        throw error;
    }
}

// Idempotent runtime initialization.
export async function ensureEditorRuntimeReady(): Promise<void> {
    if (isInitialized) return;

    if (!initializationPromise) {
        initializationPromise = initializeRuntime();
    }

    await initializationPromise;
}

export function getWasmModule(): any {
    return wasmModule;
}

export function isRuntimeReady(): boolean {
    return isInitialized;
}
