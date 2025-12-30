"use client";

import { useState } from "react";
import * as ContextMenu from "@radix-ui/react-context-menu";
import { Plus } from "lucide-react";
import { type EditorType, getEditor, EDITOR_REGISTRY } from "@/layout/editorRegistry";

interface EmptyLayoutOverlayProps {
    onCreatePanel: (editorType: EditorType) => void;
    onRestoreDefaultLayout: () => void;
}

export function EmptyLayoutOverlay({
    onCreatePanel,
    onRestoreDefaultLayout,
}: EmptyLayoutOverlayProps) {
    const editorTypes = Object.keys(EDITOR_REGISTRY) as EditorType[];

    return (
        <ContextMenu.Root>
            <ContextMenu.Trigger asChild>
                <div className="absolute inset-0 z-50 flex items-center justify-center bg-zinc-950/95 backdrop-blur-sm">
                    <button
                        type="button"
                        className="flex flex-col items-center gap-3 text-gray-500 hover:text-gray-300 transition-colors"
                        aria-label="Create panel"
                    >
                        <Plus className="w-16 h-16" />
                        <span className="text-sm">
                            right-click to add panel
                        </span>
                    </button>
                </div>
            </ContextMenu.Trigger>

            <ContextMenu.Portal>
                <ContextMenu.Content
                    className="min-w-56 z-100 bg-zinc-900/95 backdrop-blur border border-white/10 rounded-lg shadow-2xl p-1.5"
                >
                    {editorTypes.map((type) => {
                        const editor = getEditor(type);
                        const Icon = editor.icon;

                        return (
                            <ContextMenu.Item
                                key={type}
                                onSelect={() => onCreatePanel(type)}
                                className="flex items-center gap-3 px-3 py-2 text-sm text-gray-300 rounded-md
                                           hover:bg-white/10 hover:text-white outline-none cursor-pointer"
                            >
                                <Icon className="w-4 h-4 text-white/60" />
                                {editor.displayName}
                            </ContextMenu.Item>
                        );
                    })}

                    <ContextMenu.Separator className="my-1 h-px bg-white/10" />

                    <ContextMenu.Item
                        onSelect={onRestoreDefaultLayout}
                        className="flex items-center gap-3 px-3 py-2 text-sm text-gray-400 rounded-md
                                   hover:bg-white/10 hover:text-white outline-none cursor-pointer"
                    >
                        Restore default layout
                    </ContextMenu.Item>
                </ContextMenu.Content>
            </ContextMenu.Portal>
        </ContextMenu.Root>
    );
}
