# wglymr-render

## Single Responsibility

**Render ABI**

This crate owns:

- DrawItem definitions
- Draw layers
- Depth / Z semantics
- Themes and colors

## Must NOT Own

❌ Graph knowledge  
❌ Editor logic  
❌ GPU implementation  
❌ Backend-specific code

## Design Contract

Knows nothing about graphs or editors.  
Defines the rendering contract only.

> Blender equivalent: `GPU_*` abstraction layer
