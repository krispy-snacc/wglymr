"use client";

import { createContext, useContext, type ReactNode } from "react";
import type { EditorCapabilities } from "@/editor-capabilities";

const EditorCapabilitiesContext = createContext<EditorCapabilities | null>(null);

export function useEditorCapabilities(): EditorCapabilities {
    const context = useContext(EditorCapabilitiesContext);
    if (!context) {
        throw new Error("useEditorCapabilities must be used within EditorCapabilitiesProvider");
    }
    return context;
}

interface EditorCapabilitiesProviderProps {
    capabilities: EditorCapabilities;
    children: ReactNode;
}

export function EditorCapabilitiesProvider({ capabilities, children }: EditorCapabilitiesProviderProps) {
    return (
        <EditorCapabilitiesContext.Provider value={capabilities}>
            {children}
        </EditorCapabilitiesContext.Provider>
    );
}
