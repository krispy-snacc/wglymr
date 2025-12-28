"use client";

import { type EditorType, getEditor } from "./editorRegistry";
import { PanelHeader } from "./panel/PanelHeader";

interface PanelShellProps {
    editorType: EditorType;
    onEditorTypeChange: (editorType: EditorType) => void;
    viewId?: string;
}

export function PanelShell({ editorType, onEditorTypeChange, viewId }: PanelShellProps) {
    const editor = getEditor(editorType);
    const EditorComponent = editor.component as React.ComponentType<any>;

    return (
        <div className="h-full flex flex-col bg-zinc-950">
            <PanelHeader
                editorType={editorType}
                onEditorTypeChange={onEditorTypeChange}
                editorRegistryEntry={editor}
            />
            <div className="flex-1 overflow-hidden">
                {viewId ? <EditorComponent viewId={viewId} /> : <EditorComponent />}
            </div>
        </div>
    );
}
