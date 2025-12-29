"use client";

import { createContext, useContext, useEffect, useState, type ReactNode } from "react";
import { ensureEditorRuntimeReady } from "@/runtime";

interface RuntimeContextValue {
    ready: boolean;
}

const RuntimeContext = createContext<RuntimeContextValue | null>(null);

export function useRuntimeContext(): RuntimeContextValue {
    const context = useContext(RuntimeContext);
    if (!context) {
        throw new Error("useRuntimeContext must be used within RuntimeProvider");
    }
    return context;
}

interface RuntimeProviderProps {
    children: ReactNode;
}

export function RuntimeProvider({ children }: RuntimeProviderProps) {
    const [ready, setReady] = useState(false);

    useEffect(() => {
        ensureEditorRuntimeReady().then(() => {
            setReady(true);
        });
    }, []);

    if (!ready) {
        return null;
    }

    const value: RuntimeContextValue = {
        ready,
    };

    return (
        <RuntimeContext.Provider value={value}>
            {children}
        </RuntimeContext.Provider>
    );
}
