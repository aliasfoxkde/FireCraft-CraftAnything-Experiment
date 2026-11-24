# FireCraft — Adobe Fireworks CS6 Clean-Room Desktop Studio, MCP Server & Headless CLI

FireCraft is a modern, clean-room desktop reimplementation of **Adobe Fireworks CS6 (v12.0)** built in **Rust + egui/eframe**. It delivers hybrid vector and bitmap editing, crisp square-cornered CS6 Charcoal Dark and Classic Platinum Light themes with OS auto-detection, live non-destructive effects, web slices & hotspots, multi-state animation, reusable symbols/styles, multi-object selection & grouping, an embedded **Model Context Protocol (MCP) JSON-RPC 2.0 server**, a searchable **Command Palette (`Ctrl+K`)**, and headless **CLI tooling**.

---

## Implemented Feature Surface

| Area | Status | Details |
|------|--------|---------|
| **CS6 Workspace Layout** | Complete | 10-menu CS6 menu bar (`File`, `Edit`, `View`, `Select`, `Modify`, `Text`, `Commands`, `Filters`, `Window`, `Help`) with `Alt+<key>` mnemonics and arrow-key navigation; header `Cmd (Ctrl+K)`, `MCP / CLI`, `Theme: Dark / Light / Auto`, and Workspace presets (`Expanded Mode`, `CS6 Dual-Column`, `All Panels Floating`, `Iconic Mode`, `Netbook Mode`); Main Toolbar with file/edit/arrange actions only (Resolution `1920×1080` and DPI `144 ppi` live in the canvas footer); Multi-Document Tab Bar (Middle-Click or `×` to close); `Original \| Preview \| 2-Up \| 4-Up` view switcher; `Tab` / `F4` Full-Screen Canvas mode |
| **Dynamic Panel Grouping, Floating & Snapping** | Complete | Every panel group supports collapse/expand (`▾`/`▸` or double-click header), bottom vertical resize splitters, draggable tabs (drag onto another panel header to group, onto a dock column to pin, or onto the canvas to tear off as a floating window), undock/pin (`⧉` / `📌`), `≡` Panel Options menu, dual-column right docking (`RightOuter` + `RightInner`), left docking (`LeftInner`), and magnetic edge-to-edge floating window snapping |
| **Crisp CS6 Chrome & Navigation** | Complete | Zero corner radius (`CornerRadius::ZERO`) across all panels, tabs, buttons, menus, and tool slots using measured CS6 Charcoal and Classic Light palettes; `Ctrl+Scroll` cursor-anchored zoom, `Shift+Scroll` horizontal pan, `Scroll` vertical pan, `Alt+Scroll` / Middle-Drag / `Space+Drag` 2D Hand Pan, and `Insert` / `Ctrl+0` center & fit page to viewport |
| **Multi-Selection, Grouping & Context Menu** | Complete | `Shift`-click, `Ctrl`-click, or marquee drag to select multiple objects; `Ctrl+G` / `Ctrl+Shift+G` (or toolbar/context menu) to Group and Ungroup; adaptive right-click canvas context menu with Cut, Copy, Paste, Duplicate, Delete, Group/Ungroup, Arrange, Transform, Convert to Path/Symbol, Live Drop Shadow, Web Slice, and Copy CSS |
| **MCP JSON-RPC 2.0 Server & Headless CLI** | Complete | Run `firecraft mcp` for stdio JSON-RPC 2.0 (`2025-06-18` protocol) exposing `firecraft.info`, `firecraft.apply`, `firecraft.export`, `firecraft.undo`, `firecraft.redo`, `firecraft.listCommands`, and `document://current`; plus headless CLI verbs (`info`, `export`, `convert`, `batch`, `apply`, `list-commands`) and an interactive in-app **MCP / CLI Console** |
| **Searchable Command Palette (`Ctrl+K`)** | Complete | Instant fuzzy search across all 68 registered editor, filter, alignment, auto-shape, web, and theme commands |
| **Tools (31 Tools)** | Complete | `Select`: Pointer (`V`), Subselection (`A`), Scale (`Q`), Crop (`C`) · `Bitmap`: Marquee (`M`), Lasso (`L`), Magic Wand (`W`), Brush (`B`), Pencil, Eraser (`E`), Blur, Sharpen, Dodge, Burn, Rubber Stamp (`S`) · `Vector`: Line (`N`), Pen (`P`), Rectangle (`U`), Rounded Rect, Ellipse, Polygon, Star, Text (`T`), Knife · `Web`: Slice (`K`), Hotspot (`J`) · `Colors`: Eyedropper (`I`), Paint Bucket (`G`), Gradient · `View`: Hand (`H`), Zoom (`Z`), Fit (`Ctrl+0`) |
| **Live Filters & Effects** | Complete | Non-destructive Live Filters stack (`+ Add` in Properties Inspector & `Filters` menu): Drop Shadow, Inner Shadow, Outer Glow, Inner Glow, Inner Bevel, Gaussian Blur, and Brightness/Contrast |
| **Panels (21 CS6 Panels + Intelligence)** | Complete | `Optimize`, `History`, `Align`, `Path`, `Pages`, `States` (Animation frames + live playback + 3-step motion tween + onion skinning), `Layers` (hierarchical tree + blend modes + opacity + Web Layer), `Styles`, `Color Palette`, `Color Mixer`, `Swatches` (252 Color Cubes), `Info`, `Special Characters`, `Image Editing`, `Auto Shapes`, `Auto Shape Properties`, `Document Library` (Symbols), `Common Library` (clickable UI widgets), `CSS Properties`, `URL`, `Behaviors`, plus the bottom `Properties`, `Symbol Properties`, and `Intelligence` tabs |
| **Import & Export Formats** | Complete | Read/write PNG, JPEG, GIF, WebP, TIFF, BMP, ICO, editable SVG round-trip, and interactive HTML5 + CSS3 + Slice/Hotspot web export |
| **Infinite Canvas & Layered `.fw.png`** | Complete | Footer `∞ Canvas` toggle with dotted page bounds and export disclaimer. `*.fw.png` saves a normal page-bounded PNG plus an embedded project payload (iTXt) so layers, states, symbols, styles, and out-of-page objects reopen intact. |
| **Intelligence Inspector** | Complete | Bottom `Intelligence` tab with OpenAI-compatible endpoint/model/key setup, real selection-aware prompts, error reporting, and copyable MCP client config. |
| **Quality & Tests** | Complete | Zero compiler or Clippy warnings (`cargo clippy --all-targets -- -D warnings`), 28 unit tests covering color math, document model, multi-selection & grouping, polygon booleans, style gradients, layered PNG round-trip, clipboard & full-screen canvas, MCP JSON-RPC 2.0 protocol dispatch, animation states, SVG/CSS export, and editor workflows |

---

## Building & Running

```sh
cd app
cargo test
cargo clippy --all-targets -- -D warnings
cargo build --release
```

### Desktop GUI

```sh
cd app
cargo run --release -- --demo
```

### Headless CLI & MCP Server

```sh
# Print JSON summary of a document (or the built-in showcase)
firecraft.exe info --demo

# Headless export to SVG, HTML5, PNG, JPEG, WebP, TIFF, BMP, GIF, or ICO
firecraft.exe export --demo showcase.svg
firecraft.exe export --demo showcase.html

# Batch convert a folder of images/vectors
firecraft.exe batch ./assets --format webp --out ./dist

# List all 68 registered automation commands
firecraft.exe list-commands

# Start the Model Context Protocol (MCP) JSON-RPC 2.0 stdio server
firecraft.exe mcp --demo
```
