"use client";

import { useEffect, useRef } from "react";
import * as ContextMenu from "@radix-ui/react-context-menu";
import {
    ensureEditorRuntimeReady,
    createEditorView,
    attachEditorView,
    setEditorViewVisible,
    resizeEditorView,
    requestEditorRender,
    detachEditorView,
    destroyEditorView,
} from "../../runtime";

interface NodeEditorHostProps {
    panelId: string;
    viewId?: string;
}

export function NodeEditorHost({ panelId, viewId }: NodeEditorHostProps) {
    const containerRef = useRef<HTMLDivElement>(null);

    useEffect(() => {
        if (!viewId) return;

        let mounted = true;
        let resizeObserver: ResizeObserver | null = null;

        const initializeEditor = async () => {
            if (!containerRef.current) return;

            await ensureEditorRuntimeReady();

            if (!mounted) return;

            createEditorView(viewId);

            await new Promise(requestAnimationFrame);

            if (!mounted) return;

            const container = containerRef.current;
            if (!container) return;

            const canvas = container.querySelector(`#node-editor-canvas-${viewId}`) as HTMLCanvasElement;
            if (!canvas) return;

            const width = container.clientWidth;
            const height = container.clientHeight;

            attachEditorView(viewId, canvas, width, height);

            // NOTE:
            // Golden Layout may hide panels without unmounting them.
            // Visibility should be toggled via setVisible(viewId, false/true)
            // when tab activation changes. Hook will be added later.
            setEditorViewVisible(viewId, true);
            requestEditorRender(viewId);

            resizeObserver = new ResizeObserver((entries) => {
                for (const entry of entries) {
                    const { width, height } = entry.contentRect;
                    resizeEditorView(viewId, width, height);
                    requestEditorRender(viewId);
                }
            });

            resizeObserver.observe(container);
        };

        initializeEditor();

        return () => {
            mounted = false;

            if (resizeObserver) {
                resizeObserver.disconnect();
            }

            try {
                setEditorViewVisible(viewId, false);
                detachEditorView(viewId);
                destroyEditorView(viewId);
            } catch (err) {
                console.warn("Error during view cleanup:", err);
            }
        };
    }, [viewId]);

    return (
        <div className="w-full h-full bg-zinc-950/80 backdrop-blur-md">
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
