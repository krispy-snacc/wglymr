import { createRoot, type Root } from "react-dom/client";
import type { ComponentContainer, JsonValue } from "golden-layout";
import { type EditorType, getEditor } from "@/layout/editorRegistry";
import { generatePanelId, generateViewId } from "@/layout/id";
import { PanelShell } from "@/layout/PanelShell";

type PanelState = {
    panelId: string;
    editorType: EditorType;
    viewId?: string;
};

export function createPanelRenderer(glymId: string) {
    const containerRoots = new Map<ComponentContainer, Root>();

    const mountReactPanel = (container: ComponentContainer, state: JsonValue | undefined) => {
        const panelState = (state ?? {}) as Partial<PanelState>;
        const editorType = panelState.editorType as EditorType | undefined;

        if (!editorType) {
            container.element.textContent = "";
            return;
        }

        const editor = getEditor(editorType);

        let panelId = panelState.panelId;
        if (!panelId) {
            panelId = generatePanelId();
            container.setState({ ...panelState, panelId });
        }

        let effectiveViewId: string | undefined;
        if (editor.requiresViewId) {
            if (panelState.viewId) {
                effectiveViewId = panelState.viewId;
            } else {
                effectiveViewId = generateViewId(glymId, panelId);
                container.setState({ ...panelState, panelId, viewId: effectiveViewId });
            }
        }

        const ensureRoot = () => {
            const existing = containerRoots.get(container);
            if (existing) return existing;
            container.element.textContent = "";
            const root = createRoot(container.element);
            containerRoots.set(container, root);
            container.on("destroy", () => {
                const r = containerRoots.get(container);
                if (!r) return;
                containerRoots.delete(container);
                queueMicrotask(() => {
                    try {
                        r.unmount();
                    } catch {
                        // ignore
                    }
                });
            });
            return root;
        };

        const root = ensureRoot();

        const render = () => {
            const currentState = container.getState() as Partial<PanelState>;
            const currentEditorType = (currentState.editorType || editorType) as EditorType;
            const currentPanelId = currentState.panelId || panelId;
            const currentViewId = currentState.viewId || effectiveViewId;

            const handleEditorTypeChange = (newEditorType: EditorType) => {
                const freshState = container.getState() as Partial<PanelState>;
                const newEditor = getEditor(newEditorType);

                const newState: Partial<PanelState> = {
                    ...freshState,
                    panelId: currentPanelId,
                    editorType: newEditorType,
                };

                if (newEditor.requiresViewId) {
                    newState.viewId = freshState.viewId || generateViewId(glymId, currentPanelId);
                } else {
                    delete newState.viewId;
                }

                container.setState(newState);
                container.setTitle(getEditor(newEditorType).displayName);
                render();
            };

            root.render(
                <PanelShell
                    panelId={currentPanelId}
                    editorType={currentEditorType}
                    onEditorTypeChange={handleEditorTypeChange}
                    viewId={currentViewId}
                />
            );
        };

        render();

        const rerenderOnStateChange = () => render();
        container.on("stateChanged", rerenderOnStateChange);
        container.on("destroy", () => {
            container.off("stateChanged", rerenderOnStateChange);
        });
    };

    const cleanup = () => {
        const roots = Array.from(containerRoots.values());
        containerRoots.clear();
        queueMicrotask(() => {
            for (const root of roots) {
                try {
                    root.unmount();
                } catch {
                    // ignore
                }
            }
        });
    };

    return { mountReactPanel, cleanup };
}
