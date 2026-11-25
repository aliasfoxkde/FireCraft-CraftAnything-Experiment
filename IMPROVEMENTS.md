# Improvements — triaged

Raw user reports and design notes, now split by status. Open items have IDs and live in
[docs/TASKS.md](docs/TASKS.md); this file keeps the original wording as evidence.

## Fixed

| ID | Report | Evidence |
|----|--------|----------|
| I-01 | Can't paste elements from the clipboard | Clipboard paste implemented and covered in `dialogs/mod.rs` / `app.rs`; see ROADMAP pass. |
| I-02 | There is no splash screen | Splash sequence exists (`--splash`, startup splash); artwork embedded from `app/assets/splash.png`. |
| I-03 | `Ctrl`/mouse-wheel should zoom, `Shift`+scroll pan horizontally, plain scroll pan vertically, `Alt`+scroll hand-pan | Implemented in `canvas.rs` scroll handling. |
| I-04 | Upper-left corner shows an Fw icon instead of Fc | Toolbar branding uses the FireCraft mark. |
| I-05 | Improve branding / header label | Showcase page, title bar and About use FireCraft branding. |

## Open

| ID | Report | Tracked as |
|----|--------|-----------|
| I-06 | `Alt+F` should open the File menu with arrow navigation | TASKS 18.1 / 16.5 |
| I-07 | `Tab` should toggle Preview / full-screen canvas mode | TASKS 16.5 |
| I-08 | Expanding, pinning/snapping and grouping windows doesn't work | TASKS 16.3, 16.4, G-1 |
| I-09 | Align and Path windows need design/polish work | TASKS 18.1 |
| I-10 | General polish against the CS6 reference screenshots | TASKS G-1…G-4, 18.3 |
| I-11 | Commit, push and publish WASM/Windows builds | Needs owner approval to push/publish; local bundles are produced by `scripts/package-windows.sh`. |

## Rejected / not planned

| ID | Report | Why |
|----|--------|-----|
| I-12 | Ship a DLL directory with the executable | ADR-0001: one portable binary is the product decision. |
| I-13 | JavaScript-style obfuscation of the binary | No security value; breaks backtraces. Documented in `docs/build-release.md`. |
| I-14 | Recreate Adobe CS6 artwork/assets | Clean-room project; only original assets are shipped. |
