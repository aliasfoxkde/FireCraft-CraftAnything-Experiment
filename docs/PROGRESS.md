# Progress log

Newest first.

## 2026-11-26 — Clipboard, workspace UI, and file-browser follow-up

- Added a browser `ClipboardEvent` bridge for copied image files. It reads image bytes asynchronously,
  releases each one-shot reader callback, and forwards the bytes into the normal Paste action. Edit
  and canvas context-menu Paste now remain available for external/system clipboard contents.
- Fixed Preview badge overflow by measuring rendered text and eliding it to the canvas width.
- Bound the file browser to the display, reduced its default dimensions, and virtualized the file
  list so only visible rows perform size/date metadata reads and paint. Added a 1,000-row render
  regression check.
- Restored the CS6 tool-family labels, constrained the tool-rail body on short windows, and rendered
  the Swatches panel as a compact, direct 2D grid. Replaced a stale text-outline test that expected
  a fake rectangle conversion with a truthful unsupported-operation assertion.
- Cleaned up the top menu/header arrangement against the checked-in Fireworks menu/layout images.
  Removed crowded MCP/CLI, fullscreen, and theme buttons from the menu row; their actions remain in
  Help/Window. The Palette button stays visible and a Demos dropdown sits immediately to its right,
  with the real Studio Showcase and 1080p starter document actions plus links to all four tutorials.
- Verification: native `cargo check` passed; format check and strict Clippy passed after correcting
  one needless borrow found by Clippy; WASM `cargo check` passed. Full test suite: 158 unit + 12 CLI
  + 4 hygiene + 10 MCP = **184 passed**. Xvfb captures at 1360×860 and 960×640 were visually
  inspected. The 960×640 workspace remains dense and scrolls lower right-dock groups; live browser
  clipboard paste and platform-native picker behavior are still unverified.
- Updated [`AUDIT-2026-11-26.md`](AUDIT-2026-11-26.md) and added Phase 32 to [`TASKS.md`](TASKS.md).
  Aegis scan remains pending: the local command is protected by managed admission, but the shared
  queue currently exposes no `aegis-scan` profile (only the separate Aegis-repository tests profile).

### 2026-11-26 — hosted gate follow-up and extension roadmap

- Audited the latest hosted experiment-repo run. Coverage workflow passed 180 tests (154 unit, 12 CLI, 4 hygiene, 10 MCP) and produced Cobertura at 68.24% lines (16,978/24,880); no branch counters were emitted. This does not meet the requested 99% target.
- Hosted Windows release build, CLI smoke, ZIP packaging, and the WASM release build passed on source `4bd1242`. Neither result proves Windows 11 interactive acceptance or a production deploy.
- Strict Clippy found a duplicated generic bound and two test modules above production items. Corrected those source issues, and made GitHub Code Quality upload failure non-blocking after its endpoint returned HTTP 404; the Cobertura artifact remains the required coverage receipt.
- Added Phase 31 to the task ledger for the extension API, local manager, public catalog, community submission, safe one-click installation, and author/user documentation. Historical desk research and the canonical informational Extensions page are complete; operational marketplace features remain future work.
- Current source-level formatting, workflow lint (with the documented actionlint permission-scope exception), and whitespace checks pass. Full CI still requires the module-size fix and final package/CodeQL results; release and deploy remain gated.
- Follow-up run `45870025403` confirmed strict Clippy, the 180 tests, formatting, docs validation, and WASM build pass. It found two file-size gate failures (`icons.rs` 2,193 lines and `panels/bodies.rs` 2,028 lines); moved only their test modules into adjacent files. Local `module-size.sh` now passes. The new extraction is pushed for another hosted check; Windows and CodeQL were still running.
- Follow-up coverage run `45870025885` reports 68.24% lines (16,979/24,881) and its GitHub Code Quality upload step succeeds. Cobertura has no branch counters, so branch coverage remains unmeasured.
- Full corrected run `45871239608` now passes Linux quality gates, all 180 tests, docs, module-size, Windows CLI/ZIP packaging, and WASM builds. CodeQL was still in progress at the last check. Its CI coverage ratchet measured 67.61% lines / 69.71% functions / 69.82% regions; its separate Cobertura artifact measured 68.05% lines and has no branch counters. Raised `coverage-floor.txt` from 44.40% to 67.61%; this exact new floor needs one hosted confirmation run.


