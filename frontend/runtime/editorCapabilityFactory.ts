import type {
    EditorCapabilities,
    RenderCapability,
    ViewCapability,
    CommandCapability,
} from "@/editor-capabilities";
import {
    setEditorViewVisible,
    resizeEditorView,
    requestEditorViewRender,
} from "./viewManager";

function createRenderCapability(viewId: string): RenderCapability {
    return {
        requestRender(): void {
            requestEditorViewRender(viewId);
        },
        setVisible(visible: boolean): void {
            setEditorViewVisible(viewId, visible);
        },
        resize(width: number, height: number): void {
            resizeEditorView(viewId, width, height);
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

// CommandCapability stub: forwards commands to runtime dispatcher.
// IMPLEMENTATION PENDING: requires runtime command dispatcher in Rust.
function createCommandCapability(_viewId: string): CommandCapability {
    return {
        async dispatch(command) {
            // TODO: call WASM runtime.dispatchCommand(command)
            // Runtime must validate, execute, and return result
            console.warn(
                "[CommandCapability] dispatch() not yet implemented",
                command
            );
            return {
                success: false,
                error: "Runtime dispatcher not implemented",
            };
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
    };
}
