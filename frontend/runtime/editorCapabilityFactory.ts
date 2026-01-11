import type {
    EditorCapabilities,
    RenderCapability,
    ViewCapability,
    CommandCapability,
    ViewLifecycleCapability,
    InputCapability,
} from "@/editor-capabilities";
import {
    setEditorViewVisible,
    resizeEditorView,
    requestEditorViewRender,
} from "./viewManager";
import {
    createEditorView,
    attachEditorView,
    detachEditorView,
    destroyEditorView,
} from "./index";

import { getWasmModule } from "./editorRuntime";

function createRenderCapability(viewId: string): RenderCapability {
    return {
        requestRender(): void {
            requestEditorViewRender(viewId);
        },
        setVisible(visible: boolean): void {
            setEditorViewVisible(viewId, visible);
        },
        resize(cssWidth: number, cssHeight: number): void {
            resizeEditorView(viewId, cssWidth, cssHeight);
        },
    };
}

function createViewCapability(viewId: string): ViewCapability {
    return {
        getViewId(): string {
            return viewId;
        },
    };
}

function createCommandCapability(_viewId: string): CommandCapability {
    return {
        async dispatch(command) {
            const wasm = getWasmModule();

            if (!wasm || typeof wasm.dispatch_command !== "function") {
                return {
                    success: false,
                    error: "Runtime not initialized or dispatcher unavailable",
                };
            }

            try {
                // Commands are pure data => serialize once
                const json = JSON.stringify(command);

                // Call into WASM
                const result = wasm.dispatch_command(json);

                // wasm-bindgen may return JsValue or stringified JSON
                if (typeof result === "string") {
                    return JSON.parse(result);
                }

                if (result && typeof result === "object") {
                    return result as {
                        success: boolean;
                        error?: string;
                    };
                }

                return { success: true };
            } catch (err) {
                console.error("[CommandCapability] dispatch failed", err);
                return {
                    success: false,
                    error: err instanceof Error ? err.message : String(err),
                };
            }
        },
    };
}
function createViewLifecycleCapability(
    viewId: string
): ViewLifecycleCapability {
    return {
        createView(): void {
            createEditorView(viewId);
        },
        attachView(
            canvas: HTMLCanvasElement,
            width: number,
            height: number
        ): void {
            attachEditorView(viewId, canvas, width, height);
        },
        detachView(): void {
            detachEditorView(viewId);
        },
        destroyView(): void {
            destroyEditorView(viewId);
        },
    };
}

function createInputCapability(viewId: string): InputCapability {
    return {
        handleMouseMove(
            screenX: number,
            screenY: number,
            shift: boolean,
            ctrl: boolean,
            alt: boolean
        ): void {
            const wasm = getWasmModule();
            if (!wasm || typeof wasm.handle_mouse_move !== "function") {
                return;
            }
            wasm.handle_mouse_move(viewId, screenX, screenY, shift, ctrl, alt);
        },
        handleMouseDown(
            screenX: number,
            screenY: number,
            button: number,
            shift: boolean,
            ctrl: boolean,
            alt: boolean
        ): void {
            const wasm = getWasmModule();
            if (!wasm || typeof wasm.handle_mouse_down !== "function") {
                return;
            }
            wasm.handle_mouse_down(
                viewId,
                screenX,
                screenY,
                button,
                shift,
                ctrl,
                alt
            );
        },
        handleMouseUp(
            screenX: number,
            screenY: number,
            button: number,
            shift: boolean,
            ctrl: boolean,
            alt: boolean
        ): void {
            const wasm = getWasmModule();
            if (!wasm || typeof wasm.handle_mouse_up !== "function") {
                return;
            }
            wasm.handle_mouse_up(
                viewId,
                screenX,
                screenY,
                button,
                shift,
                ctrl,
                alt
            );
        },
        handleMouseEnter(
            screenX: number,
            screenY: number,
            button: number,
            shift: boolean,
            ctrl: boolean,
            alt: boolean
        ): void {
            const wasm = getWasmModule();
            if (!wasm || typeof wasm.handle_mouse_enter !== "function") {
                return;
            }
            wasm.handle_mouse_enter(
                viewId,
                screenX,
                screenY,
                button,
                shift,
                ctrl,
                alt
            );
        },
        handleMouseLeave(
            screenX: number,
            screenY: number,
            button: number,
            shift: boolean,
            ctrl: boolean,
            alt: boolean
        ): void {
            const wasm = getWasmModule();
            if (!wasm || typeof wasm.handle_mouse_leave !== "function") {
                return;
            }
            wasm.handle_mouse_leave(
                viewId,
                screenX,
                screenY,
                button,
                shift,
                ctrl,
                alt
            );
        },
    };
}

export function createEditorCapabilities(
    viewId: string | undefined
): EditorCapabilities {
    if (!viewId) {
        return {};
    }

    return {
        render: createRenderCapability(viewId),
        view: createViewCapability(viewId),
        command: createCommandCapability(viewId),
        lifecycle: createViewLifecycleCapability(viewId),
        input: createInputCapability(viewId),
    };
}
