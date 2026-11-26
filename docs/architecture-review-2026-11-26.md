# Architecture review — 2026-11-26

This durable summary records the architecture review produced during the 2026-11-26 improvement pass. The detailed visual review was generated as an HTML artifact; this Markdown version preserves its recommendations in the repository.

## Current strengths

- The document model and `FileService` establish useful seams for stateful operations and headless tests.
- Recent extractions moved arrange/edit/object commands, chrome, panel bodies, and panel geometry out of the largest modules.
- Tests exercise real egui frames and gestures, so architecture changes can preserve behavior instead of only matching screenshots.
- The protocol and command surfaces have CLI and JSON-RPC integration tests.

## Prioritized seams

### 1. Pointer gesture routing in `app.rs`

Input interpretation, gesture lifecycle, and document/layout mutation still converge in the application coordinator. Extract one coherent gesture family at a time into typed inputs and outcomes. Keep `FireCraft` as the state owner and preserve selection, undo, canvas bounds, and panel-layout invariants. Add cancel, boundary, and lost-focus cases before moving each family.

**Deletion test:** if removing the proposed module puts gesture interpretation and mutation back in `app.rs`, the seam owns behavior. If it only removes a forwarding wrapper, reject the extraction.

**Confidence:** high that this is the most valuable seam; medium on the exact first gesture family until the call graph and tests are reviewed together.

### 2. Format codecs in `formats.rs`

The file contains several import/export paths and shared format policy. Split one codec at a time only after its golden, round-trip, malformed-input, and error-path tests exist. Keep shared metadata and error types centralized; do not create a generic utilities module.

**Deletion test:** removing a codec module should make that format's parser/writer disappear from `formats.rs` without moving shared policy into a second copy.

**Confidence:** medium; preserve bytes and metadata unless an intentional correction is separately documented.

### 3. MCP/CLI protocol boundary in `mcp.rs`

The command catalogue, dispatch, JSON-RPC parsing, string helpers, and CLI orchestration are adjacent responsibilities. Separate transport parsing/serialization from canonical command execution only where it reduces coupling. Keep CLI and MCP callers routed through the same command catalog.

**Deletion test:** removing the protocol module should put parsing and response rules back into command/CLI code. If it only removes a thin wrapper, the module had no useful depth.

**Confidence:** moderate; protocol edge-case tests already provide a base for a small, incremental split.

## Implementation rules

1. Start with one behavior family and record current state transitions.
2. Add tests at the public seam for success, cancel, invalid input, and undo/recovery behavior.
3. Move code without changing semantics; compare exported bytes and document state where relevant.
4. Run the full suite and strict lints after each extraction.
5. Re-check module size, dependency direction, and architecture documentation.
6. Do not split solely to reduce line counts; every new module needs a cohesive owner and a deletion test.

## References

- [Architecture guide](architecture.md)
- [Quality gap analysis](research/quality-gap-analysis.md)
- [Active task list](TASKS.md)
- [Architecture improvement skill](https://github.com/mattpocock/skills/tree/main/skills/engineering/improve-codebase-architecture)

