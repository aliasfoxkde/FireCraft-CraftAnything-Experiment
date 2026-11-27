# File Browser

The Open / Save / Import / Export browser is a resizable in-app window, not a separate native
viewport. It follows familiar desktop file-browser structure while keeping document actions in the
editor's event loop.

## Why in-app

The previous implementation used `show_viewport_immediate`, which forced the host application to
render again around the browser and made dragging visibly lag. The in-app window shares one event
loop and one style, and allows the address bar and app state to update immediately.

## Layout

- **Address bar** — editable; press Enter or **Go** to navigate. A file path selects its parent
  directory and fills the filename.
- **Look in** — clickable breadcrumb trail.
- **Filter** — filters the current folder by name or extension.
- **File type** — dropdown:
  - Open/Import: all supported, layered `.fw.png`, raster, SVG, PSD/AI, all files
  - Save/Export: PNG, JPEG, GIF, WebP, SVG, HTML5, TIFF, BMP, ICO
- **Places** — Home, Desktop, Documents, Pictures, Downloads, Workspace.
- **File list** — dense Name / Type / Size / Modified columns; double-click a file to confirm.
- **Footer** — filename, format-specific quality/transparency controls, estimated size, primary
  action, Cancel.
- **Window bounds** — the default window is 760×500 px and is clamped to the available display;
  long directories scroll inside the file list and do not grow the dialog. Only visible file rows
  are laid out, so size and modification metadata is read only for visible rows instead of the
  entire directory on every frame.

## Behavior rules

- Layered `.fw.png` preserves pages, layers, states, symbols, styles, and off-page objects.
- Other raster/vector formats flatten to page bounds.
- A cancelled Save As clears any pending tab-close or Save-All-and-Quit continuation, so the
  document is not discarded.
- Save/Export extension selection updates the filename without destroying prior dots
  (`poster.fw.png` stays `poster.fw.png` when PNG is selected).
- Default location follows the active document's folder, then the last browsed folder.

## Implementation seam

- UI: `dialogs/file_browser.rs`
- File operations and recent-document policy: `app.rs` (`begin_file_dialog`, `commit_file_dialog`,
  `open_path`, `save_to`, `export_to`)
- Serialization: `formats.rs`

Do not put format parsing or document mutation in the browser UI.

## Known follow-up

The file browser is not a native OS picker. Native-picker parity, browser-specific picker and
download behavior, accessible row navigation, and interactive checks on Windows/macOS/Linux remain
open in the task ledger. Do not claim those platform behaviors from the shared egui implementation.