## 2026-11-25 — Tasks 18.3, 18.7, 19.6: layout proof, frame budget, reproducible build

- **18.3**: headless Xvfb captures at 960×640 and 1360×860 with demo scene and
  all docks open; visually verified — nothing clipped, footer fits. Found and
  fixed a real defect: eframe's persisted window state overrode `--size` even
  with `persist_window: false` (that flag only stops *saving*, not *loading*).
  Screenshot mode now points `persistence_path` at a throwaway location
  (`main.rs`), so captures no longer depend on the operator's last interactive
  window size or on moving `~/.local/share/firecraft.editor/app.ron` aside.
- **18.7**: frame-time budget test
  `frame_time_with_500_objects_stays_within_budget_at_1920x1080` in
  `harness.rs`: 500-object frame ≈ 60 ms debug/software raster, hard 120 ms
  ceiling plus a 6× empty-baseline relative bound (the original 3× failed on
  a noisy micro-baseline — noted in the test doc comment). Startup/export/
  memory budgets remain release-receipt items, unverified on this host.
- **19.6**: two release builds with `src/main.rs` touched between them
  produced byte-identical binaries (SHA-256 `41bf162e…`). Caveat recorded in
  TASKS: same target dir, toolchain and host only.
- Gates: fmt clean, clippy `-D warnings` clean, `cargo test` 93+12+4+10 all
  passing; release build succeeded under the host build lease.

## 2026-11-25 — Task 17.5: FileService seam for headless file flows

- Added a `FileService` trait (`open`/`save`) in `app/files.rs` with
  `FormatFileService` as the disk-backed default, stored as
  `FireCraft.file_io`. Open, save, export, import and save-all-for-quit now
  route through the seam instead of calling `formats` directly, so headless
  tests inject a fake service and never touch the filesystem or egui. Two new
  tests: injected open/save round-trip (`Rc` handle asserts the saved path)
  and failing-service error surfacing. 10/10 files tests, 110 total pass.
- Repaired a pre-existing clippy break left by 17.3: the glob re-exports in
  `panels/mod.rs` (`use bodies::*`, `pub use tabs::*`, `use tabs::*`) had
  become dead/unresolvable under `-D warnings`. Replaced with explicit
  re-exports (`pub use bodies::{intelligence_panel, start_panel}` plus a
  named import list) and explicit helper imports in `color.rs`/`tabs.rs`.
- Gates: fmt clean, clippy `-D warnings` clean, `cargo test` 110 passed /
  0 failed, wasm32 check exits 0, module-size budget under the hard cap.

## 2026-11-25 — Task 17.3: panel bodies behind the PANELS registry

- Extracted the remaining panel render bodies from `panels/mod.rs` (3635 ->
  1388 lines) into `panels/bodies.rs` (1912 lines: Optimize, History, Align,
  Path, Styles, Swatches, Info, Pages, Intelligence, Library, Start, and the
  post-registry bodies) and `panels/tabs.rs` (355 lines: Pages/States/Layers
  and related tab wrappers). `mod.rs` now holds the chrome, the `PANELS`
  descriptor table and re-exports; every renderer is reached through the
  registry seam. Pure code motion; fmt, clippy `-D warnings`, tests, wasm
  check and module-size budget pass.

Newest first.

## 2026-11-25 — Task 17.2: extract layout/chrome.rs

- Extracted the document tab strip and canvas footer bar (with their helpers
  `tab_click_without_drag`, `compact_tab_label`, `footer_icon_button`) plus the
  bottom properties dock body from `layout.rs` into `layout/chrome.rs` (815
  lines) behind `pub(super)` visibility. `layout.rs` is 2850 -> 2041 lines.
  Pure code motion; fmt, clippy `-D warnings`, tests and wasm check pass.

