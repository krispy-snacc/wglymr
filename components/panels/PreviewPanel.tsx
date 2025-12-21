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
    const [time, setTime] = useState(0);
    const [resolution, setResolution] = useState({ width: 896, height: 504 });
    const canvasRef = useRef<HTMLCanvasElement>(null);
    const startTimeRef = useRef(Date.now());

    // Mock FPS counter and time
    useEffect(() => {
        if (!isPlaying) return;
        const interval = setInterval(() => {
            setFps(Math.floor(58 + Math.random() * 4));
            setTime((Date.now() - startTimeRef.current) / 1000);
        }, 100);
        return () => clearInterval(interval);
    }, [isPlaying]);

    // Reset time when play/pause
    useEffect(() => {
        if (isPlaying) {
            startTimeRef.current = Date.now() - time * 1000;
        }
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
            <div className="shrink-0 bg-zinc-900/60 backdrop-blur-sm border-t border-white/6 px-2 py-1 flex items-center justify-between gap-3">
                {/* Left: Control Buttons */}
                <div className="flex gap-1">
                    <button
                        onClick={() => setIsPlaying(!isPlaying)}
                        className="border border-white/8 hover:border-accent/60 hover:bg-accent/5 rounded p-1.5 text-gray-400 hover:text-white transition-colors cursor-pointer"
                        title={isPlaying ? "Pause" : "Play"}
                    >
                        {isPlaying ? <Pause className="w-3.5 h-3.5" /> : <Play className="w-3.5 h-3.5" />}
                    </button>
                    <button
                        onClick={() => { setTime(0); startTimeRef.current = Date.now(); }}
                        className="border border-white/8 hover:border-accent/60 hover:bg-accent/5 rounded p-1.5 text-gray-400 hover:text-white transition-colors cursor-pointer"
                        title="Reset Time"
                    >
                        <RotateCcw className="w-3.5 h-3.5" />
                    </button>
                    <button
                        onClick={() => console.log("Fullscreen")}
                        className="border border-white/8 hover:border-accent/60 hover:bg-accent/5 rounded p-1.5 text-gray-400 hover:text-white transition-colors cursor-pointer"
                        title="Fullscreen"
                    >
                        <Maximize2 className="w-3.5 h-3.5" />
                    </button>
                </div>

                {/* Right: Stats */}
                <div className="flex items-center gap-2 sm:gap-3 font-mono text-[10px] font-medium text-gray-500">
                    <div className="flex items-center gap-1.5">
                        <span className="hidden sm:inline">Time</span>
                        <span className="text-accent/90">{time.toFixed(2)}s</span>
                    </div>
                    <div className="h-3 w-px bg-white/8" />
                    <div className="flex items-center gap-1.5">
                        <span className="hidden sm:inline">FPS</span>
                        <span className="text-green-400/80">{fps}</span>
                    </div>
                    <div className="h-3 w-px bg-white/8" />
                    <div className="flex items-center gap-1.5">
                        <span className="hidden sm:inline">Res</span>
                        <span className="text-gray-400">{resolution.width}×{resolution.height}</span>
                    </div>
                </div>
            </div>
        </div>
    );
}
