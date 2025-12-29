"use client";

import { use, useState, useEffect } from "react";
import { Navbar } from "@/ui/Navbar";
import { PreviewPanel } from "@/panels/PreviewPanel";
import { UniformPanel } from "@/panels/UniformPanel";
import { InspectorPanel } from "@/panels/InspectorPanel";
import { ClientOnly } from "@/layout/ClientOnly";
import { LoadingScreen } from "@/layout/LoadingScreen";
import { GoldenLayoutHost } from "@/layout/GoldenLayoutHost";
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
                <div className="flex-1 overflow-y-auto lg:overflow-hidden" style={{ height: 'calc(var(--vh, 1vh) * 100 - 3rem)' }}>
                    {isMounted && (isDesktop ? (
                        <div className="h-full min-h-0  overflow-hidden px-1 bg-black">
                            <GoldenLayoutHost viewId={view_id} />
                        </div>
                    ) : (
                        // Mobile Layout: Vertical stack without resizing
                        <div className="flex flex-col gap-2 p-2 pb-4">
                            {/* Preview Panel */}
                            <div className="shrink-0 rounded-lg border border-white/10 overflow-hidden">
                                <div style={{ height: 'calc(40 * var(--vh, 1vh))', minHeight: '300px' }}>
                                    <PreviewPanel panelId="mobile-preview" />
                                </div>
                            </div>

                            {/* Uniform Controls */}
                            <div className="shrink-0 rounded-lg">
                                <UniformPanel panelId="mobile-uniforms" />
                            </div>

                            {/* Inspector Panel */}
                            <div className="shrink-0 rounded-lg">
                                <ClientOnly>
                                    <InspectorPanel panelId="mobile-inspector" />
                                </ClientOnly>
                            </div>

                            {/* Node Editor - Desktop Only Message */}
                            <div className="shrink-0 rounded-lg overflow-hidden border border-white/10 bg-zinc-950/80 backdrop-blur-md">
                                <div className="flex flex-col items-center justify-center p-8 text-center space-y-4 min-h-50">
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
