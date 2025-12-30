import type { GoldenLayout } from "golden-layout";
import { type EditorType, getEditor } from "@/layout/editorRegistry";
import { generatePanelId, generateViewId } from "@/layout/id";

type PanelState = {
    panelId: string;
    editorType: EditorType;
    viewId?: string;
};

export function injectPlusButtons(gl: GoldenLayout, glymId: string) {
    const findAllStacks = (item: any): any[] => {
        const stacks: any[] = [];
        if (item.isStack) {
            stacks.push(item);
        }
        if (item.contentItems) {
            for (const child of item.contentItems) {
                stacks.push(...findAllStacks(child));
            }
        }
        return stacks;
    };

    const allStacks = gl.rootItem ? findAllStacks(gl.rootItem) : [];

    allStacks.forEach((stack) => {
        if (!stack.header?.element) return;

        const header = stack.header.element;
        const tabsContainer = header.querySelector(".lm_tabs");
        if (!tabsContainer) return;

        if (tabsContainer.querySelector(".lm_add_tab_btn")) return;

        const addBtn = document.createElement("button");
        addBtn.className = "lm_add_tab_btn";
        addBtn.setAttribute("title", "Add new tab");
        addBtn.innerHTML = "+";

        (addBtn as any).__stack = stack;

        addBtn.onclick = (e) => {
            e.preventDefault();
            e.stopPropagation();

            const targetStack = (e.currentTarget as any).__stack;

            if (!targetStack) {
                console.error("No stack reference found on button");
                return;
            }

            try {
                const newEditorType: EditorType = "uniforms";
                const newEditor = getEditor(newEditorType);
                const panelId = generatePanelId();

                const newPanelState: PanelState = {
                    panelId,
                    editorType: newEditorType,
                };

                if (newEditor.requiresViewId) {
                    newPanelState.viewId = generateViewId(glymId, panelId);
                }

                const componentConfig = {
                    type: "component" as const,
                    componentType: "panel",
                    title: newEditor.displayName,
                    componentState: newPanelState,
                };

                targetStack.addItem(componentConfig);
            } catch (error) {
                console.error("Failed to add tab:", error);
            }
        };

        tabsContainer.appendChild(addBtn);
    });
}
