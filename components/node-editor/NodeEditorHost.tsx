"use client";

import * as ContextMenu from "@radix-ui/react-context-menu";
import * as Menubar from "@radix-ui/react-menubar";
import { Grid3x3, Box, Plus, Folder, Settings } from "lucide-react";

interface NodeEditorHostProps {
    viewId: string;
}

/**
 * NodeEditorHost - Placeholder for WASM/egui node editor
 * 
 * This component serves as the mount point for the future WebAssembly-based
 * node editor. The egui canvas will be mounted directly into this container.
 * 
 * TODO: Integrate WASM module and mount egui canvas here
 * TODO: Pass viewId to WASM context for shader loading
 */
export function NodeEditorHost({ viewId }: NodeEditorHostProps) {
    return (
        <div className="w-full h-full flex flex-col bg-zinc-950/80 backdrop-blur-md">
            {/* Node Editor Menubar */}
            <Menubar.Root className="shrink-0 bg-zinc-900/60 backdrop-blur-md border-b border-white/5 px-3 py-1 flex items-center gap-1">
                <Menubar.Menu>
                    <Menubar.Trigger className="px-2.5 py-1 text-xs font-medium text-gray-400 rounded hover:bg-white/10 hover:text-white outline-none cursor-pointer data-[state=open]:bg-white/10 data-[state=open]:text-white transition-colors">
                        Add Node
                    </Menubar.Trigger>
                    <Menubar.Portal>
                        <Menubar.Content className="min-w-45 bg-zinc-900/95 backdrop-blur-lg border border-white/10 rounded-lg shadow-2xl p-1.5">
                            <Menubar.Item className="flex items-center gap-2 px-3 py-1.5 text-xs text-gray-300 rounded-md hover:bg-white/10 hover:text-white outline-none cursor-pointer transition-colors">
                                <Plus className="w-3.5 h-3.5" />
                                Input Node
                            </Menubar.Item>
                            <Menubar.Item className="flex items-center gap-2 px-3 py-1.5 text-xs text-gray-300 rounded-md hover:bg-white/10 hover:text-white outline-none cursor-pointer transition-colors">
                                <Plus className="w-3.5 h-3.5" />
                                Output Node
                            </Menubar.Item>
                            <Menubar.Separator className="h-px bg-white/10 my-1" />
                            <Menubar.Item className="flex items-center gap-2 px-3 py-1.5 text-xs text-gray-300 rounded-md hover:bg-white/10 hover:text-white outline-none cursor-pointer transition-colors">
                                <Plus className="w-3.5 h-3.5" />
                                Math Node
                            </Menubar.Item>
                            <Menubar.Item className="flex items-center gap-2 px-3 py-1.5 text-xs text-gray-300 rounded-md hover:bg-white/10 hover:text-white outline-none cursor-pointer transition-colors">
                                <Plus className="w-3.5 h-3.5" />
                                Texture Node
                            </Menubar.Item>
                        </Menubar.Content>
                    </Menubar.Portal>
                </Menubar.Menu>

                <Menubar.Menu>
                    <Menubar.Trigger className="px-2.5 py-1 text-xs font-medium text-gray-400 rounded hover:bg-white/10 hover:text-white outline-none cursor-pointer data-[state=open]:bg-white/10 data-[state=open]:text-white transition-colors">
                        Library
                    </Menubar.Trigger>
                    <Menubar.Portal>
                        <Menubar.Content className="min-w-45 bg-zinc-900/95 backdrop-blur-lg border border-white/10 rounded-lg shadow-2xl p-1.5">
                            <Menubar.Item className="flex items-center gap-2 px-3 py-1.5 text-xs text-gray-300 rounded-md hover:bg-white/10 hover:text-white outline-none cursor-pointer transition-colors">
                                <Folder className="w-3.5 h-3.5" />
                                Save Graph
                            </Menubar.Item>
                            <Menubar.Item className="flex items-center gap-2 px-3 py-1.5 text-xs text-gray-300 rounded-md hover:bg-white/10 hover:text-white outline-none cursor-pointer transition-colors">
                                <Folder className="w-3.5 h-3.5" />
                                Load Graph
                            </Menubar.Item>
                        </Menubar.Content>
                    </Menubar.Portal>
                </Menubar.Menu>

                <Menubar.Menu>
                    <Menubar.Trigger className="px-2.5 py-1 text-xs font-medium text-gray-400 rounded hover:bg-white/10 hover:text-white outline-none cursor-pointer data-[state=open]:bg-white/10 data-[state=open]:text-white transition-colors">
                        Options
                    </Menubar.Trigger>
                    <Menubar.Portal>
                        <Menubar.Content className="min-w-45 bg-zinc-900/95 backdrop-blur-lg border border-white/10 rounded-lg shadow-2xl p-1.5">
                            <Menubar.Item className="flex items-center gap-2 px-3 py-1.5 text-xs text-gray-300 rounded-md hover:bg-white/10 hover:text-white outline-none cursor-pointer transition-colors">
                                <Settings className="w-3.5 h-3.5" />
                                Grid Settings
                            </Menubar.Item>
                            <Menubar.Item className="flex items-center gap-2 px-3 py-1.5 text-xs text-gray-300 rounded-md hover:bg-white/10 hover:text-white outline-none cursor-pointer transition-colors">
                                <Settings className="w-3.5 h-3.5" />
                                Snap to Grid
                            </Menubar.Item>
                        </Menubar.Content>
                    </Menubar.Portal>
                </Menubar.Menu>
            </Menubar.Root>

            {/* Node Editor Canvas */}
            <ContextMenu.Root>
                <ContextMenu.Trigger asChild>
                    <div
                        id={`node-editor-${viewId}`}
                        className="flex-1 relative w-full overflow-hidden"
                        style={{
                            backgroundImage: `
              linear-gradient(rgba(255,255,255,0.03) 1px, transparent 1px),
              linear-gradient(90deg, rgba(255,255,255,0.03) 1px, transparent 1px)
            `,
                            backgroundSize: "20px 20px",
                        }}
                    >
                        {/* Placeholder Content */}
                        <div className="absolute inset-0 flex flex-col items-center justify-center gap-4">
                            <div className="flex items-center gap-3">
                                <Box className="w-8 h-8 text-gray-600" />
                                <Grid3x3 className="w-8 h-8 text-gray-600" />
                            </div>
                            <div className="text-center space-y-1">
                                <h3 className="text-lg font-medium text-gray-400">Node Editor</h3>
                                <p className="text-sm text-gray-600">WASM mount point</p>
                                <p className="text-xs text-gray-700 font-mono mt-2">view: {viewId}</p>
                            </div>
                        </div>

                        {/* Future WASM Canvas Mount Point */}
                        <div id={`node-editor-canvas-${viewId}`} className="absolute inset-0" />
                    </div>
                </ContextMenu.Trigger>

                <ContextMenu.Portal>
                    <ContextMenu.Content className="min-w-55 bg-zinc-900/95 backdrop-blur-lg border border-white/10 rounded-lg shadow-2xl p-1.5">
                        <ContextMenu.Item className="flex items-center gap-2 px-3 py-1.5 text-xs text-gray-300 rounded-md hover:bg-white/10 hover:text-white outline-none cursor-pointer transition-colors">
                            Add Node
                        </ContextMenu.Item>
                        <ContextMenu.Item className="flex items-center gap-2 px-3 py-1.5 text-xs text-gray-300 rounded-md hover:bg-white/10 hover:text-white outline-none cursor-pointer transition-colors">
                            Add Comment
                        </ContextMenu.Item>
                        <ContextMenu.Separator className="h-px bg-white/10 my-1" />
                        <ContextMenu.Item className="flex items-center gap-2 px-3 py-1.5 text-xs text-gray-300 rounded-md hover:bg-white/10 hover:text-white outline-none cursor-pointer transition-colors">
                            Reset View
                        </ContextMenu.Item>
                        <ContextMenu.Item className="flex items-center gap-2 px-3 py-1.5 text-xs text-gray-300 rounded-md hover:bg-white/10 hover:text-white outline-none cursor-pointer transition-colors">
                            Frame All
                        </ContextMenu.Item>
                    </ContextMenu.Content>
                </ContextMenu.Portal>
            </ContextMenu.Root>
        </div>
    );
}
