# FireCraft source map

Where things live, not why. For seams, invariants and “must not own” rules read
[`docs/architecture.md`](../docs/architecture.md); for vocabulary read
[`GLOSSARY.md`](../GLOSSARY.md).

## Entry points

- `src/main.rs` — native and WASM entry points, CLI flag parsing, eframe launch, window icon.
  Flags: `--demo`, `--new`, `--splash`, `--about`, `--tab <0-3>`, `--size 1360x860`,
  `--screenshot <path>`, plus an optional file path. `list-commands`, `info`, `export`, `convert`, `batch`, `apply` and `mcp`
  dispatch through `mcp::try_run_cli` without opening a window.
- `src/mcp.rs` — CLI verbs, the `COMMAND_CATALOG` (68 commands shared with the command palette),
  and the JSON-RPC 2.0 MCP server (`2025-06-18`).
- `src/demo.rs` — the **showcase** document (`showcase(width, height) -> Vec<Obj>`) used by
  `--demo`, the Welcome window and templates; owns page-relative layout and text fitting.

## Application shell

| File | Contents |
|------|----------|
| `src/app.rs` | `FireCraft` state, `Tool` enum (36 tools), panel-group geometry/stacking, selection, undo/redo, pointer and command handlers |
| `src/app/files.rs` | File/session intents (`New`, `Open`, `Save`, `CloseTab`, `Quit`), dirty-state and close gating |
| `src/app/find_replace.rs` | Text and color match semantics, Find Next, replacement side effects |
| `src/layout.rs` | Top-level frame, menu/toolbar/status composition, docks, shortcut dispatch, full screen |
| `src/menus.rs` | The 10 CS6 menus and their actions |
| `src/canvas.rs` | Viewport painting, god view, hit testing, canvas context menu |
| `src/panels/mod.rs` | Panel registry, dock chrome, tab overflow, drag/resize/stack gestures |
| `src/panels/tools.rs` | Left tool rail: six categories in three columns, flyouts, colour footer |
| `src/panels/properties.rs` | Properties inspector body |
| `src/panels/color.rs` | Color Mixer body |
| `src/dialogs/mod.rs` | Shared dialog chrome, settings, About, command palette, MCP console, splash |
| `src/dialogs/file_browser.rs` | In-app file browser (address bar, filters, commit/cancel) |
| `src/dialogs/find_replace.rs` | Find & Replace dialog surface |

## Document and platform

| File | Contents |
|------|----------|
| `src/doc.rs` | `Document`, `Page`, `Layer`, `Obj`, `Shape`, styles, live effects, states, snapshots |
| `src/page.rs` | Page naming and slug rules |
| `src/formats.rs` | Raster/SVG/HTML/layered-PNG serialization, format metadata, recent-file policy |
| `src/colors.rs` | Colour conversions and swatch grid |
| `src/theme.rs` | Theme mode (dark/light/system), palettes, egui visuals |
| `src/icons.rs` | Monochrome icon drawing paths |
| `src/constants.rs` | Numeric constants with their `Fireworks.exe` function addresses |
| `src/shell_windows.rs` | Windows-only AppUserModelID and Jump List COM integration |

## Assets

- `src/logo.png` — window icon, embedded at compile time (732 KB).
- `../screenshots/FireCraft Flame Tech Logo.png` — splash artwork, embedded from outside the crate
  (see task 13.10 in `docs/TASKS.md`).
- `index.html` — WASM host page (currently duplicated by `release.yml`).
- `MECHANICS.md` + `mechanics.json` — extracted CS6 values; must agree with `constants.rs`.

## Build and test

```sh
cd app
cargo check
cargo fmt --all -- --check
cargo clippy --all-targets -- -D warnings
cargo test                                  # 46 unit tests
cargo check --target wasm32-unknown-unknown
cargo run --release -- --demo
```

Tests are inline `#[cfg(test)]` modules; there is no `tests/` directory yet. `DocumentFixture` in
`app.rs` is the intent-level fixture used by command tests.
