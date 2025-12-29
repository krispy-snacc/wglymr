"use client";

import { useMemo } from "react";
import { type EditorType, getEditor } from "@/layout/editorRegistry";
import { PanelHeader } from "@/layout/panel/PanelHeader";
import { PanelProvider } from "@/context/PanelContext";
import { EditorProvider } from "@/context/EditorContext";
import { EditorCapabilitiesProvider } from "@/context/EditorCapabilitiesContext";
import { createEditorCapabilities } from "@/runtime/editorCapabilityFactory";

interface PanelShellProps {
    panelId: string;
    editorType: EditorType;
    onEditorTypeChange: (editorType: EditorType) => void;
    viewId?: string;
}

export function PanelShell({ panelId, editorType, onEditorTypeChange, viewId }: PanelShellProps) {
    const editor = getEditor(editorType);
    const EditorComponent = editor.component;

    const capabilities = useMemo(() => {
        return createEditorCapabilities(viewId);
    }, [viewId]);

    return (
        <PanelProvider panelId={panelId}>
            <EditorProvider editorType={editorType} viewId={viewId}>
                <EditorCapabilitiesProvider capabilities={capabilities}>
                    <div className="h-full flex flex-col bg-zinc-950">
                        <PanelHeader
                            editorType={editorType}
                            onEditorTypeChange={onEditorTypeChange}
                            editorRegistryEntry={editor}
                        />
                        <div className="flex-1 overflow-hidden">
                            <EditorComponent />
                        </div>
                    </div>
                </EditorCapabilitiesProvider>
            </EditorProvider>
        </PanelProvider>
    );
}
