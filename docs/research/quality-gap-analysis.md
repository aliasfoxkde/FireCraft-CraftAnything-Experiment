# Code quality and verification gap analysis

**Snapshot date:** 2026-11-26  
**Repository:** FireCraft-CraftAnything-Experiment  
**Scope:** Read-only review of source structure, existing tests, documentation and the dated audit evidence. No build or test command was run for this report. The verification figures below are from [`AUDIT-2026-11-26.md`](../AUDIT-2026-11-26.md), not newly measured here.

## Executive assessment

The project has a strong foundation for a small, single-crate graphics editor: strict Clippy has passed, 131 automated tests passed in the recorded audit, the headless egui harness exercises real layout and interaction code, and architecture seams are documented. The current evidence does not support a 99% coverage or production-readiness claim. Recorded line coverage is **51.23%** (15,449 of 30,155 instrumented lines) and function coverage is **56.28%** (811 of 1,441 functions). High coverage in focused modules coexists with substantial untested UI, icon, layout, dialog and panel code.

The largest maintainability risk is concentration of behavior in a few large modules. Size alone is not a defect, but large UI modules make ownership, regression localization and focused coverage harder. The best next step is to increase behavior proof at stable seams, then split responsibilities only where tests can preserve the current behavior. Do not pursue a global percentage by testing trivial getters or implementation details.

## Current evidence and limits

| Area | Recorded evidence | Interpretation |
|---|---|---|
| Tests | 105 unit + 12 CLI + 4 hygiene + 10 MCP = 131 pass | Good logic and process-level baseline; not equivalent to full desktop E2E coverage. |
| Coverage | 51.51% lines; 56.28% functions; 51.23% regions; 24,139 instrumented lines | Current 2026-11-26 rerun. Preserve the tool version and denominator with future measurements. |
| Higher-region-coverage modules | `harness.rs` 97.48%; `doc.rs` 90.17%; `menus.rs` 76.32%; `formats.rs` 74.78%; `theme.rs` 94.54% | Existing tests are concentrated in core/model and selected interaction paths. |
| Lower-region-coverage modules | `icons.rs` 22.28%; `layout.rs` 44.54%; `dialogs/mod.rs` 40.59%; `panels/bodies.rs` 46.30%; `panels/mod.rs` 50.56%; `properties.rs` 40.32% | Prioritize behavior branches and keyboard/mouse paths with user impact. |
| Lint / compile | Strict Clippy and `wasm32-unknown-unknown` check passed in the dated audit | A compile check does not prove packaging, browser behavior, or deployment. |
| GUI | Six deterministic Xvfb captures at 1360×860; targeted one-click collapse regression | Useful rendered-state evidence, but not a full scripted user journey or native platform test. |
| Windows | GNU cross-build and CLI smoke under Wine passed | Does not establish native Windows 11 launch, clipboard, shell integration, installer, or update behavior. |
| Docs | `scripts/check-docs.sh` passed for the then-current 37 checked documents | Validates links/claims in scope; not completeness, accuracy of every feature description, or external Wiki publication. |
| Accessibility | Selected text contrast tested at AA thresholds | No criterion-by-criterion WCAG 2.1 AA/AAA conformance evidence; keyboard, focus, screen-reader and reduced-motion validation remain incomplete. |

See the audit snapshot for exact commands and provenance. Results must be rerun after material changes before being described as current.

## Maintainability and code-smell risks

The following are structural risks, not assertions that every line is defective. The repository architecture guide documents the intended owners and invariants: [`architecture.md`](../architecture.md).

