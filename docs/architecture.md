# Architecture

FireCraft is one native Rust application with an egui/eframe presentation layer. The important
seams are **document state**, **rendering**, **interaction**, and **platform integration**. Keep
those seams explicit; avoid adding another “misc” module that reaches into all four.

## Module map

| Module | Owns | Must not own |
|--------|------|--------------|
| `app.rs` | `FireCraft` application state, tool routing, editing commands, document tabs, session ownership | Platform shell code, dialog layout |
| `app/files.rs` | File/session intents (`New`, `Open`, `Save`, `CloseTab`, `Quit`), save/dirty choreography and close gating | Dialog widgets, platform shell code |
| `app/find_replace.rs` | Match semantics for text and color modes, Find Next selection, replacement side effects | Dialog widgets, layout |
| `doc.rs` | Serialized `Document`, `Page`, `Layer`, `Obj`, `Shape`, styles, effects, snapshots | UI state, OS paths |
| `canvas.rs` | Viewport painting, page/god-view rendering, hit testing, canvas context menu | Dialog state, panel tabs |
| `layout.rs` | Top-level frame, menu/toolbar/status composition, docks, shortcuts, floating canvases | Individual panel bodies, file browser |
| `panels/mod.rs` | Panel registry, host chrome, docked panel bodies, tab overflow and drag/resize | Main window layout |
| `panels/properties.rs` | Properties inspector body | Panel chrome, layout geometry |
| `panels/color.rs` | Color Mixer body | Panel chrome, document model |
| `panels/tools.rs` | Complete left tool inventory, section rendering, color/display footer | Tool behavior (that lives in `app.rs`) |
| `dialogs/mod.rs` | Shared dialog chrome plus application settings/about/quit dialogs | File listing, Find/Replace semantics |
| `dialogs/file_browser.rs` | File browsing UI, filters, address bar, commit/cancel flow | Save/open serialization |
| `dialogs/find_replace.rs` | Find/Replace dialog surface and live scope preview | Matching/replacement implementation |
| `demo.rs` | Showcase geometry: `showcase(width, height) -> Vec<Obj>` and text fitting | Application state, layer routing, egui |
| `formats.rs` | Raster/SVG/HTML/layered-PNG serialization | Dialog or file-picker UI |
| `mcp.rs` | JSON-RPC command surface and headless CLI verbs | GUI state transitions beyond the public app commands |
| `shell_windows.rs` | AppUserModelID and Jump List COM integration | Document logic or UI |
| `theme.rs` | Palettes, egui visual style, typography, dialog/menu spacing | Widget-specific layout |

## Seams and invariants

### Document state is the single source of truth

`FireCraft.doc` is the active document; `open_docs[active_doc]` is its persisted slot. Call
`sync_active_doc_slot()` before switching, closing, saving all, or opening another document.
Selection indices are layer-local, so changing `selected_layer` must be paired with a selection
reset or an intentional re-selection.

### God view never changes export semantics

`canvas::render_viewport` chooses between the page clip and the viewport clip. God view is an
editing view only:

- page background extends across the viewport;
- objects outside page bounds render;
- hidden layers/objects render at reduced opacity;
- flat exports still use `Document` page bounds.

Do not make serialization depend on `infinite_canvas`.

### Focus belongs to text edits, not buttons

A clicked tab or toolbar button can hold egui keyboard focus. Shortcut dispatch must therefore gate
on `ctx.text_edit_focused()`, never `egui_wants_keyboard_input()`; the latter is true for any focused
widget and silently disables all single-key tool shortcuts after the first click. Requests to focus
the canvas are best-effort and must not be used as the shortcut gate.

### Dialogs own layout, app methods own effects

A dialog may mutate simple form fields directly, but multi-step effects (replace all, save all and
quit, open, export) belong to an app method. This keeps the button path testable without an egui
frame.

### The file browser is an in-app window

