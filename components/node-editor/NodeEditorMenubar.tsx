"use client";

import * as Menubar from "@radix-ui/react-menubar";
import { Plus, Folder, Settings } from "lucide-react";

export function NodeEditorMenubar() {
    return (
        <Menubar.Root className="flex items-center gap-0.5">
            <Menubar.Menu>
                <Menubar.Trigger className="px-2 py-0.5 text-xs font-normal text-gray-400 rounded hover:bg-white/10 hover:text-white outline-none cursor-pointer data-[state=open]:bg-white/10 data-[state=open]:text-white transition-colors">
                    Add Node
                </Menubar.Trigger>
                <Menubar.Portal>
                    <Menubar.Content className="min-w-40 bg-zinc-900/98 backdrop-blur-sm border border-white/12 rounded-md shadow-2xl p-0.5 z-50">
                        <Menubar.Item className="flex items-center gap-2 px-2.5 py-1.5 text-xs text-gray-400 rounded hover:bg-white/10 hover:text-white outline-none cursor-pointer transition-colors">
                            <Plus className="w-3.5 h-3.5" />
                            Input Node
                        </Menubar.Item>
                        <Menubar.Item className="flex items-center gap-2 px-2.5 py-1.5 text-xs text-gray-400 rounded hover:bg-white/10 hover:text-white outline-none cursor-pointer transition-colors">
                            <Plus className="w-3.5 h-3.5" />
                            Output Node
                        </Menubar.Item>
                        <Menubar.Separator className="h-px bg-white/8 my-0.5" />
                        <Menubar.Item className="flex items-center gap-2 px-2.5 py-1.5 text-xs text-gray-400 rounded hover:bg-white/10 hover:text-white outline-none cursor-pointer transition-colors">
                            <Plus className="w-3.5 h-3.5" />
                            Math Node
                        </Menubar.Item>
                        <Menubar.Item className="flex items-center gap-2 px-2.5 py-1.5 text-xs text-gray-400 rounded hover:bg-white/10 hover:text-white outline-none cursor-pointer transition-colors">
                            <Plus className="w-3.5 h-3.5" />
                            Texture Node
                        </Menubar.Item>
                    </Menubar.Content>
                </Menubar.Portal>
            </Menubar.Menu>

            <Menubar.Menu>
                <Menubar.Trigger className="px-2 py-0.5 text-xs font-normal text-gray-400 rounded hover:bg-white/10 hover:text-white outline-none cursor-pointer data-[state=open]:bg-white/10 data-[state=open]:text-white transition-colors">
                    Library
                </Menubar.Trigger>
                <Menubar.Portal>
                    <Menubar.Content className="min-w-40 bg-zinc-900/98 backdrop-blur-sm border border-white/12 rounded-md shadow-2xl p-0.5 z-50">
                        <Menubar.Item className="flex items-center gap-2 px-2.5 py-1.5 text-xs text-gray-400 rounded hover:bg-white/10 hover:text-white outline-none cursor-pointer transition-colors">
                            <Folder className="w-3.5 h-3.5" />
                            Save Graph
                        </Menubar.Item>
                        <Menubar.Item className="flex items-center gap-2 px-2.5 py-1.5 text-xs text-gray-400 rounded hover:bg-white/10 hover:text-white outline-none cursor-pointer transition-colors">
                            <Folder className="w-3.5 h-3.5" />
                            Load Graph
                        </Menubar.Item>
                    </Menubar.Content>
                </Menubar.Portal>
            </Menubar.Menu>

            <Menubar.Menu>
                <Menubar.Trigger className="px-2 py-0.5 text-xs font-normal text-gray-400 rounded hover:bg-white/10 hover:text-white outline-none cursor-pointer data-[state=open]:bg-white/10 data-[state=open]:text-white transition-colors">
                    Options
                </Menubar.Trigger>
                <Menubar.Portal>
                    <Menubar.Content className="min-w-40 bg-zinc-900/98 backdrop-blur-sm border border-white/12 rounded-md shadow-2xl p-0.5 z-50">
                        <Menubar.Item className="flex items-center gap-2 px-2.5 py-1.5 text-xs text-gray-400 rounded hover:bg-white/10 hover:text-white outline-none cursor-pointer transition-colors">
                            <Settings className="w-3.5 h-3.5" />
                            Grid Settings
                        </Menubar.Item>
                        <Menubar.Item className="flex items-center gap-2 px-2.5 py-1.5 text-xs text-gray-400 rounded hover:bg-white/10 hover:text-white outline-none cursor-pointer transition-colors">
                            <Settings className="w-3.5 h-3.5" />
                            Snap to Grid
                        </Menubar.Item>
                    </Menubar.Content>
                </Menubar.Portal>
            </Menubar.Menu>
        </Menubar.Root>
    );
}