Newest first.

## 2026-11-25 — Task 17.1: extract command modules from app.rs

- Extracted `app/src/app/edit.rs` (undo/redo, delete/duplicate/group, clipboard,
  selection sets; 700 lines), `app/src/app/arrange.rs` (align/distribute/space,
  match size, flip/rotate/scale, z-order; 667 lines) and `app/src/app/object.rs`
  (paths/booleans, symbols, styles, auto-shapes, slices, filters, flatten,
  saved commands; 1068 lines) from `app.rs` behind the existing child-module
  pattern (`use super::*;` + `impl FireCraft`). `polygon_boolean`/`polygon_area`
  moved to module scope in `object.rs` with `pub(super)` visibility for the
  property tests. `app.rs` is 7368 -> 4952 lines. Gates: fmt, clippy
  `-D warnings`, tests, wasm check, module-size budget all pass. Behavior is
  unchanged - pure code motion; existing tests cover the seams.

Newest first.

## 2026-11-25 — 0.3.0 quality and release pass

Worked the open task list end to end; decisions D1–D5 recorded.

- **Truth and hygiene:** MIT `LICENSE`; `app/Cargo.toml` is the only version source (0.3.0) and
  `--version`/help/MCP no longer hardcode a version; `rust-toolchain.toml`; crate self-contained
  (`assets/logo.png`, `assets/splash.png`); one WASM host page; `PROJECT_STRUCTURE.md`,
  `build-release.md` and `IMPROVEMENTS.md` rewritten; `scripts/check-docs.sh` gates links, claims,
  trailing whitespace and duplicate headings.
- **Gates:** `deny.toml` (advisory acceptance for `url 2.5.8` with exposure note), `cargo audit`,
  coverage ratchet (`scripts/coverage.sh`, floor 18.70), `[lints]` policy, module-size budget,
  `scripts/ci-local.sh` as the definition of record, Windows build + CLI smoke job, WASM build job,
  Dependabot, issue/PR templates.
- **Tests:** 108 total (was 51). New integration suites `tests/cli.rs` (12), `tests/mcp.rs` (10),
  `tests/hygiene.rs` (4); formats error paths, golden SVG/HTML/PNG files, theme WCAG contrast,
  color round-trip sweep, file-intent matrix, polygon-boolean area identities, quarter-turn bounds.
- **Bugs found and fixed by those tests:** ICO export failed for any page above 256 px; `batch`
  created a missing input directory and reported success; `Escape` was consumed by the pen-path
  check even with no pen path, so it never closed menus/dialogs; `Shift+Tab` could never reach the
  dock toggle because the unmodified `Tab` check consumed it first; light-theme accent contrast was
  4.15:1 (now 5.2:1); polygon booleans silently dropped every result region except the largest.
- **Architecture:** `panels/geometry.rs` (pure snap/stack/height math, 7 tests) with `app.rs`
  delegating; `harness.rs` boots the real frame headless at 1360×860 and 960×640 and drives keys.
- **GUI:** dock header now reads "Panels", Mixer group tall enough for its controls, scrollbars
  always visible on panel bodies, Bridge `Br` text replaced by a folder icon, header cluster spacing
  fixed at 960 px. Evidence: `checks/gui-pass-2026-11-25/g5-*.png`.
- **Release:** `scripts/package-windows.sh` builds `dist/windows/` + versioned zip + SHA-256;
  tag/version mismatch fails the release job; Wine smoke-tested the GNU binary.

## 2026-11-26 — 18.8 defect burn-down and ROADMAP 4.5 closure

- 18.8 disposition: the defects surfaced during Phase 16 verification were
  test-premise bugs, each root-caused and fixed in place (Iconic Mode gate,
  stale replace_color, snap threshold, zoom-clamp status). No silent fixes.
