# Architecture review — 2026-11-27

This review follows the `improve-codebase-architecture`, `codebase-design`,
`domain-modeling`, `implement-spec`, `diagnosing-bugs`, and `show-me-your-work` workflows.
It is a plan and evidence record, not a claim that proposed architecture work is implemented.

- Visual report: `/tmp/architecture-review-20261127.html` (opened with `xdg-open`).
- Evidence inspected: recent commits `6a6d305`, `c88bbcc`, and `3c184c3`; the current startup
  splash worktree; `docs/GLOSSARY.md`; `docs/architecture.md`; and ADR-0001 through ADR-0003.
- Read-only GitHub triage snapshot: **0 open issues** and **1 open PR**. PR #19 is the current
  user's own draft and is not an external intake PR under the triage workflow. The repository has
  `bug` and `enhancement` labels, but no canonical state labels (`needs-triage`, `needs-info`,
  `ready-for-agent`, `ready-for-human`, `wontfix`); no tracker mutations were made.
- Full test result after this review's harness regressions: 172 unit/harness, 12 CLI, 4 hygiene,
  and 10 MCP tests pass (198 total), including the 500-object frame-time gate. Earlier full-suite
  runs under contention failed that gate at 125.2 ms and 120.7 ms against 120 ms. The latest pass
  does not resolve that variability; reliability remains an open task.
- Fresh coverage receipt from this worktree: `cargo llvm-cov --summary-only -- --test-threads=1`
  reports **70.84% lines, 71.17% functions, 68.46% regions** over 32,920 lines. Existing figures
  in `docs/AUDIT.md` remain an immutable 2026-11-25 snapshot. Current line coverage exceeds the
  67.61% floor, but is not near the requested 99% target.
