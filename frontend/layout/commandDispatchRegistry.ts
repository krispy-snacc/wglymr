// Global command dispatcher registry for window-level input routing
import type { AnyCommand } from "@/commands/types";

type CommandDispatcher = (command: AnyCommand) => void;

const dispatcherRegistry = new Map<string, CommandDispatcher>();

export function registerViewDispatcher(
    viewId: string,
    dispatcher: CommandDispatcher
): void {
    dispatcherRegistry.set(viewId, dispatcher);
}

export function unregisterViewDispatcher(viewId: string): void {
    dispatcherRegistry.delete(viewId);
}

export function dispatchToView(viewId: string, command: AnyCommand): void {
    const dispatcher = dispatcherRegistry.get(viewId);
    if (dispatcher) {
        dispatcher(command);
    }
}
