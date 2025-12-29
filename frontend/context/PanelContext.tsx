"use client";

import { createContext, useContext, type ReactNode } from "react";

interface PanelContextValue {
    panelId: string;
}

const PanelContext = createContext<PanelContextValue | null>(null);

export function usePanelContext(): PanelContextValue {
    const context = useContext(PanelContext);
    if (!context) {
        throw new Error("usePanelContext must be used within PanelProvider");
    }
    return context;
}

interface PanelProviderProps {
    panelId: string;
    children: ReactNode;
}

export function PanelProvider({ panelId, children }: PanelProviderProps) {
    const value: PanelContextValue = {
        panelId,
    };

    return (
        <PanelContext.Provider value={value}>
            {children}
        </PanelContext.Provider>
    );
}
