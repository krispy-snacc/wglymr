// Global active view tracker for keyboard and window-level input targeting

let activeViewId: string | null = null;

export function setActiveView(viewId: string): void {
    activeViewId = viewId;
}

export function getActiveView(): string | null {
    return activeViewId;
}

export function clearActiveView(): void {
    activeViewId = null;
}