It intentionally is not a second immediate-mode viewport. A second viewport re-renders the host
frame around the main application and produced drag lag. The in-app browser also makes the
editable address bar and the main app's state share one event loop.

### Floating canvases follow viewport support

The document-tab float action uses an external eframe viewport when the active backend supports
native viewports. When `Context::embed_viewports()` is true (including eframe WebAssembly), the
document instead appears in a movable egui window inside the workspace. The main Tools rail and
Properties dock continue to edit the same document; window focus selects the document used by
keyboard commands. Docking or closing the embedded window returns the edited document to the tab
workspace. The web fallback exists because eframe's web backend does not implement multiple OS
viewports. Browser editing, undo/redo, move, and redock have current Chromium evidence; native OS
window save/reopen, drag/drop, and platform focus remain separate acceptance work.

### Platform code is isolated and optional

`shell_windows.rs` is compiled only on Windows. Every COM failure is logged and ignored; startup
must not depend on the taskbar shell. Non-Windows targets compile unchanged.

## Known deepening opportunities

These are ordered by payoff, not by aesthetics:

1. **`app.rs` interaction dispatcher.** Tool routing, editing commands, and document-tab
   orchestration still share a large file. Continue extracting command bodies behind request /
   outcome seams; `app/files.rs` is the first slice. Keep `FireCraft` as the state owner.
2. **`layout.rs` chrome composition.** Extract menu bar, document tab strip, and canvas footer
   into `layout/chrome.rs`; keep dock geometry and shortcuts in `layout/mod.rs`.
3. **Panel registry — started.** `panels/mod.rs` now owns a `PANELS` descriptor registry with
   titles, default heights, and render entry points; Properties and Color Mixer bodies are split
   out. Once drag/resize gestures are covered by tests, move the remaining bodies behind the same
   seam.
4. **File-operation adapter — done.** `app/files.rs` defines a `FileService` trait
   (`open`/`save`) with `FormatFileService` as the disk-backed default; open, save, export,
   import and save-all route through it, so headless tests inject a fake service instead of
   touching the filesystem. Format metadata and recent-file policy still live in `formats.rs`
   and `app/files.rs` respectively.
5. **Shell snapshot adapter — deferred.** Only add a `ShellIntegration` seam when a second shell
   target or recorded shell tests exist; otherwise it adds structure without leverage.

See `docs/adr/` for decisions that should not be re-litigated casually.

## Deepened seams (0.3.0)

- **`panels/geometry.rs`** — panel group heights, magnetic snapping and stack placement as pure
  functions over `egui::Rect`/`Pos2` (no `&mut` state). `app.rs` builds the neighbour slots and
  delegates; the rules are unit-tested without a window.
- **`harness.rs`** — a headless frame harness built on `egui::Context::run_ui`. It boots the real
  `layout::show` at 1360×860 and 960×640, feeds key events and inspects `FireCraft` state, so
  keyboard and layout regressions no longer need a display or a human. Test-only (`cfg(test)`).
- **Test topology** — logic tests stay inline; cross-module and process-level tests live in
  `app/tests/` (`cli`, `mcp`, `hygiene`). Golden files in `app/tests/golden/` are regenerated only
  with `UPDATE_GOLDEN=1`.

## Build topology

The application is a single portable executable by design on Windows. Dynamic libraries would
require shipping a matching MinGW/MSVC runtime and graphics dependencies and would make support
harder for a desktop creative tool. The release profile uses LTO, a single codegen unit, aborting
panics, and symbol stripping. See [Build & Release](build-release.md).

## Adding a feature

1. Name the domain concept in `docs/GLOSSARY.md` if it is new.
2. Put pure behavior in the owning module, not in a dialog.
3. Add a narrow test at the behavior seam (document/model or app method), not a screenshot test.
4. Put the UI in the smallest existing surface; prefer extending `panels/tools.rs` or a dialog
   submodule over growing `app.rs`.
5. Update the feature page and roadmap. Do not duplicate the same checklist in three files.
