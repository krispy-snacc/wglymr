"use client";

interface InspectorPanelProps {
    panelId: string;
    viewId?: string;
}

export function InspectorPanel({ panelId }: InspectorPanelProps) {
    const channels = [
        { id: 0, name: "channel0", texture: null },
        { id: 1, name: "channel1", texture: null },
    ];

    return (
        <div className="h-full overflow-y-auto p-4 flex flex-row gap-3  bg-zinc-950">
            {channels.map((channel) => (
                <div
                    key={channel.id}
                    className="flex flex-col gap-1.5 w-min"
                >
                    {/* Texture Preview */}
                    <button className="w-20 h-20 rounded bg-zinc-900 border border-white/5 hover:border-accent/50 flex items-center justify-center transition-all cursor-pointer group hover:bg-zinc-800">
                        <span className="text-[10px] text-gray-600 group-hover:text-gray-500">No texture</span>
                    </button>
                    <span className="text-[10px] font-medium text-gray-400 text-center truncate">{channel.name}</span>
                </div>
            ))}
        </div>
    );
}
