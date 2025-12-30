import { type LayoutConfig as LayoutConfigType } from "golden-layout";
import { type EditorType, getEditor } from "@/layout/editorRegistry";
import { generatePanelId, generateViewId } from "@/layout/id";

type PanelState = {
    panelId: string;
    editorType: EditorType;
    viewId?: string;
};

export function createDefaultLayoutConfig(glymId: string): LayoutConfigType {
    const panel1 = generatePanelId();
    const panel2 = generatePanelId();
    const panel3 = generatePanelId();
    const panel4 = generatePanelId();
    const panel5 = generatePanelId();

    return {
        settings: {
            reorderEnabled: true,
            constrainDragToContainer: true,
        },
        header: {
            show: "top",
        },
        root: {
            type: "row",
            content: [
                {
                    type: "column",
                    content: [
                        {
                            type: "component",
                            componentType: "panel",
                            title: getEditor("preview").displayName,
                            componentState: {
                                panelId: panel1,
                                editorType: "preview",
                                viewId: generateViewId(glymId, panel1),
                            } satisfies PanelState,
                        },
                        {
                            type: "component",
                            componentType: "panel",
                            title: getEditor("metadata").displayName,
                            componentState: {
                                panelId: panel2,
                                editorType: "metadata",
                            } satisfies PanelState,
                        },
                    ],
                },
                {
                    type: "column",
                    content: [
                        {
                            type: "component",
                            componentType: "panel",
                            title: getEditor("nodeEditor").displayName,
                            componentState: {
                                panelId: panel3,
                                editorType: "nodeEditor",
                                viewId: generateViewId(glymId, panel3),
                            } satisfies PanelState,
                        },
                        {
                            type: "stack",
                            content: [
                                {
                                    type: "component",
                                    componentType: "panel",
                                    title: getEditor("uniforms").displayName,
                                    componentState: {
                                        panelId: panel4,
                                        editorType: "uniforms",
                                    } satisfies PanelState,
                                },
                                {
                                    type: "component",
                                    componentType: "panel",
                                    title: getEditor("textures").displayName,
                                    componentState: {
                                        panelId: panel5,
                                        editorType: "textures",
                                    } satisfies PanelState,
                                },
                            ],
                        },
                    ],
                },
            ],
        },
    };
}
