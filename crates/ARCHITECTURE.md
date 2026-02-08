> **This document is the single source of truth for WGLYMR’s architecture.
> All systems, refactors, and features must conform to it.**

---

## 1. Project Overview

**WGLYMR** is a **Blender-inspired, Rust-based tool platform** for visual computation.

It is **not** a single node editor.
It is a **tool framework** capable of hosting:

- shader node editors
- compute graphs
- geometry nodes
- future editors (curve editor, viewport, timeline)
- multiple frontends (web, desktop, TUI)

All core logic lives in Rust.
Frontends are thin adapters.

---

## 2. Core Design Principles (Non-Negotiable)

1. **Strict separation of responsibilities**
2. **Operator-driven interaction model (Blender-style)**
3. **Backend-agnostic graph system**
4. **Frontend-agnostic runtime**
5. **Retained-mode rendering**
6. **Command-based mutation with undo/redo**
7. **Immutable snapshots for reading state**

Violating these principles is considered an architectural regression.

---

## 3. Layered Architecture (Authority Flow)

```
Frontend (Web / Desktop / TUI)
↓
Runtime (lifecycle, scheduling)
↓
Application Shell (editors, panels)
↓
Interaction System (operators)
↓
View System (layout, hit-testing, overlays)
↓
Render ABI (draw commands)
↓
Render Backend (wgpu, etc.)
↓
Document System (editable state, undo)
↓
Core Engine (graph IR, evaluation)
```

**Authority flows downward.**
**Data flows upward via snapshots and commands.**

---

## 4. Crate Responsibilities

### 4.1 `wglymr-core` — Core Engine (Headless)

**Owns:**

- Graph IR (nodes, sockets, edges)
- Type system
- Validation passes
- Diagnostics
- Execution interfaces
- Backend compilation (shader, compute, CPU)

**Does NOT own:**

- UI
- Rendering
- Input
- Undo
- Frontends

This crate must be usable **without any editor or UI**.

> Blender equivalent: `blenkernel/`

---

### 4.2 `wglymr-document` — Editable State & Undo

**Owns:**

- Editable document model
- Command definitions
- Undo/redo history
- Versioning
- Persistence
- Immutable `GraphSnapshot`

**Rules:**

- All mutations go through commands
- Produces snapshots for reading
- Never performs execution or rendering

> Blender equivalent: `DNA + undo system`

---

### 4.3 `wglymr-interaction` — Interaction & Operators

**Owns:**

- Input abstraction
- Operator definitions
- Modal interaction logic
- Operator stack & lifecycle
- Mapping hit results → semantic targets

**Rules:**

- Operators never mutate state directly
- Operators emit document commands
- Nodes never handle input

> Blender equivalent: `wm_operators`

---

### 4.4 `wglymr-view` — View Models (No GPU)

**Owns:**

- Render models (RenderNode, RenderEdge)
- Layout (nodes, edges, text)
- Hit testing
- Culling
- Overlays (curves, pickers, gizmos)

**Rules:**

- Reads snapshots only
- No GPU or backend logic
- No document mutation

> Blender equivalent: `ED_*` editors

---

### 4.5 `wglymr-render` — Render ABI

**Owns:**

- DrawItem definitions
- Draw layers
- Depth / Z semantics
- Themes and colors

**Rules:**

- Knows nothing about graphs or editors
- Defines the rendering contract only

> Blender equivalent: `GPU_*` abstraction layer

---

### 4.6 `wglymr-render-*` — Render Backends

Example: `wglymr-render-wgpu`

**Owns:**

- GPU pipelines
- Buffers
- Shaders
- Backend-specific optimizations

**Rules:**

- Consumes draw items only
- Replaceable without affecting editor logic

> Blender equivalent: GPU backend implementations

---

### 4.7 `wglymr-app` — Application Shell

**Owns:**

- Editors (node editor, future editors)
- Panels (properties, outliner)
- Workspace layout
- Document management

**Rules:**

- Assembles systems, does not implement them
- No GPU logic
- No core engine logic

> Blender equivalent: editor registration & workspaces

---

### 4.8 `wglymr-runtime` — Lifecycle & Scheduling

**Owns:**

- Main loop
- Dirty view scheduling
- View registry
- Lifecycle management

**Rules:**

- Orchestrates systems
- Owns no domain logic

> Blender equivalent: window manager & event loop

---

### 4.9 Frontends (`wglymr-frontend-*`)

**Owns:**

- Platform integration
- Event forwarding
- Surface creation
- Clipboard / OS hooks

**Rules:**

- No business logic
- No graph knowledge
- No rendering logic

> Blender equivalent: GHOST

---

## 5. Interaction Model (Blender-Style)

- All interaction is handled by **operators**
- Operators may be **modal**
- Visual feedback is implemented via **view overlays**
- Data changes are committed via **commands**

This enables:

- Curves editors
- Color picker popups
- Custom interaction nodes
- Consistent UX across editors

---

## 6. Extension Strategy

Adding new functionality should follow these patterns:

- **New graph type** → add backend in `wglymr-core`
- **New editor** → add to `wglymr-app`
- **New interaction** → add operator in `wglymr-interaction`
- **New visualization** → add view overlay in `wglymr-view`
- **New platform** → add frontend crate

No existing layers should be violated.

---

## 7. Non-Goals

WGLYMR does **not** aim to:

- Be a game engine
- Embed logic in frontends
- Tie editor logic to a single backend
- Sacrifice architecture for speed

---

## 8. Architectural Changes

Any change that affects:

- layer boundaries
- crate responsibilities
- authority rules

**MUST** be recorded as an ADR.

---

# ADR System (Architecture Decision Records)

Lives in

```
/docs/adr/
```

---

## ADR File Naming

```
0001-short-title.md
0002-another-decision.md
```

Sequential, never renumber.

---

## ADR Template (Copy This)

```md
# ADR 000X: <Decision Title>

## Status

Accepted | Proposed | Superseded

## Context

What problem are we solving?
Why is this decision needed now?

## Decision

What was decided?
Be precise.

## Alternatives Considered

- Option A
- Option B
- Option C

## Consequences

Positive:

- …

Negative:

- …

## Notes

Links to relevant code or discussions.
```

---
