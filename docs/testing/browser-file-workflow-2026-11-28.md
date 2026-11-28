# Browser file workflow receipt — 2026-11-28

## Build under test

- Product version: FireCraft 0.3.8 local validation build; this is not a public release.
- Base commit: `83268df5ff5a009792e8b20b7a20b0ff84b09c79`.
- Source patch identity: SHA-256 of `git diff --binary HEAD -- app` at test time:
  `0bc6ffca2355c4a744dc77bb1c71741da31061936e7df01e27e0d2e275809e31`.
- Rust: `1.100.0`; wasm-bindgen: `0.2.130`; target: `wasm32-unknown-unknown`.
- Browser: Chromium `154.0.8022.0`, headed under Xvfb, SwiftShader WebGL.
- Build command: `cargo build --locked --release --target wasm32-unknown-unknown`, followed by
  wasm-bindgen and assembly into `output/playwright/hill-climb-2026-11-28/current-main/`.
- Bundle SHA-256: `firecraft_bg.wasm`
  `0e45df0253a6885f5bab79ee0b4c5039623b7954216c05fc45b0da1bfcf87f18`; `firecraft.js`
  `a550193af6dd25d7985a3c427de70443d96bf6091194c6e4e7e27d2aa8c8390`; `index.html`
  `0c0d78a034f625dbeba1f5385d3ebbb3aa5710f3351726979c8fe8106f599d1a`.
- Build manifest: `output/playwright/hill-climb-2026-11-28/current-main/BUILD-INFO.txt`.

## Browser scenarios and results

All scenarios ran against the served release bundle, not a development mock. The complete file
round-trip used a 1360×860 viewport; Open and Save As were also exercised at 1024×600. Browser
page-error and application-console-error listeners remained empty during both flows.

1. **Open SVG:** File → Open invoked the browser file chooser. A real SVG file containing a
   rectangle and circle was selected. FireCraft displayed the document name, rendered the two
   shapes, and listed two editable objects in Layers.
2. **Import raster:** File → Import selected the repository's actual 128×128 PNG favicon. The
   canvas displayed it and Layers showed a third selected Bitmap object.
3. **Save layered project:** Save As downloaded `browser-project.fw.png` (421,033 bytes). The PNG
   contained the FireCraft layered-document payload. This verifies browser encoding and the
   browser download path without a filesystem path.
4. **Reopen saved project:** The downloaded file was selected through Open. The document reopened
   with its SVG rectangle, SVG ellipse, and Bitmap layer intact.
5. **Export raster:** Export downloaded `export.png` (6,359 bytes). Its first eight bytes were the
   PNG signature `89504e470d0a1a0a`.
6. **Compact browser viewport:** At 1024×600, the Save As form fit inside the editor viewport with
   Format, File name, Save, and Cancel visible. File→Open invoked the browser chooser; after an SVG
   was selected, the `compact.svg` tab and rendered rectangle were visible. See compact screenshots.
7. **Drop-path probe:** A synthetic browser `DataTransfer` SVG drop created a rendered object. This
   only proves the application drop handler with a synthetic event; OS drag-and-drop is not proven.

The file chooser in automation was supplied through Playwright's `FileChooser.setFiles`; this
exercises the real browser input, FileReader, decoder, app state, and Blob download, but does not
visually inspect the operating-system-owned picker itself.

## Evidence

Screenshots are unaltered Chromium captures from the tested bundle:

- [Opened SVG document](evidence/file-dialog-wasm-2026-11-28/browser-open-svg.png)
- [Compact Save As form at 1024×600](evidence/file-dialog-wasm-2026-11-28/browser-save-1024x600.png)
- [SVG opened at 1024×600](evidence/file-dialog-wasm-2026-11-28/browser-open-1024x600.png)
- [Imported PNG bitmap](evidence/file-dialog-wasm-2026-11-28/browser-import-png.png)
- [Compact browser Save As form](evidence/file-dialog-wasm-2026-11-28/browser-save-form.png)
- [Reopened layered project](evidence/file-dialog-wasm-2026-11-28/browser-reopen-layered.png)
- [Browser Export form](evidence/file-dialog-wasm-2026-11-28/browser-export-form.png)
- [Synthetic SVG drop probe](evidence/file-dialog-wasm-2026-11-28/browser-drop-svg.png)

The saved `.fw.png` and flattened export were validated from the downloaded bytes in the Playwright
run; they remain in the ignored local review directory, not this documentation folder.

## Limits and next acceptance

- This verifies Chromium only. Firefox, Safari, and Edge remain untested.
- A first compact Chromium attempt was run without Xvfb and stalled; it produced no evidence and was
  discarded. The successful compact run used the documented Xvfb setup and is captured above.
- Browser picker cancellation, malformed/empty files, very large files and memory pressure, multiple
  selection, permission restrictions, and browser download cancellation remain untested.
- Desktop in-app folder navigation, overwrite decisions, path errors, keyboard-only use, high-DPI
  scaling, and native Windows/macOS/Linux review remain open.
- The browser cannot write to an arbitrary user-selected path; Save/Export use the browser download
  policy. Browser Save does not attach a durable local path to the open document.
- Drop-path evidence is synthetic and does not establish OS file drag-and-drop support.
- This receipt is specific to the bundle hashes above. It is not evidence for a later build, a
  deployed page, or a tagged release.
