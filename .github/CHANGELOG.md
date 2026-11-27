# Changelog

All notable changes to **FireCraft** are documented in this file.

## [Unreleased]

## [0.3.3] — 2026-11-27

Canvas editing, bitmap correctness, and clipboard reliability update.

### Added
- **Previous Tool shortcut:** press `Shift+Q` to toggle between the current and last distinct tool.
- **Feathered bitmap eraser:** erase bitmap alpha with a brush size and feather control; group a
  drag into one undo action.
- **Browser image clipboard regression:** exercise valid PNG payload decoding and malformed payload
  rejection through the same paste entry point used by the editor.

### Changed
- Keep source bitmap pixel dimensions separate from display bounds so scaling does not resample
  clipboard, imported, or existing bitmap data; raster export follows display bounds.
- Reuse bitmap textures by content identity after reorder and cloned render snapshots.
- Paint startup with a transparent overlay that fades over the live workspace instead of blocking
  the editor, and make the Welcome view opt-in.
- Clarify the public task ledger with measured test/coverage state and remaining platform acceptance.

### Fixed
- Decode browser clipboard image payloads on the shared paste path and show an explicit error for
  malformed private image payloads instead of inserting pasted marker text.
- Preserve eraser behavior across scaled bitmap bounds and undo.

## [0.3.2] — 2026-11-27

Browser clipboard reliability and roadmap planning update.

### Fixed
- Preserve asynchronous browser image-paste data until the next editor frame so WASM image paste
  reaches FireCraft's shared bitmap paste path.

### Documentation
- Expand the public roadmap and task ledger for local/remote document history, PDF/page export,
  web publishing, provider-based AI, speech and image workflows, terminal/SSH safety, 3D research,
  and mobile/tablet support.
- Record current research and source-trust notes for Photopea, Pixlr, Affinity, Illustrator,
  Inkscape, and palette tools.

## [0.3.1] — 2026-11-27

Focused editor workflow, clipboard, and Web startup reliability release.

### Added
- **Demos menu:** open the Studio Showcase and starter document, with direct links to the tutorials.
- **Web startup diagnostics:** show an accessible error message and console detail if the WASM app
  cannot initialize.
- **Bitmap-paste behavior tests** for valid image content and malformed dimensions/pixel buffers.

### Changed
- **Tool palette:** restore compact CS6-inspired tool-family labels and keep its body usable in short
  windows; place Demos directly beside Palette.
- **File browser:** constrain the window to the display and virtualize visible rows so large folders
  do not repeatedly measure and paint off-screen metadata.
- **Preview and Swatches:** keep the Preview badge within the canvas and use a compact color grid.
- **WASM lifecycle:** retain `eframe::WebRunner` for the page lifetime so its resize observer and
  animation state remain owned after startup.
- **UI:** make external paste actions available from Edit and the canvas context menu; refine menu
  spacing and help/tutorial access.

### Fixed
- Avoid silently discarding WebRunner startup errors.
- Reject malformed clipboard bitmap payloads without inserting a broken object.
- Avoid unbounded file-browser growth and repeated metadata work for rows outside the viewport.

## [0.3.0] — 2026-11-26

Quality, evidence and release-engineering pass. No intentional behavior changes to editing tools.

### Added
- **LICENSE** (MIT), with an Adobe trademark disclaimer; referenced from the README and included in
  the Windows bundle.
- **`rust-toolchain.toml`** pinning Rust 1.100.0, rustfmt, clippy and the WASM target.
- **`scripts/package-windows.sh`** for deterministic bundle assembly (exe, docs, license,
  `BUILD-INFO.txt`, SHA-256 for executable and archive).
- **`scripts/check-docs.sh`** link/claim gate and **`scripts/ci-local.sh`** as the single CI
  definition of record, also used by the Forge mirror.
- **Integration tests** for the CLI, MCP server, formats error paths, theme contrast and file
  intents; property tests for color conversion and polygon booleans.
- **Panel geometry module** (`app/src/panels/geometry.rs`) with unit tests for dock/snap/stack and
  inspector clamping math.
- **.github/CONTRIBUTING.md**, **.github/SECURITY.md**, issue/PR templates and a Dependabot configuration.

### Changed
- **Single version source:** `app/Cargo.toml` is now the only version; `--version`, bundles, the
  About window and CI artifact names derive from it. `--version` no longer prints a fixed string.
- **Self-contained crate:** `logo.png` moved to `app/assets/logo.png` and the splash artwork to
  `app/assets/splash.png`; the build no longer reads `../screenshots/`.
- **One WASM host page:** `app/index.html` is the only source and CI copies it into `dist/wasm`.
- **CI:** adds coverage, `cargo audit`/`cargo deny`, WASM build, Windows build and CLI smoke tests.

