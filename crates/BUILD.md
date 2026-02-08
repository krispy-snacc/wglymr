# BUILD.md

**WGLYMR — Build System Specification**

> This document defines the **authoritative build system** for WGLYMR.
> All build tooling, automation, CI, and developer workflows **must conform** to this document.

---

## 1. Build Philosophy

WGLYMR uses a **two-tier build model**:

1. **Rust workspace** — engine, editors, runtime, rendering
2. **JS frontend** — web UI shell consuming WASM

Core principles:

- Cargo workspace is the **only Rust build system**
- JS tooling is **not embedded** into Cargo
- Build logic is **code**, not shell scripts
- Asset generation is **explicit and reproducible**
- Frontends are **consumers**, not authorities

---

## 2. Repository Layout (Authoritative)

```
wglymr/
├── frontend/                  # JS / React web frontend (NOT Cargo)
├── crates/                    # All Rust crates
│   ├── wglymr-core
│   ├── wglymr-document
│   ├── wglymr-interaction
│   ├── wglymr-view
│   ├── wglymr-render
│   ├── wglymr-render-wgpu
│   ├── wglymr-app
│   ├── wglymr-runtime
│   ├── wglymr-frontend-web    # WASM adapter (Rust)
│   ├── wglymr-font-assets
│   └── wglymr-color
├── xtask/                     # Rust build automation
├── docs/
│   └── adr/
├── Cargo.toml                 # Workspace root
├── ARCHITECTURE.md
└── BUILD.md
```

Rules:

- `frontend/` is **not** part of the Rust workspace
- All Rust code lives under `crates/`
- `wglymr-frontend-web` is a Rust WASM adapter, not the JS app

---

## 3. Cargo Workspace Configuration

### Root `Cargo.toml`

```toml
[workspace]
resolver = "2"

members = [
    "crates/wglymr-core",
    "crates/wglymr-document",
    "crates/wglymr-interaction",
    "crates/wglymr-view",
    "crates/wglymr-render",
    "crates/wglymr-render-wgpu",
    "crates/wglymr-app",
    "crates/wglymr-runtime",
    "crates/wglymr-frontend-web",
    "crates/wglymr-font-assets",
    "crates/wglymr-color",
    "xtask",
]

default-members = [
    "crates/wglymr-app"
]
```

Constraints:

- All crates must compile independently
- `cargo build` builds engine + app only
- Frontends do not affect workspace resolution

---

## 4. Dependency Enforcement (Architecture Guard)

Cargo dependencies **enforce architectural boundaries**.

### Allowed dependency directions

```
wglymr-frontend-web → wglymr-runtime
wglymr-runtime      → wglymr-app
wglymr-app          → interaction, view, render, document
wglymr-interaction  → document, view
wglymr-view         → document, render
wglymr-document     → core
wglymr-render-wgpu  → render, font-assets
wglymr-core         → (no internal deps)
```

Violations **must not compile**.

---

## 5. Feature Flag Policy

Cargo features are used **only for optional capabilities**, never for layering.

Example (`wglymr-render-wgpu`):

```toml
[features]
default = ["msdf-text"]
msdf-text = []
bitmap-text = []
debug-gpu = []
```

Rules:

- No feature-based architecture
- No cross-crate feature coupling
- Features only affect leaf crates

---

## 6. Font & Text Asset Pipeline (Authoritative)

WGLYMR supports **multiple text rendering modes**:

- **MSDF** — scalable UI text
- **Bitmap fonts** — pixel-aligned / small-scale text
- Runtime selection based on scale and usage

---

### 6.1 Font Assets Crate

All font assets are owned by:

```
crates/wglymr-font-assets/
├── assets/
│   ├── fonts/
│   │   ├── roboto.ttf
│   │   ├── inter.ttf
│   │   └── mono.ttf
│   └── generated/
│       ├── roboto.msdf.png
│       ├── roboto.metrics.json
│       ├── mono.bitmap.png
│       └── mono.bitmap.json
├── build.rs
└── src/lib.rs
```

Responsibilities:

- Own raw font sources
- Own generated atlases + metrics
- Embed generated data into Rust binaries
- Provide typed accessors only

No rendering or layout logic exists here.

---

### 6.2 Font Generation Policy

- Font generation is **explicit**
- Generated assets are **checked into the repo**
- Normal builds do **not regenerate fonts**

Generation uses external tools (e.g. `msdfgen`) and is invoked manually.

---

### 6.3 Font Build Entry Point

Font generation is triggered **only via xtask**:

```bash
cargo run -p xtask -- fonts
```

This command:

- Regenerates MSDF atlases
- Regenerates bitmap atlases
- Writes to `assets/generated/`

Normal `cargo build`:

- Uses existing generated assets
- Never regenerates fonts

This ensures fast, reproducible builds and stable CI.

---

### 6.4 Consumption Rules

- `wglymr-render-wgpu` depends on `wglymr-font-assets`
- Text renderer selects MSDF vs bitmap at runtime
- Core, document, view, and interaction layers never touch font assets

---

## 7. xtask — Build Automation

All automation is implemented via the `xtask` crate.

### Supported commands

```bash
cargo run -p xtask -- check
cargo run -p xtask -- test
cargo run -p xtask -- clippy
cargo run -p xtask -- fmt
cargo run -p xtask -- fonts
cargo run -p xtask -- web
```

Rules:

- No shell scripts
- No Makefiles
- xtask is the single automation entry point

---

## 8. Web Frontend Build Integration

### Rust side

`wglymr-frontend-web` builds the WASM package:

```
crates/wglymr-frontend-web/pkg/
```

This is invoked via:

```bash
cargo run -p xtask -- web
```

### JS side

The JS/React frontend:

```
frontend/
```

- Imports the generated WASM package
- Owns all UI, layout, and styling
- Is not part of the Rust build graph

Rust builds must succeed **without** `frontend/`.

---

## 9. CI Requirements

CI must:

- Build the Rust workspace
- Run tests
- Run clippy
- Use xtask exclusively

Minimal CI is sufficient at this stage.

---

## 10. Build Invariants (Hard Rules)

The following must always hold:

- Workspace builds on stable Rust
- All crates compile independently
- No implicit asset generation
- Frontends do not affect core builds
- Removing `frontend/` must not break Rust builds
- Generated assets are deterministic

Violations are build failures.

---

## 11. Non-Goals

The build system intentionally does **not** support:

- Runtime asset compilation
- Dynamic plugin loading
- Multiple build systems
- Custom dependency resolution

Any change requires an ADR.

---

## 12. Status

This document is **authoritative and active**.

Once committed:

- The build system is considered **locked**
- Future changes are evolutionary only
