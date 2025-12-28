"use client";

import { useEffect, useMemo, useRef } from "react";
import { createRoot, type Root } from "react-dom/client";
import {
    GoldenLayout,
    type ComponentContainer,
    LayoutConfig as LayoutConfigApi,
    type LayoutConfig as LayoutConfigType,
    type JsonValue,
} from "golden-layout";

import "golden-layout/dist/css/goldenlayout-base.css";


import { PanelShell } from "@/components/layout/PanelShell";
import { type EditorType, getEditor } from "@/components/layout/editorRegistry";

type PanelState = {
    viewId: string;
    editorType: EditorType;
};

const STORAGE_KEY = "wglymr.goldenLayout.layout";

function generateUniqueViewId(baseViewId: string, editorType: EditorType): string {
    if (editorType === "nodeEditor") {
        return `${baseViewId}-node-${Date.now()}-${Math.random().toString(36).substr(2, 9)}`;
    }
    return baseViewId;
}

function createDefaultLayoutConfig(viewId: string): LayoutConfigType {
    return {
        settings: {
            reorderEnabled: true,
            constrainDragToContainer: true,
        },
        header: {
            show: "top",
        },
        root: {
            type: "row",
            content: [
                {
                    type: "column",
                    content: [
                        {
                            type: "component",
                            componentType: "panel",
                            title: getEditor("preview").displayName,
                            componentState: { viewId, editorType: "preview" } satisfies PanelState,
                        },
                        {
                            type: "component",
                            componentType: "panel",
                            title: getEditor("metadata").displayName,
                            componentState: { viewId, editorType: "metadata" } satisfies PanelState,
                        },
                    ],
                },
                {
                    type: "column",
                    content: [
                        {
                            type: "component",
                            componentType: "panel",
                            title: getEditor("nodeEditor").displayName,
                            componentState: {
                                viewId: generateUniqueViewId(viewId, "nodeEditor"),
                                editorType: "nodeEditor"
                            } satisfies PanelState,
                        },
                        {
                            type: "stack",
                            content: [
                                {
                                    type: "component",
                                    componentType: "panel",
                                    title: getEditor("uniforms").displayName,
                                    componentState: { viewId, editorType: "uniforms" } satisfies PanelState,
                                },
                                {
                                    type: "component",
                                    componentType: "panel",
                                    title: getEditor("textures").displayName,
                                    componentState: { viewId, editorType: "textures" } satisfies PanelState,
                                },
                            ],
                        },
                    ],
                },
            ],
        },
    };
}

function safeJsonParse(value: string | null): unknown {
    if (!value) return undefined;
    try {
        return JSON.parse(value);
    } catch {
        return undefined;
    }
}

function normaliseLayoutConfigForViewId(config: unknown, viewId: string): LayoutConfigType | undefined {
    if (!config || typeof config !== "object") return undefined;

    const layout = config as LayoutConfigType;

    const rewriteState = (node: unknown) => {
        if (!node || typeof node !== "object") return;
        const item = node as { type?: unknown; componentState?: unknown; content?: unknown };
        if (item.type === "component") {
            const state = item.componentState as PanelState | undefined;
            if (state && typeof state === "object") {
                item.componentState = { ...state, viewId } as PanelState;
            }
        }
        const content = item.content;
        if (Array.isArray(content)) {
            for (const child of content) rewriteState(child);
        }
    };

    rewriteState(layout.root);
    return layout;
}

function coerceToLayoutConfig(config: unknown): LayoutConfigType | undefined {
    if (!config || typeof config !== "object") return undefined;
    if (LayoutConfigApi.isResolved(config as never)) {
        return LayoutConfigApi.fromResolved(config as never);
    }
    return config as LayoutConfigType;
}

interface GoldenLayoutHostProps {
    viewId: string;
    onLayoutReady?: (openEditor: (editorType: EditorType) => void) => void;
}

