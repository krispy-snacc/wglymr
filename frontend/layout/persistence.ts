import {
    LayoutConfig as LayoutConfigApi,
    type LayoutConfig as LayoutConfigType,
    type ResolvedLayoutConfig,
} from "golden-layout";

const STORAGE_KEY = "wglymr.goldenLayout.layout";
const EMPTY_KEY = "wglymr.goldenLayout.empty";

function safeJsonParse(value: string | null): unknown {
    if (!value) return undefined;
    try {
        return JSON.parse(value);
    } catch {
        return undefined;
    }
}

function coerceToLayoutConfig(config: unknown): LayoutConfigType | undefined {
    if (!config || typeof config !== "object") return undefined;
    if (LayoutConfigApi.isResolved(config as never)) {
        return LayoutConfigApi.fromResolved(config as never);
    }
    return config as LayoutConfigType;
}

export function loadLayout(): LayoutConfigType | "EMPTY" | undefined {
    if (typeof window === "undefined") return undefined;

    if (localStorage.getItem(EMPTY_KEY) === "1") {
        return "EMPTY";
    }

    const raw = localStorage.getItem(STORAGE_KEY);
    if (!raw) return undefined;

    try {
        return JSON.parse(raw);
    } catch {
        return undefined;
    }
}

export function saveLayout(
    resolved: ResolvedLayoutConfig,
    isEmpty: boolean
): void {
    if (isEmpty) {
        localStorage.setItem(EMPTY_KEY, "1");
        localStorage.removeItem(STORAGE_KEY);
        return;
    }

    localStorage.removeItem(EMPTY_KEY);
    const config = LayoutConfigApi.fromResolved(resolved);
    localStorage.setItem(STORAGE_KEY, JSON.stringify(config));
}
