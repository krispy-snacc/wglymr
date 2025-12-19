"use client";

import { useState } from "react";
import * as Collapsible from "@radix-ui/react-collapsible";
import * as Slider from "@radix-ui/react-slider";
import { ChevronDown, ChevronRight, Sliders } from "lucide-react";

export function UniformPanel() {
    const [isOpen, setIsOpen] = useState(true);
    const [seed, setSeed] = useState(0.5);
    const [subtraction, setSubtraction] = useState(0);

    return (
        <Collapsible.Root
            open={isOpen}
            onOpenChange={setIsOpen}
            className="border border-white/10 bg-zinc-950/90 backdrop-blur-md rounded-lg overflow-hidden h-full shadow-lg"
        >
            <Collapsible.Trigger className="w-full flex items-center justify-between px-4 py-3 bg-gradient-to-b from-white/5 to-transparent hover:from-white/8 transition-all group">
                <div className="flex items-center gap-2.5">
                    <div className="p-1 rounded bg-accent/10 group-hover:bg-accent/20 transition-colors">
                        <Sliders className="w-3.5 h-3.5 text-accent" />
                    </div>
                    <span className="text-sm font-semibold text-white">Uniforms</span>
                    <span className="text-xs px-1.5 py-0.5 rounded-full bg-accent/20 text-accent font-medium">2</span>
                </div>
                {isOpen ? (
                    <ChevronDown className="w-4 h-4 text-gray-400 transition-transform" />
                ) : (
                    <ChevronRight className="w-4 h-4 text-gray-400 transition-transform" />
                )}
            </Collapsible.Trigger>

            <Collapsible.Content className="px-4 py-4">
                <div className="space-y-5">
                    {/* Seed Slider */}
                    <div className="space-y-2.5 p-3 rounded-lg bg-white/[0.02] border border-white/5 hover:border-white/10 transition-colors">
                        <div className="flex items-center justify-between">
                            <label className="text-xs font-semibold text-gray-300">seed</label>
                            <span className="text-xs font-mono px-2 py-0.5 rounded bg-accent/10 text-accent border border-accent/20">{seed.toFixed(2)}</span>
                        </div>
                        <Slider.Root
                            className="relative flex items-center select-none touch-none w-full h-6"
                            value={[seed]}
                            onValueChange={(values) => setSeed(values[0])}
                            max={1}
                            step={0.01}
                        >
                            <Slider.Track className="bg-zinc-800/80 relative grow rounded-full h-1.5 shadow-inner">
                                <Slider.Range className="absolute bg-gradient-to-r from-accent to-pink-500 rounded-full h-full shadow-sm" />
                            </Slider.Track>
                            <Slider.Thumb
                                className="block w-4 h-4 bg-white border-2 border-accent rounded-full shadow-md hover:scale-110 hover:shadow-lg hover:shadow-accent/50 focus:outline-none focus:ring-2 focus:ring-accent focus:ring-offset-2 focus:ring-offset-zinc-950 transition-all cursor-grab active:cursor-grabbing"
                                aria-label="Seed"
                            />
                        </Slider.Root>
                    </div>

                    {/* Subtraction Slider */}
                    <div className="space-y-2.5 p-3 rounded-lg bg-white/[0.02] border border-white/5 hover:border-white/10 transition-colors">
                        <div className="flex items-center justify-between">
                            <label className="text-xs font-semibold text-gray-300">subtraction</label>
                            <span className="text-xs font-mono px-2 py-0.5 rounded bg-accent/10 text-accent border border-accent/20">{subtraction.toFixed(2)}</span>
                        </div>
                        <Slider.Root
                            className="relative flex items-center select-none touch-none w-full h-6"
                            value={[subtraction]}
                            onValueChange={(values) => setSubtraction(values[0])}
                            min={0}
                            max={1}
                            step={0.01}
                        >
                            <Slider.Track className="bg-zinc-800/80 relative grow rounded-full h-1.5 shadow-inner">
                                <Slider.Range className="absolute bg-gradient-to-r from-accent to-pink-500 rounded-full h-full shadow-sm" />
                            </Slider.Track>
                            <Slider.Thumb
                                className="block w-4 h-4 bg-white border-2 border-accent rounded-full shadow-md hover:scale-110 hover:shadow-lg hover:shadow-accent/50 focus:outline-none focus:ring-2 focus:ring-accent focus:ring-offset-2 focus:ring-offset-zinc-950 transition-all cursor-grab active:cursor-grabbing"
                                aria-label="Subtraction"
                            />
                        </Slider.Root>
                    </div>
                </div>
            </Collapsible.Content>
        </Collapsible.Root>
    );
}
