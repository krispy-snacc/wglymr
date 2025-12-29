import type {
    EditorCapabilities,
    RenderCapability,
    ViewCapability,
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

export function createEditorCapabilities(
    viewId: string | undefined
): EditorCapabilities {
    if (!viewId) {
        return {};
    }

    return {
        render: createRenderCapability(viewId),
        view: createViewCapability(viewId),
    };
}