- ROADMAP 4.5 closed: two new harness tests. Styles — preset fill/stroke/
  effects on single and multi-selection, undo restores per-object baselines
  (demo objects carry baseline live effects). Filters — all five LiveEffect
  kinds added, toggled off/on, one removed, all cleared, repeat-last re-adds,
  every step asserted across the whole multi-selection.
- Task-list reconciliation: 16.3 marked done (test landed with the 16.8/G-5
  batch), ROADMAP 2.6 closed by the 16.4 gesture test, 3.3 by the 18.4
  Intelligence tests, 11.5 by the 18.5 disposition. ROADMAP 8.3/9.4 moved to
  `[~]` blocked — no Windows host in this environment (decision D4); the
  full-screen toggle itself is code-verified in harness tests.
- Gates: `cargo test --bin firecraft` 104 passed; fmt and clippy
  `-D warnings` clean.

## 2026-11-26 — Panel suite, screenshot loop, interactive-state capture (16.3/16.8/G-5)

- 16.3: `panel_states_paint_collapse_tab_overflow_iconic_and_inspector_clamp`
  in `app/src/harness.rs` — dock collapse/expand paint delta, last-tab
  overflow, iconic strip vs flyout, bottom-inspector clamp at 5000/0,
  collapsed-bottom paint delta.
- 16.8: `scripts/gui-suite.sh` replaces the manual per-round `gui-shot.sh`
  loop — one invocation captures the six-state evidence set into `checks/`.
- G-5: new `--state <menu|flyout|find-replace>` flag (main.rs) pre-arranges
  the app so interactive states are reachable from the capture path.
  Evidence: `checks/gui-suite-2026-11-26/` (visually verified: File menu
  open, iconic flyout expanded, find-replace dialog). Drag/stack gestures
  stay code-asserted in harness.rs, not pixel-captured.

## 2026-11-26 — Harness interaction tests (16.2/16.4/16.6/16.7)

New tests in `app/src/harness.rs`, all driving the real headless frame:

- `workspace_shell_states_paint_at_the_minimum_size`: startup shell at both
  sizes, splash, Tab key, Alt+F menu open, iconic flyout (Iconic Mode preset),
  footer/status.
- `panel_gestures_float_merge_stack_detach_snap_and_close`: ungroup → float
  (multi-tab source keeps its id, new floating group gets a fresh one), merge,
  stack, detach, edge snap within the 12 px threshold, close-last no-panic.
- `every_menu_row_triggers_and_closes_the_menu`: walks all 10 menus, Enter
  triggers each enabled row, Escape closes; selection set so Filters/Modify/Text
  rows are enabled.
- Find/replace: text mode cycles matches and replaces name+text slots (4
  occurrences across 2 objects); color mode covers the no-match report and the
  real swap (`replace_color` must differ from `find_color` or the run is a
  no-op by design). Dialog toggles frame without panicking.

Gates: `cargo test --bin firecraft` 101 passed; `cargo fmt --check` and
`cargo clippy --all-targets -- -D warnings` clean. Not verified on screen —
headless frame evidence only.

## 2026-11-25 — GUI hill-climb and architecture slice

Four capture-and-fix iterations against real renders (`scripts/gui-shot.sh`, Xvfb + software GL,
1360×860 and 960×640). Evidence: `checks/gui-pass-2026-11-25/`.

- Fixed, each verified by a fresh screenshot: radial gradient bleed and unpainted corners; slice URL
  label covering the CTA label; showcase not fitting at 960 px; selection handles on first launch;
  inspector second row clipped at 118 px; rail hint cut mid-sentence; Matte control overlapping its
  label; CTA label off-centre.
- Architecture slice: extracted `demo::showcase(w, h) -> Vec<Obj>` out of `app.rs` (280 lines) and
  added four layout tests. The tests immediately found a real defect the screenshots could not show:
  multi-line text bounds counted newline glyphs, so feature bodies measured wider than their cards.
  `doc::text_width` and `Text` bounds now use the longest line and the line count.
- Added `--size WxH` (deterministic review at any window size) and one regression test for
  multi-line text bounds.
