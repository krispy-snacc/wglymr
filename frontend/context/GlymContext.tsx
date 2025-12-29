"use client";

import { createContext, useContext, type ReactNode } from "react";
import { documentStore } from "@/document/documentStore";

interface GlymContextValue {
    glymId: string;
    documentStore: typeof documentStore;
}

const GlymContext = createContext<GlymContextValue | null>(null);

export function useGlymContext(): GlymContextValue {
    const context = useContext(GlymContext);
    if (!context) {
        throw new Error("useGlymContext must be used within GlymProvider");
    }
    return context;
}

interface GlymProviderProps {
    glymId: string;
    children: ReactNode;
}

export function GlymProvider({ glymId, children }: GlymProviderProps) {
    const value: GlymContextValue = {
        glymId,
        documentStore,
    };

    return (
        <GlymContext.Provider value={value}>
            {children}
        </GlymContext.Provider>
    );
}
