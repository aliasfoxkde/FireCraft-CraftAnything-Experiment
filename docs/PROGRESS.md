# Progress log

Newest first.

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
