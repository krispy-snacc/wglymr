"use client";

import { createContext, useContext, type ReactNode } from "react";
import type { EditorType } from "@/layout/editorRegistry";
import {
    ensureEditorRuntimeReady,
    createEditorView,
    attachEditorView,
    setEditorViewVisible,
    resizeEditorView,
    requestEditorRender,
    detachEditorView,
    destroyEditorView,
} from "@/runtime";

interface EditorContextValue {
    editorType: EditorType;
    viewId?: string;
    runtime: {
        ensureReady: typeof ensureEditorRuntimeReady;
        createView: typeof createEditorView;
        attachView: typeof attachEditorView;
        setVisible: typeof setEditorViewVisible;
        resizeView: typeof resizeEditorView;
        requestRender: typeof requestEditorRender;
        detachView: typeof detachEditorView;
        destroyView: typeof destroyEditorView;
    };
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
        runtime: {
            ensureReady: ensureEditorRuntimeReady,
            createView: createEditorView,
            attachView: attachEditorView,
            setVisible: setEditorViewVisible,
            resizeView: resizeEditorView,
            requestRender: requestEditorRender,
            detachView: detachEditorView,
            destroyView: destroyEditorView,
        },
    };

    return (
        <EditorContext.Provider value={value}>
            {children}
        </EditorContext.Provider>
    );
}
