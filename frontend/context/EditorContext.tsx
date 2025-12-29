"use client";

import { createContext, useContext, type ReactNode } from "react";
import type { EditorType } from "@/layout/editorRegistry";

interface EditorContextValue {
    editorType: EditorType;
    viewId?: string;
}

const EditorContext = createContext<EditorContextValue | null>(null);

export function useEditorContext(): EditorContextValue {
    const context = useContext(EditorContext);
    if (!context) {
        throw new Error("useEditorContext must be used within EditorProvider");
    }
    return context;
}

interface EditorProviderProps {
    editorType: EditorType;
    viewId?: string;
    children: ReactNode;
}

export function EditorProvider({ editorType, viewId, children }: EditorProviderProps) {
    const value: EditorContextValue = {
        editorType,
        viewId,
    };

    return (
        <EditorContext.Provider value={value}>
            {children}
        </EditorContext.Provider>
    );
}
