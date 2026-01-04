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

// Detect if event is from trackpad (high-resolution continuous deltas)
function isTrackpad(event: WheelEvent): boolean {
    return event.deltaMode === 0 && Math.abs(event.deltaY) < 100;
}

// Figma-style wheel routing: trackpad-first navigation with cursor-centered zoom
export function routeWheelEvent(
    event: WheelEvent,
    context: InputContext
): AnyCommand | null {
    event.preventDefault();

    const canvas = event.target as HTMLCanvasElement;
    const rect = canvas.getBoundingClientRect();
    const localX = event.clientX - rect.left;
    const localY = event.clientY - rect.top;

    const PAN_SCALE = 1.0;
    const ZOOM_SENSITIVITY = 0.01;

    if (isTrackpad(event)) {
        if (event.ctrlKey || event.metaKey) {
            // TRACKPAD PINCH => ZOOM (cursor-centered)
            const delta = Math.max(-100, Math.min(100, event.deltaY));
            const zoomFactor = Math.exp(-delta * ZOOM_SENSITIVITY);
            return cmd.zoomView(context.viewId, zoomFactor, localX, localY);
        } else {
            // TRACKPAD TWO-FINGER MOVE => PAN (free 2D, natural direction)
            return cmd.panView(
                context.viewId,
                -event.deltaX * PAN_SCALE,
                -event.deltaY * PAN_SCALE
            );
        }
    } else {
        // MOUSE WHEEL
        if (event.shiftKey) {
            // SHIFT + WHEEL => SWAP AXES
            return cmd.panView(
                context.viewId,
                -event.deltaY * PAN_SCALE,
                -event.deltaX * PAN_SCALE
            );
        } else if (event.ctrlKey || event.metaKey) {
            // CTRL + WHEEL => PAN
            return cmd.panView(
                context.viewId,
                -event.deltaX * PAN_SCALE,
                -event.deltaY * PAN_SCALE
            );
        } else {
            // WHEEL (no modifiers) => ZOOM (cursor-centered)
            const delta = Math.max(-100, Math.min(100, event.deltaY));
            const zoomFactor = Math.exp(-delta * ZOOM_SENSITIVITY);
            return cmd.zoomView(context.viewId, zoomFactor, localX, localY);
        }
    }
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
