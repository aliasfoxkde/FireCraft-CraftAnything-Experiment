# Progress log

Newest first. One entry per work chunk. Record what changed, the gate results, and anything left
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
