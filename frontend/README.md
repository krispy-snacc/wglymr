# Frontend Build & Integration

## Overview

The `frontend/` directory contains the React/Next.js application that provides the UI shell for WGLYMR.

**Key Principle:** The frontend **imports** the WASM-compiled Rust engine, but is **not part of the Cargo workspace**.

## Structure

```
frontend/
├── wasm-pkg/              # Generated WASM bindings (gitignored)
├── app/                   # Next.js pages and routing
├── runtime/               # WASM initialization and lifecycle
├── editor-capabilities/   # TypeScript interfaces for editor features
├── layout/                # Golden Layout integration
├── panels/                # UI panels (inspector, preview, etc.)
├── ui/                    # Shared UI components
├── commands/              # Command routing to WASM
├── context/               # React contexts
└── public/                # Static assets
```

## Build Process

### 1. Build WASM Adapter

```bash
cargo run -p xtask -- web
```

Output: `frontend/wasm-pkg/`

This compiles the Rust crate `wglymr-frontend-web` to WebAssembly.

### 2. Install Dependencies

```bash
cd frontend
npm install
```

### 3. Run Development Server

```bash
npm run dev
```

### 4. Build for Production

```bash
npm run build
npm start
```

## WASM Integration

The frontend imports WASM via the `runtime/editorRuntime.ts` module:

```typescript
import init from "../wasm-pkg/wglymr_frontend_web.js";

await init(); // Initialize WASM module
await init_gpu(); // Initialize WebGPU
init_engine(); // Initialize engine runtime
start_render_loop(); // Start rendering
```

### Important Rules

- **Never** import WASM directly in components
- **Always** use `ensureEditorRuntimeReady()` from `runtime/editorRuntime.ts`
- WASM is initialized **once** globally
- All WASM calls go through capability interfaces

## Next.js Configuration

The `next.config.ts` enables WebAssembly support:

```typescript
webpack: (config, { isServer }) => {
    if (!isServer) {
        config.experiments = {
            asyncWebAssembly: true,
        };
    }
    return config;
};
```

## Alignment with BUILD.md

Per BUILD.md section 4 (Frontend/Engine Separation):

✅ Frontend is **outside** Cargo workspace  
✅ WASM is the **only** integration point  
✅ Rust never imports JS  
✅ JS never reimplements engine logic  
✅ Rust build succeeds without frontend present

## Development Workflow

1. Make Rust changes in `crates/`
2. Rebuild WASM: `cargo run -p xtask -- web`
3. Frontend hot-reloads automatically (if dev server running)
4. TypeScript types are regenerated from `wglymr_frontend_web.d.ts`

## Common Issues

**WASM module not found:**

- Run `cargo run -p xtask -- web` to generate WASM files

**WebGPU not available:**

- Use Chrome/Edge with WebGPU enabled
- Check browser compatibility

**Hot reload doesn't pick up Rust changes:**

- WASM must be rebuilt manually
- Frontend only hot-reloads TS/React changes
