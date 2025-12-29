"use client";

import { type EditorType, getEditor } from "@/layout/editorRegistry";
import { PanelHeader } from "@/layout/panel/PanelHeader";
import { PanelProvider } from "@/context/PanelContext";
import { EditorProvider } from "@/context/EditorContext";

interface PanelShellProps {
    panelId: string;
    editorType: EditorType;
    onEditorTypeChange: (editorType: EditorType) => void;
    viewId?: string;
}

export function PanelShell({ panelId, editorType, onEditorTypeChange, viewId }: PanelShellProps) {
    const editor = getEditor(editorType);
    const EditorComponent = editor.component;

    return (
        <PanelProvider panelId={panelId}>
            <EditorProvider editorType={editorType} viewId={viewId}>
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
            </EditorProvider>
        </PanelProvider>
    );
}
