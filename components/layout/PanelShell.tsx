"use client";

import { ChevronDown } from "lucide-react";
import * as Select from "@radix-ui/react-select";
import {
    type EditorType,
    EDITOR_REGISTRY,
    getEditor,
} from "./editorRegistry";

interface PanelShellProps {
    editorType: EditorType;
    onEditorTypeChange: (editorType: EditorType) => void;
    viewId?: string;
}

export function PanelShell({ editorType, onEditorTypeChange, viewId }: PanelShellProps) {
    const editor = getEditor(editorType);
    const EditorComponent = editor.component as React.ComponentType<any>;
    const HeaderControls = editor.headerControls;
    const Icon = editor.icon;

    const editorTypes = Object.keys(EDITOR_REGISTRY) as EditorType[];

    return (
        <div className="h-full flex flex-col bg-zinc-950">
            <div className="flex items-center gap-1 px-2 bg-white/2 border-b border-white/6 shrink-0" style={{ height: "28px" }}>
                <Select.Root value={editorType} onValueChange={(value) => onEditorTypeChange(value as EditorType)}>
                    <Select.Trigger className="flex items-center gap-0.5 px-1.5 py-0.5 rounded hover:bg-white/10 transition-colors cursor-pointer outline-none border-none bg-transparent data-[state=open]:bg-white/10">
                        <Icon className="w-3.5 h-3.5 text-white/60" />
                        <ChevronDown className="w-2.5 h-2.5 text-white/40" />
                    </Select.Trigger>
                    <Select.Portal>
                        <Select.Content
                            className="bg-zinc-900/98 backdrop-blur-sm border border-white/12 rounded-md shadow-2xl overflow-hidden"
                            style={{ zIndex: 9999 }}
                            position="popper"
                            sideOffset={2}
                        >
                            <Select.Viewport className="p-0.5">
                                {editorTypes.map((type) => {
                                    const ed = getEditor(type);
                                    const EdIcon = ed.icon;
                                    return (
                                        <Select.Item
                                            key={ed.id}
                                            value={ed.id}
                                            className="flex items-center gap-2 px-2.5 py-1.5 text-xs text-white/80 hover:bg-white/10 rounded cursor-pointer outline-none focus:bg-white/10 data-highlighted:bg-white/10 transition-colors"
                                        >
                                            <EdIcon className="w-3.5 h-3.5 text-white/50" />
                                            <Select.ItemText>{ed.displayName}</Select.ItemText>
                                        </Select.Item>
                                    );
                                })}
                            </Select.Viewport>
                        </Select.Content>
                    </Select.Portal>
                </Select.Root>
                {HeaderControls && (
                    <div className="flex items-center">
                        <HeaderControls />
                    </div>
                )}
            </div>
            <div className="flex-1 overflow-hidden">
                {viewId ? <EditorComponent viewId={viewId} /> : <EditorComponent />}
            </div>
        </div>
    );
}