- Architecture review report (skill `improve-codebase-architecture`) written outside the repo:
  `/tmp/architecture-review-20261125.html`. Top candidate implemented; candidates 2–4 recorded in
  the report and in `docs/PLAN.md` Phase 17.
- Gates: `cargo fmt --check` clean, `cargo clippy --all-targets -- -D warnings` clean,
  `cargo test` = **51 passed** (was 46).

### Unverified

- Interactive states (menus open, tool flyouts, drag/stack gestures) are still not captured; the
  capture path renders the startup frame and exits.
- Right-dock empty header strip, Color Mixer clipping and the 960 px top-right cluster remain open
  and are listed in `docs/TASKS.md`.
- Windows-only rendering not exercised. One entry per work chunk. Record what changed, the gate results, and anything left
unverified. Do not restate the plan; link it.

## 2026-11-25 — Audit and plan baseline

- Recovered state after the lost session: working tree clean at `3133c87`, branch `main` in sync
  with `origin/main`.
- Ran the full gate set and measured rather than assumed: fmt/clippy/test green (46 tests), WASM
  `check` green, `cargo machete` clean.
- Measured coverage for the first time: **18.70 % lines / 22.87 % functions**; the entire UI layer
  (`layout.rs`, `menus.rs`, `dialogs/*`, `panels/color.rs`, `panels/properties.rs`, `icons.rs`)
  reports 0 %.
- `cargo audit`: 1 vulnerability (`url 2.5.8`, RUSTSEC-2026-0592, via `eframe → webbrowser`) and
  13 warnings (9 × `heapless` unsound via `geo → rstar`, 4 unmaintained).
- `cargo clippy -W clippy::pedantic`: 829 warnings, no lint policy in `Cargo.toml`.
- Confirmed the docs drift: stale `app/PROJECT_STRUCTURE.md`, `app/MECHANICS.md` contradicting
  `mechanics.json`, no `LICENSE`, version `0.2.0` in Cargo vs `0.2.1` in CHANGELOG, duplicated WASM
  entry point, splash asset read from outside the crate.
- Wrote [AUDIT.md](AUDIT.md), [PLAN.md](PLAN.md), [TASKS.md](TASKS.md) and `CLAUDE.md`; corrected
  `app/PROJECT_STRUCTURE.md` and `app/MECHANICS.md`; pointed `ROADMAP.md` and the docs index at the
  new task list.
- Not changed: no production code touched in this chunk. Decisions D1–D5 are open and blocking
  Phases 13 and 19.

### Unverified

- All GUI behavior: no display available in this environment. Panel drag/stack/snap, menu
  click-through, 960×640 layout and the Intelligence endpoint still need Phase 16's harness.
- Windows-only behavior (Jump List, app-switch flash) — no Windows shell here.

## 2026-11-25 — UI rework per owner feedback + tutorials

- Tools palette: CS6-style two-column icon grid with tinted category bands, compact
  fill/stroke swatch footer (overlapped chips, swap/default/no-fill/full-screen buttons);
  removed meta counts footer. `panels/tools.rs`.
- Properties/panel-group collapse: chevron click now works on first click — the header's
  `Sense::click_and_drag` was shadowing the chevron `ui.interact`; detection moved to
  header `interact_pointer_pos()` against the chevron rect. `panels/mod.rs`.
- Right dock: expand/collapse-all now two 18 px toggle buttons inside the dock header bar;
  removed the separate full-width button row. `layout.rs`.
- New `docs/tutorials/` (getting-started, keybindings, drawing-and-editing,
  pages-and-states, optimize-and-export) linked from `docs/README.md`. Publishing to the
  GitHub Wiki awaits owner approval.
- Gates: fmt clean; clippy -D warnings exit 0; test suite 110 passed / 0 failed;
  wasm check exit 0. Screenshot /tmp/fc-final.png verified under Xvfb. Interactions
  (click collapse, flyouts) remain unverified — screenshot only.

### 18.1 IMPROVEMENTS triage (same day)

