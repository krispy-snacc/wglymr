"use client";

import { useEffect, useRef } from "react";
import * as ContextMenu from "@radix-ui/react-context-menu";
import { Grid3x3, Box } from "lucide-react";

interface NodeEditorHostProps {
    viewId: string;
}

let engineInitialized = false;

export function NodeEditorHost({ viewId }: NodeEditorHostProps) {
    console.log("Mounted Node Editor in UI", viewId);
    const containerRef = useRef<HTMLDivElement>(null);

    useEffect(() => {
        let mounted = true;

        const initializeEditor = async () => {
            if (!containerRef.current) return;

            const wasm = await import("../../wasm-pkg/wglymr_node_editor.js");
            await wasm.default();

            if (!engineInitialized) {
                wasm.init_engine();
                engineInitialized = true;
            }

            if (!mounted) return;

            wasm.create_view(viewId);

            const container = containerRef.current;
            const width = container.clientWidth;
            const height = container.clientHeight;

            const canvasId = `node-editor-canvas-${viewId}`;

            await wasm.attach_view_canvas(viewId, canvasId, width, height);

            if (!mounted) return;

            wasm.render_view(viewId);
        };

        initializeEditor();

        return () => {
            mounted = false;
        };
    }, [viewId]);

    return (
        <div className="w-full h-full bg-zinc-950/80 backdrop-blur-md p-2">
            <ContextMenu.Root>
                <ContextMenu.Trigger asChild>
                    <div
                        ref={containerRef}
                        id={`node-editor-${viewId}`}
                        className="w-full h-full relative overflow-hidden"
                        style={{
                            backgroundImage: `
              linear-gradient(rgba(255,255,255,0.03) 1px, transparent 1px),
              linear-gradient(90deg, rgba(255,255,255,0.03) 1px, transparent 1px)
            `,
                            backgroundSize: "20px 20px",
                        }}
                    >
                        <canvas id={`node-editor-canvas-${viewId}`} className="absolute inset-0" />
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
