# wglymr-view

## Single Responsibility

**View Models (No GPU)**

This crate owns:

- Render models (RenderNode, RenderEdge)
- Layout (nodes, edges, text)
- Hit testing
- Culling
- Overlays (curves, pickers, gizmos)

## Must NOT Own

❌ GPU logic  
❌ Backend-specific rendering  
❌ Document mutation  
❌ Input handling

## Design Contract

Reads snapshots only.  
Produces render models only.  
No GPU, no input handling.

> Blender equivalent: `ED_*` editors
