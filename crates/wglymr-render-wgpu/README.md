# wglymr-render-wgpu

## Single Responsibility

**Render Backend (wgpu)**

This crate owns:

- GPU pipelines
- Buffers
- Shaders
- Backend-specific optimizations
- SDF rendering
- Text rendering
- Primitive rendering

## Must NOT Own

❌ Graph knowledge  
❌ Editor logic  
❌ Document mutation  
❌ Interaction logic

## Design Contract

Consumes draw items only.  
Replaceable without affecting editor logic.

> Blender equivalent: GPU backend implementations
