import type { LucideIcon } from "lucide-react";
import { FileCode2, MonitorPlay, FileText, Sliders, Image } from "lucide-react";

import { PreviewPanel } from "@/panels/PreviewPanel";
import { NodeEditorHost } from "@/editors/nodeEditor/NodeEditorHost";
import { NodeEditorMenubar } from "@/editors/nodeEditor/NodeEditorMenubar";
import { MetadataPanel } from "@/panels/MetadataPanel";
import { UniformPanel } from "@/panels/UniformPanel";
import { InspectorPanel } from "@/panels/InspectorPanel";

export type EditorType =
    | "preview"
    | "nodeEditor"
    | "metadata"
    | "uniforms"
    | "textures";

type EditorComponentProps = {
    panelId: string;
    viewId?: string;
};

type EditorDefinition = {
    id: EditorType;
    displayName: string;
    icon: LucideIcon;
    component: React.ComponentType<EditorComponentProps>;
    toolbar?: React.ComponentType<Record<string, never>>;
    requiresViewId?: boolean;
};

export const EDITOR_REGISTRY: Record<EditorType, EditorDefinition> = {
    preview: {
        id: "preview",
        displayName: "Preview",
        icon: MonitorPlay,
        component: PreviewPanel,
        requiresViewId: true,
    },
    nodeEditor: {
        id: "nodeEditor",
        displayName: "Node Editor",
        icon: FileCode2,
        component: NodeEditorHost,
        toolbar: NodeEditorMenubar,
        requiresViewId: true,
    },
    metadata: {
        id: "metadata",
        displayName: "Metadata",
        icon: FileText,
        component: MetadataPanel,
    },
    uniforms: {
        id: "uniforms",
        displayName: "Uniforms",
        icon: Sliders,
        component: UniformPanel,
    },
    textures: {
        id: "textures",
        displayName: "Textures",
        icon: Image,
        component: InspectorPanel,
    },
};

export function getEditor(type: EditorType): EditorDefinition {
    return EDITOR_REGISTRY[type];
}
