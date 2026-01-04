"use client";

import { useEffect } from "react";
import { getActiveView } from "@/layout/activeViewTracker";
import { dispatchToView } from "@/layout/commandDispatchRegistry";
import * as cmd from "@/commands";

// Prevent all browser zoom and capture window-level gestures for canvas navigation
export function useGlobalInputCapture() {
    useEffect(() => {
        const handleWindowWheel = (event: WheelEvent) => {
            const target = event.target as HTMLElement;
            const isCanvas = target.tagName === "CANVAS" && target.id.startsWith("node-editor-canvas-");

            if (isCanvas) {
                return;
            }

            if (event.ctrlKey || event.metaKey) {
                event.preventDefault();
                event.stopPropagation();
            }
        };

        const handleKeyDown = (event: KeyboardEvent) => {
            if ((event.ctrlKey || event.metaKey) && (event.key === "+" || event.key === "-" || event.key === "=" || event.key === "_")) {
                event.preventDefault();

                const activeViewId = getActiveView();
                if (!activeViewId) {
                    return;
                }

                const isZoomIn = event.key === "+" || event.key === "=";
                const zoomFactor = isZoomIn ? 1.1 : 0.9;

                const activeCanvas = document.querySelector(`#node-editor-canvas-${activeViewId}`) as HTMLCanvasElement;
                if (!activeCanvas) return;

                const rect = activeCanvas.getBoundingClientRect();
                const centerX = rect.width / 2;
                const centerY = rect.height / 2;

                const command = cmd.zoomView(activeViewId, zoomFactor, centerX, centerY);
                dispatchToView(activeViewId, command);
            }

            if (event.ctrlKey && event.key === "0") {
                event.preventDefault();

                const activeViewId = getActiveView();
                if (!activeViewId) {
                    return;
                }

                const command = cmd.resetView(activeViewId);
                dispatchToView(activeViewId, command);
            }
        };

        window.addEventListener("wheel", handleWindowWheel, { passive: false });
        window.addEventListener("keydown", handleKeyDown, { passive: false });

        return () => {
            window.removeEventListener("wheel", handleWindowWheel);
            window.removeEventListener("keydown", handleKeyDown);
        };
    }, []);
}
