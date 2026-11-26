# Progress log

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
