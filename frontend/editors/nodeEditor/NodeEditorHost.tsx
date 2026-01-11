"use client";

import { useEffect, useRef, useCallback } from "react";
import * as ContextMenu from "@radix-ui/react-context-menu";
import { useEditorCapabilities } from "@/context/EditorCapabilitiesContext";
import { setActiveView } from "@/layout/activeViewTracker";
import { registerViewDispatcher, unregisterViewDispatcher } from "@/layout/commandDispatchRegistry";
import type { InputContext } from "@/commands";
import * as cmd from "@/commands";

export function NodeEditorHost() {
    const containerRef = useRef<HTMLDivElement>(null);
    const dragStateRef = useRef({ active: false, lastX: 0, lastY: 0, button: -1 });
    const capabilities = useEditorCapabilities();

    const renderCapability = capabilities.render;
    const viewCapability = capabilities.view;
    const commandCapability = capabilities.command;
    const lifecycleCapability = capabilities.lifecycle;
    const inputCapability = capabilities.input;

    const viewId = viewCapability?.getViewId();

    if (!viewId) {
        return null;
    }

    const createInputContext = useCallback(
        (event: PointerEvent | WheelEvent | KeyboardEvent): InputContext => ({
            viewId,
            source: event instanceof KeyboardEvent ? "keyboard" : "pointer",
            target: event.target,
            modifiers: {
                shift: event.shiftKey,
                ctrl: event.ctrlKey,
                alt: event.altKey,
                meta: event.metaKey,
            },
        }),
        [viewId]
    );

    const dispatchCommand = useCallback(
        (command: cmd.AnyCommand | null) => {
            if (command && commandCapability) {
                commandCapability.dispatch(command);
            }
        },
        [commandCapability]
    );

    useEffect(() => {
        if (!renderCapability || !viewCapability || !lifecycleCapability) return;

        let mounted = true;
        let resizeObserver: ResizeObserver | null = null;

        const initializeEditor = async () => {
            if (!containerRef.current) return;

            if (!mounted) return;

            lifecycleCapability.createView();

            await new Promise(requestAnimationFrame);

            if (!mounted) return;

            const container = containerRef.current;
            if (!container) return;

            const canvas = container.querySelector(
                `#node-editor-canvas-${viewId}`
            ) as HTMLCanvasElement;
            if (!canvas) return;

            const width = container.clientWidth;
            const height = container.clientHeight;

            lifecycleCapability.attachView(canvas, width, height);
            if (!(width * height == 0)) {
                renderCapability.setVisible(true);
                renderCapability.requestRender();
            }
            resizeObserver = new ResizeObserver((entries) => {
                for (const entry of entries) {
                    const { width, height } = entry.contentRect;
                    if (width * height == 0) {
                        renderCapability.setVisible(false);
                        continue;
                    }

                    renderCapability.setVisible(true);
                    renderCapability.resize(width, height);
                    renderCapability.requestRender();
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
                renderCapability.setVisible(false);
                lifecycleCapability.detachView();
                lifecycleCapability.destroyView();
            } catch (err) {
                console.warn("Error during view cleanup:", err);
            }
        };
    }, [renderCapability, viewCapability, lifecycleCapability, viewId]);

    useEffect(() => {
        const container = containerRef.current;
        if (!container || !commandCapability || !inputCapability) return;

        const canvas = container.querySelector(
            `#node-editor-canvas-${viewId}`
        ) as HTMLCanvasElement;
        if (!canvas) return;

        registerViewDispatcher(viewId, (command) => {
            commandCapability.dispatch(command);
        });

        let activePointerId: number | null = null;

        const handleWheel = (event: WheelEvent) => {
            setActiveView(viewId);
            event.preventDefault();
            const command = cmd.routeWheelEvent(event, createInputContext(event));
            dispatchCommand(command);
        };

        const handlePointerDown = (event: PointerEvent) => {
            setActiveView(viewId);

            const rect = canvas.getBoundingClientRect();
            const screenX = event.clientX - rect.left;
            const screenY = event.clientY - rect.top;

            inputCapability.handleMouseDown(
                screenX,
                screenY,
                event.button,
                event.shiftKey,
                event.ctrlKey,
                event.altKey
            );

            dragStateRef.current = {
                active: true,
                lastX: event.clientX,
                lastY: event.clientY,
                button: event.button,
            };
            activePointerId = event.pointerId;
            canvas.setPointerCapture(event.pointerId);
        };

        const handlePointerMove = (event: PointerEvent) => {
            const rect = canvas.getBoundingClientRect();
            const screenX = event.clientX - rect.left;
            const screenY = event.clientY - rect.top;

            inputCapability.handleMouseMove(
                screenX,
                screenY,
                event.shiftKey,
                event.ctrlKey,
                event.altKey
            );

            const drag = dragStateRef.current;
            if (!drag.active) return;

            const dx = event.clientX - drag.lastX;
            const dy = event.clientY - drag.lastY;

            if (drag.button === 1) {
                const command = cmd.routePointerDrag(
                    dx,
                    dy,
                    drag.button,
                    createInputContext(event)
                );
                dispatchCommand(command);
            }

            drag.lastX = event.clientX;
            drag.lastY = event.clientY;
        };

        const handlePointerUp = (event: PointerEvent) => {
            const rect = canvas.getBoundingClientRect();
            const screenX = event.clientX - rect.left;
            const screenY = event.clientY - rect.top;

            inputCapability.handleMouseUp(
                screenX,
                screenY,
                event.button,
                event.shiftKey,
                event.ctrlKey,
                event.altKey
            );

            if (activePointerId !== null) {
                try {
                    canvas.releasePointerCapture(activePointerId);
                } catch (err) {
                    // Ignore if pointer capture was already released
                }
                activePointerId = null;
            }
            dragStateRef.current.active = false;
        };
        const handlePointerEnter = (event: PointerEvent) => {
            const rect = canvas.getBoundingClientRect();
            const screenX = event.clientX - rect.left;
            const screenY = event.clientY - rect.top;

            inputCapability.handleMouseEnter(
                screenX,
                screenY,
                event.button,
                event.shiftKey,
                event.ctrlKey,
                event.altKey
            );

        };
        const handlePointerLeave = (event: PointerEvent) => {
            const rect = canvas.getBoundingClientRect();
            const screenX = event.clientX - rect.left;
            const screenY = event.clientY - rect.top;

            inputCapability.handleMouseUp(
                screenX,
                screenY,
                event.button,
                event.shiftKey,
                event.ctrlKey,
                event.altKey
            );

        };

        const handlePointerCancel = (event: PointerEvent) => {
            if (activePointerId !== null) {
                try {
                    canvas.releasePointerCapture(activePointerId);
                } catch (err) {
                    // Ignore if pointer capture was already released
                }
                activePointerId = null;
            }
            dragStateRef.current.active = false;
        };

        const handleKeyDown = (event: KeyboardEvent) => {
            const command = cmd.routeKeyboardEvent(event, createInputContext(event));
            dispatchCommand(command);
        };

        canvas.addEventListener("wheel", handleWheel, { passive: false });
        canvas.addEventListener("pointerdown", handlePointerDown);
        canvas.addEventListener("pointermove", handlePointerMove);
        canvas.addEventListener("pointerup", handlePointerUp);
        canvas.addEventListener("pointerenter", handlePointerEnter);
        canvas.addEventListener("pointerleave", handlePointerLeave);
        canvas.addEventListener("pointercancel", handlePointerCancel);
        canvas.addEventListener("keydown", handleKeyDown);

        canvas.tabIndex = 0;

        return () => {
            if (activePointerId !== null) {
                try {
                    canvas.releasePointerCapture(activePointerId);
                } catch (err) {
                    // Ignore if canvas is already detached
                }
            }
            dragStateRef.current.active = false;

            canvas.removeEventListener("wheel", handleWheel);
            canvas.removeEventListener("pointerdown", handlePointerDown);
            canvas.removeEventListener("pointermove", handlePointerMove);
            canvas.removeEventListener("pointerup", handlePointerUp);
            canvas.removeEventListener("pointercancel", handlePointerCancel);
            canvas.removeEventListener("keydown", handleKeyDown);

            unregisterViewDispatcher(viewId);
        };
    }, [viewId, commandCapability, inputCapability, createInputContext, dispatchCommand]);

    const handleContextMenuAction = useCallback(
        (action: string) => {
            if (!commandCapability) return;

            if (action === "reset-view") {
                const command = cmd.resetView(viewId);
                commandCapability.dispatch(command);
            }
        },
        [viewId, commandCapability]
    );

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
                        <ContextMenu.Item
                            className="flex items-center gap-2 px-3 py-1.5 text-xs text-gray-300 rounded-md hover:bg-white/10 hover:text-white outline-none cursor-pointer transition-colors"
                            onSelect={() => handleContextMenuAction("reset-view")}
                        >
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
