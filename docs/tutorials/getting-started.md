# Getting Started with FireCraft

## Launching

Run the native binary (`firecraft`) or open the WASM build in a browser.
`firecraft --demo` starts with sample content; `--screenshot out.png` renders
one frame headlessly. A brief splash plays at startup.

## Creating a document

- **File → New** (`Ctrl+N`) opens a document with the default 1920x1080 page.
- Each document is a tab in the document tab strip at the top of the canvas.
- **File → Open** (`Ctrl+O`) reads PNG, JPEG, GIF, WebP, TIFF, BMP, ICO, SVG
  and FireCraft's own format.

## The workspace

| Area | Contents |
|------|----------|
| Top | Menu bar and document tabs |
| Left | Tools palette: Select, Bitmap, Vector, Web, Colors, View categories |
| Right | Panel groups: Optimize/History/Align/Path, Pages/States/Layers, Styles/Mixer/Swatches/Info, Library/CSS |
| Bottom | Properties inspector with Live Filters |
| Footer | Status line and animation transport controls |

## Panels essentials

- Each panel group has a chevron in its header — a single click
  collapses/expands the group.
- The dock header bar has a `>>` / `<<` toggle that collapses the whole right
  dock to an iconic strip.
- Drag a panel header to float it; drag it to the left or right edge to
  re-dock. Double-click the header also toggles collapse.
- **Tab** hides all panels for a full-screen canvas; press it again to
  restore. **Shift+Tab** toggles only the right dock.

## Saving

- **Ctrl+S** saves; the first save asks for a path.
- **Ctrl+Shift+S** is Save As. **Ctrl+Q** quits with an unsaved-changes
  confirmation.
- Recent files appear at the bottom of the File menu.

Next: [Drawing & Editing](drawing-and-editing.md).
