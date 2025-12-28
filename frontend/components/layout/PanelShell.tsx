"use client";

import { type EditorType, getEditor } from "./editorRegistry";
import { PanelHeader } from "./panel/PanelHeader";

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
        <div className="h-full flex flex-col bg-zinc-950">
            <PanelHeader
                editorType={editorType}
                onEditorTypeChange={onEditorTypeChange}
                editorRegistryEntry={editor}
            />
            <div className="flex-1 overflow-hidden">
                <EditorComponent panelId={panelId} viewId={viewId} />
            </div>
        </div>
    );
}
