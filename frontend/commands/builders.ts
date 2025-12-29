// Command builders: pure functions that construct command objects.
// Runtime assigns timestamps at dispatch time for deterministic replay.

import type {
    PanViewCommand,
    ZoomViewCommand,
    ResetViewCommand,
    AddNodeCommand,
    DeleteNodeCommand,
    MoveNodeCommand,
    ConnectNodesCommand,
    DisconnectNodesCommand,
    SetUniformCommand,
    SetSelectionCommand,
    AddToSelectionCommand,
    ClearSelectionCommand,
} from "./types";

export function panView(
    viewId: string,
    dx: number,
    dy: number
): PanViewCommand {
    return {
        type: "view.pan",
        viewId,
        dx,
        dy,
    };
}

export function zoomView(
    viewId: string,
    delta: number,
    centerX?: number,
    centerY?: number
): ZoomViewCommand {
    return {
        type: "view.zoom",
        viewId,
        delta,
        centerX,
        centerY,
    };
}

export function resetView(viewId: string): ResetViewCommand {
    return {
        type: "view.reset",
        viewId,
    };
}

export function addNode(
    viewId: string,
    nodeType: string,
    position: { x: number; y: number }
): AddNodeCommand {
    return {
        type: "node.add",
        viewId,
        nodeType,
        position,
    };
}

export function deleteNode(viewId: string, nodeId: string): DeleteNodeCommand {
    return {
        type: "node.delete",
        viewId,
        nodeId,
    };
}

export function moveNode(
    viewId: string,
    nodeId: string,
    position: { x: number; y: number }
): MoveNodeCommand {
    return {
        type: "node.move",
        viewId,
        nodeId,
        position,
    };
}

export function connectNodes(
    viewId: string,
    sourceNodeId: string,
    sourceSocket: string,
    targetNodeId: string,
    targetSocket: string
): ConnectNodesCommand {
    return {
        type: "node.connect",
        viewId,
        sourceNodeId,
        sourceSocket,
        targetNodeId,
        targetSocket,
    };
}

export function disconnectNodes(
    viewId: string,
    sourceNodeId: string,
    sourceSocket: string,
    targetNodeId: string,
    targetSocket: string
): DisconnectNodesCommand {
    return {
        type: "node.disconnect",
        viewId,
        sourceNodeId,
        sourceSocket,
        targetNodeId,
        targetSocket,
    };
}

export function setUniform(
    viewId: string,
    uniformName: string,
    value: number | number[] | string | boolean
): SetUniformCommand {
    return {
        type: "uniform.set",
        viewId,
        uniformName,
        value,
    };
}

export function setSelection(
    viewId: string,
    nodeIds: string[]
): SetSelectionCommand {
    return {
        type: "selection.set",
        viewId,
        nodeIds,
    };
}

export function addToSelection(
    viewId: string,
    nodeIds: string[]
): AddToSelectionCommand {
    return {
        type: "selection.add",
        viewId,
        nodeIds,
    };
}

export function clearSelection(viewId: string): ClearSelectionCommand {
    return {
        type: "selection.clear",
        viewId,
    };
}
