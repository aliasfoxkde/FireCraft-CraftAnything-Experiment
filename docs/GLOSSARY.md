# FireCraft Glossary

Use these terms consistently in code, issues, UI copy, and architecture reviews.

| Term | Meaning |
|------|---------|
| **Document** | An open FireCraft project: pages, layers, objects, symbols, styles, and export settings. A document may have a file path or be untitled. |
| **Page** | A canvas-sized artboard inside a document. Pages own layers, guides, states, and a background. |
| **Layer** | An ordered container of objects within a page. Layers can be hidden, locked, web layers, and can carry opacity/blend settings. |
| **Object** | A vector, bitmap, text, web, gradient, symbol, or group element inside a layer. |
| **Hidden object** | An object with `visible = false`. It is excluded from normal editing and flat export, but visible as a ghost in god view. |
| **God view** | The editing-only infinite-canvas mode that extends the page background to the viewport, removes the page clip, and ghosts hidden elements. Flat exports still crop to page bounds. |
| **Page bounds** | The document rectangle that flat raster/SVG/HTML exports use. God view shows it as a dotted outline. |
| **Flat export** | A raster or vector export that contains only content inside page bounds. Layered `.fw.png` is not a flat export. |
| **Layered `.fw.png`** | A normal PNG with an embedded versioned FireCraft JSON payload in an iTXt chunk. Reopening it restores pages, layers, and off-page objects. |
| **Find/Replace mode** | `Text`, `FillColor`, `StrokeColor`, or `AnyColor`. The mode determines matching and which object color slots are written. |
| **Color slot** | One replaceable color field: flat fill, stroke, gradient start/end, or a live-effect color. |
| **Gradient fill** | A per-object linear fill with two color stops and normalized start/end points inside the object's bounds. Older documents without axis points load with the historic vertical direction. |
| **Gradient axis** | The directed line between a gradient fill's start and end points. Color interpolation projects a point onto this line and clamps the result to the two stops. |
| **Gradient stop** | A color at one end of the current two-stop gradient axis. Multi-stop editing is planned, but is not implemented yet. |
| **Tool palette** | The left rail with all CS6 tools, grouped into Select, Bitmap, Vector, Web, Colors, and View. |
| **Panel group** | A docked or floating tab group. Groups can collapse, stack, snap, and be re-docked. |
| **Inspector** | The bottom dock containing Properties, Symbol Properties, and Intelligence tabs. |
| **Intelligence** | The real OpenAI-compatible assistant panel. It never fabricates responses; missing configuration is shown as setup guidance. |
| **MCP** | The embedded Model Context Protocol JSON-RPC 2.0 server and its in-app console. |
| **Jump List** | The Windows taskbar context menu populated with recent documents and tasks (new document, about). |
| **Showcase** | The demo document built by `demo::showcase`: brand card, flame emblem, primary CTA and four feature cards. Used by `--demo`, the Welcome window and document templates. It is a document, not a dialog. |
| **Distribution bundle** | The directory/archive shipped to users: executable, README, license, and user docs. It is not a loose pile of build intermediates. |

## Naming rules

- Prefer **god view** for the editing mode and **infinite canvas** when referring to the toggle or
  user-facing setting.
- Say **page bounds**, not “canvas bounds”, when the distinction between viewport and export area
  matters.
- Say **color slot** when describing Find/Replace internals; say **fill/stroke/gradient stop** in
  user-facing copy.
