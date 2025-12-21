"use client";

import { useEffect, useState } from "react";
import { documentStore } from "./documentStore";

export function useDocumentStore() {
    const [state, setState] = useState(() => documentStore.getState());

    useEffect(() => {
        const unsubscribe = documentStore.subscribe(() => {
            setState(documentStore.getState());
        });
        return unsubscribe;
    }, []);

    return state;
}

export function useDocumentState<T>(
    selector: (state: ReturnType<typeof documentStore.getState>) => T
): T {
    const [value, setValue] = useState(() =>
        selector(documentStore.getState())
    );

    useEffect(() => {
        const unsubscribe = documentStore.subscribe(() => {
            setValue(selector(documentStore.getState()));
        });
        return unsubscribe;
    }, [selector]);

    return value;
}
