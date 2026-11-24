# Changelog

All notable changes to **FireCraft** are documented in this file.

## [Unreleased]

### Added
- **Infinite canvas god view:** the workspace now continues the page background past the page
  bounds, shows out-of-page and hidden layers/objects (hidden geometry at ghost opacity), marks the
  printable page with a dotted boundary, and expands pan limits. Exports stay page-bounded.
- **Find and Replace modes:** Text, Fill Color, Stroke Color, and Any Color, including gradient
  stops and live effects, with live match counts, Find Next, color pickers, hex input, and
  “Use Fill”/“Use Stroke”.
- **Windows shell integration:** explicit AppUserModelID, New/About jump-list tasks, and recent
  documents; shell failures are non-fatal.
- **Release profile:** `opt-level=3`, fat LTO, one codegen unit, symbol stripping, and abort-on-panic.

### Changed
- Rebuilt the left rail as a three-column, 36-tool palette so no tool hides behind a flyout; added
  Oval Marquee, Freeform, Replace Color, and Red Eye with shortcuts.
- Rebuilt the file browser as an in-app window with an editable address bar, breadcrumb navigation,
  type filter, dense columns, and quick places. This removes the drag lag of the old viewport.
- Redesigned About and standardized dialog headers/footers, movable/resizable windows, and one
  primary-button treatment.
- Fixed the Unsaved Changes / Quit prompt clipping and wired Save/Discard/Quit continuations.
- Split Find/Replace, dialog file browsing, and the tool palette into focused modules. Added a
  navigable `docs/` tree (architecture, build/release, features, ADRs, glossary) and shortened the
  README to an entry point.

### Fixed
- Hidden text objects and color values are included in Find/Replace matching and replacement.

## [0.2.1] — 2026-11-24

### Added
- **Infinite canvas mode:** new `∞ Canvas` toggle in the canvas footer draws a dotted page-bound
  box with an export disclaimer. Out-of-page objects remain editable.
- **Layered `.fw.png` documents:** saving a `*.fw.png` now embeds a versioned project payload in an
  iTXt chunk. Reopening restores layers, states, symbols, styles, and out-of-page objects while the
  visible PNG stays cropped to the page.
- **Intelligence inspector tab:** OpenAI-compatible endpoint/model/key settings, real prompt calls
  with selection context, response/error display, and MCP client-config copying.
- **Window stacking:** dropping a floating panel on the top/bottom of another creates a persistent
  stack that moves as one unit; `Detach from Window Stack` restores independence.
- **Unsaved-session prompt:** window close and `Ctrl+Q` now offer Save All / Discard / Cancel.
- **Escape returns to the Pointer tool** and cancels in-progress creation, path, and text edits.
- New shortcuts: `[`/`]` brush size, bare `+`/`-` zoom, `Home`/`End` first/last state,
  `Ctrl+Alt+I` Intelligence tab, `Ctrl+Shift+F` infinite canvas.
- **Dynamic CS6 panel grouping, floating windows & magnetic snapping:** collapse/expand chevrons
  and double-click headers, bottom resize splitters, draggable tabs (merge, dock, or tear off),
  undock/pin controls, a Panel Options menu, dual-column right and left docking, floating windows
  with edge snapping, five new CS6 panels (21 total), and `CS6 Dual-Column` / `All Panels Floating`
  workspace presets.
- **Navigation and path audit:** `Insert` fits the page, `F11` mirrors `Tab`/`F4` for Full-Screen
  Canvas, path boolean commands operate on the full multi-selection, and Subselect can drag
  individual path vertices with live alignment guides.
- Added `.github/workflows/ci.yml` for formatting, Clippy (`-D warnings`), unit tests, and
  `wasm32-unknown-unknown` verification.

### Changed
- Moved the DPI/PPI selector from the Main Toolbar into the canvas footer beside resolution and
  zoom; removed duplicate zoom/resolution/state readouts from the status bar.
- Rebuilt the state transport with one monochrome icon set and proper disabled states.
- Merged the bottom inspector's double header into the shared panel header, with the collapse
  chevron left of the tabs and double-click-to-collapse on every panel bar.
- Properties inspector height is now app-owned and survives resize; the drag can no longer snap back.
- Tool rail pins the color chips and Full Screen button outside the scroll area; tool flyouts are
  compact and fit their labels. The narrow rail uses a full-width full-screen glyph so its label
  cannot clip.
- Panel-group tabs clip their labels to the tab rect and overflow into a `»` menu instead of
  painting labels on top of each other in narrow docks.
- Removed the redundant `Fc` icon and badge next to `File` in the menu bar so menus align cleanly
  to the left.
- Styles now apply to every selected object, and style gradients render as real gradient fills on
  rectangles, ellipses, polygons, and closed paths. Shadows/glows follow the object silhouette.
- Path Join chains nearest endpoints, Split produces two contiguous contours, and Union/Intersect/
  Punch/Crop use real polygon booleans (`geo`) instead of a convex hull / bounding-box approximation.
- Context/menu audit: Paste enablement, Join/Split availability, Intersect/Crop entries, and
  corrected shortcut labels.

### Fixed
- Native clear color forced to the workspace background, preventing the light flash when Windows
  re-composites the window (notably in full-screen mode).
- Cached `system_is_light()` in `theme.rs` with `OnceLock<bool>` and `CREATE_NO_WINDOW` on Windows,
  eliminating the UI freeze when switching to System/Auto theme.
- Clipboard tests no longer open an OS clipboard connection, so headless test runs cannot hang.

## [0.2.0] — 2026-11-22

### Added
- Dual-target native (`x86_64-pc-windows-msvc`) and WebAssembly (`wasm32-unknown-unknown`) builds with GitHub Release automation (`.github/workflows/release.yml`).
- CS6 `Alt+<key>` menu mnemonics with underline indicators and full arrow-key menu navigation.
- `Ctrl+Scroll` cursor-anchored zoom, `Shift+Scroll` horizontal pan, `Scroll` vertical pan, and `Alt+Scroll` / Middle-Drag / `Space+Drag` 2D Hand Pan.
- OS clipboard image/text/multi-object Copy/Cut/Paste (`arboard`) and redesigned CS6 Welcome Splash Screen.
- Redesigned **Align** and **Path** panels with custom CS6 vector diagram icons, FireCraft flame branding, and `1920×1080 @ 144 ppi` default page size.