| Area | Observed size | Risk and recommended boundary |
|---|---:|---|
| `app/src/app.rs` | 5,098 lines | Central app state is appropriate, but interaction routing, editing operations, and document orchestration remain concentrated. Continue extracting cohesive command/gesture behavior behind typed request/outcome seams; preserve `FireCraft` as state owner. |
| `app/src/canvas.rs` | 2,225 lines | Rendering, hit testing and canvas context behavior are a dense surface. Maintain separate pure geometry/hit-test tests from viewport rendering tests; avoid broad rewrites. |
| `app/src/layout.rs` | 2,080 lines | Top-level composition and dock/shortcut behavior are large. The architecture guide already records `layout/chrome.rs`; continue only when a cohesive UI region has behavior tests. |
| `app/src/icons.rs` | 1,980 lines | Largest low-coverage risk by test signal (22.28%). Likely repetitive mapping/paint code; verify representative variants, fallback behavior, and icon-font registration without brittle pixel assertions. |
| `app/src/panels/bodies.rs` | 1,913 lines | Multiple panel bodies create a broad change surface. Test visible controls against document state and ensure disabled/empty/selection states behave consistently. |
| `app/src/dialogs/mod.rs` | 1,590 lines | Shared dialog chrome plus several dialog surfaces can couple presentation and effects. Preserve the documented rule: dialog owns layout, app method owns multi-step effects. |
| `app/src/formats.rs` | 1,586 lines | Multiple import/export codecs and persistence concerns share one module. Split by format only after format-specific golden/error tests exist; maintain cross-format invariants in a shared metadata layer. |
| `app/src/panels/mod.rs` | 1,403 lines | Registry and chrome are established, but host gestures remain sizable. Deepen the geometry/gesture seam and test drag, detach, merge, resize and close outcomes before moving code. |
| `app/src/menus.rs` | 1,252 lines | Test coverage is healthier. Guard menu command-to-action wiring and keyboard reachability as entries grow. |
| `app/src/mcp.rs` | 890 lines | Catalog, dispatch, JSON-RPC/CLI and JSON helper responsibilities risk mixing protocol with app command semantics. Separate only with protocol conformance tests and a stable command registry. |

The active architectural invariants worth treating as regression contracts are: document slot synchronization before save/switch/close; selection reset when changing layer; god-view rendering must not change export bounds; shortcut gates must depend on text-edit focus rather than generic widget focus; dialogs delegate consequential effects to app methods; and platform shell failures must not block startup. These are described in [`architecture.md`](../architecture.md) and should anchor the test plan.

## Sequenced improvement plan

Each item is intentionally small enough to complete and review independently. Put task state and proof links in [`TASKS.md`](../TASKS.md); keep measured results in dated audit snapshots rather than silently replacing historical values.

### Phase A — Close user-visible regression gaps

1. **Clipboard lifecycle and focus.** Add deterministic tests for copy, cut, paste, duplicate, empty clipboard, incompatible payload, and undo/redo. Add an egui harness path for keyboard shortcut focus after using tabs, menus and canvas. Acceptance: each command produces expected document/selection state and undo returns to the exact prior document; no shortcut is swallowed by stale widget focus.
2. **Save/recovery behavior.** Test dirty-state transitions through save, save-as, failed save, close prompt, reopen, and recovery prompt. Acceptance: injected file service covers success and failure without real user files; active document slot and dirty state remain consistent on every path.
3. **Dialog outcomes.** Cover file filters, extension selection, cancel, invalid path, overwrite confirmation and keyboard navigation. Acceptance: tests assert resulting intent and document state, not egui implementation details.
4. **Panels and canvas focus.** Extend the headless harness around a single representative test per gesture family (drag/resize, detach/reattach, tab switch, canvas focus). Acceptance: user input sequence and final layout/focus state are asserted; no screenshot-only test substitutes for state checks.

### Phase B — Raise coverage where the measured gap is largest

5. **Icons and theme.** Add mapping/registration tests covering all icon enum entries and font fallback behavior; test contrast for every shipped palette and normal/large text using a named contrast calculation. Acceptance: all icon variants resolve, fallback remains legible, and contrast tests state the precise threshold tested. Do not label partial contrast checks WCAG conformance.
6. **Layout and dialog branches.** Exercise narrow/wide window, collapsed docks, floating canvas, empty document, open dialog, and keyboard operation. Acceptance: key controls remain reachable at supported sizes and persisted layout state round-trips.
7. **Panel controls.** Test every panel action that mutates document state at its app-method seam (selection, visibility, lock, style, filter). Acceptance: behavior and undo are asserted for single and multi-selection, with coverage reporting attached to the exact module.
8. **Formats.** Expand valid/invalid/truncated fixtures and round-trip tests for each supported format. Acceptance: formats preserve documented fields and reject malformed inputs without panic; golden updates require explicit review. Keep the current `formats.rs` suite as a baseline while evaluating a format-per-module split.

### Phase C — Reduce structural concentration safely