export function GoldenLayoutHost({ viewId, onLayoutReady }: GoldenLayoutHostProps) {
    const hostRef = useRef<HTMLDivElement | null>(null);
    const glRef = useRef<GoldenLayout | null>(null);
    const registerOnceKey = useMemo(() => "registerOnce", []);

    useEffect(() => {
        const host = hostRef.current;
        if (!host) return;

        const containerRoots = new Map<ComponentContainer, Root>();

        const gl = new GoldenLayout(host);
        glRef.current = gl;

        const mountReactPanel = (container: ComponentContainer, state: JsonValue | undefined) => {
            const panelState = (state ?? {}) as Partial<PanelState>;
            const editorType = panelState.editorType as EditorType | undefined;

            if (!editorType) {
                container.element.textContent = "";
                return;
            }

            // Generate unique viewId for each panel instance
            const baseViewId = typeof panelState.viewId === "string" ? panelState.viewId : viewId;
            const needsUniqueId = editorType === "nodeEditor" && !baseViewId.includes("-node-");
            const effectiveViewId = needsUniqueId ? generateUniqueViewId(viewId, editorType) : baseViewId;

            // Update the state with the unique viewId if we generated a new one
            if (needsUniqueId) {
                container.setState({ ...panelState, viewId: effectiveViewId });
            }

            const ensureRoot = () => {
                const existing = containerRoots.get(container);
                if (existing) return existing;
                container.element.textContent = "";
                const root = createRoot(container.element);
                containerRoots.set(container, root);
                container.on("destroy", () => {
                    const r = containerRoots.get(container);
                    if (!r) return;
                    containerRoots.delete(container);
                    queueMicrotask(() => {
                        try {
                            r.unmount();
                        } catch {
                            // ignore
                        }
                    });
                });
                return root;
            };

            const root = ensureRoot();

            const render = () => {
                const currentState = container.getState() as Partial<PanelState>;
                const currentEditorType = (currentState.editorType || editorType) as EditorType;

                // Get the current viewId from state, or use effectiveViewId
                const currentViewId = (currentState.viewId as string) || effectiveViewId;

                const handleEditorTypeChange = (newEditorType: EditorType) => {
                    const freshState = container.getState() as Partial<PanelState>;
                    const currentViewIdInState = freshState.viewId as string | undefined;

                    // Generate new unique viewId if switching to node editor
                    const newViewId = generateUniqueViewId(
                        currentViewIdInState || viewId,
                        newEditorType
                    );

                    container.setState({
                        ...freshState,
                        editorType: newEditorType,
                        viewId: newViewId
                    });
                    container.setTitle(getEditor(newEditorType).displayName);
                    render();
                };

                root.render(
                    <PanelShell
                        editorType={currentEditorType}
                        onEditorTypeChange={handleEditorTypeChange}
                        viewId={currentViewId}
                    />
                );
            };

            render();

            const rerenderOnStateChange = () => render();
            container.on("stateChanged", rerenderOnStateChange);
            container.on("destroy", () => {
                container.off("stateChanged", rerenderOnStateChange);
            });
        };

        if (!(gl as unknown as { [k: string]: unknown })[registerOnceKey]) {
            (gl as unknown as { [k: string]: unknown })[registerOnceKey] = true;
            const ctor = class {
                constructor(container: ComponentContainer, state: JsonValue | undefined, virtual: boolean) {
                    void virtual;
                    mountReactPanel(container, state);
                }
            };

            gl.registerComponentConstructor("panel", ctor);
        }

        const savedRaw = safeJsonParse(typeof window !== "undefined" ? window.localStorage.getItem(STORAGE_KEY) : null);
        const savedLayoutConfig = coerceToLayoutConfig(savedRaw);
        const saved = normaliseLayoutConfigForViewId(savedLayoutConfig, viewId);
        const config = saved ?? createDefaultLayoutConfig(viewId);
        try {
            gl.loadLayout(config);
        } catch {
            gl.loadLayout(createDefaultLayoutConfig(viewId));
        }

        const save = () => {
            try {
                const resolved = gl.saveLayout();
                const next = LayoutConfigApi.fromResolved(resolved);
                window.localStorage.setItem(STORAGE_KEY, JSON.stringify(next));
            } catch {
                // ignore
            }
        };
        gl.on("stateChanged", save);

        const resize = () => {
            gl.setSize(host.clientWidth, host.clientHeight);
            gl.updateRootSize(true);
        };

        resize();
        const ro = new ResizeObserver(resize);
        ro.observe(host);

        const openEditor = (editorType: EditorType) => {
            const editor = getEditor(editorType);
            const panelViewId = generateUniqueViewId(viewId, editorType);

            try {
                gl.addComponent("panel", { viewId: panelViewId, editorType } satisfies PanelState, editor.displayName);
            } catch (e) {
                console.error("Failed to add new editor panel:", e);
            }
        };

        // Inject "+" buttons into tab bars
        const injectPlusButtons = () => {
            // Find all stacks in the layout
            const findAllStacks = (item: any): any[] => {
                const stacks: any[] = [];
                if (item.isStack) {
                    stacks.push(item);
                }
                if (item.contentItems) {
                    for (const child of item.contentItems) {
                        stacks.push(...findAllStacks(child));
                    }
                }
                return stacks;
            };

            const allStacks = gl.rootItem ? findAllStacks(gl.rootItem) : [];

            // For each stack, find its header and add a button
            allStacks.forEach((stack) => {
                if (!stack.header?.element) return;

                const header = stack.header.element;
                const tabsContainer = header.querySelector(".lm_tabs");
                if (!tabsContainer) return;

                // Check if we already added a button
                if (tabsContainer.querySelector(".lm_add_tab_btn")) return;

                const addBtn = document.createElement("button");
                addBtn.className = "lm_add_tab_btn";
                addBtn.setAttribute("title", "Add new tab");
                addBtn.innerHTML = "+";

                // Store stack reference directly on button
                (addBtn as any).__stack = stack;

                addBtn.onclick = (e) => {
                    e.preventDefault();
                    e.stopPropagation();

                    const targetStack = (e.currentTarget as any).__stack;

                    if (!targetStack) {
                        console.error("No stack reference found on button");
                        return;
                    }

                    try {
                        // Create the component config with appropriate viewId
                        const newEditorType: EditorType = "uniforms";
                        const panelViewId = generateUniqueViewId(viewId, newEditorType);

                        const componentConfig = {
                            type: "component" as const,
                            componentType: "panel",
                            title: getEditor(newEditorType).displayName,
                            componentState: { viewId: panelViewId, editorType: newEditorType } satisfies PanelState,
                        };

                        // Use addItem instead of newItem + addChild
                        targetStack.addItem(componentConfig);
                    } catch (error) {
                        console.error("Failed to add tab:", error);
                    }
                };

                tabsContainer.appendChild(addBtn);
            });
        };

        // Initial injection
        setTimeout(injectPlusButtons, 100);

        // Re-inject when layout changes
        gl.on("stateChanged", () => {
            setTimeout(injectPlusButtons, 50);
        });

        if (onLayoutReady) {
            onLayoutReady(openEditor);
        }

        return () => {
            ro.disconnect();
            gl.off("stateChanged", save);
            gl.destroy();
            glRef.current = null;
            const roots = Array.from(containerRoots.values());
            containerRoots.clear();
            queueMicrotask(() => {
                for (const root of roots) {
                    try {
                        root.unmount();
                    } catch {
                        // ignore
                    }
                }
            });
        };
    }, [registerOnceKey, viewId]);

    return (
        <div
            ref={hostRef}
            className="w-full h-full min-h-0"
            style={{ height: "100%" }}
        />
    );
}
