# wglymr-runtime

## Single Responsibility

**Lifecycle & Scheduling**

This crate owns:

- Main loop
- Dirty view scheduling
- View registry
- Lifecycle management

## Must NOT Own

❌ Domain logic  
❌ Editor implementations  
❌ Graph logic  
❌ Rendering logic

## Design Contract

Orchestrates systems.  
Owns no domain logic.

> Blender equivalent: window manager & event loop
