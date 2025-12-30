import type { GoldenLayout } from "golden-layout";
import { type EditorType, getEditor } from "@/layout/editorRegistry";
import { generatePanelId, generateViewId } from "@/layout/id";

type PanelState = {
    panelId: string;
    editorType: EditorType;
    viewId?: string;
};

export function createEditorSpawner(gl: GoldenLayout, glymId: string) {
    const openEditor = (editorType: EditorType) => {
        const editor = getEditor(editorType);
        const panelId = generatePanelId();
        const newPanelState: PanelState = {
            panelId,
            editorType,
        };

        if (editor.requiresViewId) {
            newPanelState.viewId = generateViewId(glymId, panelId);
        }

        try {
            gl.addComponent("panel", newPanelState, editor.displayName);
        } catch (e) {
            console.error("Failed to add new editor panel:", e);
        }
    };

    return { openEditor };
}