9. **Input/gesture dispatcher.** Select one complete gesture family, give it typed input and outcome values, and isolate interpretation from mutation. Acceptance: existing behavior tests pass unchanged, new tests cover boundary/cancel paths, and no layer-selection/undo/export invariant is duplicated.
10. **Codec boundaries.** Once fixtures cover the supported behavior, move one codec at a time behind narrow format modules while retaining shared metadata and error types. Acceptance: byte/golden outputs are unchanged unless an intentional format correction is documented; tests run per codec and shared round-trip cases.
11. **MCP protocol boundary.** Separate command catalog and app command dispatch from transport parsing/serialization. Acceptance: JSON-RPC success/error, malformed input, unknown command, and CLI behavior remain protocol-tested; no GUI state access is introduced into protocol code.
12. **Module size policy.** Treat line counts as review prompts, not hard quality gates. For a module above 1,500 lines, document why it remains cohesive or identify one extraction with an owner and tests. Acceptance: each extraction reduces coupling or improves test isolation and does not create generic “utils” modules.

### Phase D — Make quality claims reproducible

13. **Coverage ratchet.** Keep the existing floor and raise it only when each increment is backed by behavioral tests and stable instrumentation. Set floors by module for the modules improved in phases A–C; begin with the lowest meaningful owners, not generated or repetitive UI code. Acceptance: CI fails when a floor regresses and reports the exact source revision, toolchain, line/function denominator and excluded files.
14. **End-to-end matrix.** Identify scripted flows that cover create/edit/undo/save/reopen/export, clipboard, settings and update UI. Acceptance: run on Linux GUI and browser/WASM in CI; native Windows and macOS receipts are separately recorded. Screenshot diffs supplement interaction assertions, not replace them.
15. **Accessibility audit.** Build a criterion-level WCAG 2.1 checklist for each web surface, then run automated checks plus manual keyboard, focus, zoom, screen-reader and reduced-motion checks. Acceptance: findings map to criterion, page/component, reproduction, owner and evidence; claim only the conformance level actually assessed.
16. **Documentation evidence map.** Link each shipped feature page to implementation owner, user-facing instructions, and test/verification evidence. Acceptance: broken links fail CI; doc claims map to current behavior or are explicitly marked planned/unverified; external Wiki publication is independently verified.

## Coverage strategy and interpretation

The user's requested “99%” target is tracked as a long-term goal across separate evidence domains, not as one blended percentage. First establish per-module Rust line/function/branch denominators where supported, user-flow test inventory, documentation link/claim inventory, and criterion-level accessibility coverage. Coverage says which instrumented source lines ran; it does not establish correctness, accessibility, platform compatibility, or documentation accuracy. A defensible strategy is:

1. Preserve the current global measurement as the dated baseline (51.23% line, 56.28% function).
2. Raise a module floor only alongside a meaningful behavior gap being closed; retain uncovered-line reports for risk triage.
3. Prioritize externally visible, stateful, destructive and serialization paths over trivial getters or generated/repetitive drawing branches.
4. Use property/golden tests for geometry and format invariants, harness tests for real egui input/state, process tests for CLI/protocol, and native smoke tests for platform-only behavior.
5. Define a reproducible 99% target and ratchet for each domain independently; keep exclusions and a dated baseline visible. Report tool/version, commit SHA, and denominator. Never infer “99% tested” from an aggregate percentage alone.

## Evidence still missing

- Full interactive end-to-end flows on supported native desktop platforms.
- Real Windows 11 behavior for clipboard, focus, packaging, updater and Windows shell integration.
- macOS runtime, signing/notarization and Linux package installation/launch receipts.
- Browser WASM interaction and deployment smoke beyond successful target compilation.
- Full autosave/recovery, updater/changelog and issue-submission behavior as those features are implemented.
- Criterion-level WCAG 2.1 AAA verification; current AA contrast checks are only a narrow subset.
- A clean pattern-scan baseline: the reported Atheon scan produced substantial known false-positive noise, while Aegis was unavailable through the configured profile. Treat neither as a clean bill of health.

## Related documents

- [`architecture.md`](../architecture.md) — module ownership, invariants and extraction opportunities.
- [`AUDIT-2026-11-26.md`](../AUDIT-2026-11-26.md) — dated verification measurements and platform/release evidence.
- [`TASKS.md`](../TASKS.md) — active work items and proof requirements.
- [`PRODUCT_ROADMAP.md`](../PRODUCT_ROADMAP.md) — product phases and future scope.
- [`A11Y.md`](../A11Y.md) — accessibility scope and known limitations.
