// InputCapability: forwards raw input events to runtime for hit testing and interaction.
// Unlike commands (which are semantic actions), input events are low-level and stateful.
// The runtime maintains interaction state (hover, drag, selection) internally.

export interface InputCapability {
    handleMouseMove(
        screenX: number,
        screenY: number,
        shift: boolean,
        ctrl: boolean,
        alt: boolean
    ): void;
    handleMouseDown(
        screenX: number,
        screenY: number,
        button: number,
        shift: boolean,
        ctrl: boolean,
        alt: boolean
    ): void;
    handleMouseUp(
        screenX: number,
        screenY: number,
        button: number,
        shift: boolean,
        ctrl: boolean,
        alt: boolean
    ): void;
}
