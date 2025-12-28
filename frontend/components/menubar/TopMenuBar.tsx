"use client";

import * as Menubar from "@radix-ui/react-menubar";
import { FileText, Eye, HelpCircle } from "lucide-react";

interface TopMenuBarProps {
    viewId: string;
    onTogglePanel?: (panel: string) => void;
}

export function TopMenuBar({ viewId, onTogglePanel }: TopMenuBarProps) {
    return (
        <div className="h-12 border-b border-white/10 bg-black/80 backdrop-blur-sm flex items-center px-4 justify-between">
            <Menubar.Root className="flex gap-1">
                {/* File Menu */}
                <Menubar.Menu>
                    <Menubar.Trigger className="px-3 py-1.5 text-sm text-gray-300 hover:text-white hover:bg-white/5 rounded cursor-pointer select-none outline-none data-[state=open]:bg-white/10">
                        File
                    </Menubar.Trigger>
                    <Menubar.Portal>
                        <Menubar.Content
                            className="min-w-45 bg-zinc-900 border border-white/10 rounded-md p-1 shadow-xl"
                            align="start"
                            sideOffset={5}
                        >
                            <Menubar.Item className="text-sm text-gray-300 px-3 py-2 rounded hover:bg-white/5 hover:text-white cursor-pointer outline-none">
                                <FileText className="inline-block w-4 h-4 mr-2" />
                                New Shader
                            </Menubar.Item>
                            <Menubar.Item className="text-sm text-gray-300 px-3 py-2 rounded hover:bg-white/5 hover:text-white cursor-pointer outline-none">
                                Reset View
                            </Menubar.Item>
                            <Menubar.Separator className="h-px bg-white/10 my-1" />
                            <Menubar.Item className="text-sm text-gray-300 px-3 py-2 rounded hover:bg-white/5 hover:text-white cursor-pointer outline-none">
                                Export WGSL
                            </Menubar.Item>
                        </Menubar.Content>
                    </Menubar.Portal>
                </Menubar.Menu>

                {/* View Menu */}
                <Menubar.Menu>
                    <Menubar.Trigger className="px-3 py-1.5 text-sm text-gray-300 hover:text-white hover:bg-white/5 rounded cursor-pointer select-none outline-none data-[state=open]:bg-white/10">
                        View
                    </Menubar.Trigger>
                    <Menubar.Portal>
                        <Menubar.Content
                            className="min-w-45 bg-zinc-900 border border-white/10 rounded-md p-1 shadow-xl"
                            align="start"
                            sideOffset={5}
                        >
                            <Menubar.Item
                                className="text-sm text-gray-300 px-3 py-2 rounded hover:bg-white/5 hover:text-white cursor-pointer outline-none"
                                onSelect={() => onTogglePanel?.("uniforms")}
                            >
                                <Eye className="inline-block w-4 h-4 mr-2" />
                                Toggle Uniforms
                            </Menubar.Item>
                            <Menubar.Item
                                className="text-sm text-gray-300 px-3 py-2 rounded hover:bg-white/5 hover:text-white cursor-pointer outline-none"
                                onSelect={() => onTogglePanel?.("inspector")}
                            >
                                Toggle Inspector
                            </Menubar.Item>
                        </Menubar.Content>
                    </Menubar.Portal>
                </Menubar.Menu>

                {/* Help Menu */}
                <Menubar.Menu>
                    <Menubar.Trigger className="px-3 py-1.5 text-sm text-gray-300 hover:text-white hover:bg-white/5 rounded cursor-pointer select-none outline-none data-[state=open]:bg-white/10">
                        Help
                    </Menubar.Trigger>
                    <Menubar.Portal>
                        <Menubar.Content
                            className="min-w-45 bg-zinc-900 border border-white/10 rounded-md p-1 shadow-xl"
                            align="start"
                            sideOffset={5}
                        >
                            <Menubar.Item className="text-sm text-gray-300 px-3 py-2 rounded hover:bg-white/5 hover:text-white cursor-pointer outline-none">
                                <HelpCircle className="inline-block w-4 h-4 mr-2" />
                                Documentation
                            </Menubar.Item>
                            <Menubar.Item className="text-sm text-gray-300 px-3 py-2 rounded hover:bg-white/5 hover:text-white cursor-pointer outline-none">
                                Keyboard Shortcuts
                            </Menubar.Item>
                            <Menubar.Separator className="h-px bg-white/10 my-1" />
                            <Menubar.Item className="text-sm text-gray-300 px-3 py-2 rounded hover:bg-white/5 hover:text-white cursor-pointer outline-none">
                                About wglymr
                            </Menubar.Item>
                        </Menubar.Content>
                    </Menubar.Portal>
                </Menubar.Menu>
            </Menubar.Root>

            {/* View ID Display */}
            <div className="text-xs text-gray-500 font-mono">
                view: <span className="text-gray-400">{viewId}</span>
            </div>
        </div>
    );
}
