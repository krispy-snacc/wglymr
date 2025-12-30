"use client";

import { useMemo } from "react";
import { Monitor } from "lucide-react";
import { PanelShell } from "@/layout/PanelShell";
import { ClientOnly } from "@/layout/ClientOnly";

function generatePanelId(): string {
    return `panel-${Date.now()}-${Math.random().toString(36).slice(2, 2 + 9)}`;
}

function generateViewId(glymId: string, panelId: string): string {
    return `${glymId}-${panelId}`;
}

interface MobileLayoutHostProps {
    glymId: string;
}

export function MobileLayoutHost({ glymId }: MobileLayoutHostProps) {
    const panelIds = useMemo(() => ({
        preview: generatePanelId(),
        uniforms: generatePanelId(),
        textures: generatePanelId(),
    }), []);

    const viewIds = useMemo(() => ({
        preview: generateViewId(glymId, panelIds.preview),
    }), [glymId, panelIds.preview]);

    const noOpEditorChange = () => { };

    return (
        <div className="flex flex-col gap-2 p-2 pb-4">
            <div className="shrink-0 rounded-lg border border-white/10 overflow-hidden">
                <div style={{ height: 'calc(40 * var(--vh, 1vh))', minHeight: '300px' }}>
                    <PanelShell
                        panelId={panelIds.preview}
                        editorType="preview"
                        viewId={viewIds.preview}
                        onEditorTypeChange={noOpEditorChange}
                    />
                </div>
            </div>

            <div className="shrink-0 rounded-lg overflow-hidden border border-white/10 bg-zinc-950/80 backdrop-blur-md">
                <PanelShell
                    panelId={panelIds.uniforms}
                    editorType="uniforms"
                    onEditorTypeChange={noOpEditorChange}
                />
            </div>

            <div className="shrink-0 rounded-lg overflow-hidden border border-white/10 bg-zinc-950/80 backdrop-blur-md">
                <ClientOnly>
                    <PanelShell
                        panelId={panelIds.textures}
                        editorType="textures"
                        onEditorTypeChange={noOpEditorChange}
                    />
                </ClientOnly>
            </div>

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
    );
}
