# wglymr-document

## Single Responsibility

**Editable State & Undo**

This crate owns:

- Editable document model
- Command definitions
- Undo/redo history
- Versioning
- Persistence
- Immutable `GraphSnapshot` production

## Must NOT Own

❌ Execution  
❌ Rendering  
❌ Input handling  
❌ Backend compilation

## Design Contract

All mutations go through commands.  
Produces snapshots for reading.  
Never performs execution or rendering.

> Blender equivalent: `DNA + undo system`
