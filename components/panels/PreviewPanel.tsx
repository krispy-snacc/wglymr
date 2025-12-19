"use client";

import { useState, useEffect, useRef } from "react";
import * as ContextMenu from "@radix-ui/react-context-menu";
import { Play, Pause, RotateCcw, Maximize2 } from "lucide-react";

interface PreviewPanelProps {
    viewId: string;
}

export function PreviewPanel({ viewId }: PreviewPanelProps) {
    const [isPlaying, setIsPlaying] = useState(true);
    const [fps, setFps] = useState(60);
    const [resolution, setResolution] = useState({ width: 896, height: 504 });
    const canvasRef = useRef<HTMLCanvasElement>(null);

    // Mock FPS counter
    useEffect(() => {
        if (!isPlaying) return;
        const interval = setInterval(() => {
            setFps(Math.floor(58 + Math.random() * 4));
        }, 100);
        return () => clearInterval(interval);
    }, [isPlaying]);

    // Update resolution on resize
    useEffect(() => {
        const updateResolution = () => {
            if (canvasRef.current) {
                const rect = canvasRef.current.getBoundingClientRect();
                setResolution({ width: Math.floor(rect.width), height: Math.floor(rect.height) });
            }
        };
        updateResolution();
        window.addEventListener("resize", updateResolution);
        return () => window.removeEventListener("resize", updateResolution);
    }, []);

    return (
        <div className="w-full h-full flex flex-col bg-zinc-950/80 backdrop-blur-md overflow-hidden">
            {/* Canvas Area */}
            <ContextMenu.Root>
                <ContextMenu.Trigger asChild>
                    <div className="flex-1 relative flex items-center justify-center min-h-0">
                        {/* Canvas Placeholder - TODO: Mount WebGPU context here */}
                        <canvas
                            ref={canvasRef}
                            className="w-full h-full object-contain"
                            style={{
                                background: "radial-gradient(circle at 50% 50%, #1a1a2e 0%, #0a0a14 100%)",
                            }}
                        />
                    </div>
                </ContextMenu.Trigger>

                <ContextMenu.Portal>
                    <ContextMenu.Content className="min-w-55 bg-zinc-900/95 backdrop-blur-lg border border-white/10 rounded-lg shadow-2xl p-1.5">
                        <ContextMenu.Item className="flex items-center gap-2 px-3 py-1.5 text-xs text-gray-300 rounded-md hover:bg-white/10 hover:text-white outline-none cursor-pointer transition-colors">
                            Export Image
                        </ContextMenu.Item>
                        <ContextMenu.Item className="flex items-center gap-2 px-3 py-1.5 text-xs text-gray-300 rounded-md hover:bg-white/10 hover:text-white outline-none cursor-pointer transition-colors">
                            Copy to Clipboard
                        </ContextMenu.Item>
                        <ContextMenu.Separator className="h-px bg-white/10 my-1" />
                        <ContextMenu.Item className="flex items-center gap-2 px-3 py-1.5 text-xs text-gray-300 rounded-md hover:bg-white/10 hover:text-white outline-none cursor-pointer transition-colors">
                            Reset Camera
                        </ContextMenu.Item>
                    </ContextMenu.Content>
                </ContextMenu.Portal>
            </ContextMenu.Root>

            {/* Control Bar */}
            <div className="shrink-0 bg-zinc-900/80 backdrop-blur-sm border-t border-white/10 px-2 py-1.5 flex items-center justify-between gap-2">
                {/* Left: Control Buttons */}
                <div className="flex gap-1">
                    <button
                        onClick={() => setIsPlaying(!isPlaying)}
                        className="border border-white/10 hover:border-accent hover:bg-accent/10 rounded p-1 text-gray-300 hover:text-white transition-colors"
                        title={isPlaying ? "Pause" : "Play"}
                    >
                        {isPlaying ? <Pause className="w-3 h-3" /> : <Play className="w-3 h-3" />}
                    </button>
                    <button
                        onClick={() => console.log("Reset view")}
                        className="border border-white/10 hover:border-accent hover:bg-accent/10 rounded p-1 text-gray-300 hover:text-white transition-colors"
                        title="Reset"
                    >
                        <RotateCcw className="w-3 h-3" />
                    </button>
                    <button
                        onClick={() => console.log("Fullscreen")}
                        className="border border-white/10 hover:border-accent hover:bg-accent/10 rounded p-1 text-gray-300 hover:text-white transition-colors"
                        title="Fullscreen"
                    >
                        <Maximize2 className="w-3 h-3" />
                    </button>
                </div>

                {/* Right: Stats */}
                <div className="flex items-center gap-2 sm:gap-3 font-mono text-[10px] text-gray-400">
                    <div className="flex items-center gap-1">
                        <span className="text-gray-500 hidden sm:inline">FPS</span>
                        <span className="text-green-400 font-medium">{fps}</span>
                    </div>
                    <div className="h-2.5 w-px bg-white/10" />
                    <div className="flex items-center gap-1">
                        <span className="text-gray-500 hidden sm:inline">Resolution</span>
                        <span className="text-gray-300">{resolution.width}×{resolution.height}</span>
                    </div>
                </div>
            </div>
        </div>
    );
}
