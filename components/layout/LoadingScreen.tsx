"use client";

import { useEffect, useState } from "react";
import Image from "next/image";
import { THEME_COLOR } from "@/app/theme";

interface LoadingScreenProps {
    isLoading: boolean;
}

export function LoadingScreen({ isLoading }: LoadingScreenProps) {
    const [shouldRender, setShouldRender] = useState(true);

    useEffect(() => {
        if (!isLoading) {
            // Delay removal to allow fade-out animation
            const timeout = setTimeout(() => setShouldRender(false), 500);
            return () => clearTimeout(timeout);
        }
    }, [isLoading]);

    if (!shouldRender) return null;

    return (
        <div
            className={`fixed inset-0 z-50 flex items-center justify-center transition-opacity duration-500 ${isLoading ? "opacity-100" : "opacity-0"
                }`}
            style={{ background: THEME_COLOR }}
        >
            <div className="flex flex-col items-center gap-6">
                {/* Logo */}
                <div className="animate-pulse">
                    <Image
                        src="/wglymr_logo_full.svg"
                        alt="wglymr"
                        width={180}
                        height={46}
                        className="h-12 w-auto opacity-90"
                        priority
                    />
                </div>

                {/* Loading dots */}
                <div className="flex items-center gap-2">
                    <div className="w-2 h-2 rounded-full bg-accent animate-bounce" style={{ animationDelay: "0ms" }} />
                    <div className="w-2 h-2 rounded-full bg-accent animate-bounce" style={{ animationDelay: "150ms" }} />
                    <div className="w-2 h-2 rounded-full bg-accent animate-bounce" style={{ animationDelay: "300ms" }} />
                </div>

                {/* Optional loading text */}
                <p className="text-sm text-gray-400 animate-pulse">Loading editor...</p>
            </div>
        </div>
    );
}
