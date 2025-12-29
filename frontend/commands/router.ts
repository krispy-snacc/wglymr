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

// Mouse wheel → zoom command
export function routeWheelEvent(
    event: WheelEvent,
    context: InputContext
): AnyCommand | null {
    if (event.ctrlKey || event.metaKey) {
        return cmd.zoomView(
            context.viewId,
            -event.deltaY * 0.01,
            event.clientX,
            event.clientY
        );
    }

    return cmd.panView(context.viewId, -event.deltaX, -event.deltaY);
}

// Mouse drag with middle button → pan command
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

// Keyboard shortcuts → various commands
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

// Context menu → add node command
export function routeAddNodeAction(
    nodeType: string,
    position: { x: number; y: number },
    context: InputContext
): AnyCommand {
    return cmd.addNode(context.viewId, nodeType, position);
}
