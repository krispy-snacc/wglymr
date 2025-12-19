"use client";

import { useState } from "react";
import * as Collapsible from "@radix-ui/react-collapsible";
import * as Switch from "@radix-ui/react-switch";
import { ChevronDown, ChevronRight, Settings2 } from "lucide-react";

export function InspectorPanel() {
    const [isOpen, setIsOpen] = useState(true);
    const [float32Textures, setFloat32Textures] = useState(true);

    return (
        <Collapsible.Root
            open={isOpen}
            onOpenChange={setIsOpen}
            className="border border-white/10 bg-zinc-950/90 backdrop-blur-md rounded-lg overflow-hidden h-full shadow-lg"
        >
            <Collapsible.Trigger className="w-full flex items-center justify-between px-4 py-3 bg-linear-to-b from-white/5 to-transparent hover:from-white/8 transition-all group">
                <div className="flex items-center gap-2.5">
                    <div className="p-1 rounded bg-blue-500/10 group-hover:bg-blue-500/20 transition-colors">
                        <Settings2 className="w-3.5 h-3.5 text-blue-400" />
                    </div>
                    <span className="text-sm font-semibold text-white">Inspector</span>
                </div>
                {isOpen ? (
                    <ChevronDown className="w-4 h-4 text-gray-400 transition-transform" />
                ) : (
                    <ChevronRight className="w-4 h-4 text-gray-400 transition-transform" />
                )}
            </Collapsible.Trigger>

            <Collapsible.Content className="px-4 py-4 space-y-4">
                {/* Render Settings */}
                <div>
                    <h3 className="text-xs font-bold text-gray-200 mb-3 uppercase tracking-wider flex items-center gap-2">
                        <div className="w-1 h-3 bg-blue-400 rounded-full"></div>
                        Render Settings
                    </h3>
                    <div className="space-y-3">
                        {/* Float32 Textures Toggle */}
                        <div className="flex items-center justify-between p-3 rounded-lg bg-white/2 border border-white/5 hover:border-white/10 transition-colors group">
                            <div className="flex flex-col gap-0.5">
                                <label className="text-xs font-semibold text-gray-300 cursor-pointer">Float32 Textures</label>
                                <span className="text-[10px] text-gray-500">Enable high precision rendering</span>
                            </div>
                            <Switch.Root
                                checked={float32Textures}
                                onCheckedChange={setFloat32Textures}
                                className="w-11 h-6 bg-zinc-800 rounded-full relative data-[state=checked]:bg-linear-to-r data-[state=checked]:from-accent data-[state=checked]:to-pink-500 shadow-inner transition-all focus:outline-none focus:ring-2 focus:ring-accent focus:ring-offset-2 focus:ring-offset-zinc-950 group-hover:shadow-lg"
                            >
                                <Switch.Thumb className="block w-5 h-5 bg-white rounded-full shadow-md transition-transform translate-x-0.5 data-[state=checked]:translate-x-5" />
                            </Switch.Root>
                        </div>
                    </div>
                </div>
            </Collapsible.Content>
        </Collapsible.Root>
    );
}
