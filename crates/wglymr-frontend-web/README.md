# wglymr-frontend-web

## Single Responsibility

**WASM Adapter / Web Platform Integration**

This crate is a **Rust crate** that exposes the WGLYMR engine to JavaScript via WebAssembly.

### What This Crate Is

A WASM adapter that:

- Exposes Rust → JS API surface via wasm-bindgen
- Handles web platform integration (canvas, events, clipboard)
- Creates WebGPU surfaces
- Forwards browser events to the engine
- Produces `pkg/` output (WASM + JS bindings)

**This is NOT the React app.**

The React/Next.js application lives in `frontend/` and imports this crate's output.

### What This Crate Owns

✅ WASM bindings (wasm-bindgen glue)  
✅ Web platform integration (DOM, canvas, clipboard)  
✅ Event forwarding (browser → engine)  
✅ WebGPU surface creation

### Must NOT Own

❌ React components or UI layout  
❌ Business logic  
❌ Graph knowledge  
❌ Rendering logic (lives in wglymr-render-wgpu)  
❌ Editor implementations

### Design Contract

No UI code.  
No business logic.  
No graph knowledge.  
No rendering logic.

> Architecture: Thin adapter between browser APIs and Rust engine.
> Equivalent: Platform abstraction layer (like Blender's GHOST)

### Build

```bash
cargo run -p xtask -- web
```

Output: `crates/wglymr-frontend-web/pkg/`

This is consumed by the React app in `frontend/`.
