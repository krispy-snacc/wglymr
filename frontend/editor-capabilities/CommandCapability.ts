// CommandCapability: forwards commands to runtime.
// All validation, execution, undo/redo happens in Rust.

import type { AnyCommand, CommandResult } from "@/commands/types";

export interface CommandCapability {
    dispatch(command: AnyCommand): Promise<CommandResult>;
}
