# wglymr-core

## Single Responsibility

**Core Engine (Headless)**

This crate owns:

- Graph IR (nodes, sockets, edges)
- Type system
- Validation passes
- Diagnostics
- Execution interfaces
- Backend compilation (shader, compute, CPU)

## Must NOT Own

❌ UI  
❌ Rendering  
❌ Input  
❌ Undo  
❌ Frontends  
❌ Document mutation

## Design Contract

This crate must be usable **without any editor or UI**.

> Blender equivalent: `blenkernel/`
