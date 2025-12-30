"use client";

import { useEffect, useMemo, useRef, useState } from "react";
import { GoldenLayout, type ComponentContainer, type JsonValue } from "golden-layout";

import "golden-layout/dist/css/goldenlayout-base.css";

import { type EditorType } from "@/layout/editorRegistry";
import { createDefaultLayoutConfig } from "@/layout/defaultLayout";
import { loadLayout, saveLayout } from "@/layout/persistence";
import { createPanelRenderer } from "@/layout/panelRenderer";
import { createEditorSpawner } from "@/layout/editorSpawner";
import { injectPlusButtons } from "@/layout/uiEnhancers";
import { EmptyLayoutOverlay } from "@/layout/EmptyLayoutOverlay";

interface GoldenLayoutHostProps {
    glymId: string;
    onLayoutReady?: (openEditor: (editorType: EditorType) => void) => void;
}

export function GoldenLayoutHost({ glymId, onLayoutReady }: GoldenLayoutHostProps) {
    const hostRef = useRef<HTMLDivElement | null>(null);
    const glRef = useRef<GoldenLayout | null>(null);
    const registerOnceKey = useMemo(() => "registerOnce", []);
    const [showEmptyOverlay, setShowEmptyOverlay] = useState(false);

    useEffect(() => {
        const host = hostRef.current;
        if (!host) return;

        let alive = true;
        let layoutReady = false;

        const gl = new GoldenLayout(host);
        glRef.current = gl;

        const { mountReactPanel, cleanup } = createPanelRenderer(glymId);

        if (!(gl as unknown as { [k: string]: unknown })[registerOnceKey]) {
            (gl as unknown as { [k: string]: unknown })[registerOnceKey] = true;

            const ctor = class {
                constructor(
                    container: ComponentContainer,
                    state: JsonValue | undefined,
                    virtual: boolean
                ) {
                    void virtual;
                    mountReactPanel(container, state);
                }
            };

            gl.registerComponentConstructor("panel", ctor);
        }

        // ---- Load layout --------------------------------------------------

        const saved = loadLayout();

        if (saved !== "EMPTY") {
            if (saved) {
                gl.loadLayout(saved);
            } else {
                gl.loadLayout(createDefaultLayoutConfig(glymId));
            }
        }

        layoutReady = true;

        // ---- Empty detection ----------------------------------------------

        function hasAnyComponent(item: any): boolean {
            if (!item) return false;
            if (item.type === "component") return true;
            if (Array.isArray(item.contentItems)) {
                return item.contentItems.some(hasAnyComponent);
            }
            return false;
        }

        function isLayoutEmpty(): boolean {
            return !hasAnyComponent(gl.rootItem);
        }

        const syncLayoutState = () => {
            if (!alive) return;
            if (!layoutReady) return;
            if (!gl.isInitialised) return;
            if (!gl.rootItem) return;
            console.log("Hello there");
            queueMicrotask(() => {
                if (!alive) return;

                const empty = isLayoutEmpty();
                saveLayout(gl.saveLayout(), empty);
                setShowEmptyOverlay(empty);
                injectPlusButtons(gl, glymId);
            });
        };

        gl.on("__all", syncLayoutState);

        // ---- Resize -------------------------------------------------------

        const resize = () => {
            if (!alive) return;
            gl.setSize(host.clientWidth, host.clientHeight);
            gl.updateRootSize(true);
        };

        resize();
        const ro = new ResizeObserver(resize);
        ro.observe(host);

        setTimeout(() => {
            if (!alive) return;
            injectPlusButtons(gl, glymId);
            setShowEmptyOverlay(isLayoutEmpty());
        }, 100);

        const { openEditor } = createEditorSpawner(gl, glymId);
        onLayoutReady?.(openEditor);

        // ---- Cleanup ------------------------------------------------------

        return () => {
            alive = false;
            ro.disconnect();
            gl.destroy();
            glRef.current = null;
            cleanup();
        };
    }, [registerOnceKey, glymId, onLayoutReady]);

    const handleCreatePanel = (editorType: EditorType) => {
        const gl = glRef.current;
        if (!gl) return;

        const { openEditor } = createEditorSpawner(gl, glymId);
        openEditor(editorType);
    };

    const handleRestoreDefaultLayout = () => {
        const gl = glRef.current;
        if (!gl) return;

        gl.loadLayout(createDefaultLayoutConfig(glymId));
        saveLayout(gl.saveLayout(), false);
    };


    return (
        <div className="relative w-full h-full min-h-0">
            <div ref={hostRef} className="w-full h-full min-h-0" />
            {showEmptyOverlay && (
                <EmptyLayoutOverlay onCreatePanel={handleCreatePanel} onRestoreDefaultLayout={handleRestoreDefaultLayout} />
            )}
        </div>
    );
}
