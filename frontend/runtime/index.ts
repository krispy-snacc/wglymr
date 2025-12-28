// Public runtime API - UI code must not import WASM bindings directly.

export { ensureEditorRuntimeReady } from "./editorRuntime";

export {
    createEditorView,
    attachEditorView,
    setEditorViewVisible,
    resizeEditorView,
    detachEditorView,
    destroyEditorView,
} from "./viewManager";

export { requestEditorRender } from "./renderScheduler";
