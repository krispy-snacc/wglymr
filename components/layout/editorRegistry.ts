import type { LucideIcon } from "lucide-react";
import { FileCode2, MonitorPlay, FileText, Sliders, Image } from "lucide-react";

import { PreviewPanel } from "@/components/panels/PreviewPanel";
import { NodeEditorHost } from "@/components/node-editor/NodeEditorHost";
import { NodeEditorMenubar } from "@/components/node-editor/NodeEditorMenubar";
import { MetadataPanel } from "@/components/panels/MetadataPanel";
import { UniformPanel } from "@/components/panels/UniformPanel";
import { InspectorPanel } from "@/components/panels/InspectorPanel";

export type EditorType =
    | "preview"
    | "nodeEditor"
    | "metadata"
    | "uniforms"
    | "textures";

type EditorDefinition = {
    id: EditorType;
    displayName: string;
    icon: LucideIcon;
    component:
        | React.ComponentType<{ viewId: string }>
        | React.ComponentType<Record<string, never>>;
    headerControls?: React.ComponentType<Record<string, never>>;
};

export const EDITOR_REGISTRY: Record<EditorType, EditorDefinition> = {
    preview: {
        id: "preview",
        displayName: "Preview",
        icon: MonitorPlay,
        component: PreviewPanel,
    },
    nodeEditor: {
        id: "nodeEditor",
        displayName: "Node Editor",
        icon: FileCode2,
        component: NodeEditorHost,
        headerControls: NodeEditorMenubar,
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