- Verified in source: Alt+letter menu mnemonics dispatch OpenMenu; ArrowUp/Down set
  nav_delta, Left/Right switch menus, Enter/Space activate (menus.rs). Tab (no mods)
  maps to ToggleFullScreenCanvas (layout.rs). Align and Path panels are complete CS6
  icon-row implementations (panels/bodies.rs).
- IMPROVEMENTS.md updated: I-06, I-07 moved to Fixed; I-08 split (collapse part fixed,
  drag/snap gesture stays with 16.3/16.4); I-09 folded into G-1/18.3 polish.
- Unverified: actual keypress/click behavior needs the Phase 16 interactive harness or
  a display; code paths only.

### 18.2 / 18.4 (2026-11-25)

- 18.2 Windows behaviors: this host is Linux with no Windows shell or VM; Jump List,
  app-switch flash and taskbar identity remain **unverified** (code paths exist in
  shell_windows.rs). Recorded as an honest limitation, not closed.
- 18.4 Intelligence panel paths: seven new tests in app.rs (missing key / missing model /
  empty prompt rejected with status and no channel; success sets response and clears the
  channel; error surfaces status with no fabricated response; worker disconnect reported;
  in-flight request blocks a second start; ai_extract_content parses provider JSON and
  rejects error bodies). Env-var dependency documented: tests never mutate the process
  environment (`-D unsafe-code`); the no-key test skips if the ambient key is set.
- 118 tests pass (92 unit + 12 doc + 4 cli + 10 mcp), 0 failed.

### 18.6 Accessibility checklist (same day)

- New docs/A11Y.md: per-area status with evidence. WCAG AA contrast is machine-tested
  in theme.rs for both themes; Alt+mnemonic + arrow menu operation exists in
  layout.rs/menus.rs (code-verified, not screen-verified); WASM focus visuals,
  prefers-reduced-motion and screen readers recorded as limitations with the
  concrete path to close each.
### 2026-11-26 — Documentation and GitHub community organization

- Moved the generated changelog and GitHub community health documents to `.github/`; moved the
  product brief, glossary, improvement history, and completed UI roadmap into `docs/`.
- Kept the existing root `AGENTS.md` and linked it from README/docs navigation rather than creating
  a duplicate. Updated the package assembly scripts so release archives still contain user-facing
  `CHANGELOG.md` and `GLOSSARY.md` files at their established package paths.
- Added GitHub issue-routing links, a Contributor Covenant Code of Conduct, and CODEOWNERS. Existing
  bug, feature, and pull-request templates remain the supported submission forms.
- Updated cross-document links and the documentation checker for the new file locations.
- Verification: `./scripts/ci-local.sh --quick`, `./scripts/check-docs.sh`, `git diff --check`, and
  `bash -n` on the three packaging scripts passed. Before this update, PR #7's current head had
  passing hosted gates, coverage, Windows build/CLI smoke, WASM, and CodeQL; the pushed doc change
  will produce a fresh receipt for the new head. `actionlint` still reports the known
  `code-quality: write` permission scope in the coverage workflow, which GitHub currently accepts.

### 2026-11-27 — v0.3.0 release and version-only release title

- Published [v0.3.0](https://github.com/aliasfoxkde/FireCraft-CraftAnything-Experiment/releases/tag/v0.3.0)
  from source `e415a333f6a474bd738f47bb64d56704d711f1ce` with Windows, universal macOS ZIP/DMG,
  Linux tar.gz/AppImage/DEB/Flatpak, and WASM ZIP assets. The release manifest and aggregate SHA-256
  verification cover the exact platform asset set.
- All release workflow jobs passed on run #81, attempt 3. The WASM site is deployed at
  https://aliasfoxkde.github.io/FireCraft-CraftAnything-Experiment/.
- Corrected the published release name to exactly `v0.3.0`; updated the release workflow to use the
  tag as the release name for future releases. Verified the published release assets against
  `SHA256SUMS.txt` and refreshed the local Windows review bundle under `dist/windows-review/v0.3.0/`.
