export function generatePanelId(): string {
    return `panel-${Date.now()}-${Math.random()
        .toString(36)
        .slice(2, 2 + 9)}`;
}

export function generateViewId(glymId: string, panelId: string): string {
    return `${glymId}-${panelId}`;
}
