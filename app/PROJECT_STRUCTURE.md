# FireCraft Project Structure

## Source Files

### Core Modules
- `main.rs` — entry point, CLI arg parsing, eframe app launch, screenshot PNG generation
- `app.rs` — `FireCraft` struct (application state), `Tool` enum (28 tools), `Drag` struct, `FireCraft` impl (state management, undo/redo, pointer handling, `impl eframe::App for FireCraft` with `fn ui` rendering)
- `canvas.rs` — canvas rendering (`show()`), hit testing (`hit_test()`), texture caching (`TextureCache`)
- `doc.rs` — `Document` struct, `Obj` (document object), `Shape` enum, `Layer`, `Page`, `Snapshot`
- `shape.rs` — shape rendering primitives
- `paths.rs` — path data structures
- `page.rs` — page management

### UI/Panel Modules
- `panels.rs` — panels UI rendering (layers, properties, color, filters)
- `properties.rs` — properties panel for selected objects (fill, stroke, width, name)

### Utilities
- `colors.rs` — color utilities, swatch grid, hex conversion
- `constants.rs` — application constants (UNDO_LIMIT, ZOOM_MIN/MAX, DEFAULT_WIDTH/HEIGHT)
- `theme.rs` — `ThemeMode`, `Palette`, theme resolution
- `formats.rs` — export format support (PNG, JPEG, SVG, PDF)
- `icons.rs` — icon definitions
- `tools.rs` — tool definitions and helpers
- `history.rs` — undo/redo history

### External References
- `logo.png` — application logo (697 KB)

## Build
```bash
cargo build          # Development build
cargo run -- --screenshot path.png  # Screenshot mode (generates PNG, exits)
```

## Key Architecture
- `FireCraft::new()` — constructs initial state (dark theme, 800x600 doc, Select tool)
- `FireCraft::ui()` — renders menu bar, canvas, status bar
- `canvas::show(ui, app)` — renders workspace background + white canvas + page content
- `canvas::hit_test(app, pos)` — returns (layer_index, object_index) or None
- `on_pointer_down(doc_pos, screen_pos, alt, ctrl)` — tool-specific interaction handling
- Undo/redo via `Snapshot` (deep clone of page state)

## Current UI Layout (in `fn ui`)
1. Menu bar: File, Edit, View, Modify, Filters
2. Canvas area: `canvas::show(ui, self)` — fills available space
3. Status bar: status text, zoom %, canvas dimensions

## Tool enum (28 variants)
Select, Marquee, Lasso, Wand, Crop, Hand, Zoom, Fit, Brush, Pencil, Eraser, Blur, Sharpen, Dodge, Burn, Line, Rectangle, Ellipse, Polygon, Path, Text, Picker, Fill, Gradient

## Notes for development
- `app.rs` is the largest file (31KB) — contains state, enums, and UI rendering
- All egui widgets use `eframe::egui` namespace
- `self.show_start` — when true, canvas is NOT rendered (show splash/start screen instead)
- Screenshot path uses `--screenshot <path>` flag — generates gradient PNG and exits 0
