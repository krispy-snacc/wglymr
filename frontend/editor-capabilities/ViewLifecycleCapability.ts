// ViewLifecycleCapability: manages view creation, attachment, and destruction.
// Runtime-backed, per-view operations only.

export interface ViewLifecycleCapability {
    createView(): void;
    attachView(canvas: HTMLCanvasElement, width: number, height: number): void;
    detachView(): void;
    destroyView(): void;
}
