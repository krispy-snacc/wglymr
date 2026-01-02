// Input routers: normalize platform events into commands.

import type { AnyCommand } from "./types";
import * as cmd from "./builders";

export type InputSource = "mouse" | "keyboard" | "touch" | "pointer";

export interface InputContext {
    viewId: string;
    source: InputSource;
    target: EventTarget | null;
    modifiers: {
        shift: boolean;
        ctrl: boolean;
        alt: boolean;
        meta: boolean;
    };
}

export type InputRouter = (
    event: Event,
    context: InputContext
) => AnyCommand | null;

// Mouse wheel => zoom command
export function routeWheelEvent(
    event: WheelEvent,
    context: InputContext
): AnyCommand | null {
    event.preventDefault();

    // Wheel deltas are inverted vs drag
    const PAN_SCALE = 20.0;

    if (event.shiftKey) {
        return cmd.panView(
            context.viewId,
            -event.deltaY * PAN_SCALE,
            -event.deltaX * PAN_SCALE
        );
    }

    if (event.ctrlKey) {
        return cmd.panView(
            context.viewId,
            -event.deltaX * PAN_SCALE,
            -event.deltaY * PAN_SCALE
        );
    }

    const isZoom = event.ctrlKey || event.metaKey;

    let canvas = event.target! as HTMLCanvasElement;
    const rect = canvas.getBoundingClientRect();

    const localX = event.clientX - rect.left;
    const localY = event.clientY - rect.top;

    // Normalize wheel delta (trackpad vs mouse)
    const ZOOM_SENSITIVITY = 0.002;
    const delta = Math.max(-100, Math.min(100, event.deltaY));

    const zoomFactor = Math.exp(-delta * ZOOM_SENSITIVITY);

    return cmd.zoomView(context.viewId, zoomFactor, localX, localY);
}

// Mouse drag with middle button => pan command
export function routePointerDrag(
    dx: number,
    dy: number,
    button: number,
    context: InputContext
): AnyCommand | null {
    if (button === 1) {
        return cmd.panView(context.viewId, dx, dy);
    }

    return null;
}

// Keyboard shortcuts => various commands
export function routeKeyboardEvent(
    event: KeyboardEvent,
    context: InputContext
): AnyCommand | null {
    const { shift, ctrl, alt, meta } = context.modifiers;

    if (event.key === "Escape") {
        return cmd.clearSelection(context.viewId);
    }

    if (event.key === "Delete" || event.key === "Backspace") {
        // Command requires selected node IDs; editor must provide them
        return null;
    }

    if (event.key === "a" && (ctrl || meta) && !shift) {
        // Select all; editor must provide all node IDs
        return null;
    }

    if (event.key === "0" && (ctrl || meta)) {
        return cmd.resetView(context.viewId);
    }

    return null;
}

// Context menu => add node command
export function routeAddNodeAction(
    nodeType: string,
    position: { x: number; y: number },
    context: InputContext
): AnyCommand {
    return cmd.addNode(context.viewId, nodeType, position);
}
