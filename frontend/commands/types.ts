// Commands: pure data representing user intent.
// Frontend constructs, runtime executes.
// Enables undo/redo, recording, scripting.

export type CommandType =
    | "view.pan"
    | "view.zoom"
    | "view.reset"
    | "node.add"
    | "node.delete"
    | "node.move"
    | "node.connect"
    | "node.disconnect"
    | "uniform.set"
    | "selection.set"
    | "selection.add"
    | "selection.clear";

// Base command structure: all commands extend this
export interface Command<T extends CommandType = CommandType> {
    readonly type: T;
    readonly viewId: string;
}

// View manipulation commands
export interface PanViewCommand extends Command<"view.pan"> {
    readonly dx: number;
    readonly dy: number;
}

export interface ZoomViewCommand extends Command<"view.zoom"> {
    readonly delta: number;
    readonly centerX?: number;
    readonly centerY?: number;
}

export interface ResetViewCommand extends Command<"view.reset"> {}

// Node manipulation commands
export interface AddNodeCommand extends Command<"node.add"> {
    readonly nodeType: string;
    readonly position: { x: number; y: number };
}

export interface DeleteNodeCommand extends Command<"node.delete"> {
    readonly nodeId: string;
}

export interface MoveNodeCommand extends Command<"node.move"> {
    readonly nodeId: string;
    readonly position: { x: number; y: number };
}

export interface ConnectNodesCommand extends Command<"node.connect"> {
    readonly sourceNodeId: string;
    readonly sourceSocket: string;
    readonly targetNodeId: string;
    readonly targetSocket: string;
}

export interface DisconnectNodesCommand extends Command<"node.disconnect"> {
    readonly sourceNodeId: string;
    readonly sourceSocket: string;
    readonly targetNodeId: string;
    readonly targetSocket: string;
}

// Uniform manipulation commands
export interface SetUniformCommand extends Command<"uniform.set"> {
    readonly uniformName: string;
    readonly value: number | number[] | string | boolean;
}

// Selection commands
export interface SetSelectionCommand extends Command<"selection.set"> {
    readonly nodeIds: string[];
}

export interface AddToSelectionCommand extends Command<"selection.add"> {
    readonly nodeIds: string[];
}

export interface ClearSelectionCommand extends Command<"selection.clear"> {}

// Union type of all concrete commands
export type AnyCommand =
    | PanViewCommand
    | ZoomViewCommand
    | ResetViewCommand
    | AddNodeCommand
    | DeleteNodeCommand
    | MoveNodeCommand
    | ConnectNodesCommand
    | DisconnectNodesCommand
    | SetUniformCommand
    | SetSelectionCommand
    | AddToSelectionCommand
    | ClearSelectionCommand;

// Command result from runtime (success or error)
export type CommandResult =
    | { success: true }
    | { success: false; error: string };
