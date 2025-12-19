"use client";

import { use, useState, useEffect } from "react";
import { Panel, Group } from "react-resizable-panels";
import { Navbar } from "@/components/navbar/Navbar";
import { PreviewPanel } from "@/components/panels/PreviewPanel";
import { UniformPanel } from "@/components/panels/UniformPanel";
import { NodeEditorHost } from "@/components/node-editor/NodeEditorHost";
import { InspectorPanel } from "@/components/panels/InspectorPanel";
import { ClientOnly } from "@/components/layout/ClientOnly";
import { LoadingScreen } from "@/components/layout/LoadingScreen";
import { Monitor } from "lucide-react";

interface PageProps {
    params: Promise<{ view_id: string }>;
}

export default function ViewPage({ params }: PageProps) {
    const { view_id } = use(params);
    const [isDesktop, setIsDesktop] = useState(false);
    const [isMounted, setIsMounted] = useState(false);

    useEffect(() => {
        const checkScreenSize = () => {
            setIsDesktop(window.innerWidth >= 1024); // lg breakpoint
        };

        checkScreenSize();
        window.addEventListener('resize', checkScreenSize);

        // Small delay to ensure smooth loading experience
        const mountTimer = setTimeout(() => setIsMounted(true), 300);

        return () => {
            window.removeEventListener('resize', checkScreenSize);
            clearTimeout(mountTimer);
        };
    }, []);

    return (
        <>
            <LoadingScreen isLoading={!isMounted} />
            <div className="w-screen flex flex-col text-white overflow-hidden" style={{ height: 'calc(var(--vh, 1vh) * 100)' }}>
                {/* Navbar */}
                <Navbar />

                {/* Main Workspace */}
                <div className="flex-1 overflow-y-auto lg:overflow-hidden lg:p-2" style={{ height: 'calc(var(--vh, 1vh) * 100 - 3rem)' }}>
                    {isMounted && (isDesktop ? (
                        // Desktop Layout: Horizontal split with resizable panels
                        <Group orientation="horizontal" id="workspace" style={{ height: '100%', gap: '8px' }}>
                            {/* Left Column: Preview + Uniforms */}
                            <Panel defaultSize={40} minSize={25} id="left-column">
                                <Group orientation="vertical" id="left-panels" style={{ height: '100%', gap: '8px' }}>
                                    {/* Preview Panel */}
                                    <Panel defaultSize={70} minSize={30} id="preview">
                                        <div className="h-full rounded-lg overflow-hidden border border-white/10">
                                            <PreviewPanel viewId={view_id} />
                                        </div>
                                    </Panel>

                                    {/* Uniform Controls */}
                                    <Panel defaultSize={30} minSize={15} id="uniforms">
                                        <div className="h-full overflow-y-auto rounded-lg">
                                            <UniformPanel />
                                        </div>
                                    </Panel>
                                </Group>
                            </Panel>

                            {/* Right Column: Node Editor + Inspector */}
                            <Panel defaultSize={60} minSize={35} id="right-column">
                                <Group orientation="vertical" id="right-panels" style={{ height: '100%', gap: '8px' }}>
                                    {/* Node Editor */}
                                    <Panel defaultSize={75} minSize={40} id="node-editor">
                                        <div className="h-full rounded-lg overflow-hidden border border-white/10">
                                            <NodeEditorHost viewId={view_id} />
                                        </div>
                                    </Panel>

                                    {/* Inspector Panel */}
                                    <Panel defaultSize={25} minSize={15} id="inspector">
                                        <div className="h-full overflow-y-auto rounded-lg">
                                            <ClientOnly>
                                                <InspectorPanel />
                                            </ClientOnly>
                                        </div>
                                    </Panel>
                                </Group>
                            </Panel>
                        </Group>
                    ) : (
                        // Mobile Layout: Vertical stack without resizing
                        <div className="flex flex-col gap-2 p-2 pb-4">
                            {/* Preview Panel */}
                            <div className="flex-shrink-0 rounded-lg border border-white/10 overflow-hidden">
                                <div style={{ height: 'calc(40 * var(--vh, 1vh))', minHeight: '300px' }}>
                                    <PreviewPanel viewId={view_id} />
                                </div>
                            </div>

                            {/* Uniform Controls */}
                            <div className="flex-shrink-0 rounded-lg">
                                <UniformPanel />
                            </div>

                            {/* Inspector Panel */}
                            <div className="flex-shrink-0 rounded-lg">
                                <ClientOnly>
                                    <InspectorPanel />
                                </ClientOnly>
                            </div>

                            {/* Node Editor - Desktop Only Message */}
                            <div className="flex-shrink-0 rounded-lg overflow-hidden border border-white/10 bg-zinc-950/80 backdrop-blur-md">
                                <div className="flex flex-col items-center justify-center p-8 text-center space-y-4 min-h-[200px]">
                                    <Monitor className="w-12 h-12 text-gray-600" />
                                    <div>
                                        <h3 className="text-lg font-semibold text-gray-300 mb-2">Node Editor (Desktop Only)</h3>
                                        <p className="text-sm text-gray-500 max-w-md">
                                            The visual node editor requires a larger screen and is optimized for mouse/keyboard interaction.
                                            Please use a desktop browser to access this feature.
                                        </p>
                                    </div>
                                    <p className="text-xs text-gray-600 italic">
                                        Touch support may be added in the future
                                    </p>
                                </div>
                            </div>
                        </div>
                    ))}
                </div>
            </div>
        </>
    );
}
