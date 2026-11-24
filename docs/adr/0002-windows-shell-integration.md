# ADR 0002 — Best-effort Windows shell integration

- **Status:** Accepted
- **Date:** 2026-11-24

## Context

Windows users expect the taskbar Jump List to expose recent documents and common tasks. COM shell
APIs are unavailable or unreliable in Wine, locked-down sessions, and non-Explorer shells. Startup
must never fail because of shell integration.

## Decision

- Set an explicit AppUserModelID before the first window.
- Build a custom Jump List with recent files and tasks (`New Document`, `About FireCraft`).
- Treat every COM failure as non-fatal: log to stderr and continue.
- Rebuild the list when a document is added to the recent list.
- Compile the module only on Windows.

## Consequences

- Linux/macOS/WASM builds are unaffected.
- Jump List behavior cannot be fully verified without a real Windows Explorer shell; report that
  limitation honestly.
- Recent documents are best-effort and capped to avoid shell limits.
