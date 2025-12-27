"use client";

import { useDocumentStore } from "@/lib/useDocumentStore";
import { documentStore } from "@/lib/documentStore";

export function MetadataPanel() {
    const { title, description, isPublic } = useDocumentStore();

    return (
        <div className="h-full overflow-y-auto p-3 bg-zinc-950">
            <div className="max-w-2xl space-y-6">
                {/* Title */}
                <div className="space-y-2">
                    <label className="text-xs font-semibold text-gray-300">Title</label>
                    <input
                        type="text"
                        value={title}
                        onChange={(e) => documentStore.setState({ title: e.target.value })}
                        className="w-full px-3 py-2 bg-zinc-900/50 border border-white/8 rounded text-sm text-white placeholder:text-gray-600 focus:outline-none focus:bg-zinc-900/70 focus:border-white/20 transition-all"
                        placeholder="Untitled Shader"
                    />
                </div>

                {/* Description */}
                <div className="space-y-2">
                    <label className="text-xs font-semibold text-gray-300">Description</label>
                    <textarea
                        value={description}
                        onChange={(e) => documentStore.setState({ description: e.target.value })}
                        className="w-full px-3 py-2 bg-zinc-900/50 border border-white/8 rounded text-sm text-white placeholder:text-gray-600 focus:outline-none focus:bg-zinc-900/70 focus:border-white/20 transition-all resize-none"
                        placeholder="Add a description..."
                        rows={4}
                    />
                </div>

                {/* Visibility */}
                <div className="flex items-center justify-between p-3 rounded bg-white/2 border border-white/6 hover:bg-white/3 hover:border-white/8 transition-colors">
                    <div className="flex flex-col gap-1">
                        <label className="text-xs font-medium text-gray-300">Public</label>
                        <span className="text-[10px] text-gray-600">Make this shader visible to others</span>
                    </div>
                    <label className="relative inline-flex items-center cursor-pointer">
                        <input
                            type="checkbox"
                            checked={isPublic}
                            onChange={(e) => documentStore.setState({ isPublic: e.target.checked })}
                            className="sr-only peer"
                        />
                        <div className="w-10 h-5 bg-zinc-800/80 rounded-full peer peer-checked:bg-accent/80 shadow-inner transition-all">
                            <div className="w-4 h-4 bg-white rounded-full shadow-sm transition-transform translate-x-0.5 peer-checked:translate-x-5 mt-0.5"></div>
                        </div>
                    </label>
                </div>
            </div>
        </div>
    );
}
