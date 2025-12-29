import type { RenderCapability } from "./RenderCapability";
import type { ViewCapability } from "./ViewCapability";
import type { DocumentCapability } from "./DocumentCapability";
import type { CommandCapability } from "./CommandCapability";

export type { RenderCapability } from "./RenderCapability";
export type { ViewCapability } from "./ViewCapability";
export type { DocumentCapability } from "./DocumentCapability";
export type { CommandCapability } from "./CommandCapability";

export interface EditorCapabilities {
    render?: RenderCapability;
    view?: ViewCapability;
    document?: DocumentCapability;
    command?: CommandCapability;
}
