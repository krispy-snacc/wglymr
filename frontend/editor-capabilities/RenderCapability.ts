export interface RenderCapability {
    requestRender(): void;
    setVisible(visible: boolean): void;
    resize(width: number, height: number): void;
}
