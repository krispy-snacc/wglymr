# wglymr-interaction

## Single Responsibility

**Interaction & Operators**

This crate owns:

- Input abstraction
- Operator definitions
- Modal interaction logic
- Operator stack & lifecycle
- Mapping hit results → semantic targets

## Must NOT Own

❌ Direct document mutation  
❌ Rendering  
❌ GPU logic  
❌ Node definitions

## Design Contract

Operators never mutate state directly.  
Operators emit document commands.  
Nodes never handle input.

> Blender equivalent: `wm_operators`