- Dependency gates: `cargo deny check`, `cargo audit`, and `cargo machete` exited successfully.
  RustSec reports **0 vulnerabilities**, plus 4 unmaintained-crate advisories and 9 unsoundness
  advisories. All nine unsound findings are for `heapless 0.8.0`, pulled through
  `geo 0.33.1 → rstar 0.12.2`; RustSec lists `heapless >=0.9.4` as patched. The four unmaintained
  findings affect `ttf-parser`, `zbus_names`, `zvariant`, and `zvariant_derive`. These warnings
  still need an upstream-compatible resolution plan and are not a clean security bill of health.
  See [RUSTSEC-2026-0580](https://rustsec.org/advisories/RUSTSEC-2026-0580.html) for an example
  of the patched-version range and reachability conditions.
- Local Atheon pattern scan was run against the four edited Rust source files. It emitted 1,002
  heuristic findings across those complete files (not just changed lines), including high/critical
  labels for SQL concatenation, MCP exfiltration, and typosquatting. Spot checks resolve to ordinary
  Rust identifiers (`multi-select`), keyboard help text and geometry/control-flow; no SQL database
  client crate is declared in `app/Cargo.toml`. Treat these matches as scanner false positives, not
  confirmed vulnerabilities; no ignores or source changes were added. The attempted repo-wide/diff scan was CPU-bound without
  producing a receipt and was stopped. The Aegis managed scan profile rejects this checkout path
  because FireCraft is outside its configured workspace allowlist; no Aegis pass is claimed.

## Ranked architecture candidates

### A. Bitmap-edit request seam — Strong

**Evidence:** `app/src/app.rs` `on_pointer_down` and adjacent pointer movement/bitmap helpers;
`app/src/canvas.rs` texture/render cache; `app/src/harness.rs`; open tasks 37.2, 37.6, and 37.8.

Raster target lookup, coordinate conversion, alpha mutation, gesture undo grouping, and cache
invalidation are closely coupled to broad pointer dispatch. Eraser work exposed open behavior for
transformed/scaled targets, selection clipping, opacity/brush tips, locked or hidden targets, and
visual export parity.

**Plan:** first write an acceptance matrix for dab, drag, transformed bitmap, selected region,
locked/hidden targets, undo/redo, reorder and export. Reproduce each current failure. Add a focused
test at the nearest real call seam and see it fail before changing behavior. Then consider a small
document-space edit request/result interface, with FireCraft retaining state ownership. Preserve
one undo transaction per stroke and invalidate the correct texture identity.

**Deletion test:** removing this module would put target lookup, pixel indexing, masks and mutation
policy back into pointer dispatch. The module would earn its depth if that consolidation is proven.

### B. Shared canvas/export semantics — Worth exploring

**Evidence:** `app/src/canvas.rs` `render_viewport` and `app/src/formats.rs` `draw_obj`/flat export;
glossary rules for God view, Page bounds, and Flat export; tasks 37.5 and 37.7.

Canvas rendering and flat export intentionally use different backends. Their duplicated traversal
and visibility decisions can diverge on page clipping, hidden objects, gradients, effects or alpha.
Do not unify raster and egui backends. Inventory semantic overlap, define behavior fixtures first,
and share only stable scene visibility/bounds policy where the test demonstrates a drift risk.

**Deletion test:** without a shared policy or cross-backend contract fixtures, visibility behavior
can change independently and silently. Keep backend antialiasing and pixel differences out of the
shared contract.

### C. Byte codecs behind the file-operation seam — Worth exploring

**Evidence:** `app/src/formats.rs` contains file format dispatch, filesystem access, SVG parsing and
export, raster drawing, and layered-PNG iTXt payload handling; `app/src/app/files.rs` already owns a
`FileService` seam; tasks 32.1, 33.5, and 36.2.

If clipboard and format corpus work expands, isolate only byte codecs that can accept byte slices
and return explicit parse/write results. Keep `FileService` responsible for paths and filesystem
effects; avoid one module per extension and preserve `.fw.png` compatibility fixtures.

**Deletion test:** deleting a deep codec should move byte-level parsing into orchestration and make
error-path tests harder. Do not extract until byte-slice regression cases establish the interface.

### D. Startup splash — Do not deepen as architecture work

The overlay implementation is small and does not justify a dedicated module under the deletion
test. Keep the current 2.4 second timing, non-blocking live workspace and direct-to-editor startup
covered by harness tests; pursue a new seam only if startup orchestration grows beyond presentation.

## Systematic implementation sequence

1. **Close current behavior proof:** finish task 37.9 with the full test/lint/docs/WASM results;
   keep native visual review and PR checks distinguished from headless tests.
2. **Bitmap correctness (37.2/37.6/37.8):** matrix → failing reproductions → smallest regression
   tests → algorithm change → undo/cache/export validation → visual evidence. No general raster
   abstraction until two genuine adapters or call paths justify it.
3. **Render semantics (37.5/37.7):** record per-tool geometry/render/export expectations; create
   shared fixtures for semantics; test each backend independently; only then extract a shared
   policy if duplicate decisions are causing observed drift.
4. **Format hardening (32.1/33.5/36.2):** expand byte fixtures for malformed PNG chunks, layered
   payload versions, SVG edge cases and clipboard transfers; measure coverage; split the smallest
   codec with independent behavior and retain FileService as the path adapter.
5. **Verification reliability:** re-run the 500-object timing case under isolated/recorded host
   load; determine whether its 120 ms gate measures a real regression or is too sensitive to
   scheduler noise; preserve the threshold unless new measurements justify an ADR-backed change.
6. **Coverage and quality:** take a current `cargo llvm-cov` receipt; raise coverage by critical
   behavior and module, especially `layout`, `menus`, `dialogs`, `canvas`, and panel gestures.
   Treat 99% as an explicit target requiring a stated denominator/exclusions, not a current claim.
7. **Release readiness:** reconcile these tasks with hosted exact-SHA checks, WASM runtime/browser
   receipts, Windows 11 review, package hashes and user acceptance before publishing a release.

## Domain and decision review

No new product-domain term was required; the existing Glossary terms Bitmap/Canvas were insufficient
for any new domain concept, so this review uses existing Object, Layer, Page bounds, Flat export,
and God view terms and proposes no glossary edit. ADR-0001 (single portable binary), ADR-0002
(best-effort Windows shell), and ADR-0003 (deferred format/timeline scope) do not conflict with
these candidates. No ADR is warranted until a concrete, hard-to-reverse seam or format policy is
selected.

## Triage and implementation-spec disposition

Issue tracker scan returned no open issue to categorize or transition. The existing task ledger is
the authoritative local issue graph; the detailed ordering above is mapped to tasks 37.2, 37.5–37.9,
32.1, 33.5, and 36.2. Do not create duplicate GitHub issues. Existing state-label mappings are
missing, so state transitions must wait for deliberate repository label configuration.

The Atheon results show that broad pattern scanning currently has poor signal-to-noise on this Rust
UI code: generic words such as “send,” “insert,” and string concatenation are misclassified as
exfiltration, typosquatting, or SQL. Keep scanner output advisory until patterns are calibrated on
Rust UI fixtures and the actual changed-line diff. Do not mark large findings batches as security
defects without source-level confirmation.

## TDD evidence limitation

The startup implementation was already present as uncommitted work when this resumed session began,
so its original failing-before state cannot be reconstructed without undoing another session's
work. I added a focused harness regression for the reported Tab-focus flow, injected a temporary
early return matching the old modal splash failure shape, and observed the test fail specifically at
`Tab should enter canvas mode`; after restoring the source, the same test passed. This verifies that
the new regression check catches that failure shape, but does not claim an unmodified historical
checkout was captured. The transparent-asset test passed against the current asset without a code
change.