### Fixed
- `--version` reported `0.2.5` while the crate said `0.2.0`.
- `app/PROJECT_STRUCTURE.md` described files and counts that no longer exist.
- **Properties collapse:** the chevron now receives its first click; its full-width header double-click target no longer shadows the control.
- **Panels dock:** the dock-wide collapse/expand control is a single state-aware toggle in the header and is covered by click-level tests.

### Changed
- **Icon system:** application UI icons use the embedded Phosphor icon font; the refreshed FireCraft mark, favicon and Windows icon are included with their third-party notices.
- **Project guidance:** README and docs index link a public product roadmap, reference research, CI/CD comparison, and provider-neutral `AGENTS.md`.

## [Unreleased — pre-0.3.0]

### Added (pre-0.3.0 polish)
- **Showcase module:** the demo document is built by `demo::showcase(width, height) -> Vec<Obj>`
  instead of 280 lines inside the application state. Layout tests assert that objects stay inside
  the page, text stays inside its card at four page sizes, and the CTA label is centred.
- **Viewport size flag:** `--size 960x640` (plus `scripts/gui-shot.sh`) for deterministic GUI review
  captures under Xvfb.

### Fixed (GUI pass)
- **Radial gradients** are clipped to their object and the rect is filled with the outer stop, so
  they no longer paint a disc over neighbouring content or leave the page showing through corners.
- **Slice URL labels** render as a tab below the slice, elided to its width, instead of covering the
  button label underneath.
- **First-frame fitting:** the showcase is framed once the live canvas rect exists, so it fits at
  960x640 instead of keeping the 1360 px zoom and clipping the page.
- **Multi-line text measurement** uses the longest line for width and the line count for height; the
  previous code counted newline characters as glyphs and over-reported selection bounds.
- **Properties inspector** scrolls instead of clipping its second row at the 118 px dock height.
- **Tool-rail hint** wraps instead of being cut mid-sentence; the Optimize Matte control has its own
  row instead of covering its label.

### Changed
- **Showcase opens unselected**, so first launch is not decorated with selection handles over the
  brand emblem.
- **Document tab overflow:** the canvas tab strip now keeps the active and most-recently-used tabs
  visible, compresses padding and labels as tabs accumulate, and folds older tabs into a `…` menu
  instead of clipping them. Mouse-wheel over the strip steps through open documents.
- **Bulk tab close:** right-clicking a canvas tab offers Close Tab, Close Other Tabs, Close Tabs to
  the Left/Right, and Close All Tabs. Dirty documents stop the queue and use the normal Save /
  Don't Save / Cancel prompt rather than being discarded silently.
- **CS6 grouped tool palette:** restored group leaders with bright corner triangles, click/right-click
  flyouts, and Auto Shape controls (sides, star ratio, drag hint) under the Vector section.
- **Architecture seams:** extracted file/session intents to `app/files.rs`, added a panel registry,
  and moved Properties and Color Mixer bodies into `panels/properties.rs` and `panels/color.rs`.
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
- **Expanded Mode is the startup workspace** with five CS6 panel stacks (Optimize/History,
  Pages/States/Layers, Styles/Color/Swatches, Info/Shapes, and Libraries/CSS/URL) plus a wide
  responsive right dock; CS6 Dual-Column remains a preset.
- Rebuilt the left rail as a compact six-category CS6 palette; every group leader is directly
  visible, with flyouts for the remaining members and Oval Marquee, Freeform, Replace Color, and
  Red Eye retained.
- Text and Filters menus now derive their choices from the active document, with Font/Size and
  filter-category submenus, live effect summaries, enable toggles, and per-effect removal.
- Replaced window-control glyphs that rendered as tofu with explicit `Full`/`Exit` and `Palette`
  labels.
- Rebuilt the file browser as an in-app window with an editable address bar, breadcrumb navigation,
  type filter, dense columns, and quick places. This removes the drag lag of the old viewport.
- Redesigned About and standardized dialog headers/footers, movable/resizable windows, and one
  primary-button treatment.
- Fixed the Unsaved Changes / Quit prompt clipping and wired Save/Discard/Quit continuations.
- Split Find/Replace, dialog file browsing, and the tool palette into focused modules. Added a
  navigable `docs/` tree (architecture, build/release, features, ADRs, glossary) and shortened the
  README to an entry point.

### Fixed
- **Unsaved close/quit loop:** Discard & Quit now arms a close gate so the OS close interceptor no
  longer cancels the app's own exit and reopens the prompt. Closing an earlier tab preserves the
  active document and selection instead of swapping in the wrong document.
- **Tab focus:** single-key tool shortcuts are blocked only by a focused text edit, not by any
  clicked button. Switching views or tabs now leaves the keyboard working immediately.
- Hidden text objects and color values are included in Find/Replace matching and replacement.
- Pen clicks, live rubber-band preview, and double-click/Enter finish create a committed open path;
  the obsolete drag-to-path path that produced glitches was removed.

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
