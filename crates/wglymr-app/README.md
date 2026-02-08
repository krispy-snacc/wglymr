# wglymr-app

## Single Responsibility

**Application Shell**

This crate owns:

- Editors (node editor, future editors)
- Panels (properties, outliner)
- Workspace layout
- Document management

## Must NOT Own

❌ GPU logic  
❌ Core engine logic  
❌ System implementation details

## Design Contract

Assembles systems, does not implement them.  
No GPU logic.  
No core engine logic.

> Blender equivalent: editor registration & workspaces
