"use client";

import * as Slider from "@radix-ui/react-slider";
import { useDocumentStore } from "@/document/useDocumentStore";
import { documentStore } from "@/document/documentStore";

interface UniformPanelProps {
    panelId: string;
    viewId?: string;
}

export function UniformPanel({ panelId }: UniformPanelProps) {
    const { uniforms } = useDocumentStore();

    return (
        <div className="h-full overflow-y-auto p-3 bg-zinc-950">
            <div className="max-w-2xl space-y-5">
                {/* Seed Slider */}
                <div className="space-y-3 p-3 rounded bg-white/2 border border-white/6 hover:bg-white/3 hover:border-white/8 transition-colors">
                    <div className="flex items-center justify-between">
                        <label className="text-xs font-medium text-gray-400">seed</label>
                        <span className="text-xs font-mono px-2 py-0.5 rounded bg-accent/10 text-accent/90 border border-accent/20">{uniforms.seed.toFixed(2)}</span>
                    </div>
                    <Slider.Root
                        className="relative flex items-center select-none touch-none w-full h-5"
                        value={[uniforms.seed]}
                        onValueChange={(values) => documentStore.setUniform("seed", values[0])}
                        max={1}
                        step={0.01}
                    >
                        <Slider.Track className="bg-zinc-800/60 relative grow rounded-full h-1 shadow-inner">
                            <Slider.Range className="absolute bg-accent/60 rounded-full h-full" />
                        </Slider.Track>
                        <Slider.Thumb
                            className="block w-3 h-3 bg-white border border-accent/40 rounded-full shadow-sm hover:scale-110 focus:outline-none focus:border-accent transition-all cursor-grab active:cursor-grabbing"
                            aria-label="Seed"
                        />
                    </Slider.Root>
                </div>

                {/* Subtraction Slider */}
                <div className="space-y-3 p-3 rounded bg-white/2 border border-white/6 hover:bg-white/3 hover:border-white/8 transition-colors">
                    <div className="flex items-center justify-between">
                        <label className="text-xs font-medium text-gray-400">subtraction</label>
                        <span className="text-xs font-mono px-2 py-0.5 rounded bg-accent/10 text-accent/90 border border-accent/20">{uniforms.subtraction.toFixed(2)}</span>
                    </div>
                    <Slider.Root
                        className="relative flex items-center select-none touch-none w-full h-5"
                        value={[uniforms.subtraction]}
                        onValueChange={(values) => documentStore.setUniform("subtraction", values[0])}
                        min={0}
                        max={1}
                        step={0.01}
                    >
                        <Slider.Track className="bg-zinc-800/60 relative grow rounded-full h-1 shadow-inner">
                            <Slider.Range className="absolute bg-accent/60 rounded-full h-full" />
                        </Slider.Track>
                        <Slider.Thumb
                            className="block w-3 h-3 bg-white border border-accent/40 rounded-full shadow-sm hover:scale-110 focus:outline-none focus:border-accent transition-all cursor-grab active:cursor-grabbing"
                            aria-label="Subtraction"
                        />
                    </Slider.Root>
                </div>
            </div>
        </div>
    );
}
