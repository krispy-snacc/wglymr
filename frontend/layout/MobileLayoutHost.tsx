"use client";

import { Monitor } from "lucide-react";
import { PanelShell } from "@/layout/PanelShell";
import { ClientOnly } from "@/layout/ClientOnly";

interface MobileLayoutHostProps {
    glymId: string;
}

export function MobileLayoutHost({ glymId }: MobileLayoutHostProps) {
    const noOpEditorChange = () => { };

    return (
        <div className="flex flex-col gap-2 p-2 pb-4">
            {/* Preview */}
            <div className="shrink-0 rounded-lg border border-white/10 overflow-hidden">
                <div style={{ height: "calc(40 * var(--vh, 1vh))", minHeight: 300 }}>
                    <PanelShell
                        panelId={`mobile:${glymId}:preview`}
                        editorType="preview"
                        viewId={`view:${glymId}:preview`}
                        onEditorTypeChange={noOpEditorChange}
                    />
                </div>
            </div>

            {/* Uniforms */}
            <div className="shrink-0 rounded-lg overflow-hidden border border-white/10 bg-zinc-950/80 backdrop-blur-md">
                <PanelShell
                    panelId={`mobile:${glymId}:uniforms`}
                    editorType="uniforms"
                    onEditorTypeChange={noOpEditorChange}
                />
            </div>

            {/* Textures */}
            <div className="shrink-0 rounded-lg overflow-hidden border border-white/10 bg-zinc-950/80 backdrop-blur-md">
                <ClientOnly>
                    <PanelShell
                        panelId={`mobile:${glymId}:textures`}
                        editorType="textures"
                        onEditorTypeChange={noOpEditorChange}
                    />
                </ClientOnly>
            </div>

            {/* Node Editor Placeholder */}
            <div className="shrink-0 rounded-lg overflow-hidden border border-white/10 bg-zinc-950/80 backdrop-blur-md">
                <div className="flex flex-col items-center justify-center p-8 text-center space-y-4 min-h-50">
                    <Monitor className="w-12 h-12 text-gray-600" />
                    <div>
                        <h3 className="text-lg font-semibold text-gray-300 mb-2">
                            Node Editor (Desktop Only)
                        </h3>
                        <p className="text-sm text-gray-500 max-w-md">
                            The visual node editor requires a larger screen and is optimized for mouse and keyboard interaction.
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
