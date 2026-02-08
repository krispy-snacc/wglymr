# Architecture Decision Records (ADR)

This directory contains Architecture Decision Records for WGLYMR.

## Purpose

Per BUILD.md section 12, any change affecting:

- Workspace structure
- xtask commands
- Asset generation rules
- Feature policy

**MUST** be documented via an ADR.

## Format

ADRs follow a standard format:

- **Title**: Brief descriptive title
- **Status**: Proposed, Accepted, Deprecated, Superseded
- **Context**: Technical and business context
- **Decision**: What was decided
- **Consequences**: Positive and negative outcomes

## Numbering

ADRs are numbered sequentially:

- `0001-initial-build-system.md`
- `0002-font-asset-pipeline.md`
- etc.

## Creating an ADR

When making architectural changes:

1. Copy the template (if available)
2. Number it sequentially
3. Fill out all sections
4. Commit it with the implementation
5. Update this README if needed

## Current ADRs

None yet - BUILD.md serves as the initial specification.
