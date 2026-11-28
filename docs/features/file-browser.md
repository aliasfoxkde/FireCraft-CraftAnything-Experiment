# File dialogs and browser file workflows

FireCraft uses different file access models on desktop and in WebAssembly because the browser
does not expose arbitrary local folders to an application.

## Desktop application

Open, Save, Import, and Export use a resizable in-app file browser in the shared editor window.
It provides an editable path field, breadcrumbs, file-type and name filters, common places, a
virtualized file list, filename and format controls, and format-specific export options. The list
has a bounded height so the filename and action buttons remain visible at compact window sizes.

The desktop browser is not the operating system's native file dialog. It uses Rust filesystem
access for folder navigation and document I/O.

## WebAssembly application

- **Open and Import** invoke the browser's native file chooser. FireCraft reads the selected file
  into memory and decodes it without relying on a browser filesystem path.
- **Save and Export** show a compact FireCraft form for the filename, output format, and supported
  options. The app encodes the file in memory and starts a browser download. The browser's download
  settings choose where that download is stored.
- Browser Save does not create a persistent local path. Use Save As after reopening a downloaded
  document if you need another copy.

The WebAssembly app does not browse arbitrary folders. The native picker limits the selectable
file types to formats FireCraft can currently decode: layered PNG, supported raster images, and
SVG. PSD, AI, AVIF, and HEIC require a format bridge and are not decoded by the current app.

## Supported formats and behavior

- Layered `.fw.png` preserves pages, layers, states, symbols, styles, and off-page objects.
- Other raster and vector formats flatten to page bounds when saved or exported.
- Supported open/import formats are PNG, JPEG, GIF, WebP, TIFF, BMP, ICO, and SVG. PSD, AI,
  AVIF, and HEIC have no decoder in the current build.
- Supported save/export formats are PNG, JPEG, GIF, WebP, SVG, HTML5, TIFF, BMP, and ICO. AVIF,
  HEIC, PSD, and AI are not writable.
- PNG selected for a `.fw.png` filename writes the embedded FireCraft document payload; ordinary
  `.png` export writes a flattened image.
- A cancelled desktop Save As clears pending close/quit continuations so it cannot discard the
  document.

## Known limits

- Desktop folder browsing, overwrite prompts, path validation, and native OS picker parity need
  interactive review on Windows, macOS, and Linux.
- Browser file selection, in-memory open/import, layered Save As, reopen, and PNG download were
  exercised in Chromium at 1360×860. Open and Save As were also exercised at 1024×600. The browser
  cancel path, large-file memory limits, browser drag/drop from the operating system, and other
  browser engines remain unverified.
- A browser download is controlled by the user's browser policy; FireCraft cannot silently write
  to an arbitrary local path.

## Implementation

- Dialog UI and platform-specific presentation: `app/src/dialogs/file_browser.rs`
- Document file actions and browser byte I/O: `app/src/app/files.rs`
- Browser file picker, `FileReader`, and Blob download: `app/src/main.rs`
- Format decoding and encoding: `app/src/formats.rs`
- Workflow status and evidence: [workflow inventory](../workflows/README.md),
  [browser file workflow receipt](../testing/browser-file-workflow-2026-11-28.md)
